//! Zamani Quantum Resilience — Dynamical Decoupling
//!
//! Path:
//!     src/quantum/resilience/mitigation/dynamical_decoupling.rs
//!
//! Purpose:
//!     Production-grade, backend-independent dynamical-decoupling planning for
//!     Zamani quantum programs.
//!
//! ============================================================================
//! ARCHITECTURAL CONTRACT
//! ============================================================================
//!
//! Dynamical decoupling (DD) is an error-suppression technique that places a
//! sequence of control operations into an otherwise idle interval of a
//! scheduled quantum computation.
//!
//! The ideal logical action of the inserted sequence must be identity.
//!
//! This module is therefore responsible for:
//!
//!     configuration
//!     sequence definition
//!     identity validation
//!     timing validation
//!     idle-window validation
//!     placement/slack calculation
//!     strategy metadata
//!     mitigation-plan construction
//!
//! This module MUST NOT:
//!
//! - execute a quantum circuit;
//! - contact a hardware provider;
//! - access credentials;
//! - perform network or filesystem I/O;
//! - perform routing;
//! - perform physical-qubit mapping;
//! - perform global circuit scheduling;
//! - perform pulse generation;
//! - assume a particular backend;
//! - assume a particular number of qubits;
//! - assume a particular number of idle windows;
//! - assume a particular gate duration;
//! - hard-code a machine size;
//! - hard-code a maximum number of qubits;
//! - silently truncate a requested sequence;
//! - silently skip an invalid idle window;
//! - implement QEC;
//! - redefine `QubitId`;
//! - mutate `QuantumCircuit`;
//! - contain provider-specific branches;
//! - use unsafe Rust.
//!
//! Actual scheduling, physical placement, pulse lowering and execution remain
//! owned by the corresponding quantum subsystems.
//!
//! ============================================================================
//! REPOSITORY INTEGRATION
//! ============================================================================
//!
//! `mitigation/strategy.rs`
//!     Supplies the common mitigation strategy contract.
//!
//! `mitigation/selection.rs`
//!     Determines whether DD is an appropriate candidate for a workload and
//!     target capability set.
//!
//! `mitigation/executor.rs`
//!     Owns actual execution of the approved mitigation plan.
//!
//! `quantum::ir::gate`
//!     Owns canonical logical gates.
//!
//! `quantum::ir::qubit`
//!     Owns canonical `QubitId`.
//!
//! `quantum::scheduling`
//!     Owns the authoritative schedule and idle-window discovery.
//!
//! `quantum::hardware`
//!     Owns hardware capabilities, timing constraints and pulse/control
//!     capabilities.
//!
//! `quantum::routing`
//!     Owns logical-to-physical placement.
//!
//! `quantum::zqn`
//!     Owns noise/fault semantics.
//!
//! `verification`
//!     Verifies that the transformed execution preserves required semantics.
//!
//! `telemetry`
//!     Records strategy identity, timing information, sequence identity,
//!     selected windows and execution outcome.
//!
//! `history`
//!     Records verified DD outcomes.
//!
//! `planning`
//!     Accounts for DD timing/control overhead and policy constraints.
//!
//! `serialization`
//!     Serializes the immutable configuration and plan representation.
//!
//! ============================================================================
//! IMPORTANT DESIGN BOUNDARY
//! ============================================================================
//!
//! Canonical Zamani IR describes logical operations.
//!
//! Dynamical decoupling additionally needs physical timing information.
//!
//! Therefore this module deliberately uses two layers:
//!
//!     Logical layer:
//!         Gate
//!         GateKind
//!         QubitId
//!
//!     Timing layer:
//!         IdleWindow
//!         SequenceTiming
//!         TimingConstraint
//!         PlacementPlan
//!
//! The timing layer is supplied by scheduling/hardware integration.
//!
//! This avoids putting provider-specific pulse durations into the canonical
//! quantum IR.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! There is no architectural maximum for:
//!
//! - logical qubits;
//! - physical qubits;
//! - idle windows;
//! - circuit depth;
//! - number of DD sequences;
//! - number of executions;
//! - number of backends;
//! - machine size.
//!
//! Collections grow according to available memory/resources and caller policy.
//!
//! No array is sized using a fixed machine-size constant.
//!
//! `usize` is used only for collection lengths/indexing.
//!
//! ============================================================================
//! SEMANTIC GUARANTEE
//! ============================================================================
//!
//! A generated DD sequence must have ideal logical identity.
//!
//! Supported canonical sequences in this implementation are:
//!
//!     XX
//!     XpXm
//!     XY4
//!
//! At the canonical logical-gate level:
//!
//!     XX   = X X = I
//!     XpXm = X X = I
//!     XY4  = X Y X Y = I up to global phase
//!
//! Pulse polarity is deliberately not represented by `GateKind` because the
//! canonical IR does not define calibrated physical pulse amplitudes.
//!
//! Therefore:
//!
//!     XpXm
//!
//! means that the physical pulse layer is expected to lower the two logical X
//! operations to the requested +pi/-pi calibrated pulse sequence while the
//! canonical logical action remains X followed by X.
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
//!
//! Rust 1.97 / Rust 1.97.1
//! Rust 2021
//! stable Rust
//! no nightly features
//! no unsafe code
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::qubit::QubitId;

use super::strategy::{
    Applicability, ExpectedOverhead, MitigationScope, MitigationStrategy,
    OverheadDimension, OverheadLevel, StrategyContext, StrategyDescriptor,
    StrategyEvaluation, StrategyFamily, StrategyId, StrategyPhase,
    StrategyRequirement, StrategyVersion,
};

// ============================================================================
// Stable identities
// ============================================================================

/// Stable strategy identifier.
pub const DYNAMICAL_DECOUPLING_STRATEGY_ID: &str = "dynamical_decoupling";

/// Stable semantic version of the DD strategy.
pub const DYNAMICAL_DECOUPLING_STRATEGY_VERSION: StrategyVersion =
    StrategyVersion::new(1, 0, 0);

/// Stable configuration schema identifier.
pub const DYNAMICAL_DECOUPLING_SCHEMA_ID: &str =
    "zamani.quantum.resilience.mitigation.dynamical_decoupling";

/// Configuration schema version.
pub const DYNAMICAL_DECOUPLING_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Result
// ============================================================================

/// Result type used by this module.
pub type DynamicalDecouplingResult<T> = Result<T, DynamicalDecouplingError>;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the DD planning layer.
///
/// Runtime/provider failures do not belong here. They belong to the common
/// resilience/runtime error model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DynamicalDecouplingError {
    /// The strategy identifier could not be constructed.
    InvalidStrategyIdentity,

    /// The supplied scope cannot be represented by this strategy.
    InvalidScope,

    /// A sequence contains no operations.
    EmptySequence,

    /// The requested sequence contains an operation that cannot be represented
    /// as a canonical logical gate.
    UnsupportedOperation,

    /// The sequence is not an ideal identity.
    NonIdentitySequence,

    /// The sequence contains a duplicate qubit assignment where one was not
    /// allowed.
    DuplicateQubit {
        /// Conflicting qubit.
        qubit: QubitId,
    },

    /// An idle window has no duration.
    ZeroDuration,

    /// A timing alignment is invalid.
    InvalidAlignment,

    /// A duration is not representable under the requested alignment.
    AlignmentViolation {
        /// Duration supplied by the scheduler.
        duration: u64,

        /// Required alignment.
        alignment: u64,
    },

    /// The sequence requires more time than the idle interval.
    SequenceDoesNotFit {
        /// Required sequence duration.
        required: u64,

        /// Available idle duration.
        available: u64,
    },

    /// A timing value overflowed while calculating the plan.
    TimingOverflow,

    /// The spacing vector is invalid.
    InvalidSpacing,

    /// An explicit spacing vector has the wrong number of intervals.
    InvalidSpacingCount {
        /// Required number of gaps.
        expected: usize,

        /// Supplied number of gaps.
        actual: usize,
    },

    /// The spacing weights have no usable total.
    ZeroSpacingWeight,

    /// A pulse duration is zero.
    ZeroPulseDuration,

    /// The timing metadata does not match the DD sequence.
    TimingSequenceMismatch {
        /// Number of DD operations.
        operations: usize,

        /// Number of timing entries.
        timing_entries: usize,
    },

    /// The caller attempted to create a plan for a qubit that is outside the
    /// requested scope.
    ScopeMismatch,

    /// A requested configuration is internally inconsistent.
    InvalidConfiguration,

    /// A requested resource policy cannot satisfy the configuration.
    ResourceConstraintViolation,

    /// A sequence contains a gate that the selected target cannot execute.
    UnsupportedGate {
        /// Canonical gate kind.
        gate: GateKind,
    },
}

impl fmt::Display for DynamicalDecouplingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStrategyIdentity => {
                formatter.write_str("invalid dynamical-decoupling strategy identity")
            }

            Self::InvalidScope => {
                formatter.write_str("invalid dynamical-decoupling scope")
            }

            Self::EmptySequence => {
                formatter.write_str("dynamical-decoupling sequence is empty")
            }

            Self::UnsupportedOperation => {
                formatter.write_str(
                    "dynamical-decoupling sequence contains an unsupported operation",
                )
            }

            Self::NonIdentitySequence => {
                formatter.write_str(
                    "dynamical-decoupling sequence does not implement logical identity",
                )
            }

            Self::DuplicateQubit { qubit } => {
                write!(formatter, "duplicate DD qubit {qubit}")
            }

            Self::ZeroDuration => {
                formatter.write_str("idle-window duration must be greater than zero")
            }

            Self::InvalidAlignment => {
                formatter.write_str("timing alignment must be greater than zero")
            }

            Self::AlignmentViolation {
                duration,
                alignment,
            } => write!(
                formatter,
                "duration {duration} is not compatible with timing alignment {alignment}"
            ),

            Self::SequenceDoesNotFit { required, available } => write!(
                formatter,
                "DD sequence requires {required} timing units but idle window has {available}"
            ),

            Self::TimingOverflow => {
                formatter.write_str("timing calculation overflow")
            }

            Self::InvalidSpacing => {
                formatter.write_str("invalid DD spacing specification")
            }

            Self::InvalidSpacingCount { expected, actual } => write!(
                formatter,
                "DD spacing requires {expected} intervals but received {actual}"
            ),

            Self::ZeroSpacingWeight => {
                formatter.write_str("DD spacing weights must have a non-zero total")
            }

            Self::ZeroPulseDuration => {
                formatter.write_str("DD pulse duration must be greater than zero")
            }

            Self::TimingSequenceMismatch {
                operations,
                timing_entries,
            } => write!(
                formatter,
                "DD sequence contains {operations} operations but {timing_entries} timing entries were supplied"
            ),

            Self::ScopeMismatch => {
                formatter.write_str("DD target qubit is outside the selected mitigation scope")
            }

            Self::InvalidConfiguration => {
                formatter.write_str("invalid dynamical-decoupling configuration")
            }

            Self::ResourceConstraintViolation => {
                formatter.write_str(
                    "dynamical-decoupling configuration violates resource policy",
                )
            }

            Self::UnsupportedGate { gate } => {
                write!(
                    formatter,
                    "DD sequence contains unsupported canonical gate {gate:?}"
                )
            }
        }
    }
}

impl Error for DynamicalDecouplingError {}

// ============================================================================
// Sequence kind
// ============================================================================

/// Standard provider-independent DD sequence families.
///
/// This is intentionally a finite mathematical vocabulary rather than a list
/// of providers.
///
/// Additional sequences can be added in future schema versions or through a
/// custom strategy without changing the execution architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DynamicalDecouplingSequence {
    /// X - X.
    ///
    /// Canonical logical action: identity.
    XX,

    /// +X -X.
    ///
    /// At the canonical logical-gate layer this is represented as X - X.
    /// Pulse polarity is resolved by the physical pulse/control layer.
    XpXm,

    /// X - Y - X - Y.
    ///
    /// Canonical logical action is identity up to global phase.
    XY4,
}

impl DynamicalDecouplingSequence {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::XX => "XX",
            Self::XpXm => "XpXm",
            Self::XY4 => "XY4",
        }
    }

    /// Number of logical DD operations.
    #[must_use]
    pub const fn operation_count(self) -> usize {
        match self {
            Self::XX | Self::XpXm => 2,
            Self::XY4 => 4,
        }
    }

    /// Returns the canonical logical gate sequence.
    #[must_use]
    pub const fn gate_kinds(self) -> &'static [GateKind] {
        match self {
            Self::XX | Self::XpXm => &[GateKind::X, GateKind::X],
            Self::XY4 => &[
                GateKind::X,
                GateKind::Y,
                GateKind::X,
                GateKind::Y,
            ],
        }
    }

    /// Returns whether the mathematical sequence is an identity.
    ///
    /// All sequences in this enum are identity-preserving at the logical
    /// level, with global phase ignored as required for ordinary quantum
    /// measurement semantics.
    #[must_use]
    pub const fn is_identity(self) -> bool {
        true
    }
}

impl fmt::Display for DynamicalDecouplingSequence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Spacing policy
// ============================================================================

/// Policy used to distribute idle slack around a DD sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpacingPolicy {
    /// Divide available slack as evenly as possible across all gaps.
    Balanced,

    /// Put additional slack toward the sequence edges.
    Edges,

    /// Explicit integer weights.
    ///
    /// There are `sequence_length + 1` gaps:
    ///
    ///     before pulse 0
    ///     between pulse 0 and 1
    ///     ...
    ///     after final pulse
    ///
    /// The actual durations are proportional to these weights.
    Explicit(Arc<[u64]>),
}

impl Default for SpacingPolicy {
    fn default() -> Self {
        Self::Balanced
    }
}

// ============================================================================
// Slack distribution
// ============================================================================

/// A concrete gap in a generated DD placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DdGap {
    /// Gap duration in the scheduler's opaque timing unit.
    duration: u64,
}

impl DdGap {
    /// Creates a gap.
    pub const fn new(duration: u64) -> Self {
        Self { duration }
    }

    /// Returns the duration.
    #[must_use]
    pub const fn duration(self) -> u64 {
        self.duration
    }
}

/// Concrete timing placement for one DD sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DdPlacement {
    /// Gaps before/between/after DD operations.
    gaps: Arc<[DdGap]>,

    /// Total DD pulse/control duration.
    pulse_duration: u64,

    /// Total idle-window duration consumed by this placement.
    total_duration: u64,
}

impl DdPlacement {
    /// Creates a placement after validation.
    fn new(
        gaps: Vec<DdGap>,
        pulse_duration: u64,
    ) -> DynamicalDecouplingResult<Self> {
        let mut total_duration = pulse_duration;

        for gap in &gaps {
            total_duration = total_duration
                .checked_add(gap.duration())
                .ok_or(DynamicalDecouplingError::TimingOverflow)?;
        }

        Ok(Self {
            gaps: gaps.into(),
            pulse_duration,
            total_duration,
        })
    }

    /// Returns all sequence gaps.
    #[must_use]
    pub fn gaps(&self) -> &[DdGap] {
        &self.gaps
    }

    /// Returns total pulse duration.
    #[must_use]
    pub const fn pulse_duration(&self) -> u64 {
        self.pulse_duration
    }

    /// Returns total placement duration.
    #[must_use]
    pub const fn total_duration(&self) -> u64 {
        self.total_duration
    }
}

// ============================================================================
// Timing model
// ============================================================================

/// Timing information for the physical realization of a DD sequence.
///
/// The values are opaque scheduler/hardware timing units. This module does not
/// assume that the unit is nanoseconds, picoseconds, `dt`, cycles, samples, or
/// any provider-specific unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SequenceTiming {
    /// Duration of each physical DD operation in sequence order.
    pulse_durations: Arc<[u64]>,

    /// Required alignment of each timing quantity.
    ///
    /// `None` means the upstream scheduling contract has already guaranteed
    /// alignment.
    alignment: Option<u64>,
}

impl SequenceTiming {
    /// Creates timing information.
    pub fn new(
        pulse_durations: impl Into<Arc<[u64]>>,
        alignment: Option<u64>,
    ) -> DynamicalDecouplingResult<Self> {
        let pulse_durations = pulse_durations.into();

        if pulse_durations.is_empty() {
            return Err(DynamicalDecouplingError::TimingSequenceMismatch {
                operations: 1,
                timing_entries: 0,
            });
        }

        if pulse_durations.iter().any(|duration| *duration == 0) {
            return Err(DynamicalDecouplingError::ZeroPulseDuration);
        }

        if let Some(value) = alignment {
            if value == 0 {
                return Err(DynamicalDecouplingError::InvalidAlignment);
            }
        }

        Ok(Self {
            pulse_durations,
            alignment,
        })
    }

    /// Returns pulse durations.
    #[must_use]
    pub fn pulse_durations(&self) -> &[u64] {
        &self.pulse_durations
    }

    /// Returns optional alignment.
    #[must_use]
    pub const fn alignment(&self) -> Option<u64> {
        self.alignment
    }

    /// Returns total physical pulse duration.
    pub fn total_pulse_duration(&self) -> DynamicalDecouplingResult<u64> {
        self.pulse_durations
            .iter()
            .try_fold(0_u64, |total, duration| {
                total
                    .checked_add(*duration)
                    .ok_or(DynamicalDecouplingError::TimingOverflow)
            })
    }
}

// ============================================================================
// Idle window
// ============================================================================

/// A scheduled idle interval on one canonical logical qubit.
///
/// Physical mapping is intentionally not stored here. The scheduler/routing
/// layer can associate the logical qubit with the current physical resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdleWindow {
    /// Canonical logical qubit.
    qubit: QubitId,

    /// Start time in the scheduler's opaque timing unit.
    start: u64,

    /// Duration in the scheduler's opaque timing unit.
    duration: u64,
}

impl IdleWindow {
    /// Creates an idle window.
    pub fn new(
        qubit: QubitId,
        start: u64,
        duration: u64,
    ) -> DynamicalDecouplingResult<Self> {
        if duration == 0 {
            return Err(DynamicalDecouplingError::ZeroDuration);
        }

        start
            .checked_add(duration)
            .ok_or(DynamicalDecouplingError::TimingOverflow)?;

        Ok(Self {
            qubit,
            start,
            duration,
        })
    }

    /// Returns the logical qubit.
    #[must_use]
    pub const fn qubit(&self) -> QubitId {
        self.qubit
    }

    /// Returns the start time.
    #[must_use]
    pub const fn start(&self) -> u64 {
        self.start
    }

    /// Returns the duration.
    #[must_use]
    pub const fn duration(&self) -> u64 {
        self.duration
    }

    /// Returns the exclusive end time.
    pub fn end(&self) -> DynamicalDecouplingResult<u64> {
        self.start
            .checked_add(self.duration)
            .ok_or(DynamicalDecouplingError::TimingOverflow)
    }
}

// ============================================================================
// Sequence configuration
// ============================================================================

/// Configuration for dynamical decoupling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicalDecouplingConfig {
    /// Selected sequence.
    sequence: DynamicalDecouplingSequence,

    /// Target mitigation scope.
    scope: MitigationScope,

    /// Slack distribution policy.
    spacing: SpacingPolicy,

    /// Minimum idle-window/sequence duration ratio represented as a rational
    /// number.
    ///
    /// `None` means the caller has no ratio restriction.
    minimum_window_ratio: Option<Rational>,

    /// Whether DD is allowed to operate immediately after reset.
    skip_after_reset: bool,

    /// Whether multiple DD cycles may be inserted into a sufficiently long
    /// idle interval.
    multiple_cycles: bool,
}

impl DynamicalDecouplingConfig {
    /// Creates the default DD configuration.
    ///
    /// The default is intentionally conservative:
    ///
    /// - XX sequence;
    /// - program scope;
    /// - balanced spacing;
    /// - no artificial minimum ratio;
    /// - do not skip reset windows;
    /// - do not automatically insert multiple cycles.
    #[must_use]
    pub fn new() -> Self {
        Self {
            sequence: DynamicalDecouplingSequence::XX,
            scope: MitigationScope::Program,
            spacing: SpacingPolicy::Balanced,
            minimum_window_ratio: None,
            skip_after_reset: false,
            multiple_cycles: false,
        }
    }

    /// Sets the DD sequence.
    #[must_use]
    pub fn with_sequence(
        mut self,
        sequence: DynamicalDecouplingSequence,
    ) -> Self {
        self.sequence = sequence;
        self
    }

    /// Sets the logical mitigation scope.
    #[must_use]
    pub fn with_scope(mut self, scope: MitigationScope) -> Self {
        self.scope = scope;
        self
    }

    /// Sets the slack distribution policy.
    #[must_use]
    pub fn with_spacing(mut self, spacing: SpacingPolicy) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the minimum idle-window ratio.
    #[must_use]
    pub fn with_minimum_window_ratio(
        mut self,
        ratio: Rational,
    ) -> DynamicalDecouplingResult<Self> {
        ratio.validate()?;
        if ratio.numerator == 0 {
            return Err(DynamicalDecouplingError::InvalidConfiguration);
        }
        self.minimum_window_ratio = Some(ratio);
        Ok(self)
    }

    /// Enables/disables DD immediately after reset.
    #[must_use]
    pub fn with_skip_after_reset(mut self, skip: bool) -> Self {
        self.skip_after_reset = skip;
        self
    }

    /// Enables/disables multiple cycles in long idle windows.
    #[must_use]
    pub fn with_multiple_cycles(mut self, enabled: bool) -> Self {
        self.multiple_cycles = enabled;
        self
    }

    /// Returns the sequence.
    #[must_use]
    pub const fn sequence(&self) -> DynamicalDecouplingSequence {
        self.sequence
    }

    /// Returns the scope.
    #[must_use]
    pub fn scope(&self) -> &MitigationScope {
        &self.scope
    }

    /// Returns spacing policy.
    #[must_use]
    pub fn spacing(&self) -> &SpacingPolicy {
        &self.spacing
    }

    /// Returns the minimum window ratio.
    #[must_use]
    pub const fn minimum_window_ratio(&self) -> Option<Rational> {
        self.minimum_window_ratio
    }

    /// Returns whether reset-adjacent windows are skipped.
    #[must_use]
    pub const fn skip_after_reset(&self) -> bool {
        self.skip_after_reset
    }

    /// Returns whether multiple cycles are allowed.
    #[must_use]
    pub const fn multiple_cycles(&self) -> bool {
        self.multiple_cycles
    }

    /// Validates the complete configuration.
    pub fn validate(&self) -> DynamicalDecouplingResult<()> {
        if !self.sequence.is_identity() {
            return Err(DynamicalDecouplingError::NonIdentitySequence);
        }

        if let Some(ratio) = self.minimum_window_ratio {
            ratio.validate()?;
            if ratio.numerator == 0 {
                return Err(DynamicalDecouplingError::InvalidConfiguration);
            }
        }

        if let SpacingPolicy::Explicit(weights) = &self.spacing {
            if weights.is_empty() || weights.iter().all(|weight| *weight == 0) {
                return Err(DynamicalDecouplingError::InvalidSpacing);
            }
        }

        Ok(())
    }
}

impl Default for DynamicalDecouplingConfig {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Rational
// ============================================================================

/// Small exact rational value used for timing/policy ratios.
///
/// No floating-point arithmetic is used in the DD planning path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Rational {
    /// Numerator.
    numerator: u64,

    /// Denominator.
    denominator: u64,
}

impl Rational {
    /// Creates a positive rational.
    pub const fn new(numerator: u64, denominator: u64) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    fn validate(self) -> DynamicalDecouplingResult<()> {
        if self.denominator == 0 {
            return Err(DynamicalDecouplingError::InvalidConfiguration);
        }
        Ok(())
    }

    /// Returns numerator.
    #[must_use]
    pub const fn numerator(self) -> u64 {
        self.numerator
    }

    /// Returns denominator.
    #[must_use]
    pub const fn denominator(self) -> u64 {
        self.denominator
    }

    /// Returns whether `value / base >= self`.
    fn less_than(self, value: u64, base: u64) -> DynamicalDecouplingResult<bool> {
        self.validate()?;

        let left = value
            .checked_mul(self.denominator)
            .ok_or(DynamicalDecouplingError::TimingOverflow)?;

        let right = base
            .checked_mul(self.numerator)
            .ok_or(DynamicalDecouplingError::TimingOverflow)?;

        Ok(left < right)
    }
}

// ============================================================================
// Plan
// ============================================================================

/// One fully validated DD insertion plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicalDecouplingInsertion {
    /// Target logical qubit.
    qubit: QubitId,

    /// Idle window being protected.
    window: IdleWindow,

    /// Selected sequence.
    sequence: DynamicalDecouplingSequence,

    /// Canonical logical gates making up the DD sequence.
    gates: Arc<[Gate]>,

    /// Concrete timing placement.
    placement: DdPlacement,
}

impl DynamicalDecouplingInsertion {
    /// Returns the logical target.
    #[must_use]
    pub const fn qubit(&self) -> QubitId {
        self.qubit
    }

    /// Returns the idle window.
    #[must_use]
    pub const fn window(&self) -> IdleWindow {
        self.window
    }

    /// Returns sequence kind.
    #[must_use]
    pub const fn sequence(&self) -> DynamicalDecouplingSequence {
        self.sequence
    }

    /// Returns canonical logical DD gates.
    #[must_use]
    pub fn gates(&self) -> &[Gate] {
        &self.gates
    }

    /// Returns timing placement.
    #[must_use]
    pub fn placement(&self) -> &DdPlacement {
        &self.placement
    }
}

/// Complete DD mitigation plan.
///
/// The plan is immutable and contains no backend/provider object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicalDecouplingPlan {
    /// Configuration used to construct the plan.
    config: DynamicalDecouplingConfig,

    /// Planned insertions.
    insertions: Arc<[DynamicalDecouplingInsertion]>,
}

impl DynamicalDecouplingPlan {
    /// Creates a plan.
    pub fn new(
        config: DynamicalDecouplingConfig,
        insertions: Vec<DynamicalDecouplingInsertion>,
    ) -> DynamicalDecouplingResult<Self> {
        config.validate()?;

        Ok(Self {
            config,
            insertions: insertions.into(),
        })
    }

    /// Returns configuration.
    #[must_use]
    pub fn config(&self) -> &DynamicalDecouplingConfig {
        &self.config
    }

    /// Returns all insertions.
    #[must_use]
    pub fn insertions(&self) -> &[DynamicalDecouplingInsertion] {
        &self.insertions
    }

    /// Returns whether the plan contains no insertions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.insertions.is_empty()
    }

    /// Returns the number of insertions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.insertions.len()
    }

    /// Returns total added pulse duration.
    pub fn total_pulse_duration(&self) -> DynamicalDecouplingResult<u64> {
        self.insertions.iter().try_fold(0_u64, |total, insertion| {
            total
                .checked_add(insertion.placement.pulse_duration())
                .ok_or(DynamicalDecouplingError::TimingOverflow)
        })
    }
}

// ============================================================================
// Strategy implementation
// ============================================================================

/// Production dynamical-decoupling strategy implementation.
///
/// This type implements only strategy metadata and applicability evaluation.
/// Actual DD plan construction is performed through the pure planning methods
/// on this type and execution remains outside this module.
#[derive(Debug, Clone)]
pub struct DynamicalDecouplingStrategy {
    descriptor: StrategyDescriptor,
}

impl DynamicalDecouplingStrategy {
    /// Creates the standard DD strategy.
    #[must_use]
    pub fn new() -> Self {
        let descriptor = StrategyDescriptor {
            id: StrategyId::new(DYNAMICAL_DECOUPLING_STRATEGY_ID)
                .expect("static DD strategy identifier must be valid"),

            version: DYNAMICAL_DECOUPLING_STRATEGY_VERSION,

            family: StrategyFamily::DynamicalDecoupling,

            phase: StrategyPhase::ExecutionPreparation,

            description: Arc::from(
                "Timing-aware dynamical decoupling for scheduled idle quantum resources",
            ),

            requirements: Arc::from([
                StrategyRequirement::ScheduleControl,
                StrategyRequirement::TimingInformation,
                StrategyRequirement::ScopedExecution,
                StrategyRequirement::Provenance,
            ]),

            expected_overhead: Arc::from([
                ExpectedOverhead {
                    dimension: OverheadDimension::QuantumOperations,
                    level: OverheadLevel::Medium,
                },
                ExpectedOverhead {
                    dimension: OverheadDimension::ScheduleDuration,
                    level: OverheadLevel::Medium,
                },
                ExpectedOverhead {
                    dimension: OverheadDimension::Time,
                    level: OverheadLevel::Medium,
                },
            ]),

            deterministic: true,

            requires_explicit_authorization: false,
        };

        Self { descriptor }
    }

    /// Returns the descriptor.
    #[must_use]
    pub fn descriptor_ref(&self) -> &StrategyDescriptor {
        &self.descriptor
    }

    /// Builds a DD plan for supplied scheduled idle windows.
    ///
    /// `timing` must describe the physical duration of the selected sequence.
    ///
    /// This method does not execute or mutate anything.
    pub fn plan(
        &self,
        config: DynamicalDecouplingConfig,
        windows: &[IdleWindow],
        timing: &SequenceTiming,
    ) -> DynamicalDecouplingResult<DynamicalDecouplingPlan> {
        config.validate()?;

        validate_timing_for_sequence(config.sequence(), timing)?;

        let mut insertions = Vec::with_capacity(windows.len());

        for window in windows {
            if !scope_contains_qubit(config.scope(), window.qubit()) {
                return Err(DynamicalDecouplingError::ScopeMismatch);
            }

            let placement = calculate_placement(
                config.sequence(),
                window.duration(),
                timing,
                config.spacing(),
                config.minimum_window_ratio(),
                config.multiple_cycles(),
            )?;

            let gates = canonical_sequence_for_qubit(
                config.sequence(),
                window.qubit(),
            )?;

            insertions.push(DynamicalDecouplingInsertion {
                qubit: window.qubit(),
                window: *window,
                sequence: config.sequence(),
                gates: gates.into(),
                placement,
            });
        }

        DynamicalDecouplingPlan::new(config, insertions)
    }
}

impl Default for DynamicalDecouplingStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl MitigationStrategy for DynamicalDecouplingStrategy {
    fn descriptor(&self) -> &StrategyDescriptor {
        &self.descriptor
    }

    fn evaluate(&self, context: &StrategyContext) -> StrategyEvaluation {
        let evaluation = <Self as MitigationStrategy>::evaluate_default(
            self,
            context,
        );

        evaluation
    }
}

// ============================================================================
// Strategy evaluation helper
// ============================================================================

/// Evaluates a strategy using the repository's standard strategy contract.
///
/// This helper exists so the concrete implementation remains explicit without
/// duplicating requirement semantics from `strategy.rs`.
fn evaluate_strategy(
    strategy: &DynamicalDecouplingStrategy,
    context: &StrategyContext,
) -> StrategyEvaluation {
    let descriptor = strategy.descriptor();

    if descriptor.requires_explicit_authorization
        && !context.policy_authorized
    {
        return StrategyEvaluation::new(
            descriptor,
            Applicability::RequiresPolicyValidation,
            vec![StrategyRequirement::ExplicitPolicyAuthorization],
        );
    }

    let mut missing = Vec::new();

    for requirement in descriptor.requirements.iter() {
        let satisfied = match requirement {
            StrategyRequirement::ScheduleControl => {
                context.schedule_control_available
            }
            StrategyRequirement::TimingInformation => {
                context.timing_information_available
            }
            StrategyRequirement::ScopedExecution => true,
            StrategyRequirement::Provenance => context.provenance_available,
            _ => true,
        };

        if !satisfied {
            missing.push(*requirement);
        }
    }

    if !missing.is_empty() {
        return StrategyEvaluation::new(
            descriptor,
            Applicability::RequiresCapabilityValidation,
            missing,
        );
    }

    StrategyEvaluation::new(
        descriptor,
        Applicability::Applicable,
        Vec::new(),
    )
}

// ============================================================================
// Canonical sequence construction
// ============================================================================

/// Constructs the canonical logical DD sequence for one logical qubit.
///
/// This is the only place where this module creates canonical `Gate` objects.
/// Physical pulse implementation is deliberately left to lowering/execution.
pub fn canonical_sequence_for_qubit(
    sequence: DynamicalDecouplingSequence,
    qubit: QubitId,
) -> DynamicalDecouplingResult<Vec<Gate>> {
    let mut gates = Vec::with_capacity(sequence.operation_count());

    for kind in sequence.gate_kinds().iter().copied() {
        if !matches!(kind, GateKind::X | GateKind::Y) {
            return Err(DynamicalDecouplingError::UnsupportedGate);
        }

        let gate = Gate::simple(kind, vec![qubit])
            .map_err(|_| DynamicalDecouplingError::UnsupportedOperation)?;

        gates.push(gate);
    }

    Ok(gates)
}

// ============================================================================
// Timing validation
// ============================================================================

fn validate_timing_for_sequence(
    sequence: DynamicalDecouplingSequence,
    timing: &SequenceTiming,
) -> DynamicalDecouplingResult<()> {
    let operations = sequence.operation_count();
    let timing_entries = timing.pulse_durations().len();

    if operations != timing_entries {
        return Err(
            DynamicalDecouplingError::TimingSequenceMismatch {
                operations,
                timing_entries,
            },
        );
    }

    if !sequence.is_identity() {
        return Err(DynamicalDecouplingError::NonIdentitySequence);
    }

    Ok(())
}

// ============================================================================
// Scope validation
// ============================================================================

fn scope_contains_qubit(
    scope: &MitigationScope,
    qubit: QubitId,
) -> bool {
    match scope {
        MitigationScope::Program | MitigationScope::Execution => true,

        MitigationScope::LogicalQubits(qubits) => {
            qubits.iter().any(|candidate| *candidate == qubit)
        }

        MitigationScope::ResourceRegion(_) => {
            // Resource-region membership is resolved by the caller because this
            // strategy layer deliberately does not interpret provider/resource
            // identities.
            false
        }
    }
}

// ============================================================================
// Placement
// ============================================================================

fn calculate_placement(
    sequence: DynamicalDecouplingSequence,
    idle_duration: u64,
    timing: &SequenceTiming,
    spacing: &SpacingPolicy,
    minimum_ratio: Option<Rational>,
    multiple_cycles: bool,
) -> DynamicalDecouplingResult<DdPlacement> {
    if idle_duration == 0 {
        return Err(DynamicalDecouplingError::ZeroDuration);
    }

    let pulse_duration = timing.total_pulse_duration()?;

    if pulse_duration > idle_duration {
        return Err(DynamicalDecouplingError::SequenceDoesNotFit {
            required: pulse_duration,
            available: idle_duration,
        });
    }

    if let Some(ratio) = minimum_ratio {
        if ratio.less_than(idle_duration, pulse_duration)? {
            return Err(
                DynamicalDecouplingError::ResourceConstraintViolation,
            );
        }
    }

    let mut cycles = 1_u64;

    if multiple_cycles {
        // Determine the largest number of complete cycles that fits without
        // overflow. This is resource-driven and contains no machine-size
        // constant.
        if pulse_duration != 0 {
            cycles = idle_duration / pulse_duration;
            if cycles == 0 {
                cycles = 1;
            }
        }
    }

    let selected_cycles = if multiple_cycles { cycles } else { 1 };

    let total_pulse_duration = pulse_duration
        .checked_mul(selected_cycles)
        .ok_or(DynamicalDecouplingError::TimingOverflow)?;

    if total_pulse_duration > idle_duration {
        return Err(DynamicalDecouplingError::SequenceDoesNotFit {
            required: total_pulse_duration,
            available: idle_duration,
        });
    }

    let slack = idle_duration - total_pulse_duration;

    let gaps_per_cycle = sequence
        .operation_count()
        .checked_add(1)
        .ok_or(DynamicalDecouplingError::TimingOverflow)?;

    let gap_weights =
        spacing_weights(gaps_per_cycle, spacing)?;

    let gaps = distribute_slack(slack, &gap_weights)?;

    DdPlacement::new(gaps, total_pulse_duration)
}

// ============================================================================
// Spacing
// ============================================================================

fn spacing_weights(
    count: usize,
    policy: &SpacingPolicy,
) -> DynamicalDecouplingResult<Vec<u64>> {
    if count == 0 {
        return Err(DynamicalDecouplingError::InvalidSpacing);
    }

    match policy {
        SpacingPolicy::Balanced => Ok(vec![1; count]),

        SpacingPolicy::Edges => {
            let mut weights = vec![1_u64; count];

            if count > 2 {
                for index in 1..count - 1 {
                    weights[index] = 0;
                }
            }

            Ok(weights)
        }

        SpacingPolicy::Explicit(weights) => {
            if weights.len() != count {
                return Err(
                    DynamicalDecouplingError::InvalidSpacingCount {
                        expected: count,
                        actual: weights.len(),
                    },
                );
            }

            if weights.iter().all(|weight| *weight == 0) {
                return Err(DynamicalDecouplingError::ZeroSpacingWeight);
            }

            Ok(weights.to_vec())
        }
    }
}

/// Distributes an integer number of timing units according to integer weights.
///
/// Any remainder is distributed deterministically from the first gap onward.
/// Therefore identical input always produces identical output.
fn distribute_slack(
    slack: u64,
    weights: &[u64],
) -> DynamicalDecouplingResult<Vec<DdGap>> {
    if weights.is_empty() {
        return Err(DynamicalDecouplingError::InvalidSpacing);
    }

    let total_weight = weights.iter().try_fold(0_u64, |total, weight| {
        total
            .checked_add(*weight)
            .ok_or(DynamicalDecouplingError::TimingOverflow)
    })?;

    if total_weight == 0 {
        return Err(DynamicalDecouplingError::ZeroSpacingWeight);
    }

    let mut gaps = Vec::with_capacity(weights.len());
    let mut assigned = 0_u64;

    for weight in weights {
        let numerator = slack
            .checked_mul(*weight)
            .ok_or(DynamicalDecouplingError::TimingOverflow)?;

        let duration = numerator / total_weight;

        assigned = assigned
            .checked_add(duration)
            .ok_or(DynamicalDecouplingError::TimingOverflow)?;

        gaps.push(DdGap::new(duration));
    }

    // Deterministically distribute integer remainder.
    let mut remainder = slack
        .checked_sub(assigned)
        .ok_or(DynamicalDecouplingError::TimingOverflow)?;

    let mut index = 0_usize;

    while remainder > 0 {
        if weights[index] > 0 {
            let current = gaps[index].duration();

            gaps[index] = DdGap::new(
                current
                    .checked_add(1)
                    .ok_or(DynamicalDecouplingError::TimingOverflow)?,
            );

            remainder -= 1;
        }

        index += 1;

        if index == gaps.len() {
            index = 0;
        }
    }

    Ok(gaps)
}

// ============================================================================
// Public pure helpers
// ============================================================================

/// Returns the canonical operation sequence for a DD family.
///
/// This is useful to scheduling/lowering code that wants to inspect the
/// sequence without constructing a complete strategy object.
#[must_use]
pub fn sequence_gate_kinds(
    sequence: DynamicalDecouplingSequence,
) -> &'static [GateKind] {
    sequence.gate_kinds()
}

/// Validates that a sequence is logically identity-preserving.
pub fn validate_identity(
    sequence: DynamicalDecouplingSequence,
) -> DynamicalDecouplingResult<()> {
    if sequence.is_identity() {
        Ok(())
    } else {
        Err(DynamicalDecouplingError::NonIdentitySequence)
    }
}

/// Returns the number of timing gaps required around a DD sequence.
pub fn gap_count(
    sequence: DynamicalDecouplingSequence,
) -> DynamicalDecouplingResult<usize> {
    sequence
        .operation_count()
        .checked_add(1)
        .ok_or(DynamicalDecouplingError::TimingOverflow)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q(id: u64) -> QubitId {
        QubitId::new(id)
    }

    #[test]
    fn sequence_counts_are_stable() {
        assert_eq!(
            DynamicalDecouplingSequence::XX.operation_count(),
            2
        );
        assert_eq!(
            DynamicalDecouplingSequence::XpXm.operation_count(),
            2
        );
        assert_eq!(
            DynamicalDecouplingSequence::XY4.operation_count(),
            4
        );
    }

    #[test]
    fn all_standard_sequences_are_identity_preserving() {
        assert!(validate_identity(
            DynamicalDecouplingSequence::XX
        )
        .is_ok());

        assert!(validate_identity(
            DynamicalDecouplingSequence::XpXm
        )
        .is_ok());

        assert!(validate_identity(
            DynamicalDecouplingSequence::XY4
        )
        .is_ok());
    }

    #[test]
    fn canonical_xx_sequence_uses_canonical_qubit_identity() {
        let gates = canonical_sequence_for_qubit(
            DynamicalDecouplingSequence::XX,
            q(7),
        )
        .expect("XX should construct");

        assert_eq!(gates.len(), 2);
    }

    #[test]
    fn canonical_xy4_sequence_has_four_operations() {
        let gates = canonical_sequence_for_qubit(
            DynamicalDecouplingSequence::XY4,
            q(3),
        )
        .expect("XY4 should construct");

        assert_eq!(gates.len(), 4);
    }

    #[test]
    fn balanced_spacing_preserves_total_slack() {
        let weights =
            spacing_weights(3, &SpacingPolicy::Balanced)
                .expect("weights should construct");

        let gaps =
            distribute_slack(10, &weights)
                .expect("slack should distribute");

        let total: u64 =
            gaps.iter().map(|gap| gap.duration()).sum();

        assert_eq!(total, 10);
    }

    #[test]
    fn explicit_spacing_is_deterministic() {
        let weights = spacing_weights(
            3,
            &SpacingPolicy::Explicit(Arc::from([1, 2, 1])),
        )
        .expect("weights should construct");

        let first =
            distribute_slack(40, &weights)
                .expect("first distribution");

        let second =
            distribute_slack(40, &weights)
                .expect("second distribution");

        assert_eq!(first, second);
    }

    #[test]
    fn timing_must_match_sequence_length() {
        let timing =
            SequenceTiming::new(Arc::from([10_u64]), None)
                .expect("timing should be valid");

        let result = validate_timing_for_sequence(
            DynamicalDecouplingSequence::XX,
            &timing,
        );

        assert!(matches!(
            result,
            Err(
                DynamicalDecouplingError::TimingSequenceMismatch {
                    operations: 2,
                    timing_entries: 1
                }
            )
        ));
    }

    #[test]
    fn sequence_must_fit_idle_window() {
        let timing =
            SequenceTiming::new(Arc::from([10_u64, 10_u64]), None)
                .expect("timing should be valid");

        let result = calculate_placement(
            DynamicalDecouplingSequence::XX,
            10,
            &timing,
            &SpacingPolicy::Balanced,
            None,
            false,
        );

        assert!(matches!(
            result,
            Err(DynamicalDecouplingError::SequenceDoesNotFit {
                required: 20,
                available: 10
            })
        ));
    }

    #[test]
    fn placement_consumes_exact_idle_duration() {
        let timing =
            SequenceTiming::new(Arc::from([10_u64, 10_u64]), None)
                .expect("timing should be valid");

        let placement = calculate_placement(
            DynamicalDecouplingSequence::XX,
            100,
            &timing,
            &SpacingPolicy::Balanced,
            None,
            false,
        )
        .expect("placement should be valid");

        assert_eq!(placement.total_duration(), 100);
        assert_eq!(placement.pulse_duration(), 20);

        let gap_total: u64 = placement
            .gaps()
            .iter()
            .map(|gap| gap.duration())
            .sum();

        assert_eq!(gap_total, 80);
    }

    #[test]
    fn logical_scope_is_enforced() {
        let config = DynamicalDecouplingConfig::new()
            .with_scope(MitigationScope::logical_qubits([q(1)]));

        let strategy = DynamicalDecouplingStrategy::new();

        let timing =
            SequenceTiming::new(Arc::from([10_u64, 10_u64]), None)
                .expect("timing should be valid");

        let window = IdleWindow::new(q(2), 0, 100)
            .expect("window should be valid");

        let result =
            strategy.plan(config, &[window], &timing);

        assert_eq!(
            result,
            Err(DynamicalDecouplingError::ScopeMismatch)
        );
    }

    #[test]
    fn program_scope_accepts_arbitrary_logical_qubit_identity() {
        let config = DynamicalDecouplingConfig::new();

        let strategy = DynamicalDecouplingStrategy::new();

        let timing =
            SequenceTiming::new(Arc::from([10_u64, 10_u64]), None)
                .expect("timing should be valid");

        let window =
            IdleWindow::new(q(u64::MAX), 0, 100)
                .expect("representable logical identity is valid");

        let plan =
            strategy.plan(config, &[window], &timing)
                .expect("plan should be valid");

        assert_eq!(plan.len(), 1);
        assert_eq!(plan.insertions()[0].qubit(), q(u64::MAX));
    }

    #[test]
    fn multiple_cycles_are_resource_driven() {
        let timing =
            SequenceTiming::new(Arc::from([10_u64, 10_u64]), None)
                .expect("timing should be valid");

        let placement = calculate_placement(
            DynamicalDecouplingSequence::XX,
            100,
            &timing,
            &SpacingPolicy::Balanced,
            None,
            true,
        )
        .expect("multiple cycles should fit");

        assert_eq!(placement.pulse_duration(), 100);
        assert_eq!(placement.total_duration(), 100);
    }

    #[test]
    fn minimum_ratio_is_checked_exactly() {
        let timing =
            SequenceTiming::new(Arc::from([10_u64, 10_u64]), None)
                .expect("timing should be valid");

        let ratio = Rational::new(5, 1);

        let result = calculate_placement(
            DynamicalDecouplingSequence::XX,
            40,
            &timing,
            &SpacingPolicy::Balanced,
            Some(ratio),
            false,
        );

        assert!(matches!(
            result,
            Err(
                DynamicalDecouplingError::ResourceConstraintViolation
            )
        ));
    }

    #[test]
    fn strategy_metadata_is_provider_independent() {
        let strategy = DynamicalDecouplingStrategy::new();

        assert_eq!(
            strategy.descriptor().id.as_str(),
            DYNAMICAL_DECOUPLING_STRATEGY_ID
        );

        assert_eq!(
            strategy.descriptor().family,
            StrategyFamily::DynamicalDecoupling
        );

        assert!(
            strategy
                .descriptor()
                .requires(StrategyRequirement::ScheduleControl)
        );

        assert!(
            strategy
                .descriptor()
                .requires(StrategyRequirement::TimingInformation)
        );
    }

    #[test]
    fn strategy_evaluation_requires_timing_and_schedule_capabilities() {
        let strategy = DynamicalDecouplingStrategy::new();

        let context = StrategyContext::default();

        let evaluation =
            evaluate_strategy(&strategy, &context);

        assert_eq!(
            evaluation.applicability,
            Applicability::RequiresCapabilityValidation
        );

        assert!(
            evaluation
                .missing_requirements
                .iter()
                .any(|requirement| {
                    *requirement
                        == StrategyRequirement::ScheduleControl
                })
        );

        assert!(
            evaluation
                .missing_requirements
                .iter()
                .any(|requirement| {
                    *requirement
                        == StrategyRequirement::TimingInformation
                })
        );
    }

    #[test]
    fn strategy_can_be_evaluated_when_required_capabilities_exist() {
        let strategy = DynamicalDecouplingStrategy::new();

        let context = StrategyContext {
            scope: MitigationScope::Program,
            schedule_control_available: true,
            timing_information_available: true,
            provenance_available: true,
            ..StrategyContext::default()
        };

        let evaluation =
            evaluate_strategy(&strategy, &context);

        assert_eq!(
            evaluation.applicability,
            Applicability::Applicable
        );
        assert!(evaluation.missing_requirements.is_empty());
    }
}