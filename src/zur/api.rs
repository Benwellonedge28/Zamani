
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Public API
//!
//! Path: src/zur/api.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Provides the stable, developer-facing public API for the
//! Zamani Universal Runtime (ZUR).
//!
//! This module is a public API facade over the canonical ZUR
//! configuration, identity, policy, contract, context, lifecycle,
//! registry, service-management and runtime-composition modules.
//!
//! It does not introduce duplicate implementations of those systems.
//!
//! # PRIMARY OBJECTIVE
//!
//! Support the Zamani programming model:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//!
//! ZUR provides optional runtime-managed execution infrastructure
//! for compatible Zamani programs.
//!
//! Runtime-free AOT execution remains independent of this API.
//!
//! Runtime-enabled and hybrid execution may use ZUR when their
//! validated execution requirements need runtime services.
//!
//! # ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - The public ZUR API surface.
//! - Stable convenience re-exports.
//! - Public documentation and usage guidance.
//! - Public access to existing runtime composition types.
//!
//! This module does NOT own:
//!
//! - Runtime configuration implementation.
//! - Runtime error implementation.
//! - Runtime identity allocation implementation.
//! - Runtime version implementation.
//! - Runtime policy implementation.
//! - ABI implementation.
//! - Execution contracts.
//! - Provider contracts.
//! - Service contracts.
//! - Runtime context implementation.
//! - Runtime lifecycle implementation.
//! - Service registration.
//! - Service dependency resolution.
//! - Service activation.
//! - Service implementation.
//! - Artifact loading.
//! - Execution dispatch.
//! - Scheduling.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Classical computation.
//! - Quantum computation.
//! - HDL and hardware realization.
//! - AI and tensor computation.
//! - Hybrid execution planning.
//! - Distributed execution planning.
//!
//! Those responsibilities remain with their canonical owners.
//!
//! # FUNDAMENTAL ARCHITECTURAL BOUNDARIES
//!
//! ZUR must preserve the following distinctions:
//!
//! Requirement != Capability
//! Capability != Preference
//! Preference != Constraint
//! Constraint != Hint
//! Hint != Feasibility
//! Feasibility != Resource reservation
//! Resource reservation != Execution
//! Execution submission != Execution completion
//!
//! This API exposes operational runtime facilities.
//! It does not reinterpret source-language meaning.
//!
//! # SCALABILITY
//!
//! This module introduces no arbitrary maximum for:
//!
//! - Runtime instances.
//! - Service instances.
//! - Service dependencies.
//! - Execution sessions.
//! - Providers.
//! - Computational domains.
//! - Devices.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Memory.
//! - Threads.
//! - Distributed participants.
//! - Resource quantities.
//!
//! The underlying implementations remain subject to actual
//! allocation capacity, available resources, address-space
//! constraints and platform limitations.
//!
//! Literal infinite physical computation is not promised.
//!
//! # SAFETY
//!
//! This module uses safe Rust only.
//!
//! It introduces:
//!
//! - No unsafe blocks.
//! - No unsafe functions.
//! - No unsafe implementations.
//! - No global mutable state.
//! - No implicit runtime initialization.
//! - No fabricated resource availability.
//! - No fabricated execution success.
//! - No arbitrary hardware ceilings.
//!
//! # GRAMMAR INTEGRATION
//!
//! Relevant canonical grammar authorities include:
//!
//! - grammar/core/program.g4
//! - grammar/core/source-unit.g4
//! - grammar/execution/runtime.g4
//! - grammar/execution/runtime-capabilities.g4
//! - grammar/resources/requirements.g4
//! - grammar/core/hints.g4
//! - grammar/networking/services.g4
//! - grammar/specification/semantic-model.md
//! - grammar/specification/execution-model.md
//!
//! The grammar describes source-level intent.
//!
//! The frontend and semantic layer validate that intent and
//! translate it into canonical runtime configuration, policies,
//! service descriptors and execution contracts.
//!
//! This API receives those already-established runtime inputs.
//!
//! It must never parse grammar files or treat a source declaration
//! as automatic authorization to activate a runtime service.
//!
//! # INTEGRATION
//!
//! The public API composes the following existing modules:
//!
//! ```text
//!                   src/zur/api.rs
//!                          |
//!          +---------------+----------------+
//!          |               |                |
//!          v               v                v
//!       config           policy           error
//!          |               |                |
//!          +---------------+----------------+
//!                          |
//!                          v
//!                        core
//!                          |
//!          +---------------+----------------+
//!          |               |                |
//!          v               v                v
//!       context         lifecycle       service_registry
//!          |               |                |
//!          +---------------+----------------+
//!                          |
//!                          v
//!                    service_manager
//!                          |
//!                          v
//!                    RuntimeBuilder
//!                          |
//!                          v
//!                    RuntimeInstance
//! ```
//!
//! The API is also the public access point to the existing ABI,
//! execution, provider and service contract modules.
//!
//! It does not replace those contracts.
//!
//! # RUNTIME INITIALIZATION
//!
//! Constructing a RuntimeInstance does not initialize services.
//!
//! The lifecycle is:
//!
//! Created -> Configured -> Initializing -> Ready
//!
//! Ready -> Running -> Ready
//!
//! Ready -> Stopping -> Stopped
//!
//! Initialization or shutdown failures must be propagated.
//!
//! The authoritative lifecycle state machine is implemented by
//! zur::core::lifecycle.
//!
//! # RUNTIME INDEPENDENCE
//!
//! This API does not require every Zamani program to link ZUR.
//!
//! AOT-native and freestanding executables must remain capable
//! of running without a separate Zamani runtime installation.
//!
//! Runtime-enabled programs use ZUR only when required.
//!
//! Hybrid programs may combine direct execution with managed
//! runtime services and external execution providers.

use std::sync::Arc;

// ============================================================================
// CANONICAL RUNTIME CONFIGURATION
// ============================================================================

/// Runtime configuration and operational budget contracts.
///
/// These types are implemented in `zur::config`.
pub use crate::zur::config::{
    BudgetKey,
    BudgetLimit,
    ExecutionFallbackPolicy,
    JitPolicy,
    RuntimeConfig,
    ServiceActivation,
    ServiceKey,
};

// ============================================================================
// CANONICAL RUNTIME ERRORS
// ============================================================================

/// Canonical structured ZUR error.
///
/// This is an alias of the existing `ZurError`, not a second error type.
pub use crate::zur::error::ZurError as RuntimeError;

/// Canonical ZUR result type.
pub use crate::zur::error::ZurResult;

/// Compatibility result name for Runtime-oriented consumers.
pub use crate::zur::error::RuntimeResult;

/// Error classification.
pub use crate::zur::error::ErrorKind;

/// Structured error severity.
pub use crate::zur::error::Severity;

/// Retryability metadata.
pub use crate::zur::error::Retryability;

/// Recovery disposition metadata.
pub use crate::zur::error::RecoveryDisposition;

/// Structured error context.
pub use crate::zur::error::ErrorContext;

// ============================================================================
// CANONICAL RUNTIME IDENTITIES
// ============================================================================

/// Runtime identity namespace.
pub use crate::zur::ids::IdNamespace;

/// Runtime identity allocation errors.
pub use crate::zur::ids::IdAllocationError;

/// Runtime identity allocation interface.
pub use crate::zur::ids::IdAllocator;

/// Common runtime identity interface.
pub use crate::zur::ids::RuntimeId;

/// Identity parsing errors.
pub use crate::zur::ids::InvalidId;

/// Identity namespace validation errors.
pub use crate::zur::ids::InvalidNamespace;

/// Identity of one ZUR runtime instance.
pub use crate::zur::ids::RuntimeInstanceId;

/// Identity of one runtime-managed execution.
pub use crate::zur::ids::ExecutionId;

/// Identity of one runtime execution session.
pub use crate::zur::ids::SessionId;

/// Identity of one instantiated runtime service.
pub use crate::zur::ids::ServiceInstanceId;

/// Identity of one runtime resource lease.
pub use crate::zur::ids::ResourceLeaseId;

/// Identity of one ZUR operational action.
pub use crate::zur::ids::RuntimeOperationId;

/// Convenience allocator for runtime-instance identities.
///
/// This is a type alias over the canonical IdAllocator.
/// It does not introduce a separate allocation mechanism.
///
/// Identity uniqueness across independent allocators or processes
/// must be established by the caller's identity namespace and
/// persistence/distribution strategy.
pub type RuntimeIdentityAllocator =
    IdAllocator<RuntimeInstanceId>;

// ============================================================================
// CANONICAL RUNTIME VERSIONING
// ============================================================================

/// Canonical runtime ABI version.
pub use crate::zur::version::RuntimeAbiVersion;

/// Canonical runtime service contract version.
pub use crate::zur::version::ServiceContractVersion;

/// Canonical runtime version representation.
pub use crate::zur::version::Version;

/// Inclusive runtime version range.
pub use crate::zur::version::VersionRange;

/// ABI compatibility policy.
pub use crate::zur::version::RuntimeAbiCompatibilityPolicy;

/// ABI compatibility decision.
pub use crate::zur::version::CompatibilityDecision;

/// Compatibility classification.
pub use crate::zur::version::Compatibility;

// ============================================================================
// CANONICAL RUNTIME POLICIES
// ============================================================================

/// Operational policy for one ZUR runtime instance.
pub use crate::zur::policy::RuntimePolicy;

/// Execution strategy preference.
pub use crate::zur::policy::ExecutionStrategyPreference;

/// Retry policy.
pub use crate::zur::policy::RetryPolicy;

/// Recovery policy.
pub use crate::zur::policy::RecoveryPolicy;

/// Cancellation policy.
pub use crate::zur::policy::CancellationPolicy;

/// Resource exhaustion policy.
pub use crate::zur::policy::ResourceExhaustionPolicy;

/// Runtime security policy.
pub use crate::zur::policy::SecurityPolicy;

/// Runtime observability policy.
pub use crate::zur::policy::ObservabilityPolicy;

/// Runtime adaptation policy.
pub use crate::zur::policy::AdaptationPolicy;

// ============================================================================
// STANDARD-LIBRARY OPERATION CONTEXT
// ============================================================================

/// Existing target-independent operation context.
///
/// The standard library remains its authoritative owner.
///
/// This context provides cooperative cancellation and typed
/// extensions to runtime operations.
pub use crate::stdlib::context::OperationContext;

// ============================================================================
// RUNTIME CONTRACTS
// ============================================================================

/// Canonical runtime contract modules.
///
/// These remain owned by `zur::contracts`.
///
/// The API does not create competing ABI, execution, provider
/// or service contracts.
pub use crate::zur::contracts::{
    abi,
    execution,
    provider,
    service,
};

// ============================================================================
// PUBLIC SERVICE CONTRACTS
// ============================================================================

/// Runtime service identity.
pub use crate::zur::contracts::service::ServiceId;

/// Runtime service descriptor.
pub use crate::zur::contracts::service::ServiceDescriptor;

/// Runtime service dependency.
pub use crate::zur::contracts::service::ServiceDependency;

/// Runtime service dependency requirement.
pub use crate::zur::contracts::service::DependencyRequirement;

/// Runtime service lifecycle requirements.
pub use crate::zur::contracts::service::ServiceLifecycleRequirements;

/// Runtime service startup requirement.
pub use crate::zur::contracts::service::StartupRequirement;

/// Runtime service readiness requirement.
pub use crate::zur::contracts::service::ReadinessRequirement;

/// Runtime service shutdown requirement.
pub use crate::zur::contracts::service::ShutdownRequirement;

/// Registered runtime service descriptor.
pub use crate::zur::core::service_registry::RegisteredService;

// ============================================================================
// RUNTIME CORE
// ============================================================================

/// Core runtime modules.
///
/// Advanced consumers may use the canonical module paths directly.
pub use crate::zur::core::{
    builder,
    context,
    lifecycle,
    service_manager,
    service_registry,
};

/// Immutable runtime context.
pub use crate::zur::core::RuntimeContext;

/// Runtime context builder.
pub use crate::zur::core::RuntimeContextBuilder;

/// Runtime lifecycle coordinator.
pub use crate::zur::core::RuntimeLifecycle;

/// Runtime lifecycle state.
pub use crate::zur::core::RuntimeState;

/// Authoritative runtime service registry.
pub use crate::zur::core::ServiceRegistry;

/// Resolved service dependency graph.
pub use crate::zur::core::ResolvedServiceGraph;

/// Managed runtime service implementation contract.
pub use crate::zur::core::ManagedRuntimeService;

/// Asynchronous runtime service lifecycle future.
pub use crate::zur::core::ServiceFuture;

/// Managed service lifecycle state.
pub use crate::zur::core::ManagedServiceState;

/// Lease retaining activated runtime services.
pub use crate::zur::core::ServiceLease;

/// Runtime service manager.
pub use crate::zur::core::ServiceManager;

// ============================================================================
// RUNTIME COMPOSITION
// ============================================================================

/// Builder for one independently composed ZUR instance.
///
/// The builder requires an explicit runtime identity and an
/// authoritative service registry.
///
/// It does not automatically discover hardware, select providers,
/// allocate physical resources or initialize services.
pub use crate::zur::core::RuntimeBuilder;

/// One independently composed ZUR runtime instance.
///
/// Its lifecycle and service manager remain owned by the canonical
/// runtime core implementation.
pub use crate::zur::core::RuntimeInstance;

// ============================================================================
// PUBLIC API HELPERS
// ============================================================================

/// Constructs a runtime builder from an explicit identity and
/// an existing authoritative service registry.
///
/// This is a convenience wrapper around `RuntimeBuilder::new`.
///
/// It does not:
///
/// - Generate identities.
/// - Register services.
/// - Initialize services.
/// - Discover hardware.
/// - Select execution providers.
/// - Reserve physical resources.
///
/// # Arguments
///
/// * `runtime_id` - Caller-supplied runtime instance identity.
/// * `registry` - Existing authoritative runtime service registry.
///
/// # Returns
///
/// An independently configured runtime builder.
///
/// # Example
///
/// ```ignore
/// use std::sync::Arc;
///
/// use zamani_compiler::zur::api::{
///     IdNamespace,
///     RuntimeBuilder,
///     RuntimeInstanceId,
///     ServiceRegistry,
/// };
///
/// let namespace = IdNamespace::new(
///     "zamani.runtime.instance"
/// )?;
///
/// let runtime_id = RuntimeInstanceId::new(
///     namespace,
///     vec![1],
/// )?;
///
/// let registry = Arc::new(
///     ServiceRegistry::with_namespace(
///         "zamani.runtime.service.instance"
///     )?
/// );
///
/// let builder = RuntimeBuilder::new(
///     runtime_id,
///     registry,
/// );
///
/// let runtime = builder.build()?;
/// ```
#[must_use]
pub fn runtime_builder(
    runtime_id: RuntimeInstanceId,
    registry: Arc<ServiceRegistry>,
) -> RuntimeBuilder {
    RuntimeBuilder::new(runtime_id, registry)
}

// ============================================================================
// API TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_runtime_identity() -> RuntimeInstanceId {
        let namespace = IdNamespace::new(
            "zamani.runtime.instance",
        )
        .expect("valid runtime namespace");

        RuntimeInstanceId::new(
            namespace,
            vec![1],
        )
        .expect("valid runtime identity")
    }

    fn test_service_registry() -> Arc<ServiceRegistry> {
        Arc::new(
            ServiceRegistry::with_namespace(
                "zamani.runtime.service.instance",
            )
            .expect("valid service registry namespace"),
        )
    }

    #[test]
    fn public_api_constructs_runtime_builder() {
        let builder = runtime_builder(
            test_runtime_identity(),
            test_service_registry(),
        );

        let runtime = builder
            .build()
            .expect("runtime construction should succeed");

        assert_eq!(
            runtime.state(),
            RuntimeState::Configured,
        );

        assert!(!runtime.is_ready());
    }

    #[test]
    fn public_api_preserves_runtime_identity() {
        let identity = test_runtime_identity();

        let runtime = RuntimeBuilder::new(
            identity.clone(),
            test_service_registry(),
        )
        .build()
        .expect("runtime construction should succeed");

        assert_eq!(
            runtime.identity(),
            &identity,
        );
    }

    #[test]
    fn default_runtime_configuration_is_available() {
        let configuration = RuntimeConfig::default();

        assert_eq!(
            configuration,
            RuntimeConfig::new(),
        );
    }

    #[test]
    fn runtime_error_is_the_canonical_zur_error() {
        let error: RuntimeError = ZurError::configuration(
            "invalid runtime configuration",
        );

        assert_eq!(
            error.kind(),
            ErrorKind::Configuration,
        );
    }

    #[test]
    fn independent_runtime_instances_can_be_constructed() {
        let first = RuntimeBuilder::new(
            test_runtime_identity(),
            test_service_registry(),
        )
        .build()
        .expect("first runtime should be constructed");

        let second = RuntimeBuilder::new(
            test_runtime_identity(),
            test_service_registry(),
        )
        .build()
        .expect("second runtime should be constructed");

        // Identical identities are permitted by independently supplied
        // identity values, but they are not globally unique.
        // Production callers must allocate identities within their
        // appropriate uniqueness scope.

        assert_eq!(
            first.state(),
            RuntimeState::Configured,
        );

        assert_eq!(
            second.state(),
            RuntimeState::Configured,
        );
    }

    #[test]
    fn runtime_builder_does_not_initialize_services() {
        let runtime = RuntimeBuilder::new(
            test_runtime_identity(),
            test_service_registry(),
        )
        .build()
        .expect("runtime construction should succeed");

        assert_eq!(
            runtime.state(),
            RuntimeState::Configured,
        );

        assert!(!runtime.is_ready());
    }
}
