//! Zamani Quantum Resilience — Persistent Incident History.
//!
//! Path:
//!     src/quantum/resilience/history/incident.rs
//!
//! # Purpose
//!
//! This module defines immutable historical records for resilience incidents.
//!
//! The distinction between the incident model and incident history is
//! fundamental:
//!
//! ```text
//! quantum::resilience::model::incident
//!     └── Incident
//!         = immutable description of a correlated resilience incident
//!
//! quantum::resilience::history::incident
//!     └── IncidentHistoryRecord
//!         = immutable historical observation of an Incident
//! ```
//!
//! The history layer records what the resilience system knew at a particular
//! point in its lifecycle. It does not determine what the incident means and
//! it does not perform recovery.
//!
//! # Architectural ownership
//!
//! ```text
//! quantum::ir::qubit
//!     └── canonical logical/physical quantum-resource identities
//!
//! quantum::zqn::fault
//!     └── canonical quantum fault semantics
//!
//! quantum::resilience::model::fault
//!     └── resilience fault boundary
//!
//! quantum::resilience::model::incident
//!     └── Incident
//!
//! quantum::resilience::history::incident
//!     └── IncidentHistoryRecord
//!
//! quantum::resilience::history::execution
//!     └── execution history
//!
//! quantum::resilience::history::recovery
//!     └── recovery history
//!
//! quantum::resilience::history::statistics
//!     └── derived historical statistics
//! ```
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - immutable historical incident records;
//! - explicit history sequencing;
//! - explicit producer-supplied event time;
//! - historical incident lifecycle classification;
//! - correlation between a history record and an `Incident`;
//! - deterministic equality and ordering;
//! - structural validation;
//! - bounded projections over an individual incident record;
//! - deterministic record replacement/versioning;
//! - provenance-neutral historical storage data.
//!
//! This module does NOT own:
//!
//! - fault detection;
//! - fault diagnosis;
//! - severity inference;
//! - recovery decisions;
//! - recovery execution;
//! - QEC;
//! - routing;
//! - scheduling;
//! - hardware access;
//! - persistence backends;
//! - filesystem access;
//! - network access;
//! - serialization formats;
//! - retention policy;
//! - database indexing;
//! - cryptographic hashing;
//! - authorization;
//! - clocks;
//! - random identifiers;
//! - global mutable state.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Why history is separate from Incident
//!
//! An `Incident` is a semantic object.
//!
//! A historical record is an observation of that object at a point in the
//! resilience lifecycle.
//!
//! Therefore the following are intentionally different:
//!
//! ```text
//! Incident
//!
//!     id
//!     faults
//!
//! IncidentHistoryRecord
//!
//!     sequence
//!     incident
//!     event
//!     producer_time
//! ```
//!
//! The same incident can therefore have multiple history records:
//!
//! ```text
//! Incident X
//!    │
//!    ├── detected
//!    ├── correlated
//!    ├── diagnosed
//!    ├── recovery-started
//!    ├── recovery-completed
//!    └── verified
//! ```
//!
//! The history subsystem records those facts without changing the underlying
//! `Incident`.
//!
//! # Immutability
//!
//! All public history objects are immutable after construction.
//!
//! If a new fact becomes known, create a new history record rather than
//! modifying an existing record.
//!
//! This is required for:
//!
//! - deterministic replay;
//! - auditability;
//! - forensic analysis;
//! - concurrent readers;
//! - reproducibility;
//! - append-oriented persistence;
//! - distributed history reconciliation.
//!
//! # Determinism
//!
//! This module performs no implicit environmental inspection.
//!
//! It does not read:
//!
//! - system time;
//! - environment variables;
//! - process IDs;
//! - thread IDs;
//! - memory addresses;
//! - filesystem state;
//! - network state;
//! - random generators.
//!
//! All ordering information is explicitly supplied by the producer.
//!
//! A history record constructed from identical inputs is identical.
//!
//! # Ordering
//!
//! History records are ordered by their explicit `HistorySequence`.
//!
//! A timestamp is deliberately NOT used as the sole ordering mechanism.
//!
//! Multiple distributed producers may produce events with equal timestamps,
//! clock skew may exist, and physical time does not necessarily establish
//! causal order.
//!
//! The producer must therefore supply an explicit sequence when a total
//! deterministic order is required.
//!
//! # Scalability
//!
//! No machine-size assumptions exist in this module.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_INCIDENTS
//! MAX_FAULTS
//! MAX_HISTORY
//! MAX_QUBITS
//! MAX_DEVICES
//! MAX_BACKENDS
//! ```
//!
//! This type therefore scales semantically from a single-qubit system to a
//! distributed quantum system limited only by the resources of the concrete
//! history store and the policy selected by the caller.
//!
//! This does NOT mean that a process has infinite memory.
//!
//! It means that this data model imposes no artificial quantum-machine size
//! ceiling.
//!
//! Large deployments should store records through streaming/append-oriented
//! persistence rather than retaining the complete lifetime history in memory.
//!
//! # Resource identity
//!
//! This module intentionally does not import or redefine:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Those identities already belong to the canonical quantum IR and flow into
//! the historical record through `Incident -> ResilienceFault -> canonical
//! fault`.
//!
//! Introducing another qubit identity here would violate the repository's
//! canonical identity boundary.
//!
//! # Integration contract
//!
//! History consumers should use this module as follows:
//!
//! ```text
//! detector
//!    │
//!    ▼
//! diagnosis / correlation
//!    │
//!    ▼
//! Incident
//!    │
//!    ▼
//! IncidentHistoryRecord
//!    │
//!    ├── persistent history
//!    ├── observability
//!    ├── deterministic replay
//!    ├── statistics
//!    ├── learning
//!    └── audit/provenance
//! ```
//!
//! Recovery, diagnosis and policy layers MUST NOT mutate historical records.
//!
//! # Serialization
//!
//! Serialization is deliberately not implemented here.
//!
//! The authoritative serialization boundary is:
//!
//! ```text
//! quantum::resilience::serialization
//! ```
//!
//! A serialization implementation should encode the complete semantic state
//! of this module and preserve the explicit sequence, incident identity,
//! incident contents, lifecycle event, and producer time.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! `unsafe` is explicitly forbidden.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use core::num::NonZeroU64;

use super::super::model::incident::{Incident, IncidentId};

// ============================================================================
// History sequence
// ============================================================================

/// Monotonically assigned history position supplied by the history owner.
///
/// `HistorySequence` is deliberately opaque.
///
/// It does not represent:
///
/// - a qubit;
/// - an operation;
/// - a fault;
/// - a timestamp;
/// - a retry count;
/// - a machine size.
///
/// The sequence is supplied externally because this module must not own a
/// global counter or hidden mutable state.
///
/// A distributed history implementation may allocate sequences using any
/// suitable externally coordinated mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HistorySequence(NonZeroU64);

impl HistorySequence {
    /// Creates a history sequence from a non-zero value.
    ///
    /// Zero is rejected because it is reserved as "not assigned".
    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        match NonZeroU64::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the numeric sequence value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0.get()
    }
}

impl fmt::Display for HistorySequence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "history-{}", self.value())
    }
}

// ============================================================================
// Producer time
// ============================================================================

/// Explicit producer-supplied event time.
///
/// This is intentionally a value object rather than an invocation of the
/// system clock.
///
/// `seconds` is signed so that historical data can represent timestamps before
/// the Unix epoch when required by an external producer.
///
/// `nanoseconds` must be in the canonical range `0..1_000_000_000`.
///
/// The type carries no timezone semantics. The value represents an instant
/// relative to the Unix epoch as supplied by the producer.
///
/// A caller that does not have trustworthy physical time may omit the value
/// from `IncidentHistoryRecord`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProducerTime {
    seconds: i64,
    nanoseconds: u32,
}

impl ProducerTime {
    /// Number of nanoseconds in one second.
    ///
    /// This is a unit conversion constant, not a quantum-system limit.
    pub const NANOS_PER_SECOND: u32 = 1_000_000_000;

    /// Creates a producer timestamp.
    ///
    /// Returns `None` if `nanoseconds` is outside the canonical range.
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

    /// Returns seconds since the Unix epoch.
    #[must_use]
    pub const fn seconds(self) -> i64 {
        self.seconds
    }

    /// Returns the fractional nanosecond component.
    #[must_use]
    pub const fn nanoseconds(self) -> u32 {
        self.nanoseconds
    }
}

impl fmt::Display for ProducerTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{:09}Z",
            self.seconds,
            self.nanoseconds
        )
    }
}

// ============================================================================
// Historical lifecycle event
// ============================================================================

/// Historical classification of an incident-related fact.
///
/// These values describe what was recorded; they do not prescribe what the
/// resilience engine should do.
///
/// In particular:
///
/// - `Detected` does not mean the fault is proven;
/// - `Diagnosed` does not mean the diagnosis is correct;
/// - `RecoveryCompleted` does not mean the result is verified;
/// - `Verified` does not mean the result must be accepted.
///
/// Decision semantics remain owned by detection, diagnosis, planning,
/// recovery and verification respectively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IncidentHistoryEvent {
    /// The incident was first recorded as an observation.
    Detected,

    /// Additional observations were correlated into the incident.
    Correlated,

    /// A diagnosis was recorded for the incident.
    Diagnosed,

    /// A resilience plan was recorded as having been selected.
    PlanSelected,

    /// Adaptation began.
    AdaptationStarted,

    /// Adaptation completed.
    AdaptationCompleted,

    /// Recovery began.
    RecoveryStarted,

    /// Recovery completed.
    RecoveryCompleted,

    /// Verification began.
    VerificationStarted,

    /// Verification completed.
    VerificationCompleted,

    /// The incident was explicitly escalated.
    Escalated,

    /// The incident was explicitly rejected as non-actionable.
    Rejected,

    /// The incident was explicitly marked resolved by a higher-level owner.
    Resolved,

    /// A later record superseded this historical interpretation.
    Superseded,
}

impl IncidentHistoryEvent {
    /// Returns a stable machine-readable event name.
    ///
    /// These names are intentionally provider-independent and suitable for
    /// serialization, logging, metrics, and deterministic replay.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Detected => "incident.detected",
            Self::Correlated => "incident.correlated",
            Self::Diagnosed => "incident.diagnosed",
            Self::PlanSelected => "incident.plan_selected",
            Self::AdaptationStarted => "incident.adaptation_started",
            Self::AdaptationCompleted => "incident.adaptation_completed",
            Self::RecoveryStarted => "incident.recovery_started",
            Self::RecoveryCompleted => "incident.recovery_completed",
            Self::VerificationStarted => "incident.verification_started",
            Self::VerificationCompleted => "incident.verification_completed",
            Self::Escalated => "incident.escalated",
            Self::Rejected => "incident.rejected",
            Self::Resolved => "incident.resolved",
            Self::Superseded => "incident.superseded",
        }
    }

    /// Returns whether this event represents the beginning of an operational
    /// lifecycle phase.
    #[must_use]
    pub const fn is_start(self) -> bool {
        matches!(
            self,
            Self::Detected
                | Self::AdaptationStarted
                | Self::RecoveryStarted
                | Self::VerificationStarted
        )
    }

    /// Returns whether this event represents completion of an operational
    /// lifecycle phase.
    #[must_use]
    pub const fn is_completion(self) -> bool {
        matches!(
            self,
            Self::AdaptationCompleted
                | Self::RecoveryCompleted
                | Self::VerificationCompleted
                | Self::Resolved
        )
    }

    /// Returns whether this event is terminal from the perspective of the
    /// historical incident lifecycle.
    ///
    /// This is descriptive only. It does not prevent a later record from
    /// representing a new incident revision or a reopening.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Resolved | Self::Rejected | Self::Superseded
        )
    }
}

impl fmt::Display for IncidentHistoryEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Incident history record
// ============================================================================

/// Immutable historical record of an incident.
///
/// The record contains the complete `Incident` snapshot rather than only its
/// identifier. This is intentional.
///
/// A historical record must remain interpretable even if:
///
/// - the live incident store is unavailable;
/// - the current incident has later been revised;
/// - the execution has migrated;
/// - hardware has changed;
/// - the original telemetry source is no longer available.
///
/// The record therefore represents a historical snapshot of the incident as
/// it was known when the record was created.
///
/// # Ordering
///
/// `IncidentHistoryRecord` implements ordering using:
///
/// 1. `HistorySequence`;
/// 2. `IncidentId`;
/// 3. `IncidentHistoryEvent`;
/// 4. producer time;
/// 5. incident contents.
///
/// The primary ordering key is always the explicit history sequence.
///
/// # Memory
///
/// This structure owns an `Incident`, and therefore owns its fault collection.
/// A history store should generally persist records incrementally rather than
/// collecting an unlimited number of records in memory.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IncidentHistoryRecord {
    sequence: HistorySequence,
    incident: Incident,
    event: IncidentHistoryEvent,
    producer_time: Option<ProducerTime>,
}

impl IncidentHistoryRecord {
    /// Creates an immutable historical incident record.
    ///
    /// All values are explicit inputs. No clock, RNG, global state or
    /// environmental state is accessed.
    #[must_use]
    pub const fn new(
        sequence: HistorySequence,
        incident: Incident,
        event: IncidentHistoryEvent,
        producer_time: Option<ProducerTime>,
    ) -> Self {
        Self {
            sequence,
            incident,
            event,
            producer_time,
        }
    }

    /// Returns the history sequence.
    #[must_use]
    pub const fn sequence(&self) -> HistorySequence {
        self.sequence
    }

    /// Returns the incident snapshot.
    #[must_use]
    pub const fn incident(&self) -> &Incident {
        &self.incident
    }

    /// Returns the incident identity without cloning the incident.
    #[must_use]
    pub const fn incident_id(&self) -> IncidentId {
        self.incident.id()
    }

    /// Returns the historical lifecycle event.
    #[must_use]
    pub const fn event(&self) -> IncidentHistoryEvent {
        self.event
    }

    /// Returns the producer-supplied event time, if one was supplied.
    #[must_use]
    pub const fn producer_time(&self) -> Option<ProducerTime> {
        self.producer_time
    }

    /// Returns the number of faults in the historical incident snapshot.
    #[must_use]
    pub fn fault_count(&self) -> usize {
        self.incident.fault_count()
    }

    /// Returns whether the historical incident snapshot contains no faults.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.incident.is_empty()
    }

    /// Returns whether the record describes the beginning of a lifecycle
    /// phase.
    #[must_use]
    pub const fn is_start(&self) -> bool {
        self.event.is_start()
    }

    /// Returns whether the record describes completion of a lifecycle phase.
    #[must_use]
    pub const fn is_completion(&self) -> bool {
        self.event.is_completion()
    }

    /// Returns whether the record represents a terminal lifecycle event.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        self.event.is_terminal()
    }

    /// Returns a structurally equivalent record with a different history
    /// sequence.
    ///
    /// This is useful when an external persistence/coordination layer assigns
    /// the final sequence after a record has been assembled.
    ///
    /// The incident snapshot and event semantics are unchanged.
    #[must_use]
    pub const fn with_sequence(&self, sequence: HistorySequence) -> Self {
        Self {
            sequence,
            incident: self.incident.clone(),
            event: self.event,
            producer_time: self.producer_time,
        }
    }

    /// Returns a structurally equivalent record with a different explicit
    /// producer timestamp.
    #[must_use]
    pub const fn with_producer_time(&self, producer_time: Option<ProducerTime>) -> Self {
        Self {
            sequence: self.sequence,
            incident: self.incident.clone(),
            event: self.event,
            producer_time,
        }
    }

    /// Returns a structurally equivalent record containing a replacement
    /// incident snapshot.
    ///
    /// This does not mutate the existing record.
    #[must_use]
    pub const fn with_incident(&self, incident: Incident) -> Self {
        Self {
            sequence: self.sequence,
            incident,
            event: self.event,
            producer_time: self.producer_time,
        }
    }

    /// Returns a structurally equivalent record with another lifecycle event.
    ///
    /// Prefer constructing a new history sequence for a genuinely new
    /// historical fact. This method exists for transformations performed by
    /// serialization/reconciliation layers before persistence.
    #[must_use]
    pub const fn with_event(&self, event: IncidentHistoryEvent) -> Self {
        Self {
            sequence: self.sequence,
            incident: self.incident.clone(),
            event,
            producer_time: self.producer_time,
        }
    }

    /// Consumes the record and returns all of its components.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        HistorySequence,
        Incident,
        IncidentHistoryEvent,
        Option<ProducerTime>,
    ) {
        (
            self.sequence,
            self.incident,
            self.event,
            self.producer_time,
        )
    }

    /// Consumes the record and returns the incident snapshot.
    #[must_use]
    pub fn into_incident(self) -> Incident {
        self.incident
    }

    /// Performs structural validation owned by this module.
    ///
    /// The method deliberately does not validate:
    ///
    /// - hardware availability;
    /// - QEC configuration;
    /// - routing;
    /// - scheduling;
    /// - diagnosis correctness;
    /// - recovery correctness;
    /// - policy compliance;
    /// - result correctness.
    ///
    /// Those are higher-level concerns.
    #[must_use]
    pub fn is_structurally_valid(&self) -> bool {
        self.incident.is_structurally_valid()
    }
}

impl Ord for IncidentHistoryRecord {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.sequence
            .cmp(&other.sequence)
            .then_with(|| self.incident_id().cmp(&other.incident_id()))
            .then_with(|| self.event.cmp(&other.event))
            .then_with(|| self.producer_time.cmp(&other.producer_time))
            .then_with(|| self.incident.cmp(&other.incident))
    }
}

impl PartialOrd for IncidentHistoryRecord {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for IncidentHistoryRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.producer_time {
            Some(time) => write!(
                formatter,
                "{} {} {} @ {}",
                self.sequence,
                self.incident_id(),
                self.event,
                time
            ),
            None => write!(
                formatter,
                "{} {} {}",
                self.sequence,
                self.incident_id(),
                self.event
            ),
        }
    }
}

// ============================================================================
// History record construction helpers
// ============================================================================

/// Creates the first historical record for an incident.
///
/// The caller must supply the sequence because this module does not own a
/// global sequence generator.
#[must_use]
pub const fn record_detected(
    sequence: HistorySequence,
    incident: Incident,
    producer_time: Option<ProducerTime>,
) -> IncidentHistoryRecord {
    IncidentHistoryRecord::new(
        sequence,
        incident,
        IncidentHistoryEvent::Detected,
        producer_time,
    )
}

/// Creates a correlation history record.
#[must_use]
pub const fn record_correlated(
    sequence: HistorySequence,
    incident: Incident,
    producer_time: Option<ProducerTime>,
) -> IncidentHistoryRecord {
    IncidentHistoryRecord::new(
        sequence,
        incident,
        IncidentHistoryEvent::Correlated,
        producer_time,
    )
}

/// Creates a diagnosis history record.
#[must_use]
pub const fn record_diagnosed(
    sequence: HistorySequence,
    incident: Incident,
    producer_time: Option<ProducerTime>,
) -> IncidentHistoryRecord {
    IncidentHistoryRecord::new(
        sequence,
        incident,
        IncidentHistoryEvent::Diagnosed,
        producer_time,
    )
}

/// Creates a recovery-start history record.
#[must_use]
pub const fn record_recovery_started(
    sequence: HistorySequence,
    incident: Incident,
    producer_time: Option<ProducerTime>,
) -> IncidentHistoryRecord {
    IncidentHistoryRecord::new(
        sequence,
        incident,
        IncidentHistoryEvent::RecoveryStarted,
        producer_time,
    )
}

/// Creates a recovery-completed history record.
#[must_use]
pub const fn record_recovery_completed(
    sequence: HistorySequence,
    incident: Incident,
    producer_time: Option<ProducerTime>,
) -> IncidentHistoryRecord {
    IncidentHistoryRecord::new(
        sequence,
        incident,
        IncidentHistoryEvent::RecoveryCompleted,
        producer_time,
    )
}

/// Creates a verification-completed history record.
#[must_use]
pub const fn record_verification_completed(
    sequence: HistorySequence,
    incident: Incident,
    producer_time: Option<ProducerTime>,
) -> IncidentHistoryRecord {
    IncidentHistoryRecord::new(
        sequence,
        incident,
        IncidentHistoryEvent::VerificationCompleted,
        producer_time,
    )
}

/// Creates an escalation history record.
#[must_use]
pub const fn record_escalated(
    sequence: HistorySequence,
    incident: Incident,
    producer_time: Option<ProducerTime>,
) -> IncidentHistoryRecord {
    IncidentHistoryRecord::new(
        sequence,
        incident,
        IncidentHistoryEvent::Escalated,
        producer_time,
    )
}

/// Creates a resolved history record.
#[must_use]
pub const fn record_resolved(
    sequence: HistorySequence,
    incident: Incident,
    producer_time: Option<ProducerTime>,
) -> IncidentHistoryRecord {
    IncidentHistoryRecord::new(
        sequence,
        incident,
        IncidentHistoryEvent::Resolved,
        producer_time,
    )
}

// ============================================================================
// Deterministic history helpers
// ============================================================================

/// Returns whether two history records refer to the same incident.
///
/// This compares only `IncidentId`, not the complete incident snapshot.
#[must_use]
pub const fn same_incident(
    left: &IncidentHistoryRecord,
    right: &IncidentHistoryRecord,
) -> bool {
    left.incident_id().value() == right.incident_id().value()
}

/// Returns whether two history records represent the same historical event.
///
/// The sequence is deliberately included because two identical event kinds
/// may legitimately occur at different points in the lifecycle.
#[must_use]
pub fn same_history_event(
    left: &IncidentHistoryRecord,
    right: &IncidentHistoryRecord,
) -> bool {
    left.sequence() == right.sequence()
        && left.incident_id() == right.incident_id()
        && left.event() == right.event()
}

/// Returns the latest record from an iterator according to explicit history
/// ordering.
///
/// The iterator is consumed but no collection is created, making this useful
/// for streaming history stores.
///
/// No assumption is made about the number of records.
#[must_use]
pub fn latest_record<I>(records: I) -> Option<IncidentHistoryRecord>
where
    I: IntoIterator<Item = IncidentHistoryRecord>,
{
    records.into_iter().max()
}

/// Returns the latest record for a particular incident.
///
/// The implementation is streaming and therefore does not allocate a
/// secondary collection.
#[must_use]
pub fn latest_for_incident<I>(
    records: I,
    incident_id: IncidentId,
) -> Option<IncidentHistoryRecord>
where
    I: IntoIterator<Item = IncidentHistoryRecord>,
{
    records
        .into_iter()
        .filter(|record| record.incident_id() == incident_id)
        .max()
}

/// Counts records belonging to an incident without materializing another
/// collection.
///
/// This is intentionally generic over `IntoIterator` so a persistent history
/// backend can provide a streaming iterator.
#[must_use]
pub fn count_for_incident<I>(
    records: I,
    incident_id: IncidentId,
) -> usize
where
    I: IntoIterator<Item = IncidentHistoryRecord>,
{
    records
        .into_iter()
        .filter(|record| record.incident_id() == incident_id)
        .count()
}

/// Returns whether an incident has at least one historical record in the
/// supplied stream.
#[must_use]
pub fn contains_incident<I>(
    records: I,
    incident_id: IncidentId,
) -> bool
where
    I: IntoIterator<Item = IncidentHistoryRecord>,
{
    records
        .into_iter()
        .any(|record| record.incident_id() == incident_id)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::resilience::model::incident::IncidentId;

    fn sequence(value: u64) -> HistorySequence {
        HistorySequence::new(value).expect("test sequence must be non-zero")
    }

    fn incident_id(value: u64) -> IncidentId {
        IncidentId::new(value).expect("test incident ID must be non-zero")
    }

    fn empty_incident(value: u64) -> Incident {
        Incident::new(incident_id(value), core::iter::empty())
    }

    #[test]
    fn history_sequence_rejects_zero() {
        assert!(HistorySequence::new(0).is_none());
    }

    #[test]
    fn history_sequence_accepts_non_zero_values() {
        let value = HistorySequence::new(42).expect("non-zero sequence");
        assert_eq!(value.value(), 42);
    }

    #[test]
    fn producer_time_rejects_invalid_nanoseconds() {
        assert!(
            ProducerTime::new(
                0,
                ProducerTime::NANOS_PER_SECOND
            )
            .is_none()
        );
    }

    #[test]
    fn producer_time_accepts_canonical_nanoseconds() {
        let time = ProducerTime::new(123, 456_789_000)
            .expect("valid producer time");

        assert_eq!(time.seconds(), 123);
        assert_eq!(time.nanoseconds(), 456_789_000);
    }

    #[test]
    fn event_names_are_stable() {
        assert_eq!(
            IncidentHistoryEvent::Detected.as_str(),
            "incident.detected"
        );

        assert_eq!(
            IncidentHistoryEvent::RecoveryCompleted.as_str(),
            "incident.recovery_completed"
        );
    }

    #[test]
    fn history_record_is_immutable_by_api() {
        let incident = empty_incident(1);

        let record = IncidentHistoryRecord::new(
            sequence(1),
            incident,
            IncidentHistoryEvent::Detected,
            None,
        );

        assert_eq!(record.sequence().value(), 1);
        assert_eq!(record.incident_id().value(), 1);
        assert_eq!(
            record.event(),
            IncidentHistoryEvent::Detected
        );
    }

    #[test]
    fn history_record_preserves_incident_snapshot() {
        let incident = empty_incident(7);

        let record = IncidentHistoryRecord::new(
            sequence(10),
            incident,
            IncidentHistoryEvent::Detected,
            None,
        );

        assert_eq!(record.incident_id().value(), 7);
        assert_eq!(record.fault_count(), 0);
        assert!(record.is_empty());
    }

    #[test]
    fn history_records_order_by_sequence() {
        let incident = empty_incident(1);

        let first = IncidentHistoryRecord::new(
            sequence(1),
            incident.clone(),
            IncidentHistoryEvent::Detected,
            None,
        );

        let second = IncidentHistoryRecord::new(
            sequence(2),
            incident,
            IncidentHistoryEvent::Resolved,
            None,
        );

        assert!(first < second);
    }

    #[test]
    fn same_incident_ignores_history_sequence() {
        let incident_a = empty_incident(5);
        let incident_b = empty_incident(5);

        let first = IncidentHistoryRecord::new(
            sequence(1),
            incident_a,
            IncidentHistoryEvent::Detected,
            None,
        );

        let second = IncidentHistoryRecord::new(
            sequence(2),
            incident_b,
            IncidentHistoryEvent::Resolved,
            None,
        );

        assert!(same_incident(&first, &second));
    }

    #[test]
    fn same_history_event_requires_sequence_and_event_identity() {
        let incident = empty_incident(5);

        let first = IncidentHistoryRecord::new(
            sequence(1),
            incident.clone(),
            IncidentHistoryEvent::Detected,
            None,
        );

        let same = IncidentHistoryRecord::new(
            sequence(1),
            incident.clone(),
            IncidentHistoryEvent::Detected,
            None,
        );

        let different = IncidentHistoryRecord::new(
            sequence(2),
            incident,
            IncidentHistoryEvent::Detected,
            None,
        );

        assert!(same_history_event(&first, &same));
        assert!(!same_history_event(&first, &different));
    }

    #[test]
    fn latest_record_is_streaming_and_deterministic() {
        let incident = empty_incident(1);

        let records = vec![
            IncidentHistoryRecord::new(
                sequence(3),
                incident.clone(),
                IncidentHistoryEvent::Resolved,
                None,
            ),
            IncidentHistoryRecord::new(
                sequence(1),
                incident.clone(),
                IncidentHistoryEvent::Detected,
                None,
            ),
            IncidentHistoryRecord::new(
                sequence(2),
                incident,
                IncidentHistoryEvent::RecoveryCompleted,
                None,
            ),
        ];

        let latest = latest_record(records)
            .expect("records are non-empty");

        assert_eq!(latest.sequence().value(), 3);
        assert_eq!(
            latest.event(),
            IncidentHistoryEvent::Resolved
        );
    }

    #[test]
    fn latest_for_incident_selects_only_requested_incident() {
        let records = vec![
            IncidentHistoryRecord::new(
                sequence(1),
                empty_incident(1),
                IncidentHistoryEvent::Detected,
                None,
            ),
            IncidentHistoryRecord::new(
                sequence(2),
                empty_incident(2),
                IncidentHistoryEvent::Detected,
                None,
            ),
            IncidentHistoryRecord::new(
                sequence(3),
                empty_incident(1),
                IncidentHistoryEvent::Resolved,
                None,
            ),
        ];

        let latest = latest_for_incident(
            records,
            incident_id(1),
        )
        .expect("incident 1 exists");

        assert_eq!(latest.sequence().value(), 3);
        assert_eq!(
            latest.event(),
            IncidentHistoryEvent::Resolved
        );
    }

    #[test]
    fn count_for_incident_is_correct() {
        let records = vec![
            IncidentHistoryRecord::new(
                sequence(1),
                empty_incident(1),
                IncidentHistoryEvent::Detected,
                None,
            ),
            IncidentHistoryRecord::new(
                sequence(2),
                empty_incident(1),
                IncidentHistoryEvent::Diagnosed,
                None,
            ),
            IncidentHistoryRecord::new(
                sequence(3),
                empty_incident(2),
                IncidentHistoryEvent::Detected,
                None,
            ),
        ];

        assert_eq!(
            count_for_incident(records, incident_id(1)),
            2
        );
    }

    #[test]
    fn contains_incident_works() {
        let records = vec![
            IncidentHistoryRecord::new(
                sequence(1),
                empty_incident(9),
                IncidentHistoryEvent::Detected,
                None,
            ),
        ];

        assert!(contains_incident(records, incident_id(9)));
    }

    #[test]
    fn contains_incident_returns_false_for_missing_incident() {
        let records = vec![
            IncidentHistoryRecord::new(
                sequence(1),
                empty_incident(9),
                IncidentHistoryEvent::Detected,
                None,
            ),
        ];

        assert!(!contains_incident(records, incident_id(10)));
    }

    #[test]
    fn record_helpers_construct_expected_events() {
        let incident = empty_incident(1);

        let detected = record_detected(
            sequence(1),
            incident.clone(),
            None,
        );

        let recovery = record_recovery_started(
            sequence(2),
            incident.clone(),
            None,
        );

        let completed = record_recovery_completed(
            sequence(3),
            incident.clone(),
            None,
        );

        let verified = record_verification_completed(
            sequence(4),
            incident,
            None,
        );

        assert_eq!(
            detected.event(),
            IncidentHistoryEvent::Detected
        );

        assert_eq!(
            recovery.event(),
            IncidentHistoryEvent::RecoveryStarted
        );

        assert_eq!(
            completed.event(),
            IncidentHistoryEvent::RecoveryCompleted
        );

        assert_eq!(
            verified.event(),
            IncidentHistoryEvent::VerificationCompleted
        );
    }

    #[test]
    fn terminal_events_are_classified_correctly() {
        assert!(IncidentHistoryEvent::Resolved.is_terminal());
        assert!(IncidentHistoryEvent::Rejected.is_terminal());
        assert!(IncidentHistoryEvent::Superseded.is_terminal());

        assert!(!IncidentHistoryEvent::Detected.is_terminal());
        assert!(!IncidentHistoryEvent::Diagnosed.is_terminal());
    }

    #[test]
    fn start_and_completion_classification_is_descriptive() {
        assert!(IncidentHistoryEvent::RecoveryStarted.is_start());
        assert!(IncidentHistoryEvent::RecoveryCompleted.is_completion());

        assert!(!IncidentHistoryEvent::RecoveryStarted.is_completion());
        assert!(!IncidentHistoryEvent::RecoveryCompleted.is_start());
    }

    #[test]
    fn structural_validation_delegates_to_incident() {
        let record = IncidentHistoryRecord::new(
            sequence(1),
            empty_incident(1),
            IncidentHistoryEvent::Detected,
            None,
        );

        assert!(record.is_structurally_valid());
    }

    #[test]
    fn transformation_methods_preserve_original_record() {
        let original = IncidentHistoryRecord::new(
            sequence(1),
            empty_incident(1),
            IncidentHistoryEvent::Detected,
            None,
        );

        let changed = original.with_event(
            IncidentHistoryEvent::Resolved,
        );

        assert_eq!(
            original.event(),
            IncidentHistoryEvent::Detected
        );

        assert_eq!(
            changed.event(),
            IncidentHistoryEvent::Resolved
        );
    }

    #[test]
    fn producer_time_is_preserved() {
        let time = ProducerTime::new(
            1_000,
            123_456_789,
        )
        .expect("valid time");

        let record = IncidentHistoryRecord::new(
            sequence(1),
            empty_incident(1),
            IncidentHistoryEvent::Detected,
            Some(time),
        );

        assert_eq!(record.producer_time(), Some(time));
    }

    #[test]
    fn into_parts_round_trips() {
        let incident = empty_incident(42);

        let record = IncidentHistoryRecord::new(
            sequence(7),
            incident,
            IncidentHistoryEvent::Diagnosed,
            None,
        );

        let (sequence, incident, event, time) =
            record.into_parts();

        assert_eq!(sequence.value(), 7);
        assert_eq!(incident.id().value(), 42);
        assert_eq!(
            event,
            IncidentHistoryEvent::Diagnosed
        );
        assert_eq!(time, None);
    }
}