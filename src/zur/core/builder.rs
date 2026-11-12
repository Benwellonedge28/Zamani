
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Runtime Builder
//!
//! Path: src/zur/core/builder.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides the explicit composition boundary for constructing one
//! Zamani Universal Runtime instance.
//!
//! The builder composes existing ZUR authorities:
//!
//! - RuntimeConfig.
//! - RuntimePolicy.
//! - RuntimeContext.
//! - RuntimeLifecycle.
//! - ServiceRegistry.
//! - ServiceManager.
//! - OperationContext.
//!
//! It creates an independently owned runtime instance without
//! introducing global mutable state.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - RuntimeBuilder.
//! - RuntimeInstance.
//! - Explicit runtime composition.
//! - Runtime configuration and policy injection.
//! - Runtime service implementation installation.
//! - Runtime initialization orchestration.
//! - Runtime shutdown orchestration.
//!
//! ## THIS MODULE DOES NOT OWN
//!
//! - Runtime configuration definitions.
//! - Runtime policy definitions.
//! - Runtime identity definitions.
//! - Service descriptor definitions.
//! - Service registration identity allocation.
//! - Dependency graph construction.
//! - Service implementation contracts.
//! - Service lifecycle implementation.
//! - Provider registration or discovery.
//! - Platform discovery.
//! - Hardware selection.
//! - Resource allocation or reservation.
//! - Compiler planning.
//! - Backend selection.
//! - Artifact loading.
//! - Execution scheduling.
//! - Classical, quantum, HDL, AI or hybrid semantics.
//! - Distributed execution planning.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ## ARCHITECTURAL INVARIANTS
//!
//! 1. Every runtime instance has an explicit identity.
//! 2. Every runtime instance receives an explicit service registry.
//! 3. Configuration and policy are validated before initialization.
//! 4. Service implementations must correspond to registered descriptors.
//! 5. Service registration does not imply activation.
//! 6. Only explicitly eager services are initialized during startup.
//! 7. On-demand services remain inactive until requested.
//! 8. Disabled services are never activated by the service manager.
//! 9. Initialization failures are propagated.
//! 10. Runtime readiness is acknowledged only after initialization succeeds.
//! 11. Shutdown is not reported successful before service cleanup succeeds.
//! 12. No external callback runs while a builder-owned lock is held.
//! 13. No global mutable state is introduced.
//! 14. No hardware or computational-domain ceiling is introduced.
//! 15. No unsafe Rust is used.
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
//! These grammars describe source-level intent.
//!
//! The frontend and semantic layer must translate validated declarations
//! into RuntimeConfig, RuntimePolicy, service descriptors and other
//! canonical runtime inputs.
//!
//! This builder does not parse source, reinterpret requirements,
//! establish feasibility, discover hardware or activate services
//! merely because a source declaration exists.
//!
//! ## SCALABILITY
//!
//! The builder introduces no arbitrary limits on:
//!
//! - Runtime instances.
//! - Services.
//! - Service dependencies.
//! - Service implementations.
//! - Computational domains.
//! - Providers.
//! - Devices.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Memory.
//! - Threads.
//! - Distributed participants.
//!
//! Collection growth is subject to actual allocation capacity and
//! available system resources.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## INTEGRATION
//!
//! RuntimeContext receives the registry and operation context.
//!
//! RuntimeLifecycle owns runtime-instance state transitions.
//!
//! ServiceRegistry owns service descriptors and dependency resolution.
//!
//! ServiceManager owns implementation registration, activation,
//! readiness, rollback and service shutdown.
//!
//! RuntimeBuilder coordinates these authorities without duplicating them.
//!
//! RuntimeInstance exposes the resulting composed runtime.

use std::fmt;
use std::sync::Arc;

use crate::stdlib::context::OperationContext;

use crate::zur::config::{
    RuntimeConfig,
    ServiceKey,
};

use crate::zur::contracts::service::RegisteredService;

use crate::zur::core::context::RuntimeContext;

use crate::zur::core::lifecycle::{
    RuntimeLifecycle,
    RuntimeState,
};

use crate::zur::core::service_manager::{
    ManagedRuntimeService,
    ServiceManager,
};

use crate::zur::core::service_registry::ServiceRegistry;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::ids::RuntimeInstanceId;

use crate::zur::policy::RuntimePolicy;

use crate::zur::version::ServiceContractVersion;

// =============================================================================
// COMPONENT IDENTITY
// =============================================================================

/// Stable component identity used in builder diagnostics.
const COMPONENT: &str = "zur.core.builder";

// =============================================================================
// BUILDER ERROR CONSTRUCTION
// =============================================================================

/// Constructs a structured builder error.
fn builder_error(
    kind: crate::zur::error::ErrorKind,
    code: &'static str,
    message: impl Into<String>,
    operation: &'static str,
) -> ZurError {
    ZurError::with_code(kind, code, message)
        .unwrap_or_else(|_| {
            ZurError::invalid_data(
                "unable to construct a ZUR builder error",
            )
        })
        .with_component(COMPONENT)
        .with_operation(operation)
}

// =============================================================================
// SERVICE IMPLEMENTATION REGISTRATION
// =============================================================================

/// A registered service descriptor and its executable implementation.
///
/// The descriptor remains owned by ServiceRegistry.
///
/// The implementation is installed into ServiceManager.
///
/// This type does not claim that the service is initialized or ready.
struct PendingService {
    registered: RegisteredService,
    implementation: Arc<dyn ManagedRuntimeService>,
}

impl PendingService {
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

// =============================================================================
// RUNTIME BUILDER
// =============================================================================

/// Explicitly composes one Zamani Universal Runtime instance.
///
/// A builder requires:
///
/// - A caller-supplied runtime identity.
/// - An existing authoritative service registry.
///
/// Requiring the registry explicitly avoids silently creating
/// independent service identities that may collide when registries
/// are combined.
///
/// Configuration and policy default to their canonical defaults.
///
/// No service is initialized during builder construction.
///
/// # Example
///
/// ```ignore
/// use std::sync::Arc;
///
/// use crate::zur::config::RuntimeConfig;
/// use crate::zur::core::builder::RuntimeBuilder;
/// use crate::zur::core::service_registry::ServiceRegistry;
/// use crate::zur::ids::{
///     IdNamespace,
///     RuntimeInstanceId,
/// };
/// use crate::zur::policy::RuntimePolicy;
///
/// let runtime_id = RuntimeInstanceId::new(
///     IdNamespace::new("zamani.runtime.instance")?,
///     vec![1],
/// )?;
///
/// let registry = Arc::new(
///     ServiceRegistry::with_namespace(
///         "zamani.runtime.service.instance",
///     )?,
/// );
///
/// let runtime = RuntimeBuilder::new(
///     runtime_id,
///     registry,
/// )
/// .with_configuration(RuntimeConfig::default())
/// .with_policy(RuntimePolicy::default())
/// .build()?;
/// ```
pub struct RuntimeBuilder {
    runtime_id: RuntimeInstanceId,

    registry: Arc<ServiceRegistry>,

    configuration: RuntimeConfig,

    policy: RuntimePolicy,

    operation_context: OperationContext,

    services: Vec<PendingService>,
}

impl RuntimeBuilder {
    /// Creates a builder for one runtime instance.
    ///
    /// The runtime identity and service registry are mandatory.
    ///
    /// The supplied registry remains the authoritative owner of
    /// service descriptors, versions, instance identities and
    /// dependency resolution.
    ///
    /// This constructor does not:
    ///
    /// - Discover hardware.
    /// - Select a platform.
    /// - Register services.
    /// - Initialize services.
    /// - Reserve resources.
    /// - Start execution.
    #[must_use]
    pub fn new(
        runtime_id: RuntimeInstanceId,
        registry: Arc<ServiceRegistry>,
    ) -> Self {
        Self {
            runtime_id,
            registry,
            configuration: RuntimeConfig::default(),
            policy: RuntimePolicy::default(),
            operation_context: OperationContext::default(),
            services: Vec::new(),
        }
    }

    // -------------------------------------------------------------------------
    // CONFIGURATION
    // -------------------------------------------------------------------------

    /// Replaces the runtime configuration.
    ///
    /// The configuration is validated during build.
    ///
    /// RuntimeConfig remains the authority for:
    ///
    /// - Operational budgets.
    /// - Service activation preferences.
    /// - JIT permission.
    /// - Execution fallback policy.
    ///
    /// Configuration does not establish physical resource availability.
    #[must_use]
    pub fn with_configuration(
        mut self,
        configuration: RuntimeConfig,
    ) -> Self {
        self.configuration = configuration;
        self
    }

    /// Replaces the runtime operational policy.
    ///
    /// RuntimePolicy remains the authority for:
    ///
    /// - Execution strategy preference.
    /// - Retry behavior.
    /// - Recovery behavior.
    /// - Cancellation behavior.
    /// - Resource exhaustion behavior.
    /// - Security enforcement policy.
    /// - Observability policy.
    /// - Adaptation policy.
    ///
    /// The policy is validated during build.
    #[must_use]
    pub fn with_policy(
        mut self,
        policy: RuntimePolicy,
    ) -> Self {
        self.policy = policy;
        self
    }

    // -------------------------------------------------------------------------
    // OPERATION CONTEXT
    // -------------------------------------------------------------------------

    /// Injects the existing standard-library operation context.
    ///
    /// This context owns cooperative cancellation and typed extensions.
    ///
    /// Platform, observability and other independently owned context
    /// extensions should be installed by their owning subsystems
    /// before the context is supplied here.
    ///
    /// The builder does not discover or select platform providers.
    #[must_use]
    pub fn with_operation_context(
        mut self,
        operation_context: OperationContext,
    ) -> Self {
        self.operation_context = operation_context;
        self
    }

    // -------------------------------------------------------------------------
    // SERVICE IMPLEMENTATIONS
    // -------------------------------------------------------------------------

    /// Installs an implementation for an existing registered service.
    ///
    /// The descriptor must already be registered in the supplied
    /// authoritative ServiceRegistry.
    ///
    /// The service manager verifies that the registration identity
    /// matches the registry before accepting the implementation.
    ///
    /// This operation does not initialize the service.
    ///
    /// Multiple implementations may be installed for different
    /// registered service versions or identities.
    ///
    /// Duplicate implementation registration is rejected during build.
    ///
    /// # Errors
    ///
    /// Returns an error if the builder cannot reserve storage for
    /// another pending service implementation.
    pub fn with_service(
        mut self,
        registered: RegisteredService,
        implementation: Arc<dyn ManagedRuntimeService>,
    ) -> ZurResult<Self> {
        self.services.try_reserve(1).map_err(|error| {
            builder_error(
                crate::zur::error::ErrorKind::ResourceUnavailable,
                "zur.builder.service_capacity",
                format!(
                    "unable to reserve storage for service implementation: {error}"
                ),
                "with_service",
            )
        })?;

        self.services.push(PendingService::new(
            registered,
            implementation,
        ));

        Ok(self)
    }

    // -------------------------------------------------------------------------
    // BUILD
    // -------------------------------------------------------------------------

    /// Constructs a configured runtime instance.
    ///
    /// The returned runtime is in the Configured lifecycle state.
    ///
    /// Construction performs:
    ///
    /// 1. Configuration validation.
    /// 2. Runtime policy validation.
    /// 3. Runtime context construction.
    /// 4. Service registry injection into the context.
    /// 5. Service manager construction.
    /// 6. Service implementation registration.
    /// 7. Runtime lifecycle configuration admission.
    ///
    /// Construction does not initialize services or mark the runtime
    /// Ready. Call RuntimeInstance::initialize to perform asynchronous
    /// initialization.
    ///
    /// This separation prevents construction from secretly starting
    /// services or performing asynchronous work.
    ///
    /// # Errors
    ///
    /// Returns the original structured configuration, context,
    /// registration or lifecycle error.
    pub fn build(self) -> ZurResult<RuntimeInstance> {
        // Validate before constructing dependent runtime components.
        self.configuration.validate()?;
        self.policy.validate()?;

        // Build the immutable context and inject the exact registry
        // instance used by the service manager.
        let context = RuntimeContext::builder(
            self.runtime_id.clone(),
        )
        .with_configuration(self.configuration.clone())
        .with_policy(self.policy.clone())
        .with_operation_context(self.operation_context)
        .with_service_registry(Arc::clone(&self.registry))
        .build()?;

        // The manager shares the same authoritative registry and
        // immutable configuration as the runtime context.
        let mut service_manager = ServiceManager::new(
            Arc::clone(&self.registry),
            Arc::new(self.configuration.clone()),
        );

        // Install all implementations before initialization begins.
        //
        // If any registration fails, the partially constructed manager
        // is dropped and no service has been initialized.
        for service in self.services {
            service_manager.register_implementation(
                service.registered,
                service.implementation,
            )?;
        }

        // Admit the same validated configuration and policy into
        // the authoritative runtime lifecycle.
        let mut lifecycle = RuntimeLifecycle::new(
            self.runtime_id,
        );

        lifecycle.configure(
            context.configuration(),
            context.runtime_policy(),
        )?;

        Ok(RuntimeInstance {
            context,
            lifecycle,
            registry: self.registry,
            service_manager,
        })
    }
}

// =============================================================================
// RUNTIME INSTANCE
// =============================================================================

/// One independently composed Zamani Universal Runtime instance.
///
/// RuntimeInstance owns the coordination boundary between:
///
/// - Immutable RuntimeContext.
/// - RuntimeLifecycle.
/// - Authoritative ServiceRegistry.
/// - Operational ServiceManager.
///
/// It does not own compiler artifacts, execution plans, backend
/// implementations, domain semantics, physical resources or
/// hardware providers.
///
/// Multiple runtime instances may coexist with different identities,
/// registries, configurations, policies and operation contexts.
///
/// # Initialization
///
/// A newly built instance is Configured, not Ready.
///
/// Call initialize to run explicitly eager service initialization
/// and complete the runtime lifecycle transition.
///
/// On-demand services remain inactive until the service manager
/// is asked to activate them.
///
/// # Shutdown
///
/// Shutdown requires a cleanup-capable OperationContext.
///
/// If the original execution context has been cancelled, callers
/// should supply an appropriate non-cancelled cleanup context.
pub struct RuntimeInstance {
    context: RuntimeContext,

    lifecycle: RuntimeLifecycle,

    registry: Arc<ServiceRegistry>,

    service_manager: ServiceManager,
}

impl RuntimeInstance {
    /// Returns the immutable runtime context.
    #[must_use]
    pub fn context(&self) -> &RuntimeContext {
        &self.context
    }

    /// Returns the runtime identity.
    #[must_use]
    pub fn identity(&self) -> &RuntimeInstanceId {
        self.context.runtime_id()
    }

    /// Returns the current runtime lifecycle state.
    #[must_use]
    pub const fn state(&self) -> RuntimeState {
        self.lifecycle.state()
    }

    /// Returns whether the runtime is ready to accept execution.
    ///
    /// This is a lifecycle-state observation only. It does not
    /// establish that a particular execution is feasible, authorized,
    /// compatible or adequately resourced.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.lifecycle.is_ready()
    }

    /// Returns the authoritative service registry.
    ///
    /// Registry access does not imply that a service is initialized,
    /// ready, available or authorized.
    #[must_use]
    pub fn service_registry(&self) -> &Arc<ServiceRegistry> {
        &self.registry
    }

    /// Returns the service manager.
    ///
    /// The service manager remains the authority for service
    /// implementation registration and lifecycle operations.
    #[must_use]
    pub fn service_manager(&self) -> &ServiceManager {
        &self.service_manager
    }

    /// Returns mutable access to the service manager.
    ///
    /// This allows callers to activate registered services using
    /// the service manager's validated API.
    ///
    /// Callers must not bypass the runtime lifecycle by activating
    /// services after shutdown has begun.
    pub fn service_manager_mut(&mut self) -> ZurResult<&mut ServiceManager> {
        if !self.lifecycle.is_ready() {
            return Err(builder_error(
                crate::zur::error::ErrorKind::Lifecycle,
                "zur.builder.runtime_not_ready",
                "runtime services cannot be activated through this instance before runtime readiness",
                "service_manager_mut",
            ));
        }

        Ok(&mut self.service_manager)
    }

    // -------------------------------------------------------------------------
    // INITIALIZATION
    // -------------------------------------------------------------------------

    /// Initializes the runtime and explicitly eager services.
    ///
    /// On-demand services are not initialized.
    ///
    /// The runtime transitions:
    ///
    /// Configured -> Initializing -> Ready
    ///
    /// The Ready transition occurs only after eager initialization
    /// completes successfully.
    ///
    /// # Errors
    ///
    /// Returns the original initialization or readiness error.
    ///
    /// On failure, the runtime is marked Failed. The caller retains
    /// the RuntimeInstance and may attempt cleanup using shutdown
    /// with a suitable cleanup context.
    pub async fn initialize(&mut self) -> ZurResult<()> {
        if self.lifecycle.state() != RuntimeState::Configured {
            return Err(builder_error(
                crate::zur::error::ErrorKind::Lifecycle,
                "zur.builder.invalid_initialization_state",
                format!(
                    "runtime initialization requires Configured state; current state is {}",
                    self.lifecycle.state()
                ),
                "initialize",
            ));
        }

        self.lifecycle.begin_initialization()?;

        let initialization = self
            .service_manager
            .initialize_eager(self.context.operation_context())
            .await;

        if let Err(error) = initialization {
            // A failed initialization must never be reported as Ready.
            //
            // Cleanup is deliberately not claimed here. The caller may
            // need to supply a fresh cleanup context if initialization
            // failed because the original context was cancelled.
            self.lifecycle.mark_failed()?;

            return Err(error.with_component(COMPONENT));
        }

        if let Err(error) = self.lifecycle.mark_ready() {
            self.lifecycle.mark_failed()?;
            return Err(error.with_component(COMPONENT));
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // SERVICE ACTIVATION
    // -------------------------------------------------------------------------

    /// Activates a registered service through the canonical service manager.
    ///
    /// The service manager resolves and validates the dependency graph,
    /// applies activation policies, initializes required services,
    /// performs readiness checks and returns a service lease.
    ///
    /// The builder does not independently resolve dependencies or
    /// perform service lifecycle callbacks.
    ///
    /// # Errors
    ///
    /// Returns errors for:
    ///
    /// - Runtime not ready.
    /// - Cancellation.
    /// - Missing service registration.
    /// - Missing implementation.
    /// - Disabled required service.
    /// - Dependency or version incompatibility.
    /// - Initialization or readiness failure.
    /// - Resource or accounting failure.
    pub async fn activate_service(
        &mut self,
        identity: &ServiceKey,
        version: &ServiceContractVersion,
    ) -> ZurResult<crate::zur::core::service_manager::ServiceLease> {
        if !self.lifecycle.is_ready() {
            return Err(builder_error(
                crate::zur::error::ErrorKind::Lifecycle,
                "zur.builder.runtime_not_ready",
                "cannot activate a runtime service while the runtime is not ready",
                "activate_service",
            ));
        }

        self.service_manager
            .activate(
                identity,
                version,
                self.context.operation_context(),
            )
            .await
    }

    // -------------------------------------------------------------------------
    // SHUTDOWN
    // -------------------------------------------------------------------------

    /// Shuts down the runtime and its managed services.
    ///
    /// The supplied operation context is used for service shutdown.
    ///
    /// Callers should provide a cleanup context that has not already
    /// been cancelled when cleanup must proceed after an earlier
    /// cancellation.
    ///
    /// The runtime is marked Stopped only after ServiceManager
    /// confirms successful shutdown.
    ///
    /// # Errors
    ///
    /// Returns an error if service leases remain active, a shutdown
    /// callback fails, cancellation is observed, or lifecycle
    /// transition fails.
    ///
    /// A failed shutdown is not reported as successful cleanup.
    pub async fn shutdown(
        &mut self,
        cleanup_context: &OperationContext,
    ) -> ZurResult<()> {
        if self.lifecycle.state() == RuntimeState::Stopped {
            return Ok(());
        }

        self.lifecycle.begin_shutdown()?;

        if let Err(error) = self
            .service_manager
            .shutdown(cleanup_context)
            .await
        {
            self.lifecycle.mark_failed()?;

            return Err(error.with_component(COMPONENT));
        }

        self.lifecycle.mark_stopped()
    }
}

impl fmt::Debug for RuntimeBuilder {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("RuntimeBuilder")
            .field("runtime_id", &self.runtime_id)
            .field("configuration", &self.configuration)
            .field("policy", &self.policy)
            .field("registered_service_implementations", &self.services.len())
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for RuntimeInstance {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("RuntimeInstance")
            .field("runtime_id", self.context.runtime_id())
            .field("state", &self.lifecycle.state())
            .field(
                "service_registry",
                &"injected",
            )
            .finish_non_exhaustive()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::ids::IdNamespace;

    fn runtime_id() -> RuntimeInstanceId {
        RuntimeInstanceId::new(
            IdNamespace::new("zamani.runtime.instance")
                .expect("valid runtime identity namespace"),
            vec![1],
        )
        .expect("valid runtime identity")
    }

    fn service_registry() -> Arc<ServiceRegistry> {
        Arc::new(
            ServiceRegistry::with_namespace(
                "zamani.runtime.service.instance",
            )
            .expect("valid service identity namespace"),
        )
    }

    #[test]
    fn builder_constructs_a_configured_runtime() {
        let runtime = RuntimeBuilder::new(
            runtime_id(),
            service_registry(),
        )
        .build()
        .expect("default runtime composition should succeed");

        assert_eq!(runtime.state(), RuntimeState::Configured);
        assert!(!runtime.is_ready());
    }

    #[test]
    fn builder_preserves_runtime_identity() {
        let identity = runtime_id();

        let runtime = RuntimeBuilder::new(
            identity.clone(),
            service_registry(),
        )
        .build()
        .expect("runtime composition should succeed");

        assert_eq!(runtime.identity(), &identity);
    }

    #[test]
    fn context_receives_the_same_service_registry() {
        let registry = service_registry();

        let runtime = RuntimeBuilder::new(
            runtime_id(),
            Arc::clone(&registry),
        )
        .build()
        .expect("runtime composition should succeed");

        let context_registry = runtime
            .context()
            .service_registry::<ServiceRegistry>()
            .expect("context should contain the injected registry");

        assert!(Arc::ptr_eq(
            &registry,
            &context_registry,
        ));

        assert!(Arc::ptr_eq(
            &registry,
            runtime.service_registry(),
        ));
    }

    #[test]
    fn default_configuration_does_not_enable_jit() {
        let runtime = RuntimeBuilder::new(
            runtime_id(),
            service_registry(),
        )
        .build()
        .expect("runtime composition should succeed");

        assert!(
            !runtime.context().effective_policy().jit_policy().is_allowed()
        );
    }

    #[test]
    fn runtime_is_not_ready_before_initialization() {
        let runtime = RuntimeBuilder::new(
            runtime_id(),
            service_registry(),
        )
        .build()
        .expect("runtime composition should succeed");

        assert_eq!(runtime.state(), RuntimeState::Configured);
        assert!(!runtime.is_ready());
    }
}
