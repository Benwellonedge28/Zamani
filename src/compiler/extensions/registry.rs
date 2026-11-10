
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler Extension Provider Registry
//!
//! File: src/compiler/extensions/registry.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Provides the authoritative registry for compiler extension
//! providers implementing the canonical ExtensionProvider contract.
//!
//! This registry manages provider registration, identity lookup,
//! immutable snapshots, removal, and dependency-resolution plans.
//!
//! It does not load, initialize, execute, authorize, or sandbox
//! extension implementations.
//!
//! ## Canonical ownership
//!
//! Provider identity, descriptors and lifecycle:
//!     crate::compiler::extensions::provider
//!
//! Provider manifests:
//!     crate::compiler::extensions::manifest
//!
//! Extension discovery:
//!     crate::compiler::extensions::discovery
//!
//! Provider compatibility:
//!     crate::compiler::extensions::compatibility
//!
//! Extension validation:
//!     crate::compiler::extensions::validation
//!
//! Supply-chain verification:
//!     crate::compiler::security::supply_chain
//!
//! Canonical backend registration:
//!     crate::backend::registry
//!
//! Compiler backend facade:
//!     crate::compiler::backends::registry
//!
//! Canonical pass registration:
//!     crate::zuir::pass::registry
//!
//! Compiler pass facade:
//!     crate::compiler::passes::registry
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Provider registry storage.
//! - Provider registration and removal.
//! - Provider identity uniqueness.
//! - Provider lookup.
//! - Immutable registry snapshots.
//! - Registry generation tracking.
//! - Dependency closure resolution.
//! - Dependency version checking.
//! - Deterministic dependency ordering.
//! - Dependency-cycle reporting.
//!
//! This module does not own:
//!
//! - Provider identity or version definitions.
//! - Manifest parsing or serialization.
//! - Filesystem discovery.
//! - Provider compatibility policy.
//! - Provider admission or authorization.
//! - Publisher authentication.
//! - Cryptographic signature verification.
//! - Dynamic library loading.
//! - Provider initialization or shutdown.
//! - Backend registration.
//! - Pass registration.
//! - Dialect registration.
//! - Target discovery.
//! - Capability definitions.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Scheduling or routing.
//! - Code generation.
//! - Runtime execution.
//!
//! ## Registration security
//!
//! Registration accepts an already constructed provider instance.
//!
//! The caller must complete manifest validation, compatibility
//! evaluation, supply-chain verification and security admission
//! before registering an untrusted provider.
//!
//! Registration itself does not establish trust.
//!
//! The registry does not claim to sandbox provider code.
//!
//! Untrusted providers must not be activated in-process unless
//! the applicable security and isolation requirements permit it.
//!
//! ## Dependency semantics
//!
//! Provider dependencies are declared by ProviderDescriptor.
//!
//! Registration does not require dependencies to have already
//! been registered. This permits providers to be registered in
//! any order.
//!
//! Dependency resolution is performed against an immutable
//! registry snapshot.
//!
//! Dependencies are resolved transitively.
//!
//! A dependency must exist and its registered version must satisfy
//! the declared version requirement.
//!
//! Dependencies are ordered before their dependants.
//!
//! Cycles and unresolved dependencies are explicit errors.
//!
//! This module produces an activation order; it does not activate
//! providers or perform lifecycle operations.
//!
//! ## Determinism
//!
//! Provider identities are stored in BTreeMap.
//!
//! Dependency ordering uses deterministic topological ordering.
//!
//! Registration order does not determine provider identity or
//! dependency execution order.
//!
//! ## Concurrency
//!
//! Cloned registry handles share the same underlying registry.
//!
//! Registry metadata is protected by RwLock.
//!
//! Provider callbacks are not invoked while the registry lock
//! is held.
//!
//! Snapshots are independent immutable views of one registry
//! generation.
//!
//! A snapshot does not change when the live registry changes.
//!
//! ## Scalability
//!
//! No arbitrary universal limits are imposed on:
//!
//! - Registered providers.
//! - Provider dependencies.
//! - Provider versions.
//! - Contributions.
//! - Extension points.
//! - Computing domains.
//! - Computing technologies.
//!
//! Capacity depends on available memory, address space,
//! configured operational budgets and implementation resources.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## Integration invariant
//!
//! This registry is the provider registry only.
//!
//! Specialized implementation registries remain authoritative.
//!
//! A provider contributing a backend must integrate with
//! crate::backend::registry.
//!
//! A provider contributing a compiler pass must integrate with
//! crate::zuir::pass::registry.
//!
//! This module must never become a competing implementation
//! registry for those subsystems.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::{Arc, RwLock};

use super::provider::{
    ExtensionProvider,
    ProviderDescriptor,
    ProviderId,
    ProviderVersion,
    ProviderVersionRequirement,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable identity of the extension provider registry.
pub const EXTENSION_REGISTRY_MODULE_NAME: &str =
    "zamani.compiler.extensions.registry";

/// Public API version of the provider registry.
pub const EXTENSION_REGISTRY_API_VERSION: u16 = 1;

/// Returns the stable registry module identity.
#[must_use]
pub const fn module_name() -> &'static str {
    EXTENSION_REGISTRY_MODULE_NAME
}

/// Returns the registry API version.
#[must_use]
pub const fn api_version() -> u16 {
    EXTENSION_REGISTRY_API_VERSION
}

// ============================================================================
// REGISTERED PROVIDER
// ============================================================================

/// A registered provider and its immutable registration descriptor.
///
/// The descriptor is captured at registration time. Provider
/// implementations are required by the ExtensionProvider contract
/// to keep their descriptor stable throughout their lifetime.
///
/// The provider instance is shared through Arc. Cloning this
/// structure does not duplicate the provider implementation.
#[derive(Clone)]
pub struct RegisteredProvider {
    descriptor: ProviderDescriptor,
    provider: Arc<dyn ExtensionProvider>,
}

impl RegisteredProvider {
    fn new(
        descriptor: ProviderDescriptor,
        provider: Arc<dyn ExtensionProvider>,
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

    /// Returns the registered provider version.
    #[must_use]
    pub const fn version(&self) -> ProviderVersion {
        self.descriptor.version()
    }

    /// Returns a shared handle to the provider implementation.
    ///
    /// This operation does not initialize or execute the provider.
    #[must_use]
    pub fn provider(&self) -> &Arc<dyn ExtensionProvider> {
        &self.provider
    }

    /// Consumes the registration and returns its descriptor
    /// and provider implementation.
    ///
    /// This operation does not call provider shutdown.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (ProviderDescriptor, Arc<dyn ExtensionProvider>) {
        (self.descriptor, self.provider)
    }
}

// ============================================================================
// REGISTRY ERRORS
// ============================================================================

/// Structured errors produced by provider registry operations.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RegistryError {
    /// The provider identity is already registered.
    DuplicateProvider {
        /// Existing provider identity.
        provider_id: ProviderId,
    },

    /// The requested provider is not registered.
    ProviderNotFound {
        /// Requested identity.
        provider_id: ProviderId,
    },

    /// A registered provider declares a missing dependency.
    MissingDependency {
        /// Provider declaring the dependency.
        provider_id: ProviderId,

        /// Missing dependency identity.
        dependency_id: ProviderId,
    },

    /// A registered dependency has an incompatible version.
    IncompatibleDependency {
        /// Provider declaring the dependency.
        provider_id: ProviderId,

        /// Dependency identity.
        dependency_id: ProviderId,

        /// Required version interval.
        required: ProviderVersionRequirement,

        /// Actual registered version.
        found: ProviderVersion,
    },

    /// The selected dependency graph contains a cycle.
    DependencyCycle {
        /// Providers remaining in the cyclic dependency graph.
        ///
        /// This is a deterministic set of unresolved provider IDs.
        providers: Vec<ProviderId>,
    },

    /// Registry generation could not be incremented.
    GenerationOverflow,

    /// A registry lock was poisoned.
    LockPoisoned {
        /// Operation that encountered the poisoned lock.
        operation: &'static str,
    },
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
                    "provider `{provider_id}` is already registered"
                )
            }

            Self::ProviderNotFound { provider_id } => {
                write!(
                    formatter,
                    "provider `{provider_id}` is not registered"
                )
            }

            Self::MissingDependency {
                provider_id,
                dependency_id,
            } => {
                write!(
                    formatter,
                    "provider `{provider_id}` requires missing provider \
                     `{dependency_id}`"
                )
            }

            Self::IncompatibleDependency {
                provider_id,
                dependency_id,
                required,
                found,
            } => {
                write!(
                    formatter,
                    "provider `{provider_id}` requires dependency \
                     `{dependency_id}` with version requirement \
                     {required:?}, but registered version is {found}"
                )
            }

            Self::DependencyCycle { providers } => {
                write!(
                    formatter,
                    "provider dependency cycle involves: "
                )?;

                for (index, provider_id) in providers.iter().enumerate() {
                    if index != 0 {
                        formatter.write_str(", ")?;
                    }

                    write!(formatter, "{provider_id}")?;
                }

                Ok(())
            }

            Self::GenerationOverflow => {
                formatter.write_str(
                    "provider registry generation counter overflowed",
                )
            }

            Self::LockPoisoned { operation } => {
                write!(
                    formatter,
                    "provider registry lock is poisoned during `{operation}`"
                )
            }
        }
    }
}

impl Error for RegistryError {}

// ============================================================================
// INTERNAL REGISTRY STATE
// ============================================================================

#[derive(Default)]
struct RegistryState {
    providers: BTreeMap<ProviderId, RegisteredProvider>,
    generation: u64,
}

// ============================================================================
// PROVIDER REGISTRY
// ============================================================================

/// Thread-safe registry of compiler extension providers.
///
/// Cloning this type shares the same underlying registry.
///
/// Registration and removal are atomic with respect to other registry
/// operations.
///
/// The registry does not impose a fixed provider-count limit.
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

    /// Registers an already constructed provider instance.
    ///
    /// The provider's descriptor is captured before acquiring the
    /// registry lock. The registry never calls initialize or shutdown.
    ///
    /// The caller is responsible for completing validation,
    /// compatibility evaluation, supply-chain verification and
    /// security admission before calling this method.
    ///
    /// A duplicate provider identity is rejected without changing
    /// the registry.
    ///
    /// # Errors
    ///
    /// Returns DuplicateProvider when the identity already exists.
    /// Returns LockPoisoned when registry synchronization has failed.
    /// Returns GenerationOverflow if the generation cannot advance.
    pub fn register<P>(
        &self,
        provider: Arc<P>,
    ) -> Result<(), RegistryError>
    where
        P: ExtensionProvider + 'static,
    {
        let provider: Arc<dyn ExtensionProvider> = provider;
        self.register_shared(provider)
    }

    /// Registers a shared provider trait object.
    ///
    /// This is the object-safe integration path for provider loaders
    /// and independently developed extension hosts.
    ///
    /// The caller must complete security admission before registration.
    ///
    /// # Errors
    ///
    /// Returns DuplicateProvider when the identity already exists.
    /// Returns LockPoisoned when registry synchronization has failed.
    /// Returns GenerationOverflow if the generation cannot advance.
    pub fn register_shared(
        &self,
        provider: Arc<dyn ExtensionProvider>,
    ) -> Result<(), RegistryError> {
        // Capture metadata before acquiring the lock.
        //
        // No provider callback is invoked while the registry lock
        // is held.
        let descriptor = provider.descriptor().clone();
        let provider_id = descriptor.id().clone();

        let registered = RegisteredProvider::new(
            descriptor,
            provider,
        );

        let mut state = self
            .state
            .write()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "register",
            })?;

        if state.providers.contains_key(&provider_id) {
            return Err(RegistryError::DuplicateProvider {
                provider_id,
            });
        }

        let next_generation = state
            .generation
            .checked_add(1)
            .ok_or(RegistryError::GenerationOverflow)?;

        state.providers.insert(
            provider_id,
            registered,
        );

        state.generation = next_generation;

        Ok(())
    }

    /// Returns whether a provider identity is registered.
    ///
    /// This checks registration only. It does not establish that
    /// the provider is compatible, trusted, initialized or available.
    ///
    /// # Errors
    ///
    /// Returns LockPoisoned if registry synchronization has failed.
    pub fn contains(
        &self,
        provider_id: &ProviderId,
    ) -> Result<bool, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "contains",
            })?;

        Ok(state.providers.contains_key(provider_id))
    }

    /// Returns a shared provider implementation handle.
    ///
    /// The returned Arc keeps the provider object alive even if
    /// the provider is subsequently removed from the registry.
    ///
    /// This method does not initialize or execute the provider.
    ///
    /// # Errors
    ///
    /// Returns LockPoisoned if registry synchronization has failed.
    pub fn get(
        &self,
        provider_id: &ProviderId,
    ) -> Result<Option<Arc<dyn ExtensionProvider>>, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "get",
            })?;

        Ok(state
            .providers
            .get(provider_id)
            .map(|registered| Arc::clone(registered.provider())))
    }

    /// Returns a cloned provider descriptor.
    ///
    /// The descriptor is the immutable metadata captured when
    /// the provider was registered.
    ///
    /// # Errors
    ///
    /// Returns LockPoisoned if registry synchronization has failed.
    pub fn descriptor(
        &self,
        provider_id: &ProviderId,
    ) -> Result<Option<ProviderDescriptor>, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "descriptor",
            })?;

        Ok(state
            .providers
            .get(provider_id)
            .map(|registered| registered.descriptor().clone()))
    }

    /// Returns the number of registered providers.
    ///
    /// # Errors
    ///
    /// Returns LockPoisoned if registry synchronization has failed.
    pub fn len(&self) -> Result<usize, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "len",
            })?;

        Ok(state.providers.len())
    }

    /// Returns whether the registry contains no providers.
    ///
    /// # Errors
    ///
    /// Returns LockPoisoned if registry synchronization has failed.
    pub fn is_empty(&self) -> Result<bool, RegistryError> {
        Ok(self.len()? == 0)
    }

    /// Returns the current registry generation.
    ///
    /// The generation advances after each successful registration
    /// or removal. It can be used to detect whether a snapshot
    /// represents an older registry state.
    ///
    /// # Errors
    ///
    /// Returns LockPoisoned if registry synchronization has failed.
    pub fn generation(&self) -> Result<u64, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "generation",
            })?;

        Ok(state.generation)
    }

    /// Returns all registered provider identities in deterministic
    /// lexical order.
    ///
    /// # Errors
    ///
    /// Returns LockPoisoned if registry synchronization has failed.
    pub fn provider_ids(&self) -> Result<Vec<ProviderId>, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "provider_ids",
            })?;

        Ok(state.providers.keys().cloned().collect())
    }

    /// Returns all registered provider descriptors in deterministic
    /// provider-identity order.
    ///
    /// # Errors
    ///
    /// Returns LockPoisoned if registry synchronization has failed.
    pub fn descriptors(
        &self,
    ) -> Result<Vec<ProviderDescriptor>, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "descriptors",
            })?;

        Ok(state
            .providers
            .values()
            .map(|registered| registered.descriptor().clone())
            .collect())
    }

    /// Creates an immutable snapshot of the current registry.
    ///
    /// The snapshot is internally consistent: its provider entries
    /// and generation are copied while holding one read lock.
    ///
    /// Later registration or removal does not mutate the snapshot.
    ///
    /// # Errors
    ///
    /// Returns LockPoisoned if registry synchronization has failed.
    pub fn snapshot(&self) -> Result<RegistrySnapshot, RegistryError> {
        let state = self
            .state
            .read()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "snapshot",
            })?;

        Ok(RegistrySnapshot {
            generation: state.generation,
            providers: state.providers.clone(),
        })
    }

    /// Removes a provider registration by canonical provider identity.
    ///
    /// Removal does not shut down the provider. Existing Arc handles
    /// remain valid and independently owned.
    ///
    /// The caller must coordinate provider lifecycle and ensure
    /// that no new work is dispatched to a removed provider.
    ///
    /// # Errors
    ///
    /// Returns ProviderNotFound if the identity is not registered.
    /// Returns LockPoisoned if registry synchronization has failed.
    /// Returns GenerationOverflow if the generation cannot advance.
    pub fn unregister(
        &self,
        provider_id: &ProviderId,
    ) -> Result<RegisteredProvider, RegistryError> {
        let mut state = self
            .state
            .write()
            .map_err(|_| RegistryError::LockPoisoned {
                operation: "unregister",
            })?;

        if !state.providers.contains_key(provider_id) {
            return Err(RegistryError::ProviderNotFound {
                provider_id: provider_id.clone(),
            });
        }

        let next_generation = state
            .generation
            .checked_add(1)
            .ok_or(RegistryError::GenerationOverflow)?;

        let registered = state
            .providers
            .remove(provider_id)
            .ok_or_else(|| RegistryError::ProviderNotFound {
                provider_id: provider_id.clone(),
            })?;

        state.generation = next_generation;

        Ok(registered)
    }
}

// ============================================================================
// IMMUTABLE REGISTRY SNAPSHOT
// ============================================================================

/// Immutable snapshot of a provider registry generation.
///
/// Snapshots are independent of subsequent registry mutations.
/// They can be used as stable inputs to dependency resolution,
/// compatibility evaluation and compilation planning.
#[derive(Clone, Default)]
pub struct RegistrySnapshot {
    generation: u64,
    providers: BTreeMap<ProviderId, RegisteredProvider>,
}

impl RegistrySnapshot {
    /// Returns the registry generation represented by this snapshot.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns the number of providers in this snapshot.
    #[must_use]
    pub fn len(&self) -> usize {
        self.providers.len()
    }

    /// Returns whether this snapshot contains no providers.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }

    /// Returns whether the snapshot contains the requested provider.
    #[must_use]
    pub fn contains(
        &self,
        provider_id: &ProviderId,
    ) -> bool {
        self.providers.contains_key(provider_id)
    }

    /// Returns a registered provider entry.
    #[must_use]
    pub fn get(
        &self,
        provider_id: &ProviderId,
    ) -> Option<&RegisteredProvider> {
        self.providers.get(provider_id)
    }

    /// Returns provider identities in deterministic order.
    pub fn provider_ids(
        &self,
    ) -> impl Iterator<Item = &ProviderId> {
        self.providers.keys()
    }

    /// Returns registered provider entries in deterministic order.
    pub fn providers(
        &self,
    ) -> impl Iterator<Item = &RegisteredProvider> {
        self.providers.values()
    }

    /// Returns provider descriptors in deterministic order.
    pub fn descriptors(
        &self,
    ) -> impl Iterator<Item = &ProviderDescriptor> {
        self.providers
            .values()
            .map(RegisteredProvider::descriptor)
    }

    /// Resolves the transitive dependency closure for the requested
    /// providers and returns a deterministic dependency-first order.
    ///
    /// The result contains each provider exactly once.
    ///
    /// Dependencies are checked against the versions captured
    /// in this snapshot.
    ///
    /// A provider dependency must be registered and its version
    /// must satisfy the declared version requirement.
    ///
    /// An empty request returns an empty activation order.
    ///
    /// This method does not initialize or activate providers.
    ///
    /// # Errors
    ///
    /// Returns ProviderNotFound for an unknown requested provider.
    /// Returns MissingDependency for an unresolved dependency.
    /// Returns IncompatibleDependency for a version mismatch.
    /// Returns DependencyCycle when the dependency graph is cyclic.
    pub fn activation_order(
        &self,
        requested: &[ProviderId],
    ) -> Result<Vec<ProviderId>, RegistryError> {
        let mut selected = BTreeSet::new();
        let mut pending = Vec::new();

        // Validate explicitly requested roots first.
        for provider_id in requested {
            if !self.providers.contains_key(provider_id) {
                return Err(RegistryError::ProviderNotFound {
                    provider_id: provider_id.clone(),
                });
            }

            pending.push(provider_id.clone());
        }

        // Resolve the transitive dependency closure iteratively.
        //
        // An iterative traversal avoids recursive call-stack growth
        // for long provider dependency chains.
        while let Some(provider_id) = pending.pop() {
            if !selected.insert(provider_id.clone()) {
                continue;
            }

            let registered = self
                .providers
                .get(&provider_id)
                .ok_or_else(|| RegistryError::ProviderNotFound {
                    provider_id: provider_id.clone(),
                })?;

            for (dependency_id, requirement) in
                registered.descriptor().dependencies()
            {
                let dependency = self
                    .providers
                    .get(dependency_id)
                    .ok_or_else(|| RegistryError::MissingDependency {
                        provider_id: provider_id.clone(),
                        dependency_id: dependency_id.clone(),
                    })?;

                let actual_version = dependency.version();

                if !requirement.accepts(actual_version) {
                    return Err(RegistryError::IncompatibleDependency {
                        provider_id: provider_id.clone(),
                        dependency_id: dependency_id.clone(),
                        required: *requirement,
                        found: actual_version,
                    });
                }

                pending.push(dependency_id.clone());
            }
        }

        // Build the dependency graph for the selected closure.
        //
        // indegree(provider) is the number of dependencies that
        // must be processed before that provider.
        let mut indegree: BTreeMap<ProviderId, usize> =
            BTreeMap::new();

        let mut dependants: BTreeMap<
            ProviderId,
            BTreeSet<ProviderId>,
        > = BTreeMap::new();

        for provider_id in &selected {
            let registered = self
                .providers
                .get(provider_id)
                .ok_or_else(|| RegistryError::ProviderNotFound {
                    provider_id: provider_id.clone(),
                })?;

            let dependencies = registered.descriptor().dependencies();

            indegree.insert(
                provider_id.clone(),
                dependencies.len(),
            );

            for dependency_id in dependencies.keys() {
                dependants
                    .entry(dependency_id.clone())
                    .or_default()
                    .insert(provider_id.clone());
            }
        }

        // Kahn's topological sorting algorithm.
        //
        // BTreeSet::pop_first gives deterministic lexical tie-breaking.
        let mut ready: BTreeSet<ProviderId> = indegree
            .iter()
            .filter_map(|(provider_id, degree)| {
                (*degree == 0).then(|| provider_id.clone())
            })
            .collect();

        let mut order = Vec::with_capacity(selected.len());

        while let Some(provider_id) = ready.pop_first() {
            order.push(provider_id.clone());

            if let Some(children) = dependants.get(&provider_id) {
                for dependant_id in children {
                    let degree = indegree
                        .get_mut(dependant_id)
                        .ok_or_else(|| {
                            RegistryError::ProviderNotFound {
                                provider_id: dependant_id.clone(),
                            }
                        })?;

                    *degree = degree.saturating_sub(1);

                    if *degree == 0 {
                        ready.insert(dependant_id.clone());
                    }
                }
            }
        }

        if order.len() != selected.len() {
            let cyclic_providers = indegree
                .into_iter()
                .filter_map(|(provider_id, degree)| {
                    (degree != 0).then_some(provider_id)
                })
                .collect();

            return Err(RegistryError::DependencyCycle {
                providers: cyclic_providers,
            });
        }

        Ok(order)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{
        AtomicUsize,
        Ordering,
    };

    use super::super::provider::{
        api_major,
        ContributionKind,
        ProviderApiCompatibility,
        ProviderDependency,
    };

    static INITIALIZATION_COUNT: AtomicUsize =
        AtomicUsize::new(0);

    struct TestProvider {
        descriptor: ProviderDescriptor,
    }

    impl ExtensionProvider for TestProvider {
        fn descriptor(&self) -> &ProviderDescriptor {
            &self.descriptor
        }

        fn initialize(
            &self,
            _context: &super::super::provider::ProviderContext,
        ) -> Result<(), super::super::provider::ProviderError> {
            INITIALIZATION_COUNT.fetch_add(
                1,
                Ordering::Relaxed,
            );

            Ok(())
        }

        fn shutdown(
            &self,
            _context: &super::super::provider::ProviderContext,
        ) -> Result<(), super::super::provider::ProviderError> {
            Ok(())
        }
    }

    fn id(value: &str) -> ProviderId {
        ProviderId::new(value)
            .expect("test provider identity must be valid")
    }

    fn version(
        major: u64,
        minor: u64,
        patch: u64,
    ) -> ProviderVersion {
        ProviderVersion::new(major, minor, patch)
    }

    fn descriptor(
        provider_id: &str,
        provider_version: ProviderVersion,
        dependencies: impl IntoIterator<Item = ProviderDependency>,
    ) -> ProviderDescriptor {
        let mut descriptor = ProviderDescriptor::new(
            id(provider_id),
            provider_version,
            ProviderApiCompatibility::new(
                api_major(),
                0,
                None,
            )
            .expect("test API compatibility must be valid"),
            provider_id,
            "Registry test provider",
            [
                ContributionKind::new("compiler.extension")
                    .expect("contribution category must be valid"),
            ],
        )
        .expect("test provider descriptor must be valid");

        for dependency in dependencies {
            descriptor = descriptor
                .requiring(dependency)
                .expect("test dependency must be valid");
        }

        descriptor
    }

    fn provider(
        provider_descriptor: ProviderDescriptor,
    ) -> Arc<TestProvider> {
        Arc::new(TestProvider {
            descriptor: provider_descriptor,
        })
    }

    fn dependency(
        provider_id: &str,
        minimum: ProviderVersion,
        maximum_exclusive: ProviderVersion,
    ) -> ProviderDependency {
        ProviderDependency::new(
            id(provider_id),
            ProviderVersionRequirement::new(
                minimum,
                Some(maximum_exclusive),
            )
            .expect("test dependency interval must be valid"),
        )
    }

    #[test]
    fn creates_an_empty_registry() {
        let registry = ProviderRegistry::new();

        assert!(registry.is_empty().unwrap());
        assert_eq!(registry.len().unwrap(), 0);
        assert_eq!(registry.generation().unwrap(), 0);
    }

    #[test]
    fn registers_and_retrieves_a_provider() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider(descriptor(
                "org.example.provider",
                version(1, 0, 0),
                [],
            )))
            .unwrap();

        let provider_id = id("org.example.provider");

        assert!(registry.contains(&provider_id).unwrap());
        assert!(registry.get(&provider_id).unwrap().is_some());

        let registered = registry
            .descriptor(&provider_id)
            .unwrap()
            .unwrap();

        assert_eq!(registered.id(), &provider_id);
        assert_eq!(registered.version(), version(1, 0, 0));
    }

    #[test]
    fn rejects_duplicate_provider_identity() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider(descriptor(
                "org.example.provider",
                version(1, 0, 0),
                [],
            )))
            .unwrap();

        let result = registry.register(provider(descriptor(
            "org.example.provider",
            version(2, 0, 0),
            [],
        )));

        assert!(matches!(
            result,
            Err(RegistryError::DuplicateProvider { .. })
        ));

        assert_eq!(registry.len().unwrap(), 1);
    }

    #[test]
    fn cloned_registry_handles_share_state() {
        let registry = ProviderRegistry::new();
        let another_handle = registry.clone();

        registry
            .register(provider(descriptor(
                "org.example.shared",
                version(1, 0, 0),
                [],
            )))
            .unwrap();

        assert!(
            another_handle
                .contains(&id("org.example.shared"))
                .unwrap()
        );
    }

    #[test]
    fn enumeration_is_deterministic() {
        let registry = ProviderRegistry::new();

        for provider_id in [
            "org.example.zeta",
            "org.example.alpha",
            "org.example.middle",
        ] {
            registry
                .register(provider(descriptor(
                    provider_id,
                    version(1, 0, 0),
                    [],
                )))
                .unwrap();
        }

        let ids: Vec<String> = registry
            .provider_ids()
            .unwrap()
            .iter()
            .map(ToString::to_string)
            .collect();

        assert_eq!(
            ids,
            vec![
                "org.example.alpha",
                "org.example.middle",
                "org.example.zeta",
            ]
        );
    }

    #[test]
    fn snapshots_do_not_change_after_registry_mutation() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider(descriptor(
                "org.example.first",
                version(1, 0, 0),
                [],
            )))
            .unwrap();

        let snapshot = registry.snapshot().unwrap();

        registry
            .register(provider(descriptor(
                "org.example.second",
                version(1, 0, 0),
                [],
            )))
            .unwrap();

        assert_eq!(snapshot.len(), 1);
        assert_eq!(registry.len().unwrap(), 2);
        assert_eq!(snapshot.generation(), 1);
        assert_eq!(registry.generation().unwrap(), 2);
    }

    #[test]
    fn resolves_dependencies_before_dependants() {
        let registry = ProviderRegistry::new();

        let dependency_descriptor = descriptor(
            "org.example.dependency",
            version(1, 2, 0),
            [],
        );

        let dependant_descriptor = descriptor(
            "org.example.dependant",
            version(1, 0, 0),
            [dependency(
                "org.example.dependency",
                version(1, 0, 0),
                version(2, 0, 0),
            )],
        );

        registry
            .register(provider(dependant_descriptor))
            .unwrap();

        registry
            .register(provider(dependency_descriptor))
            .unwrap();

        let snapshot = registry.snapshot().unwrap();

        let order = snapshot
            .activation_order(&[id("org.example.dependant")])
            .unwrap();

        assert_eq!(
            order,
            vec![
                id("org.example.dependency"),
                id("org.example.dependant"),
            ]
        );
    }

    #[test]
    fn resolves_transitive_dependencies() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider(descriptor(
                "org.example.base",
                version(1, 0, 0),
                [],
            )))
            .unwrap();

        registry
            .register(provider(descriptor(
                "org.example.middle",
                version(1, 0, 0),
                [dependency(
                    "org.example.base",
                    version(1, 0, 0),
                    version(2, 0, 0),
                )],
            )))
            .unwrap();

        registry
            .register(provider(descriptor(
                "org.example.application",
                version(1, 0, 0),
                [dependency(
                    "org.example.middle",
                    version(1, 0, 0),
                    version(2, 0, 0),
                )],
            )))
            .unwrap();

        let snapshot = registry.snapshot().unwrap();

        let order = snapshot
            .activation_order(&[id("org.example.application")])
            .unwrap();

        assert_eq!(
            order,
            vec![
                id("org.example.base"),
                id("org.example.middle"),
                id("org.example.application"),
            ]
        );
    }

    #[test]
    fn reports_missing_dependencies() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider(descriptor(
                "org.example.application",
                version(1, 0, 0),
                [dependency(
                    "org.example.missing",
                    version(1, 0, 0),
                    version(2, 0, 0),
                )],
            )))
            .unwrap();

        let snapshot = registry.snapshot().unwrap();

        let result = snapshot.activation_order(&[
            id("org.example.application"),
        ]);

        assert!(matches!(
            result,
            Err(RegistryError::MissingDependency { .. })
        ));
    }

    #[test]
    fn reports_incompatible_dependency_versions() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider(descriptor(
                "org.example.dependency",
                version(3, 0, 0),
                [],
            )))
            .unwrap();

        registry
            .register(provider(descriptor(
                "org.example.application",
                version(1, 0, 0),
                [dependency(
                    "org.example.dependency",
                    version(1, 0, 0),
                    version(2, 0, 0),
                )],
            )))
            .unwrap();

        let snapshot = registry.snapshot().unwrap();

        let result = snapshot.activation_order(&[
            id("org.example.application"),
        ]);

        assert!(matches!(
            result,
            Err(RegistryError::IncompatibleDependency { .. })
        ));
    }

    #[test]
    fn reports_dependency_cycles() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider(descriptor(
                "org.example.first",
                version(1, 0, 0),
                [dependency(
                    "org.example.second",
                    version(1, 0, 0),
                    version(2, 0, 0),
                )],
            )))
            .unwrap();

        registry
            .register(provider(descriptor(
                "org.example.second",
                version(1, 0, 0),
                [dependency(
                    "org.example.first",
                    version(1, 0, 0),
                    version(2, 0, 0),
                )],
            )))
            .unwrap();

        let snapshot = registry.snapshot().unwrap();

        let result = snapshot.activation_order(&[
            id("org.example.first"),
        ]);

        assert!(matches!(
            result,
            Err(RegistryError::DependencyCycle { .. })
        ));
    }

    #[test]
    fn does_not_initialize_providers_during_registration() {
        INITIALIZATION_COUNT.store(0, Ordering::Relaxed);

        let registry = ProviderRegistry::new();

        registry
            .register(provider(descriptor(
                "org.example.not_initialized",
                version(1, 0, 0),
                [],
            )))
            .unwrap();

        assert_eq!(
            INITIALIZATION_COUNT.load(Ordering::Relaxed),
            0
        );
    }

    #[test]
    fn unregister_removes_only_the_registry_entry() {
        let registry = ProviderRegistry::new();

        registry
            .register(provider(descriptor(
                "org.example.provider",
                version(1, 0, 0),
                [],
            )))
            .unwrap();

        let removed = registry
            .unregister(&id("org.example.provider"))
            .unwrap();

        assert_eq!(
            removed.id(),
            &id("org.example.provider")
        );

        assert!(registry.is_empty().unwrap());
        assert_eq!(registry.generation().unwrap(), 2);
    }

    #[test]
    fn unregister_reports_unknown_provider() {
        let registry = ProviderRegistry::new();

        let result = registry.unregister(
            &id("org.example.missing"),
        );

        assert!(matches!(
            result,
            Err(RegistryError::ProviderNotFound { .. })
        ));
    }

    #[test]
    fn empty_activation_request_returns_empty_order() {
        let registry = ProviderRegistry::new();
        let snapshot = registry.snapshot().unwrap();

        assert!(
            snapshot
                .activation_order(&[])
                .unwrap()
                .is_empty()
        );
    }
}
