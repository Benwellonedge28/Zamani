
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
//! Provides a thread-safe, extensible registry for independently
//! implemented Zamani standard-library providers.
//!
//! Providers may implement functionality for:
//!
//! - Classical computing.
//! - Quantum computing.
//! - Hardware description and synthesis.
//! - Hybrid computing.
//! - Distributed computing.
//! - AI and machine learning.
//! - Scientific computing.
//! - Networking.
//! - Storage.
//! - Security.
//! - Embedded computing.
//! - Future computational domains.
//!
//! The registry stores provider metadata and shared provider handles.
//! It does not execute providers or select physical hardware.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - Provider registration.
//! - Provider identity uniqueness.
//! - Provider replacement.
//! - Provider removal.
//! - Provider lookup.
//! - Deterministic enumeration.
//! - Capability-declaration lookup.
//! - Operation-contract lookup.
//! - Provider-kind lookup.
//! - Provider status inspection.
//! - Immutable registry snapshots.
//! - Registry generation tracking.
//! - Optimistic concurrency control.
//!
//! ## DOES NOT OWN
//!
//! - Provider identity definitions.
//! - Provider descriptors.
//! - Provider implementation contracts.
//! - Capability definitions.
//! - Semantic requirements.
//! - Preferences.
//! - Constraints.
//! - Hints.
//! - Feasibility evaluation.
//! - Provider authentication.
//! - Provider authorization.
//! - Provider discovery.
//! - Dynamic library loading.
//! - Hardware discovery.
//! - Device allocation.
//! - Provider initialization.
//! - Provider shutdown.
//! - Provider ranking.
//! - Target selection.
//! - Scheduling.
//! - Routing.
//! - Runtime execution.
//!
//! These responsibilities remain with their canonical authorities.
//!
//! ## AUTHORITATIVE INTEGRATIONS
//!
//! Standard-library provider contracts:
//!     crate::stdlib::provider
//!
//! Standard-library capabilities:
//!     crate::stdlib::capability
//!
//! Standard-library versions:
//!     crate::stdlib::version
//!
//! Compiler extension providers:
//!     crate::compiler::extensions
//!
//! Compiler extension registry:
//!     crate::compiler::extensions::registry
//!
//! Quantum hardware providers:
//!     crate::quantum::hardware::provider
//!
//! Quantum hardware registry:
//!     crate::quantum::hardware::provider_registry
//!
//! Backend providers:
//!     crate::backend
//!
//! Distributed execution:
//!     crate::distributed
//!
//! Runtime:
//!     crate::runtime
//!
//! HAL:
//!     Canonical hardware abstraction and provider interfaces.
//!
//! ## REGISTRY INVARIANTS
//!
//! 1. Provider identities are unique within a registry.
//! 2. Provider identifiers are case-sensitive.
//! 3. Provider descriptors are captured at registration.
//! 4. Provider instances are shared using Arc.
//! 5. Registry entries are ordered deterministically.
//! 6. Every successful mutation increments the generation.
//! 7. Failed mutations do not change the generation.
//! 8. Snapshots remain unchanged after subsequent mutations.
//! 9. Provider callbacks are never invoked while a registry lock
//!    is held.
//! 10. Registry registration does not initialize a provider.
//! 11. Registry membership does not establish provider trust.
//! 12. Declared capabilities are not live availability evidence.
//! 13. Unknown capability information is never interpreted as
//!     unsupported or available.
//! 14. Provider removal does not implicitly shut down a provider.
//! 15. Provider replacement does not implicitly shut down the
//!     replaced provider.
//!
//! ## POCO-REAF
//!
//! This registry is open-world.
//!
//! Adding a new provider, vendor, architecture, computational
//! domain or hardware technology must not require editing this file.
//!
//! There are no universal maximums for:
//!
//! - Providers.
//! - Provider kinds.
//! - Computational domains.
//! - Operations.
//! - Capability declarations.
//! - Provider metadata.
//! - Distributed participants.
//! - Hardware resources.
//! - Identifier lengths.
//! - Namespace depth.
//!
//! Actual capacity is determined by available memory, address-space
//! constraints, operating-system capabilities and explicit operational
//! budgets.
//!
//! Infinite physical storage or computation is not promised.
//!
//! ## CONCURRENCY
//!
//! Registry metadata is protected by RwLock.
//!
//! Cloned registry handles share the same underlying registry.
//!
//! Read operations can proceed concurrently.
//!
//! Mutations are serialized.
//!
//! Snapshots provide independent, immutable views.
//!
//! Provider callbacks are executed only after releasing registry locks.
//!
//! ## VERSIONING
//!
//! Provider implementation versions and provider API versions are
//! preserved in their canonical ProviderDescriptor.
//!
//! Compatibility policy belongs to the appropriate integration
//! authority.
//!
//! This registry permits callers to supply compatibility predicates
//! without imposing a universal version policy.
//!
//! ## SECURITY
//!
//! Registration does not establish trust.
//!
//! The registry must not store provider credentials or secrets.
//!
//! Untrusted executable providers require admission, authentication,
//! authorization and isolation through the appropriate security
//! and runtime authorities.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, RwLock};

use crate::stdlib::capability::CapabilityId;

use crate::stdlib::provider::{
    Provider,
    ProviderDescriptor,
    ProviderId,
    ProviderKind,
    ProviderOperationId,
    ProviderStatus,
};

use crate::stdlib::version::Version;

// ============================================================================
// REGISTRY IDENTITY
// ============================================================================

/// Stable identity of the Zamani standard-library provider registry.
pub const REGISTRY_ID: &str = "zamani.stdlib.provider_registry";

/// Registry contract version.
///
/// This identifies the registry interface, not the package version,
/// provider API version or Zamani language version.
pub const REGISTRY_API_VERSION: &str = "1.0.0";

// ============================================================================
// REGISTERED PROVIDER
// ============================================================================

/// A provider instance and its immutable registration metadata.
///
/// The provider implementation is shared through Arc.
///
/// The descriptor is captured at registration time. Provider
/// implementations must honor the Provider contract requiring
/// descriptor stability throughout the provider instance lifetime.
#[derive(Clone)]
pub struct RegisteredProvider {
    descriptor: ProviderDescriptor,
    provider: Arc<dyn Provider>,
}

impl RegisteredProvider {
    /// Creates a registered-provider representation.
    ///
    /// This constructor is private because registration must be
    /// performed through ProviderRegistry.
    fn new(
        descriptor: ProviderDescriptor,
        provider: Arc<dyn Provider>,
    ) -> Self {
        Self {
            descriptor,
            provider,
        }
    }

    /// Returns the captured provider descriptor.
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

    /// Returns the implementation version.
    #[must_use]
    pub fn implementation_version(&self) -> &Version {
        self.descriptor.implementation_version()
    }

    /// Returns the provider API version.
    #[must_use]
    pub fn api_version(&self) -> &Version {
        self.descriptor.api_version()
    }

    /// Returns the shared provider implementation.
    ///
    /// This does not initialize or execute the provider.
    #[must_use]
    pub fn provider(&self) -> &Arc<dyn Provider> {
        &self.provider
    }

    /// Returns the provider's current observed status.
    ///
    /// This invokes the provider status method.
    ///
    /// It does not reserve resources or guarantee that subsequent
    /// operations will succeed.
    #[must_use]
    pub fn status(&self) -> ProviderStatus {
        self.provider.status()
    }

    /// Returns the captured provider capabilities.
    ///
    /// These are declarations, not evidence of current availability.
    #[must_use]
    pub fn declared_capabilities(
        &self,
    ) -> &[crate::stdlib::capability::Capability] {
        self.descriptor.capabilities()
    }

    /// Returns the captured provider operation declarations.
    #[must_use]
    pub fn operations(
        &self,
    ) -> &[crate::stdlib::provider::ProviderOperationDescriptor] {
        self.descriptor.operations()
    }

    /// Returns the captured provider attributes.
    #[must_use]
    pub fn attributes(
        &self,
    ) -> &std::collections::BTreeMap<String, String> {
        self.descriptor.attributes()
    }
}

impl fmt::Debug for RegisteredProvider {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("RegisteredProvider")
            .field("descriptor", &self.descriptor)
            .finish_non_exhaustive()
    }
}

// ============================================================================
// REGISTRY ERRORS
// ============================================================================

/// Errors returned by provider registry operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RegistryError {
    /// A provider with the specified identity is already registered.
    DuplicateProvider {
        /// Provider identity.
        provider_id: ProviderId,
    },

    /// The requested provider does not exist.
    ProviderNotFound {
        /// Provider identity.
        provider_id: ProviderId,
    },

    /// An expected registry generation did not match.
    ///
    /// This indicates that another mutation occurred after the
    /// caller obtained its expected generation.
    GenerationConflict {
        /// Expected generation.
        expected: u64,

        /// Actual generation.
        actual: u64,
    },

    /// The registry generation cannot be incremented.
    ///
    /// No mutation is performed when this error occurs.
    GenerationExhausted,

    /// The registry lock was poisoned.
    ///
    /// No provider callback is executed while a registry lock
    /// is held, reducing the likelihood of this condition.
    LockPoisoned,
}

impl fmt::Display for RegistryError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::DuplicateProvider { provider_id } => {
                write!(
                    formatter,
                    "provider '{provider_id}' is already registered"
                )
            }

            Self::ProviderNotFound { provider_id } => {
                write!(
                    formatter,
                    "provider '{provider_id}' is not registered"
                )
            }

            Self::GenerationConflict {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "registry generation conflict: expected \
                     {expected}, actual {actual}"
                )
            }

            Self::GenerationExhausted => {
                formatter.write_str(
                    "provider registry generation exhausted",
                )
            }

            Self::LockPoisoned => {
                formatter.write_str(
                    "provider registry lock was poisoned",
                )
            }
        }
    }
}

impl std::error::Error for RegistryError {}

// ============================================================================
// REGISTRATION RECEIPT
// ============================================================================

/// Result of a successful registry mutation.
///
/// The generation identifies the registry state immediately
/// after the mutation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistrationReceipt {
    provider_id: ProviderId,
    generation: u64,
}

impl RegistrationReceipt {
    /// Returns the affected provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the registry generation after the mutation.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }
}

// ============================================================================
// REGISTRY INTERNAL STATE
// ============================================================================

/// Internal registry state.
///
/// BTreeMap provides deterministic identity ordering.
#[derive(Default)]
struct RegistryState {
    providers: BTreeMap<ProviderId, RegisteredProvider>,
    generation: u64,
}

// ============================================================================
// PROVIDER REGISTRY
// ============================================================================

/// Thread-safe, extensible registry of standard-library providers.
///
/// Cloning this structure creates another handle to the same registry.
///
/// The registry is explicitly constructed; no global registry is
/// initialized automatically.
///
/// Provider selection, lifecycle management and execution remain
/// outside this type.
#[derive(Clone)]
pub struct ProviderRegistry {
    state: Arc<RwLock<RegistryState>>,
}

impl ProviderRegistry {
    /// Creates an empty provider registry.
    ///
    /// No provider is discovered or initialized automatically.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(
                RegistryState::default(),
            )),
        }
    }

    // ------------------------------------------------------------------------
    // INTERNAL GENERATION HANDLING
    // ------------------------------------------------------------------------

    /// Calculates the next generation without modifying state.
    fn next_generation(
        current: u64,
    ) -> Result<u64, RegistryError> {
        current
            .checked_add(1)
            .ok_or(RegistryError::GenerationExhausted)
    }

    /// Validates an optional expected generation.
    fn check_generation(
        current: u64,
        expected: Option<u64>,
    ) -> Result<(), RegistryError> {
        if let Some(expected) = expected {
            if current != expected {
                return Err(
                    RegistryError::GenerationConflict {
                        expected,
                        actual: current,
                    },
                );
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // REGISTRATION
    // ------------------------------------------------------------------------

    /// Registers a provider.
    ///
    /// Provider construction, validation and trust admission must
    /// happen before registration.
    ///
    /// Registration does not initialize the provider.
    ///
    /// The provider descriptor is captured before acquiring the
    /// registry lock.
    ///
    /// # Errors
    ///
    /// Returns:
    ///
    /// - DuplicateProvider if the identity already exists.
    /// - GenerationExhausted if generation cannot advance.
    /// - LockPoisoned if the registry lock is poisoned.
    pub fn register(
        &self,
        provider: Arc<dyn Provider>,
    ) -> Result<RegistrationReceipt, RegistryError> {
        self.register_internal(provider, None)
    }

    /// Registers a provider only if the registry has the expected
    /// generation.
    ///
    /// This operation is atomic with respect to concurrent mutations.
    ///
    /// # Errors
    ///
    /// Returns GenerationConflict if the registry changed.
    pub fn register_if_generation(
        &self,
        provider: Arc<dyn Provider>,
        expected_generation: u64,
    ) -> Result<RegistrationReceipt, RegistryError> {
        self.register_internal(
            provider,
            Some(expected_generation),
        )
    }

    fn register_internal(
        &self,
        provider: Arc<dyn Provider>,
        expected_generation: Option<u64>,
    ) -> Result<RegistrationReceipt, RegistryError> {
        // Read immutable metadata before taking the registry lock.
        let descriptor = provider.descriptor().clone();
        let provider_id = descriptor.id().clone();

        let registered = RegisteredProvider::new(
            descriptor,
            provider,
        );

        let mut state = self
            .state
            .write()
            .map_err(|_| RegistryError::LockPoisoned)?;

        Self::check_generation(
            state.generation,
            expected_generation,
        )?;

        if state.providers.contains_key(&provider_id) {
            return Err(
                RegistryError::DuplicateProvider {
                    provider_id,
                },
            );
        }

        let next = Self::next_generation(state.generation)?;

        state.providers.insert(
            provider_id.clone(),
            registered,
        );

        state.generation = next;

        Ok(RegistrationReceipt {
            provider_id,
            generation: next,
        })
    }

    // ------------------------------------------------------------------------
    // REPLACEMENT
    // ------------------------------------------------------------------------

    /// Replaces an existing provider with another instance having
    /// the same canonical identity.
    ///
    /// Returns the previous registration.
    ///
    /// The caller is responsible for coordinating the previous
    /// provider's lifecycle after this method returns.
    ///
    /// The replaced provider is not shut down automatically.
    ///
    /// # Errors
    ///
    /// Returns ProviderNotFound if the identity is absent.
    pub fn replace(
        &self,
        provider: Arc<dyn Provider>,
    ) -> Result<
        (RegistrationReceipt, RegisteredProvider),
        RegistryError,
    > {
        self.replace_internal(provider, None)
    }

    /// Replaces a provider only if the registry generation matches.
    ///
    /// Returns the previous registration.
    pub fn replace_if_generation(
        &self,
        provider: Arc<dyn Provider>,
        expected_generation: u64,
    ) -> Result<
        (RegistrationReceipt, RegisteredProvider),
        RegistryError,
    > {
        self.replace_internal(
            provider,
            Some(expected_generation),
        )
    }

    fn replace_internal(
        &self,
        provider: Arc<dyn Provider>,
        expected_generation: Option<u64>,
    ) -> Result<
        (RegistrationReceipt, RegisteredProvider),
        RegistryError,
    > {
        let descriptor = provider.descriptor().clone();
        let provider_id = descriptor.id().clone();

        let registered = RegisteredProvider::new(
            descriptor,
            provider,
        );

        let mut state = self
            .state
            .write()
            .map_err(|_| RegistryError::LockPoisoned)?;

        Self::check_generation(
            state.generation,
            expected_generation,
        )?;

        if !state.providers.contains_key(&provider_id) {
            return Err(
                RegistryError::ProviderNotFound {
                    provider_id,
                },
            );
        }

        let next = Self::next_generation(state.generation)?;

        let previous = state.providers.insert(
            provider_id.clone(),
            registered,
        );

        state.generation = next;

        let previous = previous.ok_or(
            RegistryError::LockPoisoned,
        )?;

        Ok((
            RegistrationReceipt {
                provider_id,
                generation: next,
            },
            previous,
        ))
    }

    // ------------------------------------------------------------------------
    // REMOVAL
    // ------------------------------------------------------------------------

    /// Removes a registered provider.
    ///
    /// Returns the removed registration.
    ///
    /// Removal does not automatically shut down the provider.
    ///
    /// The returned registration allows the lifecycle authority
    /// to coordinate shutdown after the registry lock is released.
    ///
    /// # Errors
    ///
    /// Returns ProviderNotFound if the provider is absent.
    pub fn remove(
        &self,
        provider_id: &ProviderId,
    ) -> Result<
        (RegistrationReceipt, RegisteredProvider),
        RegistryError,
    > {
        self.remove_internal(provider_id, None)
    }

    /// Removes a provider only if the registry generation matches.
    pub fn remove_if_generation(
        &self,
        provider_id: &ProviderId,
        expected_generation: u64,
    ) -> Result<
        (RegistrationReceipt, RegisteredProvider),
        RegistryError,
    > {
        self.remove_internal(
            provider_id,
            Some(expected_generation),
        )
    }

    fn remove_internal(
        &self,
        provider_id: &ProviderId,
        expected_generation: Option<u64>,
    ) -> Result<
        (RegistrationReceipt, RegisteredProvider),
        RegistryError,
    > {
        let mut state = self
            .state
            .write()
            .map_err(|_| RegistryError::LockPoisoned)?;

        Self::check_generation(
            state.generation,
            expected_generation,
        )?;

        if !state.providers.contains_key(provider_id) {
            return Err(
                RegistryError::ProviderNotFound {
                    provider_id: provider_id.clone(),
                },
            );
        }

        let next = Self::next_generation(state.generation)?;

        let removed = state.providers.remove(provider_id);

        state.generation = next;

        let removed = removed.ok_or(
            RegistryError::LockPoisoned,
        )?;

        Ok((
            RegistrationReceipt {
                provider_id: provider_id.clone(),
                generation: next,
            },
            removed,
        ))
    }

    // ------------------------------------------------------------------------
    // EXACT LOOKUP
    // ------------------------------------------------------------------------

    /// Returns a registered provider by exact identity.
    ///
    /// The returned value is an independent shared registration handle.
    #[must_use]
    pub fn get(
        &self,
        provider_id: &ProviderId,
    ) -> Result<Option<RegisteredProvider>, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned)?;

        Ok(state.providers.get(provider_id).cloned())
    }

    /// Returns whether a provider identity is registered.
    pub fn contains(
        &self,
        provider_id: &ProviderId,
    ) -> Result<bool, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned)?;

        Ok(state.providers.contains_key(provider_id))
    }

    // ------------------------------------------------------------------------
    // REGISTRY INFORMATION
    // ------------------------------------------------------------------------

    /// Returns the number of registered providers.
    pub fn len(&self) -> Result<usize, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned)?;

        Ok(state.providers.len())
    }

    /// Returns whether the registry is empty.
    pub fn is_empty(&self) -> Result<bool, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned)?;

        Ok(state.providers.is_empty())
    }

    /// Returns the current registry generation.
    pub fn generation(&self) -> Result<u64, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned)?;

        Ok(state.generation)
    }

    // ------------------------------------------------------------------------
    // IMMUTABLE SNAPSHOTS
    // ------------------------------------------------------------------------

    /// Creates an immutable snapshot of the current registry.
    ///
    /// The snapshot is independent of subsequent registry mutations.
    ///
    /// Snapshot construction clones registry metadata and Arc handles.
    /// Its cost therefore grows with the number of registered providers.
    ///
    /// No provider callbacks are invoked.
    pub fn snapshot(
        &self,
    ) -> Result<RegistrySnapshot, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned)?;

        Ok(RegistrySnapshot {
            generation: state.generation,
            providers: state.providers.clone(),
        })
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// IMMUTABLE REGISTRY SNAPSHOT
// ============================================================================

/// An immutable point-in-time view of a provider registry.
///
/// Snapshots allow consumers to inspect registry metadata without
/// holding a lock on the live registry.
///
/// A snapshot never changes when the live registry changes.
#[derive(Clone, Debug)]
pub struct RegistrySnapshot {
    generation: u64,
    providers: BTreeMap<ProviderId, RegisteredProvider>,
}

impl RegistrySnapshot {
    /// Returns the generation represented by this snapshot.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns the number of providers in the snapshot.
    #[must_use]
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Returns whether the snapshot contains no providers.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    /// Returns a provider by exact identity.
    #[must_use]
    pub fn get(
        &self,
        provider_id: &ProviderId,
    ) -> Option<&RegisteredProvider> {
        self.providers.get(provider_id)
    }

    /// Returns providers in deterministic identity order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &RegisteredProvider> {
        self.providers.values()
    }

    /// Returns providers declaring the specified kind.
    ///
    /// Provider kinds are descriptive metadata, not exclusive
    /// computational-domain restrictions.
    pub fn by_kind<'a>(
        &'a self,
        kind: &'a ProviderKind,
    ) -> impl Iterator<Item = &'a RegisteredProvider> + 'a {
        self.providers
            .values()
            .filter(move |provider| provider.kind() == kind)
    }

    /// Returns providers declaring a capability identity.
    ///
    /// This examines provider declarations only.
    ///
    /// It does not establish current capability availability.
    pub fn declaring_capability<'a>(
        &'a self,
        capability_id: &'a CapabilityId,
    ) -> impl Iterator<Item = &'a RegisteredProvider> + 'a {
        self.providers.values().filter(
            move |provider| {
                provider
                    .descriptor()
                    .capabilities()
                    .iter()
                    .any(|capability| {
                        capability.id() == capability_id
                    })
            },
        )
    }

    /// Returns providers declaring a specific operation.
    pub fn declaring_operation<'a>(
        &'a self,
        operation_id: &'a ProviderOperationId,
    ) -> impl Iterator<Item = &'a RegisteredProvider> + 'a {
        self.providers.values().filter(
            move |provider| {
                provider
                    .descriptor()
                    .operation(operation_id)
                    .is_some()
            },
        )
    }

    /// Returns providers with a specified observed status.
    ///
    /// This invokes each provider's status method after the snapshot
    /// has been created.
    ///
    /// Status is observational and may change immediately afterward.
    pub fn with_status(
        &self,
        status: ProviderStatus,
    ) -> impl Iterator<Item = &RegisteredProvider> {
        self.providers
            .values()
            .filter(move |provider| provider.status() == status)
    }

    /// Returns providers whose API versions satisfy a caller-supplied
    /// compatibility predicate.
    ///
    /// Version compatibility policy is deliberately supplied by the
    /// integration authority.
    ///
    /// This method does not impose a universal major-version policy.
    pub fn matching_api_version<F>(
        &self,
        mut compatible: F,
    ) -> impl Iterator<Item = &RegisteredProvider>
    where
        F: FnMut(&Version) -> bool,
    {
        self.providers
            .values()
            .filter(move |provider| {
                compatible(provider.api_version())
            })
    }

    /// Returns providers whose implementation versions satisfy a
    /// caller-supplied predicate.
    pub fn matching_implementation_version<F>(
        &self,
        mut compatible: F,
    ) -> impl Iterator<Item = &RegisteredProvider>
    where
        F: FnMut(&Version) -> bool,
    {
        self.providers
            .values()
            .filter(move |provider| {
                compatible(provider.implementation_version())
            })
    }

    /// Returns provider identities in deterministic order.
    #[must_use]
    pub fn provider_ids(&self) -> Vec<ProviderId> {
        self.providers.keys().cloned().collect()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::capability::{
        Capability,
        CapabilityKind,
        CapabilitySnapshot,
    };

    use crate::stdlib::context::OperationContext;

    use crate::stdlib::error::Result as ProviderResult;

    use crate::stdlib::provider::{
        ProviderFuture,
        ProviderOperationDescriptor,
        ProviderStatus,
    };

    fn version(value: &str) -> Version {
        Version::parse(value)
            .expect("test version must be valid")
    }

    fn provider_id(value: &str) -> ProviderId {
        ProviderId::new(value)
            .expect("test provider identity must be valid")
    }

    fn provider_kind(value: &str) -> ProviderKind {
        ProviderKind::new(value)
            .expect("test provider kind must be valid")
    }

    fn capability_id(value: &str) -> CapabilityId {
        CapabilityId::new(value)
            .expect("test capability identity must be valid")
    }

    struct TestProvider {
        descriptor: ProviderDescriptor,
        status: ProviderStatus,
    }

    impl TestProvider {
        fn new(
            id: &str,
            kind: &str,
            status: ProviderStatus,
        ) -> Self {
            Self {
                descriptor: ProviderDescriptor::new(
                    provider_id(id),
                    provider_kind(kind),
                    format!("Test provider {id}"),
                    version("1.0.0"),
                    version("1.0.0"),
                )
                .expect("test descriptor must be valid"),
                status,
            }
        }

        fn with_capability(
            mut self,
            id: &str,
        ) -> Self {
            self.descriptor
                .insert_capability(
                    Capability::new(
                        capability_id(id),
                        CapabilityKind::new("computation")
                            .expect("valid capability kind"),
                    ),
                )
                .expect("capability declaration must be unique");

            self
        }

        fn with_operation(
            mut self,
            id: &str,
        ) -> Self {
            self.descriptor
                .insert_operation(
                    ProviderOperationDescriptor::from_id(
                        id,
                        version("1.0.0"),
                    )
                    .expect("valid operation"),
                )
                .expect("operation declaration must be unique");

            self
        }
    }

    impl Provider for TestProvider {
        fn descriptor(&self) -> &ProviderDescriptor {
            &self.descriptor
        }

        fn status(&self) -> ProviderStatus {
            self.status
        }

        fn capability_snapshot<'a>(
            &'a self,
            _context: &'a OperationContext,
        ) -> ProviderFuture<'a, CapabilitySnapshot> {
            Box::pin(async {
                Ok(CapabilitySnapshot::new())
            })
        }
    }

    fn provider(
        id: &str,
        kind: &str,
    ) -> Arc<dyn Provider> {
        Arc::new(TestProvider::new(
            id,
            kind,
            ProviderStatus::Ready,
        ))
    }

    // ------------------------------------------------------------------------
    // BASIC REGISTRATION
    // ------------------------------------------------------------------------

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

        let receipt = registry
            .register(provider(
                "org.example.classical",
                "classical",
            ))
            .unwrap();

        assert_eq!(
            receipt.provider_id().as_str(),
            "org.example.classical"
        );

        assert_eq!(receipt.generation(), 1);
        assert_eq!(registry.len().unwrap(), 1);
    }

    #[test]
    fn rejects_duplicate_identity() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider(
                "org.example.classical",
                "classical",
            ))
            .unwrap();

        let result = registry.register(provider(
            "org.example.classical",
            "classical",
        ));

        assert!(matches!(
            result,
            Err(RegistryError::DuplicateProvider { .. })
        ));

        assert_eq!(registry.generation().unwrap(), 1);
        assert_eq!(registry.len().unwrap(), 1);
    }

    #[test]
    fn provider_identity_is_case_sensitive() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider("Org.Example", "classical"))
            .unwrap();

        registry
            .register(provider("org.example", "classical"))
            .unwrap();

        assert_eq!(registry.len().unwrap(), 2);
    }

    // ------------------------------------------------------------------------
    // LOOKUP
    // ------------------------------------------------------------------------

    #[test]
    fn finds_provider_by_identity() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider(
                "org.example.quantum",
                "quantum",
            ))
            .unwrap();

        let result = registry
            .get(&provider_id("org.example.quantum"))
            .unwrap();

        assert!(result.is_some());
    }

    #[test]
    fn returns_none_for_unknown_identity() {
        let registry = ProviderRegistry::new();

        assert!(
            registry
                .get(&provider_id("future.provider"))
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn contains_checks_registration() {
        let registry = ProviderRegistry::new();

        let id = provider_id("org.example.ai");

        assert!(!registry.contains(&id).unwrap());

        registry
            .register(provider(
                "org.example.ai",
                "ai",
            ))
            .unwrap();

        assert!(registry.contains(&id).unwrap());
    }

    // ------------------------------------------------------------------------
    // REPLACEMENT
    // ------------------------------------------------------------------------

    #[test]
    fn replaces_existing_provider() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider(
                "org.example.quantum",
                "quantum",
            ))
            .unwrap();

        let (receipt, previous) = registry
            .replace(Arc::new(TestProvider::new(
                "org.example.quantum",
                "quantum",
                ProviderStatus::Degraded,
            )))
            .unwrap();

        assert_eq!(receipt.generation(), 2);

        assert_eq!(
            previous.id().as_str(),
            "org.example.quantum"
        );

        assert_eq!(
            registry
                .get(&provider_id("org.example.quantum"))
                .unwrap()
                .unwrap()
                .status(),
            ProviderStatus::Degraded
        );
    }

    #[test]
    fn cannot_replace_unknown_provider() {
        let registry = ProviderRegistry::new();

        let result = registry.replace(provider(
            "org.example.missing",
            "classical",
        ));

        assert!(matches!(
            result,
            Err(RegistryError::ProviderNotFound { .. })
        ));

        assert_eq!(registry.generation().unwrap(), 0);
    }

    // ------------------------------------------------------------------------
    // REMOVAL
    // ------------------------------------------------------------------------

    #[test]
    fn removes_registered_provider() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider(
                "org.example.classical",
                "classical",
            ))
            .unwrap();

        let (receipt, removed) = registry
            .remove(&provider_id("org.example.classical"))
            .unwrap();

        assert_eq!(receipt.generation(), 2);

        assert_eq!(
            removed.id().as_str(),
            "org.example.classical"
        );

        assert!(registry.is_empty().unwrap());
    }

    #[test]
    fn cannot_remove_unknown_provider() {
        let registry = ProviderRegistry::new();

        let result = registry.remove(
            &provider_id("org.example.missing"),
        );

        assert!(matches!(
            result,
            Err(RegistryError::ProviderNotFound { .. })
        ));

        assert_eq!(registry.generation().unwrap(), 0);
    }

    // ------------------------------------------------------------------------
    // GENERATION AND CONCURRENCY
    // ------------------------------------------------------------------------

    #[test]
    fn every_successful_mutation_increments_generation() {
        let registry = ProviderRegistry::new();

        assert_eq!(registry.generation().unwrap(), 0);

        registry
            .register(provider("org.example.one", "classical"))
            .unwrap();

        assert_eq!(registry.generation().unwrap(), 1);

        registry
            .replace(provider("org.example.one", "classical"))
            .unwrap();

        assert_eq!(registry.generation().unwrap(), 2);

        registry
            .remove(&provider_id("org.example.one"))
            .unwrap();

        assert_eq!(registry.generation().unwrap(), 3);
    }

    #[test]
    fn failed_mutations_do_not_increment_generation() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider("org.example.one", "classical"))
            .unwrap();

        let result = registry.register(provider(
            "org.example.one",
            "classical",
        ));

        assert!(result.is_err());
        assert_eq!(registry.generation().unwrap(), 1);
    }

    #[test]
    fn conditional_registration_detects_stale_generation() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider("org.example.one", "classical"))
            .unwrap();

        let result = registry.register_if_generation(
            provider("org.example.two", "quantum"),
            0,
        );

        assert!(matches!(
            result,
            Err(RegistryError::GenerationConflict {
                expected: 0,
                actual: 1,
            })
        ));

        assert_eq!(registry.len().unwrap(), 1);
    }

    #[test]
    fn conditional_registration_accepts_matching_generation() {
        let registry = ProviderRegistry::new();

        let receipt = registry
            .register_if_generation(
                provider("org.example.one", "classical"),
                0,
            )
            .unwrap();

        assert_eq!(receipt.generation(), 1);
    }

    #[test]
    fn cloned_registry_handles_share_state() {
        let first = ProviderRegistry::new();
        let second = first.clone();

        first
            .register(provider("org.example.one", "classical"))
            .unwrap();

        assert_eq!(second.len().unwrap(), 1);
        assert_eq!(second.generation().unwrap(), 1);
    }

    // ------------------------------------------------------------------------
    // SNAPSHOTS
    // ------------------------------------------------------------------------

    #[test]
    fn snapshot_is_immutable_after_registry_mutation() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider("org.example.one", "classical"))
            .unwrap();

        let snapshot = registry.snapshot().unwrap();

        registry
            .register(provider("org.example.two", "quantum"))
            .unwrap();

        assert_eq!(snapshot.len(), 1);
        assert_eq!(snapshot.generation(), 1);

        assert_eq!(registry.len().unwrap(), 2);
        assert_eq!(registry.generation().unwrap(), 2);
    }

    #[test]
    fn snapshot_iteration_is_deterministic() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider("org.example.zeta", "classical"))
            .unwrap();

        registry
            .register(provider("org.example.alpha", "quantum"))
            .unwrap();

        let snapshot = registry.snapshot().unwrap();

        let ids: Vec<String> = snapshot
            .iter()
            .map(|provider| provider.id().to_string())
            .collect();

        assert_eq!(
            ids,
            vec![
                "org.example.alpha",
                "org.example.zeta",
            ]
        );
    }

    // ------------------------------------------------------------------------
    // CAPABILITY AND OPERATION DECLARATIONS
    // ------------------------------------------------------------------------

    #[test]
    fn finds_providers_declaring_capability() {
        let registry = ProviderRegistry::new();

        let provider = TestProvider::new(
            "org.example.quantum",
            "quantum",
            ProviderStatus::Ready,
        )
        .with_capability("quantum.measurement");

        registry
            .register(Arc::new(provider))
            .unwrap();

        let snapshot = registry.snapshot().unwrap();

        let matches: Vec<_> = snapshot
            .declaring_capability(
                &capability_id("quantum.measurement"),
            )
            .collect();

        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn absent_capability_declaration_is_not_unsupported_evidence() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider(
                "org.example.future",
                "future",
            ))
            .unwrap();

        let snapshot = registry.snapshot().unwrap();

        let matches: Vec<_> = snapshot
            .declaring_capability(
                &capability_id("future.computing.operation"),
            )
            .collect();

        assert!(matches.is_empty());

        // An empty result means no matching declaration was found.
        // It does not establish that the operation is impossible.
    }

    #[test]
    fn finds_providers_declaring_operation() {
        let registry = ProviderRegistry::new();

        let provider = TestProvider::new(
            "org.example.classical",
            "classical",
            ProviderStatus::Ready,
        )
        .with_operation("classical.matrix_multiplication");

        registry
            .register(Arc::new(provider))
            .unwrap();

        let snapshot = registry.snapshot().unwrap();

        let operation = ProviderOperationId::new(
            "classical.matrix_multiplication",
        )
        .unwrap();

        assert_eq!(
            snapshot.declaring_operation(&operation).count(),
            1
        );
    }

    // ------------------------------------------------------------------------
    // KIND AND STATUS
    // ------------------------------------------------------------------------

    #[test]
    fn filters_by_open_world_provider_kind() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider("org.example.one", "quantum"))
            .unwrap();

        registry
            .register(provider("org.example.two", "classical"))
            .unwrap();

        let snapshot = registry.snapshot().unwrap();

        assert_eq!(
            snapshot
                .by_kind(&provider_kind("quantum"))
                .count(),
            1
        );
    }

    #[test]
    fn filters_by_observed_status() {
        let registry = ProviderRegistry::new();

        registry
            .register(Arc::new(TestProvider::new(
                "org.example.ready",
                "classical",
                ProviderStatus::Ready,
            )))
            .unwrap();

        registry
            .register(Arc::new(TestProvider::new(
                "org.example.degraded",
                "quantum",
                ProviderStatus::Degraded,
            )))
            .unwrap();

        let snapshot = registry.snapshot().unwrap();

        assert_eq!(
            snapshot
                .with_status(ProviderStatus::Ready)
                .count(),
            1
        );
    }

    // ------------------------------------------------------------------------
    // VERSION METADATA
    // ------------------------------------------------------------------------

    #[test]
    fn filters_api_versions_using_external_policy() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider("org.example.one", "classical"))
            .unwrap();

        let snapshot = registry.snapshot().unwrap();

        let matches: Vec<_> = snapshot
            .matching_api_version(|version| {
                version.precedence_cmp(&version!("1.0.0"))
                    == std::cmp::Ordering::Equal
            })
            .collect();

        assert_eq!(matches.len(), 1);
    }
}
