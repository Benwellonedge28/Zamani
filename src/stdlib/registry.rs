
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Provider Registry
//!
//! Path: `src/stdlib/registry.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides thread-safe registration, removal, replacement, lookup,
//! enumeration and immutable snapshots of standard-library providers.
//!
//! Providers may implement classical, quantum, HDL, hybrid, distributed,
//! AI, scientific, networking, storage, security or future capabilities.
//!
//! The registry is independent of the computational domain.
//!
//! ## OWNS
//!
//! - Provider registration.
//! - Provider identity uniqueness within a registry.
//! - Provider removal.
//! - Explicit provider replacement.
//! - Provider metadata indexing.
//! - Provider lookup.
//! - Provider enumeration.
//! - Immutable registry snapshots.
//! - Registry generation tracking.
//! - Optimistic concurrency checks.
//!
//! ## DOES NOT OWN
//!
//! - Provider implementation.
//! - Provider initialization or shutdown.
//! - Hardware discovery.
//! - Provider authentication or trust.
//! - Provider admission or authorization.
//! - Capability discovery.
//! - Semantic requirement evaluation.
//! - Preference scoring.
//! - Constraint evaluation.
//! - Feasibility determination.
//! - Provider selection.
//! - Resource allocation.
//! - Scheduling or routing.
//! - Compiler extension registration.
//! - Quantum hardware registration.
//! - Runtime execution.
//!
//! These responsibilities remain with their canonical authorities.
//!
//! ## AUTHORITATIVE DEPENDENCIES
//!
//! - `crate::stdlib::provider`
//! - `crate::stdlib::capability`
//! - `crate::stdlib::version`
//! - `crate::stdlib::error`
//!
//! ## CONCURRENCY
//!
//! The registry uses `Arc<RwLock<_>>`.
//!
//! Read operations can execute concurrently.
//!
//! Mutations are serialized.
//!
//! Provider callbacks are never invoked while a registry lock is held.
//!
//! Poisoned locks produce structured errors rather than silently
//! recovering potentially inconsistent state.
//!
//! ## SNAPSHOTS
//!
//! A snapshot represents one consistent registry generation.
//!
//! Snapshots retain provider references through Arc.
//!
//! Later registry mutations do not alter existing snapshots.
//!
//! ## VERSIONING
//!
//! Provider API versions are distinct from implementation versions,
//! language versions, grammar versions and IR schema versions.
//!
//! Exact API-version matching is available through explicit lookup.
//!
//! This registry does not independently implement package dependency
//! resolution or semantic-version compatibility policy.
//!
//! ## POCO-REAF
//!
//! Provider identities, kinds, operations and computational domains
//! remain open-world.
//!
//! No closed provider/vendor enumeration exists.
//!
//! No arbitrary provider-count, operation-count, capability-count,
//! hardware-count or computational-domain limit is imposed.
//!
//! Collections grow according to available resources.
//!
//! Literal infinite physical storage or computation is not promised.

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::stdlib::error::{
    Error,
    ErrorKind,
    Result,
};

use crate::stdlib::provider::{
    Provider,
    ProviderDescriptor,
    ProviderId,
    ProviderKind,
    ProviderOperationId,
    ProviderStatus,
};

use crate::stdlib::capability::CapabilityId;

use crate::stdlib::version::Version;

// ============================================================================
// CONSTANTS
// ============================================================================

const REGISTRY_COMPONENT: &str = "stdlib.registry";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn registry_error(
    kind: ErrorKind,
    message: impl Into<String>,
) -> Error {
    Error::new(kind, message)
        .in_component(REGISTRY_COMPONENT)
}

fn not_found(
    message: impl Into<String>,
) -> Error {
    registry_error(ErrorKind::NotFound, message)
}

fn duplicate(
    message: impl Into<String>,
) -> Error {
    registry_error(ErrorKind::AlreadyExists, message)
}

fn concurrency_error(
    message: impl Into<String>,
) -> Error {
    registry_error(ErrorKind::Concurrency, message)
}

fn poisoned_lock() -> Error {
    registry_error(
        ErrorKind::Internal,
        "provider registry lock is poisoned",
    )
}

// ============================================================================
// REGISTRY STATE
// ============================================================================

/// Internal mutable registry state.
///
/// All entries are ordered by provider identity.
///
/// This ensures deterministic enumeration independently of
/// registration order.
#[derive(Default)]
struct RegistryState {
    generation: u64,
    providers: BTreeMap<ProviderId, RegisteredProvider>,
}

// ============================================================================
// REGISTERED PROVIDER
// ============================================================================

/// A provider registered in the standard-library registry.
///
/// The descriptor is an immutable registration-time metadata snapshot.
///
/// The provider is retained through Arc, allowing snapshots and callers
/// to safely share provider ownership.
///
/// A registered provider is not necessarily initialized, trusted,
//! authorized, available or feasible for a particular operation.
#[derive(Clone)]
pub struct RegisteredProvider {
    descriptor: ProviderDescriptor,
    provider: Arc<dyn Provider>,
    registered_generation: u64,
}

impl RegisteredProvider {
    /// Returns the immutable provider descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ProviderDescriptor {
        &self.descriptor
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn id(&self) -> &ProviderId {
        self.descriptor.id()
    }

    /// Returns the provider kind.
    #[must_use]
    pub fn kind(&self) -> &ProviderKind {
        self.descriptor.kind()
    }

    /// Returns the provider API version.
    #[must_use]
    pub fn api_version(&self) -> &Version {
        self.descriptor.api_version()
    }

    /// Returns the provider implementation version.
    #[must_use]
    pub fn implementation_version(&self) -> &Version {
        self.descriptor.implementation_version()
    }

    /// Returns the registry generation in which this registration
    /// became effective.
    #[must_use]
    pub const fn registered_generation(&self) -> u64 {
        self.registered_generation
    }

    /// Returns a shared reference to the provider implementation.
    ///
    /// This does not initialize or execute the provider.
    #[must_use]
    pub fn provider(&self) -> &Arc<dyn Provider> {
        &self.provider
    }

    /// Returns a cloned shared reference to the provider.
    #[must_use]
    pub fn provider_arc(&self) -> Arc<dyn Provider> {
        Arc::clone(&self.provider)
    }

    /// Observes the provider's current lifecycle status.
    ///
    /// This invokes the provider's status implementation.
    ///
    /// The caller must not interpret the result as a guarantee
    /// of future availability.
    #[must_use]
    pub fn observed_status(&self) -> ProviderStatus {
        self.provider.status()
    }

    /// Returns whether the provider declares a capability.
    ///
    /// This checks registration metadata only.
    ///
    /// It does not establish current capability availability.
    #[must_use]
    pub fn declares_capability(
        &self,
        capability: &CapabilityId,
    ) -> bool {
        self.descriptor
            .capabilities()
            .iter()
            .any(|item| item.id() == capability)
    }

    /// Returns whether the provider declares an operation.
    #[must_use]
    pub fn declares_operation(
        &self,
        operation: &ProviderOperationId,
    ) -> bool {
        self.descriptor
            .operations()
            .iter()
            .any(|item| item.id() == operation)
    }
}

impl std::fmt::Debug for RegisteredProvider {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        formatter
            .debug_struct("RegisteredProvider")
            .field("id", self.id())
            .field("kind", self.kind())
            .field("api_version", self.api_version())
            .field(
                "implementation_version",
                self.implementation_version(),
            )
            .field(
                "registered_generation",
                &self.registered_generation,
            )
            .finish_non_exhaustive()
    }
}

// ============================================================================
// REGISTRATION RECEIPT
// ============================================================================

/// Confirmation metadata for a successful registration or replacement.
///
/// The receipt describes the registry mutation.
///
/// It does not indicate that the provider has been initialized,
/// activated, authenticated or made available for execution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegistrationReceipt {
    provider_id: ProviderId,
    generation: u64,
    replaced: bool,
}

impl RegistrationReceipt {
    /// Returns the registered provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the generation after the mutation.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns whether this operation replaced an existing provider.
    #[must_use]
    pub const fn replaced(&self) -> bool {
        self.replaced
    }
}

// ============================================================================
// REGISTRY SNAPSHOT
// ============================================================================

/// An immutable view of one registry generation.
///
/// A snapshot remains valid after subsequent registrations,
/// replacements or removals.
///
/// Provider implementations remain alive while the snapshot
/// retains their Arc references.
#[derive(Clone, Debug)]
pub struct RegistrySnapshot {
    generation: u64,
    providers: Arc<[RegisteredProvider]>,
}

impl RegistrySnapshot {
    /// Returns the generation represented by this snapshot.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns the number of registered providers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Returns whether the snapshot contains no providers.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    /// Returns all providers in deterministic identity order.
    #[must_use]
    pub fn providers(&self) -> &[RegisteredProvider] {
        &self.providers
    }

    /// Finds a provider by exact identity.
    #[must_use]
    pub fn get(
        &self,
        id: &ProviderId,
    ) -> Option<&RegisteredProvider> {
        self.providers
            .binary_search_by(|provider| provider.id().cmp(id))
            .ok()
            .map(|index| &self.providers[index])
    }

    /// Finds providers of an exact provider kind.
    ///
    /// Results preserve deterministic identity ordering.
    #[must_use]
    pub fn by_kind<'a>(
        &'a self,
        kind: &'a ProviderKind,
    ) -> impl Iterator<Item = &'a RegisteredProvider> + 'a {
        self.providers
            .iter()
            .filter(move |provider| provider.kind() == kind)
    }

    /// Finds providers declaring a particular capability.
    ///
    /// Capability declarations are metadata, not availability evidence.
    #[must_use]
    pub fn by_capability<'a>(
        &'a self,
        capability: &'a CapabilityId,
    ) -> impl Iterator<Item = &'a RegisteredProvider> + 'a {
        self.providers
            .iter()
            .filter(move |provider| {
                provider.declares_capability(capability)
            })
    }

    /// Finds providers declaring a particular operation.
    #[must_use]
    pub fn by_operation<'a>(
        &'a self,
        operation: &'a ProviderOperationId,
    ) -> impl Iterator<Item = &'a RegisteredProvider> + 'a {
        self.providers
            .iter()
            .filter(move |provider| {
                provider.declares_operation(operation)
            })
    }

    /// Finds providers with an exact API-version match.
    ///
    /// Exact matching deliberately avoids silently interpreting
    /// a provider API compatibility policy.
    #[must_use]
    pub fn by_api_version<'a>(
        &'a self,
        version: &'a Version,
    ) -> impl Iterator<Item = &'a RegisteredProvider> + 'a {
        self.providers
            .iter()
            .filter(move |provider| {
                provider.api_version() == version
            })
    }

    /// Finds providers whose declared implementation version
    /// exactly matches the supplied version.
    #[must_use]
    pub fn by_implementation_version<'a>(
        &'a self,
        version: &'a Version,
    ) -> impl Iterator<Item = &'a RegisteredProvider> + 'a {
        self.providers
            .iter()
            .filter(move |provider| {
                provider.implementation_version() == version
            })
    }

    /// Finds providers whose currently observed status may accept
    /// operations.
    ///
    /// Status is queried outside registry locks.
    ///
    /// This is an observational query, not provider selection or
    /// a guarantee of future availability.
    #[must_use]
    pub fn potentially_ready(
        &self,
    ) -> Vec<&RegisteredProvider> {
        self.providers
            .iter()
            .filter(|provider| {
                provider
                    .observed_status()
                    .may_accept_operations()
            })
            .collect()
    }

    /// Returns the provider identities in deterministic order.
    #[must_use]
    pub fn provider_ids(&self) -> Vec<&ProviderId> {
        self.providers
            .iter()
            .map(RegisteredProvider::id)
            .collect()
    }
}

// ============================================================================
// PROVIDER REGISTRY
// ============================================================================

/// Thread-safe, extensible provider registry.
///
/// The registry owns provider registration metadata and shared
/// provider references.
///
/// It does not own provider lifecycle or execution.
///
/// Multiple independent registry instances may exist.
///
/// This permits separate application, compilation, runtime,
/// testing or deployment scopes without a mandatory global registry.
#[derive(Clone, Default)]
pub struct ProviderRegistry {
    state: Arc<RwLock<RegistryState>>,
}

impl ProviderRegistry {
    /// Creates an empty provider registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the current registry generation.
    ///
    /// Every successful mutation advances the generation.
    ///
    /// # Errors
    ///
    /// Returns an error if the internal lock is poisoned.
    pub fn generation(&self) -> Result<u64> {
        let state = self.state.read()
            .map_err(|_| poisoned_lock())?;

        Ok(state.generation)
    }

    /// Returns the current number of registered providers.
    ///
    /// # Errors
    ///
    /// Returns an error if the internal lock is poisoned.
    pub fn len(&self) -> Result<usize> {
        let state = self.state.read()
            .map_err(|_| poisoned_lock())?;

        Ok(state.providers.len())
    }

    /// Returns whether the registry is empty.
    ///
    /// # Errors
    ///
    /// Returns an error if the internal lock is poisoned.
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }

    // ------------------------------------------------------------------------
    // REGISTRATION
    // ------------------------------------------------------------------------

    /// Registers a provider.
    ///
    /// Provider identity must be unique within this registry.
    ///
    /// The descriptor is captured before acquiring the registry lock.
    ///
    /// Registration does not initialize, activate, authenticate,
    /// authorize or execute the provider.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The identity is already registered.
    /// - The registry lock is poisoned.
    /// - The registry generation cannot advance.
    pub fn register(
        &self,
        provider: Arc<dyn Provider>,
    ) -> Result<RegistrationReceipt> {
        let descriptor = provider.descriptor().clone();
        let id = descriptor.id().clone();

        let mut state = self.state.write()
            .map_err(|_| poisoned_lock())?;

        if state.providers.contains_key(&id) {
            return Err(duplicate(
                format!("provider {id} is already registered"),
            ));
        }

        let generation = next_generation(state.generation)?;

        let registered = RegisteredProvider {
            descriptor,
            provider,
            registered_generation: generation,
        };

        state.providers.insert(
            id.clone(),
            registered,
        );

        state.generation = generation;

        Ok(RegistrationReceipt {
            provider_id: id,
            generation,
            replaced: false,
        })
    }

    /// Replaces an existing provider with the same identity.
    ///
    /// Replacement is explicit and atomic from the registry's
    /// perspective.
    ///
    /// Existing snapshots retain their original provider references.
    ///
    /// The previous provider is not shut down.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider does not exist,
    /// the lock is poisoned or the generation cannot advance.
    pub fn replace(
        &self,
        provider: Arc<dyn Provider>,
    ) -> Result<RegistrationReceipt> {
        let descriptor = provider.descriptor().clone();
        let id = descriptor.id().clone();

        let mut state = self.state.write()
            .map_err(|_| poisoned_lock())?;

        if !state.providers.contains_key(&id) {
            return Err(not_found(
                format!("provider {id} is not registered"),
            ));
        }

        let generation = next_generation(state.generation)?;

        let registered = RegisteredProvider {
            descriptor,
            provider,
            registered_generation: generation,
        };

        state.providers.insert(
            id.clone(),
            registered,
        );

        state.generation = generation;

        Ok(RegistrationReceipt {
            provider_id: id,
            generation,
            replaced: true,
        })
    }

    /// Registers a provider only if the registry generation matches
    /// the caller's expected generation.
    ///
    /// This prevents accidental mutation based on stale observations.
    ///
    /// # Errors
    ///
    /// Returns a concurrency error if the expected generation differs
    /// from the current generation.
    pub fn register_if_generation(
        &self,
        expected_generation: u64,
        provider: Arc<dyn Provider>,
    ) -> Result<RegistrationReceipt> {
        let descriptor = provider.descriptor().clone();
        let id = descriptor.id().clone();

        let mut state = self.state.write()
            .map_err(|_| poisoned_lock())?;

        verify_generation(
            state.generation,
            expected_generation,
        )?;

        if state.providers.contains_key(&id) {
            return Err(duplicate(
                format!("provider {id} is already registered"),
            ));
        }

        let generation = next_generation(state.generation)?;

        state.providers.insert(
            id.clone(),
            RegisteredProvider {
                descriptor,
                provider,
                registered_generation: generation,
            },
        );

        state.generation = generation;

        Ok(RegistrationReceipt {
            provider_id: id,
            generation,
            replaced: false,
        })
    }

    /// Replaces an existing provider only if the expected generation
    /// matches the current registry generation.
    ///
    /// # Errors
    ///
    /// Returns a concurrency error for stale generations and a
    /// not-found error if the provider is not registered.
    pub fn replace_if_generation(
        &self,
        expected_generation: u64,
        provider: Arc<dyn Provider>,
    ) -> Result<RegistrationReceipt> {
        let descriptor = provider.descriptor().clone();
        let id = descriptor.id().clone();

        let mut state = self.state.write()
            .map_err(|_| poisoned_lock())?;

        verify_generation(
            state.generation,
            expected_generation,
        )?;

        if !state.providers.contains_key(&id) {
            return Err(not_found(
                format!("provider {id} is not registered"),
            ));
        }

        let generation = next_generation(state.generation)?;

        state.providers.insert(
            id.clone(),
            RegisteredProvider {
                descriptor,
                provider,
                registered_generation: generation,
            },
        );

        state.generation = generation;

        Ok(RegistrationReceipt {
            provider_id: id,
            generation,
            replaced: true,
        })
    }

    // ------------------------------------------------------------------------
    // REMOVAL
    // ------------------------------------------------------------------------

    /// Removes a provider by identity.
    ///
    /// Existing snapshots and cloned Arc references remain valid.
    ///
    /// Removal does not shut down the provider.
    ///
    /// Lifecycle management remains the responsibility of the
    /// lifecycle authority.
    ///
    /// # Errors
    ///
    /// Returns a not-found error if the identity is absent.
    pub fn unregister(
        &self,
        id: &ProviderId,
    ) -> Result<RegisteredProvider> {
        let mut state = self.state.write()
            .map_err(|_| poisoned_lock())?;

        if !state.providers.contains_key(id) {
            return Err(not_found(
                format!("provider {id} is not registered"),
            ));
        }

        let generation = next_generation(state.generation)?;

        let removed = state.providers.remove(id)
            .ok_or_else(|| {
                registry_error(
                    ErrorKind::Internal,
                    "provider disappeared during exclusive registry mutation",
                )
            })?;

        state.generation = generation;

        Ok(removed)
    }

    /// Removes a provider only if the expected registry generation
    /// matches the current generation.
    pub fn unregister_if_generation(
        &self,
        expected_generation: u64,
        id: &ProviderId,
    ) -> Result<RegisteredProvider> {
        let mut state = self.state.write()
            .map_err(|_| poisoned_lock())?;

        verify_generation(
            state.generation,
            expected_generation,
        )?;

        if !state.providers.contains_key(id) {
            return Err(not_found(
                format!("provider {id} is not registered"),
            ));
        }

        let generation = next_generation(state.generation)?;

        let removed = state.providers.remove(id)
            .ok_or_else(|| {
                registry_error(
                    ErrorKind::Internal,
                    "provider disappeared during exclusive registry mutation",
                )
            })?;

        state.generation = generation;

        Ok(removed)
    }

    // ------------------------------------------------------------------------
    // LOOKUP
    // ------------------------------------------------------------------------

    /// Looks up a provider by exact identity.
    ///
    /// Returns a shared registration snapshot.
    pub fn get(
        &self,
        id: &ProviderId,
    ) -> Result<Option<RegisteredProvider>> {
        let state = self.state.read()
            .map_err(|_| poisoned_lock())?;

        Ok(state.providers.get(id).cloned())
    }

    /// Checks whether a provider identity is registered.
    pub fn contains(
        &self,
        id: &ProviderId,
    ) -> Result<bool> {
        let state = self.state.read()
            .map_err(|_| poisoned_lock())?;

        Ok(state.providers.contains_key(id))
    }

    /// Finds providers by exact kind.
    pub fn by_kind(
        &self,
        kind: &ProviderKind,
    ) -> Result<Vec<RegisteredProvider>> {
        let state = self.state.read()
            .map_err(|_| poisoned_lock())?;

        Ok(state.providers
            .values()
            .filter(|provider| provider.kind() == kind)
            .cloned()
            .collect())
    }

    /// Finds providers declaring a capability.
    ///
    /// This is a metadata query, not a live availability check.
    pub fn by_capability(
        &self,
        capability: &CapabilityId,
    ) -> Result<Vec<RegisteredProvider>> {
        let state = self.state.read()
            .map_err(|_| poisoned_lock())?;

        Ok(state.providers
            .values()
            .filter(|provider| {
                provider.declares_capability(capability)
            })
            .cloned()
            .collect())
    }

    /// Finds providers declaring an operation.
    pub fn by_operation(
        &self,
        operation: &ProviderOperationId,
    ) -> Result<Vec<RegisteredProvider>> {
        let state = self.state.read()
            .map_err(|_| poisoned_lock())?;

        Ok(state.providers
            .values()
            .filter(|provider| {
                provider.declares_operation(operation)
            })
            .cloned()
            .collect())
    }

    /// Finds providers with an exact API-version match.
    pub fn by_api_version(
        &self,
        version: &Version,
    ) -> Result<Vec<RegisteredProvider>> {
        let state = self.state.read()
            .map_err(|_| poisoned_lock())?;

        Ok(state.providers
            .values()
            .filter(|provider| {
                provider.api_version() == version
            })
            .cloned()
            .collect())
    }

    // ------------------------------------------------------------------------
    // SNAPSHOTS
    // ------------------------------------------------------------------------

    /// Creates an immutable snapshot of the current registry.
    ///
    /// All entries correspond to the same registry generation.
    ///
    /// Provider callbacks are not invoked.
    pub fn snapshot(&self) -> Result<RegistrySnapshot> {
        let state = self.state.read()
            .map_err(|_| poisoned_lock())?;

        let providers: Vec<RegisteredProvider> =
            state.providers.values().cloned().collect();

        Ok(RegistrySnapshot {
            generation: state.generation,
            providers: Arc::from(providers),
        })
    }

    /// Returns a cloned registry with independent mutation state.
    ///
    /// Provider instances remain shared through Arc.
    ///
    /// Subsequent registrations or removals in either registry
    /// do not affect the other registry.
    pub fn fork(&self) -> Result<Self> {
        let state = self.state.read()
            .map_err(|_| poisoned_lock())?;

        let forked = RegistryState {
            generation: state.generation,
            providers: state.providers.clone(),
        };

        Ok(Self {
            state: Arc::new(RwLock::new(forked)),
        })
    }

    /// Removes all registrations.
    ///
    /// Existing snapshots and external provider references remain valid.
    ///
    /// This does not shut down providers.
    pub fn clear(&self) -> Result<u64> {
        let mut state = self.state.write()
            .map_err(|_| poisoned_lock())?;

        if state.providers.is_empty() {
            return Ok(state.generation);
        }

        let generation = next_generation(state.generation)?;

        state.providers.clear();
        state.generation = generation;

        Ok(generation)
    }
}

// ============================================================================
// GENERATION VALIDATION
// ============================================================================

fn next_generation(
    current: u64,
) -> Result<u64> {
    current.checked_add(1).ok_or_else(|| {
        registry_error(
            ErrorKind::ResourceUnavailable,
            "registry generation space is exhausted",
        )
    })
}

fn verify_generation(
    actual: u64,
    expected: u64,
) -> Result<()> {
    if actual != expected {
        return Err(concurrency_error(
            format!(
                "registry generation conflict: expected {expected}, found {actual}"
            ),
        ));
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::future::Future;
    use std::pin::Pin;

    use crate::stdlib::capability::{
        Capability,
        CapabilityKind,
        CapabilitySnapshot,
    };

    use crate::stdlib::context::OperationContext;

    use crate::stdlib::error::Result as StdResult;

    use crate::stdlib::provider::{
        ProviderDescriptor,
        ProviderFuture,
        ProviderId,
        ProviderKind,
    };

    fn version() -> Version {
        Version::parse("1.0.0")
            .expect("valid test version")
    }

    fn provider_id(value: &str) -> ProviderId {
        ProviderId::new(value)
            .expect("valid provider identity")
    }

    fn provider_kind(value: &str) -> ProviderKind {
        ProviderKind::new(value)
            .expect("valid provider kind")
    }

    fn descriptor(
        id: &str,
        kind: &str,
    ) -> ProviderDescriptor {
        ProviderDescriptor::new(
            provider_id(id),
            provider_kind(kind),
            format!("Provider {id}"),
            version(),
            version(),
        )
        .expect("valid provider descriptor")
    }

    struct TestProvider {
        descriptor: ProviderDescriptor,
    }

    impl TestProvider {
        fn new(
            id: &str,
            kind: &str,
        ) -> Self {
            Self {
                descriptor: descriptor(id, kind),
            }
        }
    }

    impl Provider for TestProvider {
        fn descriptor(&self) -> &ProviderDescriptor {
            &self.descriptor
        }

        fn status(&self) -> ProviderStatus {
            ProviderStatus::Ready
        }

        fn capability_snapshot<'a>(
            &'a self,
            _context: &'a OperationContext,
        ) -> ProviderFuture<'a, CapabilitySnapshot> {
            Box::pin(async {
                Err(Error::new(
                    ErrorKind::Unsupported,
                    "test provider does not implement capability discovery",
                ))
            })
        }
    }

    fn provider(
        id: &str,
        kind: &str,
    ) -> Arc<dyn Provider> {
        Arc::new(TestProvider::new(id, kind))
    }

    #[test]
    fn creates_empty_registry() {
        let registry = ProviderRegistry::new();

        assert_eq!(registry.len().unwrap(), 0);
        assert!(registry.is_empty().unwrap());
        assert_eq!(registry.generation().unwrap(), 0);
    }

    #[test]
    fn registers_provider() {
        let registry = ProviderRegistry::new();

        let receipt = registry.register(
            provider("org.example.classical", "classical"),
        ).unwrap();

        assert_eq!(
            receipt.provider_id().as_str(),
            "org.example.classical",
        );

        assert_eq!(receipt.generation(), 1);
        assert!(!receipt.replaced());

        assert_eq!(registry.len().unwrap(), 1);
    }

    #[test]
    fn rejects_duplicate_registration() {
        let registry = ProviderRegistry::new();

        registry.register(
            provider("org.example.classical", "classical"),
        ).unwrap();

        let result = registry.register(
            provider("org.example.classical", "classical"),
        );

        assert!(result.is_err());
        assert_eq!(registry.len().unwrap(), 1);
    }

    #[test]
    fn provider_identity_is_unique_per_registry() {
        let first = ProviderRegistry::new();
        let second = ProviderRegistry::new();

        first.register(
            provider("org.example.provider", "classical"),
        ).unwrap();

        second.register(
            provider("org.example.provider", "quantum"),
        ).unwrap();

        assert_eq!(first.len().unwrap(), 1);
        assert_eq!(second.len().unwrap(), 1);
    }

    #[test]
    fn lookup_uses_exact_identity() {
        let registry = ProviderRegistry::new();

        registry.register(
            provider("org.example.classical", "classical"),
        ).unwrap();

        assert!(
            registry.get(
                &provider_id("org.example.classical")
            ).unwrap().is_some()
        );

        assert!(
            registry.get(
                &provider_id("org.example.quantum")
            ).unwrap().is_none()
        );
    }

    #[test]
    fn finds_providers_by_kind() {
        let registry = ProviderRegistry::new();

        registry.register(
            provider("org.example.classical", "classical"),
        ).unwrap();

        registry.register(
            provider("org.example.quantum", "quantum"),
        ).unwrap();

        registry.register(
            provider("org.example.classical.other", "classical"),
        ).unwrap();

        assert_eq!(
            registry.by_kind(
                &provider_kind("classical")
            ).unwrap().len(),
            2
        );

        assert_eq!(
            registry.by_kind(
                &provider_kind("quantum")
            ).unwrap().len(),
            1
        );
    }

    #[test]
    fn registration_advances_generation() {
        let registry = ProviderRegistry::new();

        registry.register(
            provider("org.example.one", "classical"),
        ).unwrap();

        registry.register(
            provider("org.example.two", "quantum"),
        ).unwrap();

        assert_eq!(registry.generation().unwrap(), 2);
    }

    #[test]
    fn rejects_stale_generation_mutation() {
        let registry = ProviderRegistry::new();

        registry.register(
            provider("org.example.one", "classical"),
        ).unwrap();

        let result = registry.register_if_generation(
            0,
            provider("org.example.two", "quantum"),
        );

        assert!(result.is_err());
        assert_eq!(registry.len().unwrap(), 1);
    }

    #[test]
    fn accepts_matching_generation_mutation() {
        let registry = ProviderRegistry::new();

        registry.register(
            provider("org.example.one", "classical"),
        ).unwrap();

        let receipt = registry.register_if_generation(
            1,
            provider("org.example.two", "quantum"),
        ).unwrap();

        assert_eq!(receipt.generation(), 2);
        assert_eq!(registry.len().unwrap(), 2);
    }

    #[test]
    fn replacement_is_explicit() {
        let registry = ProviderRegistry::new();

        registry.register(
            provider("org.example.provider", "classical"),
        ).unwrap();

        let receipt = registry.replace(
            provider("org.example.provider", "quantum"),
        ).unwrap();

        assert!(receipt.replaced());

        assert_eq!(
            registry.get(
                &provider_id("org.example.provider")
            ).unwrap().unwrap().kind().as_str(),
            "quantum",
        );
    }

    #[test]
    fn replacement_requires_existing_identity() {
        let registry = ProviderRegistry::new();

        assert!(
            registry.replace(
                provider("org.example.missing", "classical")
            ).is_err()
        );
    }

    #[test]
    fn unregister_removes_provider() {
        let registry = ProviderRegistry::new();

        registry.register(
            provider("org.example.provider", "classical"),
        ).unwrap();

        let removed = registry.unregister(
            &provider_id("org.example.provider"),
        ).unwrap();

        assert_eq!(
            removed.id().as_str(),
            "org.example.provider",
        );

        assert!(registry.is_empty().unwrap());
        assert_eq!(registry.generation().unwrap(), 2);
    }

    #[test]
    fn unregister_missing_provider_returns_error() {
        let registry = ProviderRegistry::new();

        assert!(
            registry.unregister(
                &provider_id("org.example.missing")
            ).is_err()
        );
    }

    #[test]
    fn snapshots_remain_unchanged_after_mutation() {
        let registry = ProviderRegistry::new();

        registry.register(
            provider("org.example.one", "classical"),
        ).unwrap();

        let snapshot = registry.snapshot().unwrap();

        registry.register(
            provider("org.example.two", "quantum"),
        ).unwrap();

        assert_eq!(snapshot.len(), 1);
        assert_eq!(registry.len().unwrap(), 2);
    }

    #[test]
    fn snapshots_have_deterministic_identity_order() {
        let registry = ProviderRegistry::new();

        registry.register(
            provider("org.example.z", "classical"),
        ).unwrap();

        registry.register(
            provider("org.example.a", "quantum"),
        ).unwrap();

        let snapshot = registry.snapshot().unwrap();

        let ids: Vec<&str> = snapshot
            .provider_ids()
            .iter()
            .map(|id| id.as_str())
            .collect();

        assert_eq!(
            ids,
            vec![
                "org.example.a",
                "org.example.z",
            ]
        );
    }

    #[test]
    fn snapshots_retain_provider_ownership() {
        let registry = ProviderRegistry::new();

        registry.register(
            provider("org.example.provider", "classical"),
        ).unwrap();

        let snapshot = registry.snapshot().unwrap();

        registry.clear().unwrap();

        assert_eq!(snapshot.len(), 1);

        assert!(
            snapshot.get(
                &provider_id("org.example.provider")
            ).is_some()
        );
    }

    #[test]
    fn fork_has_independent_mutation_state() {
        let registry = ProviderRegistry::new();

        registry.register(
            provider("org.example.one", "classical"),
        ).unwrap();

        let fork = registry.fork().unwrap();

        fork.register(
            provider("org.example.two", "quantum"),
        ).unwrap();

        assert_eq!(registry.len().unwrap(), 1);
        assert_eq!(fork.len().unwrap(), 2);
    }

    #[test]
    fn clear_advances_generation_only_when_needed() {
        let registry = ProviderRegistry::new();

        registry.register(
            provider("org.example.one", "classical"),
        ).unwrap();

        assert_eq!(registry.clear().unwrap(), 2);
        assert_eq!(registry.clear().unwrap(), 2);
    }

    #[test]
    fn capability_and_operation_queries_are_open_world() {
        let registry = ProviderRegistry::new();

        let mut provider = TestProvider::new(
            "org.example.future",
            "future.neuromorphic",
        );

        provider.descriptor.insert_capability(
            Capability::new(
                CapabilityId::new("future.neuromorphic.compute")
                    .unwrap(),
                CapabilityKind::new("computation")
                    .unwrap(),
            ),
        ).unwrap();

        let registered: Arc<dyn Provider> = Arc::new(provider);

        registry.register(registered).unwrap();

        let capability = CapabilityId::new(
            "future.neuromorphic.compute",
        ).unwrap();

        assert_eq!(
            registry.by_capability(&capability).unwrap().len(),
            1,
        );

        assert_eq!(
            registry.snapshot().unwrap().len(),
            1,
        );
    }

    #[test]
    fn status_observation_does_not_change_registry_generation() {
        let registry = ProviderRegistry::new();

        registry.register(
            provider("org.example.provider", "classical"),
        ).unwrap();

        let generation = registry.generation().unwrap();

        let snapshot = registry.snapshot().unwrap();

        assert_eq!(snapshot.potentially_ready().len(), 1);

        assert_eq!(
            registry.generation().unwrap(),
            generation,
        );
    }
}
