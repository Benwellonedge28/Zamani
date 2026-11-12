
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Service Manager
//!
//! Path: src/zur/core/service_manager.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97+
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Manages the operational lifecycle of optional ZUR services.
//!
//! The manager coordinates:
//!
//! - Service implementation registration.
//! - Lazy activation.
//! - Explicit eager activation.
//! - Dependency-first initialization.
//! - Required and optional dependencies.
//! - Readiness verification.
//! - Reference-counted service leases.
//! - Transactional startup rollback.
//! - Reverse-order shutdown.
//! - Cooperative cancellation.
//! - Failure propagation.
//!
//! ## OWNERSHIP
//!
//! This module owns service implementation activation and lifecycle
//! coordination.
//!
//! It does not own:
//!
//! - Service descriptor definitions.
//! - Service registration identity allocation.
//! - Dependency graph construction.
//! - Service contract versions.
//! - Runtime lifecycle.
//! - Provider registration.
//! - Hardware discovery.
//! - Resource allocation.
//! - Backend selection.
//! - Domain-specific execution.
//! - Compiler planning.
//! - Distributed membership.
//! - Hybrid execution planning.
//!
//! Those responsibilities remain with their canonical subsystems.
//!
//! ## AUTHORITATIVE DEPENDENCIES
//!
//! - zur::core::service_registry
//! - zur::core::lifecycle
//! - zur::core::context
//! - zur::contracts::service
//! - zur::config
//! - zur::ids
//! - zur::error
//! - stdlib::context
//!
//! ## ARCHITECTURAL INVARIANTS
//!
//! 1. Registration is not activation.
//! 2. Activation is not physical resource reservation.
//! 3. Initialization success is not execution success.
//! 4. Required dependencies must be ready before dependents.
//! 5. Optional dependencies may remain unavailable.
//! 6. Disabled services must never be activated.
//! 7. Initialization failures must propagate.
//! 8. Failed startup must attempt rollback.
//! 9. Shutdown must respect dependency ordering.
//! 10. External callbacks must not execute while holding state locks.
//! 11. Runtime state must not be global.
//! 12. Unsafe Rust is prohibited.
//! 13. No artificial service-count ceiling exists.
//!
//! ## SCALABILITY
//!
//! Collections grow according to actual requirements and available
//! allocation capacity.
//!
//! No arbitrary limits are imposed on:
//!
//! - Services.
//! - Dependencies.
//! - Service versions.
//! - Sessions.
//! - Computational domains.
//! - Providers.
//! - Hardware resources.
//!
//! Literal infinite physical computation or storage is not promised.
//!
//! ## GRAMMAR BOUNDARY
//!
//! Relevant source-level authorities include:
//!
//! - grammar/execution/runtime.g4
//! - grammar/execution/runtime-capabilities.g4
//! - grammar/core/hints.g4
//! - grammar/resources/requirements.g4
//! - grammar/networking/services.g4
//! - grammar/specification/execution-model.md
//!
//! These define source-level intent.
//!
//! They do not directly activate runtime services.
//!
//! Semantic analysis and runtime configuration must establish
//! the operational policy before this manager is invoked.
//!
//! ## CONCURRENCY
//!
//! Lifecycle mutations require exclusive mutable access to the manager.
//!
//! Service leases use shared synchronized accounting.
//!
//! External service callbacks execute without holding the accounting
//! lock.
//!
//! The manager does not create its own asynchronous executor.

use std::collections::{BTreeMap, BTreeSet};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use crate::zur::config::{
    RuntimeConfig,
    ServiceActivation,
    ServiceKey,
};

use crate::zur::contracts::service::{
    DependencyRequirement,
    ReadinessRequirement,
    RegisteredService,
};

use crate::zur::core::service_registry::{
    ResolvedServiceGraph,
    ServiceRegistry,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::ids::ServiceInstanceId;

use crate::stdlib::context::OperationContext;

// ============================================================================
// CONSTANTS
// ============================================================================

const COMPONENT: &str = "zur.core.service_manager";

// ============================================================================
// ASYNCHRONOUS SERVICE CALLBACK
// ============================================================================

/// An asynchronous ZUR service lifecycle operation.
///
/// The returned future must be Send so it can be driven by a
/// compatible multithreaded executor.
///
/// The manager does not provide an asynchronous executor.
pub type ServiceFuture<'a> =
    Pin<Box<dyn Future<Output = ZurResult<()>> + Send + 'a>>;

// ============================================================================
// SERVICE IMPLEMENTATION CONTRACT
// ============================================================================

/// Operational implementation of a registered ZUR service.
///
/// The service descriptor remains owned by the canonical service
/// registry. This trait supplies its actual implementation.
///
/// Implementations must:
///
/// - Be thread-safe.
/// - Honor cooperative cancellation.
/// - Return errors rather than fabricate success.
/// - Release owned resources during successful shutdown.
/// - Preserve their descriptor identity and version.
/// - Avoid retaining borrowed operation contexts beyond their lifetime.
pub trait ManagedRuntimeService: Send + Sync {

    /// Initializes the service.
    ///
    /// Dependencies must already be initialized and ready.
    ///
    /// # Errors
    ///
    /// Returns the actual initialization failure.
    fn initialize<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ServiceFuture<'a>;

    /// Establishes explicit operational readiness.
    ///
    /// This operation is called when the descriptor requires
    /// explicit readiness verification.
    ///
    /// A successful return establishes only the readiness contract
    /// implemented by this service.
    ///
    /// # Errors
    ///
    /// Returns a readiness-check failure.
    fn check_readiness<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ServiceFuture<'a>;

    /// Shuts down the service.
    ///
    /// Implementations must release service-owned resources.
    ///
    /// # Errors
    ///
    /// Returns the actual shutdown failure.
    fn shutdown<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ServiceFuture<'a>;
}

// ============================================================================
// SERVICE STATE
// ============================================================================

/// Observable operational state of a managed service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ManagedServiceState {

    /// Registered but not initialized.
    Registered,

    /// Initialization is in progress.
    Initializing,

    /// Initialization completed but readiness is not established.
    Initialized,

    /// The service is ready for use.
    Ready,

    /// Shutdown is in progress.
    Stopping,

    /// Shutdown completed successfully.
    Stopped,

    /// Initialization, readiness or shutdown failed.
    Failed,
}

// ============================================================================
// SERVICE ENTRY
// ============================================================================

/// One executable service implementation and its registration.
struct ServiceEntry {
    registered: RegisteredService,
    implementation: Arc<dyn ManagedRuntimeService>,
}

impl ServiceEntry {

    fn new(
        registered: RegisteredService,
        implementation: Arc<dyn ManagedRuntimeService>,
    ) -> Self {
        Self {
            registered,
            implementation,
        }
    }
}

// ============================================================================
// SERVICE ACCOUNTING
// ============================================================================

/// Mutable operational accounting shared with service leases.
#[derive(Debug, Default)]
struct ServiceAccounting {
    states: BTreeMap<ServiceInstanceId, ManagedServiceState>,

    lease_counts: BTreeMap<ServiceInstanceId, usize>,

    activation_order: Vec<ServiceInstanceId>,

    shutdown_started: bool,
}

impl ServiceAccounting {

    fn state(
        &self,
        identity: &ServiceInstanceId,
    ) -> Option<ManagedServiceState> {
        self.states.get(identity).copied()
    }

    fn lease_count(
        &self,
        identity: &ServiceInstanceId,
    ) -> usize {
        self.lease_counts
            .get(identity)
            .copied()
            .unwrap_or_default()
    }
}

// ============================================================================
// LOCK HELPERS
// ============================================================================

fn accounting_error(
    operation: &'static str,
) -> ZurError {

    ZurError::new(
        crate::zur::error::ErrorKind::Concurrency,
        "ZUR service accounting lock is poisoned",
    )
    .with_component(COMPONENT)
    .with_operation(operation)
}

fn lock_accounting(
    accounting: &Mutex<ServiceAccounting>,
    operation: &'static str,
) -> ZurResult<
    std::sync::MutexGuard<'_, ServiceAccounting>
> {

    accounting
        .lock()
        .map_err(|_| accounting_error(operation))
}

fn manager_error(
    kind: crate::zur::error::ErrorKind,
    code: &'static str,
    message: impl Into<String>,
    operation: &'static str,
) -> ZurError {

    ZurError::with_code(kind, code, message)
        .unwrap_or_else(|_| {
            ZurError::invalid_data(
                "unable to construct service-manager error",
            )
        })
        .with_component(COMPONENT)
        .with_operation(operation)
}

// ============================================================================
// SERVICE LEASE
// ============================================================================

/// An owned lease on an activated service dependency graph.
///
/// A lease keeps the activation reference alive until it is dropped.
///
/// Cloning a lease acquires another reference.
///
/// Dropping a lease releases its references.
///
/// Lease release does not automatically execute asynchronous shutdown.
/// Shutdown is explicitly coordinated by the service manager.
pub struct ServiceLease {

    root: ServiceInstanceId,

    services: Vec<ServiceInstanceId>,

    accounting: Arc<Mutex<ServiceAccounting>>,
}

impl ServiceLease {

    /// Returns the root service instance identity.
    #[must_use]
    pub fn root(&self) -> &ServiceInstanceId {
        &self.root
    }

    /// Returns all service instances retained by this lease.
    #[must_use]
    pub fn services(&self) -> &[ServiceInstanceId] {
        &self.services
    }

    /// Returns the number of service references held by this lease.
    #[must_use]
    pub fn len(&self) -> usize {
        self.services.len()
    }

    /// Returns whether the lease contains no services.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.services.is_empty()
    }

    /// Creates another independently releasable lease.
    ///
    /// # Errors
    ///
    /// Returns an error if accounting is unavailable or a service
    /// is no longer eligible for a lease.
    pub fn try_clone(&self) -> ZurResult<Self> {

        let mut accounting = lock_accounting(
            &self.accounting,
            "clone_lease",
        )?;

        for identity in &self.services {

            if accounting.state(identity)
                != Some(ManagedServiceState::Ready)
            {
                return Err(manager_error(
                    crate::zur::error::ErrorKind::Lifecycle,
                    "zur.service_manager.lease_unavailable",
                    "cannot clone a lease containing a non-ready service",
                    "clone_lease",
                ));
            }
        }

        for identity in &self.services {

            let count = accounting
                .lease_counts
                .entry(identity.clone())
                .or_default();

            *count = count.checked_add(1).ok_or_else(|| {
                manager_error(
                    crate::zur::error::ErrorKind::ResourceUnavailable,
                    "zur.service_manager.lease_overflow",
                    "service lease reference accounting overflow",
                    "clone_lease",
                )
            })?;
        }

        Ok(Self {
            root: self.root.clone(),
            services: self.services.clone(),
            accounting: Arc::clone(&self.accounting),
        })
    }
}

impl Drop for ServiceLease {

    fn drop(&mut self) {

        let Ok(mut accounting) = self.accounting.lock() else {
            return;
        };

        for identity in &self.services {

            if let Some(count) =
                accounting.lease_counts.get_mut(identity)
            {
                *count = count.saturating_sub(1);

                if *count == 0 {
                    accounting.lease_counts.remove(identity);
                }
            }
        }
    }
}

impl std::fmt::Debug for ServiceLease {

    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {

        formatter
            .debug_struct("ServiceLease")
            .field("root", &self.root)
            .field("services", &self.services)
            .finish()
    }
}

// ============================================================================
// SERVICE MANAGER
// ============================================================================

/// Coordinates the lifecycle of registered ZUR services.
///
/// The registry remains the authority for service descriptors
/// and dependency resolution.
///
/// The manager owns executable implementations and actual
/// lifecycle operations.
///
/// # Concurrency
///
/// Lifecycle operations require exclusive mutable access.
///
/// Independent leases can be shared between threads.
///
/// No accounting lock is held while external service callbacks run.
pub struct ServiceManager {

    registry: Arc<ServiceRegistry>,

    configuration: Arc<RuntimeConfig>,

    implementations: BTreeMap<
        ServiceInstanceId,
        ServiceEntry,
    >,

    accounting: Arc<Mutex<ServiceAccounting>>,
}

impl ServiceManager {

    /// Creates a service manager.
    ///
    /// Construction performs no service initialization.
    ///
    /// No hardware discovery, resource reservation, provider
    /// discovery or service activation takes place.
    #[must_use]
    pub fn new(
        registry: Arc<ServiceRegistry>,
        configuration: Arc<RuntimeConfig>,
    ) -> Self {

        Self {
            registry,
            configuration,
            implementations: BTreeMap::new(),
            accounting: Arc::new(
                Mutex::new(ServiceAccounting::default())
            ),
        }
    }

    // ------------------------------------------------------------------------
    // IMPLEMENTATION REGISTRATION
    // ------------------------------------------------------------------------

    /// Associates an implementation with an existing registration.
    ///
    /// Registration is not activation.
    ///
    /// # Errors
    ///
    /// Returns an error when the implementation is duplicated,
    /// the registry no longer contains the registration, or
    /// manager accounting is unavailable.
    pub fn register_implementation(
        &mut self,
        registered: RegisteredService,
        implementation: Arc<dyn ManagedRuntimeService>,
    ) -> ZurResult<()> {

        let identity = registered.instance_id().clone();

        let authoritative = self.registry.get(
            registered.identity(),
            registered.version(),
        )?;

        let Some(authoritative) = authoritative else {
            return Err(manager_error(
                crate::zur::error::ErrorKind::ServiceUnavailable,
                "zur.service_manager.registration_missing",
                "service registration does not exist in the authoritative registry",
                "register_implementation",
            ));
        };

        if authoritative.instance_id() != &identity {
            return Err(manager_error(
                crate::zur::error::ErrorKind::Compatibility,
                "zur.service_manager.registration_mismatch",
                "service instance identity does not match the authoritative registration",
                "register_implementation",
            ));
        }

        if self.implementations.contains_key(&identity) {
            return Err(manager_error(
                crate::zur::error::ErrorKind::InvalidInput,
                "zur.service_manager.duplicate_implementation",
                "an implementation is already registered for this service instance",
                "register_implementation",
            ));
        }

        let mut accounting = lock_accounting(
            &self.accounting,
            "register_implementation",
        )?;

        accounting.states.insert(
            identity.clone(),
            ManagedServiceState::Registered,
        );

        drop(accounting);

        self.implementations.insert(
            identity,
            ServiceEntry::new(
                registered,
                implementation,
            ),
        );

        Ok(())
    }

    // ------------------------------------------------------------------------
    // STATE INSPECTION
    // ------------------------------------------------------------------------

    /// Returns the operational state of a service instance.
    ///
    /// Registration alone does not imply readiness.
    pub fn state(
        &self,
        identity: &ServiceInstanceId,
    ) -> ZurResult<Option<ManagedServiceState>> {

        let accounting = lock_accounting(
            &self.accounting,
            "state",
        )?;

        Ok(accounting.state(identity))
    }

    /// Returns the active lease count for a service.
    pub fn lease_count(
        &self,
        identity: &ServiceInstanceId,
    ) -> ZurResult<usize> {

        let accounting = lock_accounting(
            &self.accounting,
            "lease_count",
        )?;

        Ok(accounting.lease_count(identity))
    }

    // ------------------------------------------------------------------------
    // ACTIVATION POLICY
    // ------------------------------------------------------------------------

    fn activation_policy(
        &self,
        service: &RegisteredService,
    ) -> ServiceActivation {

        self.configuration.service_policy(
            service.identity()
        )
    }

    fn validate_activation(
        &self,
        service: &RegisteredService,
        is_root: bool,
        requirement: DependencyRequirement,
    ) -> ZurResult<bool> {

        let policy = self.activation_policy(service);

        match policy {

            ServiceActivation::Disabled => {

                if is_root || requirement == DependencyRequirement::Required {

                    return Err(manager_error(
                        crate::zur::error::ErrorKind::ServiceUnavailable,
                        "zur.service_manager.service_disabled",
                        format!(
                            "required service `{}` is disabled",
                            service.identity()
                        ),
                        "activate",
                    ));
                }

                // An optional disabled dependency is omitted.
                Ok(false)
            }

            ServiceActivation::OnDemand |
            ServiceActivation::Eager => Ok(true),
        }
    }

    // ------------------------------------------------------------------------
    // ACTIVATION
    // ------------------------------------------------------------------------

    /// Activates a registered service and its dependencies.
    ///
    /// Dependencies are initialized before their dependents.
    ///
    /// Successful activation returns a lease retaining the complete
    /// activated dependency graph.
    ///
    /// Already-ready services are reused.
    ///
    /// Optional dependencies that are absent or explicitly disabled
    /// are not activated.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The root service is not registered.
    /// - A required dependency cannot be resolved.
    /// - A required implementation is unavailable.
    /// - A service is disabled.
    /// - Initialization fails.
    /// - Readiness verification fails.
    /// - Cancellation is observed.
    /// - Manager accounting fails.
    ///
    /// Newly initialized services are rolled back in reverse order
    /// when activation fails.
    pub async fn activate(
        &mut self,
        identity: &ServiceKey,
        version: &crate::zur::version::ServiceContractVersion,
        context: &OperationContext,
    ) -> ZurResult<ServiceLease> {

        context.check_cancelled().map_err(|error| {
            ZurError::new(
                crate::zur::error::ErrorKind::Cancelled,
                error.to_string(),
            )
            .with_component(COMPONENT)
            .with_operation("activate")
        })?;

        let graph: ResolvedServiceGraph =
            self.registry.resolve(identity, version)?;

        self.activate_graph(
            graph,
            context,
        ).await
    }

    async fn activate_graph(
        &mut self,
        graph: ResolvedServiceGraph,
        context: &OperationContext,
    ) -> ZurResult<ServiceLease> {

        let mut activated = Vec::new();

        let order = graph.initialization_order();

        let root_id = graph.root().instance_id().clone();

        for service in order {

            context.check_cancelled().map_err(|error| {
                ZurError::new(
                    crate::zur::error::ErrorKind::Cancelled,
                    error.to_string(),
                )
                .with_component(COMPONENT)
                .with_operation("activate")
            })?;

            let is_root = service.instance_id() == &root_id;

            let requirement = graph
                .root()
                .descriptor()
                .dependency(service.identity())
                .map(|dependency| dependency.requirement())
                .unwrap_or(DependencyRequirement::Required);

            if !self.validate_activation(
                service,
                is_root,
                requirement,
            )? {
                continue;
            }

            let instance_id = service.instance_id().clone();

            if !self.implementations.contains_key(&instance_id) {

                let optional = !is_root
                    && requirement == DependencyRequirement::Optional;

                if optional {
                    continue;
                }

                let failure = manager_error(
                    crate::zur::error::ErrorKind::ServiceUnavailable,
                    "zur.service_manager.implementation_missing",
                    format!(
                        "no executable implementation is registered for `{}`",
                        service.identity()
                    ),
                    "activate",
                );

                self.rollback(&activated, context).await;

                return Err(failure);
            }

            let current = self.state(&instance_id)?;

            if current == Some(ManagedServiceState::Ready) {
                continue;
            }

            if current == Some(ManagedServiceState::Failed)
                || current == Some(ManagedServiceState::Stopping)
                || current == Some(ManagedServiceState::Stopped)
            {
                let failure = manager_error(
                    crate::zur::error::ErrorKind::Lifecycle,
                    "zur.service_manager.invalid_state",
                    format!(
                        "service `{}` cannot be activated from its current state",
                        service.identity()
                    ),
                    "activate",
                );

                self.rollback(&activated, context).await;

                return Err(failure);
            }

            {
                let mut accounting = lock_accounting(
                    &self.accounting,
                    "begin_initialization",
                )?;

                accounting.states.insert(
                    instance_id.clone(),
                    ManagedServiceState::Initializing,
                );
            }

            let entry = self.implementations
                .get(&instance_id)
                .ok_or_else(|| {
                    manager_error(
                        crate::zur::error::ErrorKind::ServiceUnavailable,
                        "zur.service_manager.implementation_missing",
                        "service implementation disappeared during activation",
                        "activate",
                    )
                })?;

            if let Err(error) = entry
                .implementation
                .initialize(context)
                .await
            {
                self.mark_failed(&instance_id);

                self.rollback(&activated, context).await;

                return Err(error.with_component(COMPONENT));
            }

            {
                let mut accounting = lock_accounting(
                    &self.accounting,
                    "mark_initialized",
                )?;

                accounting.states.insert(
                    instance_id.clone(),
                    ManagedServiceState::Initialized,
                );
            }

            let readiness = service.descriptor()
                .lifecycle()
                .readiness();

            if readiness == ReadinessRequirement::Explicit {

                if let Err(error) = entry
                    .implementation
                    .check_readiness(context)
                    .await
                {
                    self.mark_failed(&instance_id);

                    self.rollback(&activated, context).await;

                    return Err(error.with_component(COMPONENT));
                }
            }

            {
                let mut accounting = lock_accounting(
                    &self.accounting,
                    "mark_ready",
                )?;

                accounting.states.insert(
                    instance_id.clone(),
                    ManagedServiceState::Ready,
                );

                accounting.activation_order.push(
                    instance_id.clone()
                );
            }

            activated.push(instance_id);
        }

        // The requested root must be ready.
        if self.state(&root_id)?
            != Some(ManagedServiceState::Ready)
        {
            self.rollback(&activated, context).await;

            return Err(manager_error(
                crate::zur::error::ErrorKind::ServiceUnavailable,
                "zur.service_manager.root_not_ready",
                "requested root service did not become ready",
                "activate",
            ));
        }

        // Acquire a lease for the activated dependency graph.
        let services: Vec<ServiceInstanceId> = order
            .iter()
            .filter(|service| {
                self.state(service.instance_id()).ok()
                    == Some(Some(ManagedServiceState::Ready))
            })
            .map(|service| service.instance_id().clone())
            .collect();

        {
            let mut accounting = lock_accounting(
                &self.accounting,
                "acquire_lease",
            )?;

            for service in &services {

                let count = accounting
                    .lease_counts
                    .entry(service.clone())
                    .or_default();

                *count = count.checked_add(1).ok_or_else(|| {
                    manager_error(
                        crate::zur::error::ErrorKind::ResourceUnavailable,
                        "zur.service_manager.lease_overflow",
                        "service lease reference accounting overflow",
                        "acquire_lease",
                    )
                })?;
            }
        }

        Ok(ServiceLease {
            root: root_id,
            services,
            accounting: Arc::clone(&self.accounting),
        })
    }

    // ------------------------------------------------------------------------
    // FAILURE MANAGEMENT
    // ------------------------------------------------------------------------

    fn mark_failed(
        &self,
        identity: &ServiceInstanceId,
    ) {

        if let Ok(mut accounting) = self.accounting.lock() {

            accounting.states.insert(
                identity.clone(),
                ManagedServiceState::Failed,
            );
        }
    }

    async fn rollback(
        &mut self,
        activated: &[ServiceInstanceId],
        context: &OperationContext,
    ) {

        for identity in activated.iter().rev() {

            let Some(entry) = self.implementations.get(identity) else {
                continue;
            };

            if let Ok(mut accounting) = self.accounting.lock() {

                accounting.states.insert(
                    identity.clone(),
                    ManagedServiceState::Stopping,
                );
            }

            let result = entry
                .implementation
                .shutdown(context)
                .await;

            if let Ok(mut accounting) = self.accounting.lock() {

                accounting.states.insert(
                    identity.clone(),
                    if result.is_ok() {
                        ManagedServiceState::Stopped
                    } else {
                        ManagedServiceState::Failed
                    },
                );
            }
        }
    }

    // ------------------------------------------------------------------------
    // EAGER INITIALIZATION
    // ------------------------------------------------------------------------

    /// Initializes all explicitly eager registered services.
    ///
    /// Services with default on-demand activation are not initialized.
    ///
    /// This operation is intended for runtime initialization.
    ///
    /// # Errors
    ///
    /// Returns the first initialization or readiness failure.
    pub async fn initialize_eager(
        &mut self,
        context: &OperationContext,
    ) -> ZurResult<()> {

        let eager: Vec<RegisteredService> = self
            .implementations
            .values()
            .filter(|entry| {
                self.activation_policy(&entry.registered)
                    == ServiceActivation::Eager
            })
            .map(|entry| entry.registered.clone())
            .collect();

        for service in eager {

            self.activate(
                service.identity(),
                service.version(),
                context,
            ).await?;
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // SHUTDOWN
    // ------------------------------------------------------------------------

    /// Shuts down all managed services.
    ///
    /// Shutdown follows reverse activation order so dependents
    /// are stopped before their dependencies.
    ///
    /// The operation is idempotent.
    ///
    /// Outstanding service leases prevent shutdown from claiming
    /// successful completion.
    ///
    /// # Errors
    ///
    /// Returns an error if active leases remain or a shutdown callback
    /// fails.
    pub async fn shutdown(
        &mut self,
        context: &OperationContext,
    ) -> ZurResult<()> {

        context.check_cancelled().map_err(|error| {
            ZurError::new(
                crate::zur::error::ErrorKind::Cancelled,
                error.to_string(),
            )
            .with_component(COMPONENT)
            .with_operation("shutdown")
        })?;

        {
            let mut accounting = lock_accounting(
                &self.accounting,
                "shutdown",
            )?;

            if accounting.shutdown_started
                && accounting.activation_order.is_empty()
            {
                return Ok(());
            }

            let active: Vec<_> = accounting
                .lease_counts
                .iter()
                .filter(|(_, count)| **count > 0)
                .map(|(identity, count)| {
                    (identity.clone(), *count)
                })
                .collect();

            if !active.is_empty() {
                return Err(manager_error(
                    crate::zur::error::ErrorKind::Lifecycle,
                    "zur.service_manager.active_leases",
                    format!(
                        "cannot shut down services while {} service lease entries remain active",
                        active.len()
                    ),
                    "shutdown",
                ));
            }

            accounting.shutdown_started = true;
        }

        let order = {
            let accounting = lock_accounting(
                &self.accounting,
                "shutdown",
            )?;

            accounting.activation_order.clone()
        };

        let mut failures = Vec::new();

        for identity in order.iter().rev() {

            let Some(entry) = self.implementations.get(identity) else {
                continue;
            };

            {
                let mut accounting = lock_accounting(
                    &self.accounting,
                    "begin_shutdown",
                )?;

                accounting.states.insert(
                    identity.clone(),
                    ManagedServiceState::Stopping,
                );
            }

            match entry
                .implementation
                .shutdown(context)
                .await
            {
                Ok(()) => {

                    let mut accounting = lock_accounting(
                        &self.accounting,
                        "complete_shutdown",
                    )?;

                    accounting.states.insert(
                        identity.clone(),
                        ManagedServiceState::Stopped,
                    );
                }

                Err(error) => {

                    self.mark_failed(identity);

                    failures.push(error.to_string());
                }
            }
        }

        {
            let mut accounting = lock_accounting(
                &self.accounting,
                "finalize_shutdown",
            )?;

            accounting.activation_order.clear();

            accounting.shutdown_started = true;
        }

        if !failures.is_empty() {

            return Err(manager_error(
                crate::zur::error::ErrorKind::Lifecycle,
                "zur.service_manager.shutdown_failed",
                format!(
                    "one or more services failed to shut down: {}",
                    failures.join("; ")
                ),
                "shutdown",
            ));
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // REGISTRY ACCESS
    // ------------------------------------------------------------------------

    /// Returns the authoritative service registry.
    ///
    /// This does not activate or initialize any service.
    #[must_use]
    pub fn registry(&self) -> &Arc<ServiceRegistry> {
        &self.registry
    }

    /// Returns whether any service implementations are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.implementations.is_empty()
    }

    /// Returns the number of registered implementations.
    #[must_use]
    pub fn implementation_count(&self) -> usize {
        self.implementations.len()
    }
}

// ============================================================================
// DEBUG
// ============================================================================

impl std::fmt::Debug for ServiceManager {

    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {

        formatter
            .debug_struct("ServiceManager")
            .field(
                "implementation_count",
                &self.implementations.len(),
            )
            .field(
                "registry",
                &"injected",
            )
            .finish()
    }
}
