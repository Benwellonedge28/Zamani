
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Health and liveness monitoring for Zamani distributed participants.
//!
//! Path: `src/distributed/membership/health.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Track authenticated participant sessions, heartbeat freshness, suspected
//! failures, lease expiration, and recovery.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Local health-monitoring policy.
//! - Authenticated-session observations.
//! - Heartbeat sequence validation.
//! - Stale-session and replay detection.
//! - Failure suspicion based on monotonic local time.
//! - Recovery confirmation based on consecutive heartbeats.
//! - Health snapshots for consumers.
//!
//! This module does not own:
//!
//! - Participant identity.
//! - Membership records or membership generations.
//! - Participant discovery.
//! - Network communication or heartbeat scheduling.
//! - Peer authentication or authorization.
//! - Capability evidence or resource allocation.
//! - Placement, partitioning, routing, or scheduling.
//! - Task retry, checkpointing, or recovery execution.
//! - Distributed clocks or global time.
//! - ZUIR, ZCIR, quantum::ir, HDLIR, or backend implementations.
//!
//! # Authoritative dependencies
//!
//! - `distributed/ids.rs` owns `ParticipantId`.
//! - `distributed/membership/registry.rs` owns membership records,
//!   membership generations, and membership leases.
//! - `distributed/transport/traits.rs` owns `AuthenticatedPeer`.
//! - The transport provider owns heartbeat delivery and connection/session
//!   establishment.
//! - The security subsystem owns authentication and authorization.
//! - `grammar/spec/distributed.md` owns normative failure and recovery
//!   semantics.
//!
//! # Trust boundary
//!
//! A heartbeat is accepted only when supplied with an `AuthenticatedPeer`
//! associated with the current transport session.
//!
//! `AuthenticatedPeer` is evidence created by a transport provider after
//! authentication. Its existence does not grant authorization to execute
//! workloads.
//!
//! This module does not authenticate peers and must not be called with
//! fabricated authentication evidence from an untrusted source.
//!
//! # Time model
//!
//! All elapsed-time calculations use `std::time::Instant`.
//!
//! Remote timestamps are deliberately not accepted. Participants do not
//! need synchronized physical clocks.
//!
//! # Recovery
//!
//! A participant that becomes suspected must produce the configured number
//! of consecutive, strictly increasing heartbeat observations before the
//! registry is returned to Active.
//!
//! During recovery, heartbeats update this monitor's local observation but
//! do not prematurely renew the registry lease or expose the participant as
//! Active.
//!
//! # Scalability
//!
//! No application-defined maximum is imposed on participants, sessions,
//! heartbeat sequence values, or health observations.
//!
//! Memory, address-space, identifier representation, and configured
//! operational budgets remain finite and must be handled by their respective
//! authorities.
//!
//! # Concurrency
//!
//! A mutex protects health-session observations. Registry mutations use the
//! registry's own synchronization and generation checks.
//!
//! Consumers receive owned health snapshots and do not hold internal locks.
//!
//! # Safety
//!
//! This implementation uses safe Rust only.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

use crate::distributed::ids::ParticipantId;
use crate::distributed::membership::registry::{
    MembershipRegistry,
    MembershipRegistryError,
    MembershipState,
    ParticipantRecord,
};
use crate::distributed::transport::traits::AuthenticatedPeer;

// =============================================================================
// ERRORS
// =============================================================================

/// Errors produced by participant health monitoring.
#[derive(Debug)]
#[non_exhaustive]
pub enum HealthMonitorError {
    /// The health policy contains an invalid value.
    InvalidPolicy {
        /// Explanation of the invalid policy.
        reason: String,
    },

    /// A heartbeat sequence must be non-zero.
    InvalidHeartbeatSequence,

    /// The participant has no authenticated health-monitoring session.
    SessionNotEstablished {
        /// Participant whose session is missing.
        participant_id: ParticipantId,
    },

    /// The supplied session is no longer the current session.
    StaleSession {
        /// Participant whose session was rejected.
        participant_id: ParticipantId,
    },

    /// A heartbeat is duplicated or older than the latest accepted heartbeat.
    StaleHeartbeat {
        /// Participant whose heartbeat was rejected.
        participant_id: ParticipantId,

        /// Last accepted sequence, if one exists.
        last_accepted: Option<u128>,

        /// Rejected sequence.
        received: u128,
    },

    /// The participant is not present in the authoritative registry.
    UnknownParticipant {
        /// Participant that could not be found.
        participant_id: ParticipantId,
    },

    /// A participant is leaving and must not be revived by a heartbeat.
    ParticipantLeaving {
        /// Participant that is leaving.
        participant_id: ParticipantId,
    },

    /// A recovery observation counter cannot be incremented.
    ObservationCounterExhausted,

    /// The health monitor synchronization lock was poisoned.
    LockPoisoned,

    /// An operation against the authoritative membership registry failed.
    Registry(MembershipRegistryError),
}

impl fmt::Display for HealthMonitorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPolicy { reason } => {
                write!(formatter, "invalid health-monitor policy: {reason}")
            }

            Self::InvalidHeartbeatSequence => {
                formatter.write_str("heartbeat sequence must be non-zero")
            }

            Self::SessionNotEstablished { participant_id } => {
                write!(
                    formatter,
                    "no authenticated health session exists for {participant_id}"
                )
            }

            Self::StaleSession { participant_id } => {
                write!(
                    formatter,
                    "heartbeat belongs to a stale session for {participant_id}"
                )
            }

            Self::StaleHeartbeat {
                participant_id,
                last_accepted,
                received,
            } => {
                write!(
                    formatter,
                    "stale heartbeat for {participant_id}: \
                     last accepted sequence {last_accepted:?}, \
                     received {received}"
                )
            }

            Self::UnknownParticipant { participant_id } => {
                write!(
                    formatter,
                    "health observation references unknown participant {participant_id}"
                )
            }

            Self::ParticipantLeaving { participant_id } => {
                write!(
                    formatter,
                    "participant {participant_id} is leaving and cannot be revived"
                )
            }

            Self::ObservationCounterExhausted => {
                formatter.write_str("health observation counter exhausted")
            }

            Self::LockPoisoned => {
                formatter.write_str("health-monitor synchronization was poisoned")
            }

            Self::Registry(error) => {
                write!(formatter, "membership registry operation failed: {error}")
            }
        }
    }
}

impl Error for HealthMonitorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Registry(error) => Some(error),
            _ => None,
        }
    }
}

impl From<MembershipRegistryError> for HealthMonitorError {
    fn from(error: MembershipRegistryError) -> Self {
        Self::Registry(error)
    }
}

impl<T> From<PoisonError<T>> for HealthMonitorError {
    fn from(_: PoisonError<T>) -> Self {
        Self::LockPoisoned
    }
}

/// Result type for health-monitor operations.
pub type HealthResult<T> = Result<T, HealthMonitorError>;

// =============================================================================
// POLICY
// =============================================================================

/// Configurable health-monitoring policy.
///
/// These values are operational decisions, not language-level semantic
/// limits or hardware limits.
///
/// `suspect_after` determines when missing heartbeats cause a participant
/// to become suspected.
///
/// `recovery_heartbeats` determines how many consecutive valid heartbeats
/// are required before a suspected participant becomes Active again.
///
/// Lease expiration remains owned by `MembershipRegistry`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HealthPolicy {
    suspect_after: Duration,
    recovery_heartbeats: usize,
}

impl HealthPolicy {
    /// Construct a validated health policy.
    ///
    /// # Errors
    ///
    /// Rejects a zero suspicion interval and a zero recovery threshold.
    pub fn new(
        suspect_after: Duration,
        recovery_heartbeats: usize,
    ) -> HealthResult<Self> {
        if suspect_after.is_zero() {
            return Err(HealthMonitorError::InvalidPolicy {
                reason: "suspect_after must be greater than zero".to_owned(),
            });
        }

        if recovery_heartbeats == 0 {
            return Err(HealthMonitorError::InvalidPolicy {
                reason: "recovery_heartbeats must be greater than zero".to_owned(),
            });
        }

        Ok(Self {
            suspect_after,
            recovery_heartbeats,
        })
    }

    /// Return the interval after which missing heartbeats cause suspicion.
    #[must_use]
    pub const fn suspect_after(self) -> Duration {
        self.suspect_after
    }

    /// Return the number of consecutive heartbeats required for recovery.
    #[must_use]
    pub const fn recovery_heartbeats(self) -> usize {
        self.recovery_heartbeats
    }
}

// =============================================================================
// SESSION IDENTITY
// =============================================================================

/// Opaque identifier for one authenticated transport session.
///
/// The transport provider supplies this identifier. It should identify a
/// connection/session incarnation, not merely a participant or endpoint.
///
/// A newly authenticated session replaces the previous session for the
/// participant. Heartbeats from the previous session are rejected.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PeerSessionId(String);

impl PeerSessionId {
    /// Construct a validated session identifier.
    ///
    /// Session identifiers are open-world strings. Their uniqueness and
    /// lifecycle are the responsibility of the transport provider.
    ///
    /// # Errors
    ///
    /// Rejects empty, whitespace-only, or control-character-containing IDs.
    pub fn new(value: impl Into<String>) -> HealthResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(HealthMonitorError::InvalidPolicy {
                reason: "session identifier cannot be empty".to_owned(),
            });
        }

        if value.chars().any(char::is_control) {
            return Err(HealthMonitorError::InvalidPolicy {
                reason: "session identifier contains control characters".to_owned(),
            });
        }

        Ok(Self(value))
    }

    /// Return the opaque session identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// =============================================================================
// HEALTH STATE
// =============================================================================

/// Health classification derived from local observations.
///
/// This classification is not a semantic capability and does not prove
/// that a participant can execute a particular workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HealthStatus {
    /// No authenticated health session has been established.
    Unknown,

    /// The participant has satisfied the configured recovery threshold
    /// and has a sufficiently recent heartbeat.
    Healthy,

    /// The participant has missed the configured heartbeat interval.
    Suspected,

    /// The participant's membership lease has expired.
    ///
    /// This means that the registry has no current lease evidence. It does
    /// not prove permanent physical failure.
    Unavailable,

    /// A participant is producing heartbeats but has not yet satisfied
    /// the recovery threshold.
    Recovering,

    /// The participant has initiated graceful departure.
    Leaving,
}

/// Owned health observation for one participant.
///
/// This is a point-in-time snapshot. It does not retain a registry lock.
#[derive(Debug, Clone)]
pub struct HealthObservation {
    participant_id: ParticipantId,
    status: HealthStatus,
    membership_state: MembershipState,
    membership_generation: u64,
    last_heartbeat: Option<Instant>,
    observation_age: Duration,
    last_sequence: Option<u128>,
    consecutive_heartbeats: usize,
}

impl HealthObservation {
    /// Return the logical participant identity.
    #[must_use]
    pub const fn participant_id(&self) -> ParticipantId {
        self.participant_id
    }

    /// Return the health classification.
    #[must_use]
    pub const fn status(&self) -> HealthStatus {
        self.status
    }

    /// Return the effective membership state at observation time.
    #[must_use]
    pub const fn membership_state(&self) -> MembershipState {
        self.membership_state
    }

    /// Return the membership record generation.
    #[must_use]
    pub const fn membership_generation(&self) -> u64 {
        self.membership_generation
    }

    /// Return the local monotonic time of the latest accepted heartbeat.
    ///
    /// Returns `None` if the current session has not produced a heartbeat.
    #[must_use]
    pub const fn last_heartbeat(&self) -> Option<Instant> {
        self.last_heartbeat
    }

    /// Return the elapsed time since the latest heartbeat or session start.
    #[must_use]
    pub const fn observation_age(&self) -> Duration {
        self.observation_age
    }

    /// Return the latest accepted sender heartbeat sequence.
    #[must_use]
    pub const fn last_sequence(&self) -> Option<u128> {
        self.last_sequence
    }

    /// Return the consecutive heartbeat count in the current recovery period.
    #[must_use]
    pub const fn consecutive_heartbeats(&self) -> usize {
        self.consecutive_heartbeats
    }
}

// =============================================================================
// INTERNAL SESSION STATE
// =============================================================================

#[derive(Debug)]
struct PeerSession {
    session_id: PeerSessionId,
    started_at: Instant,
    last_heartbeat: Option<Instant>,
    last_sequence: Option<u128>,
    consecutive_heartbeats: usize,
    suspected: bool,
}

impl PeerSession {
    fn new(session_id: PeerSessionId, now: Instant) -> Self {
        Self {
            session_id,
            started_at: now,
            last_heartbeat: None,
            last_sequence: None,
            consecutive_heartbeats: 0,
            suspected: true,
        }
    }

    fn last_observation(&self) -> Instant {
        self.last_heartbeat.unwrap_or(self.started_at)
    }
}

// =============================================================================
// HEALTH MONITOR
// =============================================================================

/// Thread-safe health monitor attached to the authoritative membership
/// registry.
///
/// Cloning a monitor shares the same session observations and policy.
///
/// The monitor performs no background polling and starts no threads.
/// The runtime or transport provider calls `observe_heartbeat` when an
/// authenticated heartbeat arrives and `evaluate` periodically according
/// to its configured operational schedule.
#[derive(Clone)]
pub struct HealthMonitor {
    registry: MembershipRegistry,
    policy: HealthPolicy,
    sessions: std::sync::Arc<Mutex<HashMap<ParticipantId, PeerSession>>>,
}

impl fmt::Debug for HealthMonitor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.sessions.lock() {
            Ok(sessions) => formatter
                .debug_struct("HealthMonitor")
                .field("policy", &self.policy)
                .field("tracked_sessions", &sessions.len())
                .finish(),

            Err(_) => formatter
                .debug_struct("HealthMonitor")
                .field("policy", &self.policy)
                .field("state", &"poisoned")
                .finish(),
        }
    }
}

impl HealthMonitor {
    /// Construct a health monitor over the authoritative membership registry.
    ///
    /// The registry is cloned as a shared handle. This does not create a
    /// second registry.
    #[must_use]
    pub fn new(
        registry: MembershipRegistry,
        policy: HealthPolicy,
    ) -> Self {
        Self {
            registry,
            policy,
            sessions: std::sync::Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Return the policy used by this monitor.
    #[must_use]
    pub const fn policy(&self) -> HealthPolicy {
        self.policy
    }

    /// Begin or replace an authenticated participant session.
    ///
    /// The supplied `AuthenticatedPeer` must have been created by a trusted
    /// transport provider after completing its actual authentication
    /// procedure. This method does not itself authenticate the peer.
    ///
    /// A new session starts in recovery state. The participant is marked
    /// Suspected in the authoritative registry until the configured
    /// heartbeat threshold is satisfied.
    ///
    /// Repeating the same session identifier is idempotent and does not
    /// reset its sequence or recovery progress.
    ///
    /// # Errors
    ///
    /// Returns an error if the participant is unknown, is leaving, or
    /// the registry cannot be updated.
    pub fn begin_authenticated_session(
        &self,
        peer: &AuthenticatedPeer,
        session_id: PeerSessionId,
    ) -> HealthResult<()> {
        let participant_id = peer.participant();
        let mut sessions = self.sessions.lock()?;

        if let Some(existing) = sessions.get(&participant_id) {
            if existing.session_id == session_id {
                return Ok(());
            }
        }

        let record = self
            .registry
            .get(participant_id)?
            .ok_or(HealthMonitorError::UnknownParticipant {
                participant_id,
            })?;

        if record.state() == MembershipState::Leaving {
            return Err(HealthMonitorError::ParticipantLeaving {
                participant_id,
            });
        }

        if record.state() != MembershipState::Suspected {
            self.registry.mark_suspected(
                participant_id,
                record.generation(),
            )?;
        }

        sessions.insert(
            participant_id,
            PeerSession::new(session_id, Instant::now()),
        );

        Ok(())
    }

    /// Accept an authenticated heartbeat observation.
    ///
    /// The sequence must be strictly greater than the last accepted sequence
    /// for the current session. Sequence values are sender-local monotonic
    /// counters; they are not timestamps.
    ///
    /// A heartbeat from an old session or a replayed sequence is rejected.
    ///
    /// A participant in recovery remains Suspected until the configured
    /// number of consecutive heartbeats has been observed. Only then is the
    /// authoritative registry lease renewed and the participant returned
    /// to Active.
    ///
    /// # Errors
    ///
    /// Returns an error for an unknown session, stale session, replayed
    /// heartbeat, unknown participant, leaving participant, or registry
    /// generation conflict.
    pub fn observe_heartbeat(
        &self,
        peer: &AuthenticatedPeer,
        session_id: &PeerSessionId,
        sequence: u128,
    ) -> HealthResult<HealthObservation> {
        if sequence == 0 {
            return Err(HealthMonitorError::InvalidHeartbeatSequence);
        }

        let participant_id = peer.participant();
        let now = Instant::now();
        let mut sessions = self.sessions.lock()?;

        let session = sessions
            .get_mut(&participant_id)
            .ok_or(HealthMonitorError::SessionNotEstablished {
                participant_id,
            })?;

        if &session.session_id != session_id {
            return Err(HealthMonitorError::StaleSession {
                participant_id,
            });
        }

        if session
            .last_sequence
            .is_some_and(|last| sequence <= last)
        {
            return Err(HealthMonitorError::StaleHeartbeat {
                participant_id,
                last_accepted: session.last_sequence,
                received: sequence,
            });
        }

        let mut record = self
            .registry
            .get(participant_id)?
            .ok_or(HealthMonitorError::UnknownParticipant {
                participant_id,
            })?;

        if record.state() == MembershipState::Leaving {
            return Err(HealthMonitorError::ParticipantLeaving {
                participant_id,
            });
        }

        // Detect a missed-heartbeat interval even if evaluate() has not
        // yet been called by the runtime.
        let previous_age = now.saturating_duration_since(
            session.last_observation(),
        );

        if !session.suspected
            && previous_age >= self.policy.suspect_after
        {
            if record.state() != MembershipState::Suspected {
                record = self.registry.mark_suspected(
                    participant_id,
                    record.generation(),
                )?;
            }

            session.suspected = true;
            session.consecutive_heartbeats = 0;
        }

        let next_count = if session.suspected {
            session
                .consecutive_heartbeats
                .checked_add(1)
                .ok_or(HealthMonitorError::ObservationCounterExhausted)?
        } else {
            self.policy.recovery_heartbeats
        };

        let recovery_complete =
            next_count >= self.policy.recovery_heartbeats;

        // During recovery, do not renew the registry lease until the
        // recovery threshold is reached. This prevents a single heartbeat
        // from prematurely making an unavailable participant Active.
        if recovery_complete {
            record = self.registry.heartbeat(
                participant_id,
                record.generation(),
            )?;

            session.suspected = false;
            session.consecutive_heartbeats =
                self.policy.recovery_heartbeats;
        } else {
            session.consecutive_heartbeats = next_count;
        }

        session.last_sequence = Some(sequence);
        session.last_heartbeat = Some(now);

        let effective_state = record.effective_state_at(now);

        let status = if effective_state == MembershipState::Leaving {
            HealthStatus::Leaving
        } else if !recovery_complete {
            HealthStatus::Recovering
        } else if effective_state == MembershipState::Unavailable {
            HealthStatus::Unavailable
        } else if effective_state == MembershipState::Suspected {
            HealthStatus::Suspected
        } else {
            HealthStatus::Healthy
        };

        Ok(HealthObservation {
            participant_id,
            status,
            membership_state: effective_state,
            membership_generation: record.generation(),
            last_heartbeat: session.last_heartbeat,
            observation_age: Duration::ZERO,
            last_sequence: session.last_sequence,
            consecutive_heartbeats: session.consecutive_heartbeats,
        })
    }

    /// Evaluate the health of all registered participants at the current
    /// local monotonic time.
    ///
    /// The caller controls how frequently this method is invoked. It does
    /// not create a timer, background thread, or runtime dependency.
    ///
    /// # Errors
    ///
    /// Returns an error if registry access, registry mutation, or internal
    /// synchronization fails.
    pub fn evaluate(&self) -> HealthResult<Vec<HealthObservation>> {
        self.evaluate_at(Instant::now())
    }

    /// Evaluate health using an explicitly supplied local monotonic instant.
    ///
    /// This method is useful for deterministic tests and runtime-controlled
    /// evaluation. The supplied instant must belong to the same local
    /// monotonic clock domain as the registry records.
    ///
    /// Remote wall-clock timestamps must never be supplied here.
    pub fn evaluate_at(
        &self,
        now: Instant,
    ) -> HealthResult<Vec<HealthObservation>> {
        let records = self.registry.snapshot()?;
        let mut sessions = self.sessions.lock()?;
        let mut observations = Vec::with_capacity(records.len());

        for mut record in records {
            let participant_id = record.participant_id();

            let Some(session) = sessions.get_mut(&participant_id) else {
                let effective_state = record.effective_state_at(now);

                observations.push(HealthObservation {
                    participant_id,
                    status: if effective_state == MembershipState::Leaving {
                        HealthStatus::Leaving
                    } else {
                        HealthStatus::Unknown
                    },
                    membership_state: effective_state,
                    membership_generation: record.generation(),
                    last_heartbeat: None,
                    observation_age: Duration::ZERO,
                    last_sequence: None,
                    consecutive_heartbeats: 0,
                });

                continue;
            };

            if record.state() == MembershipState::Leaving {
                observations.push(HealthObservation {
                    participant_id,
                    status: HealthStatus::Leaving,
                    membership_state: MembershipState::Leaving,
                    membership_generation: record.generation(),
                    last_heartbeat: session.last_heartbeat,
                    observation_age: now.saturating_duration_since(
                        session.last_observation(),
                    ),
                    last_sequence: session.last_sequence,
                    consecutive_heartbeats: session.consecutive_heartbeats,
                });

                continue;
            }

            let age = now.saturating_duration_since(
                session.last_observation(),
            );

            // A previously healthy participant becomes suspected after
            // missing its configured heartbeat interval.
            if !session.suspected && age >= self.policy.suspect_after {
                if record.state() != MembershipState::Suspected {
                    record = self.registry.mark_suspected(
                        participant_id,
                        record.generation(),
                    )?;
                }

                session.suspected = true;
                session.consecutive_heartbeats = 0;
            }

            let effective_state = record.effective_state_at(now);

            let status = if effective_state == MembershipState::Leaving {
                HealthStatus::Leaving
            } else if effective_state == MembershipState::Unavailable {
                HealthStatus::Unavailable
            } else if session.suspected && session.last_heartbeat.is_some() {
                HealthStatus::Recovering
            } else if age >= self.policy.suspect_after {
                HealthStatus::Suspected
            } else if session.suspected {
                HealthStatus::Recovering
            } else {
                HealthStatus::Healthy
            };

            observations.push(HealthObservation {
                participant_id,
                status,
                membership_state: effective_state,
                membership_generation: record.generation(),
                last_heartbeat: session.last_heartbeat,
                observation_age: age,
                last_sequence: session.last_sequence,
                consecutive_heartbeats: session.consecutive_heartbeats,
            });
        }

        // Registry snapshots are already deterministic, but retain this
        // ordering guarantee at the health API boundary.
        observations.sort_by_key(|observation| {
            observation.participant_id.to_string()
        });

        Ok(observations)
    }

    /// Return the current health observation for one participant.
    ///
    /// Returns `None` if the participant is not registered.
    pub fn observe(
        &self,
        participant_id: ParticipantId,
    ) -> HealthResult<Option<HealthObservation>> {
        let observations = self.evaluate()?;

        Ok(observations
            .into_iter()
            .find(|observation| {
                observation.participant_id == participant_id
            }))
    }

    /// Remove local session-tracking state for a participant.
    ///
    /// This does not remove the participant from membership and does not
    /// mark the participant as gracefully leaving.
    ///
    /// It is intended for transport-session teardown and controlled runtime
    /// cleanup. A later session must be explicitly authenticated and begun.
    pub fn forget_session(
        &self,
        participant_id: ParticipantId,
    ) -> HealthResult<bool> {
        Ok(self.sessions.lock()?.remove(&participant_id).is_some())
    }

    /// Return the number of participants with tracked authenticated sessions.
    pub fn tracked_session_count(&self) -> HealthResult<usize> {
        Ok(self.sessions.lock()?.len())
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;
    use std::sync::Arc;

    use crate::distributed::ids::IdAllocator;
    use crate::distributed::membership::registry::{
        ParticipantEndpoint,
        ParticipantRegistration,
    };
    use crate::distributed::model::participant::{
        Participant,
        ParticipantRole,
    };
    use crate::distributed::transport::messages::ProtocolVersion;

    fn setup(
        lease: Duration,
        policy: HealthPolicy,
    ) -> (
        MembershipRegistry,
        HealthMonitor,
        AuthenticatedPeer,
        PeerSessionId,
    ) {
        let registry = MembershipRegistry::new();
        let allocator = IdAllocator::new();
        let participant_id = allocator.participant_id().unwrap();

        let participant = Participant::new(
            participant_id,
            ParticipantRole::new("worker").unwrap(),
        );

        let endpoint = ParticipantEndpoint::new(
            "memory://local/health-test",
            "test-provider",
        )
        .unwrap();

        let registration = ParticipantRegistration::new(
            participant,
            vec![endpoint],
            HashSet::from([ProtocolVersion::new(1, 0, 0)]),
            Arc::new(
                crate::frontend::semantic::capabilities::evidence::CapabilityEvidenceSet::default(),
            ),
            lease,
        )
        .unwrap();

        registry.register(registration).unwrap();

        let peer = AuthenticatedPeer::new(
            participant_id,
            "test-authentication",
            None,
        )
        .unwrap();

        let session_id = PeerSessionId::new("test-session-1").unwrap();
        let monitor = HealthMonitor::new(registry.clone(), policy);

        (registry, monitor, peer, session_id)
    }

    #[test]
    fn rejects_zero_suspicion_interval() {
        assert!(HealthPolicy::new(Duration::ZERO, 1).is_err());
    }

    #[test]
    fn rejects_zero_recovery_threshold() {
        assert!(
            HealthPolicy::new(Duration::from_secs(1), 0).is_err()
        );
    }

    #[test]
    fn session_starts_suspected_and_requires_recovery() {
        let policy = HealthPolicy::new(
            Duration::from_secs(5),
            2,
        )
        .unwrap();

        let (registry, monitor, peer, session_id) =
            setup(Duration::from_secs(30), policy);

        monitor
            .begin_authenticated_session(&peer, session_id.clone())
            .unwrap();

        let first = monitor
            .observe_heartbeat(&peer, &session_id, 1)
            .unwrap();

        assert_eq!(first.status(), HealthStatus::Recovering);

        let record = registry
            .get(peer.participant())
            .unwrap()
            .unwrap();

        assert_eq!(record.state(), MembershipState::Suspected);

        let second = monitor
            .observe_heartbeat(&peer, &session_id, 2)
            .unwrap();

        assert_eq!(second.status(), HealthStatus::Healthy);

        let record = registry
            .get(peer.participant())
            .unwrap()
            .unwrap();

        assert_eq!(record.state(), MembershipState::Active);
    }

    #[test]
    fn rejects_replayed_heartbeat_sequence() {
        let policy = HealthPolicy::new(
            Duration::from_secs(5),
            1,
        )
        .unwrap();

        let (_, monitor, peer, session_id) =
            setup(Duration::from_secs(30), policy);

        monitor
            .begin_authenticated_session(&peer, session_id.clone())
            .unwrap();

        monitor
            .observe_heartbeat(&peer, &session_id, 10)
            .unwrap();

        assert!(matches!(
            monitor.observe_heartbeat(&peer, &session_id, 10),
            Err(HealthMonitorError::StaleHeartbeat { .. })
        ));
    }

    #[test]
    fn rejects_out_of_order_heartbeat_sequence() {
        let policy = HealthPolicy::new(
            Duration::from_secs(5),
            1,
        )
        .unwrap();

        let (_, monitor, peer, session_id) =
            setup(Duration::from_secs(30), policy);

        monitor
            .begin_authenticated_session(&peer, session_id.clone())
            .unwrap();

        monitor
            .observe_heartbeat(&peer, &session_id, 20)
            .unwrap();

        assert!(matches!(
            monitor.observe_heartbeat(&peer, &session_id, 19),
            Err(HealthMonitorError::StaleHeartbeat { .. })
        ));
    }

    #[test]
    fn rejects_heartbeat_from_previous_session() {
        let policy = HealthPolicy::new(
            Duration::from_secs(5),
            1,
        )
        .unwrap();

        let (_, monitor, peer, first_session) =
            setup(Duration::from_secs(30), policy);

        monitor
            .begin_authenticated_session(&peer, first_session.clone())
            .unwrap();

        let second_session =
            PeerSessionId::new("test-session-2").unwrap();

        monitor
            .begin_authenticated_session(&peer, second_session.clone())
            .unwrap();

        assert!(matches!(
            monitor.observe_heartbeat(&peer, &first_session, 1),
            Err(HealthMonitorError::StaleSession { .. })
        ));

        assert!(monitor
            .observe_heartbeat(&peer, &second_session, 1)
            .is_ok());
    }

    #[test]
    fn missed_heartbeat_marks_participant_suspected() {
        let policy = HealthPolicy::new(
            Duration::from_secs(5),
            1,
        )
        .unwrap();

        let (registry, monitor, peer, session_id) =
            setup(Duration::from_secs(30), policy);

        monitor
            .begin_authenticated_session(&peer, session_id.clone())
            .unwrap();

        monitor
            .observe_heartbeat(&peer, &session_id, 1)
            .unwrap();

        let future = Instant::now()
            .checked_add(Duration::from_secs(6))
            .unwrap();

        let observations = monitor.evaluate_at(future).unwrap();

        let observation = observations
            .iter()
            .find(|item| item.participant_id() == peer.participant())
            .unwrap();

        assert_eq!(observation.status(), HealthStatus::Suspected);

        let record = registry
            .get(peer.participant())
            .unwrap()
            .unwrap();

        assert_eq!(record.state(), MembershipState::Suspected);
    }

    #[test]
    fn expired_membership_lease_is_unavailable() {
        let policy = HealthPolicy::new(
            Duration::from_secs(2),
            1,
        )
        .unwrap();

        let (registry, monitor, peer, session_id) =
            setup(Duration::from_secs(5), policy);

        monitor
            .begin_authenticated_session(&peer, session_id)
            .unwrap();

        let future = Instant::now()
            .checked_add(Duration::from_secs(10))
            .unwrap();

        let observations = monitor.evaluate_at(future).unwrap();

        let observation = observations
            .iter()
            .find(|item| item.participant_id() == peer.participant())
            .unwrap();

        assert_eq!(observation.status(), HealthStatus::Unavailable);

        let record = registry
            .get(peer.participant())
            .unwrap()
            .unwrap();

        assert_eq!(
            record.effective_state_at(future),
            MembershipState::Unavailable
        );
    }

    #[test]
    fn forgetting_session_does_not_remove_membership() {
        let policy = HealthPolicy::new(
            Duration::from_secs(5),
            1,
        )
        .unwrap();

        let (registry, monitor, peer, session_id) =
            setup(Duration::from_secs(30), policy);

        monitor
            .begin_authenticated_session(&peer, session_id)
            .unwrap();

        assert!(monitor
            .forget_session(peer.participant())
            .unwrap());

        assert!(registry
            .get(peer.participant())
            .unwrap()
            .is_some());

        assert_eq!(monitor.tracked_session_count().unwrap(), 0);
    }
}
