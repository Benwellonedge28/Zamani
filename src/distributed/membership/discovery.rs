
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Provider-independent participant discovery for Zamani distributed
//! computation.
//!
//! Path: `src/distributed/membership/discovery.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Discover logical participants through replaceable discovery providers
//! and submit their registration information to the authoritative
//! [`MembershipRegistry`].
//!
//! Discovery providers may use:
//!
//! - Existing networking facilities.
//! - Transport-provider integrations.
//! - Local process or execution-environment inspection.
//! - Cluster and service-discovery systems.
//! - Embedded or hardware-specific discovery.
//! - Quantum-network discovery.
//! - Application-provided discovery mechanisms.
//! - Future discovery technologies.
//!
//! This module does not prescribe any particular discovery protocol.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Discovery-provider registration and replacement.
//! - Provider invocation and result reporting.
//! - Paginated discovery.
//! - Discovery cursor validation.
//! - Submission of discovered registrations to the membership registry.
//! - Explicit reporting of duplicate and rejected registrations.
//!
//! This module does not own:
//!
//! - Logical participant identity.
//! - Participant semantic meaning.
//! - Membership records or membership lifecycle.
//! - Capability definitions or capability evidence semantics.
//! - Authentication or authorization.
//! - Physical network communication.
//! - Transport protocols or message encoding.
//! - Resource discovery semantics.
//! - Placement, partitioning, scheduling, or routing.
//! - Health monitoring or heartbeat policy.
//! - Execution, migration, checkpointing, or recovery.
//! - ZUIR, ZCIR, quantum::ir, HDLIR, or backend implementations.
//!
//! # Authoritative dependencies
//!
//! - `membership/registry.rs` owns authoritative local membership.
//! - `distributed/ids.rs` owns `ParticipantId`.
//! - `distributed/model/participant.rs` owns `Participant`.
//! - `distributed/transport/traits.rs` owns transport interfaces.
//! - `distributed/transport/messages.rs` owns protocol versions.
//! - `frontend/semantic/capabilities/evidence` owns capability evidence.
//! - Existing security modules own authentication and authorization.
//! - Existing networking modules own physical discovery mechanisms.
//!
//! # Trust boundary
//!
//! Discovery results are untrusted claims.
//!
//! A provider returning a participant registration does not prove:
//!
//! - That the participant controls the advertised endpoint.
//! - That the participant has the claimed identity.
//! - That its capability claims are truthful.
//! - That it is authorized to join a workload.
//! - That it is safe to execute remote code on it.
//!
//! Authentication and authorization must be completed by the existing
//! security authority before a discovered participant is trusted for
//! security-sensitive operations.
//!
//! # Duplicate and update semantics
//!
//! This module never silently overwrites an existing participant.
//!
//! The authoritative registry rejects duplicate registrations.
//! Updating an existing participant requires the registry's explicit
//! generation-checked update operation.
//!
//! Discovery providers must not be treated as authoritative membership
//! writers merely because they returned a newer-looking result.
//!
//! # Pagination and scalability
//!
//! Providers may return multiple pages.
//!
//! There is no fixed maximum number of providers, participants, endpoints,
//! or discovery pages imposed by this module.
//!
//! Providers and their callers remain responsible for operational budgets,
//! timeouts, cancellation, and resource consumption.
//!
//! A provider that repeats a pagination cursor is rejected for that scan.
//! This prevents a broken provider from causing an endless cursor cycle.
//!
//! Real memory, address-space, network, and provider limitations remain
//! finite and must be reported honestly.
//!
//! # Concurrency
//!
//! Provider registration is protected by an `RwLock`.
//!
//! A discovery scan takes a stable snapshot of the registered providers.
//! Provider calls occur without holding the catalog lock.
//!
//! Providers can therefore be replaced or unregistered while a scan is
//! running. The current scan retains its original `Arc` references;
//! subsequent scans observe the updated catalog.
//!
//! # Safety
//!
//! This implementation uses safe Rust only.

use core::fmt;
use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, PoisonError, RwLock};

use crate::distributed::ids::ParticipantId;
use crate::distributed::membership::registry::{
    MembershipRegistry,
    MembershipRegistryError,
    ParticipantRecord,
    ParticipantRegistration,
};

/// Error returned by a discovery provider.
///
/// The original error is retained so that callers can inspect its source
/// chain. Providers should use meaningful error types rather than reducing
/// all failures to strings.
pub type DiscoveryProviderError =
    Box<dyn Error + Send + Sync + 'static>;

/// A boxed future returned by a discovery provider.
///
/// This interface uses standard-library futures and does not require
/// Tokio, async-std, or another particular asynchronous runtime.
pub type DiscoveryFuture<'a, T> =
    Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Errors produced by discovery-provider catalog operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DiscoveryCatalogError {
    /// A provider identifier is empty, whitespace-only, or contains
    /// control characters.
    InvalidProviderId {
        /// The supplied provider identifier.
        provider_id: String,
    },

    /// A provider with the identifier is already registered.
    DuplicateProvider {
        /// The conflicting provider identifier.
        provider_id: String,
    },

    /// No provider with the requested identifier is registered.
    UnknownProvider {
        /// The requested provider identifier.
        provider_id: String,
    },

    /// The provider catalog synchronization lock was poisoned.
    LockPoisoned,
}

impl fmt::Display for DiscoveryCatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidProviderId { provider_id } => {
                write!(
                    f,
                    "invalid discovery provider identifier {provider_id:?}"
                )
            }
            Self::DuplicateProvider { provider_id } => {
                write!(
                    f,
                    "discovery provider {provider_id:?} is already registered"
                )
            }
            Self::UnknownProvider { provider_id } => {
                write!(
                    f,
                    "discovery provider {provider_id:?} is not registered"
                )
            }
            Self::LockPoisoned => {
                f.write_str("discovery provider catalog lock was poisoned")
            }
        }
    }
}

impl Error for DiscoveryCatalogError {}

impl<T> From<PoisonError<T>> for DiscoveryCatalogError {
    fn from(_: PoisonError<T>) -> Self {
        Self::LockPoisoned
    }
}

/// Errors produced by the discovery orchestration layer.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DiscoveryError {
    /// A provider returned an invalid pagination cursor.
    InvalidCursor {
        /// Provider that returned the cursor.
        provider_id: String,
    },

    /// A provider repeated a cursor during one discovery scan.
    ///
    /// Repeated cursors can otherwise create an infinite pagination loop.
    CursorCycle {
        /// Provider that repeated the cursor.
        provider_id: String,
    },

    /// The provider's page counter exhausted its representable range.
    PageCounterExhausted {
        /// Provider whose page counter was exhausted.
        provider_id: String,
    },
}

impl fmt::Display for DiscoveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCursor { provider_id } => {
                write!(
                    f,
                    "discovery provider {provider_id:?} returned an invalid cursor"
                )
            }
            Self::CursorCycle { provider_id } => {
                write!(
                    f,
                    "discovery provider {provider_id:?} repeated a pagination cursor"
                )
            }
            Self::PageCounterExhausted { provider_id } => {
                write!(
                    f,
                    "discovery page counter exhausted for provider {provider_id:?}"
                )
            }
        }
    }
}

impl Error for DiscoveryError {}

/// Result type for discovery catalog operations.
pub type DiscoveryCatalogResult<T> =
    Result<T, DiscoveryCatalogError>;

/// One page of discovered participant registrations.
///
/// The cursor is opaque to the discovery subsystem. Only the provider
/// that created it is permitted to interpret it.
///
/// The next cursor is optional. `None` indicates that the provider has
/// completed the current discovery enumeration.
#[derive(Debug)]
pub struct DiscoveryPage {
    registrations: Vec<ParticipantRegistration>,
    next_cursor: Option<String>,
}

impl DiscoveryPage {
    /// Construct a discovery page.
    ///
    /// An empty registration list is valid. A provider may have no new
    /// participants on a page while still returning a continuation cursor.
    #[must_use]
    pub fn new(
        registrations: Vec<ParticipantRegistration>,
        next_cursor: Option<String>,
    ) -> Self {
        Self {
            registrations,
            next_cursor,
        }
    }

    /// Return the registrations contained in this page.
    #[must_use]
    pub fn registrations(&self) -> &[ParticipantRegistration] {
        &self.registrations
    }

    /// Return the next opaque pagination cursor, if any.
    #[must_use]
    pub fn next_cursor(&self) -> Option<&str> {
        self.next_cursor.as_deref()
    }

    fn into_parts(self) -> (Vec<ParticipantRegistration>, Option<String>) {
        (self.registrations, self.next_cursor)
    }
}

/// Provider interface for discovering logical distributed participants.
///
/// A provider may represent a local discovery mechanism, a remote service
/// registry, a cluster membership service, an embedded environment, or
/// another discovery technology.
///
/// Implementations must:
///
/// - Return logical participant registrations rather than physical
///   scheduler decisions.
/// - Preserve the canonical `ParticipantId` of each discovered participant.
/// - Supply valid `ParticipantRegistration` values.
/// - Use the existing canonical capability evidence model.
/// - Return opaque pagination cursors that remain valid for that provider.
/// - Enforce their own transport deadlines and cancellation behavior.
/// - Avoid silently truncating results when an operational budget is
///   exhausted.
/// - Report partial or failed discovery explicitly.
/// - Avoid treating discovered data as authenticated identity.
///
/// Providers must not mutate the membership registry directly. The
/// discovery service submits their results to the authoritative registry.
///
/// A provider may be called concurrently by different discovery services.
/// Implementations must provide their own synchronization where necessary.
pub trait DiscoveryProvider: Send + Sync {
    /// Discover one page of logical participant registrations.
    ///
    /// `cursor` is `None` for the first page. A subsequent cursor is
    /// returned unchanged to the same provider.
    ///
    /// # Errors
    ///
    /// Returns the provider's original error when discovery fails.
    fn discover<'a>(
        &'a self,
        cursor: Option<String>,
    ) -> DiscoveryFuture<'a, Result<DiscoveryPage, DiscoveryProviderError>>;
}

/// Outcome of submitting one discovered registration to the registry.
#[derive(Debug)]
#[non_exhaustive]
pub enum RegistrationOutcome {
    /// The participant was newly registered.
    Registered {
        /// Logical participant identity.
        participant_id: ParticipantId,

        /// Generation assigned by the authoritative registry.
        generation: u64,
    },

    /// The participant was already present in the authoritative registry.
    ///
    /// Duplicate discovery is not treated as a successful update. The
    /// existing record remains unchanged.
    AlreadyRegistered {
        /// Logical participant identity.
        participant_id: ParticipantId,
    },

    /// The registration was rejected by the authoritative registry.
    Rejected {
        /// Logical participant identity.
        participant_id: ParticipantId,

        /// The registry's original typed error.
        error: MembershipRegistryError,
    },
}

impl RegistrationOutcome {
    /// Return the logical participant identity associated with this result.
    #[must_use]
    pub const fn participant_id(&self) -> ParticipantId {
        match self {
            Self::Registered { participant_id, .. }
            | Self::AlreadyRegistered { participant_id }
            | Self::Rejected { participant_id, .. } => *participant_id,
        }
    }

    /// Return whether this outcome created a new membership record.
    #[must_use]
    pub const fn is_new_registration(&self) -> bool {
        matches!(self, Self::Registered { .. })
    }

    /// Return whether the registration was rejected.
    #[must_use]
    pub const fn is_rejected(&self) -> bool {
        matches!(self, Self::Rejected { .. })
    }
}

/// Outcome of one provider's discovery scan.
#[derive(Debug)]
#[non_exhaustive]
pub enum ProviderDiscoveryOutcome {
    /// The provider completed all its pages successfully.
    Completed {
        /// Registered provider identifier.
        provider_id: String,

        /// Number of pages successfully retrieved.
        pages_retrieved: usize,

        /// Number of registrations returned by the provider.
        registrations_discovered: usize,

        /// Number of registrations newly accepted by the registry.
        registrations_added: usize,

        /// Number of registrations already present.
        duplicates: usize,

        /// Number of registrations rejected by the registry.
        registrations_rejected: usize,
    },

    /// The provider failed during discovery or pagination.
    ///
    /// Registrations accepted before the failure remain registered.
    /// The provider failure does not roll back independent successful work.
    Failed {
        /// Registered provider identifier.
        provider_id: String,

        /// Number of pages successfully retrieved before failure.
        pages_retrieved: usize,

        /// Number of registrations returned before failure.
        registrations_discovered: usize,

        /// Number of registrations newly accepted before failure.
        registrations_added: usize,

        /// Original provider or discovery-protocol error.
        source: DiscoveryProviderError,
    },
}

impl ProviderDiscoveryOutcome {
    /// Return the registered provider identifier.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        match self {
            Self::Completed { provider_id, .. }
            | Self::Failed { provider_id, .. } => provider_id,
        }
    }

    /// Return whether all pages for this provider were processed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(self, Self::Completed { .. })
    }

    /// Return the original provider or protocol error, if any.
    #[must_use]
    pub fn error(
        &self,
    ) -> Option<&(dyn Error + Send + Sync + 'static)> {
        match self {
            Self::Completed { .. } => None,
            Self::Failed { source, .. } => Some(source.as_ref()),
        }
    }
}

/// Complete report from one discovery scan.
///
/// This report describes the work performed during a scan. It is not a
/// snapshot of the entire distributed universe: discovery providers may
/// have different scopes, visibility, consistency, and completeness
/// guarantees.
#[derive(Debug)]
pub struct DiscoveryReport {
    provider_count: usize,
    provider_outcomes: Vec<ProviderDiscoveryOutcome>,
    registration_outcomes: Vec<RegistrationOutcome>,
}

impl DiscoveryReport {
    /// Return the number of providers included in this scan.
    #[must_use]
    pub const fn provider_count(&self) -> usize {
        self.provider_count
    }

    /// Return provider outcomes in deterministic provider-ID order.
    #[must_use]
    pub fn provider_outcomes(&self) -> &[ProviderDiscoveryOutcome] {
        &self.provider_outcomes
    }

    /// Return individual registration outcomes.
    #[must_use]
    pub fn registration_outcomes(&self) -> &[RegistrationOutcome] {
        &self.registration_outcomes
    }

    /// Return the number of newly registered participants.
    #[must_use]
    pub fn registered_count(&self) -> usize {
        self.registration_outcomes
            .iter()
            .filter(|outcome| outcome.is_new_registration())
            .count()
    }

    /// Return the number of duplicate participant announcements.
    #[must_use]
    pub fn duplicate_count(&self) -> usize {
        self.registration_outcomes
            .iter()
            .filter(|outcome| {
                matches!(outcome, RegistrationOutcome::AlreadyRegistered { .. })
            })
            .count()
    }

    /// Return the number of rejected participant registrations.
    #[must_use]
    pub fn rejected_count(&self) -> usize {
        self.registration_outcomes
            .iter()
            .filter(|outcome| outcome.is_rejected())
            .count()
    }

    /// Return whether every provider completed and no registration was
    /// rejected.
    ///
    /// A scan with no providers is incomplete. Duplicate announcements do
    /// not make a scan incomplete because they do not change membership.
    ///
    /// Completeness does not prove that every participant in the
    /// environment was discoverable or that discovered participants
    /// are authenticated.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.provider_count > 0
            && self.provider_outcomes.len() == self.provider_count
            && self.provider_outcomes.iter().all(
                ProviderDiscoveryOutcome::is_complete,
            )
            && self.rejected_count() == 0
    }

    /// Return whether at least one provider failed.
    #[must_use]
    pub fn has_provider_failures(&self) -> bool {
        self.provider_outcomes.iter().any(
            |outcome| !outcome.is_complete(),
        )
    }

    /// Return whether any registration was rejected.
    #[must_use]
    pub fn has_registration_rejections(&self) -> bool {
        self.rejected_count() > 0
    }
}

/// Provider catalog and discovery coordinator.
///
/// The coordinator owns no participant records. It submits provider results
/// to the supplied authoritative [`MembershipRegistry`].
///
/// Create it through [`DiscoveryService::new`] and inject it into the
/// runtime or membership owner. There is no global singleton.
#[derive(Clone)]
pub struct DiscoveryService {
    providers: Arc<RwLock<BTreeMap<String, Arc<dyn DiscoveryProvider>>>>,
    membership: MembershipRegistry,
}

impl fmt::Debug for DiscoveryService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.providers.read() {
            Ok(providers) => f
                .debug_struct("DiscoveryService")
                .field("provider_ids", &providers.keys().collect::<Vec<_>>())
                .finish(),
            Err(_) => f
                .debug_struct("DiscoveryService")
                .field("state", &"poisoned")
                .finish(),
        }
    }
}

impl DiscoveryService {
    /// Construct a discovery service using the authoritative membership
    /// registry.
    #[must_use]
    pub fn new(membership: MembershipRegistry) -> Self {
        Self {
            providers: Arc::new(RwLock::new(BTreeMap::new())),
            membership,
        }
    }

    /// Return a clone of the authoritative membership registry handle.
    ///
    /// Cloning the handle shares the same registry state. It does not
    /// create a second membership authority.
    #[must_use]
    pub fn membership_registry(&self) -> MembershipRegistry {
        self.membership.clone()
    }

    /// Return the number of registered discovery providers.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider catalog lock is poisoned.
    pub fn provider_count(&self) -> DiscoveryCatalogResult<usize> {
        Ok(self.providers.read()?.len())
    }

    /// Return whether the provider catalog is empty.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider catalog lock is poisoned.
    pub fn is_empty(&self) -> DiscoveryCatalogResult<bool> {
        Ok(self.providers.read()?.is_empty())
    }

    /// Return provider identifiers in deterministic order.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider catalog lock is poisoned.
    pub fn provider_ids(&self) -> DiscoveryCatalogResult<Vec<String>> {
        Ok(self.providers.read()?.keys().cloned().collect())
    }

    /// Register a discovery provider.
    ///
    /// Duplicate identifiers are rejected. Use
    /// [`DiscoveryService::replace_provider`] to replace a provider
    /// deliberately.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid or duplicate provider identifier,
    /// or if the catalog lock is poisoned.
    pub fn register_provider(
        &self,
        provider_id: impl Into<String>,
        provider: Arc<dyn DiscoveryProvider>,
    ) -> DiscoveryCatalogResult<()> {
        let provider_id = provider_id.into();
        validate_provider_id(&provider_id)?;

        let mut providers = self.providers.write()?;

        if providers.contains_key(&provider_id) {
            return Err(DiscoveryCatalogError::DuplicateProvider {
                provider_id,
            });
        }

        providers.insert(provider_id, provider);
        Ok(())
    }

    /// Replace an existing provider explicitly.
    ///
    /// The previous provider is returned so that its owner can control
    /// provider-specific shutdown and resource cleanup.
    ///
    /// An already-running scan retains its original provider `Arc`.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid or unknown provider identifier,
    /// or if the catalog lock is poisoned.
    pub fn replace_provider(
        &self,
        provider_id: impl Into<String>,
        provider: Arc<dyn DiscoveryProvider>,
    ) -> DiscoveryCatalogResult<Arc<dyn DiscoveryProvider>> {
        let provider_id = provider_id.into();
        validate_provider_id(&provider_id)?;

        let mut providers = self.providers.write()?;

        let previous = providers
            .get_mut(&provider_id)
            .ok_or_else(|| DiscoveryCatalogError::UnknownProvider {
                provider_id: provider_id.clone(),
            })?;

        Ok(std::mem::replace(previous, provider))
    }

    /// Unregister a discovery provider.
    ///
    /// Unregistering a provider does not remove participants it previously
    /// discovered. Membership removal belongs to the registry and its
    /// lifecycle policy.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid or unknown provider identifier,
    /// or if the catalog lock is poisoned.
    pub fn unregister_provider(
        &self,
        provider_id: &str,
    ) -> DiscoveryCatalogResult<Arc<dyn DiscoveryProvider>> {
        validate_provider_id(provider_id)?;

        self.providers
            .write()?
            .remove(provider_id)
            .ok_or_else(|| DiscoveryCatalogError::UnknownProvider {
                provider_id: provider_id.to_owned(),
            })
    }

    /// Discover participants from all providers currently registered.
    ///
    /// The provider list is snapshotted before any provider is called.
    /// Providers are invoked in deterministic identifier order.
    ///
    /// Each provider is enumerated page by page until it returns no
    /// continuation cursor. A repeated cursor is treated as a provider
    /// protocol failure.
    ///
    /// Successful registrations are committed immediately to the
    /// authoritative membership registry. If a later page or provider
    /// fails, earlier successful registrations remain available.
    ///
    /// Existing participant identities are not overwritten. A duplicate
    /// registration is reported as `AlreadyRegistered`.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider catalog cannot be read.
    /// Provider-specific failures are retained in the returned report
    /// rather than aborting discovery of other providers.
    pub async fn discover_all(
        &self,
    ) -> DiscoveryCatalogResult<DiscoveryReport> {
        let providers: Vec<(String, Arc<dyn DiscoveryProvider>)> = {
            let providers = self.providers.read()?;
            providers
                .iter()
                .map(|(id, provider)| (id.clone(), Arc::clone(provider)))
                .collect()
        };

        let provider_count = providers.len();
        let mut provider_outcomes = Vec::with_capacity(provider_count);
        let mut registration_outcomes = Vec::new();

        for (provider_id, provider) in providers {
            let mut cursor: Option<String> = None;
            let mut seen_cursors = HashSet::new();

            let mut pages_retrieved = 0usize;
            let mut registrations_discovered = 0usize;
            let mut registrations_added = 0usize;
            let mut duplicates = 0usize;
            let mut registrations_rejected = 0usize;

            loop {
                let page = match provider.discover(cursor.take()).await {
                    Ok(page) => page,
                    Err(source) => {
                        provider_outcomes.push(
                            ProviderDiscoveryOutcome::Failed {
                                provider_id,
                                pages_retrieved,
                                registrations_discovered,
                                registrations_added,
                                source,
                            },
                        );
                        break;
                    }
                };

                pages_retrieved = match pages_retrieved.checked_add(1) {
                    Some(value) => value,
                    None => {
                        provider_outcomes.push(
                            ProviderDiscoveryOutcome::Failed {
                                provider_id: provider_id.clone(),
                                pages_retrieved,
                                registrations_discovered,
                                registrations_added,
                                source: Box::new(
                                    DiscoveryError::PageCounterExhausted {
                                        provider_id,
                                    },
                                ),
                            },
                        );
                        break;
                    }
                };

                let (registrations, next_cursor) = page.into_parts();

                for registration in registrations {
                    registrations_discovered =
                        registrations_discovered.saturating_add(1);

                    let participant_id = registration.participant().id();

                    match self.membership.register(registration) {
                        Ok(record) => {
                            registrations_added =
                                registrations_added.saturating_add(1);

                            registration_outcomes.push(
                                RegistrationOutcome::Registered {
                                    participant_id,
                                    generation: record.generation(),
                                },
                            );
                        }

                        Err(
                            MembershipRegistryError::DuplicateParticipant {
                                ..
                            },
                        ) => {
                            duplicates = duplicates.saturating_add(1);

                            registration_outcomes.push(
                                RegistrationOutcome::AlreadyRegistered {
                                    participant_id,
                                },
                            );
                        }

                        Err(error) => {
                            registrations_rejected =
                                registrations_rejected.saturating_add(1);

                            registration_outcomes.push(
                                RegistrationOutcome::Rejected {
                                    participant_id,
                                    error,
                                },
                            );
                        }
                    }
                }

                let Some(next_cursor) = next_cursor else {
                    provider_outcomes.push(
                        ProviderDiscoveryOutcome::Completed {
                            provider_id,
                            pages_retrieved,
                            registrations_discovered,
                            registrations_added,
                            duplicates,
                            registrations_rejected,
                        },
                    );
                    break;
                };

                if next_cursor.trim().is_empty()
                    || next_cursor.chars().any(char::is_control)
                {
                    provider_outcomes.push(
                        ProviderDiscoveryOutcome::Failed {
                            provider_id: provider_id.clone(),
                            pages_retrieved,
                            registrations_discovered,
                            registrations_added,
                            source: Box::new(
                                DiscoveryError::InvalidCursor {
                                    provider_id,
                                },
                            ),
                        },
                    );
                    break;
                }

                if !seen_cursors.insert(next_cursor.clone()) {
                    provider_outcomes.push(
                        ProviderDiscoveryOutcome::Failed {
                            provider_id: provider_id.clone(),
                            pages_retrieved,
                            registrations_discovered,
                            registrations_added,
                            source: Box::new(
                                DiscoveryError::CursorCycle {
                                    provider_id,
                                },
                            ),
                        },
                    );
                    break;
                }

                cursor = Some(next_cursor);
            }
        }

        Ok(DiscoveryReport {
            provider_count,
            provider_outcomes,
            registration_outcomes,
        })
    }
}

fn validate_provider_id(
    provider_id: &str,
) -> DiscoveryCatalogResult<()> {
    if provider_id.trim().is_empty()
        || provider_id.chars().any(char::is_control)
    {
        return Err(DiscoveryCatalogError::InvalidProviderId {
            provider_id: provider_id.to_owned(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;
    use std::sync::Arc;
    use std::task::{Context, Poll, Wake, Waker};

    use crate::distributed::ids::ParticipantId;
    use crate::distributed::model::participant::Participant;
    use crate::distributed::model::participant::ParticipantRole;
    use crate::distributed::transport::messages::ProtocolVersion;
    use crate::frontend::semantic::capabilities::evidence::CapabilityEvidenceSet;

    struct TestWake;

    impl Wake for TestWake {
        fn wake(self: Arc<Self>) {}
    }

    fn block_on<F: Future>(future: F) -> F::Output {
        let waker = Waker::from(Arc::new(TestWake));
        let mut context = Context::from_waker(&waker);
        let mut future = Box::pin(future);

        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(output) => return output,
                Poll::Pending => std::thread::yield_now(),
            }
        }
    }

    fn registration(sequence: u128) -> ParticipantRegistration {
        let id = ParticipantId::new(1, sequence)
            .expect("valid participant ID");

        let participant = Participant::new(
            id,
            ParticipantRole::new("worker")
                .expect("valid participant role"),
        );

        let protocols = HashSet::from([
            ProtocolVersion::new(1, 0, 0),
        ]);

        ParticipantRegistration::new(
            participant,
            Vec::new(),
            protocols,
            Arc::new(CapabilityEvidenceSet::new()),
            std::time::Duration::from_secs(30),
        )
        .expect("valid participant registration")
    }

    struct StaticProvider {
        registrations: Vec<ParticipantRegistration>,
    }

    impl DiscoveryProvider for StaticProvider {
        fn discover<'a>(
            &'a self,
            cursor: Option<String>,
        ) -> DiscoveryFuture<'a, Result<DiscoveryPage, DiscoveryProviderError>> {
            Box::pin(async move {
                if cursor.is_some() {
                    return Ok(DiscoveryPage::new(Vec::new(), None));
                }

                Ok(DiscoveryPage::new(
                    self.registrations.clone(),
                    None,
                ))
            })
        }
    }

    struct PaginatedProvider;

    impl DiscoveryProvider for PaginatedProvider {
        fn discover<'a>(
            &'a self,
            cursor: Option<String>,
        ) -> DiscoveryFuture<'a, Result<DiscoveryPage, DiscoveryProviderError>> {
            Box::pin(async move {
                match cursor.as_deref() {
                    None => Ok(DiscoveryPage::new(
                        vec![registration(1)],
                        Some("page-2".to_owned()),
                    )),
                    Some("page-2") => Ok(DiscoveryPage::new(
                        vec![registration(2)],
                        None,
                    )),
                    Some(_) => Err(Box::new(std::io::Error::other(
                        "unexpected cursor",
                    ))),
                }
            })
        }
    }

    struct RepeatingCursorProvider;

    impl DiscoveryProvider for RepeatingCursorProvider {
        fn discover<'a>(
            &'a self,
            cursor: Option<String>,
        ) -> DiscoveryFuture<'a, Result<DiscoveryPage, DiscoveryProviderError>> {
            Box::pin(async move {
                Ok(DiscoveryPage::new(
                    Vec::new(),
                    Some(cursor.unwrap_or_else(|| "same".to_owned())),
                ))
            })
        }
    }

    struct FailingProvider;

    impl DiscoveryProvider for FailingProvider {
        fn discover<'a>(
            &'a self,
            _cursor: Option<String>,
        ) -> DiscoveryFuture<'a, Result<DiscoveryPage, DiscoveryProviderError>> {
            Box::pin(async {
                Err(Box::new(std::io::Error::other(
                    "simulated discovery failure",
                )))
            })
        }
    }

    fn service() -> DiscoveryService {
        DiscoveryService::new(MembershipRegistry::new())
    }

    #[test]
    fn registers_new_participants_in_authoritative_registry() {
        let service = service();

        service
            .register_provider(
                "static",
                Arc::new(StaticProvider {
                    registrations: vec![registration(1), registration(2)],
                }),
            )
            .expect("provider registration");

        let report = block_on(service.discover_all())
            .expect("discovery scan");

        assert_eq!(report.registered_count(), 2);
        assert_eq!(report.duplicate_count(), 0);
        assert_eq!(report.rejected_count(), 0);
        assert!(report.is_complete());

        assert_eq!(
            service.membership_registry().len().unwrap(),
            2
        );
    }

    #[test]
    fn duplicate_discovery_does_not_overwrite_membership() {
        let service = service();

        service
            .register_provider(
                "static",
                Arc::new(StaticProvider {
                    registrations: vec![registration(1)],
                }),
            )
            .unwrap();

        let first = block_on(service.discover_all()).unwrap();
        assert_eq!(first.registered_count(), 1);

        let second = block_on(service.discover_all()).unwrap();

        assert_eq!(second.registered_count(), 0);
        assert_eq!(second.duplicate_count(), 1);
        assert!(second.is_complete());

        assert_eq!(
            service.membership_registry().len().unwrap(),
            1
        );
    }

    #[test]
    fn discovers_all_pages_without_a_fixed_page_limit() {
        let service = service();

        service
            .register_provider("paged", Arc::new(PaginatedProvider))
            .unwrap();

        let report = block_on(service.discover_all()).unwrap();

        assert_eq!(report.registered_count(), 2);

        match &report.provider_outcomes()[0] {
            ProviderDiscoveryOutcome::Completed {
                pages_retrieved,
                registrations_discovered,
                ..
            } => {
                assert_eq!(*pages_retrieved, 2);
                assert_eq!(*registrations_discovered, 2);
            }
            other => panic!("unexpected provider result: {other:?}"),
        }
    }

    #[test]
    fn repeated_cursor_fails_provider_without_looping_forever() {
        let service = service();

        service
            .register_provider(
                "repeating",
                Arc::new(RepeatingCursorProvider),
            )
            .unwrap();

        let report = block_on(service.discover_all()).unwrap();

        assert!(report.has_provider_failures());

        let outcome = &report.provider_outcomes()[0];

        assert!(matches!(
            outcome,
            ProviderDiscoveryOutcome::Failed { .. }
        ));
    }

    #[test]
    fn provider_failure_does_not_abort_other_providers() {
        let service = service();

        service
            .register_provider("a-failing", Arc::new(FailingProvider))
            .unwrap();

        service
            .register_provider(
                "b-working",
                Arc::new(StaticProvider {
                    registrations: vec![registration(10)],
                }),
            )
            .unwrap();

        let report = block_on(service.discover_all()).unwrap();

        assert!(report.has_provider_failures());
        assert_eq!(report.registered_count(), 1);
        assert_eq!(
            service.membership_registry().len().unwrap(),
            1
        );
    }

    #[test]
    fn provider_replacement_affects_subsequent_scans() {
        let service = service();

        service
            .register_provider(
                "replaceable",
                Arc::new(StaticProvider {
                    registrations: vec![registration(1)],
                }),
            )
            .unwrap();

        let previous = service
            .replace_provider(
                "replaceable",
                Arc::new(StaticProvider {
                    registrations: vec![registration(2)],
                }),
            )
            .unwrap();

        assert!(Arc::strong_count(&previous) >= 1);

        let report = block_on(service.discover_all()).unwrap();

        assert_eq!(report.registered_count(), 1);

        assert!(
            service
                .membership_registry()
                .get(ParticipantId::new(1, 2).unwrap())
                .unwrap()
                .is_some()
        );
    }

    #[test]
    fn provider_ids_are_validated_and_duplicates_rejected() {
        let service = service();
        let provider = Arc::new(FailingProvider);

        assert!(matches!(
            service.register_provider(" ", provider.clone()),
            Err(DiscoveryCatalogError::InvalidProviderId { .. })
        ));

        service
            .register_provider("provider", provider.clone())
            .unwrap();

        assert!(matches!(
            service.register_provider("provider", provider),
            Err(DiscoveryCatalogError::DuplicateProvider { .. })
        ));
    }

    #[test]
    fn unregistering_provider_does_not_delete_membership() {
        let service = service();

        service
            .register_provider(
                "provider",
                Arc::new(StaticProvider {
                    registrations: vec![registration(1)],
                }),
            )
            .unwrap();

        block_on(service.discover_all()).unwrap();

        service.unregister_provider("provider").unwrap();

        assert_eq!(
            service.membership_registry().len().unwrap(),
            1
        );
    }
}
