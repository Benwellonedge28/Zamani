//! Zamani Quantum IR — Scheduling Interval Index
//!
//! Path:
//!     src/quantum/ir/scheduling/interval.rs
//!
//! # Purpose
//!
//! This module provides the scheduling-layer representation and deterministic
//! index for intervals occupied by scheduled semantic operations.
//!
//! IMPORTANT:
//!
//! This module does NOT redefine the canonical temporal interval.
//!
//! `quantum::ir::timing::interval::TimeInterval` owns:
//!
//! - temporal interval semantics;
//! - half-open `[start, end)` semantics;
//! - interval arithmetic;
//! - intersection;
//! - overlap;
//! - adjacency;
//! - temporal containment.
//!
//! This module owns the scheduling concern:
//!
//! - associating an interval with an `OperationId`;
//! - deterministic interval ordering;
//! - duplicate-operation detection;
//! - interval insertion/removal;
//! - overlap queries;
//! - conflict queries;
//! - deterministic traversal;
//! - schedule-local interval indexing.
//!
//! # Architectural boundary
//!
//! ```text
//!                    Zamani semantic IR
//!                           |
//!                           v
//!                    Operation / OperationId
//!                           |
//!                           v
//!                 scheduling::interval
//!                           |
//!              +------------+------------+
//!              |                         |
//!              v                         v
//!        TimeInterval              OperationId
//!      quantum::ir::timing       quantum::ir::identity
//!              |
//!              v
//!       scheduling::schedule
//!              |
//!              v
//!        scheduling algorithms
//!              |
//!              v
//!          hardware
//! ```
//!
//! # Ownership
//!
//! This file owns:
//!
//! - `ScheduledInterval`;
//! - `IntervalConflict`;
//! - `IntervalIndex`;
//! - deterministic interval-index errors;
//! - interval-index-local ordering and lookup semantics.
//!
//! This file does NOT own:
//!
//! - `TimePoint`;
//! - `Duration`;
//! - `TimeInterval`;
//! - `OperationId`;
//! - `QubitId`;
//! - physical qubit identity;
//! - resource topology;
//! - scheduling algorithms;
//! - routing;
//! - optimization;
//! - hardware clocks;
//! - hardware `dt`;
//! - pulse synthesis;
//! - calibration;
//! - execution.
//!
//! # Why this is separate from `timing::interval`
//!
//! A temporal interval answers:
//!
//!     "What span of semantic time is represented?"
//!
//! A scheduling interval answers:
//!
//!     "Which scheduled semantic operation occupies that span?"
//!
//! Mixing those responsibilities would force the timing layer to know about
//! operations and scheduling. That would violate the IR dependency boundary.
//!
//! # Canonical time semantics
//!
//! All intervals are half-open:
//!
//! ```text
//! [start, end)
//! ```
//!
//! Therefore:
//!
//! ```text
//! [0, 10) and [10, 20)
//! ```
//!
//! are adjacent but do not overlap.
//!
//! Zero-duration intervals are valid semantic events but do not create
//! positive-duration scheduling conflicts.
//!
//! # Scalability
//!
//! There is deliberately no:
//!
//! - maximum number of intervals;
//! - maximum number of operations;
//! - maximum number of qubits;
//! - maximum number of resources;
//! - maximum schedule depth;
//! - maximum machine size;
//! - fixed hardware topology.
//!
//! Collection sizes grow only with the data actually inserted.
//!
//! `usize` is used exclusively where the Rust standard library requires a
//! collection length, capacity, or positional index.
//!
//! Semantic identities are represented by strongly typed `OperationId`s.
//!
//! # Complexity
//!
//! The index uses a `BTreeMap` keyed by interval start time.
//!
//! Let:
//!
//! - `n` = number of indexed intervals;
//! - `k` = number of returned overlapping intervals.
//!
//! Operations have the following asymptotic characteristics:
//!
//! - insertion: `O(log n)` plus duplicate checking;
//! - removal: `O(log n)`;
//! - exact operation lookup: `O(log n)`;
//! - ordered traversal: `O(n)`;
//! - overlap query: `O(log n + k + p)` where `p` is the number of
//!   potentially long-lived intervals beginning before the query start.
//!
//! A standard-library `BTreeMap` cannot provide an augmented interval tree
//! without maintaining additional subtree metadata. This module deliberately
//! avoids unsafe code and non-standard dependencies.
//!
//! Consequently, it does not make an unjustified claim of guaranteed
//! `O(log n + k)` interval queries.
//!
//! The data structure remains deterministic and suitable for large sparse
//! schedules. A future specialized interval tree may implement the same
//! public semantic contract without changing `ScheduledInterval`.
//!
//! # Determinism
//!
//! The index never relies on hash-map iteration order.
//!
//! Canonical ordering is:
//!
//! 1. start time;
//! 2. end time;
//! 3. operation identity.
//!
//! # Thread safety
//!
//! The module contains no:
//!
//! - global state;
//! - mutable statics;
//! - thread-local scheduling state;
//! - unsafe code;
//! - interior mutable global registries.
//!
//! The index is therefore naturally compatible with ownership-based
//! concurrency. Independent indexes can safely be built in parallel.
//!
//! # Serialization
//!
//! This module does not define a serialization format.
//!
//! `quantum::ir::serialization` remains the canonical serialization owner.
//!
//! Canonical serialization should use `iter()` and serialize:
//!
//! - operation identity;
//! - start;
//! - end.
//!
//! It must not serialize:
//!
//! - B-tree implementation details;
//! - allocator state;
//! - capacity;
//! - internal bucket/node structure.
//!
//! # Hashing
//!
//! `quantum::ir::hash` remains the canonical hashing owner.
//!
//! Hashing should be based on the deterministic semantic iteration order.
//!
//! # Integration with `scheduling::schedule`
//!
//! `schedule.rs` already owns `ScheduledOperation` and the final `Schedule`
//! representation. This module intentionally does not import `schedule.rs`.
//!
//! This direction is deliberate:
//!
//! ```text
//! timing
//!   |
//!   v
//! scheduling::interval
//!   |
//!   v
//! scheduling::schedule
//! ```
//!
//! The reverse dependency would create unnecessary coupling.
//!
//! `schedule.rs` can construct a `ScheduledInterval` from its existing:
//!
//! - `OperationId`;
//! - `TimeInterval`.
//!
//! Resource conflict policy remains in `schedule.rs` because resources are
//! outside the ownership of this file.
//!
//! # Qubit integration
//!
//! This file intentionally does not import `quantum::ir::qubit`.
//!
//! Scheduling intervals are valid for:
//!
//! - quantum operations;
//! - classical operations;
//! - pulse operations;
//! - analog evolution;
//! - logical operations;
//! - distributed operations;
//! - synchronization events.
//!
//! Where a caller needs qubit identity, it must use the canonical
//! `quantum::ir::qubit::QubitId` in the resource layer.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! -----------------------------------------------------------------------------
//! This file implements scheduling interval indexing, not scheduling policy.
//! -----------------------------------------------------------------------------
//
// Safety contract.
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;

use super::super::identity::OperationId;
use super::super::timing::{TimeInterval, TimePoint};

// =============================================================================
// Result and error types
// =============================================================================

/// Result type returned by scheduling-interval operations.
pub type IntervalResult<T> = Result<T, IntervalError>;

/// Errors produced by the scheduling interval index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntervalError {
    /// The same semantic operation was already indexed.
    DuplicateOperation {
        /// The duplicated operation.
        operation: OperationId,
    },

    /// An operation expected by the caller was not present.
    OperationNotFound {
        /// Missing operation.
        operation: OperationId,
    },

    /// The supplied interval does not satisfy the required scheduling
    /// invariant.
    InvalidInterval {
        /// Start coordinate.
        start: TimePoint,

        /// End coordinate.
        end: TimePoint,
    },
}

impl fmt::Display for IntervalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateOperation { operation } => {
                write!(
                    formatter,
                    "operation {:?} already has a scheduling interval",
                    operation
                )
            }

            Self::OperationNotFound { operation } => {
                write!(
                    formatter,
                    "operation {:?} has no scheduling interval",
                    operation
                )
            }

            Self::InvalidInterval { start, end } => {
                write!(
                    formatter,
                    "invalid scheduling interval: start {:?} exceeds end {:?}",
                    start,
                    end
                )
            }
        }
    }
}

impl std::error::Error for IntervalError {}

// =============================================================================
// ScheduledInterval
// =============================================================================

/// A semantic operation associated with a canonical temporal interval.
///
/// This type is deliberately smaller than `ScheduledOperation` from
/// `scheduling::schedule`.
///
/// It represents only the relationship:
///
/// ```text
/// OperationId -> TimeInterval
/// ```
///
/// Resource ownership remains outside this type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScheduledInterval {
    operation_id: OperationId,
    interval: TimeInterval,
}

impl ScheduledInterval {
    /// Creates a scheduled interval.
    ///
    /// `TimeInterval` is already validated by the canonical timing layer.
    #[must_use]
    pub const fn new(
        operation_id: OperationId,
        interval: TimeInterval,
    ) -> Self {
        Self {
            operation_id,
            interval,
        }
    }

    /// Creates an interval directly from validated boundaries.
    ///
    /// This constructor exists for callers that already have raw semantic
    /// boundaries and want this module to perform validation.
    pub fn from_bounds(
        operation_id: OperationId,
        start: TimePoint,
        end: TimePoint,
    ) -> IntervalResult<Self> {
        let interval = TimeInterval::new(start, end).map_err(|_| {
            IntervalError::InvalidInterval { start, end }
        })?;

        Ok(Self::new(operation_id, interval))
    }

    /// Returns the semantic operation identity.
    #[must_use]
    pub const fn operation_id(self) -> OperationId {
        self.operation_id
    }

    /// Returns the canonical temporal interval.
    #[must_use]
    pub const fn interval(self) -> TimeInterval {
        self.interval
    }

    /// Returns the inclusive start time.
    #[must_use]
    pub const fn start(self) -> TimePoint {
        self.interval.start()
    }

    /// Returns the exclusive end time.
    #[must_use]
    pub const fn end(self) -> TimePoint {
        self.interval.end()
    }

    /// Returns whether this is a zero-duration event.
    #[must_use]
    pub fn is_zero_duration(self) -> bool {
        self.interval.is_empty()
    }

    /// Returns whether this interval occupies positive semantic time.
    #[must_use]
    pub fn is_non_empty(self) -> bool {
        self.interval.is_non_empty()
    }

    /// Returns whether this interval has positive-duration overlap with
    /// another scheduled interval.
    #[must_use]
    pub fn overlaps(self, other: Self) -> bool {
        self.interval.overlaps(other.interval)
    }

    /// Returns whether this interval is adjacent to another interval.
    #[must_use]
    pub fn is_adjacent(self, other: Self) -> bool {
        self.interval.is_adjacent(other.interval)
    }

    /// Returns whether this interval is temporally disjoint from another.
    #[must_use]
    pub fn is_disjoint(self, other: Self) -> bool {
        self.interval.is_disjoint(other.interval)
    }

    /// Returns the positive-duration intersection.
    #[must_use]
    pub fn intersection(self, other: Self) -> Option<TimeInterval> {
        self.interval.intersection(other.interval)
    }
}

impl Ord for ScheduledInterval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start()
            .cmp(&other.start())
            .then_with(|| self.end().cmp(&other.end()))
            .then_with(|| self.operation_id.cmp(&other.operation_id))
    }
}

impl PartialOrd for ScheduledInterval {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// =============================================================================
// IntervalConflict
// =============================================================================

/// A deterministic description of a temporal scheduling conflict.
///
/// This type deliberately contains no resource information.
///
/// Resource conflicts belong to the resource-aware schedule layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntervalConflict {
    first: ScheduledInterval,
    second: ScheduledInterval,
}

impl IntervalConflict {
    /// Creates a canonical conflict pair.
    ///
    /// The two operations must have positive-duration temporal overlap.
    ///
    /// The stored ordering is deterministic.
    pub fn new(
        first: ScheduledInterval,
        second: ScheduledInterval,
    ) -> Option<Self> {
        if first.operation_id() == second.operation_id() {
            return None;
        }

        if !first.overlaps(second) {
            return None;
        }

        if first <= second {
            Some(Self { first, second })
        } else {
            Some(Self {
                first: second,
                second: first,
            })
        }
    }

    /// Returns the first operation in canonical conflict order.
    #[must_use]
    pub const fn first(self) -> ScheduledInterval {
        self.first
    }

    /// Returns the second operation in canonical conflict order.
    #[must_use]
    pub const fn second(self) -> ScheduledInterval {
        self.second
    }

    /// Returns the positive-duration temporal intersection.
    #[must_use]
    pub fn intersection(self) -> Option<TimeInterval> {
        self.first.intersection(self.second)
    }
}

impl Ord for IntervalConflict {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.first
            .cmp(&other.first)
            .then_with(|| self.second.cmp(&other.second))
    }
}

impl PartialOrd for IntervalConflict {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// =============================================================================
// IntervalIndex
// =============================================================================

/// Deterministic index of scheduled operation intervals.
///
/// The index is deliberately independent of resource allocation.
///
/// It answers temporal questions such as:
///
/// - Which operations overlap this time span?
/// - Does an operation already have a placement?
/// - Which operations overlap this operation?
/// - What is the deterministic temporal ordering?
///
/// It does NOT answer:
///
/// - Do two operations use the same qubit?
/// - Do two operations use the same channel?
/// - Is a target topology valid?
/// - Is the schedule hardware executable?
///
/// Those are handled by resource/capability/hardware layers.
#[derive(Debug, Clone, Default)]
pub struct IntervalIndex {
    /// Intervals grouped by start time.
    ///
    /// Multiple operations may begin at exactly the same semantic time.
    by_start: BTreeMap<TimePoint, Vec<ScheduledInterval>>,

    /// Direct operation lookup.
    ///
    /// This makes duplicate detection and exact removal independent of the
    /// temporal ordering structure.
    by_operation: BTreeMap<OperationId, ScheduledInterval>,

    /// Number of indexed intervals.
    ///
    /// This is a collection count, not a semantic machine-size limit.
    len: usize,
}

impl IntervalIndex {
    /// Creates an empty interval index.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of indexed intervals.
    ///
    /// This is the number of inserted schedule entries, not a hardware limit.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the index contains no intervals.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Clears all indexed intervals.
    pub fn clear(&mut self) {
        self.by_start.clear();
        self.by_operation.clear();
        self.len = 0;
    }

    /// Inserts a scheduled interval.
    ///
    /// Each `OperationId` may have at most one scheduling interval in this
    /// index.
    ///
    /// This restriction matches the canonical `Schedule` concept where an
    /// operation has one scheduled placement. If a semantic operation needs
    /// multiple temporal events, those events should be represented as
    /// separate scheduled operations or a higher-level structured operation.
    pub fn insert(
        &mut self,
        interval: ScheduledInterval,
    ) -> IntervalResult<()> {
        let operation_id = interval.operation_id();

        if self.by_operation.contains_key(&operation_id) {
            return Err(IntervalError::DuplicateOperation {
                operation: operation_id,
            });
        }

        self.by_operation
            .insert(operation_id, interval);

        let bucket = self
            .by_start
            .entry(interval.start())
            .or_default();

        bucket.push(interval);

        // Maintain deterministic ordering inside equal-start buckets.
        bucket.sort();

        self.len += 1;

        debug_assert!(self.invariants_hold());

        Ok(())
    }

    /// Inserts many intervals atomically.
    ///
    /// If any interval would violate the duplicate-operation invariant, no
    /// interval from this call is retained.
    ///
    /// The caller's iterator may be any size supported by the host process.
    /// No architectural size is imposed by this API.
    pub fn insert_all<I>(
        &mut self,
        intervals: I,
    ) -> IntervalResult<()>
    where
        I: IntoIterator<Item = ScheduledInterval>,
    {
        let mut staged = Vec::new();

        for interval in intervals {
            if self
                .by_operation
                .contains_key(&interval.operation_id())
            {
                return Err(IntervalError::DuplicateOperation {
                    operation: interval.operation_id(),
                });
            }

            for existing in &staged {
                if existing.operation_id() == interval.operation_id() {
                    return Err(IntervalError::DuplicateOperation {
                        operation: interval.operation_id(),
                    });
                }
            }

            staged.push(interval);
        }

        // Only mutate the index after validation of the complete input.
        for interval in staged {
            self.insert(interval)?;
        }

        Ok(())
    }

    /// Removes an operation's interval.
    ///
    /// Returns the removed interval when present.
    pub fn remove(
        &mut self,
        operation_id: OperationId,
    ) -> Option<ScheduledInterval> {
        let interval = self.by_operation.remove(&operation_id)?;

        let remove_bucket = if let Some(bucket) =
            self.by_start.get_mut(&interval.start())
        {
            if let Some(position) = bucket
                .iter()
                .position(|candidate| {
                    candidate.operation_id() == operation_id
                })
            {
                bucket.remove(position);
            }

            bucket.is_empty()
        } else {
            false
        };

        if remove_bucket {
            self.by_start.remove(&interval.start());
        }

        self.len -= 1;

        debug_assert!(self.invariants_hold());

        Some(interval)
    }

    /// Returns an operation's interval.
    #[must_use]
    pub fn get(
        &self,
        operation_id: OperationId,
    ) -> Option<ScheduledInterval> {
        self.by_operation.get(&operation_id).copied()
    }

    /// Returns whether the operation is indexed.
    #[must_use]
    pub fn contains(&self, operation_id: OperationId) -> bool {
        self.by_operation.contains_key(&operation_id)
    }

    /// Returns all intervals in deterministic order.
    ///
    /// The returned vector is sorted by:
    ///
    /// 1. start;
    /// 2. end;
    /// 3. operation identity.
    ///
    /// This is a snapshot and does not expose internal storage.
    #[must_use]
    pub fn intervals(&self) -> Vec<ScheduledInterval> {
        let mut result = Vec::with_capacity(self.len);

        for bucket in self.by_start.values() {
            result.extend(bucket.iter().copied());
        }

        // Buckets are already ordered by start because BTreeMap is ordered.
        // Equal-start buckets are maintained in canonical order.
        result
    }

    /// Returns intervals that overlap the supplied temporal interval.
    ///
    /// Zero-duration intervals are not considered overlapping because the
    /// canonical timing model defines overlap as positive-duration overlap.
    #[must_use]
    pub fn overlapping(
        &self,
        query: TimeInterval,
    ) -> Vec<ScheduledInterval> {
        if query.is_empty() || self.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::new();

        // Any interval beginning at or after query.end cannot overlap the
        // half-open query.
        //
        // Intervals beginning before query.start may still overlap because
        // their end can extend into the query.
        for (_, bucket) in self.by_start.range(..query.end()) {
            for interval in bucket {
                if interval.interval().overlaps(query) {
                    result.push(*interval);
                }
            }
        }

        result.sort();

        result
    }

    /// Returns intervals that overlap the supplied scheduled interval.
    #[must_use]
    pub fn overlapping_interval(
        &self,
        query: ScheduledInterval,
    ) -> Vec<ScheduledInterval> {
        self.overlapping(query.interval())
            .into_iter()
            .filter(|candidate| {
                candidate.operation_id() != query.operation_id()
            })
            .collect()
    }

    /// Returns whether any indexed interval overlaps the query.
    #[must_use]
    pub fn has_overlap(&self, query: TimeInterval) -> bool {
        if query.is_empty() {
            return false;
        }

        self.by_start
            .range(..query.end())
            .any(|(_, bucket)| {
                bucket
                    .iter()
                    .any(|interval| interval.interval().overlaps(query))
            })
    }

    /// Returns whether another operation conflicts temporally with an
    /// operation already in the index.
    #[must_use]
    pub fn conflicts_with(
        &self,
        query: ScheduledInterval,
    ) -> Vec<IntervalConflict> {
        self.overlapping_interval(query)
            .into_iter()
            .filter_map(|candidate| {
                IntervalConflict::new(query, candidate)
            })
            .collect()
    }

    /// Returns all pairwise temporal conflicts in deterministic order.
    ///
    /// This operation is intentionally explicit because it can be
    /// `O(n²)` for highly overlapping schedules.
    ///
    /// It is therefore not used automatically during insertion.
    #[must_use]
    pub fn all_conflicts(&self) -> Vec<IntervalConflict> {
        let intervals = self.intervals();
        let mut conflicts = Vec::new();

        for left in 0..intervals.len() {
            for right in (left + 1)..intervals.len() {
                if let Some(conflict) =
                    IntervalConflict::new(
                        intervals[left],
                        intervals[right],
                    )
                {
                    conflicts.push(conflict);
                }
            }
        }

        conflicts.sort();
        conflicts.dedup();

        conflicts
    }

    /// Returns the earliest start time in the index.
    #[must_use]
    pub fn earliest_start(&self) -> Option<TimePoint> {
        self.by_start.keys().next().copied()
    }

    /// Returns the latest exclusive end time.
    #[must_use]
    pub fn latest_end(&self) -> Option<TimePoint> {
        self.by_operation
            .values()
            .map(ScheduledInterval::end)
            .max()
    }

    /// Returns the semantic span of all indexed intervals.
    ///
    /// The span is:
    ///
    /// ```text
    /// [minimum start, maximum end)
    /// ```
    ///
    /// An empty index has no span.
    #[must_use]
    pub fn span(&self) -> Option<TimeInterval> {
        let start = self.earliest_start()?;
        let end = self.latest_end()?;

        TimeInterval::new(start, end).ok()
    }

    /// Iterates over intervals in deterministic order.
    ///
    /// This method avoids exposing internal B-tree structures.
    ///
    /// The iterator is backed by a snapshot so callers cannot invalidate
    /// traversal through internal mutation.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = ScheduledInterval> + '_ {
        self.by_start
            .values()
            .flat_map(|bucket| bucket.iter().copied())
    }

    /// Checks the internal structural invariants.
    ///
    /// This method is primarily intended for tests and debug assertions.
    #[must_use]
    pub fn validate(&self) -> bool {
        self.invariants_hold()
    }

    fn invariants_hold(&self) -> bool {
        if self.len != self.by_operation.len() {
            return false;
        }

        let mut bucket_count = 0usize;

        for (start, bucket) in &self.by_start {
            let mut previous = None;

            for interval in bucket {
                if interval.start() != *start {
                    return false;
                }

                if let Some(previous_interval) = previous {
                    if previous_interval > *interval {
                        return false;
                    }
                }

                if self
                    .by_operation
                    .get(&interval.operation_id())
                    .copied()
                    != Some(*interval)
                {
                    return false;
                }

                previous = Some(*interval);
                bucket_count += 1;
            }
        }

        bucket_count == self.len
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn point(value: u128) -> TimePoint {
        TimePoint::from_attoseconds(value)
    }

    fn interval(
        operation_id: u64,
        start: u128,
        end: u128,
    ) -> ScheduledInterval {
        ScheduledInterval::from_bounds(
            operation(operation_id),
            point(start),
            point(end),
        )
        .expect("test interval must be valid")
    }

    #[test]
    fn constructs_valid_scheduled_interval() {
        let value = interval(1, 10, 20);

        assert_eq!(value.operation_id(), operation(1));
        assert_eq!(value.start(), point(10));
        assert_eq!(value.end(), point(20));
        assert!(value.is_non_empty());
        assert!(!value.is_zero_duration());
    }

    #[test]
    fn rejects_reversed_bounds() {
        let result = ScheduledInterval::from_bounds(
            operation(1),
            point(20),
            point(10),
        );

        assert!(matches!(
            result,
            Err(IntervalError::InvalidInterval { .. })
        ));
    }

    #[test]
    fn accepts_zero_duration_interval() {
        let value = interval(1, 10, 10);

        assert!(value.is_zero_duration());
        assert!(!value.is_non_empty());
    }

    #[test]
    fn adjacent_intervals_do_not_overlap() {
        let first = interval(1, 0, 10);
        let second = interval(2, 10, 20);

        assert!(!first.overlaps(second));
        assert!(first.is_adjacent(second));
    }

    #[test]
    fn overlapping_intervals_overlap() {
        let first = interval(1, 0, 10);
        let second = interval(2, 9, 20);

        assert!(first.overlaps(second));
        assert!(!first.is_adjacent(second));

        let intersection = first
            .intersection(second)
            .expect("must intersect");

        assert_eq!(intersection.start(), point(9));
        assert_eq!(intersection.end(), point(10));
    }

    #[test]
    fn equal_start_intervals_are_deterministically_ordered() {
        let first = interval(1, 10, 20);
        let second = interval(2, 10, 15);

        assert!(second < first);
    }

    #[test]
    fn inserts_and_retrieves() {
        let mut index = IntervalIndex::new();

        index
            .insert(interval(1, 0, 10))
            .expect("insert must succeed");

        assert_eq!(index.len(), 1);
        assert!(!index.is_empty());
        assert!(index.contains(operation(1)));
        assert_eq!(
            index.get(operation(1)),
            Some(interval(1, 0, 10))
        );
        assert!(index.validate());
    }

    #[test]
    fn rejects_duplicate_operation() {
        let mut index = IntervalIndex::new();

        index
            .insert(interval(1, 0, 10))
            .expect("first insert must succeed");

        let result = index.insert(interval(1, 20, 30));

        assert!(matches!(
            result,
            Err(IntervalError::DuplicateOperation { .. })
        ));

        assert_eq!(index.len(), 1);
        assert!(index.validate());
    }

    #[test]
    fn removal_preserves_invariants() {
        let mut index = IntervalIndex::new();

        index
            .insert(interval(1, 0, 10))
            .expect("insert");
        index
            .insert(interval(2, 10, 20))
            .expect("insert");

        let removed = index.remove(operation(1));

        assert_eq!(removed, Some(interval(1, 0, 10)));
        assert!(!index.contains(operation(1)));
        assert_eq!(index.len(), 1);
        assert!(index.validate());
    }

    #[test]
    fn missing_removal_returns_none() {
        let mut index = IntervalIndex::new();

        assert_eq!(index.remove(operation(99)), None);
    }

    #[test]
    fn ordered_iteration_is_deterministic() {
        let mut index = IntervalIndex::new();

        index
            .insert(interval(3, 20, 30))
            .expect("insert");
        index
            .insert(interval(1, 0, 10))
            .expect("insert");
        index
            .insert(interval(2, 10, 20))
            .expect("insert");

        let values: Vec<_> = index.iter().collect();

        assert_eq!(
            values,
            vec![
                interval(1, 0, 10),
                interval(2, 10, 20),
                interval(3, 20, 30),
            ]
        );
    }

    #[test]
    fn overlapping_query_excludes_adjacent_intervals() {
        let mut index = IntervalIndex::new();

        index
            .insert(interval(1, 0, 10))
            .expect("insert");
        index
            .insert(interval(2, 10, 20))
            .expect("insert");
        index
            .insert(interval(3, 5, 15))
            .expect("insert");

        let query = TimeInterval::new(
            point(10),
            point(20),
        )
        .expect("valid query");

        let values = index.overlapping(query);

        assert_eq!(
            values,
            vec![interval(2, 10, 20), interval(3, 5, 15)]
        );
    }

    #[test]
    fn zero_duration_query_has_no_overlap() {
        let mut index = IntervalIndex::new();

        index
            .insert(interval(1, 0, 10))
            .expect("insert");

        let query = TimeInterval::at(point(5));

        assert!(index.overlapping(query).is_empty());
        assert!(!index.has_overlap(query));
    }

    #[test]
    fn zero_duration_event_does_not_create_conflict() {
        let mut index = IntervalIndex::new();

        index
            .insert(interval(1, 0, 10))
            .expect("insert");

        let event = interval(2, 5, 5);

        assert!(index.conflicts_with(event).is_empty());
    }

    #[test]
    fn interval_specific_query_excludes_same_operation() {
        let mut index = IntervalIndex::new();

        let value = interval(1, 0, 10);

        index.insert(value).expect("insert");

        assert!(
            index
                .overlapping_interval(value)
                .is_empty()
        );
    }

    #[test]
    fn all_conflicts_are_deterministic() {
        let mut index = IntervalIndex::new();

        index
            .insert(interval(1, 0, 10))
            .expect("insert");
        index
            .insert(interval(2, 5, 15))
            .expect("insert");
        index
            .insert(interval(3, 10, 20))
            .expect("insert");
        index
            .insert(interval(4, 20, 30))
            .expect("insert");

        let conflicts = index.all_conflicts();

        assert_eq!(conflicts.len(), 2);

        assert_eq!(
            conflicts[0].first().operation_id(),
            operation(1)
        );
        assert_eq!(
            conflicts[0].second().operation_id(),
            operation(2)
        );

        assert_eq!(
            conflicts[1].first().operation_id(),
            operation(2)
        );
        assert_eq!(
            conflicts[1].second().operation_id(),
            operation(3)
        );
    }

    #[test]
    fn earliest_and_latest_times_are_correct() {
        let mut index = IntervalIndex::new();

        index
            .insert(interval(1, 100, 200))
            .expect("insert");
        index
            .insert(interval(2, 10, 50))
            .expect("insert");
        index
            .insert(interval(3, 300, 400))
            .expect("insert");

        assert_eq!(
            index.earliest_start(),
            Some(point(10))
        );

        assert_eq!(
            index.latest_end(),
            Some(point(400))
        );

        assert_eq!(
            index.span(),
            Some(
                TimeInterval::new(
                    point(10),
                    point(400)
                )
                .expect("valid span")
            )
        );
    }

    #[test]
    fn empty_index_has_no_span() {
        let index = IntervalIndex::new();

        assert_eq!(index.earliest_start(), None);
        assert_eq!(index.latest_end(), None);
        assert_eq!(index.span(), None);
    }

    #[test]
    fn clear_removes_everything() {
        let mut index = IntervalIndex::new();

        index
            .insert(interval(1, 0, 10))
            .expect("insert");
        index
            .insert(interval(2, 10, 20))
            .expect("insert");

        index.clear();

        assert!(index.is_empty());
        assert_eq!(index.len(), 0);
        assert!(!index.contains(operation(1)));
        assert!(!index.contains(operation(2)));
        assert!(index.validate());
    }

    #[test]
    fn insert_all_is_atomic_on_duplicate() {
        let mut index = IntervalIndex::new();

        index
            .insert(interval(1, 0, 10))
            .expect("insert");

        let result = index.insert_all(vec![
            interval(2, 10, 20),
            interval(2, 30, 40),
            interval(3, 40, 50),
        ]);

        assert!(matches!(
            result,
            Err(IntervalError::DuplicateOperation { .. })
        ));

        assert_eq!(index.len(), 1);
        assert!(!index.contains(operation(2)));
        assert!(!index.contains(operation(3)));
        assert!(index.validate());
    }

    #[test]
    fn insert_all_accepts_unique_operations() {
        let mut index = IntervalIndex::new();

        index
            .insert_all(vec![
                interval(1, 0, 10),
                interval(2, 10, 20),
                interval(3, 20, 30),
            ])
            .expect("batch insert must succeed");

        assert_eq!(index.len(), 3);
        assert!(index.validate());
    }

    #[test]
    fn sparse_large_identifiers_are_supported() {
        let mut index = IntervalIndex::new();

        index
            .insert(interval(u64::MAX, 0, 10))
            .expect("maximum representable operation identity must work");

        assert!(index.contains(operation(u64::MAX)));
        assert_eq!(index.len(), 1);
    }

    #[test]
    fn identical_intervals_have_deterministic_conflict_order() {
        let first = interval(1, 10, 20);
        let second = interval(2, 10, 20);

        let conflict =
            IntervalConflict::new(first, second)
                .expect("must conflict");

        assert_eq!(
            conflict.first().operation_id(),
            operation(1)
        );
        assert_eq!(
            conflict.second().operation_id(),
            operation(2)
        );
    }
}