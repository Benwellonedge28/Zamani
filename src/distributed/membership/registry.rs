
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Authoritative membership registry for Zamani distributed computation.
//!
//! Path: src/distributed/membership/registry.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Maintain the current membership view of logical distributed participants.
//!
//! The registry tracks:
//! - Logical participant descriptors.
//! - One or more physical communication endpoints.
//! - Supported transport protocol versions.
//! - Canonical semantic capability evidence.
//! - Membership lifecycle.
//! - Heartbeat and lease observations.
//! - Registration generations.
//!
//! # Ownership
//!
//! This module owns membership records and their local synchronization.
//!
//! It does not own:
//! - Logical participant identity.
//! - Participant semantic meaning.
//! - Capability definitions or requirements.
//! - Authentication or authorization.
//! - Physical network communication.
//! - Service discovery implementations.
//! - Resource allocation.
//! - Placement, scheduling, partitioning or routing.
//! - Execution lifecycle.
//! - Checkpointing or recovery.
//! - ZUIR, ZCIR, quantum::ir, HDLIR or backend implementations.
//!
//! # Authoritative dependencies
//!
//! - distributed/ids.rs:
//!   ParticipantId.
//! - distributed/model/participant.rs:
//!   Participant.
//! - frontend/semantic/capabilities/evidence:
//!   CapabilityEvidenceSet.
//! - distributed/transport/messages.rs:
//!   ProtocolVersion.
//! - distributed/membership/health.rs:
//!   Future health-monitor integration.
//! - distributed/membership/discovery.rs:
//!   Future discovery-provider integration.
//! - distributed/security/:
//!   Authentication and authorization integration.
//!
//! # Trust boundary
//!
//! Registration does not authenticate a participant.
//!
//! A discovered endpoint is untrusted until the security subsystem
//! independently establishes its identity and authorization.
//!
//! Capability evidence stored here remains evidence, not proof of trust.
//!
//! # Scalability
//!
//! There are no application-defined universal limits on:
//! - Participants.
//! - Endpoints.
//! - Protocol versions.
//! - Membership updates.
//!
//! Actual capacity is constrained by available memory, address space,
//! configured operational budgets and the underlying platform.
//!
//! # Concurrency
//!
//! The registry uses RwLock and immutable Arc-backed snapshots.
//! Readers can retain snapshots without holding the registry lock.
//!
//! Mutations are serialized and generation-checked.
//!
//! # Liveness
//!
//! Liveness is lease-based and uses monotonic time.
//! A lease expiration indicates that a participant has not renewed
//! its membership in time; it does not prove permanent failure.
//!
//! # Safety
//!
//! This implementation uses safe Rust only.

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::distributed::ids::ParticipantId;
use crate::distributed::model::participant::Participant;
use crate::distributed::transport::messages::ProtocolVersion;
use crate::frontend::semantic::capabilities::evidence::CapabilityEvidenceSet;

// =============================================================================
// ERRORS
// =============================================================================

/// Errors produced by membership registry operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MembershipRegistryError {
    /// An endpoint descriptor is invalid.
    InvalidEndpoint {
        /// Description of the validation failure.
        reason: String,
    },

    /// A participant has already been registered.
    DuplicateParticipant {
        /// Existing participant identity.
        participant_id: ParticipantId,
    },

    /// A participant does not exist in the registry.
    UnknownParticipant {
        /// Requested participant identity.
        participant_id: ParticipantId,
    },

    /// A registration update used an outdated generation.
    GenerationConflict {
        /// Participant identity.
        participant_id: ParticipantId,

        /// Expected generation.
        expected: u64,

        /// Actual generation.
        actual: u64,
    },

    /// The registry generation cannot be incremented.
    GenerationExhausted,

    /// A lease duration must be non-zero.
    InvalidLease,

    /// A participant has no supported protocol versions.
    EmptyProtocolVersions,

    /// The registry's synchronization lock was poisoned.
    LockPoisoned,
}

impl fmt::Display for MembershipRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEndpoint { reason } => {
                write!(f, "invalid membership endpoint: {reason}")
            }

            Self::DuplicateParticipant { participant_id } => {
                write!(
                    f,
                    "participant {participant_id} is already registered"
                )
            }

            Self::UnknownParticipant { participant_id } => {
                write!(
                    f,
                    "participant {participant_id} is not registered"
                )
            }

            Self::GenerationConflict {
                participant_id,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "membership generation conflict for {participant_id}: \
                     expected {expected}, found {actual}"
                )
            }

            Self::GenerationExhausted => {
                f.write_str("membership generation exhausted")
            }

            Self::InvalidLease => {
                f.write_str("membership lease must be non-zero")
            }

            Self::EmptyProtocolVersions => {
                f.write_str("participant must advertise at least one protocol version")
            }

            Self::LockPoisoned => {
                f.write_str("membership registry synchronization was poisoned")
            }
        }
    }
}

impl Error for MembershipRegistryError {}

impl<T> From<std::sync::PoisonError<T>> for MembershipRegistryError {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Self::LockPoisoned
    }
}

/// Result type for membership operations.
pub type MembershipResult<T> =
    Result<T, MembershipRegistryError>;

// =============================================================================
// ENDPOINT MODEL
// =============================================================================

/// A physical communication endpoint.
///
/// The URI is intentionally represented as an open-world string.
/// The registry does not assume IP addresses, ports, DNS, TCP, QUIC,
/// shared memory, quantum networks or any particular transport.
///
/// The networking subsystem validates and interprets the URI.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParticipantEndpoint {
    uri: String,
    transport_provider: String,
}

impl ParticipantEndpoint {
    /// Construct an endpoint descriptor.
    ///
    /// # Errors
    ///
    /// Rejects empty values and control characters.
    pub fn new(
        uri: impl Into<String>,
        transport_provider: impl Into<String>,
    ) -> MembershipResult<Self> {
        let uri = uri.into();
        let transport_provider = transport_provider.into();

        validate_endpoint_text(&uri, "URI")?;
        validate_endpoint_text(&transport_provider, "transport provider")?;

        Ok(Self {
            uri,
            transport_provider,
        })
    }

    /// Return the endpoint URI.
    #[must_use]
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Return the transport provider identifier.
    #[must_use]
    pub fn transport_provider(&self) -> &str {
        &self.transport_provider
    }
}

fn validate_endpoint_text(
    value: &str,
    field: &str,
) -> MembershipResult<()> {
    if value.trim().is_empty() {
        return Err(MembershipRegistryError::InvalidEndpoint {
            reason: format!("{field} cannot be empty"),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(MembershipRegistryError::InvalidEndpoint {
            reason: format!("{field} contains control characters"),
        });
    }

    Ok(())
}

// =============================================================================
// MEMBERSHIP STATE
// =============================================================================

/// Current membership lifecycle state.
///
/// These states describe registry observations, not proof of permanent
/// physical availability or failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MembershipState {
    /// Participant is registered and its lease is valid.
    Active,

    /// Participant is suspected of being unreachable.
    Suspected,

    /// Participant's membership lease has expired.
    Unavailable,

    /// Participant has initiated graceful departure.
    Leaving,
}

// =============================================================================
// REGISTRATION INPUT
// =============================================================================

/// Complete registration information for a logical participant.
///
/// This is a registration input, not an authenticated identity.
///
/// Security verification must occur independently before a caller
/// permits the participant to execute workloads.
#[derive(Debug, Clone)]
pub struct ParticipantRegistration {
    participant: Participant,
    endpoints: Vec<ParticipantEndpoint>,
    protocol_versions: HashSet<ProtocolVersion>,
    capabilities: Arc<CapabilityEvidenceSet>,
    lease_duration: Duration,
}

impl ParticipantRegistration {
    /// Construct registration information.
    ///
    /// Multiple endpoints and protocol versions are supported.
    ///
    /// # Errors
    ///
    /// Rejects zero-duration leases and empty protocol-version sets.
    pub fn new(
        participant: Participant,
        endpoints: Vec<ParticipantEndpoint>,
        protocol_versions: HashSet<ProtocolVersion>,
        capabilities: Arc<CapabilityEvidenceSet>,
        lease_duration: Duration,
    ) -> MembershipResult<Self> {
        if lease_duration.is_zero() {
            return Err(MembershipRegistryError::InvalidLease);
        }

        if protocol_versions.is_empty() {
            return Err(
                MembershipRegistryError::EmptyProtocolVersions
            );
        }

        Ok(Self {
            participant,
            endpoints,
            protocol_versions,
            capabilities,
            lease_duration,
        })
    }

    /// Return the logical participant.
    #[must_use]
    pub fn participant(&self) -> &Participant {
        &self.participant
    }

    /// Return the advertised endpoints.
    #[must_use]
    pub fn endpoints(&self) -> &[ParticipantEndpoint] {
        &self.endpoints
    }

    /// Return supported protocol versions.
    #[must_use]
    pub fn protocol_versions(&self) -> &HashSet<ProtocolVersion> {
        &self.protocol_versions
    }

    /// Return canonical capability evidence.
    #[must_use]
    pub fn capabilities(&self) -> &Arc<CapabilityEvidenceSet> {
        &self.capabilities
    }

    /// Return the membership lease duration.
    #[must_use]
    pub fn lease_duration(&self) -> Duration {
        self.lease_duration
    }
}

// =============================================================================
// IMMUTABLE MEMBERSHIP RECORD
// =============================================================================

/// Immutable snapshot of one registered participant.
///
/// Records are returned through Arc so that consumers can retain a
/// consistent view without holding the registry synchronization lock.
#[derive(Debug)]
pub struct ParticipantRecord {
    participant: Participant,
    endpoints: Vec<ParticipantEndpoint>,
    protocol_versions: HashSet<ProtocolVersion>,
    capabilities: Arc<CapabilityEvidenceSet>,
    state: MembershipState,
    generation: u64,
    registered_at: Instant,
    last_seen: Instant,
    lease_expires_at: Instant,
    lease_duration: Duration,
}

impl ParticipantRecord {
    /// Return the logical participant descriptor.
    #[must_use]
    pub fn participant(&self) -> &Participant {
        &self.participant
    }

    /// Return the participant identity.
    #[must_use]
    pub fn participant_id(&self) -> ParticipantId {
        self.participant.id()
    }

    /// Return registered endpoints.
    #[must_use]
    pub fn endpoints(&self) -> &[ParticipantEndpoint] {
        &self.endpoints
    }

    /// Return supported protocol versions.
    #[must_use]
    pub fn protocol_versions(&self) -> &HashSet<ProtocolVersion> {
        &self.protocol_versions
    }

    /// Return canonical semantic capability evidence.
    #[must_use]
    pub fn capabilities(&self) -> &Arc<CapabilityEvidenceSet> {
        &self.capabilities
    }

    /// Return the membership state.
    #[must_use]
    pub fn state(&self) -> MembershipState {
        self.state
    }

    /// Return the current registration generation.
    #[must_use]
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Return the original registration time.
    #[must_use]
    pub fn registered_at(&self) -> Instant {
        self.registered_at
    }

    /// Return the last heartbeat time.
    #[must_use]
    pub fn last_seen(&self) -> Instant {
        self.last_seen
    }

    /// Return the lease expiration instant.
    #[must_use]
    pub fn lease_expires_at(&self) -> Instant {
        self.lease_expires_at
    }

    /// Return the configured lease duration.
    #[must_use]
    pub fn lease_duration(&self) -> Duration {
        self.lease_duration
    }

    /// Determine whether the lease is currently valid.
    ///
    /// This checks the local monotonic clock. It does not establish
    /// remote availability or authenticated reachability.
    #[must_use]
    pub fn lease_is_valid_at(&self, now: Instant) -> bool {
        now < self.lease_expires_at
    }

    /// Determine the effective state at a given instant.
    ///
    /// An expired active or suspected participant becomes unavailable.
    #[must_use]
    pub fn effective_state_at(&self, now: Instant) -> MembershipState {
        if self.state == MembershipState::Leaving {
            return MembershipState::Leaving;
        }

        if !self.lease_is_valid_at(now) {
            return MembershipState::Unavailable;
        }

        self.state
    }
}

// =============================================================================
// REGISTRY
// =============================================================================

/// Authoritative local membership registry.
///
/// The registry maintains logical membership observations. It does not
/// perform network discovery, peer authentication, resource allocation,
/// or remote execution.
///
/// Create one registry per explicitly owned runtime or membership domain.
/// Clones share the same registry state.
#[derive(Clone, Default)]
pub struct MembershipRegistry {
    records: Arc<RwLock<HashMap<ParticipantId, Arc<ParticipantRecord>>>>,
}

impl fmt::Debug for MembershipRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.records.read() {
            Ok(records) => f
                .debug_struct("MembershipRegistry")
                .field("participant_count", &records.len())
                .finish(),

            Err(_) => f
                .debug_struct("MembershipRegistry")
                .field("state", &"poisoned")
                .finish(),
        }
    }
}

impl MembershipRegistry {
    /// Create an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of registered logical participants.
    pub fn len(&self) -> MembershipResult<usize> {
        Ok(self.records.read()?.len())
    }

    /// Determine whether the registry is empty.
    pub fn is_empty(&self) -> MembershipResult<bool> {
        Ok(self.records.read()?.is_empty())
    }

    /// Register a new participant.
    ///
    /// Existing identities are never silently overwritten.
    ///
    /// The initial generation is one.
    ///
    /// # Errors
    ///
    /// Returns DuplicateParticipant when the logical identity already
    /// exists. Use update with the current generation to replace it.
    pub fn register(
        &self,
        registration: ParticipantRegistration,
    ) -> MembershipResult<Arc<ParticipantRecord>> {
        let id = registration.participant.id();
        let mut records = self.records.write()?;

        if records.contains_key(&id) {
            return Err(
                MembershipRegistryError::DuplicateParticipant {
                    participant_id: id,
                },
            );
        }

        let now = Instant::now();
        let expiration = now
            .checked_add(registration.lease_duration)
            .ok_or(MembershipRegistryError::InvalidLease)?;

        let record = Arc::new(ParticipantRecord {
            participant: registration.participant,
            endpoints: registration.endpoints,
            protocol_versions: registration.protocol_versions,
            capabilities: registration.capabilities,
            state: MembershipState::Active,
            generation: 1,
            registered_at: now,
            last_seen: now,
            lease_expires_at: expiration,
            lease_duration: registration.lease_duration,
        });

        records.insert(id, Arc::clone(&record));

        Ok(record)
    }

    /// Replace registration information using optimistic concurrency.
    ///
    /// `expected_generation` must match the currently registered
    /// participant generation.
    ///
    /// Every successful replacement increments the generation.
    ///
    /// This prevents a stale discovery provider or health observer
    /// from silently overwriting newer membership information.
    pub fn update(
        &self,
        registration: ParticipantRegistration,
        expected_generation: u64,
    ) -> MembershipResult<Arc<ParticipantRecord>> {
        let id = registration.participant.id();
        let mut records = self.records.write()?;

        let previous = records
            .get(&id)
            .ok_or(
                MembershipRegistryError::UnknownParticipant {
                    participant_id: id,
                },
            )?;

        if previous.generation != expected_generation {
            return Err(
                MembershipRegistryError::GenerationConflict {
                    participant_id: id,
                    expected: expected_generation,
                    actual: previous.generation,
                },
            );
        }

        let generation = previous
            .generation
            .checked_add(1)
            .ok_or(MembershipRegistryError::GenerationExhausted)?;

        let now = Instant::now();

        let expiration = now
            .checked_add(registration.lease_duration)
            .ok_or(MembershipRegistryError::InvalidLease)?;

        let record = Arc::new(ParticipantRecord {
            participant: registration.participant,
            endpoints: registration.endpoints,
            protocol_versions: registration.protocol_versions,
            capabilities: registration.capabilities,
            state: MembershipState::Active,
            generation,
            registered_at: previous.registered_at,
            last_seen: now,
            lease_expires_at: expiration,
            lease_duration: registration.lease_duration,
        });

        records.insert(id, Arc::clone(&record));

        Ok(record)
    }

    /// Retrieve a participant snapshot.
    ///
    /// Expired records remain observable as unavailable until explicitly
    /// removed or renewed.
    pub fn get(
        &self,
        id: ParticipantId,
    ) -> MembershipResult<Option<Arc<ParticipantRecord>>> {
        Ok(self.records.read()?.get(&id).cloned())
    }

    /// Return all current membership snapshots.
    ///
    /// Results are sorted by their canonical textual participant ID
    /// representation for deterministic iteration.
    pub fn snapshot(
        &self,
    ) -> MembershipResult<Vec<Arc<ParticipantRecord>>> {
        let records = self.records.read()?;

        let mut result: Vec<_> = records.values().cloned().collect();

        result.sort_by_key(|record| record.participant_id().to_string());

        Ok(result)
    }

    /// Renew a participant's membership lease.
    ///
    /// A heartbeat renews the lease and restores Active state.
    ///
    /// The generation check prevents a delayed heartbeat from reviving
    /// an obsolete registration.
    pub fn heartbeat(
        &self,
        id: ParticipantId,
        expected_generation: u64,
    ) -> MembershipResult<Arc<ParticipantRecord>> {
        let mut records = self.records.write()?;

        let previous = records
            .get(&id)
            .ok_or(
                MembershipRegistryError::UnknownParticipant {
                    participant_id: id,
                },
            )?;

        if previous.generation != expected_generation {
            return Err(
                MembershipRegistryError::GenerationConflict {
                    participant_id: id,
                    expected: expected_generation,
                    actual: previous.generation,
                },
            );
        }

        if previous.state == MembershipState::Leaving {
            return Err(
                MembershipRegistryError::GenerationConflict {
                    participant_id: id,
                    expected: expected_generation,
                    actual: previous.generation,
                },
            );
        }

        let now = Instant::now();

        let expiration = now
            .checked_add(previous.lease_duration)
            .ok_or(MembershipRegistryError::InvalidLease)?;

        let generation = previous
            .generation
            .checked_add(1)
            .ok_or(MembershipRegistryError::GenerationExhausted)?;

        let record = Arc::new(ParticipantRecord {
            participant: previous.participant.clone(),
            endpoints: previous.endpoints.clone(),
            protocol_versions: previous.protocol_versions.clone(),
            capabilities: Arc::clone(&previous.capabilities),
            state: MembershipState::Active,
            generation,
            registered_at: previous.registered_at,
            last_seen: now,
            lease_expires_at: expiration,
            lease_duration: previous.lease_duration,
        });

        records.insert(id, Arc::clone(&record));

        Ok(record)
    }

    /// Mark a participant as suspected.
    ///
    /// This is an observation, not proof of failure.
    pub fn mark_suspected(
        &self,
        id: ParticipantId,
        expected_generation: u64,
    ) -> MembershipResult<Arc<ParticipantRecord>> {
        self.transition(
            id,
            expected_generation,
            MembershipState::Suspected,
        )
    }

    /// Mark a participant as gracefully leaving.
    ///
    /// A leaving participant cannot be revived by an ordinary heartbeat.
    pub fn mark_leaving(
        &self,
        id: ParticipantId,
        expected_generation: u64,
    ) -> MembershipResult<Arc<ParticipantRecord>> {
        self.transition(
            id,
            expected_generation,
            MembershipState::Leaving,
        )
    }

    fn transition(
        &self,
        id: ParticipantId,
        expected_generation: u64,
        new_state: MembershipState,
    ) -> MembershipResult<Arc<ParticipantRecord>> {
        let mut records = self.records.write()?;

        let previous = records
            .get(&id)
            .ok_or(
                MembershipRegistryError::UnknownParticipant {
                    participant_id: id,
                },
            )?;

        if previous.generation != expected_generation {
            return Err(
                MembershipRegistryError::GenerationConflict {
                    participant_id: id,
                    expected: expected_generation,
                    actual: previous.generation,
                },
            );
        }

        if previous.state == MembershipState::Leaving
            && new_state != MembershipState::Leaving
        {
            return Err(
                MembershipRegistryError::GenerationConflict {
                    participant_id: id,
                    expected: expected_generation,
                    actual: previous.generation,
                },
            );
        }

        let generation = previous
            .generation
            .checked_add(1)
            .ok_or(MembershipRegistryError::GenerationExhausted)?;

        let record = Arc::new(ParticipantRecord {
            participant: previous.participant.clone(),
            endpoints: previous.endpoints.clone(),
            protocol_versions: previous.protocol_versions.clone(),
            capabilities: Arc::clone(&previous.capabilities),
            state: new_state,
            generation,
            registered_at: previous.registered_at,
            last_seen: previous.last_seen,
            lease_expires_at: previous.lease_expires_at,
            lease_duration: previous.lease_duration,
        });

        records.insert(id, Arc::clone(&record));

        Ok(record)
    }

    /// Remove a participant using its expected generation.
    ///
    /// A stale removal request cannot delete a newer registration.
    pub fn remove(
        &self,
        id: ParticipantId,
        expected_generation: u64,
    ) -> MembershipResult<Arc<ParticipantRecord>> {
        let mut records = self.records.write()?;

        let previous = records
            .get(&id)
            .ok_or(
                MembershipRegistryError::UnknownParticipant {
                    participant_id: id,
                },
            )?;

        if previous.generation != expected_generation {
            return Err(
                MembershipRegistryError::GenerationConflict {
                    participant_id: id,
                    expected: expected_generation,
                    actual: previous.generation,
                },
            );
        }

        let removed = records
            .remove(&id)
            .ok_or(
                MembershipRegistryError::UnknownParticipant {
                    participant_id: id,
                },
            )?;

        Ok(removed)
    }

    /// Count participants whose effective state is Active.
    pub fn active_count(&self) -> MembershipResult<usize> {
        let now = Instant::now();
        let records = self.records.read()?;

        Ok(records
            .values()
            .filter(|record| {
                record.effective_state_at(now)
                    == MembershipState::Active
            })
            .count())
    }

    /// Return participants whose leases have expired.
    ///
    /// This operation observes expiry but does not delete records.
    pub fn expired_participants(
        &self,
    ) -> MembershipResult<Vec<ParticipantId>> {
        let now = Instant::now();
        let records = self.records.read()?;

        let mut expired: Vec<_> = records
            .values()
            .filter(|record| {
                record.effective_state_at(now)
                    == MembershipState::Unavailable
            })
            .map(|record| record.participant_id())
            .collect();

        expired.sort_by_key(|id| id.to_string());

        Ok(expired)
    }

    /// Remove every record from the registry.
    ///
    /// Intended for explicit runtime shutdown or controlled test cleanup.
    ///
    /// Consumers retaining Arc snapshots may continue to inspect those
    /// immutable records after removal.
    pub fn clear(
        &self,
    ) -> MembershipResult<Vec<Arc<ParticipantRecord>>> {
        let mut records = self.records.write()?;

        let removed = records.drain().map(|(_, record)| record).collect();

        Ok(removed)
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::distributed::ids::IdAllocator;
    use crate::distributed::model::participant::Participant;
    use crate::distributed::model::participant::ParticipantRole;

    fn empty_capabilities() -> Arc<CapabilityEvidenceSet> {
        // Use the canonical evidence-set constructor from the semantic
        // subsystem. The registry does not invent capability structures.
        Arc::new(CapabilityEvidenceSet::default())
    }

    fn registration(
        allocator: &IdAllocator,
    ) -> ParticipantRegistration {
        let participant = Participant::new(
            allocator.participant_id().unwrap(),
            ParticipantRole::new("worker").unwrap(),
        );

        let endpoint = ParticipantEndpoint::new(
            "memory://local/worker",
            "test-provider",
        )
        .unwrap();

        ParticipantRegistration::new(
            participant,
            vec![endpoint],
            HashSet::from([
                ProtocolVersion::new(1, 0, 0),
            ]),
            empty_capabilities(),
            Duration::from_secs(60),
        )
        .unwrap()
    }

    #[test]
    fn registers_and_retrieves_participant() {
        let registry = MembershipRegistry::new();
        let allocator = IdAllocator::new();

        let registered = registry
            .register(registration(&allocator))
            .unwrap();

        assert_eq!(registered.state(), MembershipState::Active);
        assert_eq!(registered.generation(), 1);

        let retrieved = registry
            .get(registered.participant_id())
            .unwrap()
            .unwrap();

        assert_eq!(
            retrieved.participant_id(),
            registered.participant_id()
        );
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let registry = MembershipRegistry::new();
        let allocator = IdAllocator::new();

        let first = registration(&allocator);
        let participant = first.participant().id();

        registry.register(first).unwrap();

        let second = ParticipantRegistration::new(
            Participant::new(
                participant,
                ParticipantRole::new("worker").unwrap(),
            ),
            vec![],
            HashSet::from([
                ProtocolVersion::new(1, 0, 0),
            ]),
            empty_capabilities(),
            Duration::from_secs(60),
        )
        .unwrap();

        assert!(matches!(
            registry.register(second),
            Err(MembershipRegistryError::DuplicateParticipant { .. })
        ));
    }

    #[test]
    fn heartbeat_increments_generation() {
        let registry = MembershipRegistry::new();
        let allocator = IdAllocator::new();

        let record = registry
            .register(registration(&allocator))
            .unwrap();

        let renewed = registry
            .heartbeat(record.participant_id(), record.generation())
            .unwrap();

        assert_eq!(renewed.generation(), 2);
        assert!(renewed.lease_is_valid_at(Instant::now()));
    }

    #[test]
    fn stale_heartbeat_is_rejected() {
        let registry = MembershipRegistry::new();
        let allocator = IdAllocator::new();

        let record = registry
            .register(registration(&allocator))
            .unwrap();

        registry
            .heartbeat(record.participant_id(), record.generation())
            .unwrap();

        assert!(matches!(
            registry.heartbeat(record.participant_id(), 1),
            Err(MembershipRegistryError::GenerationConflict { .. })
        ));
    }

    #[test]
    fn suspected_participant_can_recover() {
        let registry = MembershipRegistry::new();
        let allocator = IdAllocator::new();

        let record = registry
            .register(registration(&allocator))
            .unwrap();

        let suspected = registry
            .mark_suspected(
                record.participant_id(),
                record.generation(),
            )
            .unwrap();

        assert_eq!(suspected.state(), MembershipState::Suspected);

        let recovered = registry
            .heartbeat(
                suspected.participant_id(),
                suspected.generation(),
            )
            .unwrap();

        assert_eq!(recovered.state(), MembershipState::Active);
    }

    #[test]
    fn leaving_participant_cannot_be_revived_by_heartbeat() {
        let registry = MembershipRegistry::new();
        let allocator = IdAllocator::new();

        let record = registry
            .register(registration(&allocator))
            .unwrap();

        let leaving = registry
            .mark_leaving(
                record.participant_id(),
                record.generation(),
            )
            .unwrap();

        assert!(registry
            .heartbeat(
                leaving.participant_id(),
                leaving.generation(),
            )
            .is_err());
    }

    #[test]
    fn retained_snapshot_survives_removal() {
        let registry = MembershipRegistry::new();
        let allocator = IdAllocator::new();

        let record = registry
            .register(registration(&allocator))
            .unwrap();

        registry
            .remove(record.participant_id(), record.generation())
            .unwrap();

        assert_eq!(record.state(), MembershipState::Active);
        assert!(registry.is_empty().unwrap());
    }

    #[test]
    fn snapshot_order_is_deterministic() {
        let registry = MembershipRegistry::new();
        let allocator = IdAllocator::new();

        registry.register(registration(&allocator)).unwrap();
        registry.register(registration(&allocator)).unwrap();

        let snapshot = registry.snapshot().unwrap();

        let identifiers: Vec<_> = snapshot
            .iter()
            .map(|record| record.participant_id().to_string())
            .collect();

        let mut sorted = identifiers.clone();
        sorted.sort();

        assert_eq!(identifiers, sorted);
    }

    #[test]
    fn rejects_zero_duration_lease() {
        let allocator = IdAllocator::new();
        let participant = Participant::new(
            allocator.participant_id().unwrap(),
            ParticipantRole::new("worker").unwrap(),
        );

        let result = ParticipantRegistration::new(
            participant,
            vec![],
            HashSet::from([
                ProtocolVersion::new(1, 0, 0),
            ]),
            empty_capabilities(),
            Duration::ZERO,
        );

        assert!(matches!(
            result,
            Err(MembershipRegistryError::InvalidLease)
        ));
    }

    #[test]
    fn rejects_invalid_endpoint() {
        assert!(ParticipantEndpoint::new(
            "",
            "test-provider",
        )
        .is_err());

        assert!(ParticipantEndpoint::new(
            "memory://local",
            "",
        )
        .is_err());
    }
}
