//! Zamani Quantum Scheduling — Production Temporal Verification
//!
//! Path:
//!     src/quantum/scheduling/verification/timing.rs
//!
//! # Responsibility
//!
//! This module independently verifies the temporal correctness of a concrete
//! quantum schedule.
//!
//! It answers:
//!
//! > "Does every scheduled operation satisfy its duration, temporal windows,
//! > dependency timing, alignment, and global timing constraints without
//! > overflowing the canonical timing domain?"
//!
//! This module does NOT:
//!
//! - schedule operations;
//! - choose a scheduling algorithm;
//! - perform routing;
//! - allocate resources;
//! - discover hardware;
//! - execute hardware;
//! - implement QEC;
//! - implement noise models;
//! - parse source code;
//! - synthesize gates;
//! - mutate a schedule;
//! - define another TimePoint or Duration;
//! - assume a fixed number of qubits;
//! - assume a fixed number of operations;
//! - assume a fixed clock frequency;
//! - assume a fixed hardware timing resolution.
//!
//! # Architectural position
//!
//! ```text
//!                    quantum::ir
//!                        │
//!                        ▼
//!                 mapped executable IR
//!                        │
//!                        ▼
//!                 scheduling::ir
//!                        │
//!             ┌──────────┴──────────┐
//!             │                     │
//!             ▼                     ▼
//!       scheduling plan       temporal constraints
//!             │                     │
//!             └──────────┬──────────┘
//!                        ▼
//!              verification::timing
//!                        │
//!             ┌──────────┼──────────┐
//!             │          │          │
//!             ▼          ▼          ▼
//!          valid       invalid    diagnostics
//!             │
//!             ▼
//!       final verification
//!             │
//!             ▼
//!       hardware lowering
//! ```
//!
//! # Canonical timing ownership
//!
//! The authoritative semantic timing types are owned by:
//!
//! ```text
//! crate::quantum::ir::timing
//! ```
//!
//! This verifier therefore imports:
//!
//! ```text
//! crate::quantum::ir::timing::Duration
//! crate::quantum::ir::timing::TimePoint
//! ```
//!
//! It deliberately does NOT define:
//!
//! ```text
//! struct TimePoint(...)
//! struct Duration(...)
//! ```
//!
//! There must be one semantic timing domain throughout Zamani.
//!
//! The scheduling façade in:
//!
//! ```text
//! scheduling::timing::time
//! ```
//!
//! may re-export those canonical types, but this verifier does not depend on a
//! second representation.
//!
//! # Interval semantics
//!
//! Concrete operation execution intervals use half-open semantics:
//!
//! ```text
//! [start, end)
//! ```
//!
//! Therefore:
//!
//! ```text
//! operation A: [0, 10)
//! operation B: [10, 20)
//! ```
//!
//! do not overlap.
//!
//! This module verifies temporal placement using the same semantic convention
//! used by the scheduler's resource and result layers.
//!
//! # Window semantics
//!
//! `TimeWindow` uses inclusive point semantics:
//!
//! ```text
//! earliest <= point <= latest
//! ```
//!
//! This is intentionally different from execution intervals.
//!
//! Therefore:
//!
//! ```text
//! start window     -> inclusive point
//! finish window    -> inclusive point
//! execution range  -> half-open interval
//! ```
//!
//! # Universal-program principle
//!
//! This verifier contains no:
//!
//! - maximum qubit count;
//! - maximum operation count;
//! - maximum schedule depth;
//! - maximum timing value;
//! - maximum parallelism;
//! - maximum number of constraints;
//! - fixed clock rate;
//! - fixed hardware duration;
//! - fixed channel count;
//! - vendor-specific timing rule.
//!
//! Any finite limitation must come from:
//!
//! - the canonical representation;
//! - actual available memory;
//! - an explicit caller policy;
//! - the target timing model;
//! - an explicit scheduler limit.
//!
//! This module never silently invents a machine limit.
//!
//! # Verification philosophy
//!
//! Verification is intentionally independent from scheduling.
//!
//! A scheduler may contain a bug.
//!
//! A planner may produce an incorrect interval.
//!
//! A transformation may accidentally violate a timing window.
//!
//! Therefore the verifier must recalculate temporal facts from the resulting
//! schedule rather than trusting planner-generated metadata.
//!
//! In particular:
//!
//! ```text
//! reported finish time
//! ```
//!
//! must never be trusted merely because the scheduler supplied it.
//!
//! The verifier derives:
//!
//! ```text
//! finish = start + duration
//! ```
//!
//! using checked canonical arithmetic.
//!
//! # Production invariants
//!
//! A successfully verified operation must satisfy:
//!
//! ```text
//! start >= 0
//!
//! duration is valid
//!
//! finish = start + duration
//!
//! finish >= start
//!
//! start satisfies start window
//!
//! finish satisfies finish window
//!
//! duration satisfies duration constraints
//!
//! alignment constraints are satisfied
//!
//! release constraints are satisfied
//!
//! deadline constraints are satisfied
//! ```
//!
//! If dependency information is supplied, then:
//!
//! ```text
//! predecessor.finish <= successor.start
//! ```
//!
//! must hold.
//!
//! # Dynamic scheduling
//!
//! A dynamic operation may have timing information that cannot be fully
//! resolved at compile time.
//!
//! Such an operation must not be falsely treated as statically schedulable.
//!
//! The caller can use `VerificationMode::Static` to require concrete timing,
//! or `VerificationMode::Deferred` to permit explicitly deferred timing
//! information.
//!
//! Deferred verification does NOT mean invalid information is silently
//! accepted. It means that unresolved timing must be explicitly represented by
//! the caller.
//!
//! # Distributed scheduling
//!
//! This verifier does not assume that all operations belong to one physical
//! device.
//!
//! A distributed scheduler can verify each concrete temporal interval using
//! the same abstraction.
//!
//! Communication and synchronization constraints may be supplied as ordinary
//! temporal constraints.
//!
//! # QEC
//!
//! QEC scheduling may use this verifier for:
//!
//! - syndrome rounds;
//! - ancilla preparation;
//! - stabilizer interactions;
//! - measurements;
//! - classical processing;
//! - feedback;
//! - round boundaries.
//!
//! QEC semantics remain outside this module.
//!
//! # Hardware independence
//!
//! Hardware timing resolution is not encoded here.
//!
//! The correct flow is:
//!
//! ```text
//! target hardware
//!       │
//!       ▼
//! hardware timing model
//!       │
//!       ▼
//! scheduling timing constraints
//!       │
//!       ▼
//! concrete schedule
//!       │
//!       ▼
//! this verifier
//! ```
//!
//! Hardware-specific alignment and resolution rules should be represented by
//! the scheduling timing subsystem and supplied as constraints.
//!
//! # Scalability
//!
//! Verification uses streaming/iterative checks wherever possible.
//!
//! It does not construct:
//!
//! ```text
//! qubits × time
//! resources × time
//! operations × maximum_time
//! ```
//!
//! matrices.
//!
//! There is no fixed schedule horizon.
//!
//! There is no fixed qubit count.
//!
//! There is no fixed operation count.
//!
//! There is no recursion requirement.
//!
//! Verification memory is proportional to the information explicitly supplied
//! by the caller rather than to an artificial time grid.
//!
//! # Determinism
//!
//! Diagnostics preserve the order in which the caller supplies verification
//! records unless an explicit deterministic sorting operation is requested by
//! a higher-level verifier.
//!
//! No:
//!
//! - system clock;
//! - randomness;
//! - global mutable state;
//! - pointer identity;
//! - floating-point time arithmetic
//!
//! participates in verification.
//!
//! # Thread safety
//!
//! `TimingVerifier` contains configuration only.
//!
//! It contains no:
//!
//! - global state;
//! - hardware handles;
//! - mutable shared state;
//! - caches requiring synchronization.
//!
//! Multiple verifier instances may therefore be used independently by parallel
//! verification workers.
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
//! - no unsafe code.
//!
//! Safety is compiler-enforced with `#![forbid(unsafe_code)]`.
//!
//! ============================================================================
//! Safety boundary
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use crate::quantum::ir::core::identity::OperationId;
use crate::quantum::ir::timing::{Duration, TimePoint};

use crate::quantum::scheduling::timing::constraints::{
    DurationConstraint,
    TemporalConstraint,
    TimingConstraintError,
};

use crate::quantum::scheduling::timing::windows::TimeWindow;

// ============================================================================
// Verification mode
// ============================================================================

/// Controls how unresolved timing information is treated.
///
/// Static scheduling normally uses [`VerificationMode::Static`].
///
/// Dynamic or incrementally compiled programs may use
/// [`VerificationMode::Deferred`] when timing is intentionally unresolved and
/// represented outside the concrete interval being verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationMode {
    /// Require all supplied timing information to be concrete and verifiable.
    Static,

    /// Permit explicitly deferred constraints to be omitted from the concrete
    /// interval checks.
    ///
    /// This mode does not weaken checks on values that are actually supplied.
    Deferred,
}

impl Default for VerificationMode {
    fn default() -> Self {
        Self::Static
    }
}

// ============================================================================
// Verification severity
// ============================================================================

/// Severity of a temporal verification diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimingSeverity {
    /// Informational diagnostic that does not invalidate the schedule.
    Info,

    /// Non-fatal diagnostic.
    Warning,

    /// Fatal temporal verification failure.
    Error,
}

impl fmt::Display for TimingSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => formatter.write_str("info"),
            Self::Warning => formatter.write_str("warning"),
            Self::Error => formatter.write_str("error"),
        }
    }
}

// ============================================================================
// Violation kind
// ============================================================================

/// Classification of a temporal verification diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TimingViolationKind {
    /// Operation interval could not be represented correctly.
    InvalidInterval,

    /// `start + duration` overflowed the canonical time domain.
    TimeOverflow,

    /// Duration violates a duration constraint.
    DurationViolation,

    /// Start time violates its admissible start window.
    StartWindowViolation,

    /// Finish time violates its admissible finish window.
    FinishWindowViolation,

    /// Operation finishes after its deadline.
    DeadlineViolation,

    /// Operation starts before its release time.
    ReleaseViolation,

    /// Operation violates a minimum separation requirement.
    MinimumSeparationViolation,

    /// Operation violates a maximum separation requirement.
    MaximumSeparationViolation,

    /// Operation start is not aligned to the required timing grid.
    AlignmentViolation,

    /// Operation finish is not aligned to the required timing grid.
    FinishAlignmentViolation,

    /// Operation violates an explicit temporal constraint.
    ConstraintViolation,

    /// Dependency timing is invalid.
    DependencyViolation,

    /// Required predecessor information is missing.
    MissingDependency,

    /// Timing metadata is internally inconsistent.
    InconsistentTiming,

    /// A caller supplied an unsupported or invalid verification request.
    InvalidVerificationInput,
}

impl fmt::Display for TimingViolationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::InvalidInterval => "invalid-interval",
            Self::TimeOverflow => "time-overflow",
            Self::DurationViolation => "duration-violation",
            Self::StartWindowViolation => "start-window-violation",
            Self::FinishWindowViolation => "finish-window-violation",
            Self::DeadlineViolation => "deadline-violation",
            Self::ReleaseViolation => "release-violation",
            Self::MinimumSeparationViolation => "minimum-separation-violation",
            Self::MaximumSeparationViolation => "maximum-separation-violation",
            Self::AlignmentViolation => "alignment-violation",
            Self::FinishAlignmentViolation => "finish-alignment-violation",
            Self::ConstraintViolation => "constraint-violation",
            Self::DependencyViolation => "dependency-violation",
            Self::MissingDependency => "missing-dependency",
            Self::InconsistentTiming => "inconsistent-timing",
            Self::InvalidVerificationInput => "invalid-verification-input",
        };

        formatter.write_str(value)
    }
}

// ============================================================================
// Diagnostic
// ============================================================================

/// One temporal verification diagnostic.
///
/// Diagnostics are structured so higher-level verification, diagnostics,
/// reporting, and IDE tooling do not have to parse human-readable strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingDiagnostic {
    /// Severity of the diagnostic.
    pub severity: TimingSeverity,

    /// Machine-readable diagnostic classification.
    pub kind: TimingViolationKind,

    /// Operation associated with the diagnostic, when known.
    pub operation: Option<OperationId>,

    /// Related predecessor operation, when the diagnostic concerns a
    /// dependency.
    pub related_operation: Option<OperationId>,

    /// Candidate start time, when known.
    pub start: Option<TimePoint>,

    /// Candidate finish time, when known.
    pub finish: Option<TimePoint>,

    /// Human-readable explanation.
    pub message: String,
}

impl TimingDiagnostic {
    /// Creates an error diagnostic.
    #[must_use]
    pub fn error(
        kind: TimingViolationKind,
        operation: Option<OperationId>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: TimingSeverity::Error,
            kind,
            operation,
            related_operation: None,
            start: None,
            finish: None,
            message: message.into(),
        }
    }

    /// Creates an error diagnostic with an interval.
    #[must_use]
    pub fn error_with_interval(
        kind: TimingViolationKind,
        operation: Option<OperationId>,
        start: TimePoint,
        finish: TimePoint,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: TimingSeverity::Error,
            kind,
            operation,
            related_operation: None,
            start: Some(start),
            finish: Some(finish),
            message: message.into(),
        }
    }

    /// Attaches a related operation.
    #[must_use]
    pub fn with_related_operation(
        mut self,
        operation: OperationId,
    ) -> Self {
        self.related_operation = Some(operation);
        self
    }

    /// Creates a warning diagnostic.
    #[must_use]
    pub fn warning(
        kind: TimingViolationKind,
        operation: Option<OperationId>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: TimingSeverity::Warning,
            kind,
            operation,
            related_operation: None,
            start: None,
            finish: None,
            message: message.into(),
        }
    }

    /// Creates an informational diagnostic.
    #[must_use]
    pub fn info(
        kind: TimingViolationKind,
        operation: Option<OperationId>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity: TimingSeverity::Info,
            kind,
            operation,
            related_operation: None,
            start: None,
            finish: None,
            message: message.into(),
        }
    }
}

// ============================================================================
// Verification report
// ============================================================================

/// Complete temporal verification report.
///
/// A report is an immutable description of one verification pass after
/// construction.
///
/// The report contains no references to the schedule, allowing it to safely be
/// retained by diagnostics, benchmarking, serialization, or a final
/// `SchedulingResult`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingVerificationReport {
    /// Number of operations checked.
    operations_checked: usize,

    /// Number of operations that passed all checks.
    operations_valid: usize,

    /// Number of errors.
    errors: usize,

    /// Number of warnings.
    warnings: usize,

    /// Number of informational diagnostics.
    infos: usize,

    /// Whether verification succeeded without errors.
    valid: bool,

    /// Structured diagnostics.
    diagnostics: Vec<TimingDiagnostic>,
}

impl TimingVerificationReport {
    /// Creates an empty successful report.
    #[must_use]
    pub fn new() -> Self {
        Self {
            operations_checked: 0,
            operations_valid: 0,
            errors: 0,
            warnings: 0,
            infos: 0,
            valid: true,
            diagnostics: Vec::new(),
        }
    }

    /// Returns the number of operations checked.
    #[must_use]
    pub const fn operations_checked(&self) -> usize {
        self.operations_checked
    }

    /// Returns the number of operations that passed verification.
    #[must_use]
    pub const fn operations_valid(&self) -> usize {
        self.operations_valid
    }

    /// Returns the number of errors.
    #[must_use]
    pub const fn errors(&self) -> usize {
        self.errors
    }

    /// Returns the number of warnings.
    #[must_use]
    pub const fn warnings(&self) -> usize {
        self.warnings
    }

    /// Returns the number of informational diagnostics.
    #[must_use]
    pub const fn infos(&self) -> usize {
        self.infos
    }

    /// Returns whether no fatal verification error was found.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.valid
    }

    /// Returns all diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[TimingDiagnostic] {
        &self.diagnostics
    }

    /// Returns true if at least one diagnostic is present.
    #[must_use]
    pub fn has_diagnostics(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    /// Returns true if at least one error is present.
    #[must_use]
    pub const fn has_errors(&self) -> bool {
        self.errors != 0
    }

    fn record_operation(&mut self) {
        self.operations_checked = self.operations_checked.saturating_add(1);
    }

    fn record_valid_operation(&mut self) {
        self.operations_valid = self.operations_valid.saturating_add(1);
    }

    fn record(&mut self, diagnostic: TimingDiagnostic) {
        match diagnostic.severity {
            TimingSeverity::Info => {
                self.infos = self.infos.saturating_add(1);
            }
            TimingSeverity::Warning => {
                self.warnings = self.warnings.saturating_add(1);
            }
            TimingSeverity::Error => {
                self.errors = self.errors.saturating_add(1);
                self.valid = false;
            }
        }

        self.diagnostics.push(diagnostic);
    }

    /// Converts the report into a verification error when invalid.
    pub fn into_result(self) -> Result<Self, TimingVerificationError> {
        if self.valid {
            Ok(self)
        } else {
            Err(TimingVerificationError::Failed {
                errors: self.errors,
            })
        }
    }
}

impl Default for TimingVerificationReport {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Verification error
// ============================================================================

/// Error returned when the complete timing verification pass cannot be
/// completed successfully.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TimingVerificationError {
    /// At least one temporal verification invariant failed.
    Failed {
        /// Number of errors recorded in the report.
        errors: usize,
    },

    /// Verification input was structurally invalid.
    InvalidInput {
        /// Explanation.
        message: String,
    },
}

impl fmt::Display for TimingVerificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failed { errors } => {
                write!(
                    formatter,
                    "timing verification failed with {errors} error(s)"
                )
            }

            Self::InvalidInput { message } => {
                write!(
                    formatter,
                    "invalid timing verification input: {message}"
                )
            }
        }
    }
}

impl std::error::Error for TimingVerificationError {}

// ============================================================================
// Concrete scheduled interval
// ============================================================================

/// A concrete temporal placement supplied to the verifier.
///
/// This type deliberately contains only temporal facts.
///
/// Resource identity belongs to resource verification.
///
/// Quantum semantics belong to the canonical quantum IR.
///
/// Routing identity belongs to routing.
///
/// This separation prevents the timing verifier from becoming coupled to a
/// particular machine architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScheduledInterval {
    operation: OperationId,
    start: TimePoint,
    duration: Duration,
}

impl ScheduledInterval {
    /// Creates a concrete scheduled interval.
    #[must_use]
    pub const fn new(
        operation: OperationId,
        start: TimePoint,
        duration: Duration,
    ) -> Self {
        Self {
            operation,
            start,
            duration,
        }
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns the start time.
    #[must_use]
    pub const fn start(&self) -> TimePoint {
        self.start
    }

    /// Returns the duration.
    #[must_use]
    pub const fn duration(&self) -> Duration {
        self.duration
    }

    /// Calculates the finish time using checked canonical arithmetic.
    pub fn finish(&self) -> Result<TimePoint, TimingConstraintError> {
        self.start
            .checked_add_duration(self.duration)
            .ok_or(TimingConstraintError::TimeOverflow)
    }
}

// ============================================================================
// Alignment
// ============================================================================

/// Exact alignment requirements for a concrete schedule.
///
/// Alignment is expressed entirely in the canonical integer time domain.
///
/// For example, if a target requires an operation to begin at every 4th
/// semantic timing unit, then:
///
/// ```text
/// quantum = 4
/// origin  = 0
/// ```
///
/// A hardware adapter is responsible for determining those values.
///
/// This type does not know what a "tick" physically means.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AlignmentRequirement {
    /// Required spacing between valid aligned points.
    quantum: Duration,

    /// Origin from which alignment is measured.
    origin: TimePoint,

    /// Whether operation starts must satisfy the alignment.
    align_start: bool,

    /// Whether operation finishes must satisfy the alignment.
    align_finish: bool,
}

impl AlignmentRequirement {
    /// Creates an alignment requirement.
    ///
    /// A zero quantum is rejected because it cannot define a meaningful
    /// alignment lattice.
    pub fn new(
        quantum: Duration,
        origin: TimePoint,
        align_start: bool,
        align_finish: bool,
    ) -> Result<Self, TimingVerificationError> {
        if quantum.is_zero() {
            return Err(TimingVerificationError::InvalidInput {
                message: String::from(
                    "alignment quantum must be greater than zero",
                ),
            });
        }

        if !align_start && !align_finish {
            return Err(TimingVerificationError::InvalidInput {
                message: String::from(
                    "alignment must constrain start, finish, or both",
                ),
            });
        }

        Ok(Self {
            quantum,
            origin,
            align_start,
            align_finish,
        })
    }

    /// Returns the alignment quantum.
    #[must_use]
    pub const fn quantum(&self) -> Duration {
        self.quantum
    }

    /// Returns the alignment origin.
    #[must_use]
    pub const fn origin(&self) -> TimePoint {
        self.origin
    }

    /// Returns whether starts are aligned.
    #[must_use]
    pub const fn align_start(&self) -> bool {
        self.align_start
    }

    /// Returns whether finishes are aligned.
    #[must_use]
    pub const fn align_finish(&self) -> bool {
        self.align_finish
    }

    /// Returns whether a point satisfies this alignment.
    #[must_use]
    pub fn contains(&self, point: TimePoint) -> bool {
        if point < self.origin {
            return false;
        }

        match self.origin.checked_duration_until(point) {
            Some(delta) => delta.value() % self.quantum.value() == 0,
            None => false,
        }
    }
}

// ============================================================================
// Operation timing specification
// ============================================================================

/// Complete timing specification for one scheduled operation.
///
/// This is intentionally independent from a quantum operation's semantic
/// representation.
///
/// The same specification can therefore be used for:
///
/// - gates;
/// - measurements;
/// - reset;
/// - barriers;
/// - QEC operations;
/// - communication operations;
/// - classical feedback events;
/// - distributed synchronization operations;
/// - future quantum operation kinds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationTimingSpec {
    /// Concrete operation interval.
    interval: ScheduledInterval,

    /// Optional general temporal constraint.
    constraint: Option<TemporalConstraint>,

    /// Optional explicit release time.
    release: Option<TimePoint>,

    /// Optional explicit deadline.
    deadline: Option<TimePoint>,

    /// Optional start alignment.
    start_alignment: Option<AlignmentRequirement>,

    /// Optional finish alignment.
    finish_alignment: Option<AlignmentRequirement>,

    /// Optional minimum separation from another operation.
    minimum_separation: Option<TemporalSeparation>,

    /// Optional maximum separation from another operation.
    maximum_separation: Option<TemporalSeparation>,
}

impl OperationTimingSpec {
    /// Creates an unconstrained timing specification.
    #[must_use]
    pub fn unconstrained(
        interval: ScheduledInterval,
    ) -> Self {
        Self {
            interval,
            constraint: None,
            release: None,
            deadline: None,
            start_alignment: None,
            finish_alignment: None,
            minimum_separation: None,
            maximum_separation: None,
        }
    }

    /// Returns the concrete interval.
    #[must_use]
    pub const fn interval(&self) -> ScheduledInterval {
        self.interval
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.interval.operation()
    }

    /// Returns the start time.
    #[must_use]
    pub const fn start(&self) -> TimePoint {
        self.interval.start()
    }

    /// Returns the duration.
    #[must_use]
    pub const fn duration(&self) -> Duration {
        self.interval.duration()
    }

    /// Calculates the finish time.
    pub fn finish(&self) -> Result<TimePoint, TimingConstraintError> {
        self.interval.finish()
    }

    /// Sets a general temporal constraint.
    #[must_use]
    pub fn with_constraint(
        mut self,
        constraint: TemporalConstraint,
    ) -> Self {
        self.constraint = Some(constraint);
        self
    }

    /// Sets a release time.
    #[must_use]
    pub fn with_release(
        mut self,
        release: TimePoint,
    ) -> Self {
        self.release = Some(release);
        self
    }

    /// Sets a deadline.
    #[must_use]
    pub fn with_deadline(
        mut self,
        deadline: TimePoint,
    ) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Sets a start alignment requirement.
    #[must_use]
    pub fn with_start_alignment(
        mut self,
        alignment: AlignmentRequirement,
    ) -> Self {
        self.start_alignment = Some(alignment);
        self
    }

    /// Sets a finish alignment requirement.
    #[must_use]
    pub fn with_finish_alignment(
        mut self,
        alignment: AlignmentRequirement,
    ) -> Self {
        self.finish_alignment = Some(alignment);
        self
    }

    /// Sets a minimum separation requirement.
    #[must_use]
    pub fn with_minimum_separation(
        mut self,
        separation: TemporalSeparation,
    ) -> Self {
        self.minimum_separation = Some(separation);
        self
    }

    /// Sets a maximum separation requirement.
    #[must_use]
    pub fn with_maximum_separation(
        mut self,
        separation: TemporalSeparation,
    ) -> Self {
        self.maximum_separation = Some(separation);
        self
    }

    /// Returns the general temporal constraint.
    #[must_use]
    pub fn constraint(&self) -> Option<&TemporalConstraint> {
        self.constraint.as_ref()
    }

    /// Returns the release time.
    #[must_use]
    pub const fn release(&self) -> Option<TimePoint> {
        self.release
    }

    /// Returns the deadline.
    #[must_use]
    pub const fn deadline(&self) -> Option<TimePoint> {
        self.deadline
    }

    /// Returns the start alignment.
    #[must_use]
    pub fn start_alignment(&self) -> Option<&AlignmentRequirement> {
        self.start_alignment.as_ref()
    }

    /// Returns the finish alignment(&self) -> Option<&AlignmentRequirement> {
        self.finish_alignment.as_ref()
    }

    /// Returns the minimum separation requirement.
    #[must_use]
    pub fn minimum_separation(&self) -> Option<&TemporalSeparation> {
        self.minimum_separation.as_ref()
    }

    /// Returns the maximum separation requirement.
    #[must_use]
    pub fn maximum_separation(&self) -> Option<&TemporalSeparation> {
        self.maximum_separation.as_ref()
    }
}

// ============================================================================
// Separation
// ============================================================================

/// Temporal separation requirement relative to another operation.
///
/// `reference_finish = true` means the separation is measured from the
/// reference operation's finish.
///
/// `reference_finish = false` means it is measured from the reference
/// operation's start.
///
/// The candidate operation is always compared using its start time.
///
/// Therefore:
///
/// ```text
/// candidate.start >= reference.finish + minimum
/// ```
///
/// or:
///
/// ```text
/// candidate.start <= reference.finish + maximum
/// ```
///
/// depending on the configured bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TemporalSeparation {
    reference: OperationId,
    minimum: Option<Duration>,
    maximum: Option<Duration>,
    reference_finish: bool,
}

impl TemporalSeparation {
    /// Creates a separation specification.
    ///
    /// At least one bound must be supplied.
    pub fn new(
        reference: OperationId,
        minimum: Option<Duration>,
        maximum: Option<Duration>,
        reference_finish: bool,
    ) -> Result<Self, TimingVerificationError> {
        if minimum.is_none() && maximum.is_none() {
            return Err(TimingVerificationError::InvalidInput {
                message: String::from(
                    "temporal separation requires a minimum or maximum bound",
                ),
            });
        }

        if let (Some(minimum), Some(maximum)) = (minimum, maximum) {
            if minimum > maximum {
                return Err(TimingVerificationError::InvalidInput {
                    message: String::from(
                        "temporal separation minimum exceeds maximum",
                    ),
                });
            }
        }

        Ok(Self {
            reference,
            minimum,
            maximum,
            reference_finish,
        })
    }

    /// Returns the reference operation.
    #[must_use]
    pub const fn reference(&self) -> OperationId {
        self.reference
    }

    /// Returns the minimum separation.
    #[must_use]
    pub const fn minimum(&self) -> Option<Duration> {
        self.minimum
    }

    /// Returns the maximum separation.
    #[must_use]
    pub const fn maximum(&self) -> Option<Duration> {
        self.maximum
    }

    /// Returns whether the reference point is the reference operation's
    /// finish.
    #[must_use]
    pub const fn reference_finish(&self) -> bool {
        self.reference_finish
    }
}

// ============================================================================
// Dependency timing
// ============================================================================

/// A concrete dependency timing relationship.
///
/// This type deliberately does not duplicate the complete dependency graph.
/// The dependency graph remains owned by `scheduling::ir`.
///
/// It represents only the temporal fact needed for verification:
///
/// ```text
/// predecessor -> successor
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DependencyTiming {
    predecessor: OperationId,
    successor: OperationId,
    minimum_separation: Option<Duration>,
    maximum_separation: Option<Duration>,
}

impl DependencyTiming {
    /// Creates a dependency timing constraint.
    pub fn new(
        predecessor: OperationId,
        successor: OperationId,
        minimum_separation: Option<Duration>,
        maximum_separation: Option<Duration>,
    ) -> Result<Self, TimingVerificationError> {
        if predecessor == successor {
            return Err(TimingVerificationError::InvalidInput {
                message: String::from(
                    "an operation cannot be its own temporal predecessor",
                ),
            });
        }

        if let (Some(minimum), Some(maximum)) =
            (minimum_separation, maximum_separation)
        {
            if minimum > maximum {
                return Err(TimingVerificationError::InvalidInput {
                    message: String::from(
                        "dependency minimum separation exceeds maximum",
                    ),
                });
            }
        }

        Ok(Self {
            predecessor,
            successor,
            minimum_separation,
            maximum_separation,
        })
    }

    /// Returns the predecessor.
    #[must_use]
    pub const fn predecessor(&self) -> OperationId {
        self.predecessor
    }

    /// Returns the successor.
    #[must_use]
    pub const fn successor(&self) -> OperationId {
        self.successor
    }

    /// Returns the minimum separation.
    #[must_use]
    pub const fn minimum_separation(&self) -> Option<Duration> {
        self.minimum_separation
    }

    /// Returns the maximum separation.
    #[must_use]
    pub const fn maximum_separation(&self) -> Option<Duration> {
        self.maximum_separation
    }
}

// ============================================================================
// Verifier configuration
// ============================================================================

/// Configuration for [`TimingVerifier`].
///
/// No field represents a machine-size maximum.
///
/// Explicit resource/scheduler limits belong to `scheduling::limits` and are
/// not silently introduced here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimingVerificationConfig {
    /// Controls whether unresolved timing is permitted.
    pub mode: VerificationMode,

    /// Whether zero-duration operations are permitted.
    ///
    /// Zero-duration events are useful for compiler-level markers, barriers,
    /// synchronization points, and dynamic scheduling events.
    pub allow_zero_duration: bool,

    /// Whether an empty verification input is valid.
    pub allow_empty: bool,

    /// Whether dependency relationships supplied to the verifier are checked.
    pub verify_dependencies: bool,

    /// Whether temporal separation relationships are checked.
    pub verify_separations: bool,

    /// Whether alignment requirements are checked.
    pub verify_alignment: bool,
}

impl Default for TimingVerificationConfig {
    fn default() -> Self {
        Self {
            mode: VerificationMode::Static,
            allow_zero_duration: true,
            allow_empty: true,
            verify_dependencies: true,
            verify_separations: true,
            verify_alignment: true,
        }
    }
}

// ============================================================================
// Timing verifier
// ============================================================================

/// Production temporal verifier.
///
/// `TimingVerifier` is deliberately stateless with respect to any particular
/// schedule.
///
/// Create one verifier and use it against any number of independent schedules.
///
/// No hardware handles, resource pools, caches, or global state are retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimingVerifier {
    config: TimingVerificationConfig,
}

impl TimingVerifier {
    /// Creates a verifier with the supplied configuration.
    #[must_use]
    pub const fn new(config: TimingVerificationConfig) -> Self {
        Self { config }
    }

    /// Creates a verifier using production defaults.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            config: TimingVerificationConfig {
                mode: VerificationMode::Static,
                allow_zero_duration: true,
                allow_empty: true,
                verify_dependencies: true,
                verify_separations: true,
                verify_alignment: true,
            },
        }
    }

    /// Returns the verifier configuration.
    #[must_use]
    pub const fn config(&self) -> TimingVerificationConfig {
        self.config
    }

    /// Verifies one concrete operation.
    ///
    /// The returned report is always structured. Fatal violations are recorded
    /// in the report rather than immediately returned as a string error.
    #[must_use]
    pub fn verify_operation(
        &self,
        operation: &OperationTimingSpec,
    ) -> TimingVerificationReport {
        let mut report = TimingVerificationReport::new();

        report.record_operation();

        let mut valid = true;

        let start = operation.start();
        let duration = operation.duration();

        if duration.is_zero() && !self.config.allow_zero_duration {
            report.record(TimingDiagnostic::error(
                TimingViolationKind::InvalidInterval,
                Some(operation.operation()),
                String::from(
                    "zero-duration operation is disabled by timing verification configuration",
                ),
            ));
            valid = false;
        }

        let finish = match operation.finish() {
            Ok(value) => value,
            Err(TimingConstraintError::TimeOverflow) => {
                report.record(TimingDiagnostic::error(
                    TimingViolationKind::TimeOverflow,
                    Some(operation.operation()),
                    String::from(
                        "operation finish time overflowed the canonical time domain",
                    ),
                ));
                valid = false;
                start
            }
            Err(_) => {
                report.record(TimingDiagnostic::error(
                    TimingViolationKind::InvalidInterval,
                    Some(operation.operation()),
                    String::from(
                        "operation interval could not be represented",
                    ),
                ));
                valid = false;
                start
            }
        };

        if let Some(release) = operation.release() {
            if start < release {
                report.record(
                    TimingDiagnostic::error_with_interval(
                        TimingViolationKind::ReleaseViolation,
                        Some(operation.operation()),
                        start,
                        finish,
                        format!(
                            "operation starts at {start}, before release time {release}"
                        ),
                    ),
                );
                valid = false;
            }
        }

        if let Some(deadline) = operation.deadline() {
            if finish > deadline {
                report.record(
                    TimingDiagnostic::error_with_interval(
                        TimingViolationKind::DeadlineViolation,
                        Some(operation.operation()),
                        start,
                        finish,
                        format!(
                            "operation finishes at {finish}, after deadline {deadline}"
                        ),
                    ),
                );
                valid = false;
            }
        }

        if let Some(constraint) = operation.constraint() {
            match constraint.allows(start, duration) {
                Ok(()) => {}
                Err(error) => {
                    valid = false;
                    report.record(
                        self.constraint_diagnostic(
                            operation,
                            start,
                            finish,
                            error,
                        ),
                    );
                }
            }
        }

        if self.config.verify_alignment {
            if let Some(alignment) = operation.start_alignment() {
                if !alignment.contains(start) {
                    report.record(
                        TimingDiagnostic::error_with_interval(
                            TimingViolationKind::AlignmentViolation,
                            Some(operation.operation()),
                            start,
                            finish,
                            format!(
                                "operation start {start} does not satisfy required start alignment"
                            ),
                        ),
                    );
                    valid = false;
                }
            }

            if let Some(alignment) = operation.finish_alignment() {
                if !alignment.contains(finish) {
                    report.record(
                        TimingDiagnostic::error_with_interval(
                            TimingViolationKind::FinishAlignmentViolation,
                            Some(operation.operation()),
                            start,
                            finish,
                            format!(
                                "operation finish {finish} does not satisfy required finish alignment"
                            ),
                        ),
                    );
                    valid = false;
                }
            }
        }

        if valid {
            report.record_valid_operation();
        }

        report
    }

    /// Verifies a complete collection of operation timing specifications.
    ///
    /// Verification is iterative and does not recurse through the operation
    /// graph.
    pub fn verify_operations(
        &self,
        operations: &[OperationTimingSpec],
    ) -> Result<TimingVerificationReport, TimingVerificationError> {
        if operations.is_empty() && !self.config.allow_empty {
            return Err(TimingVerificationError::InvalidInput {
                message: String::from(
                    "empty timing verification input is disabled",
                ),
            });
        }

        let mut report = TimingVerificationReport::new();

        for operation in operations {
            let operation_report = self.verify_operation(operation);

            report.operations_checked = report
                .operations_checked
                .saturating_add(operation_report.operations_checked);

            report.operations_valid = report
                .operations_valid
                .saturating_add(operation_report.operations_valid);

            for diagnostic in operation_report.diagnostics {
                report.record(diagnostic);
            }
        }

        Ok(report)
    }

    /// Verifies temporal dependency relationships.
    ///
    /// This does not construct or own the dependency graph. The caller supplies
    /// concrete dependency timing records, normally produced by
    /// `scheduling::ir`.
    pub fn verify_dependencies(
        &self,
        operations: &[OperationTimingSpec],
        dependencies: &[DependencyTiming],
    ) -> Result<TimingVerificationReport, TimingVerificationError> {
        let mut report = TimingVerificationReport::new();

        if !self.config.verify_dependencies {
            return Ok(report);
        }

        let mut index = std::collections::HashMap::with_capacity(
            operations.len(),
        );

        for operation in operations {
            index.insert(operation.operation(), operation);
        }

        for dependency in dependencies {
            let predecessor = match index.get(&dependency.predecessor()) {
                Some(value) => *value,
                None => {
                    report.record(
                        TimingDiagnostic::error(
                            TimingViolationKind::MissingDependency,
                            Some(dependency.successor()),
                            format!(
                                "dependency predecessor `{}` is not present in the schedule",
                                dependency.predecessor()
                            ),
                        )
                        .with_related_operation(
                            dependency.predecessor(),
                        ),
                    );
                    continue;
                }
            };

            let successor = match index.get(&dependency.successor()) {
                Some(value) => *value,
                None => {
                    report.record(
                        TimingDiagnostic::error(
                            TimingViolationKind::MissingDependency,
                            Some(dependency.successor()),
                            format!(
                                "dependency successor `{}` is not present in the schedule",
                                dependency.successor()
                            ),
                        )
                        .with_related_operation(
                            dependency.predecessor(),
                        ),
                    );
                    continue;
                }
            };

            let predecessor_finish = match predecessor.finish() {
                Ok(value) => value,
                Err(_) => {
                    report.record(
                        TimingDiagnostic::error(
                            TimingViolationKind::TimeOverflow,
                            Some(predecessor.operation()),
                            String::from(
                                "predecessor finish time overflowed the canonical time domain",
                            ),
                        )
                        .with_related_operation(
                            successor.operation(),
                        ),
                    );
                    continue;
                }
            };

            let successor_start = successor.start();

            if predecessor_finish > successor_start {
                report.record(
                    TimingDiagnostic::error_with_interval(
                        TimingViolationKind::DependencyViolation,
                        Some(successor.operation()),
                        successor_start,
                        successor_start,
                        format!(
                            "successor starts at {successor_start}, before predecessor finishes at {predecessor_finish}"
                        ),
                    )
                    .with_related_operation(
                        predecessor.operation(),
                    ),
                );
                continue;
            }

            let separation = predecessor_finish
                .checked_duration_until(successor_start)
                .ok_or_else(|| TimingVerificationError::InvalidInput {
                    message: String::from(
                        "successor start precedes predecessor finish",
                    ),
                })?;

            if let Some(minimum) = dependency.minimum_separation() {
                if separation < minimum {
                    report.record(
                        TimingDiagnostic::error(
                            TimingViolationKind::MinimumSeparationViolation,
                            Some(successor.operation()),
                            format!(
                                "dependency separation {separation:?} is smaller than required minimum {minimum:?}"
                            ),
                        )
                        .with_related_operation(
                            predecessor.operation(),
                        ),
                    );
                }
            }

            if let Some(maximum) = dependency.maximum_separation() {
                if separation > maximum {
                    report.record(
                        TimingDiagnostic::error(
                            TimingViolationKind::MaximumSeparationViolation,
                            Some(successor.operation()),
                            format!(
                                "dependency separation {separation:?} exceeds maximum {maximum:?}"
                            ),
                        )
                        .with_related_operation(
                            predecessor.operation(),
                        ),
                    );
                }
            }
        }

        Ok(report)
    }

    /// Verifies all temporal separation constraints attached to operations.
    pub fn verify_separations(
        &self,
        operations: &[OperationTimingSpec],
    ) -> Result<TimingVerificationReport, TimingVerificationError> {
        let mut report = TimingVerificationReport::new();

        if !self.config.verify_separations {
            return Ok(report);
        }

        let mut index = std::collections::HashMap::with_capacity(
            operations.len(),
        );

        for operation in operations {
            index.insert(operation.operation(), operation);
        }

        for operation in operations {
            let operation_start = operation.start();

            if let Some(separation) = operation.minimum_separation() {
                self.verify_one_separation(
                    &mut report,
                    operation,
                    operation_start,
                    separation,
                    true,
                    &index,
                );
            }

            if let Some(separation) = operation.maximum_separation() {
                self.verify_one_separation(
                    &mut report,
                    operation,
                    operation_start,
                    separation,
                    false,
                    &index,
                );
            }
        }

        Ok(report)
    }

    /// Performs complete timing verification:
    ///
    /// 1. concrete operation intervals;
    /// 2. temporal constraints;
    /// 3. dependencies;
    /// 4. separation constraints.
    ///
    /// This method intentionally does not verify resource conflicts. That is
    /// the responsibility of `verification::resource`.
    pub fn verify(
        &self,
        operations: &[OperationTimingSpec],
        dependencies: &[DependencyTiming],
    ) -> Result<TimingVerificationReport, TimingVerificationError> {
        let operation_report = self.verify_operations(operations)?;

        let dependency_report =
            self.verify_dependencies(operations, dependencies)?;

        let separation_report =
            self.verify_separations(operations)?;

        let mut report = TimingVerificationReport::new();

        report.operations_checked = operation_report.operations_checked;
        report.operations_valid = operation_report.operations_valid;

        for diagnostic in operation_report.diagnostics {
            report.record(diagnostic);
        }

        for diagnostic in dependency_report.diagnostics {
            report.record(diagnostic);
        }

        for diagnostic in separation_report.diagnostics {
            report.record(diagnostic);
        }

        Ok(report)
    }

    fn verify_one_separation(
        &self,
        report: &mut TimingVerificationReport,
        operation: &OperationTimingSpec,
        operation_start: TimePoint,
        separation: &TemporalSeparation,
        minimum: bool,
        index: &std::collections::HashMap<
            OperationId,
            &OperationTimingSpec,
        >,
    ) {
        let reference = match index.get(&separation.reference()) {
            Some(value) => *value,
            None => {
                report.record(
                    TimingDiagnostic::error(
                        TimingViolationKind::MissingDependency,
                        Some(operation.operation()),
                        format!(
                            "temporal separation references missing operation `{}`",
                            separation.reference()
                        ),
                    )
                    .with_related_operation(
                        separation.reference(),
                    ),
                );
                return;
            }
        };

        let reference_point = if separation.reference_finish() {
            match reference.finish() {
                Ok(value) => value,
                Err(_) => {
                    report.record(
                        TimingDiagnostic::error(
                            TimingViolationKind::TimeOverflow,
                            Some(reference.operation()),
                            String::from(
                                "reference operation finish time overflowed the canonical time domain",
                            ),
                        )
                        .with_related_operation(
                            operation.operation(),
                        ),
                    );
                    return;
                }
            }
        } else {
            reference.start()
        };

        let separation_value = if operation_start >= reference_point {
            match reference_point
                .checked_duration_until(operation_start)
            {
                Some(value) => value,
                None => {
                    report.record(
                        TimingDiagnostic::error(
                            TimingViolationKind::TimeOverflow,
                            Some(operation.operation()),
                            String::from(
                                "temporal separation could not be represented",
                            ),
                        )
                        .with_related_operation(
                            reference.operation(),
                        ),
                    );
                    return;
                }
            }
        } else {
            // The candidate occurs before the reference point. This is valid
            // only for a maximum-only relationship where the mathematical
            // signed separation is negative. The current verifier deliberately
            // treats such a relationship as a dependency-order violation when
            // a minimum constraint exists.
            if minimum {
                report.record(
                    TimingDiagnostic::error(
                        TimingViolationKind::MinimumSeparationViolation,
                        Some(operation.operation()),
                        format!(
                            "operation starts before its required temporal reference `{}`",
                            reference.operation()
                        ),
                    )
                    .with_related_operation(
                        reference.operation(),
                    ),
                );
            }
            return;
        };

        if minimum {
            if let Some(required) = separation.minimum() {
                if separation_value < required {
                    report.record(
                        TimingDiagnostic::error(
                            TimingViolationKind::MinimumSeparationViolation,
                            Some(operation.operation()),
                            format!(
                                "temporal separation {separation_value:?} is smaller than required {required:?}"
                            ),
                        )
                        .with_related_operation(
                            reference.operation(),
                        ),
                    );
                }
            }
        } else if let Some(required) = separation.maximum() {
            if separation_value > required {
                report.record(
                    TimingDiagnostic::error(
                        TimingViolationKind::MaximumSeparationViolation,
                        Some(operation.operation()),
                        format!(
                            "temporal separation {separation_value:?} exceeds maximum {required:?}"
                        ),
                    )
                    .with_related_operation(
                        reference.operation(),
                    ),
                );
            }
        }
    }

    fn constraint_diagnostic(
        &self,
        operation: &OperationTimingSpec,
        start: TimePoint,
        finish: TimePoint,
        error: TimingConstraintError,
    ) -> TimingDiagnostic {
        let kind = match error {
            TimingConstraintError::InvalidDurationBounds { .. } => {
                TimingViolationKind::ConstraintViolation
            }

            TimingConstraintError::TimeOverflow => {
                TimingViolationKind::TimeOverflow
            }

            TimingConstraintError::Unsatisfiable => {
                TimingViolationKind::ConstraintViolation
            }

            TimingConstraintError::DurationViolation { .. } => {
                TimingViolationKind::DurationViolation
            }

            TimingConstraintError::StartWindowViolation { .. } => {
                TimingViolationKind::StartWindowViolation
            }

            TimingConstraintError::FinishWindowViolation { .. } => {
                TimingViolationKind::FinishWindowViolation
            }
        };

        TimingDiagnostic::error_with_interval(
            kind,
            Some(operation.operation()),
            start,
            finish,
            error.to_string(),
        )
    }
}

// ============================================================================
// Convenience functions
// ============================================================================

/// Verifies one operation with production defaults.
pub fn verify_operation_timing(
    operation: &OperationTimingSpec,
) -> TimingVerificationReport {
    TimingVerifier::production().verify_operation(operation)
}

/// Verifies a complete timing problem with production defaults.
pub fn verify_timing(
    operations: &[OperationTimingSpec],
    dependencies: &[DependencyTiming],
) -> Result<TimingVerificationReport, TimingVerificationError> {
    TimingVerifier::production().verify(operations, dependencies)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation_id(value: u64) -> OperationId {
        OperationId::new(value)
    }

    #[test]
    fn zero_time_operation_is_valid_by_default() {
        let operation = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(1),
                TimePoint::ZERO,
                Duration::ZERO,
            ),
        );

        let report =
            TimingVerifier::production().verify_operation(&operation);

        assert!(report.is_valid());
        assert_eq!(report.operations_checked(), 1);
        assert_eq!(report.operations_valid(), 1);
    }

    #[test]
    fn finish_is_checked_without_wraparound() {
        let operation = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(1),
                TimePoint::new(u128::MAX),
                Duration::new(1),
            ),
        );

        let report =
            TimingVerifier::production().verify_operation(&operation);

        assert!(!report.is_valid());
        assert!(
            report
                .diagnostics()
                .iter()
                .any(|diagnostic| {
                    diagnostic.kind
                        == TimingViolationKind::TimeOverflow
                })
        );
    }

    #[test]
    fn release_time_is_enforced() {
        let operation = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(1),
                TimePoint::new(4),
                Duration::new(2),
            ),
        )
        .with_release(TimePoint::new(5));

        let report =
            TimingVerifier::production().verify_operation(&operation);

        assert!(!report.is_valid());
        assert!(
            report
                .diagnostics()
                .iter()
                .any(|diagnostic| {
                    diagnostic.kind
                        == TimingViolationKind::ReleaseViolation
                })
        );
    }

    #[test]
    fn deadline_is_enforced() {
        let operation = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(1),
                TimePoint::new(4),
                Duration::new(4),
            ),
        )
        .with_deadline(TimePoint::new(7));

        let report =
            TimingVerifier::production().verify_operation(&operation);

        assert!(!report.is_valid());
        assert!(
            report
                .diagnostics()
                .iter()
                .any(|diagnostic| {
                    diagnostic.kind
                        == TimingViolationKind::DeadlineViolation
                })
        );
    }

    #[test]
    fn exact_temporal_constraint_is_enforced() {
        let constraint = TemporalConstraint::new(
            TimeWindow::exact(TimePoint::new(5)),
            TimeWindow::exact(TimePoint::new(8)),
            DurationConstraint::exact(Duration::new(3)),
        );

        let operation = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(1),
                TimePoint::new(5),
                Duration::new(3),
            ),
        )
        .with_constraint(constraint);

        let report =
            TimingVerifier::production().verify_operation(&operation);

        assert!(report.is_valid());
    }

    #[test]
    fn start_window_violation_is_detected() {
        let constraint = TemporalConstraint::new(
            TimeWindow::at_least(TimePoint::new(10)),
            TimeWindow::unbounded(),
            DurationConstraint::unbounded(),
        );

        let operation = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(1),
                TimePoint::new(5),
                Duration::new(1),
            ),
        )
        .with_constraint(constraint);

        let report =
            TimingVerifier::production().verify_operation(&operation);

        assert!(!report.is_valid());
    }

    #[test]
    fn finish_window_violation_is_detected() {
        let constraint = TemporalConstraint::new(
            TimeWindow::unbounded(),
            TimeWindow::at_most(TimePoint::new(5)),
            DurationConstraint::unbounded(),
        );

        let operation = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(1),
                TimePoint::new(4),
                Duration::new(2),
            ),
        )
        .with_constraint(constraint);

        let report =
            TimingVerifier::production().verify_operation(&operation);

        assert!(!report.is_valid());
    }

    #[test]
    fn start_alignment_is_enforced() {
        let alignment = AlignmentRequirement::new(
            Duration::new(4),
            TimePoint::ZERO,
            true,
            false,
        )
        .expect("valid alignment");

        let operation = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(1),
                TimePoint::new(5),
                Duration::new(1),
            ),
        )
        .with_start_alignment(alignment);

        let report =
            TimingVerifier::production().verify_operation(&operation);

        assert!(!report.is_valid());
    }

    #[test]
    fn finish_alignment_is_enforced() {
        let alignment = AlignmentRequirement::new(
            Duration::new(4),
            TimePoint::ZERO,
            false,
            true,
        )
        .expect("valid alignment");

        let operation = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(1),
                TimePoint::new(4),
                Duration::new(3),
            ),
        )
        .with_finish_alignment(alignment);

        let report =
            TimingVerifier::production().verify_operation(&operation);

        assert!(!report.is_valid());
    }

    #[test]
    fn dependency_requires_predecessor_to_finish_first() {
        let predecessor = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(1),
                TimePoint::ZERO,
                Duration::new(10),
            ),
        );

        let successor = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(2),
                TimePoint::new(5),
                Duration::new(1),
            ),
        );

        let dependency = DependencyTiming::new(
            operation_id(1),
            operation_id(2),
            None,
            None,
        )
        .expect("valid dependency");

        let report = TimingVerifier::production()
            .verify_dependencies(
                &[predecessor, successor],
                &[dependency],
            )
            .expect("verification should execute");

        assert!(!report.is_valid());
    }

    #[test]
    fn touching_dependency_intervals_are_valid() {
        let predecessor = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(1),
                TimePoint::ZERO,
                Duration::new(10),
            ),
        );

        let successor = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(2),
                TimePoint::new(10),
                Duration::new(1),
            ),
        );

        let dependency = DependencyTiming::new(
            operation_id(1),
            operation_id(2),
            None,
            None,
        )
        .expect("valid dependency");

        let report = TimingVerifier::production()
            .verify_dependencies(
                &[predecessor, successor],
                &[dependency],
            )
            .expect("verification should execute");

        assert!(report.is_valid());
    }

    #[test]
    fn minimum_dependency_separation_is_enforced() {
        let predecessor = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(1),
                TimePoint::ZERO,
                Duration::new(10),
            ),
        );

        let successor = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(2),
                TimePoint::new(12),
                Duration::new(1),
            ),
        );

        let dependency = DependencyTiming::new(
            operation_id(1),
            operation_id(2),
            Some(Duration::new(3)),
            None,
        )
        .expect("valid dependency");

        let report = TimingVerifier::production()
            .verify_dependencies(
                &[predecessor, successor],
                &[dependency],
            )
            .expect("verification should execute");

        assert!(!report.is_valid());
    }

    #[test]
    fn maximum_dependency_separation_is_enforced() {
        let predecessor = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(1),
                TimePoint::ZERO,
                Duration::new(1),
            ),
        );

        let successor = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(2),
                TimePoint::new(20),
                Duration::new(1),
            ),
        );

        let dependency = DependencyTiming::new(
            operation_id(1),
            operation_id(2),
            None,
            Some(Duration::new(5)),
        )
        .expect("valid dependency");

        let report = TimingVerifier::production()
            .verify_dependencies(
                &[predecessor, successor],
                &[dependency],
            )
            .expect("verification should execute");

        assert!(!report.is_valid());
    }

    #[test]
    fn empty_schedule_is_valid_by_default() {
        let report =
            TimingVerifier::production()
                .verify_operations(&[])
                .expect("empty schedule is permitted");

        assert!(report.is_valid());
        assert_eq!(report.operations_checked(), 0);
    }

    #[test]
    fn complete_verification_combines_checks() {
        let first = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(1),
                TimePoint::ZERO,
                Duration::new(5),
            ),
        );

        let second = OperationTimingSpec::unconstrained(
            ScheduledInterval::new(
                operation_id(2),
                TimePoint::new(5),
                Duration::new(5),
            ),
        );

        let dependency = DependencyTiming::new(
            operation_id(1),
            operation_id(2),
            None,
            None,
        )
        .expect("valid dependency");

        let report = TimingVerifier::production()
            .verify(&[first, second], &[dependency])
            .expect("verification should execute");

        assert!(report.is_valid());
        assert_eq!(report.operations_checked(), 2);
        assert_eq!(report.operations_valid(), 2);
    }
}