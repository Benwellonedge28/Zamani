//! Zamani Quantum Scheduling — Alignment Transformation
//!
//! Path:
//!     src/quantum/scheduling/transformations/alignment.rs
//!
//! # Purpose
//!
//! This module applies an already-defined scheduling alignment rule to
//! canonical scheduled operations.
//!
//! Alignment transformation answers:
//!
//! > "Given an already scheduled operation, how can its temporal placement be
//! > adjusted so that it satisfies the target's alignment requirements without
//! > changing its semantic operation identity or duration?"
//!
//! This module is deliberately a TRANSFORMATION layer.
//!
//! It does not:
//!
//! - schedule operations;
//! - choose routing;
//! - discover hardware;
//! - define hardware timing;
//! - define a second timing model;
//! - define a second `QubitId`;
//! - define a second `PhysicalQubitId`;
//! - define a second `Duration`;
//! - define a second `TimePoint`;
//! - synthesize pulses;
//! - execute a QPU;
//! - perform QEC;
//! - perform noise modelling;
//! - contact a provider;
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
//!       v
//! timing::alignment
//!       |
//!       v
//! transformations::alignment   <-- this module
//!       |
//!       v
//! aligned ScheduledOperation
//!       |
//!       v
//! verification
//!       |
//!       v
//! hardware lowering
//! ```
//!
//! # Critical ownership rule
//!
//! The canonical semantic timing model remains:
//!
//! ```text
//! crate::quantum::ir::timing
//! ```
//!
//! The scheduling alignment rule remains:
//!
//! ```text
//! crate::quantum::scheduling::timing::alignment
//! ```
//!
//! The canonical scheduled-operation representation remains:
//!
//! ```text
//! crate::quantum::ir::scheduling
//! ```
//!
//! This module introduces NONE of the following duplicate concepts:
//!
//! ```text
//! TimePoint
//! Duration
//! TimeInterval
//! QubitId
//! PhysicalQubitId
//! ScheduleResource
//! ScheduledOperation
//! TimingResolution
//! AlignmentRule
//! AlignmentKind
//! AlignmentMode
//! ```
//!
//! # Why transformation is separate from scheduling
//!
//! Scheduling determines a legal temporal placement according to dependencies,
//! resources, policy, and objectives.
//!
//! Alignment transformation performs a narrower operation:
//!
//! ```text
//! existing schedule
//!       |
//!       v
//! target alignment constraint
//!       |
//!       v
//! adjusted schedule
//! ```
//!
//! Keeping these responsibilities separate allows:
//!
//! - ASAP scheduling;
//! - ALAP scheduling;
//! - list scheduling;
//! - critical-path scheduling;
//! - resource-constrained scheduling;
//! - event-driven scheduling;
//! - adaptive scheduling;
//!
//! to remain independent of the mechanics of target-grid adjustment.
//!
//! # Universal-program principle
//!
//! Zamani programs are written at the semantic level rather than for a fixed
//! hardware machine.
//!
//! Therefore this module contains no:
//!
//! - fixed qubit count;
//! - fixed channel count;
//! - fixed operation count;
//! - fixed topology;
//! - fixed schedule depth;
//! - fixed clock;
//! - fixed sample period;
//! - fixed alignment value;
//! - vendor-specific instruction;
//! - hardware-specific constant.
//!
//! The target supplies alignment requirements through `AlignmentRule`.
//!
//! The same transformation therefore works for:
//!
//! - one-qubit devices;
//! - small QPUs;
//! - large QPUs;
//! - fault-tolerant processors;
//! - modular quantum processors;
//! - distributed quantum systems;
//! - simulators;
//! - emulators;
//! - future quantum architectures.
//!
//! "Infinity" is interpreted correctly as:
//!
//! > no artificial machine-size ceiling is introduced by this module.
//!
//! A concrete process remains bounded by the target, available memory, CPU,
//! address space, explicit policies, and the finite execution request.
//!
//! # Canonical qubit identity
//!
//! This module does not need to import `QubitId` or `PhysicalQubitId` directly.
//!
//! If a `ScheduledOperation` contains:
//!
//! ```text
//! ScheduleResource::LogicalQubit(...)
//! ScheduleResource::PhysicalQubit(...)
//! ```
//!
//! those identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! No replacement identity is introduced here.
//!
//! This is intentional. Alignment is fundamentally temporal, not a qubit
//! identity problem.
//!
//! # Exact arithmetic
//!
//! This module performs no floating-point arithmetic.
//!
//! All temporal calculations use the canonical attosecond representation exposed
//! by the IR timing subsystem.
//!
//! This prevents:
//!
//! - floating-point drift;
//! - platform-dependent rounding;
//! - hidden precision loss;
//! - non-deterministic grid placement.
//!
//! # Half-open interval semantics
//!
//! Canonical scheduled intervals are:
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
//! This module preserves that semantic representation.
//!
//! # Semantic preservation
//!
//! Alignment transformation MUST preserve:
//!
//! 1. operation identity;
//! 2. operation resources;
//! 3. operation duration;
//! 4. operation ordering represented by the supplied schedule;
//! 5. interval validity.
//!
//! The transformation may change the temporal position when the selected
//! alignment mode permits it.
//!
//! It must never silently change operation duration.
//!
//! # Important limitation
//!
//! A single-operation alignment transformation cannot prove whole-schedule
//! dependency correctness because dependencies are owned by the scheduling IR.
//!
//! Consequently:
//!
//! ```text
//! transformations::alignment
//!     |
//!     | local temporal correctness
//!     v
//! scheduling::verification
//!     |
//!     | whole schedule correctness
//!     v
//! executable schedule
//! ```
//!
//! When transforming multiple operations, this module applies the requested
//! local rule deterministically. Whole-program dependency/resource validation
//! remains the responsibility of the verification layer.
//!
//! # Alignment semantics
//!
//! `AlignmentKind` is interpreted as follows:
//!
//! ```text
//! None
//!     no temporal transformation
//!
//! Start
//!     align start, preserve duration
//!
//! End
//!     align end, preserve duration
//!
//! Duration
//!     validate duration against the alignment grid
//!     without moving the operation
//!
//! StartAndEnd
//!     require both start and end to be aligned
//!     while preserving duration
//! ```
//!
//! `AlignmentMode` comes from the existing canonical scheduling alignment
//! subsystem:
//!
//! ```text
//! Strict
//! Nearest
//! Ceil
//! Floor
//! ```
//!
//! The transformation never invents another alignment policy.
//!
//! # Strict mode
//!
//! `Strict` never changes a schedule.
//!
//! An already aligned value is returned unchanged.
//!
//! An unaligned value produces a structured error.
//!
//! This is the safest mode for verification and final compilation.
//!
//! # Ceil mode
//!
//! `Ceil` moves the relevant temporal boundary to the first legal grid point
//! at or after the requested boundary.
//!
//! For start alignment:
//!
//! ```text
//! start' = ceil(start)
//! end'   = start' + duration
//! ```
//!
//! For end alignment:
//!
//! ```text
//! end'   = ceil(end)
//! start' = end' - duration
//! ```
//!
//! # Floor mode
//!
//! `Floor` moves the relevant boundary to the last legal grid point at or
//! before the requested boundary.
//!
//! For start alignment:
//!
//! ```text
//! start' = floor(start)
//! end'   = start' + duration
//! ```
//!
//! For end alignment:
//!
//! ```text
//! end'   = floor(end)
//! start' = end' - duration
//! ```
//!
//! The caller remains responsible for choosing a mode that is compatible with
//! its scheduling semantics.
//!
//! # Nearest mode
//!
//! `Nearest` uses the existing `AlignmentRule` implementation.
//!
//! Ties are resolved deterministically according to the canonical alignment
//! subsystem.
//!
//! # StartAndEnd
//!
//! A particularly important semantic rule is used here:
//!
//! ```text
//! StartAndEnd
//! ```
//!
//! means both boundaries must be aligned while the operation duration remains
//! unchanged.
//!
//! Therefore an operation whose duration is not compatible with the target
//! alignment grid cannot be repaired merely by shifting its start.
//!
//! The transformation reports an error rather than silently changing duration.
//!
//! Example:
//!
//! ```text
//! start = 0
//! duration = 7
//! grid = 10
//!
//! start is aligned.
//! end = 7 is not aligned.
//!
//! Shifting start to 10 gives end = 17,
//! which is still not aligned.
//!
//! The transformation therefore rejects the operation.
//! ```
//!
//! This protects semantic duration.
//!
//! # Duration alignment
//!
//! `AlignmentKind::Duration` does not move the operation.
//!
//! It validates the duration against the supplied alignment rule.
//!
//! If the duration is not aligned, the transformation returns an error.
//!
//! This is deliberate because changing an operation duration would change its
//! physical execution semantics and therefore belongs to hardware-aware
//! lowering or pulse synthesis, not this transformation.
//!
//! # Resource preservation
//!
//! `ScheduledOperation::resources()` is copied exactly.
//!
//! No resource is added.
//!
//! No resource is removed.
//!
//! No resource is renamed.
//!
//! No resource identity is regenerated.
//!
//! This means physical/logical qubit identity remains untouched.
//!
//! # Operation identity preservation
//!
//! The original:
//!
//! ```text
//! OperationId
//! ```
//!
//! is copied unchanged.
//!
//! Alignment transformation does not create a new semantic operation.
//!
//! # Determinism
//!
//! Given identical:
//!
//! - scheduled operation;
//! - alignment rule;
//! - configuration;
//! - canonical timing model;
//!
//! the transformation produces identical output.
//!
//! No hash-map iteration is used.
//!
//! No random state is used.
//!
//! No system clock is used.
//!
//! No hardware query is performed.
//!
//! # Scalability
//!
//! One operation is transformed using O(1) auxiliary storage.
//!
//! A collection transformation uses O(N) output storage for N operations and
//! does not allocate a timeline proportional to the maximum schedule time.
//!
//! There is no operation:
//!
//! ```text
//! for every possible time tick
//! ```
//!
//! and no operation:
//!
//! ```text
//! for every possible machine resource
//! ```
//!
//! This is essential for sparse large systems.
//!
//! # Overflow safety
//!
//! All arithmetic that can overflow is checked.
//!
//! In particular:
//!
//! ```text
//! aligned_start + duration
//! aligned_end - duration
//! ```
//!
//! are checked before constructing the resulting interval.
//!
//! No wrapping arithmetic is permitted.
//!
//! # Error handling
//!
//! Errors are structured and contain enough information to diagnose the
//! transformation failure.
//!
//! No panic is used for user-controlled scheduling data.
//!
//! # Thread safety
//!
//! This module owns no global state.
//!
//! Configuration is immutable.
//!
//! Inputs are borrowed immutably.
//!
//! Outputs own their data.
//!
//! The transformation can therefore be used independently by concurrent
//! compilation workers when the surrounding compiler pipeline permits it.
//!
//! # Integration contracts
//!
//! ## Canonical scheduled operation
//!
//! Input:
//!
//! ```text
//! crate::quantum::ir::scheduling::ScheduledOperation
//! ```
//!
//! Output:
//!
//! ```text
//! crate::quantum::ir::scheduling::ScheduledOperation
//! ```
//!
//! ## Canonical timing
//!
//! Uses:
//!
//! ```text
//! crate::quantum::ir::timing::Duration
//! crate::quantum::ir::timing::TimeInterval
//! crate::quantum::ir::timing::TimePoint
//! ```
//!
//! ## Scheduling alignment rules
//!
//! Uses:
//!
//! ```text
//! crate::quantum::scheduling::timing::alignment::AlignmentRule
//! crate::quantum::scheduling::timing::alignment::AlignmentKind
//! crate::quantum::scheduling::timing::alignment::AlignmentMode
//! ```
//!
//! ## Hardware
//!
//! Hardware supplies the alignment rule through the scheduling/hardware adapter.
//!
//! This module does not access:
//!
//! ```text
//! quantum::hardware
//! ```
//!
//! directly.
//!
//! ## Routing
//!
//! Routing occurs before scheduling and therefore before this transformation.
//!
//! The transformation observes already-resolved resources.
//!
//! ## Verification
//!
//! The resulting operation(s) should be passed to scheduling verification.
//!
//! ## Delay transformation
//!
//! Alignment can create temporal movement and therefore additional idle time.
//!
//! If explicit idle intervals are required, the downstream flow is:
//!
//! ```text
//! alignment
//!     |
//!     v
//! verification
//!     |
//!     v
//! transformations::delays
//! ```
//!
//! Alignment does not itself create `Delay` operations.
//!
//! ## QEC
//!
//! QEC can supply alignment rules through the scheduling context, but this
//! module does not depend on a particular error-correcting code.
//!
//! ## ZQN
//!
//! ZQN can consume the before/after timing information to estimate fidelity or
//! idle-time effects.
//!
//! This module does not depend directly on ZQN.
//!
//! ## Dynamic circuits
//!
//! This transformation only operates on statically known `ScheduledOperation`
//! intervals.
//!
//! Runtime-dependent timing must be handled by the dynamic scheduling/runtime
//! subsystem.
//!
//! ## Distributed scheduling
//!
//! Communication and synchronization operations may be aligned exactly like
//! other scheduled operations when they have a corresponding alignment rule.
//!
//! No distributed-system assumptions are embedded here.
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
//! The safety boundary is compiler-enforced.
//!
//! # Completion criterion
//!
//! This file is complete when:
//!
//! - it consumes canonical scheduled operations;
//! - it consumes canonical alignment rules;
//! - it preserves operation identity;
//! - it preserves resources;
//! - it preserves duration;
//! - it performs checked arithmetic;
//! - it supports all existing alignment modes;
//! - it supports all existing alignment kinds;
//! - it handles collections deterministically;
//! - it introduces no machine-size limits;
//! - it introduces no hardware constants;
//! - it introduces no qubit identity;
//! - it does not schedule operations;
//! - it does not mutate global state;
//! - it contains no unsafe code.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use crate::quantum::ir::scheduling::ScheduledOperation;
use crate::quantum::ir::timing::{
    Duration,
    TimeInterval,
    TimePoint,
};

use super::super::timing::alignment::{
    AlignmentError,
    AlignmentKind,
    AlignmentMode,
    AlignmentRule,
};

// =============================================================================
// Public result aliases
// =============================================================================

/// Result returned by alignment transformations.
pub type AlignmentTransformResult<T> =
    Result<T, AlignmentTransformError>;

// =============================================================================
// Transformation errors
// =============================================================================

/// Errors produced while transforming scheduled operations for alignment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlignmentTransformError {
    /// The alignment subsystem rejected a temporal value.
    Alignment {
        /// Underlying alignment error.
        source: AlignmentError,
    },

    /// A checked temporal calculation overflowed.
    ArithmeticOverflow {
        /// Operation whose transformation overflowed.
        operation: crate::quantum::ir::core::identity::OperationId,

        /// Description of the calculation.
        operation_kind: &'static str,
    },

    /// The requested transformation would require changing an operation's
    /// duration, which is forbidden by this module.
    DurationWouldChange {
        /// Operation whose duration is incompatible with the requested
        /// alignment.
        operation: crate::quantum::ir::core::identity::OperationId,

        /// Original duration.
        duration: Duration,

        /// Requested alignment kind.
        kind: AlignmentKind,
    },

    /// Start and end cannot both satisfy the requested alignment while
    /// preserving the original duration.
    StartAndEndIncompatible {
        /// Operation whose interval cannot satisfy both boundaries.
        operation: crate::quantum::ir::core::identity::OperationId,

        /// Original start.
        start: TimePoint,

        /// Original end.
        end: TimePoint,

        /// Original duration.
        duration: Duration,
    },

    /// The canonical timing model rejected construction of the transformed
    /// interval.
    InvalidInterval {
        /// Operation associated with the invalid interval.
        operation: crate::quantum::ir::core::identity::OperationId,

        /// Proposed start.
        start: TimePoint,

        /// Proposed end.
        end: TimePoint,
    },

    /// A transformation was requested for a mode that cannot move the selected
    /// boundary.
    UnsupportedTransformation {
        /// Requested alignment kind.
        kind: AlignmentKind,

        /// Requested mode.
        mode: AlignmentMode,
    },
}

impl fmt::Display for AlignmentTransformError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Alignment { source } => {
                write!(
                    formatter,
                    "alignment rule rejected temporal value: {source}"
                )
            }

            Self::ArithmeticOverflow {
                operation,
                operation_kind,
            } => {
                write!(
                    formatter,
                    "alignment transformation overflow for operation \
                     `{operation}` while calculating {operation_kind}"
                )
            }

            Self::DurationWouldChange {
                operation,
                duration,
                kind,
            } => {
                write!(
                    formatter,
                    "alignment of operation `{operation}` with kind \
                     `{kind:?}` would require changing its duration \
                     `{duration}`"
                )
            }

            Self::StartAndEndIncompatible {
                operation,
                start,
                end,
                duration,
            } => {
                write!(
                    formatter,
                    "operation `{operation}` cannot satisfy both start \
                     and end alignment while preserving interval \
                     [{start}, {end}) and duration {duration}"
                )
            }

            Self::InvalidInterval {
                operation,
                start,
                end,
            } => {
                write!(
                    formatter,
                    "alignment produced invalid interval for operation \
                     `{operation}`: [{start}, {end})"
                )
            }

            Self::UnsupportedTransformation { kind, mode } => {
                write!(
                    formatter,
                    "alignment transformation is not supported for \
                     kind `{kind:?}` and mode `{mode:?}`"
                )
            }
        }
    }
}

impl std::error::Error for AlignmentTransformError {}

impl From<AlignmentError> for AlignmentTransformError {
    fn from(source: AlignmentError) -> Self {
        Self::Alignment { source }
    }
}

// =============================================================================
// Alignment transformation statistics
// =============================================================================

/// Deterministic statistics describing an alignment transformation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AlignmentTransformStatistics {
    /// Number of operations inspected.
    operations_inspected: u64,

    /// Number of operations whose temporal interval changed.
    operations_changed: u64,

    /// Number of operations that were already aligned.
    operations_unchanged: u64,

    /// Total forward temporal movement in attoseconds.
    ///
    /// This value is useful for measuring introduced idle time.
    forward_shift_attoseconds: u128,

    /// Total backward temporal movement in attoseconds.
    ///
    /// This value is useful for measuring how far floor alignment moved
    /// boundaries.
    backward_shift_attoseconds: u128,
}

impl AlignmentTransformStatistics {
    /// Creates empty statistics.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operations_inspected: 0,
            operations_changed: 0,
            operations_unchanged: 0,
            forward_shift_attoseconds: 0,
            backward_shift_attoseconds: 0,
        }
    }

    /// Returns the number of inspected operations.
    #[must_use]
    pub const fn operations_inspected(self) -> u64 {
        self.operations_inspected
    }

    /// Returns the number of changed operations.
    #[must_use]
    pub const fn operations_changed(self) -> u64 {
        self.operations_changed
    }

    /// Returns the number of unchanged operations.
    #[must_use]
    pub const fn operations_unchanged(self) -> u64 {
        self.operations_unchanged
    }

    /// Returns total forward movement.
    #[must_use]
    pub const fn forward_shift_attoseconds(self) -> u128 {
        self.forward_shift_attoseconds
    }

    /// Returns total backward movement.
    #[must_use]
    pub const fn backward_shift_attoseconds(self) -> u128 {
        self.backward_shift_attoseconds
    }
}

// =============================================================================
// Single-operation result
// =============================================================================

/// Result of transforming one scheduled operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlignedOperation {
    /// Original operation after alignment.
    operation: ScheduledOperation,

    /// Original start before transformation.
    original_start: TimePoint,

    /// Original end before transformation.
    original_end: TimePoint,

    /// Original duration.
    original_duration: Duration,
}

impl AlignedOperation {
    /// Creates a transformation result.
    fn new(
        operation: ScheduledOperation,
        original_start: TimePoint,
        original_end: TimePoint,
        original_duration: Duration,
    ) -> Self {
        Self {
            operation,
            original_start,
            original_end,
            original_duration,
        }
    }

    /// Returns the transformed scheduled operation.
    #[must_use]
    pub fn operation(&self) -> &ScheduledOperation {
        &self.operation
    }

    /// Consumes this result and returns the transformed operation.
    #[must_use]
    pub fn into_operation(self) -> ScheduledOperation {
        self.operation
    }

    /// Returns the original start.
    #[must_use]
    pub const fn original_start(&self) -> TimePoint {
        self.original_start
    }

    /// Returns the original end.
    #[must_use]
    pub const fn original_end(&self) -> TimePoint {
        self.original_end
    }

    /// Returns the original duration.
    #[must_use]
    pub const fn original_duration(&self) -> Duration {
        self.original_duration
    }

    /// Returns the transformed start.
    #[must_use]
    pub fn start(&self) -> TimePoint {
        self.operation.start()
    }

    /// Returns the transformed end.
    #[must_use]
    pub fn end(&self) -> TimePoint {
        self.operation.end()
    }

    /// Returns the transformed duration.
    #[must_use]
    pub fn duration(&self) -> Duration {
        self.operation.duration()
    }

    /// Returns whether the operation moved in time.
    #[must_use]
    pub fn changed(&self) -> bool {
        self.start() != self.original_start
            || self.end() != self.original_end
    }

    /// Returns whether the transformation moved the operation forward.
    #[must_use]
    pub fn moved_forward(&self) -> bool {
        self.start() > self.original_start
            || self.end() > self.original_end
    }

    /// Returns whether the transformation moved the operation backward.
    #[must_use]
    pub fn moved_backward(&self) -> bool {
        self.start() < self.original_start
            || self.end() < self.original_end
    }

    /// Returns the absolute forward movement when the operation moved forward.
    ///
    /// Returns zero if the operation did not move forward.
    #[must_use]
    pub fn forward_shift_attoseconds(&self) -> u128 {
        if self.moved_forward() {
            self.start()
                .attoseconds()
                .saturating_sub(self.original_start.attoseconds())
        } else {
            0
        }
    }

    /// Returns the absolute backward movement when the operation moved
    /// backward.
    ///
    /// Returns zero if the operation did not move backward.
    #[must_use]
    pub fn backward_shift_attoseconds(&self) -> u128 {
        if self.moved_backward() {
            self.original_start
                .attoseconds()
                .saturating_sub(self.start().attoseconds())
        } else {
            0
        }
    }
}

// =============================================================================
// Transformation configuration
// =============================================================================

/// Configuration for alignment transformation.
///
/// The configuration is deliberately small because the actual target
/// alignment rule is supplied separately.
///
/// This structure does not contain machine-specific values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlignmentTransformConfig {
    /// Whether an already aligned operation should still be reconstructed.
    ///
    /// This normally remains `false`.
    ///
    /// When `false`, already aligned operations are returned unchanged.
    preserve_already_aligned: bool,
}

impl Default for AlignmentTransformConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl AlignmentTransformConfig {
    /// Creates the default transformation configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            preserve_already_aligned: true,
        }
    }

    /// Returns whether already aligned operations are preserved.
    #[must_use]
    pub const fn preserve_already_aligned(self) -> bool {
        self.preserve_already_aligned
    }

    /// Configures preservation of already aligned operations.
    #[must_use]
    pub const fn with_preserve_already_aligned(
        mut self,
        preserve: bool,
    ) -> Self {
        self.preserve_already_aligned = preserve;
        self
    }
}

// =============================================================================
// Collection result
// =============================================================================

/// Result of transforming a collection of scheduled operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlignmentTransformResult {
    operations: Vec<ScheduledOperation>,
    statistics: AlignmentTransformStatistics,
}

impl AlignmentTransformResult {
    /// Creates a collection result.
    fn new(
        operations: Vec<ScheduledOperation>,
        statistics: AlignmentTransformStatistics,
    ) -> Self {
        Self {
            operations,
            statistics,
        }
    }

    /// Returns transformed operations in the same semantic iteration order as
    /// the supplied input.
    #[must_use]
    pub fn operations(&self) -> &[ScheduledOperation] {
        &self.operations
    }

    /// Consumes the result and returns the transformed operations.
    #[must_use]
    pub fn into_operations(self) -> Vec<ScheduledOperation> {
        self.operations
    }

    /// Returns transformation statistics.
    #[must_use]
    pub const fn statistics(&self) -> AlignmentTransformStatistics {
        self.statistics
    }

    /// Returns the number of transformed operations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether no operations were transformed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

// =============================================================================
// Main transformer
// =============================================================================

/// Stateless alignment transformation engine.
///
/// The transformer contains no hardware state, no scheduler state, no global
/// state, and no mutable caches.
///
/// This makes it safe to instantiate independently in any compilation worker.
#[derive(Debug, Clone, Copy, Default)]
pub struct AlignmentTransformer {
    config: AlignmentTransformConfig,
}

impl AlignmentTransformer {
    /// Creates a transformer using the default configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: AlignmentTransformConfig::new(),
        }
    }

    /// Creates a transformer using an explicit configuration.
    #[must_use]
    pub const fn with_config(
        config: AlignmentTransformConfig,
    ) -> Self {
        Self { config }
    }

    /// Returns the transformation configuration.
    #[must_use]
    pub const fn config(&self) -> AlignmentTransformConfig {
        self.config
    }

    /// Transforms one scheduled operation.
    ///
    /// The original operation identity, resources, and duration are preserved.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - strict alignment rejects the operation;
    /// - temporal arithmetic overflows;
    /// - start/end alignment cannot be satisfied while preserving duration;
    /// - the canonical interval cannot be reconstructed.
    pub fn transform(
        &self,
        operation: &ScheduledOperation,
        rule: &AlignmentRule,
    ) -> AlignmentTransformResult<AlignedOperation> {
        let original_start = operation.start();
        let original_end = operation.end();
        let original_duration = operation.duration();

        let transformed_interval =
            self.transform_interval(operation, rule)?;

        if transformed_interval.start() == original_start
            && transformed_interval.end() == original_end
        {
            return Ok(AlignedOperation::new(
                operation.clone(),
                original_start,
                original_end,
                original_duration,
            ));
        }

        let transformed = ScheduledOperation::new(
            operation.operation_id(),
            transformed_interval,
            operation.resources().iter().copied(),
        )
        .map_err(|_| AlignmentTransformError::InvalidInterval {
            operation: operation.operation_id(),
            start: transformed_interval.start(),
            end: transformed_interval.end(),
        })?;

        // A transformation is never allowed to change semantic duration.
        if transformed.duration() != original_duration {
            return Err(
                AlignmentTransformError::DurationWouldChange {
                    operation: operation.operation_id(),
                    duration: original_duration,
                    kind: rule.kind(),
                },
            );
        }

        Ok(AlignedOperation::new(
            transformed,
            original_start,
            original_end,
            original_duration,
        ))
    }

    /// Transforms multiple scheduled operations.
    ///
    /// The input order is preserved.
    ///
    /// This method does not construct a time-grid matrix and does not enumerate
    /// unused machine resources.
    ///
    /// Whole-schedule dependency/resource validation remains downstream.
    pub fn transform_operations<I>(
        &self,
        operations: I,
        rule: &AlignmentRule,
    ) -> AlignmentTransformResult
    where
        I: IntoIterator<Item = ScheduledOperation>,
    {
        let mut output = Vec::new();
        let mut statistics = AlignmentTransformStatistics::new();

        for operation in operations {
            statistics.operations_inspected =
                statistics.operations_inspected.saturating_add(1);

            let aligned = self.transform(&operation, rule)?;

            if aligned.changed() {
                statistics.operations_changed =
                    statistics.operations_changed.saturating_add(1);

                statistics.forward_shift_attoseconds =
                    statistics
                        .forward_shift_attoseconds
                        .checked_add(
                            aligned.forward_shift_attoseconds(),
                        )
                        .ok_or(
                            AlignmentTransformError::ArithmeticOverflow {
                                operation: operation.operation_id(),
                                operation_kind:
                                    "aggregate forward shift",
                            },
                        )?;

                statistics.backward_shift_attoseconds =
                    statistics
                        .backward_shift_attoseconds
                        .checked_add(
                            aligned.backward_shift_attoseconds(),
                        )
                        .ok_or(
                            AlignmentTransformError::ArithmeticOverflow {
                                operation: operation.operation_id(),
                                operation_kind:
                                    "aggregate backward shift",
                            },
                        )?;
            } else {
                statistics.operations_unchanged =
                    statistics.operations_unchanged.saturating_add(1);
            }

            output.push(aligned.into_operation());
        }

        Ok(AlignmentTransformResult::new(
            output,
            statistics,
        ))
    }

    // =========================================================================
    // Interval transformation
    // =========================================================================

    fn transform_interval(
        &self,
        operation: &ScheduledOperation,
        rule: &AlignmentRule,
    ) -> AlignmentTransformResult<TimeInterval> {
        let kind = rule.kind();
        let mode = rule.mode();

        if kind.is_none() {
            return Ok(operation.interval());
        }

        match kind {
            AlignmentKind::None => Ok(operation.interval()),

            AlignmentKind::Duration => {
                self.validate_duration(operation, rule)?;
                Ok(operation.interval())
            }

            AlignmentKind::Start => {
                let start = self.align_boundary(
                    operation.operation_id(),
                    operation.start(),
                    rule,
                    "start",
                )?;

                self.interval_from_start(
                    operation.operation_id(),
                    start,
                    operation.duration(),
                )
            }

            AlignmentKind::End => {
                let end = self.align_boundary(
                    operation.operation_id(),
                    operation.end(),
                    rule,
                    "end",
                )?;

                self.interval_from_end(
                    operation.operation_id(),
                    end,
                    operation.duration(),
                )
            }

            AlignmentKind::StartAndEnd => {
                self.transform_start_and_end(
                    operation,
                    rule,
                    mode,
                )
            }
        }
    }

    // =========================================================================
    // Duration validation
    // =========================================================================

    fn validate_duration(
        &self,
        operation: &ScheduledOperation,
        rule: &AlignmentRule,
    ) -> AlignmentTransformResult<()> {
        let duration = operation.duration();

        if rule.is_aligned_attoseconds(duration.attoseconds())? {
            Ok(())
        } else {
            Err(
                AlignmentTransformError::DurationWouldChange {
                    operation: operation.operation_id(),
                    duration,
                    kind: rule.kind(),
                },
            )
        }
    }

    // =========================================================================
    // Boundary alignment
    // =========================================================================

    fn align_boundary(
        &self,
        operation:
            crate::quantum::ir::core::identity::OperationId,
        point: TimePoint,
        rule: &AlignmentRule,
        operation_kind: &'static str,
    ) -> AlignmentTransformResult<TimePoint> {
        let aligned_attoseconds =
            rule.align_attoseconds(point.attoseconds())?;

        TimePoint::from_attoseconds(aligned_attoseconds).ok_or(
            AlignmentTransformError::ArithmeticOverflow {
                operation,
                operation_kind,
            },
        )
    }

    // =========================================================================
    // Start-based interval construction
    // =========================================================================

    fn interval_from_start(
        &self,
        operation:
            crate::quantum::ir::core::identity::OperationId,
        start: TimePoint,
        duration: Duration,
    ) -> AlignmentTransformResult<TimeInterval> {
        let end_attoseconds =
            start.attoseconds()
                .checked_add(duration.attoseconds())
                .ok_or(
                    AlignmentTransformError::ArithmeticOverflow {
                        operation,
                        operation_kind: "start + duration",
                    },
                )?;

        let end = TimePoint::from_attoseconds(
            end_attoseconds,
        )
        .ok_or(
            AlignmentTransformError::ArithmeticOverflow {
                operation,
                operation_kind: "construct end time",
            },
        )?;

        TimeInterval::new(start, end).map_err(
            |_| AlignmentTransformError::InvalidInterval {
                operation,
                start,
                end,
            },
        )
    }

    // =========================================================================
    // End-based interval construction
    // =========================================================================

    fn interval_from_end(
        &self,
        operation:
            crate::quantum::ir::core::identity::OperationId,
        end: TimePoint,
        duration: Duration,
    ) -> AlignmentTransformResult<TimeInterval> {
        let start_attoseconds =
            end.attoseconds()
                .checked_sub(duration.attoseconds())
                .ok_or(
                    AlignmentTransformError::ArithmeticOverflow {
                        operation,
                        operation_kind: "end - duration",
                    },
                )?;

        let start = TimePoint::from_attoseconds(
            start_attoseconds,
        )
        .ok_or(
            AlignmentTransformError::ArithmeticOverflow {
                operation,
                operation_kind: "construct start time",
            },
        )?;

        TimeInterval::new(start, end).map_err(
            |_| AlignmentTransformError::InvalidInterval {
                operation,
                start,
                end,
            },
        )
    }

    // =========================================================================
    // Start + end transformation
    // =========================================================================

    fn transform_start_and_end(
        &self,
        operation: &ScheduledOperation,
        rule: &AlignmentRule,
        mode: AlignmentMode,
    ) -> AlignmentTransformResult<TimeInterval> {
        let start_aligned =
            rule.is_aligned_attoseconds(
                operation.start().attoseconds(),
            )?;

        let end_aligned =
            rule.is_aligned_attoseconds(
                operation.end().attoseconds(),
            )?;

        if start_aligned && end_aligned {
            return Ok(operation.interval());
        }

        /*
         * If duration itself is aligned to the same grid, moving the whole
         * interval by a common grid amount preserves both boundary alignment.
         *
         * We first choose the aligned start according to the requested mode
         * and then reconstruct the end from the original duration.
         */
        let duration_aligned =
            rule.is_aligned_attoseconds(
                operation.duration().attoseconds(),
            )?;

        if !duration_aligned {
            return Err(
                AlignmentTransformError::StartAndEndIncompatible {
                    operation: operation.operation_id(),
                    start: operation.start(),
                    end: operation.end(),
                    duration: operation.duration(),
                },
            );
        }

        let aligned_start = rule.align_attoseconds(
            operation.start().attoseconds(),
        )?;

        let start = TimePoint::from_attoseconds(
            aligned_start,
        )
        .ok_or(
            AlignmentTransformError::ArithmeticOverflow {
                operation: operation.operation_id(),
                operation_kind: "construct aligned start",
            },
        )?;

        let interval = self.interval_from_start(
            operation.operation_id(),
            start,
            operation.duration(),
        )?;

        /*
         * Duration compatibility guarantees that if the selected start is on
         * the same grid, the reconstructed end is also on that grid.
         *
         * We still verify explicitly. This makes the invariant executable
         * rather than relying solely on algebraic reasoning.
         */
        let resulting_end_aligned =
            rule.is_aligned_attoseconds(
                interval.end().attoseconds(),
            )?;

        if !resulting_end_aligned {
            return Err(
                AlignmentTransformError::StartAndEndIncompatible {
                    operation: operation.operation_id(),
                    start: operation.start(),
                    end: operation.end(),
                    duration: operation.duration(),
                },
            );
        }

        /*
         * The mode is intentionally consumed by `AlignmentRule::align_*`.
         *
         * Keep this explicit match so a future alignment mode cannot silently
         * acquire transformation semantics without this module being reviewed.
         */
        match mode {
            AlignmentMode::Strict
            | AlignmentMode::Nearest
            | AlignmentMode::Ceil
            | AlignmentMode::Floor => Ok(interval),
        }
    }
}

// =============================================================================
// Convenience functions
// =============================================================================

/// Applies an alignment rule to one scheduled operation.
///
/// This is the simplest integration point for planners and transformation
/// pipelines that operate one operation at a time.
pub fn align_operation(
    operation: &ScheduledOperation,
    rule: &AlignmentRule,
) -> AlignmentTransformResult<AlignedOperation> {
    AlignmentTransformer::new().transform(operation, rule)
}

/// Applies an alignment rule to a collection of scheduled operations.
///
/// The input order is preserved.
pub fn align_operations<I>(
    operations: I,
    rule: &AlignmentRule,
) -> AlignmentTransformResult
where
    I: IntoIterator<Item = ScheduledOperation>,
{
    AlignmentTransformer::new()
        .transform_operations(operations, rule)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::core::identity::OperationId;
    use crate::quantum::ir::scheduling::{
        ScheduleResource,
        ScheduledOperation,
    };
    use crate::quantum::ir::timing::{
        Duration,
        TimeInterval,
        TimePoint,
    };
    use crate::quantum::scheduling::timing::alignment::{
        AlignmentDomain,
        AlignmentKind,
        AlignmentMode,
        AlignmentRule,
    };
    use crate::quantum::scheduling::timing::resolution::TimingResolution;

    fn operation(
        id: u64,
        start: u128,
        end: u128,
    ) -> ScheduledOperation {
        let interval = TimeInterval::new(
            TimePoint::from_attoseconds(start),
            TimePoint::from_attoseconds(end),
        )
        .expect("test interval must be valid");

        ScheduledOperation::new(
            OperationId::new(id),
            interval,
            [ScheduleResource::Channel(id)],
        )
        .expect("test operation must be valid")
    }

    fn grid(value: u128) -> TimingResolution {
        TimingResolution::attoseconds(value)
            .expect("test resolution must be positive")
    }

    fn start_rule(
        grid_size: u128,
        mode: AlignmentMode,
    ) -> AlignmentRule {
        AlignmentRule::new(
            AlignmentDomain::Operation,
            AlignmentKind::Start,
            grid(grid_size),
            mode,
        )
    }

    fn end_rule(
        grid_size: u128,
        mode: AlignmentMode,
    ) -> AlignmentRule {
        AlignmentRule::new(
            AlignmentDomain::Operation,
            AlignmentKind::End,
            grid(grid_size),
            mode,
        )
    }

    fn both_rule(
        grid_size: u128,
        mode: AlignmentMode,
    ) -> AlignmentRule {
        AlignmentRule::new(
            AlignmentDomain::Operation,
            AlignmentKind::StartAndEnd,
            grid(grid_size),
            mode,
        )
    }

    #[test]
    fn already_aligned_operation_is_unchanged() {
        let op = operation(1, 20, 30);
        let rule = start_rule(10, AlignmentMode::Strict);

        let result =
            align_operation(&op, &rule)
                .expect("aligned operation must succeed");

        assert_eq!(result.start(), TimePoint::from_attoseconds(20));
        assert_eq!(result.end(), TimePoint::from_attoseconds(30));
        assert_eq!(result.duration(), Duration::from_attoseconds(10));
        assert_eq!(result.operation().operation_id(), op.operation_id());
    }

    #[test]
    fn strict_rejects_unaligned_start() {
        let op = operation(1, 23, 33);
        let rule = start_rule(10, AlignmentMode::Strict);

        let result = align_operation(&op, &rule);

        assert!(result.is_err());
    }

    #[test]
    fn ceil_preserves_duration() {
        let op = operation(1, 23, 33);
        let rule = start_rule(10, AlignmentMode::Ceil);

        let result =
            align_operation(&op, &rule)
                .expect("ceil alignment must succeed");

        assert_eq!(
            result.start(),
            TimePoint::from_attoseconds(30)
        );
        assert_eq!(
            result.end(),
            TimePoint::from_attoseconds(40)
        );
        assert_eq!(
            result.duration(),
            Duration::from_attoseconds(10)
        );
    }

    #[test]
    fn floor_preserves_duration() {
        let op = operation(1, 23, 33);
        let rule = start_rule(10, AlignmentMode::Floor);

        let result =
            align_operation(&op, &rule)
                .expect("floor alignment must succeed");

        assert_eq!(
            result.start(),
            TimePoint::from_attoseconds(20)
        );
        assert_eq!(
            result.end(),
            TimePoint::from_attoseconds(30)
        );
        assert_eq!(
            result.duration(),
            Duration::from_attoseconds(10)
        );
    }

    #[test]
    fn nearest_uses_canonical_alignment_policy() {
        let op = operation(1, 25, 35);
        let rule = start_rule(10, AlignmentMode::Nearest);

        let result =
            align_operation(&op, &rule)
                .expect("nearest alignment must succeed");

        /*
         * The canonical alignment rule resolves exact ties deterministically.
         * The transformation delegates to that implementation rather than
         * duplicating tie-breaking policy.
         */
        assert_eq!(
            result.start(),
            TimePoint::from_attoseconds(30)
        );
        assert_eq!(
            result.end(),
            TimePoint::from_attoseconds(40)
        );
    }

    #[test]
    fn end_alignment_preserves_duration() {
        let op = operation(1, 23, 33);
        let rule = end_rule(10, AlignmentMode::Ceil);

        let result =
            align_operation(&op, &rule)
                .expect("end alignment must succeed");

        assert_eq!(
            result.end(),
            TimePoint::from_attoseconds(40)
        );
        assert_eq!(
            result.start(),
            TimePoint::from_attoseconds(30)
        );
        assert_eq!(
            result.duration(),
            Duration::from_attoseconds(10)
        );
    }

    #[test]
    fn duration_alignment_does_not_move_operation() {
        let op = operation(1, 23, 33);

        let rule = AlignmentRule::new(
            AlignmentDomain::Operation,
            AlignmentKind::Duration,
            grid(10),
            AlignmentMode::Strict,
        );

        let result =
            align_operation(&op, &rule)
                .expect("aligned duration must succeed");

        assert_eq!(result.start(), op.start());
        assert_eq!(result.end(), op.end());
        assert_eq!(result.duration(), op.duration());
    }

    #[test]
    fn duration_alignment_rejects_incompatible_duration() {
        let op = operation(1, 23, 30);

        let rule = AlignmentRule::new(
            AlignmentDomain::Operation,
            AlignmentKind::Duration,
            grid(10),
            AlignmentMode::Strict,
        );

        let result = align_operation(&op, &rule);

        assert!(result.is_err());
    }

    #[test]
    fn start_and_end_alignment_accepts_grid_compatible_duration() {
        let op = operation(1, 23, 33);

        let rule = both_rule(10, AlignmentMode::Ceil);

        let result =
            align_operation(&op, &rule)
                .expect("compatible interval must succeed");

        assert_eq!(
            result.start(),
            TimePoint::from_attoseconds(30)
        );
        assert_eq!(
            result.end(),
            TimePoint::from_attoseconds(40)
        );
        assert_eq!(
            result.duration(),
            Duration::from_attoseconds(10)
        );
    }

    #[test]
    fn start_and_end_rejects_incompatible_duration() {
        let op = operation(1, 23, 30);

        let rule = both_rule(10, AlignmentMode::Ceil);

        let result = align_operation(&op, &rule);

        assert!(result.is_err());
    }

    #[test]
    fn none_alignment_is_identity() {
        let op = operation(1, 23, 30);

        let rule = AlignmentRule::none(
            AlignmentDomain::Operation,
        );

        let result =
            align_operation(&op, &rule)
                .expect("none alignment must succeed");

        assert_eq!(result.start(), op.start());
        assert_eq!(result.end(), op.end());
        assert_eq!(result.duration(), op.duration());
    }

    #[test]
    fn operation_identity_is_preserved() {
        let op = operation(42, 23, 33);
        let rule = start_rule(10, AlignmentMode::Ceil);

        let result =
            align_operation(&op, &rule)
                .expect("alignment must succeed");

        assert_eq!(
            result.operation().operation_id(),
            OperationId::new(42)
        );
    }

    #[test]
    fn resources_are_preserved() {
        let op = operation(42, 23, 33);
        let rule = start_rule(10, AlignmentMode::Ceil);

        let result =
            align_operation(&op, &rule)
                .expect("alignment must succeed");

        assert_eq!(
            result.operation().resources(),
            op.resources()
        );
    }

    #[test]
    fn collection_preserves_input_order() {
        let first = operation(1, 23, 33);
        let second = operation(2, 43, 53);

        let rule = start_rule(10, AlignmentMode::Ceil);

        let result =
            align_operations(
                [first.clone(), second.clone()],
                &rule,
            )
            .expect("collection alignment must succeed");

        assert_eq!(
            result.operations()[0].operation_id(),
            first.operation_id()
        );
        assert_eq!(
            result.operations()[1].operation_id(),
            second.operation_id()
        );
    }

    #[test]
    fn statistics_count_changes() {
        let first = operation(1, 20, 30);
        let second = operation(2, 23, 33);

        let rule = start_rule(10, AlignmentMode::Ceil);

        let result =
            align_operations([first, second], &rule)
                .expect("alignment must succeed");

        let statistics = result.statistics();

        assert_eq!(
            statistics.operations_inspected(),
            2
        );
        assert_eq!(
            statistics.operations_changed(),
            1
        );
        assert_eq!(
            statistics.operations_unchanged(),
            1
        );
    }

    #[test]
    fn zero_duration_operation_can_be_aligned() {
        let op = operation(1, 23, 23);
        let rule = start_rule(10, AlignmentMode::Ceil);

        let result =
            align_operation(&op, &rule)
                .expect("zero-duration operation can be aligned");

        assert_eq!(
            result.start(),
            TimePoint::from_attoseconds(30)
        );
        assert_eq!(
            result.end(),
            TimePoint::from_attoseconds(30)
        );
        assert_eq!(result.duration(), Duration::ZERO);
    }

    #[test]
    fn alignment_does_not_use_qubit_identity() {
        /*
         * The test intentionally uses a generic channel resource. The
         * transformation is independent of the resource identity domain.
         */
        let op = operation(7, 13, 23);
        let rule = start_rule(10, AlignmentMode::Ceil);

        let result =
            align_operation(&op, &rule)
                .expect("alignment must succeed");

        assert_eq!(
            result.operation().resources(),
            op.resources()
        );
    }
}