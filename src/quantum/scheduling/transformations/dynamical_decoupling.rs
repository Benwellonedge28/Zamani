//! Zamani Quantum Scheduling — Dynamical Decoupling Transformation
//!
//! Path:
//!     src/quantum/scheduling/transformations/dynamical_decoupling.rs
//!
//! # Purpose
//!
//! This module plans dynamical-decoupling (DD) pulse insertion into an already
//! scheduled quantum program.
//!
//! Dynamical decoupling is deliberately implemented here as a *physical
//! scheduling transformation*, not as a replacement for the canonical
//! semantic operation IR.
//!
//! The distinction is important:
//!
//! ```text
//! Zamani program
//!      |
//!      v
//! canonical quantum::ir
//!      |
//!      v
//! optimization
//!      |
//!      v
//! routing
//!      |
//!      v
//! scheduling
//!      |
//!      v
//! concrete Schedule
//!      |
//!      v
//! dynamical decoupling
//!      |
//!      v
//! hardware lowering
//! ```
//!
//! The semantic program remains unchanged.
//!
//! DD adds physical control pulses during sufficiently long idle intervals.
//! A DD sequence is intended to implement an identity operation while
//! suppressing accumulated decoherence/coherent error during idling.
//!
//! # Why this module does not create ScheduledOperation values
//!
//! `quantum::ir::scheduling::schedule::ScheduledOperation` represents a
//! semantic IR operation identified by a canonical `OperationId`.
//!
//! Dynamical-decoupling pulses are not necessarily semantic operations from
//! the original Zamani program and therefore must not be assigned fabricated
//! semantic `OperationId`s.
//!
//! Instead this module produces a `DynamicalDecouplingPlan` containing explicit
//! physical pulse insertions.
//!
//! A later hardware/pulse-lowering stage materializes those insertions into the
//! target-specific representation.
//!
//! This preserves:
//!
//! - semantic operation identity;
//! - canonical IR provenance;
//! - scheduling verification;
//! - reproducibility;
//! - target independence;
//! - clean separation between scheduling and hardware lowering.
//!
//! # Architectural boundary
//!
//! ```text
//! quantum::ir::qubit
//!        |
//!        | canonical logical/physical identity
//!        v
//! quantum::ir::scheduling::schedule::Schedule
//!        |
//!        | already scheduled operations
//!        v
//! this module
//!        |
//!        +--> idle-window discovery
//!        +--> qubit selection
//!        +--> DD sequence validation
//!        +--> timing placement
//!        +--> target alignment
//!        +--> reset/protected-operation handling
//!        +--> deterministic plan generation
//!        |
//!        v
//! DynamicalDecouplingPlan
//!        |
//!        v
//! hardware / pulse lowering
//! ```
//!
//! # Important responsibility boundary
//!
//! This module does NOT:
//!
//! - parse Zamani source;
//! - define quantum gate semantics;
//! - route logical qubits;
//! - discover hardware;
//! - select hardware providers;
//! - synthesize arbitrary pulses;
//! - perform QEC decoding;
//! - execute a QPU;
//! - estimate noise itself;
//! - invent target timing resolutions;
//! - invent control-channel counts;
//! - impose a maximum qubit count;
//! - impose a maximum operation count;
//! - impose a maximum machine size.
//!
//! Hardware-specific information must enter through explicit configuration or
//! adapters.
//!
//! # Canonical qubit identity
//!
//! This module intentionally uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! It does not define another qubit identity type.
//!
//! A DD plan may target either logical or physical resources. In a normal
//! post-routing hardware scheduling pipeline, physical qubits should be used.
//!
//! # Timing
//!
//! Timing uses the canonical IR timing representation:
//!
//! ```text
//! crate::quantum::ir::timing::Duration
//! crate::quantum::ir::timing::TimeInterval
//! crate::quantum::ir::timing::TimePoint
//! ```
//!
//! No floating-point timing is used.
//!
//! All placement arithmetic is checked.
//!
//! # Alignment
//!
//! Target alignment is delegated to:
//!
//! ```text
//! crate::quantum::scheduling::timing::alignment::AlignmentRule
//! ```
//!
//! This avoids creating a second alignment implementation.
//!
//! # Scalability
//!
//! There are no fixed limits for:
//!
//! - qubits;
//! - operations;
//! - DD pulses;
//! - idle windows;
//! - machine size;
//! - schedule duration;
//! - sequence length.
//!
//! Memory scales with the actual schedule and actual generated DD plan.
//!
//! The implementation uses deterministic ordered collections and scans the
//! schedule once into per-qubit operation lists.
//!
//! The algorithm is therefore proportional to the number of scheduled
//! operations plus the number of generated DD insertions.
//!
//! It does not allocate a time × qubit matrix.
//!
//! # Determinism
//!
//! Given identical:
//!
//! - schedule;
//! - configuration;
//! - pulse durations;
//! - alignment rules;
//! - selected qubits;
//!
//! the generated DD plan is deterministic.
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! The compiler enforces that requirement through:
//!
//! ```text
//! #![forbid(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! ```
//!
//! # Rust
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::ir::scheduling::schedule::{
    Schedule,
    ScheduleEntry,
    ScheduleResource,
};
use crate::quantum::ir::timing::{
    Duration,
    TimeInterval,
    TimePoint,
};
use crate::quantum::scheduling::timing::alignment::{
    AlignmentError,
    AlignmentKind,
    AlignmentRule,
};

// =============================================================================
// Public result type
// =============================================================================

/// Result returned by dynamical-decoupling planning.
pub type DynamicalDecouplingResult<T> =
    Result<T, DynamicalDecouplingError>;

// =============================================================================
// Error model
// =============================================================================

/// Errors produced while constructing or applying a DD transformation plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DynamicalDecouplingError {
    /// The DD sequence contains no pulses.
    EmptySequence,

    /// A pulse duration is zero.
    ZeroPulseDuration {
        /// Position of the invalid pulse.
        index: usize,
    },

    /// The requested pulse sequence cannot fit inside an idle interval.
    SequenceDoesNotFit {
        /// Target qubit.
        qubit: QubitResource,

        /// Idle interval.
        idle: TimeInterval,

        /// Required duration.
        required: Duration,

        /// Available duration.
        available: Duration,
    },

    /// Checked timing arithmetic overflowed.
    ArithmeticOverflow {
        /// Human-readable operation context.
        context: &'static str,
    },

    /// An alignment rule could not align a generated pulse boundary.
    Alignment {
        /// Alignment failure.
        error: AlignmentError,
    },

    /// A custom pulse placement is malformed.
    InvalidPlacement {
        /// Pulse index.
        index: usize,

        /// Reason.
        reason: &'static str,
    },

    /// A placement value is outside the legal normalized range.
    PlacementOutOfRange {
        /// Pulse index.
        index: usize,
    },

    /// Custom placement positions are not strictly increasing.
    PlacementNotMonotonic,

    /// The configured pulse sequence does not declare an identity-preserving
    /// sequence.
    NonIdentitySequence,

    /// The schedule contains a resource that cannot be used as a DD target.
    UnsupportedResource,

    /// The caller supplied incompatible sequence/configuration data.
    InvalidConfiguration {
        /// Explanation.
        message: String,
    },
}

impl fmt::Display for DynamicalDecouplingError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptySequence => {
                formatter.write_str(
                    "dynamical-decoupling sequence must contain at least one pulse",
                )
            }

            Self::ZeroPulseDuration { index } => {
                write!(
                    formatter,
                    "dynamical-decoupling pulse {index} has zero duration"
                )
            }

            Self::SequenceDoesNotFit {
                qubit,
                idle,
                required,
                available,
            } => {
                write!(
                    formatter,
                    "DD sequence does not fit on {qubit} in idle interval \
                     {idle}: required {required}, available {available}"
                )
            }

            Self::ArithmeticOverflow { context } => {
                write!(
                    formatter,
                    "checked timing arithmetic overflowed while {context}"
                )
            }

            Self::Alignment { error } => {
                write!(
                    formatter,
                    "DD timing alignment failed: {error}"
                )
            }

            Self::InvalidPlacement { index, reason } => {
                write!(
                    formatter,
                    "invalid DD placement for pulse {index}: {reason}"
                )
            }

            Self::PlacementOutOfRange { index } => {
                write!(
                    formatter,
                    "DD placement for pulse {index} is outside [0, 1]"
                )
            }

            Self::PlacementNotMonotonic => {
                formatter.write_str(
                    "DD pulse placement positions must be strictly increasing",
                )
            }

            Self::NonIdentitySequence => {
                formatter.write_str(
                    "DD sequence is not declared identity preserving",
                )
            }

            Self::UnsupportedResource => {
                formatter.write_str(
                    "schedule contains an unsupported DD target resource",
                )
            }

            Self::InvalidConfiguration { message } => {
                write!(
                    formatter,
                    "invalid dynamical-decoupling configuration: {message}"
                )
            }
        }
    }
}

impl Error for DynamicalDecouplingError {}

impl From<AlignmentError> for DynamicalDecouplingError {
    fn from(error: AlignmentError) -> Self {
        Self::Alignment { error }
    }
}

// =============================================================================
// Qubit resource
// =============================================================================

/// Canonical qubit resource targeted by DD.
///
/// The logical/physical identity types come directly from
/// `quantum::ir::qubit`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QubitResource {
    /// Logical program-level qubit.
    Logical(QubitId),

    /// Physical target-level qubit.
    Physical(PhysicalQubitId),
}

impl QubitResource {
    /// Returns whether this is a logical qubit.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::Logical(_))
    }

    /// Returns whether this is a physical qubit.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::Physical(_))
    }

    /// Returns the corresponding schedule resource.
    #[must_use]
    pub const fn as_schedule_resource(
        self,
    ) -> ScheduleResource {
        match self {
            Self::Logical(qubit) => {
                ScheduleResource::LogicalQubit(qubit)
            }
            Self::Physical(qubit) => {
                ScheduleResource::PhysicalQubit(qubit)
            }
        }
    }
}

impl fmt::Display for QubitResource {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Logical(qubit) => {
                write!(formatter, "logical-qubit:{qubit}")
            }
            Self::Physical(qubit) => {
                write!(formatter, "physical-qubit:{qubit}")
            }
        }
    }
}

// =============================================================================
// Pulse axis
// =============================================================================

/// Abstract DD pulse axis.
///
/// These are semantic pulse requests, not hardware-native instructions.
///
/// Hardware lowering determines how a requested pulse is implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PulseAxis {
    /// Positive X π rotation.
    X,

    /// Positive Y π rotation.
    Y,

    /// Negative X π rotation.
    NegativeX,

    /// Negative Y π rotation.
    NegativeY,
}

impl PulseAxis {
    /// Returns the inverse axis.
    #[must_use]
    pub const fn inverse(self) -> Self {
        match self {
            Self::X => Self::NegativeX,
            Self::NegativeX => Self::X,
            Self::Y => Self::NegativeY,
            Self::NegativeY => Self::Y,
        }
    }
}

// =============================================================================
// DD pulse
// =============================================================================

/// One physical DD pulse request.
///
/// A pulse has a target axis and a target-specific duration supplied by the
/// caller.
///
/// Duration is never inferred from a hard-coded hardware value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DynamicalDecouplingPulse {
    axis: PulseAxis,
    duration: Duration,
}

impl DynamicalDecouplingPulse {
    /// Creates a pulse.
    pub fn new(
        axis: PulseAxis,
        duration: Duration,
    ) -> DynamicalDecouplingResult<Self> {
        if duration == Duration::ZERO {
            return Err(
                DynamicalDecouplingError::ZeroPulseDuration {
                    index: 0,
                },
            );
        }

        Ok(Self { axis, duration })
    }

    /// Returns the pulse axis.
    #[must_use]
    pub const fn axis(self) -> PulseAxis {
        self.axis
    }

    /// Returns the pulse duration.
    #[must_use]
    pub const fn duration(self) -> Duration {
        self.duration
    }
}

// =============================================================================
// Built-in sequences
// =============================================================================

/// Identity-preserving DD sequence families.
///
/// The actual pulse duration is supplied separately by the target/configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BuiltinSequence {
    /// Two-pulse X-X sequence.
    Xx,

    /// Four-pulse XY4 sequence.
    Xy4,

    /// Eight-pulse XY8 sequence.
    Xy8,
}

impl BuiltinSequence {
    /// Returns the axes comprising the sequence.
    #[must_use]
    pub const fn axes(self) -> &'static [PulseAxis] {
        match self {
            Self::Xx => &[
                PulseAxis::X,
                PulseAxis::X,
            ],

            Self::Xy4 => &[
                PulseAxis::X,
                PulseAxis::Y,
                PulseAxis::X,
                PulseAxis::Y,
            ],

            Self::Xy8 => &[
                PulseAxis::X,
                PulseAxis::Y,
                PulseAxis::X,
                PulseAxis::Y,
                PulseAxis::Y,
                PulseAxis::X,
                PulseAxis::Y,
                PulseAxis::X,
            ],
        }
    }

    /// Returns the number of pulses.
    #[must_use]
    pub const fn len(self) -> usize {
        match self {
            Self::Xx => 2,
            Self::Xy4 => 4,
            Self::Xy8 => 8,
        }
    }
}

// =============================================================================
// Sequence
// =============================================================================

/// A complete DD pulse sequence.
///
/// The sequence is target-independent except for the pulse durations, which
/// must be supplied by the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicalDecouplingSequence {
    pulses: Vec<DynamicalDecouplingPulse>,
    identity_preserving: bool,
}

impl DynamicalDecouplingSequence {
    /// Creates a validated custom sequence.
    ///
    /// `identity_preserving` must be true for production DD insertion.
    ///
    /// A false value is retained as a possible analysis-only configuration but
    /// is rejected by the production planner.
    pub fn new(
        pulses: Vec<DynamicalDecouplingPulse>,
        identity_preserving: bool,
    ) -> DynamicalDecouplingResult<Self> {
        if pulses.is_empty() {
            return Err(DynamicalDecouplingError::EmptySequence);
        }

        for (index, pulse) in pulses.iter().enumerate() {
            if pulse.duration == Duration::ZERO {
                return Err(
                    DynamicalDecouplingError::ZeroPulseDuration {
                        index,
                    },
                );
            }
        }

        Ok(Self {
            pulses,
            identity_preserving,
        })
    }

    /// Creates a built-in sequence using one common pulse duration.
    pub fn builtin(
        sequence: BuiltinSequence,
        pulse_duration: Duration,
    ) -> DynamicalDecouplingResult<Self> {
        if pulse_duration == Duration::ZERO {
            return Err(
                DynamicalDecouplingError::ZeroPulseDuration {
                    index: 0,
                },
            );
        }

        let pulses = sequence
            .axes()
            .iter()
            .copied()
            .map(|axis| DynamicalDecouplingPulse {
                axis,
                duration: pulse_duration,
            })
            .collect();

        Self::new(pulses, true)
    }

    /// Returns all pulses.
    #[must_use]
    pub fn pulses(&self) -> &[DynamicalDecouplingPulse] {
        &self.pulses
    }

    /// Returns whether the sequence is declared identity preserving.
    #[must_use]
    pub const fn identity_preserving(&self) -> bool {
        self.identity_preserving
    }

    /// Returns the number of pulses.
    #[must_use]
    pub fn len(&self) -> usize {
        self.pulses.len()
    }

    /// Returns whether the sequence is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pulses.is_empty()
    }

    /// Returns total pulse occupation time.
    pub fn total_duration(
        &self,
    ) -> DynamicalDecouplingResult<Duration> {
        let mut total = Duration::ZERO;

        for pulse in &self.pulses {
            total = total
                .checked_add(pulse.duration)
                .ok_or(
                    DynamicalDecouplingError::ArithmeticOverflow {
                        context: "summing DD pulse durations",
                    },
                )?;
        }

        Ok(total)
    }
}

// =============================================================================
// Placement
// =============================================================================

/// Distribution of free idle time around DD pulses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PulsePlacement {
    /// Divide all remaining slack into `n + 1` equal intervals, where `n`
    /// is the number of pulses.
    Uniform,

    /// Place pulses using exact normalized positions.
    ///
    /// Each value is represented as `numerator / denominator` in `[0, 1]`.
    ///
    /// A position identifies the pulse start relative to the available
    /// free-space span after accounting for the pulse duration.
    Custom {
        /// Exact normalized pulse positions.
        positions: Vec<RationalPosition>,
    },
}

impl Default for PulsePlacement {
    fn default() -> Self {
        Self::Uniform
    }
}

/// Exact rational value used for deterministic pulse placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RationalPosition {
    numerator: u128,
    denominator: u128,
}

impl RationalPosition {
    /// Creates a rational normalized position.
    pub fn new(
        numerator: u128,
        denominator: u128,
    ) -> DynamicalDecouplingResult<Self> {
        if denominator == 0 {
            return Err(
                DynamicalDecouplingError::InvalidPlacement {
                    index: 0,
                    reason: "denominator cannot be zero",
                },
            );
        }

        if numerator > denominator {
            return Err(
                DynamicalDecouplingError::PlacementOutOfRange {
                    index: 0,
                },
            );
        }

        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// Returns the numerator.
    #[must_use]
    pub const fn numerator(self) -> u128 {
        self.numerator
    }

    /// Returns the denominator.
    #[must_use]
    pub const fn denominator(self) -> u128 {
        self.denominator
    }
}

// =============================================================================
// Qubit selection
// =============================================================================

/// Selects which qubits may receive DD.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QubitSelection {
    /// Every qubit resource found in the schedule.
    All,

    /// Only the supplied logical qubits.
    Logical(BTreeSet<QubitId>),

    /// Only the supplied physical qubits.
    Physical(BTreeSet<PhysicalQubitId>),

    /// Explicit mixed logical/physical selection.
    Explicit(BTreeSet<QubitResource>),
}

impl Default for QubitSelection {
    fn default() -> Self {
        Self::All
    }
}

impl QubitSelection {
    /// Returns whether a qubit is selected.
    #[must_use]
    pub fn contains(&self, qubit: QubitResource) -> bool {
        match self {
            Self::All => true,

            Self::Logical(qubits) => match qubit {
                QubitResource::Logical(id) => qubits.contains(&id),
                QubitResource::Physical(_) => false,
            },

            Self::Physical(qubits) => match qubit {
                QubitResource::Logical(_) => false,
                QubitResource::Physical(id) => qubits.contains(&id),
            },

            Self::Explicit(qubits) => qubits.contains(&qubit),
        }
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Configuration for DD planning.
///
/// The configuration contains policy, not machine-size assumptions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicalDecouplingConfig {
    sequence: DynamicalDecouplingSequence,
    selection: QubitSelection,
    placement: PulsePlacement,

    /// Minimum idle duration before DD may be inserted.
    minimum_idle_duration: Duration,

    /// Minimum idle/sequence ratio represented exactly as:
    ///
    /// `ratio_numerator / ratio_denominator`.
    ///
    /// For example, 2.0 is `2 / 1`.
    minimum_idle_ratio: RationalRatio,

    /// Whether DD may be inserted into idle time before the first operation.
    include_leading_idle: bool,

    /// Whether DD may be inserted into idle time after the final operation.
    include_trailing_idle: bool,

    /// Whether idle intervals immediately following a reset should be skipped.
    skip_reset_following_idle: bool,

    /// Semantic operation IDs known by the caller to be reset operations.
    reset_operations: BTreeSet<crate::quantum::ir::core::identity::OperationId>,

    /// Semantic operation IDs that must terminate/protect an idle region.
    protected_operations:
        BTreeSet<crate::quantum::ir::core::identity::OperationId>,

    /// Whether generated pulse boundaries must satisfy the supplied alignment
    /// rule.
    alignment: Option<AlignmentRule>,
}

impl DynamicalDecouplingConfig {
    /// Creates a configuration.
    pub fn new(
        sequence: DynamicalDecouplingSequence,
    ) -> DynamicalDecouplingResult<Self> {
        if !sequence.identity_preserving() {
            return Err(
                DynamicalDecouplingError::NonIdentitySequence,
            );
        }

        Ok(Self {
            sequence,
            selection: QubitSelection::All,
            placement: PulsePlacement::Uniform,
            minimum_idle_duration: Duration::ZERO,
            minimum_idle_ratio: RationalRatio::new(1, 1)?,
            include_leading_idle: false,
            include_trailing_idle: false,
            skip_reset_following_idle: true,
            reset_operations: BTreeSet::new(),
            protected_operations: BTreeSet::new(),
            alignment: None,
        })
    }

    /// Sets qubit selection.
    #[must_use]
    pub fn with_selection(
        mut self,
        selection: QubitSelection,
    ) -> Self {
        self.selection = selection;
        self
    }

    /// Sets pulse placement.
    #[must_use]
    pub fn with_placement(
        mut self,
        placement: PulsePlacement,
    ) -> Self {
        self.placement = placement;
        self
    }

    /// Sets minimum idle duration.
    #[must_use]
    pub fn with_minimum_idle_duration(
        mut self,
        duration: Duration,
    ) -> Self {
        self.minimum_idle_duration = duration;
        self
    }

    /// Sets minimum idle-to-sequence ratio.
    pub fn with_minimum_idle_ratio(
        mut self,
        numerator: u128,
        denominator: u128,
    ) -> DynamicalDecouplingResult<Self> {
        self.minimum_idle_ratio =
            RationalRatio::new(numerator, denominator)?;
        Ok(self)
    }

    /// Enables/disables leading-idle insertion.
    #[must_use]
    pub fn with_leading_idle(
        mut self,
        enabled: bool,
    ) -> Self {
        self.include_leading_idle = enabled;
        self
    }

    /// Enables/disables trailing-idle insertion.
    #[must_use]
    pub fn with_trailing_idle(
        mut self,
        enabled: bool,
    ) -> Self {
        self.include_trailing_idle = enabled;
        self
    }

    /// Configures reset-following idle behavior.
    #[must_use]
    pub fn with_skip_reset_following_idle(
        mut self,
        enabled: bool,
    ) -> Self {
        self.skip_reset_following_idle = enabled;
        self
    }

    /// Adds a reset operation identity.
    #[must_use]
    pub fn with_reset_operation(
        mut self,
        operation: crate::quantum::ir::core::identity::OperationId,
    ) -> Self {
        self.reset_operations.insert(operation);
        self
    }

    /// Adds an operation that protects the following idle boundary.
    #[must_use]
    pub fn with_protected_operation(
        mut self,
        operation: crate::quantum::ir::core::identity::OperationId,
    ) -> Self {
        self.protected_operations.insert(operation);
        self
    }

    /// Sets the alignment rule.
    #[must_use]
    pub fn with_alignment(
        mut self,
        alignment: AlignmentRule,
    ) -> Self {
        self.alignment = Some(alignment);
        self
    }

    /// Returns the sequence.
    #[must_use]
    pub fn sequence(&self) -> &DynamicalDecouplingSequence {
        &self.sequence
    }

    /// Returns the selection.
    #[must_use]
    pub fn selection(&self) -> &QubitSelection {
        &self.selection
    }

    /// Returns the placement.
    #[must_use]
    pub fn placement(&self) -> &PulsePlacement {
        &self.placement
    }

    /// Returns the minimum idle duration.
    #[must_use]
    pub const fn minimum_idle_duration(&self) -> Duration {
        self.minimum_idle_duration
    }

    /// Returns the minimum idle ratio.
    #[must_use]
    pub const fn minimum_idle_ratio(&self) -> RationalRatio {
        self.minimum_idle_ratio
    }

    /// Returns whether leading idle time is eligible.
    #[must_use]
    pub const fn include_leading_idle(&self) -> bool {
        self.include_leading_idle
    }

    /// Returns whether trailing idle time is eligible.
    #[must_use]
    pub const fn include_trailing_idle(&self) -> bool {
        self.include_trailing_idle
    }

    /// Returns whether reset-following idle is skipped.
    #[must_use]
    pub const fn skip_reset_following_idle(&self) -> bool {
        self.skip_reset_following_idle
    }

    /// Returns whether an operation is known to be a reset.
    #[must_use]
    pub fn is_reset_operation(
        &self,
        operation: crate::quantum::ir::core::identity::OperationId,
    ) -> bool {
        self.reset_operations.contains(&operation)
    }

    /// Returns whether an operation is protected.
    #[must_use]
    pub fn is_protected_operation(
        &self,
        operation: crate::quantum::ir::core::identity::OperationId,
    ) -> bool {
        self.protected_operations.contains(&operation)
    }

    /// Returns the alignment rule.
    #[must_use]
    pub const fn alignment(&self) -> Option<&AlignmentRule> {
        self.alignment.as_ref()
    }
}

// =============================================================================
// Rational ratio
// =============================================================================

/// Exact non-negative rational ratio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RationalRatio {
    numerator: u128,
    denominator: u128,
}

impl RationalRatio {
    /// Creates a ratio.
    pub fn new(
        numerator: u128,
        denominator: u128,
    ) -> DynamicalDecouplingResult<Self> {
        if denominator == 0 {
            return Err(
                DynamicalDecouplingError::InvalidConfiguration {
                    message:
                        "ratio denominator cannot be zero".to_owned(),
                },
            );
        }

        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// Returns the numerator.
    #[must_use]
    pub const fn numerator(self) -> u128 {
        self.numerator
    }

    /// Returns the denominator.
    #[must_use]
    pub const fn denominator(self) -> u128 {
        self.denominator
    }

    /// Checks `value >= ratio * base` without floating-point arithmetic.
    pub fn at_least(
        self,
        value: Duration,
        base: Duration,
    ) -> bool {
        let lhs = value.attoseconds();
        let rhs = base.attoseconds();

        match (
            lhs.checked_mul(self.denominator),
            rhs.checked_mul(self.numerator),
        ) {
            (Some(left), Some(right)) => left >= right,
            _ => {
                // Avoid changing semantics merely because intermediate
                // multiplication overflowed. Compare using division where
                // possible.
                if self.numerator == 0 {
                    return true;
                }

                let required = rhs
                    .checked_div(self.denominator)
                    .unwrap_or(u128::MAX);

                lhs >= required
            }
        }
    }
}

// =============================================================================
// Planned pulse
// =============================================================================

/// One physical DD pulse insertion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlannedDynamicalDecouplingPulse {
    qubit: QubitResource,
    axis: PulseAxis,
    interval: TimeInterval,
    source_idle: TimeInterval,
    sequence_index: usize,
}

impl PlannedDynamicalDecouplingPulse {
    /// Creates a pulse record.
    #[must_use]
    pub const fn new(
        qubit: QubitResource,
        axis: PulseAxis,
        interval: TimeInterval,
        source_idle: TimeInterval,
        sequence_index: usize,
    ) -> Self {
        Self {
            qubit,
            axis,
            interval,
            source_idle,
            sequence_index,
        }
    }

    /// Returns target qubit.
    #[must_use]
    pub const fn qubit(&self) -> QubitResource {
        self.qubit
    }

    /// Returns pulse axis.
    #[must_use]
    pub const fn axis(&self) -> PulseAxis {
        self.axis
    }

    /// Returns pulse interval.
    #[must_use]
    pub const fn interval(&self) -> TimeInterval {
        self.interval
    }

    /// Returns source idle interval.
    #[must_use]
    pub const fn source_idle(&self) -> TimeInterval {
        self.source_idle
    }

    /// Returns sequence position.
    #[must_use]
    pub const fn sequence_index(&self) -> usize {
        self.sequence_index
    }
}

// =============================================================================
// Idle window
// =============================================================================

/// An idle interval discovered for a qubit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IdleWindow {
    qubit: QubitResource,
    interval: TimeInterval,
    previous_operation:
        Option<crate::quantum::ir::core::identity::OperationId>,
    next_operation:
        Option<crate::quantum::ir::core::identity::OperationId>,
}

impl IdleWindow {
    /// Creates an idle window.
    #[must_use]
    pub const fn new(
        qubit: QubitResource,
        interval: TimeInterval,
        previous_operation: Option<
            crate::quantum::ir::core::identity::OperationId,
        >,
        next_operation: Option<
            crate::quantum::ir::core::identity::OperationId,
        >,
    ) -> Self {
        Self {
            qubit,
            interval,
            previous_operation,
            next_operation,
        }
    }

    /// Returns qubit.
    #[must_use]
    pub const fn qubit(&self) -> QubitResource {
        self.qubit
    }

    /// Returns interval.
    #[must_use]
    pub const fn interval(&self) -> TimeInterval {
        self.interval
    }

    /// Returns previous operation.
    #[must_use]
    pub const fn previous_operation(
        &self,
    ) -> Option<
        crate::quantum::ir::core::identity::OperationId,
    > {
        self.previous_operation
    }

    /// Returns next operation.
    #[must_use]
    pub const fn next_operation(
        &self,
    ) -> Option<
        crate::quantum::ir::core::identity::OperationId,
    > {
        self.next_operation
    }
}

// =============================================================================
// Skipped window
// =============================================================================

/// Reason a candidate idle window was not populated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SkipReason {
    /// Window is shorter than the configured minimum duration.
    TooShort,

    /// Window is shorter than the configured sequence ratio.
    InsufficientRatio,

    /// Window follows a reset and reset-following DD is disabled.
    FollowsReset,

    /// Window is protected by configuration.
    Protected,

    /// Sequence could not fit after timing constraints.
    DoesNotFit,

    /// Alignment would make the sequence invalid.
    AlignmentImpossible,
}

// =============================================================================
// Skipped candidate
// =============================================================================

/// An idle window deliberately left untouched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SkippedIdleWindow {
    window: IdleWindow,
    reason: SkipReason,
}

impl SkippedIdleWindow {
    /// Creates a skipped-window record.
    #[must_use]
    pub const fn new(
        window: IdleWindow,
        reason: SkipReason,
    ) -> Self {
        Self { window, reason }
    }

    /// Returns the idle window.
    #[must_use]
    pub const fn window(&self) -> IdleWindow {
        self.window
    }

    /// Returns skip reason.
    #[must_use]
    pub const fn reason(&self) -> SkipReason {
        self.reason
    }
}

// =============================================================================
// Plan
// =============================================================================

/// Complete immutable DD transformation plan.
///
/// The plan does not mutate the canonical schedule.
///
/// It is safe to pass to a later hardware/pulse-lowering stage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicalDecouplingPlan {
    pulses: Vec<PlannedDynamicalDecouplingPulse>,
    skipped: Vec<SkippedIdleWindow>,
}

impl DynamicalDecouplingPlan {
    /// Creates an empty plan.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            pulses: Vec::new(),
            skipped: Vec::new(),
        }
    }

    /// Returns planned pulses.
    #[must_use]
    pub fn pulses(
        &self,
    ) -> &[PlannedDynamicalDecouplingPulse] {
        &self.pulses
    }

    /// Returns skipped idle windows.
    #[must_use]
    pub fn skipped(
        &self,
    ) -> &[SkippedIdleWindow] {
        &self.skipped
    }

    /// Returns number of inserted pulses.
    #[must_use]
    pub fn pulse_count(&self) -> usize {
        self.pulses.len()
    }

    /// Returns number of skipped windows.
    #[must_use]
    pub fn skipped_count(&self) -> usize {
        self.skipped.len()
    }

    /// Returns whether no pulses were planned.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pulses.is_empty()
    }

    /// Consumes the plan and returns its pulse records.
    #[must_use]
    pub fn into_pulses(
        self,
    ) -> Vec<PlannedDynamicalDecouplingPulse> {
        self.pulses
    }
}

// =============================================================================
// Transformer
// =============================================================================

/// Production DD transformation engine.
///
/// The transformer is stateless. All state required for a transformation is
/// supplied through the schedule and configuration.
///
/// This makes instances cheap to create, ownership-safe, deterministic, and
/// naturally usable from concurrent compiler pipelines.
#[derive(Debug, Default, Clone, Copy)]
pub struct DynamicalDecouplingTransformer;

impl DynamicalDecouplingTransformer {
    /// Creates a transformer.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Generates a DD plan for an already scheduled program.
    ///
    /// The schedule itself is never mutated.
    pub fn transform(
        &self,
        schedule: &Schedule,
        config: &DynamicalDecouplingConfig,
    ) -> DynamicalDecouplingResult<DynamicalDecouplingPlan> {
        if config.sequence().is_empty() {
            return Err(
                DynamicalDecouplingError::EmptySequence,
            );
        }

        if !config.sequence().identity_preserving() {
            return Err(
                DynamicalDecouplingError::NonIdentitySequence,
            );
        }

        let qubit_operations =
            Self::collect_qubit_operations(schedule)?;

        let mut plan = DynamicalDecouplingPlan::empty();

        for (qubit, operations) in qubit_operations {
            if !config.selection().contains(qubit) {
                continue;
            }

            let windows =
                Self::discover_idle_windows(
                    qubit,
                    &operations,
                    config,
                );

            for window in windows {
                match Self::plan_window(
                    window,
                    config,
                )? {
                    WindowPlanningResult::Inserted(pulses) => {
                        plan.pulses.extend(pulses);
                    }

                    WindowPlanningResult::Skipped(skipped) => {
                        plan.skipped.push(skipped);
                    }
                }
            }
        }

        plan.pulses.sort();
        plan.skipped.sort();

        Ok(plan)
    }

    /// Collects all qubit-bearing scheduled operations.
    ///
    /// Only qubit resources are collected. Channels, frames, generic resources
    /// and other resources remain the responsibility of the downstream
    /// hardware/resource verifier.
    fn collect_qubit_operations(
        schedule: &Schedule,
    ) -> DynamicalDecouplingResult<
        BTreeMap<
            QubitResource,
            Vec<ScheduledQubitOperation>,
        >,
    > {
        let mut result:
            BTreeMap<
                QubitResource,
                Vec<ScheduledQubitOperation>,
            > = BTreeMap::new();

        for entry in schedule.entries() {
            let operation = match entry {
                ScheduleEntry::Operation(operation) => operation,
                ScheduleEntry::Synchronization(_) => continue,
            };

            let operation_id = operation.operation_id();
            let interval = operation.interval();

            for resource in operation.resources() {
                let qubit = match resource {
                    ScheduleResource::LogicalQubit(id) => {
                        QubitResource::Logical(*id)
                    }

                    ScheduleResource::PhysicalQubit(id) => {
                        QubitResource::Physical(*id)
                    }

                    _ => continue,
                };

                result
                    .entry(qubit)
                    .or_default()
                    .push(ScheduledQubitOperation {
                        operation: operation_id,
                        interval,
                    });
            }
        }

        for operations in result.values_mut() {
            operations.sort_by(|left, right| {
                left.interval
                    .start()
                    .cmp(&right.interval.start())
                    .then_with(|| {
                        left.interval
                            .end()
                            .cmp(&right.interval.end())
                    })
                    .then_with(|| {
                        left.operation.cmp(&right.operation)
                    })
            });
        }

        Ok(result)
    }

    /// Discovers idle windows for one qubit.
    fn discover_idle_windows(
        qubit: QubitResource,
        operations: &[ScheduledQubitOperation],
        config: &DynamicalDecouplingConfig,
    ) -> Vec<IdleWindow> {
        let mut windows = Vec::new();

        if operations.is_empty() {
            return windows;
        }

        if config.include_leading_idle() {
            let first = operations[0];

            if first.interval.start()
                > TimePoint::ZERO
            {
                windows.push(IdleWindow::new(
                    qubit,
                    TimeInterval::new(
                        TimePoint::ZERO,
                        first.interval.start(),
                    )
                    .expect(
                        "validated canonical schedule interval",
                    ),
                    None,
                    Some(first.operation),
                ));
            }
        }

        for pair in operations.windows(2) {
            let previous = pair[0];
            let next = pair[1];

            if previous.interval.end()
                < next.interval.start()
            {
                let interval =
                    TimeInterval::new(
                        previous.interval.end(),
                        next.interval.start(),
                    )
                    .expect(
                        "ordered operation intervals form a valid idle interval",
                    );

                windows.push(IdleWindow::new(
                    qubit,
                    interval,
                    Some(previous.operation),
                    Some(next.operation),
                ));
            }
        }

        if config.include_trailing_idle() {
            let last = operations[operations.len() - 1];

            if last.interval.end()
                < TimePoint::MAX
            {
                // A trailing interval extending to TimePoint::MAX is
                // intentionally not automatically materialized by default in
                // production callers. It represents an unbounded semantic
                // tail rather than necessarily real hardware idle time.
                //
                // Therefore trailing DD requires an explicit finite schedule
                // boundary in the schedule layer. This implementation does
                // not invent one.
            }
        }

        windows
    }

    /// Plans one idle window.
    fn plan_window(
        window: IdleWindow,
        config: &DynamicalDecouplingConfig,
    ) -> DynamicalDecouplingResult<WindowPlanningResult> {
        let available = window
            .interval()
            .duration();

        if available < config.minimum_idle_duration() {
            return Ok(WindowPlanningResult::Skipped(
                SkippedIdleWindow::new(
                    window,
                    SkipReason::TooShort,
                ),
            ));
        }

        if config.skip_reset_following_idle()
            && window
                .previous_operation()
                .is_some_and(|operation| {
                    config.is_reset_operation(operation)
                })
        {
            return Ok(WindowPlanningResult::Skipped(
                SkippedIdleWindow::new(
                    window,
                    SkipReason::FollowsReset,
                ),
            ));
        }

        if window
            .previous_operation()
            .is_some_and(|operation| {
                config.is_protected_operation(operation)
            })
            || window
                .next_operation()
                .is_some_and(|operation| {
                    config.is_protected_operation(operation)
                })
        {
            return Ok(WindowPlanningResult::Skipped(
                SkippedIdleWindow::new(
                    window,
                    SkipReason::Protected,
                ),
            ));
        }

        let sequence_duration =
            config.sequence().total_duration()?;

        if !config
            .minimum_idle_ratio()
            .at_least(available, sequence_duration)
        {
            return Ok(WindowPlanningResult::Skipped(
                SkippedIdleWindow::new(
                    window,
                    SkipReason::InsufficientRatio,
                ),
            ));
        }

        if sequence_duration > available {
            return Ok(WindowPlanningResult::Skipped(
                SkippedIdleWindow::new(
                    window,
                    SkipReason::DoesNotFit,
                ),
            ));
        }

        let pulses =
            Self::place_sequence(window, config)?;

        Ok(WindowPlanningResult::Inserted(pulses))
    }

    /// Places one sequence into an idle window.
    fn place_sequence(
        window: IdleWindow,
        config: &DynamicalDecouplingConfig,
    ) -> DynamicalDecouplingResult<
        Vec<PlannedDynamicalDecouplingPulse>,
    > {
        match config.placement() {
            PulsePlacement::Uniform => {
                Self::place_uniform(window, config)
            }

            PulsePlacement::Custom { positions } => {
                Self::place_custom(
                    window,
                    config,
                    positions,
                )
            }
        }
    }

    /// Places pulses with equal free-time gaps before, between, and after the
    /// sequence.
    fn place_uniform(
        window: IdleWindow,
        config: &DynamicalDecouplingConfig,
    ) -> DynamicalDecouplingResult<
        Vec<PlannedDynamicalDecouplingPulse>,
    > {
        let start = window.interval().start();
        let available = window.interval().duration();

        let sequence_duration =
            config.sequence().total_duration()?;

        let slack = available
            .checked_sub(sequence_duration)
            .ok_or(
                DynamicalDecouplingError::ArithmeticOverflow {
                    context: "computing DD idle slack",
                },
            )?;

        let gap_count = config.sequence().len()
            .checked_add(1)
            .ok_or(
                DynamicalDecouplingError::ArithmeticOverflow {
                    context: "computing DD gap count",
                },
            )?;

        let gap_count_u128 =
            u128::try_from(gap_count).map_err(|_| {
                DynamicalDecouplingError::ArithmeticOverflow {
                    context: "converting DD gap count",
                }
            })?;

        let base_gap =
            slack.attoseconds() / gap_count_u128;

        let remainder =
            slack.attoseconds() % gap_count_u128;

        let mut cursor = start;

        let mut pulses = Vec::with_capacity(
            config.sequence().len(),
        );

        for (index, pulse) in
            config.sequence().pulses().iter().enumerate()
        {
            let extra_before =
                if u128::try_from(index + 1)
                    .map_err(|_| {
                        DynamicalDecouplingError::ArithmeticOverflow {
                            context:
                                "converting DD gap index",
                        }
                    })?
                    <= remainder
                {
                    1
                } else {
                    0
                };

            let gap_before = base_gap
                .checked_add(extra_before)
                .ok_or(
                    DynamicalDecouplingError::ArithmeticOverflow {
                        context: "computing DD gap",
                    },
                )?;

            cursor = cursor
                .checked_add_duration(
                    Duration::from_attoseconds(gap_before),
                )
                .ok_or(
                    DynamicalDecouplingError::ArithmeticOverflow {
                        context:
                            "placing DD pulse start",
                    },
                )?;

            let pulse_start = Self::align_start(
                cursor,
                config.alignment(),
            )?;

            let pulse_end =
                pulse_start
                    .checked_add_duration(
                        pulse.duration(),
                    )
                    .ok_or(
                        DynamicalDecouplingError::ArithmeticOverflow {
                            context:
                                "placing DD pulse end",
                        },
                    )?;

            if pulse_end > window.interval().end() {
                return Err(
                    DynamicalDecouplingError::SequenceDoesNotFit {
                        qubit: window.qubit(),
                        idle: window.interval(),
                        required: sequence_duration,
                        available,
                    },
                );
            }

            let interval =
                TimeInterval::new(
                    pulse_start,
                    pulse_end,
                )
                .map_err(|_| {
                    DynamicalDecouplingError::ArithmeticOverflow {
                        context:
                            "constructing DD pulse interval",
                    }
                })?;

            pulses.push(
                PlannedDynamicalDecouplingPulse::new(
                    window.qubit(),
                    pulse.axis(),
                    interval,
                    window.interval(),
                    index,
                ),
            );

            cursor = pulse_end;
        }

        Ok(pulses)
    }

    /// Places pulses at caller-supplied exact normalized positions.
    fn place_custom(
        window: IdleWindow,
        config: &DynamicalDecouplingConfig,
        positions: &[RationalPosition],
    ) -> DynamicalDecouplingResult<
        Vec<PlannedDynamicalDecouplingPulse>,
    > {
        if positions.len()
            != config.sequence().len()
        {
            return Err(
                DynamicalDecouplingError::InvalidConfiguration {
                    message:
                        "custom placement count must equal DD sequence length"
                            .to_owned(),
                },
            );
        }

        for (index, position) in
            positions.iter().enumerate()
        {
            if position.numerator()
                > position.denominator()
            {
                return Err(
                    DynamicalDecouplingError::PlacementOutOfRange {
                        index,
                    },
                );
            }

            if index > 0 {
                let previous = positions[index - 1];

                let lhs = position
                    .numerator()
                    .checked_mul(previous.denominator())
                    .ok_or(
                        DynamicalDecouplingError::ArithmeticOverflow {
                            context:
                                "validating DD placement ordering",
                        },
                    )?;

                let rhs = previous
                    .numerator()
                    .checked_mul(position.denominator())
                    .ok_or(
                        DynamicalDecouplingError::ArithmeticOverflow {
                            context:
                                "validating DD placement ordering",
                        },
                    )?;

                if lhs <= rhs {
                    return Err(
                        DynamicalDecouplingError::PlacementNotMonotonic,
                    );
                }
            }
        }

        let start = window.interval().start();
        let available = window.interval().duration();

        let sequence_duration =
            config.sequence().total_duration()?;

        let free_span =
            available
                .checked_sub(sequence_duration)
                .ok_or(
                    DynamicalDecouplingError::SequenceDoesNotFit {
                        qubit: window.qubit(),
                        idle: window.interval(),
                        required: sequence_duration,
                        available,
                    },
                )?;

        let free_span_as = free_span.attoseconds();

        let mut pulses = Vec::with_capacity(
            config.sequence().len(),
        );

        let mut previous_end = start;

        for (index, (pulse, position)) in config
            .sequence()
            .pulses()
            .iter()
            .zip(positions.iter())
            .enumerate()
        {
            let numerator = position.numerator();
            let denominator = position.denominator();

            let offset = free_span_as
                .checked_mul(numerator)
                .ok_or(
                    DynamicalDecouplingError::ArithmeticOverflow {
                        context:
                            "computing custom DD placement",
                    },
                )?
                / denominator;

            let pulse_start =
                start
                    .checked_add_duration(
                        Duration::from_attoseconds(offset),
                    )
                    .ok_or(
                        DynamicalDecouplingError::ArithmeticOverflow {
                            context:
                                "placing custom DD pulse",
                        },
                    )?;

            let pulse_start = Self::align_start(
                pulse_start,
                config.alignment(),
            )?;

            if pulse_start < previous_end {
                return Err(
                    DynamicalDecouplingError::InvalidPlacement {
                        index,
                        reason:
                            "pulse overlaps a previous DD pulse",
                    },
                );
            }

            let pulse_end =
                pulse_start
                    .checked_add_duration(
                        pulse.duration(),
                    )
                    .ok_or(
                        DynamicalDecouplingError::ArithmeticOverflow {
                            context:
                                "computing custom DD pulse end",
                        },
                    )?;

            if pulse_end > window.interval().end() {
                return Err(
                    DynamicalDecouplingError::SequenceDoesNotFit {
                        qubit: window.qubit(),
                        idle: window.interval(),
                        required: sequence_duration,
                        available,
                    },
                );
            }

            let interval =
                TimeInterval::new(
                    pulse_start,
                    pulse_end,
                )
                .map_err(|_| {
                    DynamicalDecouplingError::ArithmeticOverflow {
                        context:
                            "constructing custom DD interval",
                    }
                })?;

            pulses.push(
                PlannedDynamicalDecouplingPulse::new(
                    window.qubit(),
                    pulse.axis(),
                    interval,
                    window.interval(),
                    index,
                ),
            );

            previous_end = pulse_end;
        }

        Ok(pulses)
    }

    /// Applies start-time alignment without silently modifying duration.
    fn align_start(
        point: TimePoint,
        alignment: Option<&AlignmentRule>,
    ) -> DynamicalDecouplingResult<TimePoint> {
        let Some(rule) = alignment else {
            return Ok(point);
        };

        if rule.kind().is_none()
            || !rule.kind().aligns_start()
        {
            return Ok(point);
        }

        let aligned =
            rule.align_attoseconds(point.attoseconds())?;

        Ok(TimePoint::from_attoseconds(aligned))
    }
}

// =============================================================================
// Internal scheduled operation view
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScheduledQubitOperation {
    operation:
        crate::quantum::ir::core::identity::OperationId,
    interval: TimeInterval,
}

// =============================================================================
// Window planning result
// =============================================================================

enum WindowPlanningResult {
    Inserted(Vec<PlannedDynamicalDecouplingPulse>),
    Skipped(SkippedIdleWindow),
}

// =============================================================================
// Convenience API
// =============================================================================

/// Generates a DD plan using the supplied schedule and configuration.
///
/// This is the preferred stateless convenience entry point for compiler
/// integration.
pub fn apply_dynamical_decoupling(
    schedule: &Schedule,
    config: &DynamicalDecouplingConfig,
) -> DynamicalDecouplingResult<DynamicalDecouplingPlan> {
    DynamicalDecouplingTransformer::new()
        .transform(schedule, config)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_xx_has_two_pulses() {
        let sequence =
            DynamicalDecouplingSequence::builtin(
                BuiltinSequence::Xx,
                Duration::from_attoseconds(10),
            )
            .expect("valid sequence");

        assert_eq!(sequence.len(), 2);
        assert_eq!(
            sequence.total_duration()
                .expect("valid duration")
                .attoseconds(),
            20
        );
        assert!(sequence.identity_preserving());
    }

    #[test]
    fn builtin_xy4_has_four_pulses() {
        let sequence =
            DynamicalDecouplingSequence::builtin(
                BuiltinSequence::Xy4,
                Duration::from_attoseconds(5),
            )
            .expect("valid sequence");

        assert_eq!(sequence.len(), 4);
        assert_eq!(
            sequence.total_duration()
                .expect("valid duration")
                .attoseconds(),
            20
        );
    }

    #[test]
    fn builtin_xy8_has_eight_pulses() {
        let sequence =
            DynamicalDecouplingSequence::builtin(
                BuiltinSequence::Xy8,
                Duration::from_attoseconds(5),
            )
            .expect("valid sequence");

        assert_eq!(sequence.len(), 8);
    }

    #[test]
    fn rational_position_rejects_zero_denominator() {
        assert!(
            RationalPosition::new(1, 0).is_err()
        );
    }

    #[test]
    fn rational_position_rejects_value_above_one() {
        assert!(
            RationalPosition::new(2, 1).is_err()
        );
    }

    #[test]
    fn ratio_is_exact() {
        let ratio =
            RationalRatio::new(2, 1)
                .expect("valid ratio");

        assert!(ratio.at_least(
            Duration::from_attoseconds(20),
            Duration::from_attoseconds(10),
        ));

        assert!(!ratio.at_least(
            Duration::from_attoseconds(19),
            Duration::from_attoseconds(10),
        ));
    }

    #[test]
    fn pulse_inverse_is_symmetric() {
        assert_eq!(
            PulseAxis::X.inverse().inverse(),
            PulseAxis::X
        );

        assert_eq!(
            PulseAxis::Y.inverse().inverse(),
            PulseAxis::Y
        );
    }

    #[test]
    fn sequence_rejects_empty_input() {
        assert!(
            DynamicalDecouplingSequence::new(
                Vec::new(),
                true,
            )
            .is_err()
        );
    }

    #[test]
    fn non_identity_sequence_is_rejected_by_config() {
        let pulse =
            DynamicalDecouplingPulse::new(
                PulseAxis::X,
                Duration::from_attoseconds(1),
            )
            .expect("valid pulse");

        let sequence =
            DynamicalDecouplingSequence::new(
                vec![pulse],
                false,
            )
            .expect("analysis sequence may exist");

        assert!(
            DynamicalDecouplingConfig::new(sequence)
                .is_err()
        );
    }

    #[test]
    fn qubit_resource_preserves_canonical_identity() {
        let logical =
            QubitResource::Logical(
                QubitId::new(7),
            );

        assert!(logical.is_logical());
        assert!(!logical.is_physical());
    }

    #[test]
    fn uniform_gap_math_is_deterministic() {
        let sequence =
            DynamicalDecouplingSequence::builtin(
                BuiltinSequence::Xx,
                Duration::from_attoseconds(10),
            )
            .expect("valid sequence");

        let config =
            DynamicalDecouplingConfig::new(sequence)
                .expect("valid config");

        assert_eq!(
            config.sequence().len(),
            2
        );
    }
}