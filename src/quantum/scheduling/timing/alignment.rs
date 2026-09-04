//! Hardware-aware timing alignment for the Zamani quantum scheduler.
//!
//! # Responsibility
//!
//! This module defines *alignment constraints and operations*.
//!
//! Alignment answers:
//!
//! > "At which legal points on the target timing grid may an operation
//! > start or finish?"
//!
//! It does not answer:
//!
//! - which operation runs next;
//! - which physical qubit an operation uses;
//! - which hardware provider is being used;
//! - how a quantum operation is synthesized;
//! - how logical qubits are routed;
//! - how QEC is performed;
//! - how a QPU is contacted.
//!
//! Those responsibilities belong to other subsystems.
//!
//! # Architectural boundary
//!
//! ```text
//! quantum::ir
//!      |
//!      v
//! routing
//!      |
//!      v
//! scheduling
//!      |
//!      +---- timing::resolution
//!      |
//!      +---- timing::alignment  <--- this module
//!      |
//!      +---- resources
//!      |
//!      +---- constraints
//!      |
//!      v
//! hardware adapter
//! ```
//!
//! Hardware supplies timing requirements through the hardware adapter.
//! The scheduler consumes those requirements without knowing whether the
//! target is superconducting, trapped-ion, neutral-atom, photonic, spin,
//! annealing, modular, distributed, simulated, or a future technology.
//!
//! # Exact arithmetic
//!
//! This module intentionally does not use floating-point arithmetic.
//!
//! Alignment is expressed using exact integer arithmetic over the canonical
//! scheduler time representation. The underlying `TimingResolution` owns the
//! representation of the target timing grid.
//!
//! # Scalability
//!
//! No qubit count, operation count, channel count, topology size, schedule
//! depth, or machine size is encoded here.
//!
//! The implementation uses O(1) storage for an individual alignment rule.
//! Applying a rule to a collection of operations is the responsibility of
//! the scheduler/planner and should be performed incrementally rather than
//! by constructing a timeline proportional to machine size.
//!
//! # Important distinction
//!
//! Alignment is not the same thing as scheduling.
//!
//! For example:
//!
//! ```text
//! dependency says:
//!     operation B cannot start before operation A finishes
//!
//! resource model says:
//!     channel C is unavailable until T
//!
//! alignment says:
//!     B may start only on legal timing boundaries
//!
//! scheduler says:
//!     choose the earliest/latest/optimal legal time satisfying all three
//! ```
//!
//! # Hardware examples
//!
//! A target may independently require:
//!
//! - pulse/gate start alignment;
//! - acquisition/measurement start alignment;
//! - control-channel alignment;
//! - frame alignment;
//! - operation-duration granularity;
//! - target-specific alignment;
//! - no alignment at all.
//!
//! These are represented as data rather than hard-coded assumptions.
//!
//! # Rust
//!
//! Designed for Rust 1.97 / 1.97.1.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.

use super::resolution::TimingResolution;

/// Result type used by alignment operations.
pub type AlignmentResult<T> = Result<T, AlignmentError>;

/// The semantic kind of alignment being requested.
///
/// The scheduler may use different alignment domains for different
/// operations. For example, a gate may use pulse alignment while a
/// measurement/acquisition may use acquire alignment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum AlignmentKind {
    /// No alignment restriction.
    None,

    /// Alignment of an operation's start time.
    Start,

    /// Alignment of an operation's end time.
    End,

    /// Alignment of an operation's duration.
    Duration,

    /// Both start and end must satisfy the alignment rule.
    StartAndEnd,
}

impl AlignmentKind {
    /// Returns `true` when this kind imposes no alignment.
    #[must_use]
    pub const fn is_none(self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns `true` when the start time must be aligned.
    #[must_use]
    pub const fn aligns_start(self) -> bool {
        matches!(self, Self::Start | Self::StartAndEnd)
    }

    /// Returns `true` when the end time must be aligned.
    #[must_use]
    pub const fn aligns_end(self) -> bool {
        matches!(self, Self::End | Self::StartAndEnd)
    }

    /// Returns `true` when the duration must be aligned.
    #[must_use]
    pub const fn aligns_duration(self) -> bool {
        matches!(self, Self::Duration)
    }
}

/// Policy used when a requested time is not already aligned.
///
/// Alignment itself must not silently change a schedule. The caller chooses
/// whether a shift is permitted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum AlignmentMode {
    /// Reject an unaligned value.
    Strict,

    /// Round toward the nearest legal point.
    ///
    /// Ties are resolved toward the later legal point. This provides
    /// deterministic behavior.
    Nearest,

    /// Move to the first legal point at or after the requested value.
    Ceil,

    /// Move to the last legal point at or before the requested value.
    Floor,
}

impl Default for AlignmentMode {
    fn default() -> Self {
        Self::Strict
    }
}

/// Semantic domain to which an alignment rule applies.
///
/// This is deliberately broader than "gate" versus "measurement".
/// Future hardware can introduce new operation classes without requiring
/// changes to the underlying timing representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum AlignmentDomain {
    /// General quantum operation timing.
    Operation,

    /// Control/pulse emission timing.
    Control,

    /// Measurement/acquisition timing.
    Acquisition,

    /// Readout timing.
    Readout,

    /// Classical feedback timing.
    Feedback,

    /// Communication timing between distributed quantum resources.
    Communication,

    /// Synchronization/barrier timing.
    Synchronization,

    /// Target-defined alignment domain.
    Custom(u64),
}

impl AlignmentDomain {
    /// Returns whether this is a target-defined custom domain.
    #[must_use]
    pub const fn is_custom(self) -> bool {
        matches!(self, Self::Custom(_))
    }
}

/// Immutable description of one alignment rule.
///
/// An alignment rule does not contain a list of operations. It describes a
/// reusable target constraint that can be applied to arbitrarily many
/// operations.
///
/// # Examples
///
/// A target might expose:
///
/// ```text
/// pulse alignment    = 16 dt
/// acquire alignment  = 64 dt
/// ```
///
/// Those become separate `AlignmentRule` values.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct AlignmentRule {
    domain: AlignmentDomain,
    kind: AlignmentKind,
    resolution: TimingResolution,
    mode: AlignmentMode,
}

impl AlignmentRule {
    /// Creates a new alignment rule.
    ///
    /// `TimingResolution::Continuous` is valid and means that no discrete
    /// grid is imposed by this rule. The rule can still carry semantic
    /// information about what is being aligned.
    #[must_use]
    pub const fn new(
        domain: AlignmentDomain,
        kind: AlignmentKind,
        resolution: TimingResolution,
        mode: AlignmentMode,
    ) -> Self {
        Self {
            domain,
            kind,
            resolution,
            mode,
        }
    }

    /// Creates a rule imposing no alignment.
    #[must_use]
    pub const fn none(domain: AlignmentDomain) -> Self {
        Self {
            domain,
            kind: AlignmentKind::None,
            resolution: TimingResolution::Continuous,
            mode: AlignmentMode::Strict,
        }
    }

    /// Returns the semantic domain.
    #[must_use]
    pub const fn domain(self) -> AlignmentDomain {
        self.domain
    }

    /// Returns the alignment kind.
    #[must_use]
    pub const fn kind(self) -> AlignmentKind {
        self.kind
    }

    /// Returns the target timing resolution.
    #[must_use]
    pub const fn resolution(self) -> TimingResolution {
        self.resolution
    }

    /// Returns the selected adjustment mode.
    #[must_use]
    pub const fn mode(self) -> AlignmentMode {
        self.mode
    }

    /// Returns whether this rule imposes no restriction.
    #[must_use]
    pub const fn is_none(self) -> bool {
        self.kind.is_none()
    }

    /// Checks whether a time represented in canonical attoseconds satisfies
    /// this alignment rule.
    ///
    /// The conversion to/from the scheduler's public `TimePoint` is kept
    /// outside this module so that this module does not duplicate the
    /// canonical time representation.
    pub fn is_aligned_attoseconds(&self, attoseconds: u128) -> AlignmentResult<bool> {
        if self.is_none() {
            return Ok(true);
        }

        self.resolution
            .is_aligned_attoseconds(attoseconds)
            .map_err(AlignmentError::from)
    }

    /// Aligns a canonical attosecond time according to this rule.
    ///
    /// For `Strict`, an already-aligned value is returned unchanged.
    ///
    /// For `Ceil`, `Floor`, and `Nearest`, the returned value is the legal
    /// target-grid point selected by the requested mode.
    pub fn align_attoseconds(&self, attoseconds: u128) -> AlignmentResult<u128> {
        if self.is_none() {
            return Ok(attoseconds);
        }

        match self.mode {
            AlignmentMode::Strict => {
                if self.is_aligned_attoseconds(attoseconds)? {
                    Ok(attoseconds)
                } else {
                    Err(AlignmentError::UnalignedTime {
                        attoseconds,
                        domain: self.domain,
                        kind: self.kind,
                    })
                }
            }
            AlignmentMode::Ceil => self
                .resolution
                .ceil_attoseconds(attoseconds)
                .map_err(AlignmentError::from),
            AlignmentMode::Floor => self
                .resolution
                .floor_attoseconds(attoseconds)
                .map_err(AlignmentError::from),
            AlignmentMode::Nearest => {
                let floor = self
                    .resolution
                    .floor_attoseconds(attoseconds)
                    .map_err(AlignmentError::from)?;

                let ceil = self
                    .resolution
                    .ceil_attoseconds(attoseconds)
                    .map_err(AlignmentError::from)?;

                if floor == ceil {
                    return Ok(floor);
                }

                let lower_distance = attoseconds
                    .checked_sub(floor)
                    .ok_or(AlignmentError::ArithmeticOverflow)?;

                let upper_distance = ceil
                    .checked_sub(attoseconds)
                    .ok_or(AlignmentError::ArithmeticOverflow)?;

                // Ties deliberately resolve upward so that the result is
                // deterministic and never moves an operation earlier when
                // both choices are equally distant.
                if upper_distance <= lower_distance {
                    Ok(ceil)
                } else {
                    Ok(floor)
                }
            }
        }
    }

    /// Returns the amount by which a time must be shifted when aligned using
    /// ceil semantics.
    ///
    /// This is useful to the scheduler because the scheduler can account for
    /// the shift as idle time instead of silently changing operation timing.
    pub fn ceil_shift_attoseconds(&self, attoseconds: u128) -> AlignmentResult<u128> {
        let aligned = self
            .resolution
            .ceil_attoseconds(attoseconds)
            .map_err(AlignmentError::from)?;

        aligned
            .checked_sub(attoseconds)
            .ok_or(AlignmentError::ArithmeticOverflow)
    }

    /// Returns the amount by which a time would move when aligned using floor
    /// semantics.
    pub fn floor_shift_attoseconds(&self, attoseconds: u128) -> AlignmentResult<u128> {
        let aligned = self
            .resolution
            .floor_attoseconds(attoseconds)
            .map_err(AlignmentError::from)?;

        attoseconds
            .checked_sub(aligned)
            .ok_or(AlignmentError::ArithmeticOverflow)
    }
}

/// A complete set of alignment rules supplied by a target.
///
/// The scheduler should obtain this structure from the hardware adapter
/// rather than constructing machine-specific values internally.
///
/// A target may omit any domain by using `None`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlignmentProfile {
    operation: Option<AlignmentRule>,
    control: Option<AlignmentRule>,
    acquisition: Option<AlignmentRule>,
    readout: Option<AlignmentRule>,
    feedback: Option<AlignmentRule>,
    communication: Option<AlignmentRule>,
    synchronization: Option<AlignmentRule>,
}

impl Default for AlignmentProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl AlignmentProfile {
    /// Creates an empty alignment profile.
    ///
    /// An empty profile means that the target has supplied no discrete
    /// alignment restrictions. It does not impose arbitrary defaults.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operation: None,
            control: None,
            acquisition: None,
            readout: None,
            feedback: None,
            communication: None,
            synchronization: None,
        }
    }

    /// Sets the general operation alignment rule.
    #[must_use]
    pub const fn with_operation(mut self, rule: AlignmentRule) -> Self {
        self.operation = Some(rule);
        self
    }

    /// Sets the control/pulse alignment rule.
    #[must_use]
    pub const fn with_control(mut self, rule: AlignmentRule) -> Self {
        self.control = Some(rule);
        self
    }

    /// Sets the acquisition alignment rule.
    #[must_use]
    pub const fn with_acquisition(mut self, rule: AlignmentRule) -> Self {
        self.acquisition = Some(rule);
        self
    }

    /// Sets the readout alignment rule.
    #[must_use]
    pub const fn with_readout(mut self, rule: AlignmentRule) -> Self {
        self.readout = Some(rule);
        self
    }

    /// Sets the classical-feedback alignment rule.
    #[must_use]
    pub const fn with_feedback(mut self, rule: AlignmentRule) -> Self {
        self.feedback = Some(rule);
        self
    }

    /// Sets the distributed-communication alignment rule.
    #[must_use]
    pub const fn with_communication(mut self, rule: AlignmentRule) -> Self {
        self.communication = Some(rule);
        self
    }

    /// Sets the synchronization alignment rule.
    #[must_use]
    pub const fn with_synchronization(mut self, rule: AlignmentRule) -> Self {
        self.synchronization = Some(rule);
        self
    }

    /// Returns the rule for a domain.
    #[must_use]
    pub const fn rule(&self, domain: AlignmentDomain) -> Option<&AlignmentRule> {
        match domain {
            AlignmentDomain::Operation => self.operation.as_ref(),
            AlignmentDomain::Control => self.control.as_ref(),
            AlignmentDomain::Acquisition => self.acquisition.as_ref(),
            AlignmentDomain::Readout => self.readout.as_ref(),
            AlignmentDomain::Feedback => self.feedback.as_ref(),
            AlignmentDomain::Communication => self.communication.as_ref(),
            AlignmentDomain::Synchronization => self.synchronization.as_ref(),
            AlignmentDomain::Custom(_) => None,
        }
    }

    /// Returns the operation alignment rule.
    #[must_use]
    pub const fn operation(&self) -> Option<&AlignmentRule> {
        self.operation.as_ref()
    }

    /// Returns the control alignment rule.
    #[must_use]
    pub const fn control(&self) -> Option<&AlignmentRule> {
        self.control.as_ref()
    }

    /// Returns the acquisition alignment rule.
    #[must_use]
    pub const fn acquisition(&self) -> Option<&AlignmentRule> {
        self.acquisition.as_ref()
    }

    /// Returns the readout alignment rule.
    #[must_use]
    pub const fn readout(&self) -> Option<&AlignmentRule> {
        self.readout.as_ref()
    }

    /// Returns the feedback alignment rule.
    #[must_use]
    pub const fn feedback(&self) -> Option<&AlignmentRule> {
        self.feedback.as_ref()
    }

    /// Returns the communication alignment rule.
    #[must_use]
    pub const fn communication(&self) -> Option<&AlignmentRule> {
        self.communication.as_ref()
    }

    /// Returns the synchronization alignment rule.
    #[must_use]
    pub const fn synchronization(&self) -> Option<&AlignmentRule> {
        self.synchronization.as_ref()
    }

    /// Returns `true` when the profile contains no active restrictions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operation.is_none()
            && self.control.is_none()
            && self.acquisition.is_none()
            && self.readout.is_none()
            && self.feedback.is_none()
            && self.communication.is_none()
            && self.synchronization.is_none()
    }
}

/// Alignment requirements for one scheduled operation.
///
/// This structure deliberately contains no quantum operation implementation.
/// The scheduler/planner creates it from the operation's semantics and the
/// target's alignment profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct OperationAlignment {
    /// Optional rule applied to the operation start.
    start: Option<AlignmentRule>,

    /// Optional rule applied to the operation end.
    end: Option<AlignmentRule>,

    /// Optional rule applied to operation duration.
    duration: Option<AlignmentRule>,
}

impl OperationAlignment {
    /// Creates an unrestricted alignment requirement.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            start: None,
            end: None,
            duration: None,
        }
    }

    /// Creates an alignment requirement for operation start.
    #[must_use]
    pub const fn start(rule: AlignmentRule) -> Self {
        Self {
            start: Some(rule),
            end: None,
            duration: None,
        }
    }

    /// Creates an alignment requirement for operation end.
    #[must_use]
    pub const fn end(rule: AlignmentRule) -> Self {
        Self {
            start: None,
            end: Some(rule),
            duration: None,
        }
    }

    /// Creates an alignment requirement for operation duration.
    #[must_use]
    pub const fn duration(rule: AlignmentRule) -> Self {
        Self {
            start: None,
            end: None,
            duration: Some(rule),
        }
    }

    /// Creates a complete alignment requirement.
    #[must_use]
    pub const fn new(
        start: Option<AlignmentRule>,
        end: Option<AlignmentRule>,
        duration: Option<AlignmentRule>,
    ) -> Self {
        Self {
            start,
            end,
            duration,
        }
    }

    /// Returns the start rule.
    #[must_use]
    pub const fn start_rule(&self) -> Option<&AlignmentRule> {
        self.start.as_ref()
    }

    /// Returns the end rule.
    #[must_use]
    pub const fn end_rule(&self) -> Option<&AlignmentRule> {
        self.end.as_ref()
    }

    /// Returns the duration rule.
    #[must_use]
    pub const fn duration_rule(&self) -> Option<&AlignmentRule> {
        self.duration.as_ref()
    }

    /// Returns whether no alignment is required.
    #[must_use]
    pub fn is_unrestricted(&self) -> bool {
        self.start.is_none() && self.end.is_none() && self.duration.is_none()
    }

    /// Validates a complete interval represented using canonical attoseconds.
    ///
    /// The method checks:
    ///
    /// - `start <= end`;
    /// - start alignment;
    /// - end alignment;
    /// - duration alignment.
    ///
    /// It does not check dependency or resource constraints. Those belong to
    /// the corresponding scheduler verification modules.
    pub fn validate_interval(
        &self,
        start_attoseconds: u128,
        end_attoseconds: u128,
    ) -> AlignmentResult<()> {
        if end_attoseconds < start_attoseconds {
            return Err(AlignmentError::InvalidInterval {
                start_attoseconds,
                end_attoseconds,
            });
        }

        let duration = end_attoseconds
            .checked_sub(start_attoseconds)
            .ok_or(AlignmentError::ArithmeticOverflow)?;

        if let Some(rule) = self.start {
            if !rule.is_aligned_attoseconds(start_attoseconds)? {
                return Err(AlignmentError::UnalignedStart {
                    attoseconds: start_attoseconds,
                    domain: rule.domain(),
                });
            }
        }

        if let Some(rule) = self.end {
            if !rule.is_aligned_attoseconds(end_attoseconds)? {
                return Err(AlignmentError::UnalignedEnd {
                    attoseconds: end_attoseconds,
                    domain: rule.domain(),
                });
            }
        }

        if let Some(rule) = self.duration {
            if !rule.is_aligned_attoseconds(duration)? {
                return Err(AlignmentError::UnalignedDuration {
                    attoseconds: duration,
                    domain: rule.domain(),
                });
            }
        }

        Ok(())
    }
}

/// Result of aligning an operation interval.
///
/// This structure makes shifts explicit. The scheduler can therefore account
/// for alignment-induced idle time and explain why an operation moved.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct AlignedInterval {
    original_start_attoseconds: u128,
    original_end_attoseconds: u128,
    aligned_start_attoseconds: u128,
    aligned_end_attoseconds: u128,
}

impl AlignedInterval {
    /// Constructs an alignment result.
    ///
    /// The constructor is crate-visible because the scheduler should normally
    /// create these values through [`align_interval`].
    pub(crate) const fn new(
        original_start_attoseconds: u128,
        original_end_attoseconds: u128,
        aligned_start_attoseconds: u128,
        aligned_end_attoseconds: u128,
    ) -> Self {
        Self {
            original_start_attoseconds,
            original_end_attoseconds,
            aligned_start_attoseconds,
            aligned_end_attoseconds,
        }
    }

    /// Original interval start.
    #[must_use]
    pub const fn original_start_attoseconds(self) -> u128 {
        self.original_start_attoseconds
    }

    /// Original interval end.
    #[must_use]
    pub const fn original_end_attoseconds(self) -> u128 {
        self.original_end_attoseconds
    }

    /// Aligned interval start.
    #[must_use]
    pub const fn aligned_start_attoseconds(self) -> u128 {
        self.aligned_start_attoseconds
    }

    /// Aligned interval end.
    #[must_use]
    pub const fn aligned_end_attoseconds(self) -> u128 {
        self.aligned_end_attoseconds
    }

    /// Amount by which the start moved later.
    #[must_use]
    pub fn start_shift_attoseconds(self) -> Option<u128> {
        self.aligned_start_attoseconds
            .checked_sub(self.original_start_attoseconds)
    }

    /// Amount by which the end moved later.
    #[must_use]
    pub fn end_shift_attoseconds(self) -> Option<u128> {
        self.aligned_end_attoseconds
            .checked_sub(self.original_end_attoseconds)
    }

    /// Returns the aligned duration.
    #[must_use]
    pub fn duration_attoseconds(self) -> Option<u128> {
        self.aligned_end_attoseconds
            .checked_sub(self.aligned_start_attoseconds)
    }

    /// Returns whether alignment changed the interval.
    #[must_use]
    pub const fn changed(self) -> bool {
        self.original_start_attoseconds != self.aligned_start_attoseconds
            || self.original_end_attoseconds != self.aligned_end_attoseconds
    }
}

/// Aligns an interval while preserving its original duration whenever
/// possible.
///
/// This is the preferred operation for scheduler rescheduling because
/// shifting an operation is normally safer than changing its duration.
///
/// The operation:
///
/// 1. validates the input interval;
/// 2. aligns the start according to the start rule;
/// 3. derives the end from the original duration;
/// 4. aligns the end if required;
/// 5. rejects cases where the requested constraints cannot be satisfied
///    without changing duration.
///
/// A transformation pass may intentionally change duration, but that should
/// be implemented in `transformations/`, not here.
pub fn align_interval(
    requirements: &OperationAlignment,
    start_attoseconds: u128,
    end_attoseconds: u128,
) -> AlignmentResult<AlignedInterval> {
    if end_attoseconds < start_attoseconds {
        return Err(AlignmentError::InvalidInterval {
            start_attoseconds,
            end_attoseconds,
        });
    }

    let original_duration = end_attoseconds
        .checked_sub(start_attoseconds)
        .ok_or(AlignmentError::ArithmeticOverflow)?;

    let aligned_start = match requirements.start {
        Some(rule) => rule.align_attoseconds(start_attoseconds)?,
        None => start_attoseconds,
    };

    let mut aligned_end = aligned_start
        .checked_add(original_duration)
        .ok_or(AlignmentError::ArithmeticOverflow)?;

    if let Some(rule) = requirements.end {
        let candidate_end = rule.align_attoseconds(aligned_end)?;

        if candidate_end != aligned_end {
            // We intentionally do not silently change operation duration.
            //
            // If an enclosing scheduler wants to preserve an aligned end
            // instead, it must explicitly move the operation or use a
            // transformation that owns duration changes.
            return Err(AlignmentError::DurationWouldChange {
                original_duration_attoseconds: original_duration,
                candidate_start_attoseconds: aligned_start,
                requested_end_attoseconds: candidate_end,
            });
        }

        aligned_end = candidate_end;
    }

    if let Some(rule) = requirements.duration {
        if !rule.is_aligned_attoseconds(original_duration)? {
            return Err(AlignmentError::UnalignedDuration {
                attoseconds: original_duration,
                domain: rule.domain(),
            });
        }
    }

    Ok(AlignedInterval::new(
        start_attoseconds,
        end_attoseconds,
        aligned_start,
        aligned_end,
    ))
}

/// Error returned by alignment operations.
///
/// Alignment errors are structured so callers can make programmatic
/// decisions without parsing human-readable messages.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AlignmentError {
    /// The requested interval is invalid.
    InvalidInterval {
        /// Interval start.
        start_attoseconds: u128,

        /// Interval end.
        end_attoseconds: u128,
    },

    /// A strict alignment rule rejected a time.
    UnalignedTime {
        /// Requested time.
        attoseconds: u128,

        /// Alignment domain.
        domain: AlignmentDomain,

        /// Alignment kind.
        kind: AlignmentKind,
    },

    /// Operation start violates alignment.
    UnalignedStart {
        /// Requested start.
        attoseconds: u128,

        /// Alignment domain.
        domain: AlignmentDomain,
    },

    /// Operation end violates alignment.
    UnalignedEnd {
        /// Requested end.
        attoseconds: u128,

        /// Alignment domain.
        domain: AlignmentDomain,
    },

    /// Operation duration violates alignment.
    UnalignedDuration {
        /// Requested duration.
        attoseconds: u128,

        /// Alignment domain.
        domain: AlignmentDomain,
    },

    /// Aligning the end would implicitly alter the operation duration.
    DurationWouldChange {
        /// Original duration.
        original_duration_attoseconds: u128,

        /// Start after alignment.
        candidate_start_attoseconds: u128,

        /// End required by the alignment rule.
        requested_end_attoseconds: u128,
    },

    /// Exact arithmetic exceeded the representable range.
    ArithmeticOverflow,

    /// The underlying timing resolution rejected the operation.
    InvalidResolution {
        /// Human-readable reason supplied by the resolution layer.
        reason: String,
    },
}

impl core::fmt::Display for AlignmentError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidInterval {
                start_attoseconds,
                end_attoseconds,
            } => write!(
                formatter,
                "invalid timing interval: start {start_attoseconds} exceeds end {end_attoseconds}"
            ),

            Self::UnalignedTime {
                attoseconds,
                domain,
                kind,
            } => write!(
                formatter,
                "time {attoseconds} is not aligned for domain {domain:?} and kind {kind:?}"
            ),

            Self::UnalignedStart {
                attoseconds,
                domain,
            } => write!(
                formatter,
                "operation start {attoseconds} is not aligned for domain {domain:?}"
            ),

            Self::UnalignedEnd {
                attoseconds,
                domain,
            } => write!(
                formatter,
                "operation end {attoseconds} is not aligned for domain {domain:?}"
            ),

            Self::UnalignedDuration {
                attoseconds,
                domain,
            } => write!(
                formatter,
                "operation duration {attoseconds} is not aligned for domain {domain:?}"
            ),

            Self::DurationWouldChange {
                original_duration_attoseconds,
                candidate_start_attoseconds,
                requested_end_attoseconds,
            } => write!(
                formatter,
                "alignment would change duration {original_duration_attoseconds} \
                 from start {candidate_start_attoseconds} to end {requested_end_attoseconds}"
            ),

            Self::ArithmeticOverflow => {
                write!(formatter, "timing alignment arithmetic overflow")
            }

            Self::InvalidResolution { reason } => {
                write!(formatter, "invalid timing resolution: {reason}")
            }
        }
    }
}

impl std::error::Error for AlignmentError {}

impl From<super::resolution::ResolutionError> for AlignmentError {
    fn from(error: super::resolution::ResolutionError) -> Self {
        match error {
            super::resolution::ResolutionError::ArithmeticOverflow => {
                Self::ArithmeticOverflow
            }

            other => Self::InvalidResolution {
                reason: other.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resolution(value: u128) -> TimingResolution {
        TimingResolution::from_attoseconds(value)
            .expect("test resolution must be valid")
    }

    #[test]
    fn unrestricted_alignment_accepts_any_time() {
        let rule = AlignmentRule::none(AlignmentDomain::Operation);

        assert!(rule.is_aligned_attoseconds(0).expect("valid"));
        assert!(rule.is_aligned_attoseconds(u128::MAX).expect("valid"));
    }

    #[test]
    fn strict_alignment_accepts_exact_grid_point() {
        let rule = AlignmentRule::new(
            AlignmentDomain::Control,
            AlignmentKind::Start,
            resolution(10),
            AlignmentMode::Strict,
        );

        assert!(rule.is_aligned_attoseconds(0).expect("valid"));
        assert!(rule.is_aligned_attoseconds(10).expect("valid"));
        assert!(rule.is_aligned_attoseconds(100).expect("valid"));
    }

    #[test]
    fn strict_alignment_rejects_non_grid_point() {
        let rule = AlignmentRule::new(
            AlignmentDomain::Control,
            AlignmentKind::Start,
            resolution(10),
            AlignmentMode::Strict,
        );

        assert!(matches!(
            rule.align_attoseconds(17),
            Err(AlignmentError::UnalignedTime { .. })
        ));
    }

    #[test]
    fn ceil_alignment_moves_forward() {
        let rule = AlignmentRule::new(
            AlignmentDomain::Control,
            AlignmentKind::Start,
            resolution(10),
            AlignmentMode::Ceil,
        );

        assert_eq!(
            rule.align_attoseconds(17).expect("valid"),
            20
        );
    }

    #[test]
    fn floor_alignment_moves_backward() {
        let rule = AlignmentRule::new(
            AlignmentDomain::Control,
            AlignmentKind::Start,
            resolution(10),
            AlignmentMode::Floor,
        );

        assert_eq!(
            rule.align_attoseconds(17).expect("valid"),
            10
        );
    }

    #[test]
    fn nearest_alignment_is_deterministic() {
        let rule = AlignmentRule::new(
            AlignmentDomain::Control,
            AlignmentKind::Start,
            resolution(10),
            AlignmentMode::Nearest,
        );

        assert_eq!(
            rule.align_attoseconds(14).expect("valid"),
            10
        );

        // Tie resolves upward.
        assert_eq!(
            rule.align_attoseconds(15).expect("valid"),
            20
        );

        assert_eq!(
            rule.align_attoseconds(16).expect("valid"),
            20
        );
    }

    #[test]
    fn ceil_shift_is_explicit() {
        let rule = AlignmentRule::new(
            AlignmentDomain::Acquisition,
            AlignmentKind::Start,
            resolution(64),
            AlignmentMode::Ceil,
        );

        assert_eq!(
            rule.ceil_shift_attoseconds(65).expect("valid"),
            63
        );
    }

    #[test]
    fn profile_is_empty_initially() {
        let profile = AlignmentProfile::new();

        assert!(profile.is_empty());
        assert!(profile.operation().is_none());
        assert!(profile.acquisition().is_none());
    }

    #[test]
    fn profile_stores_independent_domains() {
        let profile = AlignmentProfile::new()
            .with_control(AlignmentRule::new(
                AlignmentDomain::Control,
                AlignmentKind::Start,
                resolution(16),
                AlignmentMode::Strict,
            ))
            .with_acquisition(AlignmentRule::new(
                AlignmentDomain::Acquisition,
                AlignmentKind::Start,
                resolution(64),
                AlignmentMode::Strict,
            ));

        assert!(profile.control().is_some());
        assert!(profile.acquisition().is_some());
        assert!(profile.operation().is_none());
    }

    #[test]
    fn interval_alignment_preserves_duration() {
        let requirement = OperationAlignment::start(AlignmentRule::new(
            AlignmentDomain::Control,
            AlignmentKind::Start,
            resolution(10),
            AlignmentMode::Ceil,
        ));

        let result = align_interval(&requirement, 17, 27)
            .expect("alignment should succeed");

        assert_eq!(result.original_start_attoseconds(), 17);
        assert_eq!(result.original_end_attoseconds(), 27);
        assert_eq!(result.aligned_start_attoseconds(), 20);
        assert_eq!(result.aligned_end_attoseconds(), 30);
        assert_eq!(result.duration_attoseconds(), Some(10));
    }

    #[test]
    fn interval_alignment_rejects_invalid_interval() {
        let requirement = OperationAlignment::none();

        assert!(matches!(
            align_interval(&requirement, 20, 10),
            Err(AlignmentError::InvalidInterval { .. })
        ));
    }

    #[test]
    fn interval_alignment_does_not_silently_change_duration() {
        let requirement = OperationAlignment::new(
            Some(AlignmentRule::new(
                AlignmentDomain::Control,
                AlignmentKind::Start,
                resolution(10),
                AlignmentMode::Ceil,
            )),
            Some(AlignmentRule::new(
                AlignmentDomain::Acquisition,
                AlignmentKind::End,
                resolution(16),
                AlignmentMode::Strict,
            )),
            None,
        );

        let result = align_interval(&requirement, 17, 27);

        assert!(matches!(
            result,
            Err(AlignmentError::DurationWouldChange { .. })
                | Err(AlignmentError::UnalignedEnd { .. })
        ));
    }

    #[test]
    fn duration_alignment_is_checked_without_changing_duration() {
        let requirement = OperationAlignment::duration(AlignmentRule::new(
            AlignmentDomain::Control,
            AlignmentKind::Duration,
            resolution(10),
            AlignmentMode::Strict,
        ));

        assert!(align_interval(&requirement, 0, 20).is_ok());
        assert!(matches!(
            align_interval(&requirement, 0, 21),
            Err(AlignmentError::UnalignedDuration { .. })
        ));
    }
}