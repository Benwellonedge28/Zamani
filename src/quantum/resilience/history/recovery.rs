//! Zamani Quantum Resilience — Recovery History
//!
//! Path:
//!     src/quantum/resilience/history/recovery.rs
//!
//! Purpose:
//!     Defines the immutable, deterministic historical representation of
//!     recovery operations performed by the quantum resilience subsystem.
//!
//! ============================================================================
//! ARCHITECTURAL ROLE
//! ============================================================================
//!
//! Detection
//!     |
//!     v
//! Diagnosis
//!     |
//!     v
//! Policy
//!     |
//!     v
//! Planning
//!     |
//!     v
//! Recovery execution
//!     |
//!     +---------------------------+
//!     |                           |
//!     v                           v
//! execution outcome          verification outcome
//!     |                           |
//!     +-------------+-------------+
//!                   |
//!                   v
//!          history::recovery
//!                   |
//!        +----------+----------+
//!        |          |          |
//!        v          v          v
//!    statistics   learning   audit/replay
//!
//! This module records what happened during a recovery operation.
//!
//! It does NOT:
//!
//! - execute recovery;
//! - select recovery plans;
//! - diagnose incidents;
//! - perform routing;
//! - perform scheduling;
//! - compile;
//! - optimize;
//! - implement QEC;
//! - implement mitigation;
//! - access hardware;
//! - access providers;
//! - authorize recovery;
//! - persist records;
//! - serialize records to a wire format;
//! - read the system clock;
//! - generate random identifiers;
//! - define a competing quantum fault ontology;
//! - define a competing quantum-resource identity.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! ============================================================================
//! CORE PRINCIPLE
//! ============================================================================
//!
//! History is an immutable record of facts.
//!
//! It must never silently convert:
//!
//!     attempted recovery
//!
//! into:
//!
//!     successful recovery
//!
//! merely because an action completed.
//!
//! Recovery execution, recovery outcome and verification are separate facts.
//!
//! ```text
//! Recovery execution
//!        |
//!        +--> action outcome
//!        |
//!        +--> recovery disposition
//!        |
//!        +--> verification outcome
//!        |
//!        +--> failure/error code
//!        |
//!        +--> provenance
//!        |
//!        v
//! Historical record
//! ```
//!
//! In particular:
//!
//!     availability != correctness
//!
//! A recovery record therefore MUST be capable of representing:
//!
//! - successful execution but failed verification;
//! - successful execution with accepted degradation;
//! - stale-plan termination;
//! - replanning;
//! - escalation;
//! - rejection;
//! - partial recovery;
//! - action-level failures;
//! - verification inconclusive;
//! - cancellation;
//! - unknown/undetermined outcome.
//!
//! ============================================================================
//! WRITE ONCE / SCALE EVERYWHERE
//! ============================================================================
//!
//! This file contains no quantum-machine-size assumptions.
//!
//! There is deliberately no:
//!
//! - MAX_QUBITS;
//! - MAX_PHYSICAL_QUBITS;
//! - MAX_ACTIONS;
//! - MAX_RECOVERY_ATTEMPTS;
//! - MAX_INCIDENTS;
//! - MAX_BACKENDS;
//! - MAX_DEVICES;
//! - fixed retry count;
//! - fixed timeout;
//! - fixed topology;
//! - provider-specific branch;
//! - fixed recovery depth.
//!
//! A recovery history entry may describe recovery of:
//!
//! - one qubit;
//! - a small QPU;
//! - a large QPU;
//! - a fault-tolerant logical machine;
//! - a simulator;
//! - an emulator;
//! - a heterogeneous fleet;
//! - a distributed quantum execution.
//!
//! The only limits are those imposed by the representation and the resources
//! available to the caller/storage system.
//!
//! "Infinite scale" therefore means:
//!
//!     no artificial quantum-machine-size ceiling in the semantic model.
//!
//! It does NOT mean infinite memory, infinite storage, or infinite execution
//! capacity.
//!
//! ============================================================================
//! CANONICAL QUANTUM IDENTITY
//! ============================================================================
//!
//! This module intentionally does not import QubitId or PhysicalQubitId.
//!
//! That is correct and intentional.
//!
//! Recovery history should record the recovery operation itself, not reconstruct
//! a second resource graph.
//!
//! When affected quantum resources are eventually required in recovery history,
//! they MUST be represented using the canonical identities owned by:
//!
//!     crate::quantum::ir::qubit
//!
//! Specifically, depending on the canonical IR contract:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! The history layer must never introduce:
//!
//!     RecoveryQubitId
//!     HistoryQubitId
//!     ResilienceQubitId
//!
//! or any other competing identity.
//!
//! ============================================================================
//! INCIDENT IDENTITY
//! ============================================================================
//!
//! An incident is owned by:
//!
//!     quantum::resilience::model::incident
//!
//! Recovery history therefore records the stable incident identity rather than
//! copying the complete incident/fault model.
//!
//! This preserves the distinction:
//!
//!     Fault       = canonical observed fault
//!     Incident    = resilience grouping of faults
//!     Recovery    = response to an incident
//!     History     = immutable record of that response
//!
//! ============================================================================
//! PLAN IDENTITY
//! ============================================================================
//!
//! Recovery plans are owned by:
//!
//!     quantum::resilience::planning::plan
//!
//! This module deliberately records the stable plan identity and plan revision
//! as opaque values instead of embedding a RecoveryPlan.
//!
//! This avoids:
//!
//! - copying potentially large plans;
//! - coupling history to plan execution;
//! - creating ownership cycles;
//! - making historical records executable;
//! - making historical records mutable through shared plan state.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! Construction is deterministic.
//!
//! This file never:
//!
//! - calls SystemTime::now();
//! - reads environment variables;
//! - generates UUIDs;
//! - generates random numbers;
//! - accesses global mutable state;
//! - depends on HashMap iteration order;
//! - depends on thread scheduling;
//! - contacts hardware;
//! - contacts a provider.
//!
//! Every identity, timestamp, outcome and provenance value is supplied by the
//! caller.
//!
//! Given identical inputs, an identical historical value is produced.
//!
//! ============================================================================
//! IMMUTABILITY
//! ============================================================================
//!
//! Historical records are immutable after construction.
//!
//! If recovery state changes:
//!
//!     old record
//!         |
//!         +--> remains historically true
//!
//!     new state
//!         |
//!         +--> new historical record
//!
//! A history record must never be edited to rewrite history.
//!
//! ============================================================================
//! SECURITY
//! ============================================================================
//!
//! A recovery-history entry is DATA, not a capability.
//!
//! It must never contain:
//!
//! - credentials;
//! - API keys;
//! - passwords;
//! - private keys;
//! - bearer tokens;
//! - raw authorization headers;
//! - device pointers;
//! - memory addresses;
//! - executable callbacks.
//!
//! Action names and identifiers are descriptive data only.
//!
//! ============================================================================
//! PROVENANCE
//! ============================================================================
//!
//! The record supports caller-supplied opaque provenance references.
//!
//! Examples:
//!
//! - program digest;
//! - canonical IR digest;
//! - target snapshot;
//! - capability snapshot;
//! - policy snapshot;
//! - calibration snapshot;
//! - implementation-set digest;
//! - verification record.
//!
//! This module does not prescribe a cryptographic algorithm.
//!
//! Cryptographic integrity belongs to the verification/security/serialization
//! layers.
//!
//! ============================================================================
//! HISTORY VS TELEMETRY
//! ============================================================================
//!
//! Telemetry is generally high-volume and transient.
//!
//! History is a durable semantic record.
//!
//! Therefore this file records recovery-level facts rather than every low-level
//! event.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! recovery/recoverer.rs
//!     |
//!     | produces recovery execution facts
//!     v
//! history/recovery.rs
//!     |
//!     +--> history/statistics.rs
//!     +--> learning/*
//!     +--> audit/replay
//!     +--> serialization/*
//!     +--> persistence
//!
//! planning/plan.rs
//!     |
//!     | supplies plan identity/version
//!     v
//! history/recovery.rs
//!
//! model/incident.rs
//!     |
//!     | supplies IncidentId
//!     v
//! history/recovery.rs
//!
//! verification/*
//!     |
//!     | supplies verification outcome
//!     v
//! history/recovery.rs
//!
//! errors/codes.rs
//!     |
//!     | supplies canonical ResilienceErrorCode
//!     v
//! history/recovery.rs
//!
//! No recovery executor is embedded in this module.
//!
//! ============================================================================
//! RUST CONTRACT
//! ============================================================================
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//! - stable Rust
//! - no nightly features
//! - no unsafe
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use std::sync::Arc;

use super::super::errors::codes::ResilienceErrorCode;
use super::super::model::incident::IncidentId;

// ============================================================================
// Schema
// ============================================================================

/// Stable schema identifier for recovery history records.
pub const RECOVERY_HISTORY_SCHEMA_ID: &str =
    "zamani.quantum.resilience.history.recovery";

/// Current semantic schema version.
pub const RECOVERY_HISTORY_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Generic opaque identity
// ============================================================================

/// Stable opaque identity used by the history layer.
///
/// This type intentionally carries no provider or hardware semantics.
///
/// It can represent:
///
/// - recovery operation identity;
/// - execution identity;
/// - plan identity;
/// - provenance identity.
///
/// The meaning is determined by the field containing it, never by this type
/// itself.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HistoryId(Arc<str>);

impl HistoryId {
    /// Creates a non-empty history identity.
    ///
    /// No normalization or hidden generation occurs.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, RecoveryHistoryError> {
        let value = value.into();

        if value.is_empty() {
            return Err(RecoveryHistoryError::EmptyIdentity);
        }

        Ok(Self(value))
    }

    /// Returns the identity as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for HistoryId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// History sequence
// ============================================================================

/// Caller-supplied position in a recovery-history stream.
///
/// This is an ordering value.
///
/// It is NOT:
///
/// - a qubit count;
/// - a device count;
/// - a retry limit;
/// - a recovery depth limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecoveryHistorySequence(u64);

impl RecoveryHistorySequence {
    /// Creates a history sequence value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the sequence value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for RecoveryHistorySequence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// ============================================================================
// Attempt number
// ============================================================================

/// Caller-supplied recovery-attempt number.
///
/// There is no hard-coded retry limit.
///
/// Policy decides how many attempts are permitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecoveryAttemptNumber(u64);

impl RecoveryAttemptNumber {
    /// Creates an attempt number.
    ///
    /// Zero is valid so both zero-based and one-based external conventions can
    /// be represented.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the attempt number.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for RecoveryAttemptNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// ============================================================================
// Explicit timestamp
// ============================================================================

/// Caller-supplied recovery timestamp.
///
/// This value NEVER reads the system clock.
///
/// The representation is:
///
///     signed Unix seconds + nanoseconds
///
/// The nanosecond component must be less than one billion.
///
/// No retention policy is encoded here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecoveryTimestamp {
    seconds: i64,
    nanoseconds: u32,
}

impl RecoveryTimestamp {
    /// Number of nanoseconds per second.
    pub const NANOS_PER_SECOND: u32 = 1_000_000_000;

    /// Creates a valid timestamp.
    ///
    /// Returns `None` when `nanoseconds >= 1_000_000_000`.
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

    /// Returns Unix seconds.
    #[must_use]
    pub const fn seconds(self) -> i64 {
        self.seconds
    }

    /// Returns nanoseconds.
    #[must_use]
    pub const fn nanoseconds(self) -> u32 {
        self.nanoseconds
    }

    /// Returns whether the value is structurally valid.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.nanoseconds < Self::NANOS_PER_SECOND
    }
}

impl fmt::Display for RecoveryTimestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{:09}",
            self.seconds,
            self.nanoseconds
        )
    }
}

// ============================================================================
// Recovery disposition
// ============================================================================

/// Historical classification of the recovery operation.
///
/// This is a FACT recorded by history.
///
/// It is not a policy decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RecoveryDisposition {
    /// Recovery completed and the result was accepted after verification.
    Accepted,

    /// Recovery completed and the result was accepted under explicit
    /// degradation.
    Degraded,

    /// The current recovery plan could not safely continue.
    NeedsReplan,

    /// Automatic recovery stopped and requires escalation.
    Escalated,

    /// The resulting execution/result was explicitly rejected.
    Rejected,

    /// Recovery was cancelled.
    Cancelled,

    /// Recovery failed without reaching an accepted state.
    Failed,

    /// The recovery lifecycle ended without a definitive classification.
    Unknown,
}

impl RecoveryDisposition {
    /// Returns whether the recovery produced an accepted result.
    #[must_use]
    pub const fn is_accepted(self) -> bool {
        matches!(self, Self::Accepted | Self::Degraded)
    }

    /// Returns whether the accepted result is explicitly degraded.
    #[must_use]
    pub const fn is_degraded(self) -> bool {
        matches!(self, Self::Degraded)
    }

    /// Returns whether a replacement plan is required.
    #[must_use]
    pub const fn requires_replan(self) -> bool {
        matches!(self, Self::NeedsReplan)
    }

    /// Returns whether human/operator/distributed escalation is required.
    #[must_use]
    pub const fn requires_escalation(self) -> bool {
        matches!(self, Self::Escalated)
    }

    /// Returns whether the result must not be accepted.
    #[must_use]
    pub const fn is_rejected(self) -> bool {
        matches!(self, Self::Rejected | Self::Failed | Self::Cancelled)
    }
}

// ============================================================================
// Action disposition
// ============================================================================

/// Historical result of one recovery action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RecoveryActionDisposition {
    /// Action completed successfully.
    Applied,

    /// Action completed but the execution remains degraded.
    AppliedDegraded,

    /// Action was intentionally skipped.
    Skipped,

    /// Action could not run because its preconditions failed.
    PreconditionsFailed,

    /// Action failed but the overall recovery could potentially be replanned.
    FailedRecoverable,

    /// Action failed and automatic continuation was unsafe.
    FailedFatal,

    /// Action was cancelled before completion.
    Cancelled,

    /// Action outcome could not be determined.
    Unknown,
}

impl RecoveryActionDisposition {
    /// Returns whether the action completed successfully.
    #[must_use]
    pub const fn succeeded(self) -> bool {
        matches!(
            self,
            Self::Applied | Self::AppliedDegraded
        )
    }

    /// Returns whether the action itself completed in degraded mode.
    #[must_use]
    pub const fn degraded(self) -> bool {
        matches!(self, Self::AppliedDegraded)
    }

    /// Returns whether the action failed.
    #[must_use]
    pub const fn failed(self) -> bool {
        matches!(
            self,
            Self::PreconditionsFailed
                | Self::FailedRecoverable
                | Self::FailedFatal
        )
    }
}

// ============================================================================
// Verification disposition
// ============================================================================

/// Historical verification result associated with recovery.
///
/// Execution success does NOT imply verification success.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RecoveryVerificationDisposition {
    /// Verification was not performed.
    NotPerformed,

    /// Verification accepted the result.
    Passed,

    /// Verification rejected the result.
    Failed,

    /// Verification could not establish a definitive answer.
    Inconclusive,
}

impl RecoveryVerificationDisposition {
    /// Returns whether the result was actually verified.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Passed)
    }

    /// Returns whether verification failed.
    #[must_use]
    pub const fn failed(self) -> bool {
        matches!(self, Self::Failed)
    }

    /// Returns whether verification did not establish correctness.
    #[must_use]
    pub const fn not_verified(self) -> bool {
        !self.is_verified()
    }
}

// ============================================================================
// Recovery phase
// ============================================================================

/// Phase in which a recovery action was recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RecoveryPhase {
    /// Protective containment.
    Containment,

    /// Preconditions and state validation.
    Validation,

    /// Ownership/lease acquisition.
    Ownership,

    /// Actual recovery/adaptation action.
    Execution,

    /// Post-action verification.
    Verification,

    /// Cleanup/finalization.
    Finalization,
}

// ============================================================================
// Action record
// ============================================================================

/// Immutable historical record for one recovery action.
///
/// Action identity and kind are opaque strings because the history layer must
/// not depend on concrete executable recovery implementations.
///
/// The action name is descriptive data only.
///
/// It does not identify a function, callback, plugin pointer or executable
/// object.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecoveryActionRecord {
    sequence: u64,
    action_id: HistoryId,
    action_kind: HistoryId,
    phase: RecoveryPhase,
    disposition: RecoveryActionDisposition,
    started_at: Option<RecoveryTimestamp>,
    completed_at: Option<RecoveryTimestamp>,
    error_code: Option<ResilienceErrorCode>,
    detail: Option<Arc<str>>,
}

impl RecoveryActionRecord {
    /// Creates an action history record.
    pub fn new(
        sequence: u64,
        action_id: HistoryId,
        action_kind: HistoryId,
        phase: RecoveryPhase,
        disposition: RecoveryActionDisposition,
    ) -> Result<Self, RecoveryHistoryError> {
        Ok(Self {
            sequence,
            action_id,
            action_kind,
            phase,
            disposition,
            started_at: None,
            completed_at: None,
            error_code: None,
            detail: None,
        })
    }

    /// Sets the caller-supplied start timestamp.
    #[must_use]
    pub const fn with_started_at(
        mut self,
        timestamp: RecoveryTimestamp,
    ) -> Self {
        self.started_at = Some(timestamp);
        self
    }

    /// Sets the caller-supplied completion timestamp.
    #[must_use]
    pub const fn with_completed_at(
        mut self,
        timestamp: RecoveryTimestamp,
    ) -> Self {
        self.completed_at = Some(timestamp);
        self
    }

    /// Associates the canonical resilience error code.
    #[must_use]
    pub const fn with_error_code(
        mut self,
        code: ResilienceErrorCode,
    ) -> Self {
        self.error_code = Some(code);
        self
    }

    /// Adds caller-supplied diagnostic detail.
    ///
    /// Detail is descriptive data and must never contain secrets.
    pub fn with_detail(
        mut self,
        detail: impl Into<Arc<str>>,
    ) -> Result<Self, RecoveryHistoryError> {
        let detail = detail.into();

        if detail.is_empty() {
            return Err(RecoveryHistoryError::EmptyDetail);
        }

        self.detail = Some(detail);
        Ok(self)
    }

    /// Returns the action sequence.
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the action identity.
    #[must_use]
    pub fn action_id(&self) -> &HistoryId {
        &self.action_id
    }

    /// Returns the action kind.
    #[must_use]
    pub fn action_kind(&self) -> &HistoryId {
        &self.action_kind
    }

    /// Returns the recovery phase.
    #[must_use]
    pub const fn phase(&self) -> RecoveryPhase {
        self.phase
    }

    /// Returns the action disposition.
    #[must_use]
    pub const fn disposition(&self) -> RecoveryActionDisposition {
        self.disposition
    }

    /// Returns the optional start timestamp.
    #[must_use]
    pub const fn started_at(&self) -> Option<RecoveryTimestamp> {
        self.started_at
    }

    /// Returns the optional completion timestamp.
    #[must_use]
    pub const fn completed_at(&self) -> Option<RecoveryTimestamp> {
        self.completed_at
    }

    /// Returns the optional canonical resilience error code.
    #[must_use]
    pub const fn error_code(&self) -> Option<ResilienceErrorCode> {
        self.error_code
    }

    /// Returns optional descriptive detail.
    #[must_use]
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    /// Validates action-level structural invariants.
    ///
    /// This performs no hardware, policy or semantic verification.
    pub fn validate(&self) -> Result<(), RecoveryHistoryError> {
        if self.action_id.as_str().is_empty() {
            return Err(RecoveryHistoryError::EmptyIdentity);
        }

        if self.action_kind.as_str().is_empty() {
            return Err(RecoveryHistoryError::EmptyIdentity);
        }

        if let (Some(start), Some(end)) = (self.started_at, self.completed_at) {
            if end < start {
                return Err(RecoveryHistoryError::InvalidTimestampOrder);
            }
        }

        Ok(())
    }
}

// ============================================================================
// Verification record
// ============================================================================

/// Immutable historical verification record.
///
/// This deliberately stores the result of verification, not the verification
/// implementation itself.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecoveryVerificationRecord {
    disposition: RecoveryVerificationDisposition,
    verifier_id: Option<HistoryId>,
    verification_reference: Option<HistoryId>,
    detail: Option<Arc<str>>,
}

impl RecoveryVerificationRecord {
    /// Creates a verification record.
    #[must_use]
    pub const fn new(
        disposition: RecoveryVerificationDisposition,
    ) -> Self {
        Self {
            disposition,
            verifier_id: None,
            verification_reference: None,
            detail: None,
        }
    }

    /// Associates the stable verifier identity.
    #[must_use]
    pub fn with_verifier_id(mut self, id: HistoryId) -> Self {
        self.verifier_id = Some(id);
        self
    }

    /// Associates an external verification-record identity.
    #[must_use]
    pub fn with_reference(mut self, reference: HistoryId) -> Self {
        self.verification_reference = Some(reference);
        self
    }

    /// Adds descriptive verification detail.
    pub fn with_detail(
        mut self,
        detail: impl Into<Arc<str>>,
    ) -> Result<Self, RecoveryHistoryError> {
        let detail = detail.into();

        if detail.is_empty() {
            return Err(RecoveryHistoryError::EmptyDetail);
        }

        self.detail = Some(detail);
        Ok(self)
    }

    /// Returns the verification disposition.
    #[must_use]
    pub const fn disposition(
        &self,
    ) -> RecoveryVerificationDisposition {
        self.disposition
    }

    /// Returns the optional verifier identity.
    #[must_use]
    pub fn verifier_id(&self) -> Option<&HistoryId> {
        self.verifier_id.as_ref()
    }

    /// Returns the optional verification-record identity.
    #[must_use]
    pub fn verification_reference(&self) -> Option<&HistoryId> {
        self.verification_reference.as_ref()
    }

    /// Returns optional verification detail.
    #[must_use]
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }
}

// ============================================================================
// Provenance
// ============================================================================

/// Immutable recovery provenance references.
///
/// These are opaque identities/digests.
///
/// This module does not interpret or cryptographically verify them.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct RecoveryProvenance {
    program: Option<HistoryId>,
    canonical_ir: Option<HistoryId>,
    target_snapshot: Option<HistoryId>,
    capability_snapshot: Option<HistoryId>,
    policy_snapshot: Option<HistoryId>,
    calibration_snapshot: Option<HistoryId>,
    strategy_set: Option<HistoryId>,
}

impl RecoveryProvenance {
    /// Creates an empty provenance object.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            program: None,
            canonical_ir: None,
            target_snapshot: None,
            capability_snapshot: None,
            policy_snapshot: None,
            calibration_snapshot: None,
            strategy_set: None,
        }
    }

    /// Associates the program identity/digest.
    #[must_use]
    pub fn with_program(mut self, value: HistoryId) -> Self {
        self.program = Some(value);
        self
    }

    /// Associates the canonical IR identity/digest.
    #[must_use]
    pub fn with_canonical_ir(mut self, value: HistoryId) -> Self {
        self.canonical_ir = Some(value);
        self
    }

    /// Associates the target snapshot identity.
    #[must_use]
    pub fn with_target_snapshot(mut self, value: HistoryId) -> Self {
        self.target_snapshot = Some(value);
        self
    }

    /// Associates the capability snapshot identity.
    #[must_use]
    pub fn with_capability_snapshot(
        mut self,
        value: HistoryId,
    ) -> Self {
        self.capability_snapshot = Some(value);
        self
    }

    /// Associates the policy snapshot identity.
    #[must_use]
    pub fn with_policy_snapshot(mut self, value: HistoryId) -> Self {
        self.policy_snapshot = Some(value);
        self
    }

    /// Associates the calibration snapshot identity.
    #[must_use]
    pub fn with_calibration_snapshot(
        mut self,
        value: HistoryId,
    ) -> Self {
        self.calibration_snapshot = Some(value);
        self
    }

    /// Associates the recovery-strategy implementation-set identity.
    #[must_use]
    pub fn with_strategy_set(mut self, value: HistoryId) -> Self {
        self.strategy_set = Some(value);
        self
    }

    /// Returns the program provenance.
    #[must_use]
    pub fn program(&self) -> Option<&HistoryId> {
        self.program.as_ref()
    }

    /// Returns the canonical IR provenance.
    #[must_use]
    pub fn canonical_ir(&self) -> Option<&HistoryId> {
        self.canonical_ir.as_ref()
    }

    /// Returns the target snapshot provenance.
    #[must_use]
    pub fn target_snapshot(&self) -> Option<&HistoryId> {
        self.target_snapshot.as_ref()
    }

    /// Returns the capability snapshot provenance.
    #[must_use]
    pub fn capability_snapshot(&self) -> Option<&HistoryId> {
        self.capability_snapshot.as_ref()
    }

    /// Returns the policy snapshot provenance.
    #[must_use]
    pub fn policy_snapshot(&self) -> Option<&HistoryId> {
        self.policy_snapshot.as_ref()
    }

    /// Returns the calibration snapshot provenance.
    #[must_use]
    pub fn calibration_snapshot(&self) -> Option<&HistoryId> {
        self.calibration_snapshot.as_ref()
    }

    /// Returns the strategy-set provenance.
    #[must_use]
    pub fn strategy_set(&self) -> Option<&HistoryId> {
        self.strategy_set.as_ref()
    }

    /// Returns whether no provenance values were supplied.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.program.is_none()
            && self.canonical_ir.is_none()
            && self.target_snapshot.is_none()
            && self.capability_snapshot.is_none()
            && self.policy_snapshot.is_none()
            && self.calibration_snapshot.is_none()
            && self.strategy_set.is_none()
    }
}

// ============================================================================
// Recovery history entry
// ============================================================================

/// Immutable historical record of one recovery operation.
///
/// This is the primary type exported by this module.
///
/// It records:
///
/// - which recovery operation occurred;
/// - which execution it belonged to;
/// - which incident it addressed;
/// - which plan revision was used;
/// - which attempt was recorded;
/// - when it started/completed;
/// - which actions were executed;
/// - what recovery disposition resulted;
/// - what verification established;
/// - which error code, if any, was associated;
/// - which provenance references were active.
///
/// It does not contain executable recovery objects.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecoveryHistoryEntry {
    schema_version: u16,
    sequence: RecoveryHistorySequence,
    operation_id: HistoryId,
    execution_id: HistoryId,
    incident_id: Option<IncidentId>,
    plan_id: HistoryId,
    plan_version: u64,
    attempt: RecoveryAttemptNumber,
    started_at: Option<RecoveryTimestamp>,
    completed_at: Option<RecoveryTimestamp>,
    disposition: RecoveryDisposition,
    verification: RecoveryVerificationRecord,
    actions: Vec<RecoveryActionRecord>,
    error_code: Option<ResilienceErrorCode>,
    provenance: RecoveryProvenance,
    detail: Option<Arc<str>>,
}

impl RecoveryHistoryEntry {
    /// Creates a new immutable recovery-history entry.
    ///
    /// The action collection is consumed and canonicalized by action sequence.
    ///
    /// The history entry itself imposes no action-count limit.
    pub fn new<I>(
        sequence: RecoveryHistorySequence,
        operation_id: HistoryId,
        execution_id: HistoryId,
        incident_id: Option<IncidentId>,
        plan_id: HistoryId,
        plan_version: u64,
        attempt: RecoveryAttemptNumber,
        disposition: RecoveryDisposition,
        verification: RecoveryVerificationRecord,
        actions: I,
    ) -> Result<Self, RecoveryHistoryError>
    where
        I: IntoIterator<Item = RecoveryActionRecord>,
    {
        let mut actions: Vec<RecoveryActionRecord> =
            actions.into_iter().collect();

        for action in &actions {
            action.validate()?;
        }

        actions.sort_by_key(RecoveryActionRecord::sequence);

        let entry = Self {
            schema_version: RECOVERY_HISTORY_SCHEMA_VERSION,
            sequence,
            operation_id,
            execution_id,
            incident_id,
            plan_id,
            plan_version,
            attempt,
            started_at: None,
            completed_at: None,
            disposition,
            verification,
            actions,
            error_code: None,
            provenance: RecoveryProvenance::new(),
            detail: None,
        };

        entry.validate()?;

        Ok(entry)
    }

    /// Sets the caller-supplied recovery start timestamp.
    ///
    /// The original entry remains unchanged.
    #[must_use]
    pub const fn with_started_at(
        mut self,
        timestamp: RecoveryTimestamp,
    ) -> Self {
        self.started_at = Some(timestamp);
        self
    }

    /// Sets the caller-supplied recovery completion timestamp.
    ///
    /// The original entry remains unchanged.
    #[must_use]
    pub const fn with_completed_at(
        mut self,
        timestamp: RecoveryTimestamp,
    ) -> Self {
        self.completed_at = Some(timestamp);
        self
    }

    /// Associates the canonical resilience error code.
    #[must_use]
    pub const fn with_error_code(
        mut self,
        code: ResilienceErrorCode,
    ) -> Self {
        self.error_code = Some(code);
        self
    }

    /// Associates provenance.
    #[must_use]
    pub fn with_provenance(
        mut self,
        provenance: RecoveryProvenance,
    ) -> Self {
        self.provenance = provenance;
        self
    }

    /// Associates descriptive detail.
    ///
    /// Detail must not contain credentials, secrets or executable data.
    pub fn with_detail(
        mut self,
        detail: impl Into<Arc<str>>,
    ) -> Result<Self, RecoveryHistoryError> {
        let detail = detail.into();

        if detail.is_empty() {
            return Err(RecoveryHistoryError::EmptyDetail);
        }

        self.detail = Some(detail);
        Ok(self)
    }

    /// Returns the history schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns the history sequence.
    #[must_use]
    pub const fn sequence(&self) -> RecoveryHistorySequence {
        self.sequence
    }

    /// Returns the recovery operation identity.
    #[must_use]
    pub fn operation_id(&self) -> &HistoryId {
        &self.operation_id
    }

    /// Returns the execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &HistoryId {
        &self.execution_id
    }

    /// Returns the associated incident identity.
    #[must_use]
    pub const fn incident_id(&self) -> Option<IncidentId> {
        self.incident_id
    }

    /// Returns the recovery-plan identity.
    #[must_use]
    pub fn plan_id(&self) -> &HistoryId {
        &self.plan_id
    }

    /// Returns the plan revision.
    #[must_use]
    pub const fn plan_version(&self) -> u64 {
        self.plan_version
    }

    /// Returns the recovery attempt number.
    #[must_use]
    pub const fn attempt(&self) -> RecoveryAttemptNumber {
        self.attempt
    }

    /// Returns the recovery start timestamp.
    #[must_use]
    pub const fn started_at(&self) -> Option<RecoveryTimestamp> {
        self.started_at
    }

    /// Returns the recovery completion timestamp.
    #[must_use]
    pub const fn completed_at(&self) -> Option<RecoveryTimestamp> {
        self.completed_at
    }

    /// Returns the final recovery disposition.
    #[must_use]
    pub const fn disposition(&self) -> RecoveryDisposition {
        self.disposition
    }

    /// Returns the verification record.
    #[must_use]
    pub fn verification(&self) -> &RecoveryVerificationRecord {
        &self.verification
    }

    /// Returns all action records in deterministic sequence order.
    #[must_use]
    pub fn actions(&self) -> &[RecoveryActionRecord] {
        &self.actions
    }

    /// Returns the number of action records.
    #[must_use]
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    /// Returns the canonical resilience error code, if any.
    #[must_use]
    pub const fn error_code(&self) -> Option<ResilienceErrorCode> {
        self.error_code
    }

    /// Returns the provenance record.
    #[must_use]
    pub fn provenance(&self) -> &RecoveryProvenance {
        &self.provenance
    }

    /// Returns optional descriptive detail.
    #[must_use]
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    /// Returns whether this record represents an accepted recovery.
    #[must_use]
    pub const fn is_accepted(&self) -> bool {
        self.disposition.is_accepted()
    }

    /// Returns whether the accepted recovery is degraded.
    #[must_use]
    pub const fn is_degraded(&self) -> bool {
        self.disposition.is_degraded()
    }

    /// Returns whether replanning is required.
    #[must_use]
    pub const fn requires_replanning(&self) -> bool {
        self.disposition.requires_replan()
    }

    /// Returns whether escalation is required.
    #[must_use]
    pub const fn requires_escalation(&self) -> bool {
        self.disposition.requires_escalation()
    }

    /// Returns whether the result has actually been verified.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.verification.disposition().is_verified()
    }

    /// Returns whether verification failed.
    #[must_use]
    pub const fn verification_failed(&self) -> bool {
        self.verification.disposition().failed()
    }

    /// Returns whether the historical entry contains at least one failed
    /// action.
    #[must_use]
    pub fn contains_failed_action(&self) -> bool {
        self.actions
            .iter()
            .any(|action| action.disposition().failed())
    }

    /// Returns whether all recorded actions succeeded.
    ///
    /// An empty action list returns `true` mathematically, but such an entry
    /// may still be invalid for an actual recovery workflow depending on the
    /// surrounding policy.
    #[must_use]
    pub fn all_actions_succeeded(&self) -> bool {
        self.actions
            .iter()
            .all(|action| action.disposition().succeeded())
    }

    /// Returns the number of failed actions.
    #[must_use]
    pub fn failed_action_count(&self) -> usize {
        self.actions
            .iter()
            .filter(|action| action.disposition().failed())
            .count()
    }

    /// Performs structural validation.
    ///
    /// This function does NOT:
    ///
    /// - validate hardware;
    /// - validate capabilities;
    /// - validate policy;
    /// - verify quantum semantics;
    /// - verify cryptographic provenance;
    /// - access the backend.
    ///
    /// Those belong elsewhere.
    pub fn validate(&self) -> Result<(), RecoveryHistoryError> {
        if self.schema_version == 0 {
            return Err(RecoveryHistoryError::InvalidSchemaVersion);
        }

        if self.operation_id.as_str().is_empty()
            || self.execution_id.as_str().is_empty()
            || self.plan_id.as_str().is_empty()
        {
            return Err(RecoveryHistoryError::EmptyIdentity);
        }

        if let (Some(start), Some(end)) =
            (self.started_at, self.completed_at)
        {
            if end < start {
                return Err(RecoveryHistoryError::InvalidTimestampOrder);
            }
        }

        for window in self.actions.windows(2) {
            if window[0].sequence() > window[1].sequence() {
                return Err(RecoveryHistoryError::NonCanonicalActionOrder);
            }
        }

        for action in &self.actions {
            action.validate()?;
        }

        Ok(())
    }

    /// Returns a new history entry with a different history sequence.
    ///
    /// This is useful when a persistence layer assigns stream position after
    /// constructing the semantic record.
    #[must_use]
    pub const fn with_sequence(
        mut self,
        sequence: RecoveryHistorySequence,
    ) -> Self {
        self.sequence = sequence;
        self
    }

    /// Returns a new history entry with a different attempt number.
    #[must_use]
    pub const fn with_attempt(
        mut self,
        attempt: RecoveryAttemptNumber,
    ) -> Self {
        self.attempt = attempt;
        self
    }

    /// Consumes the entry and returns its action records.
    #[must_use]
    pub fn into_actions(self) -> Vec<RecoveryActionRecord> {
        self.actions
    }
}

// ============================================================================
// History errors
// ============================================================================

/// Structural errors owned by the recovery-history value model.
///
/// This is intentionally small.
///
/// Operational resilience failures continue to use the canonical
/// `ResilienceErrorCode` registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryHistoryError {
    /// An identity field was empty.
    EmptyIdentity,

    /// Descriptive detail was empty.
    EmptyDetail,

    /// The schema version is invalid.
    InvalidSchemaVersion,

    /// Start timestamp occurs after completion timestamp.
    InvalidTimestampOrder,

    /// Action records were not in canonical sequence order.
    NonCanonicalActionOrder,
}

impl fmt::Display for RecoveryHistoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentity => {
                formatter.write_str("recovery history identity must not be empty")
            }
            Self::EmptyDetail => {
                formatter.write_str("recovery history detail must not be empty")
            }
            Self::InvalidSchemaVersion => {
                formatter.write_str("invalid recovery history schema version")
            }
            Self::InvalidTimestampOrder => {
                formatter.write_str(
                    "recovery history completion timestamp precedes start timestamp",
                )
            }
            Self::NonCanonicalActionOrder => {
                formatter.write_str(
                    "recovery action records are not in canonical sequence order",
                )
            }
        }
    }
}

impl std::error::Error for RecoveryHistoryError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> HistoryId {
        HistoryId::new(value).expect("test identity must be valid")
    }

    fn timestamp(seconds: i64) -> RecoveryTimestamp {
        RecoveryTimestamp::new(seconds, 0)
            .expect("test timestamp must be valid")
    }

    fn action(
        sequence: u64,
        action_id: &str,
        kind: &str,
        disposition: RecoveryActionDisposition,
    ) -> RecoveryActionRecord {
        RecoveryActionRecord::new(
            sequence,
            id(action_id),
            id(kind),
            RecoveryPhase::Execution,
            disposition,
        )
        .expect("test action must be valid")
    }

    #[test]
    fn history_id_rejects_empty_values() {
        assert!(HistoryId::new("").is_err());
    }

    #[test]
    fn timestamp_rejects_invalid_nanoseconds() {
        assert!(
            RecoveryTimestamp::new(
                0,
                RecoveryTimestamp::NANOS_PER_SECOND
            )
            .is_none()
        );
    }

    #[test]
    fn timestamp_accepts_valid_nanoseconds() {
        assert!(
            RecoveryTimestamp::new(
                0,
                RecoveryTimestamp::NANOS_PER_SECOND - 1
            )
            .is_some()
        );
    }

    #[test]
    fn action_records_validate_timestamp_order() {
        let action = action(
            0,
            "action-0",
            "retry",
            RecoveryActionDisposition::Applied,
        )
        .with_started_at(timestamp(20))
        .with_completed_at(timestamp(10));

        assert_eq!(
            action.validate(),
            Err(RecoveryHistoryError::InvalidTimestampOrder)
        );
    }

    #[test]
    fn history_entry_sorts_actions_deterministically() {
        let first = action(
            1,
            "action-1",
            "resume",
            RecoveryActionDisposition::Applied,
        );

        let second = action(
            0,
            "action-0",
            "restart",
            RecoveryActionDisposition::Applied,
        );

        let entry = RecoveryHistoryEntry::new(
            RecoveryHistorySequence::new(10),
            id("recovery-1"),
            id("execution-1"),
            None,
            id("plan-1"),
            1,
            RecoveryAttemptNumber::new(0),
            RecoveryDisposition::Accepted,
            RecoveryVerificationRecord::new(
                RecoveryVerificationDisposition::Passed,
            ),
            [first, second],
        )
        .expect("entry should be valid");

        assert_eq!(entry.actions()[0].sequence(), 0);
        assert_eq!(entry.actions()[1].sequence(), 1);
    }

    #[test]
    fn accepted_recovery_is_not_automatically_verified() {
        let entry = RecoveryHistoryEntry::new(
            RecoveryHistorySequence::new(0),
            id("recovery-1"),
            id("execution-1"),
            None,
            id("plan-1"),
            1,
            RecoveryAttemptNumber::new(0),
            RecoveryDisposition::Accepted,
            RecoveryVerificationRecord::new(
                RecoveryVerificationDisposition::NotPerformed,
            ),
            std::iter::empty(),
        )
        .expect("entry should be valid");

        assert!(entry.is_accepted());
        assert!(!entry.is_verified());
    }

    #[test]
    fn degraded_recovery_is_accepted_but_marked_degraded() {
        let entry = RecoveryHistoryEntry::new(
            RecoveryHistorySequence::new(0),
            id("recovery-1"),
            id("execution-1"),
            None,
            id("plan-1"),
            1,
            RecoveryAttemptNumber::new(1),
            RecoveryDisposition::Degraded,
            RecoveryVerificationRecord::new(
                RecoveryVerificationDisposition::Passed,
            ),
            std::iter::empty(),
        )
        .expect("entry should be valid");

        assert!(entry.is_accepted());
        assert!(entry.is_degraded());
        assert!(entry.is_verified());
    }

    #[test]
    fn failed_action_is_recorded_without_reinterpreting_recovery() {
        let action = action(
            0,
            "action-0",
            "reroute",
            RecoveryActionDisposition::FailedRecoverable,
        )
        .with_error_code(ResilienceErrorCode::ReroutingFailed);

        let entry = RecoveryHistoryEntry::new(
            RecoveryHistorySequence::new(1),
            id("recovery-1"),
            id("execution-1"),
            None,
            id("plan-1"),
            2,
            RecoveryAttemptNumber::new(0),
            RecoveryDisposition::NeedsReplan,
            RecoveryVerificationRecord::new(
                RecoveryVerificationDisposition::NotPerformed,
            ),
            [action],
        )
        .expect("entry should be valid");

        assert!(entry.contains_failed_action());
        assert_eq!(entry.failed_action_count(), 1);
        assert!(entry.requires_replanning());
    }

    #[test]
    fn provenance_is_explicit() {
        let provenance = RecoveryProvenance::new()
            .with_program(id("program-digest"))
            .with_canonical_ir(id("ir-digest"))
            .with_capability_snapshot(id("capability-snapshot"));

        assert!(!provenance.is_empty());
        assert_eq!(
            provenance.program().map(HistoryId::as_str),
            Some("program-digest")
        );
        assert_eq!(
            provenance.canonical_ir().map(HistoryId::as_str),
            Some("ir-digest")
        );
    }

    #[test]
    fn identical_inputs_produce_equal_records() {
        let make_entry = || {
            RecoveryHistoryEntry::new(
                RecoveryHistorySequence::new(7),
                id("recovery-7"),
                id("execution-7"),
                None,
                id("plan-7"),
                3,
                RecoveryAttemptNumber::new(2),
                RecoveryDisposition::Accepted,
                RecoveryVerificationRecord::new(
                    RecoveryVerificationDisposition::Passed,
                ),
                [action(
                    0,
                    "action-0",
                    "reroute",
                    RecoveryActionDisposition::Applied,
                )],
            )
            .expect("entry should be valid")
            .with_started_at(timestamp(100))
            .with_completed_at(timestamp(101))
        };

        assert_eq!(make_entry(), make_entry());
    }

    #[test]
    fn no_fixed_action_limit_exists() {
        let actions = (0_u64..4096_u64).map(|index| {
            action(
                index,
                &format!("action-{index}"),
                "generic",
                RecoveryActionDisposition::Applied,
            )
        });

        let entry = RecoveryHistoryEntry::new(
            RecoveryHistorySequence::new(1),
            id("recovery-large"),
            id("execution-large"),
            None,
            id("plan-large"),
            1,
            RecoveryAttemptNumber::new(0),
            RecoveryDisposition::Accepted,
            RecoveryVerificationRecord::new(
                RecoveryVerificationDisposition::Passed,
            ),
            actions,
        )
        .expect("large test entry should be valid");

        assert_eq!(entry.action_count(), 4096);
        assert!(entry.all_actions_succeeded());
    }
}