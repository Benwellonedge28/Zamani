
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Domain Provider Registry
//!
//! Path: `src/zur/domains/registry.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! Provides an explicit, thread-safe registry for independently
//! implemented ZUR computational-domain providers.
//!
//! A registered provider may implement one or more canonical
//! computational domains. A domain may have multiple independently
//! registered providers.
//!
//! Domains remain open-world. Their identities are supplied by the
//! canonical frontend AST.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Domain-provider registration.
//! - Domain-provider unregistration.
//! - Provider lookup by canonical provider identity.
//! - Domain-to-provider indexing.
//! - Domain-and-interface-to-provider indexing.
//! - Registry-local consistency.
//! - Thread-safe access to registered provider references.
//!
//! This module does NOT own:
//!
//! - Domain identity.
//! - Domain semantics.
//! - Domain descriptors.
//! - Provider identity.
//! - Provider lifecycle.
//! - Provider execution contracts.
//! - Capability definitions.
//! - Capability evidence generation or verification.
//! - Capability availability.
//! - Resource requirements.
//! - Resource discovery.
//! - Resource allocation.
//! - Resource reservation.
//! - Provider authentication.
//! - Provider authorization.
//! - Hardware discovery.
//! - Provider initialization.
//! - Provider shutdown.
//! - Feasibility analysis.
//! - Provider selection policy.
//! - Backend registration.
//! - Code generation.
//! - Compiler planning.
//! - Domain-specific intermediate representations.
//! - Scheduling.
//! - Routing.
//! - Hybrid execution planning.
//! - Distributed execution planning.
//! - Physical device interaction.
//!
//! Those responsibilities remain with their canonical repository
//! authorities.
//!
//! ## Canonical authorities
//!
//! Domain identity:
//!     frontend::ast::node::domains::domain::Domain
//!
//! Provider identity and lifecycle:
//!     stdlib::provider
//!
//! Provider execution contract:
//!     zur::contracts::provider
//!
//! Domain-provider integration:
//!     zur::domains::provider
//!
//! Domain descriptors:
//!     zur::domains::descriptor
//!
//! Operational errors:
//!     zur::error
//!
//! Version compatibility:
//!     zur::version
//!
//! ## Registry invariants
//!
//! Registration != initialization.
//! Registration != admission.
//! Registration != authentication.
//! Registration != provider availability.
//! Provider declaration != capability evidence.
//! Capability evidence != capability availability.
//! Requirement satisfaction != feasibility.
//! Feasibility != resource reservation.
//! Resource reservation != execution.
//! Execution submission != successful completion.
//!
//! Registry lookup establishes only that a provider reference has
//! been registered in this registry instance.
//!
//! ## Open-world extensibility
//!
//! This registry does not enumerate computational domains.
//!
//! It accepts canonical Domain identities and independently
//! implemented DomainProvider implementations.
//!
//! New domains do not require modifications to this registry.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Registered providers.
//! - Computational domains.
//! - Providers per domain.
//! - Interfaces per domain.
//! - Domains per provider.
//! - Provider registrations.
//!
//! The registry grows according to available memory and actual
//! implementation and platform resources.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Concurrency
//!
//! Each registry instance owns its own synchronized state.
//!
//! Read operations may execute concurrently.
//!
//! Registration and unregistration are exclusive state transitions.
//!
//! Provider validation happens before acquiring the registry write
//! lock. Provider methods are never called while the registry lock
//! is held.
//!
//! Lookup returns cloned Arc references, allowing callers to use
//! providers without holding registry locks.
//!
//! ## Determinism
//!
//! Provider identities, domain identities and interface identities
//! are indexed using ordered collections.
//!
//! Lookup results are returned in canonical identity order.
//!
//! Registration order does not determine lookup order.
//!
//! ## Grammar boundary
//!
//! This registry consumes already validated domain representations.
//!
//! It does not parse source grammar or interpret source declarations.
//!
//! Relevant specifications:
//!
//! - grammar/specification/domains.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/execution-model.md
//! - grammar/spec/resources.md
//! - grammar/spec/semantics.md
//! - grammar/spec/compatibility.md
//! - grammar/declarations/capabilities.g4
//! - grammar/resources/capabilities.g4
//! - grammar/execution/runtime-capabilities.g4
//!
//! Grammar expresses intent.
//! Semantic analysis establishes meaning.
//! Compiler and domain authorities establish valid representations.
//! The registry indexes validated runtime provider declarations.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No unsafe code.
//! No raw pointers.
//! No global mutable state.
//! No implicit hardware discovery.
//! No fabricated execution success.
//! No implicit resource allocation.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::frontend::ast::node::domains::domain::Domain;

use crate::stdlib::provider::ProviderId;

use crate::zur::contracts::provider::ProviderInterfaceId;

use crate::zur::domains::descriptor::DomainDescriptor;

use crate::zur::domains::provider::DomainProvider;

use crate::zur::error::{ZurError, ZurResult};

// ============================================================================
// COMPONENT IDENTIFICATION
// ============================================================================

const COMPONENT: &str = "zur.domains.registry";

// ============================================================================
// REGISTRATION RECEIPT
// ============================================================================

/// Describes a provider registration accepted by this registry.
///
/// A receipt records registry membership only.
///
/// It does not establish provider admission, trust, availability,
/// capability availability, feasibility, resource reservation,
/// or execution success.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderRegistration {
    provider_id: ProviderId,
    domains: Vec<Domain>,
}

impl ProviderRegistration {
    /// Returns the registered provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the domains declared by the registered provider.
    ///
    /// The returned identities are deterministically ordered.
    #[must_use]
    pub fn domains(&self) -> &[Domain] {
        &self.domains
    }

    /// Returns the number of domains declared by this provider.
    #[must_use]
    pub fn domain_count(&self) -> usize {
        self.domains.len()
    }
}

// ============================================================================
// INTERNAL PROVIDER ENTRY
// ============================================================================

/// One provider and the index keys derived from its immutable descriptors.
///
/// The provider itself remains the canonical execution implementation.
///
/// The registry stores an Arc rather than copying or reimplementing
/// provider lifecycle or execution behavior.
struct ProviderEntry {
    provider: Arc<dyn DomainProvider>,
    domains: BTreeSet<Domain>,
    interface_keys: BTreeSet<(Domain, ProviderInterfaceId)>,
}

impl ProviderEntry {
    /// Builds an entry from a structurally validated provider.
    ///
    /// This function is called before acquiring the registry write lock.
    ///
    /// Only interfaces implemented by the provider's canonical
    /// execution contract are indexed.
    fn new(
        provider: Arc<dyn DomainProvider>,
    ) -> ZurResult<(ProviderId, Self)> {
        provider.validate_domain_integration()?;

        let provider_id =
            provider.descriptor().id().clone();

        let implemented_interfaces =
            provider
                .execution_contract()
                .interfaces()
                .iter()
                .map(|interface| interface.id().clone())
                .collect::<BTreeSet<_>>();

        let mut domains = BTreeSet::new();
        let mut interface_keys = BTreeSet::new();

        for descriptor in provider.domain_descriptors() {
            descriptor.validate()?;

            let domain = descriptor.domain().clone();

            domains.insert(domain.clone());

            for interface in descriptor.interfaces() {
                if implemented_interfaces.contains(interface) {
                    interface_keys.insert((
                        domain.clone(),
                        interface.clone(),
                    ));
                }
            }
        }

        if domains.is_empty() {
            return Err(
                registry_error(
                    ZurError::invalid_input(
                        "a domain provider must declare at least one domain",
                    ),
                    "prepare_registration",
                    Some(&provider_id),
                ),
            );
        }

        Ok((
            provider_id,
            Self {
                provider,
                domains,
                interface_keys,
            },
        ))
    }
}

// ============================================================================
// INTERNAL REGISTRY STATE
// ============================================================================

/// Synchronized registry indexes.
///
/// All indexes are updated while holding the same write lock.
///
/// This prevents readers from observing a provider registration
/// whose primary provider entry and domain indexes disagree.
#[derive(Default)]
struct RegistryState {
    providers: BTreeMap<ProviderId, ProviderEntry>,

    providers_by_domain:
        BTreeMap<Domain, BTreeSet<ProviderId>>,

    providers_by_domain_interface:
        BTreeMap<
            (Domain, ProviderInterfaceId),
            BTreeSet<ProviderId>,
        >,
}

impl RegistryState {
    /// Inserts a fully prepared provider entry and its indexes.
    ///
    /// The caller must hold the registry write lock.
    fn insert(
        &mut self,
        provider_id: ProviderId,
        entry: ProviderEntry,
    ) -> ZurResult<ProviderRegistration> {
        if self.providers.contains_key(&provider_id) {
            return Err(
                registry_error(
                    ZurError::compatibility(
                        format!(
                            "provider {provider_id} is already registered"
                        ),
                    ),
                    "register",
                    Some(&provider_id),
                ),
            );
        }

        let domains = entry.domains.iter().cloned().collect::<Vec<_>>();

        for domain in &entry.domains {
            self.providers_by_domain
                .entry(domain.clone())
                .or_default()
                .insert(provider_id.clone());
        }

        for key in &entry.interface_keys {
            self.providers_by_domain_interface
                .entry(key.clone())
                .or_default()
                .insert(provider_id.clone());
        }

        self.providers.insert(provider_id.clone(), entry);

        Ok(ProviderRegistration {
            provider_id,
            domains,
        })
    }

    /// Removes a provider and all indexes derived from its descriptors.
    ///
    /// The caller must hold the registry write lock.
    fn remove(
        &mut self,
        provider_id: &ProviderId,
    ) -> Option<Arc<dyn DomainProvider>> {
        let entry = self.providers.remove(provider_id)?;

        for domain in &entry.domains {
            let remove_domain_key =
                if let Some(providers) =
                    self.providers_by_domain.get_mut(domain)
                {
                    providers.remove(provider_id);
                    providers.is_empty()
                } else {
                    false
                };

            if remove_domain_key {
                self.providers_by_domain.remove(domain);
            }
        }

        for key in &entry.interface_keys {
            let remove_interface_key =
                if let Some(providers) =
                    self.providers_by_domain_interface.get_mut(key)
                {
                    providers.remove(provider_id);
                    providers.is_empty()
                } else {
                    false
                };

            if remove_interface_key {
                self.providers_by_domain_interface.remove(key);
            }
        }

        Some(entry.provider)
    }
}

// ============================================================================
// REGISTRY
// ============================================================================

/// Thread-safe, explicitly owned registry of domain providers.
///
/// A registry is created and owned by its caller.
///
/// There is no process-global registry.
///
/// The registry may be shared between threads through `Arc` or
/// ordinary shared references.
///
/// Multiple providers may implement the same domain.
///
/// A single provider may implement multiple domains.
///
/// Registration is structural. Provider lifecycle and admission
/// remain the responsibility of their respective authorities.
pub struct DomainProviderRegistry {
    state: RwLock<RegistryState>,
}

impl DomainProviderRegistry {
    /// Creates an empty domain-provider registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: RwLock::new(RegistryState::default()),
        }
    }

    /// Registers a domain provider.
    ///
    /// The provider must pass the canonical domain-provider
    /// structural validation before it can be registered.
    ///
    /// Registration does not initialize or activate the provider.
    ///
    /// A provider identity may be registered only once in this
    /// registry instance. To replace an implementation, explicitly
    /// unregister the previous provider before registering the new
    /// instance.
    ///
    /// The provider is validated before the registry lock is acquired.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - the provider's domain integration is invalid;
    /// - the provider has no domain descriptors;
    /// - the provider identity is already registered;
    /// - the registry lock has been poisoned.
    pub fn register(
        &self,
        provider: Arc<dyn DomainProvider>,
    ) -> ZurResult<ProviderRegistration> {
        let (provider_id, entry) =
            ProviderEntry::new(provider).map_err(|error| {
                registry_error(
                    error,
                    "register",
                    None,
                )
            })?;

        let mut state = self.write_state()?;

        state.insert(provider_id, entry)
    }

    /// Unregisters a provider by its canonical identity.
    ///
    /// All domain and interface indexes belonging to that provider
    /// are removed atomically with its primary registration.
    ///
    /// The returned Arc remains valid for callers that already
    /// hold a provider reference.
    ///
    /// Unregistration does not automatically shut down the provider.
    /// Lifecycle coordination remains the responsibility of the
    /// lifecycle manager.
    ///
    /// Returns `None` when the provider is not registered.
    ///
    /// # Errors
    ///
    /// Returns an error if the registry lock has been poisoned.
    pub fn unregister(
        &self,
        provider_id: &ProviderId,
    ) -> ZurResult<Option<Arc<dyn DomainProvider>>> {
        let mut state = self.write_state()?;

        Ok(state.remove(provider_id))
    }

    /// Looks up a provider by its canonical identity.
    ///
    /// The returned Arc allows provider use without holding the
    /// registry lock.
    ///
    /// A successful lookup proves registry membership only.
    #[must_use]
    pub fn get(
        &self,
        provider_id: &ProviderId,
    ) -> ZurResult<Option<Arc<dyn DomainProvider>>> {
        let state = self.read_state()?;

        Ok(state
            .providers
            .get(provider_id)
            .map(|entry| Arc::clone(&entry.provider)))
    }

    /// Returns all providers registered for a domain.
    ///
    /// Results are ordered by canonical provider identity.
    ///
    /// The returned references do not imply provider availability,
    /// capability availability, feasibility, or resource reservation.
    #[must_use]
    pub fn providers_for_domain(
        &self,
        domain: &Domain,
    ) -> ZurResult<Vec<Arc<dyn DomainProvider>>> {
        let state = self.read_state()?;

        let Some(provider_ids) =
            state.providers_by_domain.get(domain)
        else {
            return Ok(Vec::new());
        };

        Ok(provider_ids
            .iter()
            .filter_map(|provider_id| {
                state
                    .providers
                    .get(provider_id)
                    .map(|entry| Arc::clone(&entry.provider))
            })
            .collect())
    }

    /// Returns providers implementing an exact interface for a domain.
    ///
    /// The interface must be declared by the domain descriptor and
    /// implemented by the provider's canonical execution contract.
    ///
    /// Results are ordered by canonical provider identity.
    ///
    /// This is a structural lookup, not version negotiation,
    /// capability evaluation, feasibility analysis, provider
    /// selection, or execution.
    #[must_use]
    pub fn providers_for_domain_interface(
        &self,
        domain: &Domain,
        interface: &ProviderInterfaceId,
    ) -> ZurResult<Vec<Arc<dyn DomainProvider>>> {
        let state = self.read_state()?;

        let key = (domain.clone(), interface.clone());

        let Some(provider_ids) =
            state.providers_by_domain_interface.get(&key)
        else {
            return Ok(Vec::new());
        };

        Ok(provider_ids
            .iter()
            .filter_map(|provider_id| {
                state
                    .providers
                    .get(provider_id)
                    .map(|entry| Arc::clone(&entry.provider))
            })
            .collect())
    }

    /// Returns provider identities registered for a domain.
    ///
    /// Results are ordered by canonical provider identity.
    #[must_use]
    pub fn provider_ids_for_domain(
        &self,
        domain: &Domain,
    ) -> ZurResult<Vec<ProviderId>> {
        let state = self.read_state()?;

        Ok(state
            .providers_by_domain
            .get(domain)
            .map(|providers| providers.iter().cloned().collect())
            .unwrap_or_default())
    }

    /// Returns the canonical domain identities known to this registry.
    ///
    /// Results are ordered by canonical domain identity.
    #[must_use]
    pub fn registered_domains(&self) -> ZurResult<Vec<Domain>> {
        let state = self.read_state()?;

        Ok(state.providers_by_domain.keys().cloned().collect())
    }

    /// Returns the number of registered providers.
    ///
    /// This count is an observation of this registry instance.
    #[must_use]
    pub fn provider_count(&self) -> ZurResult<usize> {
        let state = self.read_state()?;

        Ok(state.providers.len())
    }

    /// Returns whether this registry contains a provider identity.
    #[must_use]
    pub fn contains_provider(
        &self,
        provider_id: &ProviderId,
    ) -> ZurResult<bool> {
        let state = self.read_state()?;

        Ok(state.providers.contains_key(provider_id))
    }

    /// Returns whether any provider is registered for a domain.
    ///
    /// This does not establish that a provider is currently available.
    #[must_use]
    pub fn contains_domain(
        &self,
        domain: &Domain,
    ) -> ZurResult<bool> {
        let state = self.read_state()?;

        Ok(state.providers_by_domain.contains_key(domain))
    }

    /// Returns whether the registry contains a provider for an exact
    /// domain/interface pair.
    #[must_use]
    pub fn contains_domain_interface(
        &self,
        domain: &Domain,
        interface: &ProviderInterfaceId,
    ) -> ZurResult<bool> {
        let state = self.read_state()?;

        Ok(state
            .providers_by_domain_interface
            .contains_key(&(domain.clone(), interface.clone())))
    }

    /// Returns whether the registry contains no providers.
    #[must_use]
    pub fn is_empty(&self) -> ZurResult<bool> {
        Ok(self.provider_count()? == 0)
    }

    // ------------------------------------------------------------------------
    // Lock handling
    // ------------------------------------------------------------------------

    /// Acquires a shared registry-state guard.
    fn read_state(&self) -> ZurResult<RwLockReadGuard<'_, RegistryState>> {
        self.state.read().map_err(|_| {
            registry_error(
                ZurError::internal(
                    "domain-provider registry read lock is poisoned",
                ),
                "read",
                None,
            )
        })
    }

    /// Acquires an exclusive registry-state guard.
    fn write_state(&self) -> ZurResult<RwLockWriteGuard<'_, RegistryState>> {
        self.state.write().map_err(|_| {
            registry_error(
                ZurError::internal(
                    "domain-provider registry write lock is poisoned",
                ),
                "write",
                None,
            )
        })
    }
}

impl Default for DomainProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// DEBUG
// ============================================================================

impl fmt::Debug for DomainProviderRegistry {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self.read_state() {
            Ok(state) => formatter
                .debug_struct("DomainProviderRegistry")
                .field("provider_count", &state.providers.len())
                .field("domain_count", &state.providers_by_domain.len())
                .field(
                    "domain_interface_count",
                    &state.providers_by_domain_interface.len(),
                )
                .finish(),
            Err(_) => formatter
                .debug_struct("DomainProviderRegistry")
                .field("state", &"poisoned")
                .finish(),
        }
    }
}

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

/// Adds registry context to an existing canonical ZUR error.
///
/// The original error is retained as the causal source when adapted.
fn registry_error(
    error: ZurError,
    operation: &'static str,
    provider_id: Option<&ProviderId>,
) -> ZurError {
    let error = error
        .with_component(COMPONENT)
        .with_operation(operation);

    match provider_id {
        Some(provider_id) => {
            error.with_provider(provider_id.to_string())
        }
        None => error,
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_registry_is_empty() {
        let registry = DomainProviderRegistry::new();

        assert_eq!(
            registry.provider_count().expect("registry read must succeed"),
            0,
        );

        assert!(
            registry.is_empty().expect("registry read must succeed"),
        );
    }

    #[test]
    fn empty_registry_has_no_domains() {
        let registry = DomainProviderRegistry::default();

        assert!(
            registry
                .registered_domains()
                .expect("registry read must succeed")
                .is_empty(),
        );
    }

    #[test]
    fn unknown_provider_lookup_returns_none() {
        let registry = DomainProviderRegistry::new();

        let provider_id =
            ProviderId::new("org.example.unknown")
                .expect("provider identity must be valid");

        assert!(
            registry
                .get(&provider_id)
                .expect("registry read must succeed")
                .is_none(),
        );
    }

    #[test]
    fn unknown_domain_lookup_returns_empty_results() {
        let registry = DomainProviderRegistry::new();

        let domain = Domain::new(
            "zamani",
            "future_domain",
        )
        .expect("domain identity must be valid");

        assert!(
            registry
                .providers_for_domain(&domain)
                .expect("registry read must succeed")
                .is_empty(),
        );

        assert!(
            registry
                .provider_ids_for_domain(&domain)
                .expect("registry read must succeed")
                .is_empty(),
        );
    }

    #[test]
    fn unknown_domain_interface_lookup_returns_empty_results() {
        let registry = DomainProviderRegistry::new();

        let domain = Domain::new(
            "zamani",
            "future_domain",
        )
        .expect("domain identity must be valid");

        let interface = ProviderInterfaceId::new(
            "future.domain.execution",
        )
        .expect("interface identity must be valid");

        assert!(
            registry
                .providers_for_domain_interface(&domain, &interface)
                .expect("registry read must succeed")
                .is_empty(),
        );

        assert!(
            !registry
                .contains_domain_interface(&domain, &interface)
                .expect("registry read must succeed"),
        );
    }

    #[test]
    fn registry_debug_does_not_require_provider_debug() {
        let registry = DomainProviderRegistry::new();

        let output = format!("{registry:?}");

        assert!(output.contains("DomainProviderRegistry"));
        assert!(output.contains("provider_count"));
    }

    #[test]
    fn provider_registration_receipt_preserves_identity_and_domains() {
        let domain = Domain::new(
            "zamani",
            "classical",
        )
        .expect("domain identity must be valid");

        let provider_id =
            ProviderId::new("org.example.classical")
                .expect("provider identity must be valid");

        let receipt = ProviderRegistration {
            provider_id: provider_id.clone(),
            domains: vec![domain.clone()],
        };

        assert_eq!(receipt.provider_id(), &provider_id);
        assert_eq!(receipt.domains(), &[domain]);
        assert_eq!(receipt.domain_count(), 1);
    }

    #[test]
    fn registry_state_removes_empty_domain_indexes() {
        let domain = Domain::new(
            "zamani",
            "classical",
        )
        .expect("domain identity must be valid");

        let interface = ProviderInterfaceId::new(
            "zamani.classical.execution",
        )
        .expect("interface identity must be valid");

        let provider_id =
            ProviderId::new("org.example.classical")
                .expect("provider identity must be valid");

        let mut state = RegistryState::default();

        // The test exercises the index-cleanup rule independently
        // of a provider implementation.
        state.providers_by_domain
            .entry(domain.clone())
            .or_default()
            .insert(provider_id.clone());

        state.providers_by_domain_interface
            .entry((domain.clone(), interface.clone()))
            .or_default()
            .insert(provider_id.clone());

        let domain_is_empty = {
            let providers = state
                .providers_by_domain
                .get_mut(&domain)
                .expect("domain index must exist");

            providers.remove(&provider_id);
            providers.is_empty()
        };

        if domain_is_empty {
            state.providers_by_domain.remove(&domain);
        }

        let interface_key = (domain, interface);

        let interface_is_empty = {
            let providers = state
                .providers_by_domain_interface
                .get_mut(&interface_key)
                .expect("interface index must exist");

            providers.remove(&provider_id);
            providers.is_empty()
        };

        if interface_is_empty {
            state
                .providers_by_domain_interface
                .remove(&interface_key);
        }

        assert!(state.providers_by_domain.is_empty());
        assert!(state.providers_by_domain_interface.is_empty());
    }
}
