
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani ZHAL — Hardware Provider Registry
//!
//! Path:
//!     src/zhal/providers/registry.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version 1.97
//!
//! Safety:
//!     Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module provides the canonical in-process registry for executable
//! universal hardware providers.
//!
//! It connects the universal ZHAL hardware boundary to independently
//! implemented provider implementations.
//!
//! Supported provider domains are open-ended and include:
//!
//! - classical computing;
//! - quantum computing;
//! - HDL and programmable hardware;
//! - accelerators;
//! - hybrid computing;
//! - distributed hardware;
//! - embedded systems;
//! - future computational technologies.
//!
//! The registry does not maintain a closed list of supported domains.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! THIS MODULE OWNS:
//!
//! - Provider implementation registration.
//! - Provider implementation lookup.
//! - Provider implementation handles.
//! - Duplicate-registration protection.
//! - Explicit provider replacement.
//! - Explicit provider removal.
//! - Deterministic enumeration.
//! - Provider metadata queries.
//! - Registry generation.
//! - Optimistic concurrency.
//! - Immutable registry snapshots.
//! - Registry integrity inspection.
//! - Thread-safe registry storage.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - Provider identity definitions.
//! - Provider descriptors.
//! - Provider lifecycle definitions.
//! - Provider API compatibility policy.
//! - Provider discovery.
//! - Dynamic library loading.
//! - Provider authentication.
//! - Provider trust verification.
//! - Provider authorization.
//! - Hardware device discovery.
//! - Device registration.
//! - Device allocation.
//! - Resource reservation.
//! - Hardware scheduling.
//! - Hardware routing.
//! - Quantum execution.
//! - HDL realization.
//! - Distributed execution.
//! - Compiler target selection.
//! - Semantic requirement evaluation.
//!
//! ============================================================================
//! CANONICAL AUTHORITIES
//! ============================================================================
//!
//! Provider identity and metadata:
//!     crate::stdlib::provider
//!
//! Provider implementation contract:
//!     crate::stdlib::provider::Provider
//!
//! Provider lifecycle:
//!     crate::stdlib::provider::ProviderLifecycle
//!
//! Capability identity:
//!     crate::stdlib::capability
//!
//! Structured errors:
//!     crate::stdlib::error
//!
//! Version representation:
//!     crate::stdlib::version
//!
//! Provider validation and API compatibility:
//!     src/zhal/providers/validation.rs
//!
//! Provider lifecycle coordination:
//!     src/zhal/providers/lifecycle.rs
//!
//! Provider discovery:
//!     src/zhal/providers/discovery.rs
//!
//! Hardware device inventory:
//!     src/zhal/devices/inventory.rs
//!
//! Physical resource allocation:
//!     Canonical runtime resource-management authority.
//!
//! ============================================================================
//! IMPORTANT PROVIDER DISTINCTION
//! ============================================================================
//!
//! A ProviderDescriptor describes an implementation.
//!
//! An Arc<dyn Provider> represents the executable implementation.
//!
//! Both are required.
//!
//! The registry stores the implementation together with an immutable
//! descriptor snapshot.
//!
//! It never assumes that registration means initialization, authorization,
//! trust verification, hardware availability or execution readiness.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! There are no arbitrary universal maximums for:
//!
//! - registered providers;
//! - provider identities;
//! - provider operations;
//! - hardware categories;
//! - computing domains;
//! - devices;
//! - hardware resources;
//! - distributed participants.
//!
//! Registry growth is subject to available memory, address-space limits,
//! platform constraints and explicitly configured operational budgets.
//!
//! The registry does not promise literal infinite physical storage.
//!
//! ============================================================================
//! CONCURRENCY
//! ============================================================================
//!
//! A single RwLock protects the complete mutable registry state.
//!
//! Provider storage and generation therefore change atomically.
//!
//! Provider callbacks are never invoked while holding the registry lock.
//!
//! Existing handles remain valid after provider removal because they retain
//! an Arc to the original provider implementation.
//!
//! ============================================================================
//! VERSIONING
//! ============================================================================
//!
//! Every successful structural mutation increments the registry generation.
//!
//! Generation numbers are local to one registry instance.
//!
//! They are not timestamps or distributed consensus versions.
//!
//! ============================================================================
//! SECURITY
//! ============================================================================
//!
//! Provider registration does not establish trust.
//!
//! Provider validation, admission, authentication, authorization and
//! isolation remain separate responsibilities.
//!
//! Credentials must never be stored in provider descriptors or registry
//! metadata.
//!
//! The registry does not load or execute dynamically supplied machine code.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! Provider identity ordering is deterministic through BTreeMap.
//!
//! Query results follow canonical provider identity ordering.
//!
//! Snapshots preserve a consistent registry generation.
//!
//! Provider registration order does not define program semantics.

use std::collections::BTreeMap;
use std::fmt;
use std::num::NonZeroUsize;
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::stdlib::capability::CapabilityId;

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
};

// ============================================================================
// SCHEMA
// ============================================================================

/// Stable identifier of the ZHAL provider registry schema.
pub const PROVIDER_REGISTRY_SCHEMA_ID: &str =
    "zamani.zhal.providers.registry";

/// Major version of the provider registry schema.
pub const PROVIDER_REGISTRY_SCHEMA_MAJOR: u16 = 1;

/// Minor version of the provider registry schema.
pub const PROVIDER_REGISTRY_SCHEMA_MINOR: u16 = 0;

/// Patch version of the provider registry schema.
pub const PROVIDER_REGISTRY_SCHEMA_PATCH: u16 = 0;

/// Complete registry schema version.
pub const PROVIDER_REGISTRY_SCHEMA_VERSION: (u16, u16, u16) = (
    PROVIDER_REGISTRY_SCHEMA_MAJOR,
    PROVIDER_REGISTRY_SCHEMA_MINOR,
    PROVIDER_REGISTRY_SCHEMA_PATCH,
);

const REGISTRY_COMPONENT: &str = "zhal.providers.registry";

// ============================================================================
// ERRORS
// ============================================================================

/// Creates a structured registry error.
fn registry_error(
    kind: ErrorKind,
    message: impl Into<String>,
) -> Error {
    Error::new(kind, message)
        .in_component(REGISTRY_COMPONENT)
}

/// Creates an invalid-input error.
fn invalid_input(
    message: impl Into<String>,
) -> Error {
    registry_error(ErrorKind::InvalidInput, message)
}

/// Creates a registry-invariant error.
fn invariant_error(
    message: impl Into<String>,
) -> Error {
    registry_error(ErrorKind::Internal, message)
}

/// Creates a lock-poisoning error.
///
/// A poisoned lock is not silently recovered because the registry cannot
/// establish that a previous interrupted mutation preserved its invariants.
fn lock_poisoned() -> Error {
    invariant_error(
        "provider registry lock is poisoned",
    )
}

// ============================================================================
// REGISTRY GENERATION
// ============================================================================

/// Monotonic generation of one provider registry instance.
///
/// Generation zero represents a registry that has not been structurally
/// modified.
///
/// A generation is not a timestamp, globally unique identifier or
/// distributed consensus version.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct RegistryGeneration(u64);

impl RegistryGeneration {
    /// Initial generation.
    pub const INITIAL: Self = Self(0);

    /// Returns the underlying generation number.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Computes the next generation without wrapping.
    ///
    /// # Errors
    ///
    /// Returns an internal error if the generation space is exhausted.
    fn checked_next(self) -> Result<Self> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or_else(|| {
                invariant_error(
                    "provider registry generation exhausted",
                )
            })
    }
}

impl Default for RegistryGeneration {
    fn default() -> Self {
        Self::INITIAL
    }
}

impl fmt::Display for RegistryGeneration {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// ============================================================================
// REGISTERED PROVIDER
// ============================================================================

/// Internal immutable registration record.
struct RegisteredProvider {
    /// Executable provider implementation.
    implementation: Arc<dyn Provider>,

    /// Immutable descriptor captured at registration.
    descriptor: Arc<ProviderDescriptor>,

    /// Generation at which this implementation was registered.
    registration_generation: RegistryGeneration,
}

impl RegisteredProvider {
    /// Creates a public provider handle.
    fn handle(&self) -> ProviderHandle {
        ProviderHandle {
            implementation: Arc::clone(
                &self.implementation,
            ),
            descriptor: Arc::clone(
                &self.descriptor,
            ),
            registration_generation:
                self.registration_generation,
        }
    }
}

// ============================================================================
// REGISTRY STATE
// ============================================================================

/// Complete mutable registry state.
///
/// Both provider storage and generation are protected by the same lock.
///
/// This prevents the inconsistent-generation problem caused by maintaining
/// separate locks for these related fields.
struct RegistryState {
    generation: RegistryGeneration,

    providers: BTreeMap<
        ProviderId,
        RegisteredProvider,
    >,
}

impl RegistryState {
    fn new() -> Self {
        Self {
            generation: RegistryGeneration::INITIAL,
            providers: BTreeMap::new(),
        }
    }
}

// ============================================================================
// PROVIDER HANDLE
// ============================================================================

/// Reference-counted handle to a registered provider implementation.
///
/// A handle keeps its provider alive even after the provider is removed
/// from the registry.
///
/// Removal prevents future registry lookups but does not forcibly terminate
/// existing users of the provider.
///
/// Provider shutdown belongs to the lifecycle authority.
#[derive(Clone)]
pub struct ProviderHandle {
    implementation: Arc<dyn Provider>,

    descriptor: Arc<ProviderDescriptor>,

    registration_generation: RegistryGeneration,
}

impl ProviderHandle {
    /// Returns the immutable provider descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ProviderDescriptor {
        &self.descriptor
    }

    /// Returns the canonical provider identity.
    #[must_use]
    pub fn id(&self) -> &ProviderId {
        self.descriptor.id()
    }

    /// Returns the executable provider implementation.
    ///
    /// The caller may use the canonical Provider interface or retain
    /// specialized domain-specific interfaces through the original
    /// concrete provider before registration.
    #[must_use]
    pub fn provider(&self) -> &dyn Provider {
        self.implementation.as_ref()
    }

    /// Returns the provider implementation Arc.
    #[must_use]
    pub fn implementation(&self) -> Arc<dyn Provider> {
        Arc::clone(&self.implementation)
    }

    /// Returns the generation at which this implementation was registered.
    #[must_use]
    pub const fn registration_generation(
        &self,
    ) -> RegistryGeneration {
        self.registration_generation
    }

    /// Obtains the provider's current status.
    ///
    /// This calls the provider outside the registry lock.
    #[must_use]
    pub fn status(
        &self,
    ) -> crate::stdlib::provider::ProviderStatus {
        self.implementation.status()
    }

    /// Returns whether the implementation and captured descriptor still
    /// agree on provider identity.
    ///
    /// This is a local integrity check, not provider trust verification.
    #[must_use]
    pub fn identity_is_consistent(&self) -> bool {
        self.implementation.descriptor().id()
            == self.descriptor.id()
    }
}

impl fmt::Debug for ProviderHandle {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ProviderHandle")
            .field("provider_id", self.id())
            .field(
                "registration_generation",
                &self.registration_generation,
            )
            .finish_non_exhaustive()
    }
}

// ============================================================================
// PROVIDER QUERY
// ============================================================================

/// Metadata-only provider query.
///
/// All populated requirements are combined using logical AND.
///
/// Empty capability and operation lists impose no restrictions.
///
/// This query does not perform hardware discovery, provider callbacks,
/// capability observation, resource negotiation or target selection.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProviderQuery {
    kind: Option<ProviderKind>,

    required_capabilities: Vec<CapabilityId>,

    required_operations: Vec<ProviderOperationId>,
}

impl ProviderQuery {
    /// Creates an unconstrained query.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Restricts results to one provider kind.
    #[must_use]
    pub fn with_kind(
        mut self,
        kind: ProviderKind,
    ) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Requires a declared capability.
    ///
    /// A declaration does not prove current availability.
    #[must_use]
    pub fn requiring_capability(
        mut self,
        capability: CapabilityId,
    ) -> Self {
        self.required_capabilities.push(capability);
        self
    }

    /// Requires a declared provider operation.
    #[must_use]
    pub fn requiring_operation(
        mut self,
        operation: ProviderOperationId,
    ) -> Self {
        self.required_operations.push(operation);
        self
    }

    /// Returns whether a descriptor satisfies this metadata query.
    #[must_use]
    pub fn matches(
        &self,
        descriptor: &ProviderDescriptor,
    ) -> bool {
        if let Some(kind) = &self.kind {
            if descriptor.kind() != kind {
                return false;
            }
        }

        for capability in &self.required_capabilities {
            if descriptor.capability(capability).is_none() {
                return false;
            }
        }

        for operation in &self.required_operations {
            if descriptor.operation(operation).is_none() {
                return false;
            }
        }

        true
    }
}

// ============================================================================
// SNAPSHOT
// ============================================================================

/// Immutable consistent snapshot of registered provider metadata.
///
/// Snapshots do not retain the registry lock.
///
/// Provider implementations are deliberately excluded from snapshots.
/// A snapshot is descriptive registry state, not an execution handle.
#[derive(Debug, Clone)]
pub struct ProviderRegistrySnapshot {
    schema_id: &'static str,

    schema_version: (u16, u16, u16),

    generation: RegistryGeneration,

    providers: BTreeMap<
        ProviderId,
        Arc<ProviderDescriptor>,
    >,
}

impl ProviderRegistrySnapshot {
    /// Returns the registry schema identifier.
    #[must_use]
    pub const fn schema_id(&self) -> &'static str {
        self.schema_id
    }

    /// Returns the registry schema version.
    #[must_use]
    pub const fn schema_version(
        &self,
    ) -> (u16, u16, u16) {
        self.schema_version
    }

    /// Returns the captured generation.
    #[must_use]
    pub const fn generation(
        &self,
    ) -> RegistryGeneration {
        self.generation
    }

    /// Returns the number of providers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Returns whether the snapshot is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    /// Looks up a descriptor by provider identity.
    #[must_use]
    pub fn get(
        &self,
        id: &ProviderId,
    ) -> Option<&ProviderDescriptor> {
        self.providers
            .get(id)
            .map(Arc::as_ref)
    }

    /// Returns provider IDs in deterministic order.
    #[must_use]
    pub fn provider_ids(&self) -> Vec<ProviderId> {
        self.providers.keys().cloned().collect()
    }

    /// Returns descriptors in deterministic order.
    #[must_use]
    pub fn descriptors(&self) -> Vec<Arc<ProviderDescriptor>> {
        self.providers.values().cloned().collect()
    }

    /// Returns descriptors matching a metadata query.
    #[must_use]
    pub fn query(
        &self,
        query: &ProviderQuery,
    ) -> Vec<Arc<ProviderDescriptor>> {
        self.providers
            .values()
            .filter(|descriptor| {
                query.matches(descriptor.as_ref())
            })
            .cloned()
            .collect()
    }
}

// ============================================================================
// REGISTRY
// ============================================================================

/// Thread-safe universal hardware-provider registry.
///
/// The registry is explicitly constructed and owned by its application.
///
/// It does not create hidden global state.
///
/// Multiple registry instances may coexist, including isolated registries
/// for different runtime contexts, tests, tenants or deployment environments.
#[derive(Clone)]
pub struct ProviderRegistry {
    state: Arc<RwLock<RegistryState>>,
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ProviderRegistry {
    /// Creates an empty provider registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(
                RwLock::new(RegistryState::new()),
            ),
        }
    }

    // ------------------------------------------------------------------------
    // Lock access
    // ------------------------------------------------------------------------

    fn read_state(
        &self,
    ) -> Result<RwLockReadGuard<'_, RegistryState>> {
        self.state
            .read()
            .map_err(|_| lock_poisoned())
    }

    fn write_state(
        &self,
    ) -> Result<RwLockWriteGuard<'_, RegistryState>> {
        self.state
            .write()
            .map_err(|_| lock_poisoned())
    }

    // ------------------------------------------------------------------------
    // Generation
    // ------------------------------------------------------------------------

    /// Returns the current registry generation.
    pub fn generation(
        &self,
    ) -> Result<RegistryGeneration> {
        Ok(self.read_state()?.generation)
    }

    // ------------------------------------------------------------------------
    // Registration
    // ------------------------------------------------------------------------

    /// Registers an executable hardware provider.
    ///
    /// The provider descriptor is captured before acquiring the registry
    /// lock. Provider callbacks are therefore never invoked while holding
    /// the registry lock.
    ///
    /// Duplicate provider identities are rejected.
    ///
    /// Registration does not initialize, authenticate, authorize or activate
    /// the provider.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - the provider identity already exists;
    /// - the registry lock is poisoned;
    /// - the registry generation is exhausted.
    pub fn register(
        &self,
        provider: Arc<dyn Provider>,
    ) -> Result<RegistryGeneration> {
        let descriptor = Arc::new(
            provider.descriptor().clone(),
        );

        let provider_id = descriptor.id().clone();

        let mut state = self.write_state()?;

        if state.providers.contains_key(&provider_id) {
            return Err(registry_error(
                ErrorKind::AlreadyExists,
                format!(
                    "provider {} is already registered",
                    provider_id,
                ),
            ));
        }

        let next = state.generation.checked_next()?;

        state.providers.insert(
            provider_id,
            RegisteredProvider {
                implementation: provider,
                descriptor,
                registration_generation: next,
            },
        );

        state.generation = next;

        Ok(next)
    }

    /// Registers a provider only if the registry generation matches
    /// the caller's expected generation.
    ///
    /// The comparison and insertion occur under the same write lock.
    pub fn register_if_generation(
        &self,
        expected: RegistryGeneration,
        provider: Arc<dyn Provider>,
    ) -> Result<RegistryGeneration> {
        let descriptor = Arc::new(
            provider.descriptor().clone(),
        );

        let provider_id = descriptor.id().clone();

        let mut state = self.write_state()?;

        if state.generation != expected {
            return Err(registry_error(
                ErrorKind::Concurrency,
                format!(
                    "registry generation mismatch: expected {}, actual {}",
                    expected,
                    state.generation,
                ),
            ));
        }

        if state.providers.contains_key(&provider_id) {
            return Err(registry_error(
                ErrorKind::AlreadyExists,
                format!(
                    "provider {} is already registered",
                    provider_id,
                ),
            ));
        }

        let next = state.generation.checked_next()?;

        state.providers.insert(
            provider_id,
            RegisteredProvider {
                implementation: provider,
                descriptor,
                registration_generation: next,
            },
        );

        state.generation = next;

        Ok(next)
    }

    // ------------------------------------------------------------------------
    // Replacement
    // ------------------------------------------------------------------------

    /// Explicitly replaces an existing provider implementation.
    ///
    /// The replacement must use the same canonical provider identity.
    ///
    /// Existing handles continue to reference the previous implementation.
    ///
    /// No provider lifecycle operation is invoked automatically.
    pub fn replace(
        &self,
        provider: Arc<dyn Provider>,
    ) -> Result<RegistryGeneration> {
        let descriptor = Arc::new(
            provider.descriptor().clone(),
        );

        let provider_id = descriptor.id().clone();

        let mut state = self.write_state()?;

        if !state.providers.contains_key(&provider_id) {
            return Err(registry_error(
                ErrorKind::NotFound,
                format!(
                    "cannot replace unregistered provider {}",
                    provider_id,
                ),
            ));
        }

        let next = state.generation.checked_next()?;

        state.providers.insert(
            provider_id,
            RegisteredProvider {
                implementation: provider,
                descriptor,
                registration_generation: next,
            },
        );

        state.generation = next;

        Ok(next)
    }

    /// Replaces a provider only if the expected generation matches.
    pub fn replace_if_generation(
        &self,
        expected: RegistryGeneration,
        provider: Arc<dyn Provider>,
    ) -> Result<RegistryGeneration> {
        let descriptor = Arc::new(
            provider.descriptor().clone(),
        );

        let provider_id = descriptor.id().clone();

        let mut state = self.write_state()?;

        if state.generation != expected {
            return Err(registry_error(
                ErrorKind::Concurrency,
                format!(
                    "registry generation mismatch: expected {}, actual {}",
                    expected,
                    state.generation,
                ),
            ));
        }

        if !state.providers.contains_key(&provider_id) {
            return Err(registry_error(
                ErrorKind::NotFound,
                format!(
                    "provider {} is not registered",
                    provider_id,
                ),
            ));
        }

        let next = state.generation.checked_next()?;

        state.providers.insert(
            provider_id,
            RegisteredProvider {
                implementation: provider,
                descriptor,
                registration_generation: next,
            },
        );

        state.generation = next;

        Ok(next)
    }

    // ------------------------------------------------------------------------
    // Removal
    // ------------------------------------------------------------------------

    /// Removes a provider from future registry lookups.
    ///
    /// Existing handles remain valid until their references are dropped.
    ///
    /// Returns the removed provider handle.
    pub fn remove(
        &self,
        id: &ProviderId,
    ) -> Result<ProviderHandle> {
        let mut state = self.write_state()?;

        if !state.providers.contains_key(id) {
            return Err(registry_error(
                ErrorKind::NotFound,
                format!(
                    "provider {} is not registered",
                    id,
                ),
            ));
        }

        let next = state.generation.checked_next()?;

        let removed = state.providers.remove(id).ok_or_else(|| {
            invariant_error(
                "provider disappeared during an exclusive registry mutation",
            )
        })?;

        state.generation = next;

        Ok(removed.handle())
    }

    /// Removes a provider only when the expected generation matches.
    pub fn remove_if_generation(
        &self,
        expected: RegistryGeneration,
        id: &ProviderId,
    ) -> Result<(
        ProviderHandle,
        RegistryGeneration,
    )> {
        let mut state = self.write_state()?;

        if state.generation != expected {
            return Err(registry_error(
                ErrorKind::Concurrency,
                format!(
                    "registry generation mismatch: expected {}, actual {}",
                    expected,
                    state.generation,
                ),
            ));
        }

        if !state.providers.contains_key(id) {
            return Err(registry_error(
                ErrorKind::NotFound,
                format!(
                    "provider {} is not registered",
                    id,
                ),
            ));
        }

        let next = state.generation.checked_next()?;

        let removed = state.providers.remove(id).ok_or_else(|| {
            invariant_error(
                "provider disappeared during an exclusive registry mutation",
            )
        })?;

        state.generation = next;

        Ok((removed.handle(), next))
    }

    /// Removes every provider.
    ///
    /// An empty registry remains at its existing generation.
    ///
    /// Returns the number of removed providers.
    pub fn clear(&self) -> Result<usize> {
        let mut state = self.write_state()?;

        if state.providers.is_empty() {
            return Ok(0);
        }

        let next = state.generation.checked_next()?;

        let removed = std::mem::take(
            &mut state.providers,
        );

        state.generation = next;

        let count = removed.len();

        // Drop the removed registrations after releasing the lock.
        drop(state);
        drop(removed);

        Ok(count)
    }

    // ------------------------------------------------------------------------
    // Lookup
    // ------------------------------------------------------------------------

    /// Looks up a provider by canonical identity.
    ///
    /// Returns None when it is not registered.
    pub fn get(
        &self,
        id: &ProviderId,
    ) -> Result<Option<ProviderHandle>> {
        let state = self.read_state()?;

        Ok(state.providers.get(id).map(
            RegisteredProvider::handle,
        ))
    }

    /// Looks up a provider and returns NotFound when absent.
    pub fn get_required(
        &self,
        id: &ProviderId,
    ) -> Result<ProviderHandle> {
        self.get(id)?.ok_or_else(|| {
            registry_error(
                ErrorKind::NotFound,
                format!(
                    "provider {} is not registered",
                    id,
                ),
            )
        })
    }

    /// Checks whether a provider is registered.
    pub fn contains(
        &self,
        id: &ProviderId,
    ) -> Result<bool> {
        Ok(self.read_state()?.providers.contains_key(id))
    }

    /// Returns the number of registered providers.
    pub fn len(&self) -> Result<usize> {
        Ok(self.read_state()?.providers.len())
    }

    /// Returns whether the registry is empty.
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.read_state()?.providers.is_empty())
    }

    // ------------------------------------------------------------------------
    // Enumeration
    // ------------------------------------------------------------------------

    /// Returns all provider IDs in deterministic canonical order.
    pub fn provider_ids(&self) -> Result<Vec<ProviderId>> {
        Ok(self
            .read_state()?
            .providers
            .keys()
            .cloned()
            .collect())
    }

    /// Returns all provider handles in deterministic canonical order.
    pub fn providers(&self) -> Result<Vec<ProviderHandle>> {
        Ok(self
            .read_state()?
            .providers
            .values()
            .map(RegisteredProvider::handle)
            .collect())
    }

    // ------------------------------------------------------------------------
    // Queries
    // ------------------------------------------------------------------------

    /// Returns every provider matching a metadata query.
    ///
    /// There is no hidden result ceiling.
    ///
    /// Callers needing bounded retrieval should use query_page.
    pub fn query(
        &self,
        query: &ProviderQuery,
    ) -> Result<Vec<ProviderHandle>> {
        let state = self.read_state()?;

        Ok(state
            .providers
            .values()
            .filter(|entry| {
                query.matches(entry.descriptor.as_ref())
            })
            .map(RegisteredProvider::handle)
            .collect())
    }

    /// Returns a page of matching providers.
    ///
    /// Results are ordered by canonical provider identity.
    ///
    /// `after` is an exclusive continuation key.
    ///
    /// A caller may request successive pages without allocating a vector
    /// containing the complete registry.
    pub fn query_page(
        &self,
        query: &ProviderQuery,
        after: Option<&ProviderId>,
        limit: NonZeroUsize,
    ) -> Result<Vec<ProviderHandle>> {
        let state = self.read_state()?;

        let entries = match after {
            Some(id) => {
                state.providers.range((
                    Excluded(id.clone()),
                    Unbounded,
                ))
            }
            None => state.providers.range::<ProviderId, _>((
                Unbounded,
                Unbounded,
            )),
        };

        Ok(entries
            .filter(|(_, entry)| {
                query.matches(entry.descriptor.as_ref())
            })
            .take(limit.get())
            .map(|(_, entry)| entry.handle())
            .collect())
    }

    /// Returns the first provider matching a metadata query.
    pub fn first_match(
        &self,
        query: &ProviderQuery,
    ) -> Result<Option<ProviderHandle>> {
        let state = self.read_state()?;

        Ok(state
            .providers
            .values()
            .find(|entry| {
                query.matches(entry.descriptor.as_ref())
            })
            .map(RegisteredProvider::handle))
    }

    // ------------------------------------------------------------------------
    // Snapshots
    // ------------------------------------------------------------------------

    /// Creates an immutable, internally consistent registry snapshot.
    ///
    /// The provider map and generation are read under the same lock.
    pub fn snapshot(&self) -> Result<ProviderRegistrySnapshot> {
        let state = self.read_state()?;

        let providers = state
            .providers
            .iter()
            .map(|(id, entry)| {
                (
                    id.clone(),
                    Arc::clone(&entry.descriptor),
                )
            })
            .collect();

        Ok(ProviderRegistrySnapshot {
            schema_id: PROVIDER_REGISTRY_SCHEMA_ID,
            schema_version: PROVIDER_REGISTRY_SCHEMA_VERSION,
            generation: state.generation,
            providers,
        })
    }

    // ------------------------------------------------------------------------
    // Integrity
    // ------------------------------------------------------------------------

    /// Verifies registry identity and descriptor consistency.
    ///
    /// Provider methods are called only after the registry lock has been
    /// released.
    ///
    /// This verifies structural consistency. It does not establish provider
    /// trust, security, physical hardware health or API compatibility.
    pub fn verify_integrity(&self) -> Result<()> {
        let entries: Vec<_> = {
            let state = self.read_state()?;

            state
                .providers
                .iter()
                .map(|(id, entry)| {
                    (
                        id.clone(),
                        Arc::clone(&entry.implementation),
                        Arc::clone(&entry.descriptor),
                    )
                })
                .collect()
        };

        for (registered_id, implementation, descriptor) in entries {
            if registered_id != *descriptor.id() {
                return Err(invariant_error(
                    format!(
                        "registry key {} differs from descriptor identity {}",
                        registered_id,
                        descriptor.id(),
                    ),
                ));
            }

            if implementation.descriptor() != descriptor.as_ref() {
                return Err(invariant_error(
                    format!(
                        "provider {} changed its immutable descriptor",
                        registered_id,
                    ),
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::context::OperationContext;

    use crate::stdlib::provider::{
        ProviderFuture,
        ProviderStatus,
    };

    use crate::stdlib::capability::CapabilitySnapshot;

    use crate::stdlib::version::Version;

    struct TestProvider {
        descriptor: ProviderDescriptor,
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
                    "capability observation is not implemented by this test provider",
                ))
            })
        }
    }

    fn test_provider(id: &str) -> Arc<dyn Provider> {
        let descriptor = ProviderDescriptor::new(
            ProviderId::new(id).expect("valid provider ID"),
            ProviderKind::new("hardware.test").expect("valid kind"),
            format!("Test provider {id}"),
            Version::parse("1.0.0").expect("valid version"),
            Version::parse("1.0.0").expect("valid version"),
        )
        .expect("valid descriptor");

        Arc::new(TestProvider { descriptor })
    }

    fn provider_id(id: &str) -> ProviderId {
        ProviderId::new(id).expect("valid provider ID")
    }

    #[test]
    fn registry_starts_empty() {
        let registry = ProviderRegistry::new();

        assert_eq!(registry.len().expect("length"), 0);
        assert!(registry.is_empty().expect("empty"));

        assert_eq!(
            registry.generation().expect("generation"),
            RegistryGeneration::INITIAL,
        );
    }

    #[test]
    fn registration_stores_executable_provider() {
        let registry = ProviderRegistry::new();

        let generation = registry
            .register(test_provider("org.test.cpu"))
            .expect("registration");

        assert_eq!(generation.get(), 1);

        let handle = registry
            .get_required(&provider_id("org.test.cpu"))
            .expect("provider");

        assert_eq!(handle.id().as_str(), "org.test.cpu");
        assert_eq!(handle.status(), ProviderStatus::Ready);
        assert!(handle.identity_is_consistent());
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let registry = ProviderRegistry::new();

        registry
            .register(test_provider("org.test.cpu"))
            .expect("first registration");

        let result = registry.register(
            test_provider("org.test.cpu"),
        );

        assert!(matches!(
            result,
            Err(error)
                if error.to_string().contains("already registered")
        ));

        assert_eq!(registry.len().expect("length"), 1);
        assert_eq!(
            registry.generation().expect("generation").get(),
            1,
        );
    }

    #[test]
    fn replacement_is_explicit() {
        let registry = ProviderRegistry::new();

        registry
            .register(test_provider("org.test.cpu"))
            .expect("registration");

        let generation = registry
            .replace(test_provider("org.test.cpu"))
            .expect("replacement");

        assert_eq!(generation.get(), 2);
    }

    #[test]
    fn replacement_requires_existing_identity() {
        let registry = ProviderRegistry::new();

        assert!(
            registry
                .replace(test_provider("org.test.missing"))
                .is_err()
        );

        assert_eq!(registry.len().expect("length"), 0);
    }

    #[test]
    fn removal_preserves_existing_handles() {
        let registry = ProviderRegistry::new();

        registry
            .register(test_provider("org.test.cpu"))
            .expect("registration");

        let id = provider_id("org.test.cpu");

        let existing = registry
            .get_required(&id)
            .expect("existing handle");

        let removed = registry.remove(&id).expect("removal");

        assert!(!registry.contains(&id).expect("contains"));

        // Existing handles retain the original implementation.
        assert_eq!(
            existing.id().as_str(),
            "org.test.cpu",
        );

        assert_eq!(
            removed.id().as_str(),
            "org.test.cpu",
        );
    }

    #[test]
    fn missing_provider_returns_none() {
        let registry = ProviderRegistry::new();

        assert!(
            registry
                .get(&provider_id("org.test.missing"))
                .expect("lookup")
                .is_none()
        );
    }

    #[test]
    fn missing_required_provider_returns_error() {
        let registry = ProviderRegistry::new();

        assert!(
            registry
                .get_required(&provider_id("org.test.missing"))
                .is_err()
        );
    }

    #[test]
    fn provider_ids_are_deterministic() {
        let registry = ProviderRegistry::new();

        registry
            .register(test_provider("org.test.zeta"))
            .expect("zeta");

        registry
            .register(test_provider("org.test.alpha"))
            .expect("alpha");

        let ids = registry.provider_ids().expect("IDs");

        assert_eq!(
            ids.iter()
                .map(ProviderId::as_str)
                .collect::<Vec<_>>(),
            vec![
                "org.test.alpha",
                "org.test.zeta",
            ],
        );
    }

    #[test]
    fn optimistic_generation_prevents_stale_writes() {
        let registry = ProviderRegistry::new();

        let expected = registry
            .generation()
            .expect("generation");

        registry
            .register(test_provider("org.test.cpu"))
            .expect("registration");

        let result = registry.register_if_generation(
            expected,
            test_provider("org.test.gpu"),
        );

        assert!(result.is_err());

        assert_eq!(registry.len().expect("length"), 1);
    }

    #[test]
    fn snapshot_is_consistent_and_independent() {
        let registry = ProviderRegistry::new();

        registry
            .register(test_provider("org.test.cpu"))
            .expect("registration");

        let snapshot = registry.snapshot().expect("snapshot");

        assert_eq!(snapshot.len(), 1);
        assert_eq!(snapshot.generation().get(), 1);

        registry.clear().expect("clear");

        assert_eq!(snapshot.len(), 1);
        assert!(registry.is_empty().expect("empty"));
    }

    #[test]
    fn clear_advances_generation_once() {
        let registry = ProviderRegistry::new();

        registry
            .register(test_provider("org.test.cpu"))
            .expect("cpu");

        registry
            .register(test_provider("org.test.gpu"))
            .expect("gpu");

        assert_eq!(
            registry.clear().expect("clear"),
            2,
        );

        assert_eq!(
            registry.generation().expect("generation").get(),
            3,
        );

        assert_eq!(registry.clear().expect("empty clear"), 0);

        assert_eq!(
            registry.generation().expect("generation").get(),
            3,
        );
    }

    #[test]
    fn query_without_constraints_returns_all_providers() {
        let registry = ProviderRegistry::new();

        registry
            .register(test_provider("org.test.cpu"))
            .expect("cpu");

        registry
            .register(test_provider("org.test.gpu"))
            .expect("gpu");

        let results = registry
            .query(&ProviderQuery::new())
            .expect("query");

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn query_page_returns_deterministic_continuation() {
        let registry = ProviderRegistry::new();

        for id in ["org.test.a", "org.test.b", "org.test.c"] {
            registry
                .register(test_provider(id))
                .expect("registration");
        }

        let first = registry
            .query_page(
                &ProviderQuery::new(),
                None,
                NonZeroUsize::new(2).expect("nonzero"),
            )
            .expect("first page");

        assert_eq!(first.len(), 2);

        let continuation = first.last().expect("last").id().clone();

        let second = registry
            .query_page(
                &ProviderQuery::new(),
                Some(&continuation),
                NonZeroUsize::new(2).expect("nonzero"),
            )
            .expect("second page");

        assert_eq!(second.len(), 1);
        assert_eq!(
            second[0].id().as_str(),
            "org.test.c",
        );
    }

    #[test]
    fn integrity_check_accepts_valid_provider() {
        let registry = ProviderRegistry::new();

        registry
            .register(test_provider("org.test.cpu"))
            .expect("registration");

        assert!(registry.verify_integrity().is_ok());
    }

    #[test]
    fn cloned_registries_share_the_same_state() {
        let registry = ProviderRegistry::new();

        let clone = registry.clone();

        registry
            .register(test_provider("org.test.cpu"))
            .expect("registration");

        assert_eq!(clone.len().expect("length"), 1);
    }

    #[test]
    fn independent_registries_have_independent_state() {
        let first = ProviderRegistry::new();
        let second = ProviderRegistry::new();

        first
            .register(test_provider("org.test.cpu"))
            .expect("registration");

        assert_eq!(first.len().expect("length"), 1);
        assert_eq!(second.len().expect("length"), 0);
    }
}
