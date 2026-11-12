
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Service Registry
//!
//! Path: src/zur/core/service_registry.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! PURPOSE
//!
//! Provides the authoritative registry for declarative ZUR runtime
//! service descriptors.
//!
//! The registry owns:
//!
//! - Service descriptor registration.
//! - Service contract version indexing.
//! - Service instance identity allocation.
//! - Duplicate registration detection.
//! - Exact service lookup.
//! - Version-compatible dependency resolution.
//! - Dependency initialization ordering.
//! - Circular dependency detection.
//! - Required dependency validation.
//! - Optional dependency handling.
//! - Safe service descriptor removal.
//! - Concurrent registry access.
//!
//! OWNERSHIP BOUNDARIES
//!
//! This module does not own:
//!
//! - Service contract definitions.
//! - Service implementations.
//! - Service initialization.
//! - Service shutdown.
//! - Service activation.
//! - Service lifecycle execution.
//! - Physical resource discovery.
//! - Resource reservation.
//! - Hardware discovery.
//! - Provider implementation.
//! - Backend registration.
//! - Domain provider registration.
//! - Distributed service discovery.
//! - Network service registries.
//! - Source-language service semantics.
//! - Compiler planning.
//!
//! Those responsibilities remain with their authoritative subsystems.
//!
//! ARCHITECTURAL INVARIANTS
//!
//! 1. Registration is not activation.
//! 2. Registration is not availability.
//! 3. Registration is not initialization.
//! 4. Registration is not resource reservation.
//! 5. Registration is not execution success.
//! 6. Required dependencies must resolve before execution.
//! 7. Optional dependencies may be absent.
//! 8. Incompatible versions are never silently accepted.
//! 9. Circular dependencies are rejected.
//! 10. Removal must preserve required dependency validity.
//! 11. Registry state is instance-owned.
//! 12. No global mutable state.
//! 13. No unsafe Rust.
//! 14. No fixed service, version or dependency count.
//!
//! SCALABILITY
//!
//! Registry collections grow according to actual requirements and
//! available allocation capacity.
//!
//! No artificial hardware, provider, service, dependency, domain,
//! or computational-resource ceiling is imposed.
//!
//! This does not promise infinite physical memory.
//!
//! IDENTITY
//!
//! ServiceKey identifies a service contract.
//! ServiceInstanceId identifies one registered service instance.
//!
//! Instance identity allocation is scoped to the supplied allocator
//! namespace. Independently constructed registries must use distinct
//! namespaces if their identities will be combined.
//!
//! INTEGRATION
//!
//! config.rs:
//!     ServiceKey
//!
//! ids.rs:
//!     ServiceInstanceId
//!     IdAllocator
//!     IdNamespace
//!
//! version.rs:
//!     ServiceContractVersion
//!     VersionRange
//!
//! contracts/service.rs:
//!     ServiceDescriptor
//!     ServiceDependency
//!     DependencyRequirement
//!
//! error.rs:
//!     ZurError
//!     ZurResult
//!
//! core/context.rs:
//!     Explicit registry injection.
//!
//! core/service_manager.rs:
//!     Dependency resolution and service lifecycle coordination.
//!
//! core/lifecycle.rs:
//!     Service initialization ordering.
//!
//! execution/:
//!     Runtime service resolution for execution requirements.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::zur::config::ServiceKey;

use crate::zur::contracts::service::{
    DependencyRequirement,
    ServiceDescriptor,
    ServiceDependency,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::ids::{
    IdAllocator,
    IdNamespace,
    ServiceInstanceId,
};

use crate::zur::version::ServiceContractVersion;

// ============================================================================
// REGISTERED SERVICE
// ============================================================================

/// An immutable snapshot of a registered runtime service.
///
/// A registered service contains:
///
/// - Its allocated runtime instance identity.
/// - Its validated service descriptor.
///
/// It does not contain a running service implementation.
///
/// Cloning this structure does not register another service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisteredService {
    instance_id: ServiceInstanceId,
    descriptor: ServiceDescriptor,
}

impl RegisteredService {
    /// Constructs a registered-service snapshot.
    ///
    /// This constructor is private because registration must be
    /// performed through ServiceRegistry.
    fn new(
        instance_id: ServiceInstanceId,
        descriptor: ServiceDescriptor,
    ) -> Self {
        Self {
            instance_id,
            descriptor,
        }
    }

    /// Returns the runtime service instance identity.
    #[must_use]
    pub fn instance_id(&self) -> &ServiceInstanceId {
        &self.instance_id
    }

    /// Returns the immutable service descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ServiceDescriptor {
        &self.descriptor
    }

    /// Returns the service contract identity.
    #[must_use]
    pub fn identity(&self) -> &ServiceKey {
        self.descriptor.identity()
    }

    /// Returns the registered contract version.
    #[must_use]
    pub fn version(&self) -> &ServiceContractVersion {
        self.descriptor.version()
    }
}

// ============================================================================
// REGISTRY ERRORS
// ============================================================================

/// Constructs a structured registry error.
fn registry_error(
    kind: RegistryErrorKind,
    message: impl Into<String>,
    operation: &'static str,
) -> ZurError {
    let error_kind = match kind {
        RegistryErrorKind::InvalidDescriptor => {
            crate::zur::error::ErrorKind::Validation
        }

        RegistryErrorKind::Duplicate => {
            crate::zur::error::ErrorKind::InvalidInput
        }

        RegistryErrorKind::NotFound => {
            crate::zur::error::ErrorKind::ServiceUnavailable
        }

        RegistryErrorKind::Dependency => {
            crate::zur::error::ErrorKind::Dependency
        }

        RegistryErrorKind::Cycle => {
            crate::zur::error::ErrorKind::Dependency
        }

        RegistryErrorKind::LockPoisoned => {
            crate::zur::error::ErrorKind::Concurrency
        }

        RegistryErrorKind::IdentityAllocation => {
            crate::zur::error::ErrorKind::ResourceUnavailable
        }
    };

    ZurError::with_code(
        error_kind,
        match kind {
            RegistryErrorKind::InvalidDescriptor => {
                "zur.registry.invalid_descriptor"
            }

            RegistryErrorKind::Duplicate => {
                "zur.registry.duplicate"
            }

            RegistryErrorKind::NotFound => {
                "zur.registry.not_found"
            }

            RegistryErrorKind::Dependency => {
                "zur.registry.dependency"
            }

            RegistryErrorKind::Cycle => {
                "zur.registry.dependency_cycle"
            }

            RegistryErrorKind::LockPoisoned => {
                "zur.registry.lock_poisoned"
            }

            RegistryErrorKind::IdentityAllocation => {
                "zur.registry.identity_allocation"
            }
        },
        message,
    )
    .unwrap_or_else(|_| {
        ZurError::invalid_data(
            "failed to construct a registry error",
        )
    })
    .with_component("zur.core.service_registry")
    .with_operation(operation)
}

/// Internal registry error classifications.
#[derive(Debug, Clone, Copy)]
enum RegistryErrorKind {
    InvalidDescriptor,
    Duplicate,
    NotFound,
    Dependency,
    Cycle,
    LockPoisoned,
    IdentityAllocation,
}

// ============================================================================
// REGISTRY STORAGE
// ============================================================================

/// Internal service registry state.
///
/// Service identities are indexed first.
///
/// Contract versions are indexed second.
///
/// Ordered collections provide deterministic traversal and lookup.
///
/// Multiple compatible or incompatible versions of the same service
/// may coexist as separate registrations.
#[derive(Debug, Default)]
struct RegistryState {
    services:
        BTreeMap<
            ServiceKey,
            BTreeMap<
                ServiceContractVersion,
                RegisteredService,
            >,
        >,
}

impl RegistryState {
    fn service_count(&self) -> usize {
        self.services
            .values()
            .map(BTreeMap::len)
            .sum()
    }

    fn is_empty(&self) -> bool {
        self.services.is_empty()
    }

    fn find(
        &self,
        identity: &ServiceKey,
        version: &ServiceContractVersion,
    ) -> Option<&RegisteredService> {
        self.services
            .get(identity)?
            .get(version)
    }

    fn find_compatible(
        &self,
        identity: &ServiceKey,
        dependency: &ServiceDependency,
    ) -> Option<&RegisteredService> {
        self.services
            .get(identity)?
            .iter()
            .rev()
            .find(|(version, _)| {
                dependency.accepts_version(version)
            })
            .map(|(_, service)| service)
    }
}

// ============================================================================
// RESOLUTION RESULT
// ============================================================================

/// A validated dependency-resolution result.
///
/// The initialization order places dependencies before their dependents.
///
/// The root service is included as the final entry.
///
/// This structure is a declarative resolution result. It does not
/// initialize, activate, schedule or execute any service.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedServiceGraph {
    root: RegisteredService,
    initialization_order: Vec<RegisteredService>,
}

impl ResolvedServiceGraph {
    /// Returns the requested root service.
    #[must_use]
    pub fn root(&self) -> &RegisteredService {
        &self.root
    }

    /// Returns the dependency-first initialization order.
    ///
    /// The final entry is the root service.
    #[must_use]
    pub fn initialization_order(
        &self,
    ) -> &[RegisteredService] {
        &self.initialization_order
    }

    /// Returns the number of services in the resolved graph.
    #[must_use]
    pub fn len(&self) -> usize {
        self.initialization_order.len()
    }

    /// Returns whether the resolved graph contains no services.
    ///
    /// A successfully resolved graph always contains its root.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.initialization_order.is_empty()
    }
}

// ============================================================================
// SERVICE REGISTRY
// ============================================================================

/// Thread-safe, instance-owned ZUR service registry.
///
/// The registry stores declarative service registrations.
///
/// It does not start or initialize registered services.
///
/// # Concurrency
///
/// Multiple readers may inspect the registry concurrently.
///
/// Mutations are exclusive.
///
/// Registry operations never invoke external service implementations
/// while holding the internal lock.
///
/// # Identity scope
///
/// Each registry receives an explicit ServiceInstanceId allocator.
///
/// The caller must select a namespace appropriate for the identity
/// scope in which the registry's service identities will be used.
///
/// # Example
///
/// ```ignore
/// use crate::zur::core::service_registry::ServiceRegistry;
/// use crate::zur::ids::IdNamespace;
///
/// let namespace = IdNamespace::new(
///     "zamani.runtime.service.instance"
/// )?;
///
/// let registry = ServiceRegistry::new(namespace);
/// ```
pub struct ServiceRegistry {
    state: RwLock<RegistryState>,
    instance_ids: IdAllocator<ServiceInstanceId>,
}

impl ServiceRegistry {
    /// Constructs an empty service registry.
    ///
    /// The namespace is used for allocating ServiceInstanceId values.
    ///
    /// Construction performs no service discovery, registration,
    /// initialization, hardware inspection or resource allocation.
    #[must_use]
    pub fn new(
        identity_namespace: IdNamespace,
    ) -> Self {
        Self {
            state: RwLock::new(RegistryState::default()),
            instance_ids: IdAllocator::new(identity_namespace),
        }
    }

    /// Constructs a registry from a validated namespace string.
    ///
    /// # Errors
    ///
    /// Returns a configuration error when the namespace is invalid.
    pub fn with_namespace(
        namespace: impl AsRef<str>,
    ) -> ZurResult<Self> {
        let namespace = IdNamespace::new(namespace.as_ref())
            .map_err(|error| {
                ZurError::configuration(error.to_string())
                    .with_component(
                        "zur.core.service_registry",
                    )
                    .with_operation("with_namespace")
            })?;

        Ok(Self::new(namespace))
    }

    // ------------------------------------------------------------------------
    // INTERNAL LOCK ACCESS
    // ------------------------------------------------------------------------

    fn read_state(
        &self,
        operation: &'static str,
    ) -> ZurResult<RwLockReadGuard<'_, RegistryState>> {
        self.state.read().map_err(|_| {
            registry_error(
                RegistryErrorKind::LockPoisoned,
                "service registry read lock is poisoned",
                operation,
            )
        })
    }

    fn write_state(
        &self,
        operation: &'static str,
    ) -> ZurResult<RwLockWriteGuard<'_, RegistryState>> {
        self.state.write().map_err(|_| {
            registry_error(
                RegistryErrorKind::LockPoisoned,
                "service registry write lock is poisoned",
                operation,
            )
        })
    }

    // ------------------------------------------------------------------------
    // REGISTRATION
    // ------------------------------------------------------------------------

    /// Registers a service descriptor.
    ///
    /// Registration:
    ///
    /// 1. Validates the descriptor.
    /// 2. Checks for an existing registration with the same identity
    ///    and exact contract version.
    /// 3. Allocates a new ServiceInstanceId.
    /// 4. Inserts the registration atomically.
    /// 5. Checks the resulting registered dependency graph for cycles.
    ///
    /// Missing dependencies are permitted during registration so that
    /// services can be registered in any order.
    ///
    /// Required dependencies must be resolved before a service can
    /// be considered ready for execution.
    ///
    /// Optional dependencies may remain absent.
    ///
    /// Registration does not activate the service.
    ///
    /// # Errors
    ///
    /// Returns an error for:
    ///
    /// - An invalid descriptor.
    /// - Duplicate identity and version.
    /// - An allocation failure.
    /// - A circular dependency introduced by the registration.
    /// - A poisoned registry lock.
    pub fn register(
        &self,
        descriptor: ServiceDescriptor,
    ) -> ZurResult<RegisteredService> {
        descriptor.validate().map_err(|error| {
            registry_error(
                RegistryErrorKind::InvalidDescriptor,
                format!(
                    "invalid service descriptor: {error}"
                ),
                "register",
            )
        })?;

        let identity = descriptor.identity().clone();
        let version = descriptor.version().clone();

        let mut state = self.write_state("register")?;

        // Duplicate registrations are rejected before identity allocation.
        if state.find(&identity, &version).is_some() {
            return Err(registry_error(
                RegistryErrorKind::Duplicate,
                format!(
                    "service `{identity}` version `{version}` \
                     is already registered"
                ),
                "register",
            ));
        }

        // Identity allocation is explicit and scoped to this registry.
        let instance_id = self.instance_ids.try_next().map_err(
            |error| {
                registry_error(
                    RegistryErrorKind::IdentityAllocation,
                    format!(
                        "unable to allocate service instance identity: {error}"
                    ),
                    "register",
                )
            },
        )?;

        let registered = RegisteredService::new(
            instance_id,
            descriptor,
        );

        state
            .services
            .entry(identity.clone())
            .or_default()
            .insert(version.clone(), registered.clone());

        // Check cycles after insertion so forward references are
        // supported while circular dependency graphs are rejected.
        if let Err(error) = validate_dependency_cycles(&state) {
            if let Some(versions) = state.services.get_mut(&identity) {
                versions.remove(&version);

                if versions.is_empty() {
                    state.services.remove(&identity);
                }
            }

            return Err(error);
        }

        Ok(registered)
    }

    // ------------------------------------------------------------------------
    // LOOKUP
    // ------------------------------------------------------------------------

    /// Looks up an exact service identity and contract version.
    ///
    /// Returns a snapshot when the exact registration exists.
    ///
    /// This method does not establish service availability,
    /// initialization or operational readiness.
    pub fn get(
        &self,
        identity: &ServiceKey,
        version: &ServiceContractVersion,
    ) -> ZurResult<Option<RegisteredService>> {
        let state = self.read_state("get")?;

        Ok(state
            .find(identity, version)
            .cloned())
    }

    /// Looks up a registered service by instance identity.
    ///
    /// Instance identities are distinct from service contract identities.
    pub fn get_by_instance_id(
        &self,
        instance_id: &ServiceInstanceId,
    ) -> ZurResult<Option<RegisteredService>> {
        let state = self.read_state("get_by_instance_id")?;

        for versions in state.services.values() {
            for registered in versions.values() {
                if registered.instance_id() == instance_id {
                    return Ok(Some(registered.clone()));
                }
            }
        }

        Ok(None)
    }

    /// Returns the highest registered version of a service.
    ///
    /// This is a registry lookup policy only.
    ///
    /// It does not establish that the version is compatible with
    /// a particular executable artifact or provider.
    pub fn latest(
        &self,
        identity: &ServiceKey,
    ) -> ZurResult<Option<RegisteredService>> {
        let state = self.read_state("latest")?;

        Ok(state
            .services
            .get(identity)
            .and_then(|versions| versions.last_key_value())
            .map(|(_, service)| service.clone()))
    }

    /// Returns all registered versions of a service.
    ///
    /// Versions are returned in deterministic ascending order.
    pub fn versions(
        &self,
        identity: &ServiceKey,
    ) -> ZurResult<Vec<RegisteredService>> {
        let state = self.read_state("versions")?;

        Ok(state
            .services
            .get(identity)
            .map(|versions| {
                versions.values().cloned().collect()
            })
            .unwrap_or_default())
    }

    /// Returns whether an exact identity/version pair is registered.
    pub fn contains(
        &self,
        identity: &ServiceKey,
        version: &ServiceContractVersion,
    ) -> ZurResult<bool> {
        let state = self.read_state("contains")?;

        Ok(state.find(identity, version).is_some())
    }

    /// Returns whether a service identity has any registered versions.
    pub fn contains_identity(
        &self,
        identity: &ServiceKey,
    ) -> ZurResult<bool> {
        let state = self.read_state("contains_identity")?;

        Ok(state.services.contains_key(identity))
    }

    // ------------------------------------------------------------------------
    // DEPENDENCY RESOLUTION
    // ------------------------------------------------------------------------

    /// Resolves a service and all its dependencies.
    ///
    /// Required dependencies must:
    ///
    /// - Be registered.
    /// - Have an accepted contract version.
    /// - Be reachable without circular dependencies.
    ///
    /// Optional dependencies are included when a compatible
    /// registration exists.
    ///
    /// Missing or incompatible optional dependencies are omitted.
    ///
    /// When multiple versions satisfy a dependency, the highest
    /// compatible registered version is selected deterministically.
    ///
    /// The resulting initialization order places dependencies
    /// before their dependents.
    ///
    /// This method does not initialize services or prove that
    /// their implementations are operational.
    pub fn resolve(
        &self,
        identity: &ServiceKey,
        version: &ServiceContractVersion,
    ) -> ZurResult<ResolvedServiceGraph> {
        let state = self.read_state("resolve")?;

        let root = state
            .find(identity, version)
            .cloned()
            .ok_or_else(|| {
                registry_error(
                    RegistryErrorKind::NotFound,
                    format!(
                        "service `{identity}` version `{version}` \
                         is not registered"
                    ),
                    "resolve",
                )
            })?;

        let mut visiting = BTreeSet::new();
        let mut visited = BTreeSet::new();
        let mut order = Vec::new();

        resolve_visit(
            &state,
            &root,
            &mut visiting,
            &mut visited,
            &mut order,
        )?;

        Ok(ResolvedServiceGraph {
            root,
            initialization_order: order,
        })
    }

    /// Resolves the highest registered version of a service.
    ///
    /// The selected root is the highest version currently registered.
    /// Dependency compatibility is then evaluated against the
    /// descriptors of the selected dependency graph.
    pub fn resolve_latest(
        &self,
        identity: &ServiceKey,
    ) -> ZurResult<ResolvedServiceGraph> {
        let service = self
            .latest(identity)?
            .ok_or_else(|| {
                registry_error(
                    RegistryErrorKind::NotFound,
                    format!(
                        "service `{identity}` has no registered versions"
                    ),
                    "resolve_latest",
                )
            })?;

        self.resolve(
            service.identity(),
            service.version(),
        )
    }

    // ------------------------------------------------------------------------
    // REMOVAL
    // ------------------------------------------------------------------------

    /// Removes an exact service registration.
    ///
    /// Removal is rejected if it would leave an existing required
    /// dependency without any compatible registered version.
    ///
    /// Optional dependencies do not prevent removal.
    ///
    /// Removing a descriptor does not stop an already-running
    /// implementation. The service manager owns active-instance
    /// lifecycle and shutdown coordination.
    ///
    /// Callers must coordinate lifecycle shutdown before removal
    /// when the registration is associated with an active instance.
    pub fn remove(
        &self,
        identity: &ServiceKey,
        version: &ServiceContractVersion,
    ) -> ZurResult<Option<RegisteredService>> {
        let mut state = self.write_state("remove")?;

        let Some(existing) = state.find(identity, version).cloned() else {
            return Ok(None);
        };

        // Preserve the integrity of all required dependency declarations.
        for (dependent_identity, versions) in &state.services {
            for dependent in versions.values() {
                for (dependency_identity, dependency)
                    in dependent.descriptor().dependencies()
                {
                    if dependency_identity != identity {
                        continue;
                    }

                    if dependency.requirement()
                        != DependencyRequirement::Required
                    {
                        continue;
                    }

                    let removed_version_is_accepted =
                        dependency.accepts_version(version);

                    if !removed_version_is_accepted {
                        continue;
                    }

                    let alternative_exists = versions_for_dependency(
                        &state,
                        identity,
                        dependency,
                        Some(version),
                    );

                    if !alternative_exists {
                        return Err(registry_error(
                            RegistryErrorKind::Dependency,
                            format!(
                                "cannot remove `{identity}` version `{version}`: \
                                 required by `{dependent_identity}` version `{}` \
                                 and no compatible alternative remains",
                                dependent.version()
                            ),
                            "remove",
                        ));
                    }
                }
            }
        }

        if let Some(versions) = state.services.get_mut(identity) {
            versions.remove(version);

            if versions.is_empty() {
                state.services.remove(identity);
            }
        }

        Ok(Some(existing))
    }

    // ------------------------------------------------------------------------
    // REGISTRY INFORMATION
    // ------------------------------------------------------------------------

    /// Returns the number of registered service identities.
    ///
    /// This is a collection count, not a configured service ceiling.
    pub fn identity_count(&self) -> ZurResult<usize> {
        let state = self.read_state("identity_count")?;

        Ok(state.services.len())
    }

    /// Returns the total number of registered service versions.
    pub fn service_count(&self) -> ZurResult<usize> {
        let state = self.read_state("service_count")?;

        Ok(state.service_count())
    }

    /// Returns whether the registry has no registered services.
    pub fn is_empty(&self) -> ZurResult<bool> {
        let state = self.read_state("is_empty")?;

        Ok(state.is_empty())
    }

    /// Returns a deterministic snapshot of all registered services.
    ///
    /// Services are ordered by identity and then contract version.
    pub fn all(&self) -> ZurResult<Vec<RegisteredService>> {
        let state = self.read_state("all")?;

        Ok(state
            .services
            .values()
            .flat_map(|versions| versions.values().cloned())
            .collect())
    }

    /// Returns the namespace used for service instance identities.
    #[must_use]
    pub fn identity_namespace(&self) -> &IdNamespace {
        self.instance_ids.namespace()
    }
}

// ============================================================================
// DEPENDENCY RESOLUTION ALGORITHM
// ============================================================================

/// Resolves one service recursively.
///
/// The traversal is deterministic because descriptor dependencies
/// and registered service versions use ordered collections.
///
/// The visited set prevents duplicate initialization entries.
///
/// The visiting set detects cycles in the selected dependency graph.
fn resolve_visit(
    state: &RegistryState,
    service: &RegisteredService,
    visiting: &mut BTreeSet<(
        ServiceKey,
        ServiceContractVersion,
    )>,
    visited: &mut BTreeSet<(
        ServiceKey,
        ServiceContractVersion,
    )>,
    order: &mut Vec<RegisteredService>,
) -> ZurResult<()> {
    let identity = service.identity().clone();
    let version = service.version().clone();

    let key = (identity.clone(), version.clone());

    if visited.contains(&key) {
        return Ok(());
    }

    if !visiting.insert(key.clone()) {
        return Err(registry_error(
            RegistryErrorKind::Cycle,
            format!(
                "circular service dependency detected at \
                 `{identity}` version `{version}`"
            ),
            "resolve",
        ));
    }

    for (dependency_identity, dependency)
        in service.descriptor().dependencies()
    {
        let selected = state.find_compatible(
            dependency_identity,
            dependency,
        );

        match selected {
            Some(dependency_service) => {
                resolve_visit(
                    state,
                    dependency_service,
                    visiting,
                    visited,
                    order,
                )?;
            }

            None => {
                if dependency.requirement()
                    == DependencyRequirement::Required
                {
                    visiting.remove(&key);

                    return Err(registry_error(
                        RegistryErrorKind::Dependency,
                        format!(
                            "required dependency `{dependency_identity}` \
                             of `{identity}` version `{version}` \
                             is missing or has no compatible version"
                        ),
                        "resolve",
                    ));
                }
            }
        }
    }

    visiting.remove(&key);

    if visited.insert(key) {
        order.push(service.clone());
    }

    Ok(())
}

// ============================================================================
// REGISTRATION-TIME CYCLE VALIDATION
// ============================================================================

/// Validates all currently resolvable dependency edges.
///
/// Missing dependencies are deliberately ignored here because
/// forward registration references are permitted.
///
/// Once a dependency is registered, its relationship participates
/// in subsequent cycle validation.
///
/// Required dependency completeness is checked during resolve().
fn validate_dependency_cycles(
    state: &RegistryState,
) -> ZurResult<()> {
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();

    for versions in state.services.values() {
        for service in versions.values() {
            validate_cycle_visit(
                state,
                service,
                &mut visiting,
                &mut visited,
            )?;
        }
    }

    Ok(())
}

/// Performs cycle detection over registered compatible dependencies.
fn validate_cycle_visit(
    state: &RegistryState,
    service: &RegisteredService,
    visiting: &mut BTreeSet<(
        ServiceKey,
        ServiceContractVersion,
    )>,
    visited: &mut BTreeSet<(
        ServiceKey,
        ServiceContractVersion,
    )>,
) -> ZurResult<()> {
    let key = (
        service.identity().clone(),
        service.version().clone(),
    );

    if visited.contains(&key) {
        return Ok(());
    }

    if !visiting.insert(key.clone()) {
        return Err(registry_error(
            RegistryErrorKind::Cycle,
            format!(
                "registration introduces a circular dependency involving \
                 `{}` version `{}`",
                service.identity(),
                service.version()
            ),
            "validate_dependency_cycles",
        ));
    }

    for (dependency_identity, dependency)
        in service.descriptor().dependencies()
    {
        if let Some(dependency_service) =
            state.find_compatible(
                dependency_identity,
                dependency,
            )
        {
            validate_cycle_visit(
                state,
                dependency_service,
                visiting,
                visited,
            )?;
        }
    }

    visiting.remove(&key);
    visited.insert(key);

    Ok(())
}

// ============================================================================
// REMOVAL VALIDATION
// ============================================================================

/// Checks whether a compatible alternative remains after removing
/// one particular version.
fn versions_for_dependency(
    state: &RegistryState,
    identity: &ServiceKey,
    dependency: &ServiceDependency,
    excluded: Option<&ServiceContractVersion>,
) -> bool {
    state
        .services
        .get(identity)
        .map(|versions| {
            versions.iter().any(|(version, _)| {
                excluded != Some(version)
                    && dependency.accepts_version(version)
            })
        })
        .unwrap_or(false)
}

// ============================================================================
// DEBUG
// ============================================================================

impl fmt::Debug for ServiceRegistry {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self.state.read() {
            Ok(state) => {
                formatter
                    .debug_struct("ServiceRegistry")
                    .field(
                        "identity_namespace",
                        self.instance_ids.namespace(),
                    )
                    .field(
                        "identity_count",
                        &state.services.len(),
                    )
                    .field(
                        "service_count",
                        &state.service_count(),
                    )
                    .finish()
            }

            Err(_) => {
                formatter
                    .debug_struct("ServiceRegistry")
                    .field(
                        "identity_namespace",
                        self.instance_ids.namespace(),
                    )
                    .field(
                        "state",
                        &"poisoned",
                    )
                    .finish()
            }
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    use crate::zur::contracts::service::{
        ServiceDependency,
    };

    fn namespace() -> IdNamespace {
        IdNamespace::new(
            "zamani.test.runtime.service.instance",
        )
        .expect("valid namespace")
    }

    fn registry() -> ServiceRegistry {
        ServiceRegistry::new(namespace())
    }

    fn key(value: &str) -> ServiceKey {
        ServiceKey::new(value)
            .expect("valid service identity")
    }

    fn version(value: &str) -> ServiceContractVersion {
        ServiceContractVersion::from_str(value)
            .expect("valid service version")
    }

    fn descriptor(
        identity: &str,
        contract_version: &str,
    ) -> ServiceDescriptor {
        ServiceDescriptor::new(
            key(identity),
            version(contract_version),
        )
    }

    fn range(
        minimum: &str,
        maximum: &str,
    ) -> crate::zur::version::VersionRange<
        ServiceContractVersion,
    > {
        crate::zur::version::VersionRange::new(
            version(minimum),
            version(maximum),
        )
        .expect("valid version range")
    }

    // ------------------------------------------------------------------------
    // CONSTRUCTION
    // ------------------------------------------------------------------------

    #[test]
    fn creates_empty_registry() {
        let registry = registry();

        assert!(registry.is_empty().unwrap());
        assert_eq!(registry.identity_count().unwrap(), 0);
        assert_eq!(registry.service_count().unwrap(), 0);
    }

    #[test]
    fn accepts_valid_namespace_string() {
        let registry = ServiceRegistry::with_namespace(
            "zamani.test.registry.instance",
        )
        .unwrap();

        assert!(registry.is_empty().unwrap());
    }

    #[test]
    fn rejects_invalid_namespace_string() {
        assert!(
            ServiceRegistry::with_namespace("")
                .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // REGISTRATION
    // ------------------------------------------------------------------------

    #[test]
    fn registers_service() {
        let registry = registry();

        let registered = registry
            .register(descriptor(
                "zamani.test.executor",
                "1.0.0",
            ))
            .unwrap();

        assert_eq!(
            registered.identity().as_str(),
            "zamani.test.executor",
        );

        assert_eq!(
            registered.version().to_string(),
            "1.0.0",
        );

        assert_eq!(registry.service_count().unwrap(), 1);
    }

    #[test]
    fn allocates_distinct_instance_identities() {
        let registry = registry();

        let first = registry
            .register(descriptor(
                "zamani.test.first",
                "1.0.0",
            ))
            .unwrap();

        let second = registry
            .register(descriptor(
                "zamani.test.second",
                "1.0.0",
            ))
            .unwrap();

        assert_ne!(
            first.instance_id(),
            second.instance_id(),
        );
    }

    #[test]
    fn rejects_duplicate_identity_and_version() {
        let registry = registry();

        registry
            .register(descriptor(
                "zamani.test.executor",
                "1.0.0",
            ))
            .unwrap();

        assert!(
            registry.register(
                descriptor(
                    "zamani.test.executor",
                    "1.0.0",
                )
            ).is_err()
        );

        assert_eq!(registry.service_count().unwrap(), 1);
    }

    #[test]
    fn permits_multiple_versions_of_same_service() {
        let registry = registry();

        registry
            .register(descriptor(
                "zamani.test.executor",
                "1.0.0",
            ))
            .unwrap();

        registry
            .register(descriptor(
                "zamani.test.executor",
                "2.0.0",
            ))
            .unwrap();

        assert_eq!(registry.identity_count().unwrap(), 1);
        assert_eq!(registry.service_count().unwrap(), 2);
    }

    // ------------------------------------------------------------------------
    // LOOKUP
    // ------------------------------------------------------------------------

    #[test]
    fn retrieves_exact_service() {
        let registry = registry();

        let registered = registry
            .register(descriptor(
                "zamani.test.executor",
                "1.0.0",
            ))
            .unwrap();

        let retrieved = registry
            .get(
                registered.identity(),
                registered.version(),
            )
            .unwrap()
            .unwrap();

        assert_eq!(registered, retrieved);
    }

    #[test]
    fn retrieves_service_by_instance_identity() {
        let registry = registry();

        let registered = registry
            .register(descriptor(
                "zamani.test.executor",
                "1.0.0",
            ))
            .unwrap();

        let retrieved = registry
            .get_by_instance_id(
                registered.instance_id(),
            )
            .unwrap()
            .unwrap();

        assert_eq!(registered, retrieved);
    }

    #[test]
    fn missing_service_lookup_returns_none() {
        let registry = registry();

        assert!(
            registry
                .get(
                    &key("zamani.test.executor"),
                    &version("1.0.0"),
                )
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn latest_returns_highest_registered_version() {
        let registry = registry();

        registry
            .register(descriptor(
                "zamani.test.executor",
                "1.0.0",
            ))
            .unwrap();

        registry
            .register(descriptor(
                "zamani.test.executor",
                "2.0.0",
            ))
            .unwrap();

        assert_eq!(
            registry
                .latest(&key("zamani.test.executor"))
                .unwrap()
                .unwrap()
                .version()
                .to_string(),
            "2.0.0",
        );
    }

    // ------------------------------------------------------------------------
    // DEPENDENCIES
    // ------------------------------------------------------------------------

    #[test]
    fn permits_forward_dependency_registration() {
        let registry = registry();

        let dependent = descriptor(
            "zamani.test.executor",
            "1.0.0",
        )
        .with_dependency(
            key("zamani.core.memory"),
            ServiceDependency::required(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap();

        registry.register(dependent).unwrap();

        assert!(
            registry.resolve(
                &key("zamani.test.executor"),
                &version("1.0.0"),
            ).is_err()
        );

        registry
            .register(descriptor(
                "zamani.core.memory",
                "1.5.0",
            ))
            .unwrap();

        assert!(
            registry.resolve(
                &key("zamani.test.executor"),
                &version("1.0.0"),
            ).is_ok()
        );
    }

    #[test]
    fn required_dependency_is_resolved() {
        let registry = registry();

        registry
            .register(descriptor(
                "zamani.core.memory",
                "1.5.0",
            ))
            .unwrap();

        let executor = descriptor(
            "zamani.test.executor",
            "1.0.0",
        )
        .with_dependency(
            key("zamani.core.memory"),
            ServiceDependency::required(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap();

        registry.register(executor).unwrap();

        let graph = registry
            .resolve(
                &key("zamani.test.executor"),
                &version("1.0.0"),
            )
            .unwrap();

        assert_eq!(graph.len(), 2);

        assert_eq!(
            graph.initialization_order()[0]
                .identity()
                .as_str(),
            "zamani.core.memory",
        );

        assert_eq!(
            graph.initialization_order()[1]
                .identity()
                .as_str(),
            "zamani.test.executor",
        );
    }

    #[test]
    fn optional_missing_dependency_is_permitted() {
        let registry = registry();

        let service = descriptor(
            "zamani.test.executor",
            "1.0.0",
        )
        .with_dependency(
            key("zamani.optional.metrics"),
            ServiceDependency::optional(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap();

        registry.register(service).unwrap();

        let graph = registry
            .resolve(
                &key("zamani.test.executor"),
                &version("1.0.0"),
            )
            .unwrap();

        assert_eq!(graph.len(), 1);
    }

    #[test]
    fn optional_compatible_dependency_is_included() {
        let registry = registry();

        registry
            .register(descriptor(
                "zamani.optional.metrics",
                "1.5.0",
            ))
            .unwrap();

        let service = descriptor(
            "zamani.test.executor",
            "1.0.0",
        )
        .with_dependency(
            key("zamani.optional.metrics"),
            ServiceDependency::optional(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap();

        registry.register(service).unwrap();

        let graph = registry
            .resolve(
                &key("zamani.test.executor"),
                &version("1.0.0"),
            )
            .unwrap();

        assert_eq!(graph.len(), 2);
    }

    #[test]
    fn chooses_highest_compatible_dependency_version() {
        let registry = registry();

        registry
            .register(descriptor(
                "zamani.core.memory",
                "1.0.0",
            ))
            .unwrap();

        registry
            .register(descriptor(
                "zamani.core.memory",
                "1.8.0",
            ))
            .unwrap();

        let executor = descriptor(
            "zamani.test.executor",
            "1.0.0",
        )
        .with_dependency(
            key("zamani.core.memory"),
            ServiceDependency::required(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap();

        registry.register(executor).unwrap();

        let graph = registry
            .resolve(
                &key("zamani.test.executor"),
                &version("1.0.0"),
            )
            .unwrap();

        assert_eq!(
            graph.initialization_order()[0]
                .version()
                .to_string(),
            "1.8.0",
        );
    }

    // ------------------------------------------------------------------------
    // CYCLE DETECTION
    // ------------------------------------------------------------------------

    #[test]
    fn rejects_circular_dependencies() {
        let registry = registry();

        let first = descriptor(
            "zamani.test.first",
            "1.0.0",
        )
        .with_dependency(
            key("zamani.test.second"),
            ServiceDependency::required(
                range("1.0.0", "1.0.0"),
            ),
        )
        .unwrap();

        registry.register(first).unwrap();

        let second = descriptor(
            "zamani.test.second",
            "1.0.0",
        )
        .with_dependency(
            key("zamani.test.first"),
            ServiceDependency::required(
                range("1.0.0", "1.0.0"),
            ),
        )
        .unwrap();

        assert!(
            registry.register(second).is_err()
        );

        assert!(
            !registry.contains_identity(
                &key("zamani.test.second"),
            ).unwrap()
        );
    }

    #[test]
    fn detects_optional_dependency_cycles_too() {
        let registry = registry();

        let first = descriptor(
            "zamani.test.first",
            "1.0.0",
        )
        .with_dependency(
            key("zamani.test.second"),
            ServiceDependency::optional(
                range("1.0.0", "1.0.0"),
            ),
        )
        .unwrap();

        registry.register(first).unwrap();

        let second = descriptor(
            "zamani.test.second",
            "1.0.0",
        )
        .with_dependency(
            key("zamani.test.first"),
            ServiceDependency::optional(
                range("1.0.0", "1.0.0"),
            ),
        )
        .unwrap();

        assert!(
            registry.register(second).is_err()
        );
    }

    // ------------------------------------------------------------------------
    // REMOVAL
    // ------------------------------------------------------------------------

    #[test]
    fn removes_unreferenced_service() {
        let registry = registry();

        registry
            .register(descriptor(
                "zamani.test.executor",
                "1.0.0",
            ))
            .unwrap();

        let removed = registry
            .remove(
                &key("zamani.test.executor"),
                &version("1.0.0"),
            )
            .unwrap();

        assert!(removed.is_some());
        assert!(registry.is_empty().unwrap());
    }

    #[test]
    fn rejects_removal_that_breaks_required_dependency() {
        let registry = registry();

        registry
            .register(descriptor(
                "zamani.core.memory",
                "1.0.0",
            ))
            .unwrap();

        let executor = descriptor(
            "zamani.test.executor",
            "1.0.0",
        )
        .with_dependency(
            key("zamani.core.memory"),
            ServiceDependency::required(
                range("1.0.0", "1.0.0"),
            ),
        )
        .unwrap();

        registry.register(executor).unwrap();

        assert!(
            registry.remove(
                &key("zamani.core.memory"),
                &version("1.0.0"),
            ).is_err()
        );
    }

    #[test]
    fn allows_removal_when_compatible_alternative_exists() {
        let registry = registry();

        registry
            .register(descriptor(
                "zamani.core.memory",
                "1.0.0",
            ))
            .unwrap();

        registry
            .register(descriptor(
                "zamani.core.memory",
                "1.5.0",
            ))
            .unwrap();

        let executor = descriptor(
            "zamani.test.executor",
            "1.0.0",
        )
        .with_dependency(
            key("zamani.core.memory"),
            ServiceDependency::required(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap();

        registry.register(executor).unwrap();

        assert!(
            registry.remove(
                &key("zamani.core.memory"),
                &version("1.0.0"),
            ).unwrap().is_some()
        );
    }

    #[test]
    fn absent_removal_returns_none() {
        let registry = registry();

        assert!(
            registry.remove(
                &key("zamani.test.missing"),
                &version("1.0.0"),
            ).unwrap().is_none()
        );
    }

    // ------------------------------------------------------------------------
    // REGISTRY INFORMATION
    // ------------------------------------------------------------------------

    #[test]
    fn counts_service_identities_and_versions() {
        let registry = registry();

        registry
            .register(descriptor(
                "zamani.test.executor",
                "1.0.0",
            ))
            .unwrap();

        registry
            .register(descriptor(
                "zamani.test.executor",
                "2.0.0",
            ))
            .unwrap();

        registry
            .register(descriptor(
                "zamani.core.memory",
                "1.0.0",
            ))
            .unwrap();

        assert_eq!(registry.identity_count().unwrap(), 2);
        assert_eq!(registry.service_count().unwrap(), 3);
    }

    #[test]
    fn all_returns_deterministic_order() {
        let registry = registry();

        registry
            .register(descriptor(
                "zamani.test.zeta",
                "1.0.0",
            ))
            .unwrap();

        registry
            .register(descriptor(
                "zamani.test.alpha",
                "1.0.0",
            ))
            .unwrap();

        let all = registry.all().unwrap();

        assert_eq!(
            all[0].identity().as_str(),
            "zamani.test.alpha",
        );

        assert_eq!(
            all[1].identity().as_str(),
            "zamani.test.zeta",
        );
    }

    // ------------------------------------------------------------------------
    // CONCURRENCY
    // ------------------------------------------------------------------------

    #[test]
    fn concurrent_readers_can_lookup_services() {
        use std::sync::Arc;
        use std::thread;

        let registry = Arc::new(registry());

        registry
            .register(descriptor(
                "zamani.test.executor",
                "1.0.0",
            ))
            .unwrap();

        let handles: Vec<_> = (0..16)
            .map(|_| {
                let registry = Arc::clone(&registry);

                thread::spawn(move || {
                    registry
                        .get(
                            &key("zamani.test.executor"),
                            &version("1.0.0"),
                        )
                        .unwrap()
                        .is_some()
                })
            })
            .collect();

        for handle in handles {
            assert!(handle.join().unwrap());
        }
    }

    #[test]
    fn concurrent_registration_preserves_unique_instances() {
        use std::collections::BTreeSet;
        use std::sync::Arc;
        use std::thread;

        let registry = Arc::new(registry());

        let handles: Vec<_> = (0..16)
            .map(|index| {
                let registry = Arc::clone(&registry);

                thread::spawn(move || {
                    let service = descriptor(
                        &format!(
                            "zamani.test.service{index}"
                        ),
                        "1.0.0",
                    );

                    registry
                        .register(service)
                        .unwrap()
                })
            })
            .collect();

        let mut identities = BTreeSet::new();

        for handle in handles {
            let registered = handle.join().unwrap();

            assert!(
                identities.insert(
                    registered.instance_id().clone()
                )
            );
        }

        assert_eq!(registry.service_count().unwrap(), 16);
    }
}
