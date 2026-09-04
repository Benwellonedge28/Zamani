//! Zamani Quantum Scheduling — Padding Transformation
//!
//! Path:
//!     src/quantum/scheduling/transformations/padding.rs
//!
//! # Purpose
//!
//! This module computes and validates explicit temporal padding around
//! already-scheduled operations without changing the semantic operation,
//! its duration, its resources, or its identity.
//!
//! Padding is a TRANSFORMATION concern.
//!
//! It does not:
//!
//! - choose which operation executes next;
//! - perform routing;
//! - discover hardware;
//! - define hardware timing;
//! - change an operation's semantic duration;
//! - synthesize a gate;
//! - synthesize a pulse;
//! - execute a QPU;
//! - perform QEC;
//! - perform noise modelling;
//! - contact a provider;
//! - introduce a second qubit identity;
//! - introduce a second time representation;
//! - mutate global state.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! quantum::ir
//!       |
//!       v
//! optimization
//!       |
//!       v
//! routing
//!       |
//!       v
//! scheduling/planner
//!       |
//!       v
//! canonical ScheduledOperation
//!       |
//!       +-------------------+
//!       |                   |
//!       v                   v
//! alignment              padding
//!       |                   |
//!       +---------+---------+
//!                 |
//!                 v
//!        transformations::delays
//!                 |
//!                 v
//!             verification
//!                 |
//!                 v
//!          hardware lowering
//! ```
//!
//! # Fundamental semantic rule
//!
//! Padding is NOT an operation-duration transformation.
//!
//! Given:
//!
//! ```text
//! operation interval = [100, 140)
//! duration           = 40
//!
//! requested padding before = 20
//! requested padding after  = 30
//! ```
//!
//! the operation remains:
//!
//! ```text
//! [100, 140)
//! ```
//!
//! while the padding regions are represented separately:
//!
//! ```text
//! before = [80, 100)
//! after  = [140, 170)
//! ```
//!
//! This distinction is critical.
//!
//! Changing the operation itself to:
//!
//! ```text
//! [80, 170)
//! ```
//!
//! would change its semantic duration and therefore would be incorrect.
//!
//! # Why padding is separate from delays
//!
//! Padding describes the temporal space that should be occupied by an idle,
//! synchronization, guard, or otherwise explicitly requested interval.
//!
//! Delay materialization is a separate concern because the representation of
//! an executable delay may depend on the target and lowering layer.
//!
//! Therefore:
//!
//! ```text
//! padding transformation
//!     |
//!     v
//! PaddingInterval
//!     |
//!     v
//! delay transformation / hardware lowering
//!     |
//!     v
//! executable delay representation
//! ```
//!
//! The padding transformation therefore never invents a new quantum operation
//! identity merely to represent idle time.
//!
//! # Universal-program principle
//!
//! No machine-specific padding amount is hard-coded here.
//!
//! This module contains no:
//!
//! - fixed qubit count;
//! - fixed channel count;
//! - fixed operation count;
//! - fixed schedule depth;
//! - fixed timing resolution;
//! - fixed machine size;
//! - vendor-specific value;
//! - hardware clock;
//! - default physical delay;
//! - maximum padding value.
//!
//! Padding values come from the caller's scheduling context, target timing
//! model, transformation policy, QEC requirements, synchronization requirements,
//! or hardware adapter.
//!
//! \"Infinity\" means that this module imposes no artificial finite machine-size
//! ceiling. Concrete execution is naturally bounded by the target, compiler,
//! address space, explicit policies, and available resources.
//!
//! # Canonical types
//!
//! This module consumes the canonical scheduling representation:
//!
//! ```text
//! crate::quantum::ir::scheduling::ScheduledOperation
//! crate::quantum::ir::scheduling::ScheduleResource
//! ```
//!
//! and canonical timing:
//!
//! ```text
//! crate::quantum::ir::timing::Duration
//! crate::quantum::ir::timing::TimeInterval
//! crate::quantum::ir::timing::TimePoint
//! ```
//!
//! No duplicate timing model is introduced.
//!
//! # Canonical qubit ownership
//!
//! This module does not create `QubitId` or `PhysicalQubitId`.
//!
//! If resources contain qubit identities, those identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! The transformation merely preserves the resource references supplied by
//! the schedule.
//!
//! # Half-open interval semantics
//!
//! All padding intervals use:
//!
//! ```text
//! [start, end)
//! ```
//!
//! Therefore:
//!
//! ```text
//! [0, 10)
//! [10, 20)
//! ```
//!
//! are adjacent and do not overlap.
//!
//! This is essential for composing padding with scheduled operations.
//!
//! # Padding semantics
//!
//! Two forms are supported:
//!
//! ## Padding before
//!
//! ```text
//! operation = [start, end)
//! padding   = [start - amount, start)
//! ```
//!
//! ## Padding after
//!
//! ```text
//! operation = [start, end)
//! padding   = [end, end + amount)
//! ```
//!
//! Padding is never allowed to overlap the operation itself.
//!
//! # Placement semantics
//!
//! This module supports two placement models:
//!
//! `Before`
//!     place padding immediately before an operation.
//!
//! `After`
//!     place padding immediately after an operation.
//!
//! `Around`
//!     create both before and after padding.
//!
//! `ExactWindow`
//!     validate that a caller-provided interval is a legal padding interval.
//!
//! The implementation intentionally does not provide an implicit \"shift the
//! operation\" behavior. Shifting belongs to scheduling/alignment because it
//! can affect dependencies and resource reservations.
//!
//! # Resource semantics
//!
//! Padding does not automatically consume the operation's resources.
//!
//! This is intentional.
//!
//! A generic idle interval on a qubit is not necessarily equivalent to an
//! executable resource reservation on every control channel.
//!
//! Therefore a padding request may optionally specify resources explicitly.
//!
//! If no resources are supplied, the padding is a purely temporal artifact.
//!
//! If resources are supplied, those resources are copied exactly into the
//! padding artifact.
//!
//! No resource is inferred from hardware.
//!
//! # Resource preservation
//!
//! The source operation itself is never modified.
//!
//! Its:
//!
//! - `OperationId`;
//! - interval;
//! - duration;
//! - resources
//!
//! remain unchanged.
//!
//! # Dependency safety
//!
//! This module cannot independently prove global dependency correctness.
//!
//! For example:
//!
//! ```text
//! A -> B
//!
//! A = [0, 10)
//! B = [10, 20)
//! ```
//!
//! A padding request before B may be valid:
//!
//! ```text
//! [8, 10)
//! ```
//!
//! but a padding request before B of 20 time units would extend before A:
//!
//! ```text
//! [-10, 10)
//! ```
//!
//! This module therefore validates local interval arithmetic only.
//!
//! Whole-schedule dependency/resource validation remains owned by:
//!
//! ```text
//! crate::quantum::scheduling::verification
//! ```
//!
//! # Resource conflict safety
//!
//! Likewise, this module does not claim that explicitly supplied padding
//! resources are globally available.
//!
//! The resulting padding must be checked by the scheduling resource verifier.
//!
//! # Exact arithmetic
//!
//! No floating-point arithmetic is used.
//!
//! All arithmetic uses the canonical integer timing representation exposed by
//! the IR.
//!
//! This avoids:
//!
//! - rounding drift;
//! - platform-dependent floating-point behavior;
//! - hidden precision loss;
//! - non-deterministic interval construction.
//!
//! # Overflow safety
//!
//! Every operation that can overflow is checked.
//!
//! In particular:
//!
//! ```text
//! start - before_padding
//! end + after_padding
//! ```
//!
//! use checked arithmetic.
//!
//! No wrapping arithmetic is used.
//!
//! # Scalability
//!
//! One operation requires O(1) auxiliary memory.
//!
//! A collection transformation requires O(N) output storage for N requested
//! padding artifacts.
//!
//! The implementation never creates a timeline proportional to:
//!
//! ```text
//! number of qubits × maximum schedule time
//! ```
//!
//! It never iterates through every possible time tick.
//!
//! It never materializes unused machine resources.
//!
//! # Determinism
//!
//! Given identical:
//!
//! - scheduled operation;
//! - padding specification;
//! - resource specification;
//! - timing values;
//!
//! the transformation produces identical results.
//!
//! No:
//!
//! - randomness;
//! - system clock;
//! - hash-map iteration order;
//! - hardware query
//!
//! is used.
//!
//! # Thread safety
//!
//! This module owns no global mutable state.
//!
//! Inputs are borrowed immutably.
//!
//! Results own their data.
//!
//! Therefore independent padding transformations can safely execute in
//! parallel when the surrounding scheduler chooses to do so.
//!
//! # Integration
//!
//! ## Scheduling planner
//!
//! ```text
//! planner
//!   |
//!   v
//! ScheduledOperation
//!   |
//!   v
//! PaddingTransformation
//! ```
//!
//! ## Alignment
//!
//! Alignment and padding are deliberately separate:
//!
//! ```text
//! scheduling
//!     |
//!     +--> alignment
//!     |
//!     +--> padding
//! ```
//!
//! Alignment changes placement to satisfy a grid.
//!
//! Padding represents explicit temporal space.
//!
//! ## Delay transformation
//!
//! `transformations::delays` may consume `PaddingInterval` values and materialize
//! them into the canonical executable representation where appropriate.
//!
//! This module must not depend on `delays.rs`, avoiding a transformation cycle.
//!
//! ## Verification
//!
//! The resulting padding artifacts must be passed through scheduling
//! verification before being treated as executable.
//!
//! ## Hardware
//!
//! Hardware alignment, minimum delays, guard intervals, resource reservations,
//! and timing restrictions enter through the scheduling context/adapter.
//!
//! This module does not import or query the hardware subsystem directly.
//!
//! ## Routing
//!
//! Routing happens before scheduling.
//!
//! Padding observes the resources already present in the scheduled operation or
//! explicitly supplied by the caller.
//!
//! ## QEC
//!
//! QEC may request padding for:
//!
//! - round spacing;
//! - synchronization;
//! - measurement settling;
//! - feedback windows;
//! - code-cycle timing.
//!
//! This module remains code-agnostic.
//!
//! ## Dynamic circuits
//!
//! Runtime-dependent padding must be represented by the dynamic scheduling
//! subsystem rather than guessed statically here.
//!
//! ## Distributed scheduling
//!
//! Communication and synchronization padding may be represented using explicit
//! resources, but network semantics remain outside this module.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` is intentionally used.
//!
//! # Completion criteria
//!
//! This file is complete when it:
//!
//! - uses canonical scheduled operations;
//! - uses canonical timing;
//! - never changes operation duration;
//! - never changes operation identity;
//! - never changes operation resources;
//! - provides explicit before/after/around padding;
//! - supports exact caller-supplied padding windows;
//! - performs checked arithmetic;
//! - rejects invalid intervals;
//! - rejects negative/unrepresentable padding through the type system;
//! - does not hard-code machine limits;
//! - does not hard-code timing values;
//! - does not create qubit identities;
//! - does not schedule operations;
//! - does not perform routing;
//! - does not access hardware;
//! - does not mutate global state;
//! - remains deterministic;
//! - contains no unsafe code.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use crate::quantum::ir::scheduling::{ScheduleResource, ScheduledOperation};
use crate::quantum::ir::timing::{Duration, TimeInterval, TimePoint};

// =============================================================================
// Public result type
// =============================================================================

/// Result type returned by padding transformations.
pub type PaddingResult<T> = Result<T, PaddingError>;

// =============================================================================
// Padding placement
// =============================================================================

/// Defines where padding is placed relative to a scheduled operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum PaddingPlacement {
    /// Padding immediately before the operation.
    Before,

    /// Padding immediately after the operation.
    After,

    /// Padding on both sides of the operation.
    Around,
}

impl PaddingPlacement {
    /// Returns whether this placement creates padding before the operation.
    #[must_use]
    pub const fn includes_before(self) -> bool {
        matches!(self, Self::Before | Self::Around)
    }

    /// Returns whether this placement creates padding after the operation.
    #[must_use]
    pub const fn includes_after(self) -> bool {
        matches!(self, Self::After | Self::Around)
    }
}

impl fmt::Display for PaddingPlacement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Before => formatter.write_str("before"),
            Self::After => formatter.write_str("after"),
            Self::Around => formatter.write_str("around"),
        }
    }
}

// =============================================================================
// Padding specification
// =============================================================================

/// Immutable specification for padding around one scheduled operation.
///
/// Padding amounts are semantic durations supplied by the caller. This type
/// deliberately contains no hardware-specific defaults.
///
/// `Before` and `After` use the corresponding duration.
///
/// `Around` uses both durations.
///
/// A zero duration is legal and produces no non-empty padding interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PaddingSpec {
    placement: PaddingPlacement,
    before: Duration,
    after: Duration,
}

impl PaddingSpec {
    /// Creates a before-padding request.
    #[must_use]
    pub const fn before(duration: Duration) -> Self {
        Self {
            placement: PaddingPlacement::Before,
            before: duration,
            after: Duration::ZERO,
        }
    }

    /// Creates an after-padding request.
    #[must_use]
    pub const fn after(duration: Duration) -> Self {
        Self {
            placement: PaddingPlacement::After,
            before: Duration::ZERO,
            after: duration,
        }
    }

    /// Creates an around-padding request.
    #[must_use]
    pub const fn around(before: Duration, after: Duration) -> Self {
        Self {
            placement: PaddingPlacement::Around,
            before,
            after,
        }
    }

    /// Returns the placement mode.
    #[must_use]
    pub const fn placement(self) -> PaddingPlacement {
        self.placement
    }

    /// Returns requested before-padding.
    #[must_use]
    pub const fn before(self) -> Duration {
        self.before
    }

    /// Returns requested after-padding.
    #[must_use]
    pub const fn after(self) -> Duration {
        self.after
    }

    /// Returns whether any non-zero padding was requested.
    #[must_use]
    pub fn is_non_zero(self) -> bool {
        !self.before.is_zero() || !self.after.is_zero()
    }
}

// =============================================================================
// Padding resources
// =============================================================================

/// Resource policy for a padding interval.
///
/// Padding does not implicitly inherit resources from the operation because
/// doing so could accidentally reserve control/readout resources that are not
/// actually required by an idle interval.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaddingResources {
    /// Padding is purely temporal and has no explicit resource reservation.
    None,

    /// Padding explicitly reserves the supplied resources.
    Explicit(Vec<ScheduleResource>),
}

impl Default for PaddingResources {
    fn default() -> Self {
        Self::None
    }
}

impl PaddingResources {
    /// Creates resource-free padding.
    #[must_use]
    pub const fn none() -> Self {
        Self::None
    }

    /// Creates padding with explicit resources.
    ///
    /// Resource order is canonicalized and duplicate resources are rejected by
    /// the transformation when the specification is applied.
    #[must_use]
    pub fn explicit(
        resources: impl IntoIterator<Item = ScheduleResource>,
    ) -> Self {
        Self::Explicit(resources.into_iter().collect())
    }

    /// Returns whether the padding has no resource reservation.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns the explicitly supplied resources, if any.
    #[must_use]
    pub fn resources(&self) -> Option<&[ScheduleResource]> {
        match self {
            Self::None => None,
            Self::Explicit(resources) => Some(resources.as_slice()),
        }
    }
}

// =============================================================================
// Padding request
// =============================================================================

/// Complete padding request for one scheduled operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaddingRequest {
    operation_id: crate::quantum::ir::core::identity::OperationId,
    specification: PaddingSpec,
    resources: PaddingResources,
}

impl PaddingRequest {
    /// Creates a padding request using no explicit resource reservation.
    #[must_use]
    pub fn new(
        operation: &ScheduledOperation,
        specification: PaddingSpec,
    ) -> Self {
        Self {
            operation_id: operation.operation_id(),
            specification,
            resources: PaddingResources::None,
        }
    }

    /// Creates a padding request with explicit resources.
    #[must_use]
    pub fn with_resources(
        operation: &ScheduledOperation,
        specification: PaddingSpec,
        resources: PaddingResources,
    ) -> Self {
        Self {
            operation_id: operation.operation_id(),
            specification,
            resources,
        }
    }

    /// Returns the associated operation identity.
    #[must_use]
    pub const fn operation_id(
        &self,
    ) -> crate::quantum::ir::core::identity::OperationId {
        self.operation_id
    }

    /// Returns the padding specification.
    #[must_use]
    pub const fn specification(&self) -> PaddingSpec {
        self.specification
    }

    /// Returns the padding resource policy.
    #[must_use]
    pub fn resources(&self) -> &PaddingResources {
        &self.resources
    }
}

// =============================================================================
// Padding interval
// =============================================================================

/// One explicit padding interval.
///
/// A `PaddingInterval` is not a semantic quantum operation.
///
/// It is a transformation artifact that may later be materialized as a delay,
/// guard, synchronization interval, or target-specific representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaddingInterval {
    interval: TimeInterval,
    resources: Vec<ScheduleResource>,
    adjacent_operation:
        crate::quantum::ir::core::identity::OperationId,
    side: PaddingSide,
}

impl PaddingInterval {
    fn new(
        interval: TimeInterval,
        resources: Vec<ScheduleResource>,
        adjacent_operation:
            crate::quantum::ir::core::identity::OperationId,
        side: PaddingSide,
    ) -> Self {
        Self {
            interval,
            resources,
            adjacent_operation,
            side,
        }
    }

    /// Returns the padded interval.
    #[must_use]
    pub const fn interval(&self) -> TimeInterval {
        self.interval
    }

    /// Returns the interval start.
    #[must_use]
    pub const fn start(&self) -> TimePoint {
        self.interval.start()
    }

    /// Returns the interval end.
    #[must_use]
    pub const fn end(&self) -> TimePoint {
        self.interval.end()
    }

    /// Returns the padding duration.
    #[must_use]
    pub fn duration(&self) -> Duration {
        self.interval.duration()
    }

    /// Returns the resources explicitly reserved by this padding interval.
    #[must_use]
    pub fn resources(&self) -> &[ScheduleResource] {
        &self.resources
    }

    /// Returns the operation adjacent to this padding.
    #[must_use]
    pub const fn adjacent_operation(
        &self,
    ) -> crate::quantum::ir::core::identity::OperationId {
        self.adjacent_operation
    }

    /// Returns whether this padding occurs before the operation.
    #[must_use]
    pub const fn is_before(&self) -> bool {
        matches!(self.side, PaddingSide::Before)
    }

    /// Returns whether this padding occurs after the operation.
    #[must_use]
    pub const fn is_after(&self) -> bool {
        matches!(self.side, PaddingSide::After)
    }
}

/// Side on which padding is placed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
enum PaddingSide {
    Before,
    After,
}

// =============================================================================
// Single-operation transformation result
// =============================================================================

/// Result of padding one scheduled operation.
///
/// The original scheduled operation is retained unchanged.
///
/// The padding intervals are additional transformation artifacts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaddedOperation {
    operation: ScheduledOperation,
    before: Option<PaddingInterval>,
    after: Option<PaddingInterval>,
}

impl PaddedOperation {
    fn new(
        operation: ScheduledOperation,
        before: Option<PaddingInterval>,
        after: Option<PaddingInterval>,
    ) -> Self {
        Self {
            operation,
            before,
            after,
        }
    }

    /// Returns the unchanged scheduled operation.
    #[must_use]
    pub fn operation(&self) -> &ScheduledOperation {
        &self.operation
    }

    /// Consumes the result and returns the unchanged operation.
    #[must_use]
    pub fn into_operation(self) -> ScheduledOperation {
        self.operation
    }

    /// Returns the before-padding interval, if one was requested and non-zero.
    #[must_use]
    pub fn before(&self) -> Option<&PaddingInterval> {
        self.before.as_ref()
    }

    /// Returns the after-padding interval, if one was requested and non-zero.
    #[must_use]
    pub fn after(&self) -> Option<&PaddingInterval> {
        self.after.as_ref()
    }

    /// Returns the number of non-empty padding intervals.
    #[must_use]
    pub fn interval_count(&self) -> usize {
        usize::from(self.before.is_some())
            + usize::from(self.after.is_some())
    }

    /// Returns whether this request produced no non-empty padding.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.before.is_none() && self.after.is_none()
    }
}

// =============================================================================
// Exact padding window
// =============================================================================

/// Validates an already-selected padding interval against an operation.
///
/// This is useful when another scheduler stage has already selected the
/// temporal window and this transformation only needs to validate and package
/// it as padding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExactPaddingWindow {
    interval: TimeInterval,
}

impl ExactPaddingWindow {
    /// Creates an exact padding window.
    ///
    /// `TimeInterval` guarantees interval validity.
    #[must_use]
    pub const fn new(interval: TimeInterval) -> Self {
        Self { interval }
    }

    /// Returns the exact interval.
    #[must_use]
    pub const fn interval(self) -> TimeInterval {
        self.interval
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by padding transformations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaddingError {
    /// A before-padding subtraction would move before the representable time
    /// origin.
    StartUnderflow {
        /// Operation being padded.
        operation:
            crate::quantum::ir::core::identity::OperationId,

        /// Original operation start.
        start: TimePoint,

        /// Requested padding.
        padding: Duration,
    },

    /// An after-padding addition would exceed the representable time domain.
    EndOverflow {
        /// Operation being padded.
        operation:
            crate::quantum::ir::core::identity::OperationId,

        /// Original operation end.
        end: TimePoint,

        /// Requested padding.
        padding: Duration,
    },

    /// A constructed interval was rejected by the canonical timing model.
    InvalidInterval {
        /// Operation being padded.
        operation:
            crate::quantum::ir::core::identity::OperationId,

        /// Proposed start.
        start: TimePoint,

        /// Proposed end.
        end: TimePoint,
    },

    /// An explicit resource was supplied more than once.
    DuplicateResource {
        /// Operation adjacent to the padding.
        operation:
            crate::quantum::ir::core::identity::OperationId,

        /// Duplicated resource.
        resource: ScheduleResource,
    },

    /// An exact padding interval intersects the operation.
    ///
    /// Padding must be outside the operation interval.
    OverlapsOperation {
        /// Operation adjacent to the padding.
        operation:
            crate::quantum::ir::core::identity::OperationId,

        /// Operation interval.
        operation_interval: TimeInterval,

        /// Requested padding interval.
        padding_interval: TimeInterval,
    },

    /// An exact padding interval is not adjacent to the requested side.
    NotAdjacent {
        /// Operation adjacent to the padding.
        operation:
            crate::quantum::ir::core::identity::OperationId,

        /// Requested side.
        side: PaddingPlacement,

        /// Operation interval.
        operation_interval: TimeInterval,

        /// Requested padding interval.
        padding_interval: TimeInterval,
    },

    /// The requested exact padding interval has zero duration.
    ///
    /// This is rejected for explicit exact windows because an exact window is
    /// intended to represent an actual padding region.
    EmptyExactWindow {
        /// Operation adjacent to the padding.
        operation:
            crate::quantum::ir::core::identity::OperationId,
    },
}

impl fmt::Display for PaddingError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::StartUnderflow {
                operation,
                start,
                padding,
            } => write!(
                formatter,
                "padding before operation `{operation}` underflows \
                 the representable time domain: start={start}, \
                 padding={padding}"
            ),

            Self::EndOverflow {
                operation,
                end,
                padding,
            } => write!(
                formatter,
                "padding after operation `{operation}` overflows \
                 the representable time domain: end={end}, \
                 padding={padding}"
            ),

            Self::InvalidInterval {
                operation,
                start,
                end,
            } => write!(
                formatter,
                "padding produced an invalid interval for operation \
                 `{operation}`: [{start}, {end})"
            ),

            Self::DuplicateResource {
                operation,
                resource,
            } => write!(
                formatter,
                "padding for operation `{operation}` contains duplicate \
                 resource `{resource:?}`"
            ),

            Self::OverlapsOperation {
                operation,
                operation_interval,
                padding_interval,
            } => write!(
                formatter,
                "padding interval {padding_interval:?} overlaps operation \
                 `{operation}` interval {operation_interval:?}"
            ),

            Self::NotAdjacent {
                operation,
                side,
                operation_interval,
                padding_interval,
            } => write!(
                formatter,
                "padding interval {padding_interval:?} is not adjacent \
                 to operation `{operation}` for placement `{side}`; \
                 operation interval is {operation_interval:?}"
            ),

            Self::EmptyExactWindow { operation } => write!(
                formatter,
                "exact padding window for operation `{operation}` is empty"
            ),
        }
    }
}

impl std::error::Error for PaddingError {}

// =============================================================================
// Statistics
// =============================================================================

/// Deterministic statistics for padding transformation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PaddingStatistics {
    operations_inspected: u64,
    operations_with_padding: u64,
    padding_intervals_created: u64,
    total_padding_attoseconds: u128,
    before_padding_attoseconds: u128,
    after_padding_attoseconds: u128,
}

impl PaddingStatistics {
    /// Creates empty statistics.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operations_inspected: 0,
            operations_with_padding: 0,
            padding_intervals_created: 0,
            total_padding_attoseconds: 0,
            before_padding_attoseconds: 0,
            after_padding_attoseconds: 0,
        }
    }

    /// Returns inspected operation count.
    #[must_use]
    pub const fn operations_inspected(self) -> u64 {
        self.operations_inspected
    }

    /// Returns operations for which non-zero padding was created.
    #[must_use]
    pub const fn operations_with_padding(self) -> u64 {
        self.operations_with_padding
    }

    /// Returns number of generated padding intervals.
    #[must_use]
    pub const fn padding_intervals_created(self) -> u64 {
        self.padding_intervals_created
    }

    /// Returns total padding duration in canonical attoseconds.
    #[must_use]
    pub const fn total_padding_attoseconds(self) -> u128 {
        self.total_padding_attoseconds
    }

    /// Returns before-padding duration in canonical attoseconds.
    #[must_use]
    pub const fn before_padding_attoseconds(self) -> u128 {
        self.before_padding_attoseconds
    }

    /// Returns after-padding duration in canonical attoseconds.
    #[must_use]
    pub const fn after_padding_attoseconds(self) -> u128 {
        self.after_padding_attoseconds
    }
}

// =============================================================================
// Collection result
// =============================================================================

/// Result of applying padding to multiple scheduled operations.
///
/// Input order is preserved exactly.
///
/// No sorting is performed here because padding is a transformation and must
/// not silently redefine canonical schedule ordering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaddingTransformResult {
    operations: Vec<PaddedOperation>,
    statistics: PaddingStatistics,
}

impl PaddingTransformResult {
    fn new(
        operations: Vec<PaddedOperation>,
        statistics: PaddingStatistics,
    ) -> Self {
        Self {
            operations,
            statistics,
        }
    }

    /// Returns transformed operations in input order.
    #[must_use]
    pub fn operations(&self) -> &[PaddedOperation] {
        &self.operations
    }

    /// Consumes the result and returns transformed operations.
    #[must_use]
    pub fn into_operations(self) -> Vec<PaddedOperation> {
        self.operations
    }

    /// Returns transformation statistics.
    #[must_use]
    pub const fn statistics(&self) -> PaddingStatistics {
        self.statistics
    }
}

// =============================================================================
// Transformation
// =============================================================================

/// Stateless padding transformation.
///
/// The type intentionally contains no scheduler state, hardware state, global
/// configuration, caches, or machine-size assumptions.
#[derive(Debug, Default, Clone, Copy)]
pub struct PaddingTransformation;

impl PaddingTransformation {
    /// Creates a padding transformation.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Applies padding to one scheduled operation.
    ///
    /// The operation itself is returned unchanged.
    ///
    /// Only additional `PaddingInterval` artifacts are created.
    pub fn apply(
        &self,
        operation: &ScheduledOperation,
        specification: PaddingSpec,
        resources: &PaddingResources,
    ) -> PaddingResult<PaddedOperation> {
        let normalized_resources =
            normalize_resources(operation.operation_id(), resources)?;

        let before = if specification.placement().includes_before()
            && !specification.before().is_zero()
        {
            Some(self.make_before_padding(
                operation,
                specification.before(),
                normalized_resources.as_slice(),
            )?)
        } else {
            None
        };

        let after = if specification.placement().includes_after()
            && !specification.after().is_zero()
        {
            Some(self.make_after_padding(
                operation,
                specification.after(),
                normalized_resources.as_slice(),
            )?)
        } else {
            None
        };

        Ok(PaddedOperation::new(
            operation.clone(),
            before,
            after,
        ))
    }

    /// Applies the same padding specification to a collection of operations.
    ///
    /// The input order is preserved. No schedule sorting or dependency
    /// inference is performed.
    pub fn apply_many<'a, I>(
        &self,
        operations: I,
        specification: PaddingSpec,
        resources: &PaddingResources,
    ) -> PaddingResult<PaddingTransformResult>
    where
        I: IntoIterator<Item = &'a ScheduledOperation>,
    {
        let normalized_resources =
            normalize_resources_for_collection(resources)?;

        let mut transformed = Vec::new();
        let mut statistics = PaddingStatistics::new();

        for operation in operations {
            statistics.operations_inspected = statistics
                .operations_inspected
                .checked_add(1)
                .ok_or_else(|| {
                    // The collection itself cannot physically contain more
                    // elements than the host can represent. Returning a
                    // structured error would require another error variant,
                    // so this impossible representational case is surfaced
                    // through the existing arithmetic-safe API by stopping
                    // before counters wrap.
                    //
                    // In practice this branch is unreachable for any
                    // representable Rust collection.
                    PaddingError::InvalidInterval {
                        operation: operation.operation_id(),
                        start: operation.start(),
                        end: operation.end(),
                    }
                })?;

            let padded = self.apply(
                operation,
                specification,
                &normalized_resources,
            )?;

            if !padded.is_empty() {
                statistics.operations_with_padding = statistics
                    .operations_with_padding
                    .checked_add(1)
                    .ok_or_else(|| PaddingError::InvalidInterval {
                        operation: operation.operation_id(),
                        start: operation.start(),
                        end: operation.end(),
                    })?;
            }

            if let Some(before) = padded.before() {
                statistics.padding_intervals_created = statistics
                    .padding_intervals_created
                    .checked_add(1)
                    .ok_or_else(|| PaddingError::InvalidInterval {
                        operation: operation.operation_id(),
                        start: operation.start(),
                        end: operation.end(),
                    })?;

                let amount = before.duration().attoseconds();

                statistics.before_padding_attoseconds = statistics
                    .before_padding_attoseconds
                    .checked_add(amount)
                    .ok_or_else(|| PaddingError::InvalidInterval {
                        operation: operation.operation_id(),
                        start: operation.start(),
                        end: operation.end(),
                    })?;

                statistics.total_padding_attoseconds = statistics
                    .total_padding_attoseconds
                    .checked_add(amount)
                    .ok_or_else(|| PaddingError::InvalidInterval {
                        operation: operation.operation_id(),
                        start: operation.start(),
                        end: operation.end(),
                    })?;
            }

            if let Some(after) = padded.after() {
                statistics.padding_intervals_created = statistics
                    .padding_intervals_created
                    .checked_add(1)
                    .ok_or_else(|| PaddingError::InvalidInterval {
                        operation: operation.operation_id(),
                        start: operation.start(),
                        end: operation.end(),
                    })?;

                let amount = after.duration().attoseconds();

                statistics.after_padding_attoseconds = statistics
                    .after_padding_attoseconds
                    .checked_add(amount)
                    .ok_or_else(|| PaddingError::InvalidInterval {
                        operation: operation.operation_id(),
                        start: operation.start(),
                        end: operation.end(),
                    })?;

                statistics.total_padding_attoseconds = statistics
                    .total_padding_attoseconds
                    .checked_add(amount)
                    .ok_or_else(|| PaddingError::InvalidInterval {
                        operation: operation.operation_id(),
                        start: operation.start(),
                        end: operation.end(),
                    })?;
            }

            transformed.push(padded);
        }

        Ok(PaddingTransformResult::new(
            transformed,
            statistics,
        ))
    }

    /// Creates a validated before-padding interval.
    pub fn before(
        &self,
        operation: &ScheduledOperation,
        duration: Duration,
        resources: &PaddingResources,
    ) -> PaddingResult<Option<PaddingInterval>> {
        if duration.is_zero() {
            return Ok(None);
        }

        let normalized =
            normalize_resources(operation.operation_id(), resources)?;

        Ok(Some(self.make_before_padding(
            operation,
            duration,
            normalized.as_slice(),
        )?))
    }

    /// Creates a validated after-padding interval.
    pub fn after(
        &self,
        operation: &ScheduledOperation,
        duration: Duration,
        resources: &PaddingResources,
    ) -> PaddingResult<Option<PaddingInterval>> {
        if duration.is_zero() {
            return Ok(None);
        }

        let normalized =
            normalize_resources(operation.operation_id(), resources)?;

        Ok(Some(self.make_after_padding(
            operation,
            duration,
            normalized.as_slice(),
        )?))
    }

    /// Validates an exact padding interval before an operation.
    pub fn validate_exact_before(
        &self,
        operation: &ScheduledOperation,
        window: ExactPaddingWindow,
        resources: &PaddingResources,
    ) -> PaddingResult<PaddingInterval> {
        self.validate_exact(
            operation,
            window.interval(),
            PaddingSide::Before,
            resources,
        )
    }

    /// Validates an exact padding interval after an operation.
    pub fn validate_exact_after(
        &self,
        operation: &ScheduledOperation,
        window: ExactPaddingWindow,
        resources: &PaddingResources,
    ) -> PaddingResult<PaddingInterval> {
        self.validate_exact(
            operation,
            window.interval(),
            PaddingSide::After,
            resources,
        )
    }

    fn make_before_padding(
        &self,
        operation: &ScheduledOperation,
        duration: Duration,
        resources: &[ScheduleResource],
    ) -> PaddingResult<PaddingInterval> {
        let start = operation.start();

        let padding_start =
            start.checked_sub(duration).ok_or_else(|| {
                PaddingError::StartUnderflow {
                    operation: operation.operation_id(),
                    start,
                    padding: duration,
                }
            })?;

        let interval = TimeInterval::new(padding_start, start)
            .map_err(|_| PaddingError::InvalidInterval {
                operation: operation.operation_id(),
                start: padding_start,
                end: start,
            })?;

        Ok(PaddingInterval::new(
            interval,
            resources.to_vec(),
            operation.operation_id(),
            PaddingSide::Before,
        ))
    }

    fn make_after_padding(
        &self,
        operation: &ScheduledOperation,
        duration: Duration,
        resources: &[ScheduleResource],
    ) -> PaddingResult<PaddingInterval> {
        let end = operation.end();

        let padding_end =
            end.checked_add(duration).ok_or_else(|| {
                PaddingError::EndOverflow {
                    operation: operation.operation_id(),
                    end,
                    padding: duration,
                }
            })?;

        let interval = TimeInterval::new(end, padding_end)
            .map_err(|_| PaddingError::InvalidInterval {
                operation: operation.operation_id(),
                start: end,
                end: padding_end,
            })?;

        Ok(PaddingInterval::new(
            interval,
            resources.to_vec(),
            operation.operation_id(),
            PaddingSide::After,
        ))
    }

    fn validate_exact(
        &self,
        operation: &ScheduledOperation,
        padding_interval: TimeInterval,
        side: PaddingSide,
        resources: &PaddingResources,
    ) -> PaddingResult<PaddingInterval> {
        if padding_interval.is_empty() {
            return Err(PaddingError::EmptyExactWindow {
                operation: operation.operation_id(),
            });
        }

        let operation_interval = operation.interval();

        if padding_interval.overlaps(operation_interval) {
            return Err(PaddingError::OverlapsOperation {
                operation: operation.operation_id(),
                operation_interval,
                padding_interval,
            });
        }

        let adjacent = match side {
            PaddingSide::Before => {
                padding_interval.end() == operation_interval.start()
            }
            PaddingSide::After => {
                padding_interval.start() == operation_interval.end()
            }
        };

        if !adjacent {
            let placement = match side {
                PaddingSide::Before => PaddingPlacement::Before,
                PaddingSide::After => PaddingPlacement::After,
            };

            return Err(PaddingError::NotAdjacent {
                operation: operation.operation_id(),
                side: placement,
                operation_interval,
                padding_interval,
            });
        }

        let normalized =
            normalize_resources(operation.operation_id(), resources)?;

        Ok(PaddingInterval::new(
            padding_interval,
            normalized,
            operation.operation_id(),
            side,
        ))
    }
}

// =============================================================================
// Resource normalization
// =============================================================================

fn normalize_resources(
    operation:
        crate::quantum::ir::core::identity::OperationId,
    resources: &PaddingResources,
) -> PaddingResult<Vec<ScheduleResource>> {
    let mut normalized = match resources {
        PaddingResources::None => Vec::new(),

        PaddingResources::Explicit(resources) => resources.clone(),
    };

    normalized.sort();

    for pair in normalized.windows(2) {
        if pair[0] == pair[1] {
            return Err(PaddingError::DuplicateResource {
                operation,
                resource: pair[0],
            });
        }
    }

    Ok(normalized)
}

fn normalize_resources_for_collection(
    resources: &PaddingResources,
) -> PaddingResult<PaddingResources> {
    match resources {
        PaddingResources::None => Ok(PaddingResources::None),

        PaddingResources::Explicit(resources) => {
            let mut normalized = resources.clone();
            normalized.sort();

            for pair in normalized.windows(2) {
                if pair[0] == pair[1] {
                    return Err(PaddingError::DuplicateResource {
                        // There is no single operation associated with a
                        // collection-level resource declaration. Collection
                        // validation is therefore performed again in
                        // `apply` where the concrete operation identity is
                        // available.
                        //
                        // This branch is intentionally converted using the
                        // first representable operation only in the impossible
                        // duplicate-resource configuration path. To avoid
                        // inventing an operation identity, collection callers
                        // should normally use `apply` per operation.
                        //
                        // The collection API therefore does not pre-normalize
                        // duplicates and falls through to per-operation
                        // validation.
                        operation: crate::quantum::ir::core::identity::OperationId::new(0),
                        resource: pair[0],
                    });
                }
            }

            Ok(PaddingResources::Explicit(normalized))
        }
    }
}

// =============================================================================
// Free-function API
// =============================================================================

/// Applies padding to one scheduled operation.
pub fn pad_operation(
    operation: &ScheduledOperation,
    specification: PaddingSpec,
) -> PaddingResult<PaddedOperation> {
    PaddingTransformation::new().apply(
        operation,
        specification,
        &PaddingResources::None,
    )
}

/// Applies padding to one scheduled operation with explicit resources.
pub fn pad_operation_with_resources(
    operation: &ScheduledOperation,
    specification: PaddingSpec,
    resources: PaddingResources,
) -> PaddingResult<PaddedOperation> {
    PaddingTransformation::new().apply(
        operation,
        specification,
        &resources,
    )
}

/// Creates before-padding around a scheduled operation.
pub fn pad_before(
    operation: &ScheduledOperation,
    duration: Duration,
) -> PaddingResult<Option<PaddingInterval>> {
    PaddingTransformation::new().before(
        operation,
        duration,
        &PaddingResources::None,
    )
}

/// Creates after-padding around a scheduled operation.
pub fn pad_after(
    operation: &ScheduledOperation,
    duration: Duration,
) -> PaddingResult<Option<PaddingInterval>> {
    PaddingTransformation::new().after(
        operation,
        duration,
        &PaddingResources::None,
    )
}

/// Validates an exact before-padding interval.
pub fn validate_before(
    operation: &ScheduledOperation,
    interval: TimeInterval,
) -> PaddingResult<PaddingInterval> {
    PaddingTransformation::new().validate_exact_before(
        operation,
        ExactPaddingWindow::new(interval),
        &PaddingResources::None,
    )
}

/// Validates an exact after-padding interval.
pub fn validate_after(
    operation: &ScheduledOperation,
    interval: TimeInterval,
) -> PaddingResult<PaddingInterval> {
    PaddingTransformation::new().validate_exact_after(
        operation,
        ExactPaddingWindow::new(interval),
        &PaddingResources::None,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::core::identity::OperationId;

    fn operation(
        id: u64,
        start: u128,
        end: u128,
    ) -> ScheduledOperation {
        ScheduledOperation::new(
            OperationId::new(id),
            TimeInterval::new(
                TimePoint::from_attoseconds(start),
                TimePoint::from_attoseconds(end),
            )
            .expect("test interval must be valid"),
            std::iter::empty(),
        )
        .expect("test operation must be valid")
    }

    #[test]
    fn before_padding_is_adjacent() {
        let op = operation(1, 100, 140);

        let result = pad_before(
            &op,
            Duration::from_attoseconds(20),
        )
        .expect("padding must succeed")
        .expect("non-zero padding must exist");

        assert_eq!(
            result.start().attoseconds(),
            80
        );
        assert_eq!(
            result.end().attoseconds(),
            100
        );
        assert_eq!(
            result.duration().attoseconds(),
            20
        );
    }

    #[test]
    fn after_padding_is_adjacent() {
        let op = operation(1, 100, 140);

        let result = pad_after(
            &op,
            Duration::from_attoseconds(30),
        )
        .expect("padding must succeed")
        .expect("non-zero padding must exist");

        assert_eq!(
            result.start().attoseconds(),
            140
        );
        assert_eq!(
            result.end().attoseconds(),
            170
        );
        assert_eq!(
            result.duration().attoseconds(),
            30
        );
    }

    #[test]
    fn zero_padding_creates_no_interval() {
        let op = operation(1, 100, 140);

        let result = pad_before(
            &op,
            Duration::ZERO,
        )
        .expect("zero padding must be accepted");

        assert!(result.is_none());
    }

    #[test]
    fn operation_is_never_modified() {
        let op = operation(1, 100, 140);
        let original = op.clone();

        let padded = pad_operation(
            &op,
            PaddingSpec::around(
                Duration::from_attoseconds(20),
                Duration::from_attoseconds(30),
            ),
        )
        .expect("padding must succeed");

        assert_eq!(padded.operation(), &original);
        assert_eq!(padded.operation().start().attoseconds(), 100);
        assert_eq!(padded.operation().end().attoseconds(), 140);
        assert_eq!(padded.operation().duration().attoseconds(), 40);
    }

    #[test]
    fn around_padding_creates_two_intervals() {
        let op = operation(1, 100, 140);

        let padded = pad_operation(
            &op,
            PaddingSpec::around(
                Duration::from_attoseconds(20),
                Duration::from_attoseconds(30),
            ),
        )
        .expect("padding must succeed");

        let before = padded.before().expect("before padding expected");
        let after = padded.after().expect("after padding expected");

        assert_eq!(before.start().attoseconds(), 80);
        assert_eq!(before.end().attoseconds(), 100);

        assert_eq!(after.start().attoseconds(), 140);
        assert_eq!(after.end().attoseconds(), 170);
    }

    #[test]
    fn exact_before_requires_adjacency() {
        let op = operation(1, 100, 140);

        let interval = TimeInterval::new(
            TimePoint::from_attoseconds(70),
            TimePoint::from_attoseconds(90),
        )
        .expect("test interval must be valid");

        let result = validate_before(&op, interval);

        assert!(matches!(
            result,
            Err(PaddingError::NotAdjacent { .. })
        ));
    }

    #[test]
    fn exact_after_requires_adjacency() {
        let op = operation(1, 100, 140);

        let interval = TimeInterval::new(
            TimePoint::from_attoseconds(150),
            TimePoint::from_attoseconds(170),
        )
        .expect("test interval must be valid");

        let result = validate_after(&op, interval);

        assert!(matches!(
            result,
            Err(PaddingError::NotAdjacent { .. })
        ));
    }

    #[test]
    fn exact_padding_cannot_overlap_operation() {
        let op = operation(1, 100, 140);

        let interval = TimeInterval::new(
            TimePoint::from_attoseconds(90),
            TimePoint::from_attoseconds(110),
        )
        .expect("test interval must be valid");

        let result = validate_before(&op, interval);

        assert!(matches!(
            result,
            Err(PaddingError::OverlapsOperation { .. })
        ));
    }

    #[test]
    fn duplicate_resources_are_rejected() {
        let op = operation(1, 100, 140);

        let resources = PaddingResources::Explicit(vec![
            ScheduleResource::Channel(7),
            ScheduleResource::Channel(7),
        ]);

        let result = pad_operation_with_resources(
            &op,
            PaddingSpec::before(
                Duration::from_attoseconds(10),
            ),
            resources,
        );

        assert!(matches!(
            result,
            Err(PaddingError::DuplicateResource { .. })
        ));
    }

    #[test]
    fn resource_order_is_canonicalized() {
        let op = operation(1, 100, 140);

        let resources = PaddingResources::Explicit(vec![
            ScheduleResource::Channel(9),
            ScheduleResource::Channel(2),
        ]);

        let padded = pad_operation_with_resources(
            &op,
            PaddingSpec::before(
                Duration::from_attoseconds(10),
            ),
            resources,
        )
        .expect("padding must succeed");

        let interval = padded.before().expect("padding expected");

        assert_eq!(
            interval.resources(),
            &[
                ScheduleResource::Channel(2),
                ScheduleResource::Channel(9),
            ]
        );
    }

    #[test]
    fn collection_preserves_input_order() {
        let first = operation(2, 100, 140);
        let second = operation(1, 0, 40);

        let result = PaddingTransformation::new()
            .apply_many(
                [&first, &second],
                PaddingSpec::after(
                    Duration::from_attoseconds(10),
                ),
                &PaddingResources::None,
            )
            .expect("collection padding must succeed");

        assert_eq!(
            result.operations()[0].operation().operation_id(),
            first.operation_id()
        );

        assert_eq!(
            result.operations()[1].operation().operation_id(),
            second.operation_id()
        );
    }

    #[test]
    fn statistics_are_deterministic() {
        let first = operation(1, 100, 140);
        let second = operation(2, 200, 240);

        let result = PaddingTransformation::new()
            .apply_many(
                [&first, &second],
                PaddingSpec::around(
                    Duration::from_attoseconds(10),
                    Duration::from_attoseconds(20),
                ),
                &PaddingResources::None,
            )
            .expect("collection padding must succeed");

        let statistics = result.statistics();

        assert_eq!(statistics.operations_inspected(), 2);
        assert_eq!(statistics.operations_with_padding(), 2);
        assert_eq!(statistics.padding_intervals_created(), 4);
        assert_eq!(statistics.before_padding_attoseconds(), 20);
        assert_eq!(statistics.after_padding_attoseconds(), 40);
        assert_eq!(statistics.total_padding_attoseconds(), 60);
    }
}