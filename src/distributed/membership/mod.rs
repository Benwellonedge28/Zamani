
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Distributed Membership Subsystem
//!
//! Path: `src/distributed/membership/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! This module is the public composition root for distributed participant
//! membership in Zamani.
//!
//! It integrates:
//!
//! - `registry`: authoritative logical participant membership;
//! - `discovery`: extensible participant discovery providers;
//! - `health`: authenticated-session health and liveness observations.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Membership subsystem composition.
//! - Public membership module exports.
//! - Shared ownership of the authoritative registry.
//! - Construction of discovery and health services.
//! - The public integration boundary for runtime consumers.
//!
//! It does not own:
//!
//! - Logical participant identity.
//! - Participant semantic meaning.
//! - Capability definitions or evidence.
//! - Resource discovery or allocation.
//! - Authentication or authorization algorithms.
//! - Network communication or transport implementations.
//! - Placement, scheduling, partitioning or routing.
//! - Task execution or process migration.
//! - Checkpointing or recovery execution.
//! - ZUIR, ZCIR, quantum::ir, HDLIR or backend implementations.
//!
//! # Authoritative ownership
//!
//! The following existing files remain authoritative:
//!
//! - `distributed/ids.rs`:
//!   ParticipantId.
//! - `distributed/model/participant.rs`:
//!   Participant.
//! - `distributed/membership/registry.rs`:
//!   MembershipRegistry.
//! - `distributed/membership/discovery.rs`:
//!   DiscoveryService.
//! - `distributed/membership/health.rs`:
//!   HealthMonitor.
//! - `distributed/transport/messages.rs`:
//!   ProtocolVersion.
//! - `distributed/transport/traits.rs`:
//!   AuthenticatedPeer.
//! - `frontend/semantic/capabilities/evidence`:
//!   CapabilityEvidenceSet.
//!
//! # Security boundary
//!
//! Membership registration does not establish trust.
//!
//! Discovery results are untrusted claims.
//!
//! An authenticated peer is evidence that an authentication procedure
//! has established an identity. It is not an authorization grant.
//!
//! The security subsystem remains responsible for authentication,
//! authorization, credential validation and workload permissions.
//!
//! A discovered participant MUST NOT automatically become eligible
//! for arbitrary remote workload execution.
//!
//! # Scalability and POCO-REAF
//!
//! This module imposes no application-defined universal maximum on:
//!
//! - Participants.
//! - Membership records.
//! - Discovery providers.
//! - Discovery pages.
//! - Endpoints.
//! - Membership updates.
//! - Authenticated sessions.
//! - Health observations.
//!
//! Actual capacity depends on available memory, address space,
//! identifier representation, configured operational budgets and
//! underlying platform capabilities.
//!
//! This module does not assume:
//!
//! - One participant equals one physical machine.
//! - One endpoint equals one participant.
//! - One participant has only one endpoint.
//! - One transport provider serves the entire system.
//! - One fixed network topology.
//! - Permanent participant availability.
//!
//! # Concurrency
//!
//! The registry, discovery service and health monitor provide their
//! own synchronization.
//!
//! This composition root introduces no additional global locks.
//!
//! Cloning a MembershipSubsystem shares the underlying services and
//! authoritative registry.
//!
//! # Lifecycle
//!
//! Construction does not start background threads, open network
//! connections, discover participants or start remote execution.
//!
//! The owning runtime controls initialization, periodic health
//! evaluation, discovery invocation and graceful shutdown.
//!
//! # Safety
//!
//! This module and its integration interfaces use safe Rust only.

// =============================================================================
// MODULE DECLARATIONS
// =============================================================================

/// Provider-independent participant discovery.
pub mod discovery;

/// Authenticated participant health and liveness monitoring.
pub mod health;

/// Authoritative participant membership registry.
pub mod registry;

// =============================================================================
// PUBLIC API RE-EXPORTS
// =============================================================================

// Discovery ------------------------------------------------------------------

pub use discovery::{
    DiscoveryCatalogError,
    DiscoveryCatalogResult,
    DiscoveryError,
    DiscoveryFuture,
    DiscoveryPage,
    DiscoveryProvider,
    DiscoveryProviderError,
    DiscoveryReport,
    DiscoveryService,
    ProviderDiscoveryOutcome,
    RegistrationOutcome,
};

// Health ---------------------------------------------------------------------

pub use health::{
    HealthMonitor,
    HealthMonitorError,
    HealthObservation,
    HealthPolicy,
    HealthResult,
    HealthStatus,
    PeerSessionId,
};

// Registry -------------------------------------------------------------------

pub use registry::{
    MembershipRegistry,
    MembershipRegistryError,
    MembershipResult,
    MembershipState,
    ParticipantEndpoint,
    ParticipantRecord,
    ParticipantRegistration,
};

// =============================================================================
// STANDARD LIBRARY
// =============================================================================

use core::fmt;

// =============================================================================
// MEMBERSHIP SUBSYSTEM COMPOSITION
// =============================================================================

/// Composed membership services sharing one authoritative registry.
///
/// `MembershipSubsystem` is the recommended integration boundary for
/// runtime components that need participant registration, discovery
/// and health monitoring.
///
/// The subsystem contains:
///
/// - One authoritative `MembershipRegistry`.
/// - One `DiscoveryService` connected to that registry.
/// - One `HealthMonitor` connected to that registry.
///
/// The discovery service and health monitor receive clones of the
/// same registry handle. They do not construct independent registries.
///
/// # Ownership
///
/// The application or runtime owns the lifecycle of this subsystem.
///
/// Cloning the subsystem shares the underlying registry, discovery
/// provider catalog and health-session observations.
///
/// # Thread safety
///
/// The underlying services use synchronized shared ownership.
///
/// `MembershipSubsystem` can therefore be shared between runtime
/// components without introducing global mutable state.
///
/// # Security
///
/// This type does not authenticate participants or grant permissions.
///
/// Security decisions remain with the existing security authority.
#[derive(Clone)]
pub struct MembershipSubsystem {
    registry: MembershipRegistry,
    discovery: DiscoveryService,
    health: HealthMonitor,
}

impl fmt::Debug for MembershipSubsystem {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("MembershipSubsystem")
            .field("registry", &self.registry)
            .field("discovery", &self.discovery)
            .field("health", &self.health)
            .finish()
    }
}

impl MembershipSubsystem {
    /// Construct a membership subsystem with a new authoritative registry.
    ///
    /// The supplied health policy is shared by the health monitor.
    ///
    /// No background workers, network connections or discovery
    /// operations are started during construction.
    ///
    /// # Arguments
    ///
    /// - `health_policy`: Validated operational health-monitoring policy.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use std::time::Duration;
    /// use zamani_compiler::distributed::membership::{
    ///     HealthPolicy,
    ///     MembershipSubsystem,
    /// };
    ///
    /// let policy = HealthPolicy::new(
    ///     Duration::from_secs(10),
    ///     3,
    /// )?;
    ///
    /// let membership = MembershipSubsystem::new(policy);
    /// ```
    #[must_use]
    pub fn new(
        health_policy: HealthPolicy,
    ) -> Self {
        Self::with_registry(
            MembershipRegistry::new(),
            health_policy,
        )
    }

    /// Construct membership services over an existing registry.
    ///
    /// This constructor is intended for integration with an existing
    /// runtime or another explicitly owned membership authority.
    ///
    /// Both discovery and health monitoring receive clones of the
    /// supplied registry handle.
    ///
    /// No second authoritative membership store is created.
    ///
    /// # Arguments
    ///
    /// - `registry`: Existing authoritative registry.
    /// - `health_policy`: Validated health-monitoring policy.
    #[must_use]
    pub fn with_registry(
        registry: MembershipRegistry,
        health_policy: HealthPolicy,
    ) -> Self {
        let discovery = DiscoveryService::new(
            registry.clone(),
        );

        let health = HealthMonitor::new(
            registry.clone(),
            health_policy,
        );

        Self {
            registry,
            discovery,
            health,
        }
    }

    /// Borrow the authoritative membership registry.
    ///
    /// This is the preferred access point for direct membership
    /// queries and generation-checked membership operations.
    #[must_use]
    pub fn registry(&self) -> &MembershipRegistry {
        &self.registry
    }

    /// Obtain a shared handle to the authoritative registry.
    ///
    /// The returned handle refers to the same underlying membership
    /// state. It does not create a new registry.
    #[must_use]
    pub fn registry_handle(&self) -> MembershipRegistry {
        self.registry.clone()
    }

    /// Borrow the discovery service.
    ///
    /// Use this service to register, replace and unregister discovery
    /// providers and to execute discovery scans.
    #[must_use]
    pub fn discovery(&self) -> &DiscoveryService {
        &self.discovery
    }

    /// Borrow the health monitor.
    ///
    /// Use this service to establish authenticated peer sessions,
    /// observe heartbeat sequences and evaluate participant health.
    #[must_use]
    pub fn health(&self) -> &HealthMonitor {
        &self.health
    }

    /// Consume the subsystem and return its three shared service handles.
    ///
    /// All returned handles retain their existing shared registry
    /// relationship.
    ///
    /// This operation does not shut down providers, remove participants
    /// or terminate authenticated sessions.
    #[must_use]
    pub fn into_components(
        self,
    ) -> (
        MembershipRegistry,
        DiscoveryService,
        HealthMonitor,
    ) {
        (
            self.registry,
            self.discovery,
            self.health,
        )
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;

    fn test_policy() -> HealthPolicy {
        HealthPolicy::new(
            Duration::from_secs(10),
            3,
        )
        .expect("valid health policy")
    }

    #[test]
    fn constructs_complete_membership_subsystem() {
        let membership = MembershipSubsystem::new(
            test_policy(),
        );

        assert!(
            membership.registry().is_empty().unwrap()
        );

        assert_eq!(
            membership.discovery().provider_count().unwrap(),
            0
        );

        assert_eq!(
            membership.health().tracked_session_count().unwrap(),
            0
        );
    }

    #[test]
    fn cloned_registry_handles_share_authoritative_state() {
        let membership = MembershipSubsystem::new(
            test_policy(),
        );

        let first = membership.registry_handle();
        let second = membership.registry_handle();

        assert!(
            first.is_empty().unwrap()
        );

        assert!(
            second.is_empty().unwrap()
        );
    }

    #[test]
    fn supplied_registry_is_preserved() {
        let registry = MembershipRegistry::new();

        let membership = MembershipSubsystem::with_registry(
            registry.clone(),
            test_policy(),
        );

        assert!(
            membership.registry().is_empty().unwrap()
        );

        assert!(
            registry.is_empty().unwrap()
        );
    }

    #[test]
    fn discovery_uses_the_shared_registry_handle() {
        let membership = MembershipSubsystem::new(
            test_policy(),
        );

        let discovery_registry =
            membership.discovery().membership_registry();

        assert!(
            discovery_registry.is_empty().unwrap()
        );

        assert_eq!(
            discovery_registry.len().unwrap(),
            membership.registry().len().unwrap()
        );
    }

    #[test]
    fn cloning_subsystem_preserves_shared_services() {
        let membership = MembershipSubsystem::new(
            test_policy(),
        );

        let cloned = membership.clone();

        assert!(
            cloned.registry().is_empty().unwrap()
        );

        assert_eq!(
            cloned.discovery().provider_count().unwrap(),
            membership.discovery().provider_count().unwrap()
        );

        assert_eq!(
            cloned.health().policy(),
            membership.health().policy()
        );
    }

    #[test]
    fn component_extraction_preserves_registry_access() {
        let membership = MembershipSubsystem::new(
            test_policy(),
        );

        let (
            registry,
            discovery,
            health,
        ) = membership.into_components();

        assert!(registry.is_empty().unwrap());

        assert_eq!(
            discovery.membership_registry().len().unwrap(),
            registry.len().unwrap()
        );

        assert_eq!(
            health.policy(),
            test_policy()
        );
    }
}
