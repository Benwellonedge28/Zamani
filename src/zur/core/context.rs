
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Runtime Context
//!
//! Path: src/zur/core/context.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! PURPOSE
//!
//! Provides the immutable, explicitly injected operational context
//! shared by components of one Zamani Universal Runtime instance.
//!
//! The context connects:
//!
//! - Runtime identity.
//! - Runtime configuration.
//! - Effective operational policies.
//! - Standard-library operation context.
//! - Cooperative cancellation.
//! - Optional runtime service registry.
//! - Platform-specific context extensions.
//! - Observability context extensions.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - RuntimeContext.
//! - RuntimeContextBuilder.
//! - Explicit runtime-context composition.
//! - Runtime-instance identity association.
//! - Validated effective-policy association.
//! - Optional service-registry instance injection.
//! - Context-level access to operation services.
//!
//! EXISTING AUTHORITIES
//!
//! - config.rs owns RuntimeConfig.
//! - policy.rs owns RuntimePolicy and EffectiveRuntimePolicy.
//! - ids.rs owns RuntimeInstanceId.
//! - error.rs owns ZurError and ZurResult.
//! - stdlib/context.rs owns OperationContext and cancellation.
//! - contracts/service.rs owns service descriptors.
//! - contracts/provider.rs owns provider execution contracts.
//! - core/service_registry.rs will own service registration.
//! - platforms/ will own platform integrations.
//! - observability/ will own observability implementations.
//!
//! THIS MODULE DOES NOT OWN
//!
//! - Source-language semantics.
//! - AST or intermediate representations.
//! - Execution plans.
//! - Capability definitions.
//! - Requirement evaluation.
//! - Feasibility evaluation.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Service registration.
//! - Service lifecycle implementation.
//! - Provider selection.
//! - Scheduling.
//! - Execution.
//! - Global mutable state.
//!
//! SCALABILITY
//!
//! No arbitrary limits are imposed on:
//!
//! - Runtime instances.
//! - Context instances.
//! - Computational domains.
//! - Providers.
//! - Services.
//! - Devices.
//! - CPUs.
//! - GPUs.
//! - QPUs.
//! - Memory.
//! - Threads.
//! - Distributed participants.
//! - Context extensions.
//!
//! Storage grows subject to actual available resources and Rust's
//! representational constraints.
//!
//! This does not promise literal infinite physical computation.
//!
//! SAFETY
//!
//! Safe Rust only.
//!
//! No raw pointers.
//! No unsafe implementations.
//! No global mutable state.
//! No implicit hardware selection.
//! No fabricated resource availability.
//! No implicit service initialization.
//!
//! INTEGRATION
//!
//! RuntimeBuilder creates a RuntimeContext.
//!
//! RuntimeContext is then shared with:
//!
//! - Execution.
//! - Memory.
//! - Concurrency.
//! - Scheduling.
//! - Runtime services.
//! - Domain adapters.
//! - Hybrid coordination.
//! - Distributed integration.
//! - Security.
//! - Reliability.
//! - Observability.
//! - JIT.
//!
//! The context is immutable after construction.
//! Mutable operational state belongs to the subsystem that owns it.

use core::fmt;

use std::any::Any;
use std::sync::Arc;

use crate::stdlib::context::{
    CancellationSignal,
    OperationContext,
};

use crate::zur::config::RuntimeConfig;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::ids::RuntimeInstanceId;

use crate::zur::policy::{
    EffectiveRuntimePolicy,
    RuntimePolicy,
};

// ============================================================================
// RUNTIME CONTEXT
// ============================================================================

/// Immutable operational context for one ZUR runtime instance.
///
/// A RuntimeContext is an information and dependency-sharing boundary.
///
/// It does not execute programs, allocate physical resources,
/// initialize services, select providers or establish feasibility.
///
/// Cloning a context shares its immutable configuration and operational
/// dependencies through Arc.
///
/// Mutable execution state must remain owned by the appropriate
/// execution session, service, provider or runtime subsystem.
///
/// # Concurrency
///
/// RuntimeContext is safe to share between threads when its injected
/// dependencies satisfy the required Send + Sync contracts.
///
/// # Isolation
///
/// Different runtime instances may use different configurations,
/// policies, cancellation contexts, registries and platform extensions.
///
/// No process-wide runtime singleton is required.
#[derive(Clone)]
pub struct RuntimeContext {
    /// Identity of the runtime instance.
    runtime_id: RuntimeInstanceId,

    /// Validated combination of runtime policy and configuration.
    effective_policy: Arc<EffectiveRuntimePolicy>,

    /// Standard-library operation context.
    ///
    /// Owns cooperative cancellation observation and typed extensions.
    operation_context: OperationContext,

    /// Optional service-registry instance.
    ///
    /// The concrete registry implementation belongs to its own module.
    ///
    /// Type erasure prevents this context from depending on an
    /// implementation that has not yet been established.
    service_registry: Option<Arc<dyn Any + Send + Sync>>,
}

impl RuntimeContext {
    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    /// Creates a builder for a runtime context.
    ///
    /// A runtime identity must be supplied explicitly.
    ///
    /// Identity allocation belongs to the caller or the runtime
    /// composition layer. This method does not create a global
    /// identity allocator.
    #[must_use]
    pub fn builder(
        runtime_id: RuntimeInstanceId,
    ) -> RuntimeContextBuilder {
        RuntimeContextBuilder::new(runtime_id)
    }

    // ------------------------------------------------------------------------
    // Runtime identity
    // ------------------------------------------------------------------------

    /// Returns the identity of this runtime instance.
    #[must_use]
    pub fn runtime_id(&self) -> &RuntimeInstanceId {
        &self.runtime_id
    }

    // ------------------------------------------------------------------------
    // Configuration
    // ------------------------------------------------------------------------

    /// Returns the validated runtime configuration.
    ///
    /// RuntimeConfig remains the authoritative owner of:
    ///
    /// - Operational budgets.
    /// - Service activation preferences.
    /// - JIT permission.
    /// - Execution fallback configuration.
    ///
    /// Configuration does not establish physical resource availability.
    #[must_use]
    pub fn configuration(&self) -> &RuntimeConfig {
        self.effective_policy.configuration()
    }

    /// Returns the configured operational policy.
    #[must_use]
    pub fn runtime_policy(&self) -> &RuntimePolicy {
        self.effective_policy.runtime_policy()
    }

    /// Returns the validated combined runtime policy.
    ///
    /// This is a policy description, not proof of feasibility,
    /// resource reservation, service availability or authorization.
    #[must_use]
    pub fn effective_policy(&self) -> &EffectiveRuntimePolicy {
        &self.effective_policy
    }

    /// Returns a shared reference to the effective policy.
    ///
    /// Cloning the Arc does not clone the underlying policy.
    #[must_use]
    pub fn shared_effective_policy(
        &self,
    ) -> Arc<EffectiveRuntimePolicy> {
        Arc::clone(&self.effective_policy)
    }

    // ------------------------------------------------------------------------
    // Standard-library operation context
    // ------------------------------------------------------------------------

    /// Returns the associated standard-library operation context.
    ///
    /// This context owns cooperative cancellation observation and
    /// typed operation extensions.
    #[must_use]
    pub fn operation_context(&self) -> &OperationContext {
        &self.operation_context
    }

    /// Returns whether cancellation has been requested.
    ///
    /// Cancellation is cooperative.
    ///
    /// This does not prove that a remote operation, device operation,
    /// operating-system process or physical computation has stopped.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.operation_context.is_cancelled()
    }

    /// Returns the shared cancellation signal.
    ///
    /// The signal is read-only.
    ///
    /// The runtime cancellation owner remains responsible for
    /// issuing cancellation requests.
    #[must_use]
    pub fn cancellation_signal(
        &self,
    ) -> Arc<dyn CancellationSignal> {
        self.operation_context.cancellation_signal()
    }

    /// Checks whether the runtime context has received cancellation.
    ///
    /// # Errors
    ///
    /// Returns a structured ZUR cancellation error when cancellation
    /// has been requested.
    ///
    /// This operation does not forcibly terminate any execution.
    pub fn check_cancelled(&self) -> ZurResult<()> {
        if !self.is_cancelled() {
            return Ok(());
        }

        let reason = self
            .operation_context
            .cancellation_reason()
            .unwrap_or_else(|| {
                "runtime operation was cancelled".to_owned()
            });

        Err(
            ZurError::cancelled(reason)
                .with_component("zur.core.context")
                .with_operation("check_cancelled")
                .with_execution(self.runtime_id.to_string()),
        )
    }

    /// Returns an operation-context extension by concrete type.
    ///
    /// This allows platform, observability, security and other
    /// independently owned contexts to be injected without
    /// introducing duplicate implementations.
    ///
    /// The extension must implement Any + Send + Sync.
    #[must_use]
    pub fn extension<T>(&self) -> Option<&T>
    where
        T: Any + Send + Sync,
    {
        self.operation_context.extension::<T>()
    }

    /// Returns a shared operation-context extension.
    ///
    /// The underlying value remains owned by the operation context.
    #[must_use]
    pub fn extension_arc<T>(&self) -> Option<Arc<T>>
    where
        T: Any + Send + Sync,
    {
        self.operation_context.extension_arc::<T>()
    }

    /// Returns the number of operation-context extensions.
    ///
    /// This is an actual collection count, not a configured
    /// architecture ceiling.
    #[must_use]
    pub fn extension_count(&self) -> usize {
        self.operation_context.extension_count()
    }

    // ------------------------------------------------------------------------
    // Service registry integration
    // ------------------------------------------------------------------------

    /// Returns whether a service-registry instance was injected.
    ///
    /// This does not establish that any particular service is
    /// registered, initialized, ready or available.
    #[must_use]
    pub fn has_service_registry(&self) -> bool {
        self.service_registry.is_some()
    }

    /// Retrieves the injected service registry by its concrete type.
    ///
    /// The registry implementation remains owned by its own subsystem.
    ///
    /// This method performs no registration, discovery, initialization,
    /// lookup or service activation.
    ///
    /// # Type safety
    ///
    /// The returned Arc is available only when the requested type
    /// exactly matches the registered concrete type.
    #[must_use]
    pub fn service_registry<T>(&self) -> Option<Arc<T>>
    where
        T: Any + Send + Sync,
    {
        self.service_registry
            .as_ref()
            .cloned()
            .and_then(|registry| {
                Arc::downcast::<T>(registry).ok()
            })
    }

    // ------------------------------------------------------------------------
    // Child operation contexts
    // ------------------------------------------------------------------------

    /// Creates a child operation context.
    ///
    /// The child inherits:
    ///
    /// - The runtime identity.
    /// - The effective runtime policy.
    /// - The cancellation signal.
    /// - The operation-context scope.
    /// - Existing typed operation extensions.
    ///
    /// The child receives its own operation name.
    ///
    /// The returned child context is an operation context, not
    /// a new runtime instance.
    ///
    /// # Errors
    ///
    /// Returns a ZUR input error when the child operation name
    /// is invalid.
    pub fn child_operation_context(
        &self,
        operation: impl Into<String>,
    ) -> ZurResult<OperationContext> {
        let builder = self
            .operation_context
            .child_builder()
            .with_operation(operation)
            .map_err(|error| {
                ZurError::invalid_input(error.to_string())
                    .with_component("zur.core.context")
                    .with_operation("child_operation_context")
            })?;

        Ok(builder.build())
    }

    // ------------------------------------------------------------------------
    // Validation
    // ------------------------------------------------------------------------

    /// Validates the context's configuration and effective policies.
    ///
    /// This method does not:
    ///
    /// - Discover hardware.
    /// - Resolve physical resources.
    /// - Initialize services.
    /// - Validate an execution artifact.
    /// - Determine target feasibility.
    /// - Establish provider availability.
    /// - Authorize execution.
    ///
    /// Those operations belong to their respective subsystems.
    pub fn validate(&self) -> ZurResult<()> {
        self.configuration().validate()?;

        self.runtime_policy().validate()?;

        Ok(())
    }
}

impl fmt::Debug for RuntimeContext {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("RuntimeContext")
            .field("runtime_id", &self.runtime_id)
            .field("effective_policy", &self.effective_policy)
            .field("operation_context", &self.operation_context)
            .field(
                "has_service_registry",
                &self.has_service_registry(),
            )
            .finish()
    }
}

// ============================================================================
// RUNTIME CONTEXT BUILDER
// ============================================================================

/// Builder for an immutable RuntimeContext.
///
/// Construction is explicit and deterministic.
///
/// The builder does not inspect the environment, discover hardware,
/// register services, initialize providers or allocate physical resources.
///
/// The runtime identity is mandatory.
///
/// Configuration and policy default to their respective canonical
/// defaults when not explicitly supplied.
pub struct RuntimeContextBuilder {
    runtime_id: RuntimeInstanceId,

    configuration: RuntimeConfig,

    policy: RuntimePolicy,

    operation_context: OperationContext,

    service_registry: Option<Arc<dyn Any + Send + Sync>>,
}

impl RuntimeContextBuilder {
    /// Creates a builder with a mandatory runtime identity.
    #[must_use]
    pub fn new(
        runtime_id: RuntimeInstanceId,
    ) -> Self {
        Self {
            runtime_id,

            configuration: RuntimeConfig::default(),

            policy: RuntimePolicy::default(),

            operation_context: OperationContext::default(),

            service_registry: None,
        }
    }

    /// Replaces the runtime configuration.
    ///
    /// The configuration is validated during build.
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
    /// The policy is validated during build.
    #[must_use]
    pub fn with_policy(
        mut self,
        policy: RuntimePolicy,
    ) -> Self {
        self.policy = policy;
        self
    }

    /// Injects an existing standard-library operation context.
    ///
    /// This allows the caller to provide cooperative cancellation,
    /// descriptive metadata and typed platform or observability
    /// extensions.
    ///
    /// The supplied context is shared rather than reconstructed.
    #[must_use]
    pub fn with_operation_context(
        mut self,
        context: OperationContext,
    ) -> Self {
        self.operation_context = context;
        self
    }

    /// Injects an existing service-registry instance.
    ///
    /// The concrete registry implementation belongs to the
    /// authoritative service-registry subsystem.
    ///
    /// This method does not register or initialize services.
    ///
    /// The registry is shared through Arc.
    #[must_use]
    pub fn with_service_registry<T>(
        mut self,
        registry: Arc<T>,
    ) -> Self
    where
        T: Any + Send + Sync,
    {
        self.service_registry = Some(registry);
        self
    }

    /// Removes an injected service-registry instance.
    ///
    /// This affects only the context being constructed.
    #[must_use]
    pub fn without_service_registry(
        mut self,
    ) -> Self {
        self.service_registry = None;
        self
    }

    /// Constructs the immutable runtime context.
    ///
    /// This operation:
    ///
    /// 1. Validates the runtime configuration.
    /// 2. Validates the runtime operational policy.
    /// 3. Constructs the effective combined policy.
    /// 4. Shares the validated policy through Arc.
    /// 5. Constructs the immutable runtime context.
    ///
    /// It does not initialize the runtime lifecycle or activate
    /// optional services.
    ///
    /// # Errors
    ///
    /// Returns a structured ZUR error when configuration or
    /// operational policy validation fails.
    pub fn build(self) -> ZurResult<RuntimeContext> {
        let effective_policy = EffectiveRuntimePolicy::new(
            self.policy,
            self.configuration,
        )?;

        let context = RuntimeContext {
            runtime_id: self.runtime_id,

            effective_policy: Arc::new(effective_policy),

            operation_context: self.operation_context,

            service_registry: self.service_registry,
        };

        context.validate()?;

        Ok(context)
    }
}

impl fmt::Debug for RuntimeContextBuilder {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("RuntimeContextBuilder")
            .field("runtime_id", &self.runtime_id)
            .field("configuration", &self.configuration)
            .field("policy", &self.policy)
            .field("operation_context", &self.operation_context)
            .field(
                "has_service_registry",
                &self.service_registry.is_some(),
            )
            .finish()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::ids::IdNamespace;

    use crate::stdlib::context::{
        CancellationSignal,
        OperationContext,
    };

    use std::sync::atomic::{
        AtomicBool,
        Ordering,
    };

    fn runtime_id() -> RuntimeInstanceId {
        RuntimeInstanceId::new(
            IdNamespace::new(
                "zamani.runtime.instance",
            )
            .expect("valid runtime identity namespace"),

            vec![1],
        )
        .expect("valid runtime identity")
    }

    fn context() -> RuntimeContext {
        RuntimeContext::builder(runtime_id())
            .build()
            .expect("default context should be valid")
    }

    // ------------------------------------------------------------------------
    // CONSTRUCTION
    // ------------------------------------------------------------------------

    #[test]
    fn builds_default_runtime_context() {
        let context = context();

        assert_eq!(
            context.runtime_id(),
            &runtime_id(),
        );

        assert!(!context.is_cancelled());

        assert!(!context.has_service_registry());
    }

    #[test]
    fn default_configuration_is_preserved() {
        let context = context();

        assert!(
            context.configuration().is_default_configuration()
        );
    }

    #[test]
    fn default_runtime_policy_is_preserved() {
        let context = context();

        assert!(
            context.runtime_policy().is_default_policy()
        );
    }

    #[test]
    fn effective_policy_is_available() {
        let context = context();

        assert!(
            context.effective_policy().jit_policy()
                == crate::zur::config::JitPolicy::Disabled
        );
    }

    // ------------------------------------------------------------------------
    // CONFIGURATION AND POLICY
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_custom_configuration() {
        let configuration = RuntimeConfig::new()
            .with_jit_policy(
                crate::zur::config::JitPolicy::Allow,
            );

        let context = RuntimeContext::builder(runtime_id())
            .with_configuration(configuration)
            .build()
            .expect("valid configuration");

        assert!(
            context.effective_policy().jit_policy()
                == crate::zur::config::JitPolicy::Allow
        );
    }

    #[test]
    fn accepts_custom_runtime_policy() {
        let policy = RuntimePolicy::new();

        let context = RuntimeContext::builder(runtime_id())
            .with_policy(policy)
            .build()
            .expect("valid runtime policy");

        assert!(context.validate().is_ok());
    }

    #[test]
    fn effective_policy_is_shared_between_clones() {
        let original = context();

        let cloned = original.clone();

        assert!(
            Arc::ptr_eq(
                &original.effective_policy,
                &cloned.effective_policy,
            )
        );
    }

    // ------------------------------------------------------------------------
    // OPERATION CONTEXT
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_existing_operation_context() {
        let operation_context = OperationContext::builder()
            .with_component("zamani.runtime")
            .expect("valid component")
            .with_operation("execute")
            .expect("valid operation")
            .build();

        let context = RuntimeContext::builder(runtime_id())
            .with_operation_context(operation_context)
            .build()
            .expect("valid context");

        assert_eq!(
            context.operation_context().component(),
            Some("zamani.runtime")
        );

        assert_eq!(
            context.operation_context().operation(),
            Some("execute")
        );
    }

    #[test]
    fn clones_share_operation_context() {
        let original = context();

        let cloned = original.clone();

        assert!(
            !cloned.is_cancelled()
        );

        assert_eq!(
            original.extension_count(),
            cloned.extension_count()
        );
    }

    // ------------------------------------------------------------------------
    // CANCELLATION
    // ------------------------------------------------------------------------

    struct TestCancellation {
        cancelled: AtomicBool,
    }

    impl CancellationSignal for TestCancellation {
        fn is_cancelled(&self) -> bool {
            self.cancelled.load(Ordering::Acquire)
        }
    }

    #[test]
    fn cancellation_is_observed() {
        let signal = Arc::new(TestCancellation {
            cancelled: AtomicBool::new(true),
        });

        let operation_context = OperationContext::builder()
            .with_cancellation(signal)
            .build();

        let context = RuntimeContext::builder(runtime_id())
            .with_operation_context(operation_context)
            .build()
            .expect("valid context");

        assert!(context.is_cancelled());

        assert!(context.check_cancelled().is_err());
    }

    #[test]
    fn non_cancelled_context_passes_cancellation_check() {
        let context = context();

        assert!(context.check_cancelled().is_ok());
    }

    #[test]
    fn cancellation_signal_is_shared() {
        let signal = Arc::new(TestCancellation {
            cancelled: AtomicBool::new(false),
        });

        let operation_context = OperationContext::builder()
            .with_cancellation(signal.clone())
            .build();

        let context = RuntimeContext::builder(runtime_id())
            .with_operation_context(operation_context)
            .build()
            .expect("valid context");

        signal.cancelled.store(true, Ordering::Release);

        assert!(context.is_cancelled());
    }

    // ------------------------------------------------------------------------
    // SERVICE REGISTRY
    // ------------------------------------------------------------------------

    #[derive(Debug, Default)]
    struct TestRegistry {
        marker: bool,
    }

    #[test]
    fn injects_service_registry() {
        let registry = Arc::new(TestRegistry {
            marker: true,
        });

        let context = RuntimeContext::builder(runtime_id())
            .with_service_registry(registry.clone())
            .build()
            .expect("valid context");

        assert!(context.has_service_registry());

        let retrieved = context
            .service_registry::<TestRegistry>()
            .expect("registry should be available");

        assert!(retrieved.marker);

        assert!(Arc::ptr_eq(&registry, &retrieved));
    }

    #[test]
    fn registry_lookup_rejects_wrong_concrete_type() {
        let context = RuntimeContext::builder(runtime_id())
            .with_service_registry(
                Arc::new(TestRegistry::default()),
            )
            .build()
            .expect("valid context");

        assert!(
            context.service_registry::<String>().is_none()
        );
    }

    #[test]
    fn can_explicitly_remove_service_registry() {
        let context = RuntimeContext::builder(runtime_id())
            .with_service_registry(
                Arc::new(TestRegistry::default()),
            )
            .without_service_registry()
            .build()
            .expect("valid context");

        assert!(!context.has_service_registry());
    }

    // ------------------------------------------------------------------------
    // CHILD CONTEXT
    // ------------------------------------------------------------------------

    #[test]
    fn creates_child_operation_context() {
        let operation_context = OperationContext::builder()
            .with_component("zamani.runtime")
            .expect("valid component")
            .with_operation("parent")
            .expect("valid operation")
            .build();

        let context = RuntimeContext::builder(runtime_id())
            .with_operation_context(operation_context)
            .build()
            .expect("valid context");

        let child = context
            .child_operation_context("child")
            .expect("valid child operation");

        assert_eq!(
            child.component(),
            Some("zamani.runtime")
        );

        assert_eq!(
            child.operation(),
            Some("child")
        );
    }

    #[test]
    fn rejects_invalid_child_operation_name() {
        let context = context();

        assert!(
            context.child_operation_context("").is_err()
        );

        assert!(
            context.child_operation_context("   ").is_err()
        );
    }

    // ------------------------------------------------------------------------
    // IMMUTABILITY AND VALIDATION
    // ------------------------------------------------------------------------

    #[test]
    fn context_validation_succeeds() {
        let context = context();

        assert!(context.validate().is_ok());
    }

    #[test]
    fn cloned_context_preserves_runtime_identity() {
        let original = context();

        let cloned = original.clone();

        assert_eq!(
            original.runtime_id(),
            cloned.runtime_id()
        );
    }

    #[test]
    fn context_is_independent_between_runtime_instances() {
        let first = RuntimeContext::builder(runtime_id())
            .build()
            .unwrap();

        let second_id = RuntimeInstanceId::new(
            IdNamespace::new(
                "zamani.runtime.instance",
            )
            .unwrap(),
            vec![2],
        )
        .unwrap();

        let second = RuntimeContext::builder(second_id)
            .build()
            .unwrap();

        assert_ne!(
            first.runtime_id(),
            second.runtime_id()
        );
    }

    #[test]
    fn debug_does_not_expose_registry_contents() {
        let context = RuntimeContext::builder(runtime_id())
            .with_service_registry(
                Arc::new(String::from("private-registry")),
            )
            .build()
            .unwrap();

        let debug = format!("{context:?}");

        assert!(debug.contains("has_service_registry"));

        assert!(!debug.contains("private-registry"));
    }
}
