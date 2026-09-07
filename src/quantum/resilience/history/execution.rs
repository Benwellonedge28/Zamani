//! Zamani Quantum Resilience — Execution History
//!
//! Path:
//!     src/quantum/resilience/history/execution.rs
//!
//! Purpose:
//!     Provides the immutable historical record of a quantum execution.
//!
//! Architectural position:
//!
//!     Zamani program
//!          |
//!          v
//!     Canonical Quantum IR
//!          |
//!          v
//!     compilation / routing / scheduling
//!          |
//!          v
//!     execution runtime / hardware HAL
//!          |
//!          +--------------------+
//!          |                    |
//!          v                    v
//!      observations          result
//!          |                    |
//!          +---------+----------+
//!                    |
//!                    v
//!             verification
//!                    |
//!                    v
//!          history::execution
//!
//! This module records what happened during an execution.
//!
//! It does NOT:
//! - execute quantum programs;
//! - perform routing;
//! - perform scheduling;
//! - perform optimization;
//! - implement QEC;
//! - implement error mitigation;
//! - diagnose faults;
//! - choose recovery strategies;
//! - authorize recovery;
//! - access hardware;
//! - access a provider;
//! - read the system clock;
//! - generate identifiers implicitly;
//! - serialize data into a wire format;
//! - persist data to storage;
//! - define a competing quantum-resource identity;
//! - define a competing fault ontology.
//!
//! Those responsibilities belong to the appropriate surrounding subsystem.
//!
//! # Core architectural rule
//!
//! History is an immutable record of observed execution facts.
//!
//! ```text
//! execution
//!     |
//!     +--> outcome
//!     +--> verification state
//!     +--> incident association
//!     +--> execution attempt
//!     +--> timestamps supplied by caller
//!     +--> provenance supplied by caller
//!     +--> stable failure code when applicable
//! ```
//!
//! History MUST NOT silently reinterpret an execution.
//!
//! For example:
//!
//! ```text
//! execution failed
//!         |
//!         v
//! history records FAILED
//!         |
//!         v
//! diagnosis may later determine why
//!         |
//!         v
//! planner may later decide what to do
//! ```
//!
//! History therefore remains useful for deterministic replay, audit,
//! benchmarking, statistics, recovery analysis and future learning.
//!
//! # Write once, scale everywhere
//!
//! This file deliberately contains no:
//!
//! - maximum qubit count;
//! - maximum operation count;
//! - maximum execution count;
//! - maximum number of attempts;
//! - maximum number of incidents;
//! - maximum backend count;
//! - provider identifiers;
//! - hardware-specific resource indexes;
//! - fixed topology;
//! - fixed retry count;
//! - fixed timeout;
//! - fixed result size.
//!
//! The only bounded scalar values in this file are identifiers/counters whose
//! representation is part of the data contract. They are NOT machine-size
//! limits.
//!
//! The actual amount of history that can be retained is determined by the
//! storage, memory, persistence policy and available resources of the caller.
//!
//! "Infinity" therefore means that this semantic model introduces no artificial
//! finite quantum-machine ceiling.
//!
//! # Canonical quantum identity rule
//!
//! This file intentionally does NOT import:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! That is deliberate.
//!
//! An execution history record should not reconstruct a second physical/logical
//! resource model. If affected quantum resources are required, they must be
//! supplied by the canonical execution/provenance layer using the repository's
//! canonical:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! identities.
//!
//! This prevents:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! ResilienceQubitId
//! RecoveryQubitId
//! ```
//!
//! from becoming competing identity systems.
//!
//! # Execution identity
//!
//! The repository currently contains execution-identity definitions in several
//! recovery components. History deliberately does not duplicate those concrete
//! types.
//!
//! Instead, this module stores an opaque stable execution identity as `Arc<str>`.
//!
//! The runtime/recovery layer can convert its canonical execution identity into
//! this representation at the history boundary.
//!
//! The string has NO provider-specific semantics here.
//!
//! It is simply the stable identity of the execution being recorded.
//!
//! # Determinism
//!
//! This module performs no implicit observation of the environment.
//!
//! It does not call:
//!
//! - `SystemTime::now()`;
//! - random generators;
//! - UUID generators;
//! - environment variables;
//! - filesystem APIs;
//! - network APIs;
//! - provider APIs;
//! - global mutable state.
//!
//! All time, identity and provenance values are explicitly supplied by the
//! caller.
//!
//! Therefore constructing an identical record from identical inputs produces
//! an identical value.
//!
//! # Historical immutability
//!
//! Once an `ExecutionHistoryEntry` has been constructed, it exposes no mutable
//! access to its fields.
//!
//! If an execution later changes state, a new historical entry must be created.
//!
//! This is important because history can be consumed concurrently by:
//!
//! - statistics;
//! - diagnosis;
//! - planning;
//! - learning;
//! - observability;
//! - audit;
//! - deterministic replay.
//!
//! # Verification rule
//!
//! Execution completion and result verification are different facts.
//!
//! ```text
//! ExecutionStatus
//!     = what happened to execution
//!
//! VerificationStatus
//!     = what the verification subsystem established
//! ```
//!
//! A successful execution MUST NOT automatically be interpreted as a verified
//! correct quantum result.
//!
//! This distinction is especially important for resilience because availability
//! alone is never sufficient to accept a recovered result.
//!
//! # Provenance
//!
//! The record can carry opaque stable digests for:
//!
//! - program;
//! - canonical IR;
//! - target/capability snapshot;
//! - resilience policy;
//! - strategy implementation set;
//! - calibration snapshot.
//!
//! These are identifiers/digests only.
//!
//! This module does not prescribe a cryptographic algorithm or wire format.
//! The verification/provenance and serialization layers own those contracts.
//!
//! # Error integration
//!
//! Failure records use the canonical:
//!
//! ```text
//! quantum::resilience::errors::codes::ResilienceErrorCode
//! ```
//!
//! This file does not define another resilience error-code enumeration.
//!
//! # History vs telemetry
//!
//! Telemetry may be high-volume and transient.
//!
//! History is a durable semantic record.
//!
//! Therefore this type should record execution-level facts rather than every
//! low-level debug event.
//!
//! # Rust contract
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! `unsafe` is explicitly forbidden.
//!
//! # Integration contract
//!
//! ```text
//! quantum::resilience::recovery
//!              |
//!              | execution outcome
//!              v
//! quantum::resilience::history::execution
//!              |
//!              +--> history::statistics
//!              +--> history::recovery
//!              +--> learning
//!              +--> observability
//!              +--> deterministic replay
//!              +--> persistence / serialization
//! ```
//!
//! The record is intentionally independent of concrete storage.
//!
//! A storage implementation can later append, stream, index, shard, archive,
//! compress or replicate these records without modifying this semantic type.
//!
//! # No hard-coded machine assumptions
//!
//! An execution can represent:
//!
//! - one physical qubit;
//! - a small QPU;
//! - a large QPU;
//! - a fault-tolerant logical machine;
//! - a simulator;
//! - an emulator;
//! - a heterogeneous fleet;
//! - a distributed quantum execution.
//!
//! Nothing in this file assumes how many qubits or operations were involved.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. execution identity is stable and opaque;
//! 2. execution outcome is explicitly represented;
//! 3. verification state is independent from execution status;
//! 4. incidents can be associated without duplicating incident semantics;
//! 5. failure identity uses the canonical resilience error-code registry;
//! 6. timestamps are supplied explicitly;
//! 7. timestamps do not invoke an implicit clock;
//! 8. provenance is explicit;
//! 9. no provider-specific execution type is required;
//! 10. no physical qubit identity is duplicated;
//! 11. no fixed quantum-machine limit exists;
//! 12. records are immutable after construction;
//! 13. validation does not require hardware access;
//! 14. the record can be serialized later without changing its semantic model;
//! 15. deterministic replay can reconstruct the same historical value.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use std::sync::Arc;

use super::super::errors::codes::ResilienceErrorCode;
use super::super::model::incident::IncidentId;

// =============================================================================
// Stable execution sequence
// =============================================================================

/// Stable position of a historical execution record within the history stream.
///
/// This is an ordering value, not a machine-size limit.
///
/// The history owner supplies it. This module never allocates sequence numbers
/// implicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionSequence(u64);

impl ExecutionSequence {
    /// Creates a sequence value from a caller-owned stable value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying sequence value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ExecutionSequence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// =============================================================================
// Attempt identity
// =============================================================================

/// Identifies one attempt of an execution.
///
/// Attempt numbers are intentionally caller-owned.
///
/// There is no hard-coded retry limit.
///
/// For example:
///
/// ```text
/// execution E
///     attempt 0
///     attempt 1
///     attempt 2
///     ...
/// ```
///
/// The policy layer decides how many attempts are permitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AttemptNumber(u64);

impl AttemptNumber {
    /// Creates an attempt number.
    ///
    /// Zero is intentionally permitted so callers may use either zero-based
    /// or one-based conventions according to their execution contract.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying attempt number.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for AttemptNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// =============================================================================
// Explicit execution timestamp
// =============================================================================

/// An explicitly supplied execution timestamp.
///
/// The timestamp uses Unix-epoch seconds plus nanoseconds.
///
/// This is a VALUE type. It does not read the system clock.
///
/// Negative seconds are allowed so that the representation remains valid for
/// dates before the Unix epoch.
///
/// `nanoseconds` must be in:
///
/// ```text
/// 0 .. 1_000_000_000
/// ```
///
/// There is deliberately no timestamp retention policy here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionTimestamp {
    seconds: i64,
    nanoseconds: u32,
}

impl ExecutionTimestamp {
    /// Number of nanoseconds in one second.
    pub const NANOS_PER_SECOND: u32 = 1_000_000_000;

    /// Constructs a timestamp.
    ///
    /// Returns `None` if the nanosecond component is outside the canonical
    /// range.
    #[must_use]
    pub const fn new(seconds: i64, nanoseconds: u32) -> Option<Self> {
        if nanoseconds < Self::NANOS_PER_SECOND {
            Some(Self {
                seconds,
                nanoseconds,
            })
        } else {
            None
        }
    }

    /// Returns the signed Unix-epoch seconds component.
    #[must_use]
    pub const fn seconds(self) -> i64 {
        self.seconds
    }

    /// Returns the nanosecond component.
    #[must_use]
    pub const fn nanoseconds(self) -> u32 {
        self.nanoseconds
    }

    /// Returns whether this timestamp is structurally valid.
    ///
    /// Instances constructed by [`Self::new`] are always valid.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.nanoseconds < Self::NANOS_PER_SECOND
    }
}

impl fmt::Display for ExecutionTimestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{:09}",
            self.seconds, self.nanoseconds
        )
    }
}

// =============================================================================
// Execution status
// =============================================================================

/// Historical execution outcome.
///
/// This type records what happened to the execution itself.
///
/// It does NOT decide whether the resulting quantum computation is semantically
/// correct. That is represented separately by [`VerificationStatus`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExecutionStatus {
    /// Execution completed normally according to the execution layer.
    Succeeded,

    /// Execution terminated because of an execution failure.
    Failed,

    /// Execution was explicitly cancelled.
    Cancelled,

    /// Execution exceeded an applicable execution deadline.
    TimedOut,

    /// Execution was explicitly aborted.
    Aborted,

    /// Execution was rejected before successful execution.
    Rejected,

    /// Execution produced only a partial execution outcome.
    ///
    /// This is important for dynamic/distributed systems where some work may
    /// have completed before a failure boundary.
    PartiallyCompleted,

    /// The execution layer cannot classify the outcome.
    Unknown,
}

impl ExecutionStatus {
    /// Returns whether the execution reached a normal completion state.
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Succeeded)
    }

    /// Returns whether the execution represents an unsuccessful outcome.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(
            self,
            Self::Failed
                | Self::Cancelled
                | Self::TimedOut
                | Self::Aborted
                | Self::Rejected
                | Self::PartiallyCompleted
        )
    }

    /// Returns whether the execution outcome is explicitly terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

// =============================================================================
// Verification status
// =============================================================================

/// Historical state of result verification.
///
/// Execution status and verification status are intentionally independent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerificationStatus {
    /// Verification was not performed.
    NotPerformed,

    /// Verification completed successfully according to the verification
    /// contract.
    Passed,

    /// Verification determined that the result failed the applicable
    /// verification requirements.
    Failed,

    /// Verification was attempted but could not establish a definitive result.
    Inconclusive,
}

impl VerificationStatus {
    /// Returns whether verification passed.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Passed)
    }

    /// Returns whether verification produced a definitive negative result.
    #[must_use]
    pub const fn is_rejected(self) -> bool {
        matches!(self, Self::Failed)
    }

    /// Returns whether no definitive verification result exists.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(
            self,
            Self::NotPerformed | Self::Inconclusive
        )
    }
}

// =============================================================================
// Execution failure
// =============================================================================

/// Historical execution failure information.
///
/// The stable failure identity comes from the canonical resilience error-code
/// registry.
///
/// Human-readable detail is optional and opaque to this module.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionFailure {
    code: ResilienceErrorCode,
    detail: Option<Arc<str>>,
}

impl ExecutionFailure {
    /// Creates a failure with only its stable machine-readable error code.
    #[must_use]
    pub const fn new(code: ResilienceErrorCode) -> Self {
        Self {
            code,
            detail: None,
        }
    }

    /// Creates a failure with stable error identity and caller-supplied detail.
    ///
    /// The detail is diagnostic data only. It must not be treated as an
    /// authorization token, recovery command or executable instruction.
    #[must_use]
    pub fn with_detail<S>(code: ResilienceErrorCode, detail: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            code,
            detail: Some(Arc::<str>::from(detail.as_ref())),
        }
    }

    /// Returns the canonical resilience error code.
    #[must_use]
    pub const fn code(&self) -> ResilienceErrorCode {
        self.code
    }

    /// Returns optional diagnostic detail.
    #[must_use]
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    /// Returns whether diagnostic detail is present.
    #[must_use]
    pub fn has_detail(&self) -> bool {
        self.detail.is_some()
    }
}

// =============================================================================
// Execution provenance
// =============================================================================

/// Opaque execution provenance references.
///
/// Each value is a caller-supplied stable identifier/digest.
///
/// This module does not prescribe:
///
/// - SHA-256;
/// - SHA-3;
/// - BLAKE;
/// - Merkle trees;
/// - signatures;
/// - certificates.
///
/// Those belong to the repository's verification/security/serialization
/// contracts.
///
/// Keeping these values opaque means history can evolve without coupling the
/// execution record to one cryptographic implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ExecutionProvenance {
    program_digest: Option<Arc<str>>,
    ir_digest: Option<Arc<str>>,
    target_digest: Option<Arc<str>>,
    policy_digest: Option<Arc<str>>,
    strategy_digest: Option<Arc<str>>,
    calibration_digest: Option<Arc<str>>,
}

impl ExecutionProvenance {
    /// Creates empty provenance.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            program_digest: None,
            ir_digest: None,
            target_digest: None,
            policy_digest: None,
            strategy_digest: None,
            calibration_digest: None,
        }
    }

    /// Sets the canonical program identity/digest.
    #[must_use]
    pub fn with_program_digest<S>(mut self, value: S) -> Self
    where
        S: AsRef<str>,
    {
        self.program_digest = Some(Arc::<str>::from(value.as_ref()));
        self
    }

    /// Sets the canonical IR identity/digest.
    #[must_use]
    pub fn with_ir_digest<S>(mut self, value: S) -> Self
    where
        S: AsRef<str>,
    {
        self.ir_digest = Some(Arc::<str>::from(value.as_ref()));
        self
    }

    /// Sets the target/capability snapshot identity.
    #[must_use]
    pub fn with_target_digest<S>(mut self, value: S) -> Self
    where
        S: AsRef<str>,
    {
        self.target_digest = Some(Arc::<str>::from(value.as_ref()));
        self
    }

    /// Sets the resilience-policy identity.
    #[must_use]
    pub fn with_policy_digest<S>(mut self, value: S) -> Self
    where
        S: AsRef<str>,
    {
        self.policy_digest = Some(Arc::<str>::from(value.as_ref()));
        self
    }

    /// Sets the strategy/implementation identity.
    #[must_use]
    pub fn with_strategy_digest<S>(mut self, value: S) -> Self
    where
        S: AsRef<str>,
    {
        self.strategy_digest = Some(Arc::<str>::from(value.as_ref()));
        self
    }

    /// Sets the calibration snapshot identity.
    #[must_use]
    pub fn with_calibration_digest<S>(mut self, value: S) -> Self
    where
        S: AsRef<str>,
    {
        self.calibration_digest = Some(Arc::<str>::from(value.as_ref()));
        self
    }

    /// Returns the program identity/digest.
    #[must_use]
    pub fn program_digest(&self) -> Option<&str> {
        self.program_digest.as_deref()
    }

    /// Returns the canonical IR identity/digest.
    #[must_use]
    pub fn ir_digest(&self) -> Option<&str> {
        self.ir_digest.as_deref()
    }

    /// Returns the target/capability snapshot identity.
    #[must_use]
    pub fn target_digest(&self) -> Option<&str> {
        self.target_digest.as_deref()
    }

    /// Returns the resilience-policy identity.
    #[must_use]
    pub fn policy_digest(&self) -> Option<&str> {
        self.policy_digest.as_deref()
    }

    /// Returns the strategy/implementation identity.
    #[must_use]
    pub fn strategy_digest(&self) -> Option<&str> {
        self.strategy_digest.as_deref()
    }

    /// Returns the calibration snapshot identity.
    #[must_use]
    pub fn calibration_digest(&self) -> Option<&str> {
        self.calibration_digest.as_deref()
    }

    /// Returns whether no provenance fields are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.program_digest.is_none()
            && self.ir_digest.is_none()
            && self.target_digest.is_none()
            && self.policy_digest.is_none()
            && self.strategy_digest.is_none()
            && self.calibration_digest.is_none()
    }
}

// =============================================================================
// Execution history entry
// =============================================================================

/// Immutable historical record of one execution attempt.
///
/// This is the primary type exported by this file.
///
/// An entry represents one observed historical fact:
///
/// ```text
/// execution identity
///     +
/// sequence
///     +
/// attempt
///     +
/// outcome
///     +
/// verification state
///     +
/// optional incident
///     +
/// optional timing
///     +
/// optional failure
///     +
/// provenance
/// ```
///
/// It is deliberately independent of storage.
///
/// A history database may store millions, billions or more entries as resources
/// permit. No artificial entry-count limit is encoded here.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionHistoryEntry {
    execution_id: Arc<str>,
    sequence: ExecutionSequence,
    attempt: AttemptNumber,
    incident_id: Option<IncidentId>,
    started_at: Option<ExecutionTimestamp>,
    finished_at: Option<ExecutionTimestamp>,
    status: ExecutionStatus,
    verification: VerificationStatus,
    failure: Option<ExecutionFailure>,
    provenance: ExecutionProvenance,
}

impl ExecutionHistoryEntry {
    /// Creates an execution-history entry.
    ///
    /// All values are supplied explicitly. No clock, randomness or global state
    /// is consulted.
    #[must_use]
    pub fn new<S>(
        execution_id: S,
        sequence: ExecutionSequence,
        attempt: AttemptNumber,
        status: ExecutionStatus,
    ) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            execution_id: Arc::<str>::from(execution_id.as_ref()),
            sequence,
            attempt,
            incident_id: None,
            started_at: None,
            finished_at: None,
            status,
            verification: VerificationStatus::NotPerformed,
            failure: None,
            provenance: ExecutionProvenance::empty(),
        }
    }

    /// Associates the execution with a resilience incident.
    ///
    /// The incident itself remains owned by
    /// `quantum::resilience::model::incident`.
    #[must_use]
    pub const fn with_incident(mut self, incident_id: IncidentId) -> Self {
        self.incident_id = Some(incident_id);
        self
    }

    /// Records an explicit start timestamp.
    #[must_use]
    pub const fn with_started_at(
        mut self,
        timestamp: ExecutionTimestamp,
    ) -> Self {
        self.started_at = Some(timestamp);
        self
    }

    /// Records an explicit completion timestamp.
    #[must_use]
    pub const fn with_finished_at(
        mut self,
        timestamp: ExecutionTimestamp,
    ) -> Self {
        self.finished_at = Some(timestamp);
        self
    }

    /// Records verification status.
    #[must_use]
    pub const fn with_verification(
        mut self,
        verification: VerificationStatus,
    ) -> Self {
        self.verification = verification;
        self
    }

    /// Associates a canonical resilience error code with the execution.
    #[must_use]
    pub const fn with_failure(
        mut self,
        failure: ExecutionFailure,
    ) -> Self {
        self.failure = Some(failure);
        self
    }

    /// Associates execution provenance.
    #[must_use]
    pub fn with_provenance(
        mut self,
        provenance: ExecutionProvenance,
    ) -> Self {
        self.provenance = provenance;
        self
    }

    /// Returns the stable execution identity.
    ///
    /// The returned identity is opaque to this module.
    #[must_use]
    pub fn execution_id(&self) -> &str {
        &self.execution_id
    }

    /// Returns the history sequence.
    #[must_use]
    pub const fn sequence(&self) -> ExecutionSequence {
        self.sequence
    }

    /// Returns the attempt number.
    #[must_use]
    pub const fn attempt(&self) -> AttemptNumber {
        self.attempt
    }

    /// Returns the associated incident, if one exists.
    #[must_use]
    pub const fn incident_id(&self) -> Option<IncidentId> {
        self.incident_id
    }

    /// Returns the supplied start timestamp.
    #[must_use]
    pub const fn started_at(&self) -> Option<ExecutionTimestamp> {
        self.started_at
    }

    /// Returns the supplied completion timestamp.
    #[must_use]
    pub const fn finished_at(&self) -> Option<ExecutionTimestamp> {
        self.finished_at
    }

    /// Returns the historical execution outcome.
    #[must_use]
    pub const fn status(&self) -> ExecutionStatus {
        self.status
    }

    /// Returns the historical verification state.
    #[must_use]
    pub const fn verification(&self) -> VerificationStatus {
        self.verification
    }

    /// Returns failure information, if recorded.
    #[must_use]
    pub fn failure(&self) -> Option<&ExecutionFailure> {
        self.failure.as_ref()
    }

    /// Returns execution provenance.
    #[must_use]
    pub const fn provenance(&self) -> &ExecutionProvenance {
        &self.provenance
    }

    /// Returns whether the execution identity is non-empty.
    #[must_use]
    pub fn has_execution_id(&self) -> bool {
        !self.execution_id.is_empty()
    }

    /// Returns whether an incident is associated with this execution.
    #[must_use]
    pub const fn has_incident(&self) -> bool {
        self.incident_id.is_some()
    }

    /// Returns whether a failure was recorded.
    #[must_use]
    pub const fn has_failure(&self) -> bool {
        self.failure.is_some()
    }

    /// Returns whether the result was verified successfully.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.verification.is_verified()
    }

    /// Returns whether the execution itself succeeded.
    #[must_use]
    pub const fn succeeded(&self) -> bool {
        self.status.is_success()
    }

    /// Returns whether this entry represents a successful AND verified result.
    ///
    /// This is deliberately stricter than `succeeded()`.
    ///
    /// A successful execution with no verification is not considered an
    /// accepted historical result.
    #[must_use]
    pub const fn succeeded_and_verified(&self) -> bool {
        self.status.is_success() && self.verification.is_verified()
    }

    /// Returns whether this record contains a complete start/end interval.
    #[must_use]
    pub const fn has_complete_timing(&self) -> bool {
        self.started_at.is_some() && self.finished_at.is_some()
    }

    /// Calculates execution duration in nanoseconds when both timestamps are
    /// present and the completion timestamp is not earlier than the start.
    ///
    /// `u128` is used for the returned duration so this semantic helper does
    /// not impose a practical execution-duration ceiling caused by `u64`
    /// arithmetic.
    ///
    /// The method performs no clock access.
    #[must_use]
    pub fn duration_nanoseconds(&self) -> Option<u128> {
        let start = self.started_at?;
        let finish = self.finished_at?;

        if !start.is_valid() || !finish.is_valid() {
            return None;
        }

        let seconds_delta =
            i128::from(finish.seconds()) - i128::from(start.seconds());

        let nanos_delta = i128::from(finish.nanoseconds())
            - i128::from(start.nanoseconds());

        let total = seconds_delta
            .checked_mul(i128::from(ExecutionTimestamp::NANOS_PER_SECOND))?
            .checked_add(nanos_delta)?;

        if total < 0 {
            return None;
        }

        u128::try_from(total).ok()
    }

    /// Returns whether the historical record is structurally valid.
    ///
    /// This method intentionally performs only value-local validation.
    ///
    /// It does not ask:
    ///
    /// - whether the hardware existed;
    /// - whether the incident was real;
    /// - whether a policy allowed execution;
    /// - whether the target supported the program;
    /// - whether the result is physically correct;
    /// - whether the failure was correctly diagnosed.
    ///
    /// Those questions belong to other layers.
    #[must_use]
    pub fn is_structurally_valid(&self) -> bool {
        if self.execution_id.is_empty() {
            return false;
        }

        if let Some(timestamp) = self.started_at {
            if !timestamp.is_valid() {
                return false;
            }
        }

        if let Some(timestamp) = self.finished_at {
            if !timestamp.is_valid() {
                return false;
            }
        }

        if self.has_complete_timing() && self.duration_nanoseconds().is_none() {
            return false;
        }

        if let Some(failure) = &self.failure {
            // The error code is an enum and therefore structurally valid by
            // construction. The explicit match is intentionally avoided so
            // adding future error-code variants cannot invalidate history.
            let _ = failure.code();
        }

        true
    }

    /// Returns whether the execution has a terminal status and therefore can
    /// normally be treated as a completed historical attempt.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        self.status.is_terminal()
    }

    /// Returns whether the entry is still missing verification.
    #[must_use]
    pub const fn requires_verification(&self) -> bool {
        matches!(
            self.verification,
            VerificationStatus::NotPerformed
                | VerificationStatus::Inconclusive
        )
    }

    /// Returns a new entry with a different verification state.
    ///
    /// The original historical entry is unchanged.
    ///
    /// This is useful when the architecture intentionally represents
    /// execution and post-execution verification as separate immutable
    /// revisions.
    #[must_use]
    pub const fn with_verification_state(
        &self,
        verification: VerificationStatus,
    ) -> Self {
        Self {
            execution_id: self.execution_id.clone(),
            sequence: self.sequence,
            attempt: self.attempt,
            incident_id: self.incident_id,
            started_at: self.started_at,
            finished_at: self.finished_at,
            status: self.status,
            verification,
            failure: self.failure.clone(),
            provenance: self.provenance.clone(),
        }
    }

    /// Returns a new entry associated with another incident.
    ///
    /// The execution identity and outcome remain unchanged.
    #[must_use]
    pub const fn with_incident_id(
        &self,
        incident_id: Option<IncidentId>,
    ) -> Self {
        Self {
            execution_id: self.execution_id.clone(),
            sequence: self.sequence,
            attempt: self.attempt,
            incident_id,
            started_at: self.started_at,
            finished_at: self.finished_at,
            status: self.status,
            verification: self.verification,
            failure: self.failure.clone(),
            provenance: self.provenance.clone(),
        }
    }

    /// Returns all semantic fields as owned components.
    ///
    /// This is useful for persistence adapters and serialization layers.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        Arc<str>,
        ExecutionSequence,
        AttemptNumber,
        Option<IncidentId>,
        Option<ExecutionTimestamp>,
        Option<ExecutionTimestamp>,
        ExecutionStatus,
        VerificationStatus,
        Option<ExecutionFailure>,
        ExecutionProvenance,
    ) {
        (
            self.execution_id,
            self.sequence,
            self.attempt,
            self.incident_id,
            self.started_at,
            self.finished_at,
            self.status,
            self.verification,
            self.failure,
            self.provenance,
        )
    }
}

// =============================================================================
// Convenience conversions
// =============================================================================

impl<S> From<(S, ExecutionSequence, AttemptNumber, ExecutionStatus)>
    for ExecutionHistoryEntry
where
    S: AsRef<str>,
{
    fn from(
        value: (S, ExecutionSequence, AttemptNumber, ExecutionStatus),
    ) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_rejects_invalid_nanoseconds() {
        assert!(
            ExecutionTimestamp::new(
                0,
                ExecutionTimestamp::NANOS_PER_SECOND
            )
            .is_none()
        );
    }

    #[test]
    fn timestamp_accepts_valid_nanoseconds() {
        let timestamp =
            ExecutionTimestamp::new(10, 123_456_789).expect("valid timestamp");

        assert_eq!(timestamp.seconds(), 10);
        assert_eq!(timestamp.nanoseconds(), 123_456_789);
        assert!(timestamp.is_valid());
    }

    #[test]
    fn execution_record_is_deterministic() {
        let timestamp =
            ExecutionTimestamp::new(100, 0).expect("valid timestamp");

        let first = ExecutionHistoryEntry::new(
            "execution-a",
            ExecutionSequence::new(7),
            AttemptNumber::new(0),
            ExecutionStatus::Succeeded,
        )
        .with_started_at(timestamp)
        .with_finished_at(
            ExecutionTimestamp::new(101, 500_000_000)
                .expect("valid timestamp"),
        );

        let second = ExecutionHistoryEntry::new(
            "execution-a",
            ExecutionSequence::new(7),
            AttemptNumber::new(0),
            ExecutionStatus::Succeeded,
        )
        .with_started_at(timestamp)
        .with_finished_at(
            ExecutionTimestamp::new(101, 500_000_000)
                .expect("valid timestamp"),
        );

        assert_eq!(first, second);
    }

    #[test]
    fn duration_is_calculated_without_clock_access() {
        let record = ExecutionHistoryEntry::new(
            "execution-a",
            ExecutionSequence::new(0),
            AttemptNumber::new(0),
            ExecutionStatus::Succeeded,
        )
        .with_started_at(
            ExecutionTimestamp::new(10, 100).expect("valid timestamp"),
        )
        .with_finished_at(
            ExecutionTimestamp::new(12, 200).expect("valid timestamp"),
        );

        assert_eq!(
            record.duration_nanoseconds(),
            Some(2_000_000_100)
        );
    }

    #[test]
    fn backwards_timing_is_invalid() {
        let record = ExecutionHistoryEntry::new(
            "execution-a",
            ExecutionSequence::new(0),
            AttemptNumber::new(0),
            ExecutionStatus::Succeeded,
        )
        .with_started_at(
            ExecutionTimestamp::new(20, 0).expect("valid timestamp"),
        )
        .with_finished_at(
            ExecutionTimestamp::new(19, 0).expect("valid timestamp"),
        );

        assert!(!record.is_structurally_valid());
        assert_eq!(record.duration_nanoseconds(), None);
    }

    #[test]
    fn empty_execution_identity_is_invalid() {
        let record = ExecutionHistoryEntry::new(
            "",
            ExecutionSequence::new(0),
            AttemptNumber::new(0),
            ExecutionStatus::Unknown,
        );

        assert!(!record.is_structurally_valid());
    }

    #[test]
    fn successful_execution_is_not_automatically_verified() {
        let record = ExecutionHistoryEntry::new(
            "execution-a",
            ExecutionSequence::new(0),
            AttemptNumber::new(0),
            ExecutionStatus::Succeeded,
        );

        assert!(record.succeeded());
        assert!(!record.is_verified());
        assert!(!record.succeeded_and_verified());
        assert!(record.requires_verification());
    }

    #[test]
    fn successful_verified_execution_is_distinguished() {
        let record = ExecutionHistoryEntry::new(
            "execution-a",
            ExecutionSequence::new(0),
            AttemptNumber::new(0),
            ExecutionStatus::Succeeded,
        )
        .with_verification(VerificationStatus::Passed);

        assert!(record.succeeded());
        assert!(record.is_verified());
        assert!(record.succeeded_and_verified());
    }

    #[test]
    fn incident_identity_is_preserved_without_copying_faults() {
        let incident_id =
            IncidentId::new(42).expect("non-zero incident identifier");

        let record = ExecutionHistoryEntry::new(
            "execution-a",
            ExecutionSequence::new(1),
            AttemptNumber::new(2),
            ExecutionStatus::Failed,
        )
        .with_incident(incident_id);

        assert_eq!(record.incident_id(), Some(incident_id));
        assert!(record.has_incident());
    }

    #[test]
    fn provenance_is_value_based() {
        let provenance = ExecutionProvenance::empty()
            .with_program_digest("program-1")
            .with_ir_digest("ir-1")
            .with_target_digest("target-1")
            .with_policy_digest("policy-1")
            .with_strategy_digest("strategy-1")
            .with_calibration_digest("calibration-1");

        assert_eq!(provenance.program_digest(), Some("program-1"));
        assert_eq!(provenance.ir_digest(), Some("ir-1"));
        assert_eq!(provenance.target_digest(), Some("target-1"));
        assert_eq!(provenance.policy_digest(), Some("policy-1"));
        assert_eq!(provenance.strategy_digest(), Some("strategy-1"));
        assert_eq!(provenance.calibration_digest(), Some("calibration-1"));
        assert!(!provenance.is_empty());
    }

    #[test]
    fn failure_uses_canonical_error_code() {
        let code = ResilienceErrorCode::InvalidArgument;
        let failure = ExecutionFailure::with_detail(
            code,
            "execution request was invalid",
        );

        assert_eq!(failure.code(), code);
        assert_eq!(
            failure.detail(),
            Some("execution request was invalid")
        );
    }

    #[test]
    fn immutable_revision_does_not_modify_original() {
        let original = ExecutionHistoryEntry::new(
            "execution-a",
            ExecutionSequence::new(0),
            AttemptNumber::new(0),
            ExecutionStatus::Succeeded,
        );

        let verified =
            original.with_verification_state(VerificationStatus::Passed);

        assert_eq!(
            original.verification(),
            VerificationStatus::NotPerformed
        );
        assert_eq!(
            verified.verification(),
            VerificationStatus::Passed
        );
    }

    #[test]
    fn tuple_conversion_is_supported() {
        let record: ExecutionHistoryEntry = (
            "execution-a",
            ExecutionSequence::new(0),
            AttemptNumber::new(0),
            ExecutionStatus::Succeeded,
        )
            .into();

        assert_eq!(record.execution_id(), "execution-a");
        assert!(record.succeeded());
    }
}