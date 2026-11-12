
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Service Activation Interface
//!
//! Path: `src/zur/services/activation.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! ## PURPOSE
//!
//! Defines the stable service-activation interface used by ZUR
//! components that need to activate, initialize, or shut down
//! registered runtime services.
//!
//! This module is an integration boundary.
//!
//! It does not implement a second service manager.
//!
//! Actual service lifecycle operations remain owned by:
//!
//!     crate::zur::core::service_manager
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - The service activation interface.
//! - The asynchronous activation future contract.
//! - The asynchronous eager-initialization future contract.
//! - The asynchronous shutdown future contract.
//! - The adapter connecting the interface to ServiceManager.
//!
//! This module does NOT own:
//!
//! - Service descriptors.
//! - Service identities.
//! - Service contract versions.
//! - Service registration.
//! - Dependency graph construction.
//! - Dependency resolution.
//! - Dependency cycle detection.
//! - Activation policy definitions.
//! - Service implementations.
//! - Service initialization callbacks.
//! - Readiness verification implementation.
//! - Service lease accounting.
//! - Startup rollback.
//! - Shutdown ordering.
//! - Runtime lifecycle.
//! - Provider discovery.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Backend selection.
//! - Domain-specific execution.
//!
//! Those responsibilities remain with their authoritative modules.
//!
//! ## ARCHITECTURAL INVARIANTS
//!
//! 1. Registration is not activation.
//! 2. Dependency resolution is not initialization.
//! 3. Initialization is not readiness.
//! 4. Readiness is not resource reservation.
//! 5. Resource reservation is not execution success.
//! 6. Required dependencies must be ready before dependents.
//! 7. Optional dependencies may remain unavailable.
//! 8. Disabled services must not be activated.
//! 9. Activation failures must remain observable.
//! 10. Cancellation must be propagated to the activation owner.
//! 11. Successful activation must return an owned service lease.
//! 12. Shutdown must not silently discard outstanding leases.
//! 13. No service lifecycle callback is executed by this adapter.
//! 14. No global mutable state is introduced.
//! 15. Unsafe Rust is prohibited.
//!
//! ## DEPENDENCY DIRECTION
//!
//!     contracts/service.rs
//!              |
//!              v
//!     core/service_registry.rs
//!              |
//!              v
//!     core/service_manager.rs
//!              |
//!              v
//!     services/activation.rs
//!              |
//!              v
//!     execution/, builder and other ZUR consumers
//!
//! The adapter depends on the service manager.
//! The service manager must not depend on this adapter.
//!
//! ## GRAMMAR BOUNDARY
//!
//! Relevant source-level authorities include:
//!
//! - grammar/execution/runtime.g4
//! - grammar/execution/runtime-capabilities.g4
//! - grammar/networking/services.g4
//! - grammar/resources/requirements.g4
//! - grammar/core/hints.g4
//! - grammar/specification/execution-model.md
//!
//! These grammars describe source-level declarations and intent.
//!
//! They do not directly activate services.
//!
//! The frontend and semantic layer must validate source declarations
//! and translate them into canonical runtime configuration,
//! service descriptors, policies, and execution requests.
//!
//! This module receives already-validated runtime identities and
//! contract versions.
//!
//! It must never parse grammar or interpret source text.
//!
//! ## SCALABILITY
//!
//! This interface introduces no artificial maximum on:
//!
//! - Services.
//! - Service dependencies.
//! - Service versions.
//! - Runtime instances.
//! - Activation requests.
//! - Execution sessions.
//! - Providers.
//! - Computational domains.
//! - Hardware resources.
//!
//! It introduces no fixed hardware inventory or closed domain enum.
//!
//! Actual limits are determined by the service manager,
//! implementation capabilities, available memory, and platform
//! resources.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No unsafe implementations.
//! No global mutable state.
//! No fabricated activation success.
//!
//! ## ERROR HANDLING
//!
//! This interface preserves the canonical ZurResult and ZurError.
//!
//! It does not suppress, replace, or silently recover from errors
//! returned by the authoritative service manager.

use std::future::Future;
use std::pin::Pin;

use crate::stdlib::context::OperationContext;

use crate::zur::config::ServiceKey;

use crate::zur::contracts::service::ServiceLease;

use crate::zur::core::service_manager::ServiceManager;

use crate::zur::error::ZurResult;

use crate::zur::version::ServiceContractVersion;

// ============================================================================
// ASYNCHRONOUS OPERATION CONTRACT
// ============================================================================

/// An asynchronous service-management operation.
///
/// The future is `Send` so that callers may drive it using a
/// compatible multithreaded asynchronous executor.
///
/// ZUR does not create an executor in this module.
///
/// The lifetime is tied to the borrowed activation controller
/// and operation context.
///
/// No `'static` requirement is imposed because service operations
/// may legitimately borrow their manager and context.
pub type ServiceOperationFuture<'a, T> =
    Pin<Box<dyn Future<Output = ZurResult<T>> + Send + 'a>>;

/// Future returned by a successful service activation request.
///
/// A successful result contains an owned lease on the activated
/// service and the dependencies retained by that activation.
pub type ActivationFuture<'a> =
    ServiceOperationFuture<'a, ServiceLease>;

// ============================================================================
// SERVICE ACTIVATION INTERFACE
// ============================================================================

/// Stable interface for runtime service activation and shutdown.
///
/// Implementations must delegate actual service lifecycle operations
/// to the authoritative service-management subsystem.
///
/// This interface deliberately does not expose service implementation
/// internals, dependency graph mutation, or physical resource selection.
///
/// # Concurrency
///
/// Implementations must document their lifecycle synchronization
/// requirements.
///
/// The caller must not concurrently mutate the same activation
/// controller unless its implementation explicitly supports it.
///
/// Implementations must not hold internal accounting locks while
/// awaiting external service callbacks.
///
/// # Cancellation
///
/// Implementations must honor the cancellation behavior of the
/// supplied operation context.
///
/// A cancellation request must not be reported as successful
/// activation unless activation actually completed and a valid
/// lease was produced.
pub trait ServiceActivationController {

    /// Activates a registered service and its required dependencies.
    ///
    /// The service registry remains authoritative for resolving
    /// the requested service version and dependency graph.
    ///
    /// The implementation must initialize dependencies before
    /// their dependents.
    ///
    /// Optional dependencies may remain unavailable when their
    /// contracts and activation policies permit that outcome.
    ///
    /// A successful result returns an owned service lease.
    ///
    /// The caller must retain the lease for as long as it needs
    /// the activated service graph to remain in use.
    ///
    /// Dropping the lease releases its accounting references.
    /// It does not itself execute asynchronous shutdown.
    ///
    /// # Errors
    ///
    /// Returns the original service-management error when:
    ///
    /// - The requested service is not registered.
    /// - Its requested version is unavailable.
    /// - A required dependency cannot be resolved.
    /// - A required implementation is unavailable.
    /// - Activation is disabled by policy.
    /// - Initialization fails.
    /// - Readiness verification fails.
    /// - Cancellation is observed.
    /// - Service accounting fails.
    fn activate<'a>(
        &'a mut self,
        identity: &'a ServiceKey,
        version: &'a ServiceContractVersion,
        context: &'a OperationContext,
    ) -> ActivationFuture<'a>;

    /// Initializes services explicitly configured for eager activation.
    ///
    /// Services configured for on-demand activation must not be
    /// initialized merely because this operation is called.
    ///
    /// The implementation must preserve dependency ordering,
    /// readiness verification, cancellation, and failure propagation.
    ///
    /// # Errors
    ///
    /// Returns the first activation or readiness error.
    fn initialize_eager<'a>(
        &'a mut self,
        context: &'a OperationContext,
    ) -> ServiceOperationFuture<'a, ()>;

    /// Shuts down managed services.
    ///
    /// Shutdown must respect reverse activation order so that
    /// dependents are stopped before their dependencies.
    ///
    /// Outstanding service leases must not be silently invalidated.
    ///
    /// Implementations must report incomplete or failed shutdown.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - Cancellation prevents shutdown from proceeding.
    /// - Active leases prevent safe shutdown.
    /// - A service shutdown callback fails.
    /// - Service accounting or lifecycle state is unavailable.
    fn shutdown<'a>(
        &'a mut self,
        context: &'a OperationContext,
    ) -> ServiceOperationFuture<'a, ()>;
}

// ============================================================================
// AUTHORITATIVE SERVICE MANAGER ADAPTER
// ============================================================================

/// Connects the stable activation interface to the authoritative
/// ZUR service manager.
///
/// This implementation intentionally contains no independent
/// activation algorithm.
///
/// All actual activation, dependency handling, readiness checking,
/// lease accounting, rollback, and shutdown remain in
/// `core::service_manager::ServiceManager`.
impl ServiceActivationController for ServiceManager {

    fn activate<'a>(
        &'a mut self,
        identity: &'a ServiceKey,
        version: &'a ServiceContractVersion,
        context: &'a OperationContext,
    ) -> ActivationFuture<'a> {

        Box::pin(
            ServiceManager::activate(
                self,
                identity,
                version,
                context,
            )
        )
    }

    fn initialize_eager<'a>(
        &'a mut self,
        context: &'a OperationContext,
    ) -> ServiceOperationFuture<'a, ()> {

        Box::pin(
            ServiceManager::initialize_eager(
                self,
                context,
            )
        )
    }

    fn shutdown<'a>(
        &'a mut self,
        context: &'a OperationContext,
    ) -> ServiceOperationFuture<'a, ()> {

        Box::pin(
            ServiceManager::shutdown(
                self,
                context,
            )
        )
    }
}

// ============================================================================
// CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    /// Ensures the authoritative service manager implements the
    /// public activation interface.
    fn assert_activation_controller<
        T: ServiceActivationController,
    >() {}

    #[test]
    fn service_manager_implements_activation_controller() {
        assert_activation_controller::<ServiceManager>();
    }

    #[test]
    fn activation_future_is_a_send_future() {

        fn assert_send<T: Send>() {}

        fn check_future_type<'a>() {
            assert_send::<ActivationFuture<'a>>();
        }

        check_future_type::<'static>();
    }

    #[test]
    fn service_operation_future_is_a_send_future() {

        fn assert_send<T: Send>() {}

        fn check_future_type<'a>() {
            assert_send::<ServiceOperationFuture<'a, ()>>();
        }

        check_future_type::<'static>();
    }
}
