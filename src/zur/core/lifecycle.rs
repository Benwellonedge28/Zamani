
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Runtime Lifecycle
//!
//! Path: `src/zur/core/lifecycle.rs`
//! Edition: Rust 2021
//! Minimum Rust: 1.97+
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines the authoritative lifecycle state machine for one ZUR runtime
//! instance.
//!
//! The lifecycle represents the operational state of the runtime itself.
//! It does not represent the lifecycle of an individual runtime service,
//! execution session, task, provider, device, or distributed participant.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Runtime lifecycle states.
//! - Runtime lifecycle transition validation.
//! - Runtime configuration admission into the lifecycle.
//! - Initialization state transitions.
//! - Runtime readiness state transitions.
//! - Aggregate execution activity state transitions.
//! - Runtime shutdown state transitions.
//! - Runtime failure state transitions.
//! - Runtime lifecycle identity association.
//!
//! This module does NOT own:
//!
//! - Runtime configuration definitions.
//! - Runtime policy definitions.
//! - Runtime identity definitions.
//! - Service descriptors.
//! - Service registration.
//! - Service discovery.
//! - Service initialization or implementation.
//! - Service dependency resolution.
//! - Service startup ordering.
//! - Service rollback.
//! - Service shutdown callbacks.
//! - Provider lifecycle.
//! - Execution session lifecycle.
//! - Task lifecycle.
//! - Physical resource allocation.
//! - Resource reservation or release.
//! - Hardware discovery.
//! - Hardware selection.
//! - Compiler planning.
//! - Domain-specific execution.
//! - Distributed membership or recovery.
//! - Hybrid execution planning.
//! - Logging, tracing or metrics implementation.
//!
//! These responsibilities remain with their canonical repository owners.
//!
//! ## EXISTING AUTHORITIES
//!
//! This module integrates with:
//!
//! - `zur::ids::RuntimeInstanceId`:
//!   Canonical runtime instance identity.
//!
//! - `zur::config::RuntimeConfig`:
//!   Canonical runtime configuration.
//!
//! - `zur::policy::RuntimePolicy`:
//!   Canonical runtime operational policy.
//!
//! - `zur::error::ZurError` and `ZurResult`:
//!   Canonical ZUR error infrastructure.
//!
//! - `zur::contracts::service`:
//!   Declarative service lifecycle requirements.
//!
//! - `stdlib::lifecycle`:
//!   Existing standard-library component lifecycle management.
//!
//! - `stdlib::context`:
//!   Existing operation context and cooperative cancellation.
//!
//! The lifecycle state machine does not duplicate these authorities.
//!
//! ## LIFECYCLE OWNERSHIP BOUNDARY
//!
//! Runtime lifecycle:
//!
//!     Created
//!       |
//!       v
//!     Configured
//!       |
//!       v
//!     Initializing
//!       |
//!       v
//!     Ready
//!       |
//!       v
//!     Running
//!       |
//!       v
//!     Ready
//!       |
//!       v
//!     Stopping
//!       |
//!       v
//!     Stopped
//!
//! A runtime may enter Failed from any non-stopped state.
//!
//! Failed runtimes may enter Stopping so that cleanup can be attempted.
//!
//! The state machine does not claim that initialization, execution,
//! cancellation or shutdown has physically completed merely because
//! an operation was requested.
//!
//! Each transition must be explicitly acknowledged by the component
//! responsible for performing that operation.
//!
//! ## SERVICE LIFECYCLE SEPARATION
//!
//! The following distinction is mandatory:
//!
//! Runtime lifecycle:
//!     This module.
//!
//! Service lifecycle requirements:
//!     `zur::contracts::service`.
//!
//! Service initialization and shutdown:
//!     `zur::core::service_manager`.
//!
//! Standard-library component lifecycle:
//!     `stdlib::lifecycle`.
//!
//! Provider lifecycle:
//!     `zur::contracts::provider` and the responsible provider manager.
//!
//! This module must not introduce another ManagedService trait,
//! service registry, provider registry, or dependency graph.
//!
//! ## CONCURRENCY
//!
//! Lifecycle transitions require exclusive mutable access to this
//! coordinator.
//!
//! This makes state transitions deterministic and prevents concurrent
//! callers from independently changing the same lifecycle state.
//!
//! A runtime instance that exposes lifecycle transitions concurrently
//! must place this coordinator behind its own synchronization boundary,
//! such as a Mutex.
//!
//! The synchronization owner must not hold a lock while awaiting service
//! initialization, service shutdown, provider operations, or other
//! asynchronous callbacks.
//!
//! This module deliberately does not introduce a global lock.
//!
//! ## EXECUTION ACTIVITY
//!
//! `Running` describes aggregate runtime execution activity.
//!
//! The lifecycle coordinator does not count individual tasks or sessions.
//!
//! The execution manager owns active-operation accounting.
//!
//! It may call `execution_quiesced` only after it has established that
//! the runtime has no remaining active execution covered by this state.
//!
//! This avoids introducing a second task/session accounting authority.
//!
//! ## FAILURE AND CLEANUP
//!
//! `mark_failed` records the lifecycle state only.
//!
//! The original causal `ZurError` must be propagated or retained by
//! the caller and the appropriate observability/reliability subsystem.
//!
//! A transition to Failed does not claim that resources have been
//! released or services have stopped.
//!
//! Cleanup must be coordinated by the service manager and reliability
//! subsystem before the runtime is marked Stopped.
//!
//! ## SCALABILITY
//!
//! This module introduces no arbitrary limits on:
//!
//! - Runtime instances.
//! - Computational domains.
//! - Providers.
//! - Services.
//! - Tasks.
//! - Threads.
//! - CPUs.
//! - GPUs.
//! - QPUs.
//! - Memory.
//! - Distributed participants.
//! - Hardware resources.
//!
//! It contains no fixed-size hardware inventory or domain enumeration.
//!
//! The lifecycle state machine has a finite number of states because
//! lifecycle phases are finite concepts, not because the runtime has
//! a fixed computational capacity.
//!
//! Actual execution remains limited by available physical and
//! implementation resources.
//!
//! Literal infinite computation is not promised.
//!
//! ## GRAMMAR BOUNDARY
//!
//! Relevant source-level declarations include:
//!
//! - `grammar/execution/runtime.g4`
//! - `grammar/execution/runtime-capabilities.g4`
//! - `grammar/core/hints.g4`
//! - `grammar/resources/requirements.g4`
//! - `grammar/specification/execution-model.md`
//!
//! These grammars describe source-level execution intent, runtime
//! configuration intent, capability requirements, hints and resources.
//!
//! They do not define this Rust state machine.
//!
//! Source-level declarations must be parsed and semantically validated
//! before an adapter constructs RuntimeConfig and RuntimePolicy.
//!
//! This module must not parse grammar, reinterpret requirements,
//! select hardware, or convert hints into mandatory lifecycle actions.
//!
//! ## DETERMINISM
//!
//! This module:
//!
//! - Does not inspect hardware.
//! - Does not access the network.
//! - Does not access the filesystem.
//! - Does not read environment variables.
//! - Does not read the clock.
//! - Does not generate random values.
//! - Does not initialize global state.
//! - Does not spawn threads.
//! - Does not execute callbacks.
//!
//! ## FREEZE CRITERIA
//!
//! This module is ready to freeze when:
//!
//! 1. `zur::core` exports this module.
//! 2. `zur::mod` exports the core module.
//! 3. Runtime construction validates RuntimeConfig and RuntimePolicy.
//! 4. The runtime builder owns initialization orchestration.
//! 5. The service manager owns service lifecycle operations.
//! 6. The execution manager owns active execution accounting.
//! 7. Shutdown waits for required cleanup before marking Stopped.
//! 8. Invalid transitions return structured errors.
//! 9. Lifecycle tests pass.
//! 10. Formatting, compilation, tests and Clippy pass.

use core::fmt;

use crate::zur::config::RuntimeConfig;
use crate::zur::error::{ZurError, ZurResult};
use crate::zur::ids::RuntimeInstanceId;
use crate::zur::policy::RuntimePolicy;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable identifier for this lifecycle implementation.
const COMPONENT: &str = "zur.core.lifecycle";

// ============================================================================
// RUNTIME STATE
// ============================================================================

/// Observable operational state of one ZUR runtime instance.
///
/// These states describe runtime lifecycle, not the state of an individual
/// service, provider, task, execution session, device, or computation.
///
/// The enum is non-exhaustive so future lifecycle states can be introduced
/// without making downstream exhaustive matches part of the stable API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RuntimeState {
    /// The runtime instance has been constructed.
    ///
    /// Configuration has not yet been admitted into the lifecycle.
    Created,

    /// Runtime configuration and operational policy have been validated.
    Configured,

    /// Runtime initialization is in progress.
    ///
    /// The runtime builder and service manager are responsible for
    /// performing actual initialization.
    Initializing,

    /// Runtime initialization has completed successfully.
    ///
    /// Required runtime services must have satisfied their readiness
    /// requirements before this state is entered.
    Ready,

    /// The runtime has one or more active executions.
    ///
    /// The execution manager owns the authoritative accounting of
    /// active executions.
    Running,

    /// Runtime shutdown and cleanup are in progress.
    Stopping,

    /// Runtime shutdown has completed according to the runtime's
    /// declared shutdown contract.
    Stopped,

    /// Runtime initialization, execution coordination, or shutdown
    /// has failed.
    ///
    /// A failed runtime is not considered ready or operational.
    Failed,
}

impl RuntimeState {
    /// Returns the stable textual representation of the state.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Configured => "configured",
            Self::Initializing => "initializing",
            Self::Ready => "ready",
            Self::Running => "running",
            Self::Stopping => "stopping",
            Self::Stopped => "stopped",
            Self::Failed => "failed",
        }
    }

    /// Returns whether the runtime is ready to accept execution.
    #[must_use]
    pub const fn is_ready(self) -> bool {
        matches!(self, Self::Ready | Self::Running)
    }

    /// Returns whether runtime execution is currently active.
    ///
    /// This is an aggregate lifecycle indication. It is not an active
    /// task or session count.
    #[must_use]
    pub const fn is_running(self) -> bool {
        matches!(self, Self::Running)
    }

    /// Returns whether runtime shutdown has completed.
    #[must_use]
    pub const fn is_stopped(self) -> bool {
        matches!(self, Self::Stopped)
    }

    /// Returns whether the runtime has failed.
    #[must_use]
    pub const fn is_failed(self) -> bool {
        matches!(self, Self::Failed)
    }

    /// Returns whether the runtime may accept new executions.
    ///
    /// This is a lifecycle-state check only. The execution dispatcher
    /// must still validate artifact compatibility, authorization,
    /// capabilities, requirements, constraints and resource availability.
    #[must_use]
    pub const fn accepts_execution(self) -> bool {
        matches!(self, Self::Ready | Self::Running)
    }
}

impl fmt::Display for RuntimeState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// LIFECYCLE ERROR CONSTRUCTION
// ============================================================================

/// Constructs a structured error for an invalid lifecycle transition.
fn invalid_transition(
    operation: &'static str,
    current: RuntimeState,
    requested: RuntimeState,
) -> ZurError {
    ZurError::invalid_input(format!(
        "invalid runtime lifecycle transition: {current} -> {requested}"
    ))
    .with_component(COMPONENT)
    .with_operation(operation)
}

/// Constructs a structured error for an operation that cannot be
/// performed in the current lifecycle state.
fn invalid_operation(
    operation: &'static str,
    current: RuntimeState,
    explanation: &str,
) -> ZurError {
    ZurError::invalid_input(format!(
        "runtime lifecycle operation `{operation}` is invalid while \
         runtime state is `{current}`: {explanation}"
    ))
    .with_component(COMPONENT)
    .with_operation(operation)
}

// ============================================================================
// RUNTIME LIFECYCLE
// ============================================================================

/// Authoritative lifecycle state coordinator for one ZUR runtime instance.
///
/// The coordinator is intentionally small and does not own runtime
/// configuration, service implementations, execution sessions, providers,
/// or resource allocations.
///
/// It is suitable for embedding inside a runtime instance or runtime
/// manager.
///
/// The type is not internally synchronized. A runtime instance that shares
/// it across threads must provide synchronization at its ownership
/// boundary.
///
/// Lifecycle transitions are explicit. No constructor, state query,
/// or transition performs hardware discovery or service initialization.
#[derive(Debug)]
pub struct RuntimeLifecycle {
    runtime_id: RuntimeInstanceId,
    state: RuntimeState,
}

impl RuntimeLifecycle {
    /// Creates a lifecycle coordinator for an existing runtime identity.
    ///
    /// The caller supplies the canonical identity. This constructor does
    /// not generate identities or claim global uniqueness.
    #[must_use]
    pub fn new(runtime_id: RuntimeInstanceId) -> Self {
        Self {
            runtime_id,
            state: RuntimeState::Created,
        }
    }

    /// Returns the identity of the runtime instance.
    #[must_use]
    pub fn runtime_id(&self) -> &RuntimeInstanceId {
        &self.runtime_id
    }

    /// Returns the current lifecycle state.
    #[must_use]
    pub const fn state(&self) -> RuntimeState {
        self.state
    }

    /// Returns whether the runtime is ready to accept execution.
    ///
    /// This does not establish that a particular execution is feasible
    /// or authorized.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        self.state.is_ready()
    }

    /// Returns whether the runtime is stopped.
    #[must_use]
    pub const fn is_stopped(&self) -> bool {
        self.state.is_stopped()
    }

    /// Returns whether the runtime has failed.
    #[must_use]
    pub const fn is_failed(&self) -> bool {
        self.state.is_failed()
    }

    // ------------------------------------------------------------------------
    // CONFIGURATION
    // ------------------------------------------------------------------------

    /// Validates and admits runtime configuration and operational policy.
    ///
    /// This operation validates the canonical configuration and policy
    /// objects. It does not inspect the host, select hardware, initialize
    /// services, reserve resources, or establish execution feasibility.
    ///
    /// The lifecycle changes from Created to Configured only after both
    /// validation operations succeed.
    ///
    /// If validation fails, the lifecycle remains Created.
    ///
    /// # Errors
    ///
    /// Returns the canonical configuration or policy validation error.
    /// Returns a lifecycle error if the runtime is not in Created state.
    pub fn configure(
        &mut self,
        configuration: &RuntimeConfig,
        policy: &RuntimePolicy,
    ) -> ZurResult<()> {
        if self.state != RuntimeState::Created {
            return Err(invalid_operation(
                "configure",
                self.state,
                "configuration can only be admitted before initialization",
            ));
        }

        configuration.validate()?;
        policy.validate()?;

        self.state = RuntimeState::Configured;

        Ok(())
    }

    // ------------------------------------------------------------------------
    // INITIALIZATION
    // ------------------------------------------------------------------------

    /// Marks runtime initialization as started.
    ///
    /// The runtime builder must arrange actual service initialization
    /// through the service manager before calling `mark_ready`.
    ///
    /// This method does not initialize services itself.
    ///
    /// # Errors
    ///
    /// Returns an error unless the runtime is Configured.
    pub fn begin_initialization(&mut self) -> ZurResult<()> {
        self.transition(
            "begin_initialization",
            RuntimeState::Configured,
            RuntimeState::Initializing,
        )
    }

    /// Marks runtime initialization as successfully completed.
    ///
    /// The caller must establish that:
    ///
    /// - Required runtime initialization has completed.
    /// - Mandatory service dependencies have been resolved.
    /// - Required services have completed their declared initialization.
    /// - Required readiness checks have succeeded.
    ///
    /// This method does not independently verify those external facts.
    ///
    /// # Errors
    ///
    /// Returns an error unless the runtime is Initializing.
    pub fn mark_ready(&mut self) -> ZurResult<()> {
        self.transition(
            "mark_ready",
            RuntimeState::Initializing,
            RuntimeState::Ready,
        )
    }

    /// Marks initialization or another runtime operation as failed.
    ///
    /// The original causal error must be propagated or retained by the
    /// caller. This state transition does not replace that error.
    ///
    /// A failed runtime must not be treated as ready.
    ///
    /// The runtime may subsequently enter Stopping so that cleanup
    /// can be attempted.
    ///
    /// Repeated calls while already Failed are idempotent.
    ///
    /// # Errors
    ///
    /// Returns an error if shutdown has already completed.
    pub fn mark_failed(&mut self) -> ZurResult<()> {
        match self.state {
            RuntimeState::Stopped => Err(invalid_operation(
                "mark_failed",
                self.state,
                "a stopped runtime cannot transition back to Failed",
            )),

            RuntimeState::Failed => Ok(()),

            _ => {
                self.state = RuntimeState::Failed;
                Ok(())
            }
        }
    }

    // ------------------------------------------------------------------------
    // EXECUTION ACTIVITY
    // ------------------------------------------------------------------------

    /// Marks the beginning of runtime-managed execution activity.
    ///
    /// This transition may be called for the first execution when the
    /// runtime is Ready. Further execution submissions while already
    /// Running leave the aggregate state as Running.
    ///
    /// The execution manager remains responsible for checking whether
    /// a specific execution is valid, authorized, compatible and feasible.
    ///
    /// It must also maintain authoritative active-operation accounting.
    ///
    /// # Errors
    ///
    /// Returns an error unless the runtime is Ready or Running.
    pub fn begin_execution(&mut self) -> ZurResult<()> {
        match self.state {
            RuntimeState::Ready => {
                self.state = RuntimeState::Running;
                Ok(())
            }

            RuntimeState::Running => Ok(()),

            current => Err(invalid_operation(
                "begin_execution",
                current,
                "runtime must be Ready or Running before accepting execution",
            )),
        }
    }

    /// Marks the runtime as quiescent after execution activity.
    ///
    /// The caller must establish that all execution activity covered by
    /// the runtime's aggregate Running state has completed or otherwise
    /// ceased.
    ///
    /// This method does not count tasks, sessions, providers or remote
    /// operations. Those counts belong to their respective managers.
    ///
    /// # Errors
    ///
    /// Returns an error unless the runtime is Running.
    pub fn execution_quiesced(&mut self) -> ZurResult<()> {
        self.transition(
            "execution_quiesced",
            RuntimeState::Running,
            RuntimeState::Ready,
        )
    }

    // ------------------------------------------------------------------------
    // SHUTDOWN
    // ------------------------------------------------------------------------

    /// Begins runtime shutdown.
    ///
    /// The runtime builder or service manager must coordinate:
    ///
    /// - Rejection of new execution submissions.
    /// - Cancellation or completion of active operations.
    /// - Service shutdown in dependency-safe order.
    /// - Provider session closure.
    /// - Resource cleanup.
    /// - Final observability and diagnostic reporting.
    ///
    /// This state transition does not perform those operations.
    ///
    /// The caller must ensure that any in-progress initialization or
    /// execution is being safely stopped before marking the runtime
    /// Stopped.
    ///
    /// Repeated calls while Stopping or Stopped are idempotent.
    pub fn begin_shutdown(&mut self) -> ZurResult<()> {
        match self.state {
            RuntimeState::Stopping | RuntimeState::Stopped => Ok(()),

            RuntimeState::Created
            | RuntimeState::Configured
            | RuntimeState::Initializing
            | RuntimeState::Ready
            | RuntimeState::Running
            | RuntimeState::Failed => {
                self.state = RuntimeState::Stopping;
                Ok(())
            }
        }
    }

    /// Marks runtime shutdown as successfully completed.
    ///
    /// The caller must establish that all required shutdown and cleanup
    /// operations have completed according to their authoritative
    /// service, provider and resource contracts.
    ///
    /// A request to stop is not sufficient evidence of completed shutdown.
    ///
    /// Repeated calls after Stopped are idempotent.
    ///
    /// # Errors
    ///
    /// Returns an error unless shutdown has entered Stopping.
    pub fn mark_stopped(&mut self) -> ZurResult<()> {
        match self.state {
            RuntimeState::Stopping => {
                self.state = RuntimeState::Stopped;
                Ok(())
            }

            RuntimeState::Stopped => Ok(()),

            current => Err(invalid_operation(
                "mark_stopped",
                current,
                "runtime cleanup must enter Stopping before completion",
            )),
        }
    }

    // ------------------------------------------------------------------------
    // INTERNAL TRANSITION VALIDATION
    // ------------------------------------------------------------------------

    /// Performs a checked transition from one expected state to another.
    fn transition(
        &mut self,
        operation: &'static str,
        expected: RuntimeState,
        requested: RuntimeState,
    ) -> ZurResult<()> {
        if self.state != expected {
            return Err(invalid_transition(
                operation,
                self.state,
                requested,
            ));
        }

        self.state = requested;

        Ok(())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::ids::IdNamespace;

    fn runtime_id() -> RuntimeInstanceId {
        RuntimeInstanceId::new(
            IdNamespace::new("zamani.runtime.instance")
                .expect("valid test namespace"),
            vec![1],
        )
        .expect("valid test runtime identity")
    }

    fn configured_lifecycle() -> RuntimeLifecycle {
        let mut lifecycle = RuntimeLifecycle::new(runtime_id());

        lifecycle
            .configure(
                &RuntimeConfig::default(),
                &RuntimePolicy::default(),
            )
            .expect("default configuration and policy are valid");

        lifecycle
    }

    #[test]
    fn new_runtime_starts_in_created_state() {
        let lifecycle = RuntimeLifecycle::new(runtime_id());

        assert_eq!(lifecycle.state(), RuntimeState::Created);
        assert!(!lifecycle.is_ready());
        assert!(!lifecycle.is_stopped());
        assert!(!lifecycle.is_failed());
    }

    #[test]
    fn runtime_identity_is_preserved() {
        let id = runtime_id();
        let lifecycle = RuntimeLifecycle::new(id.clone());

        assert_eq!(lifecycle.runtime_id(), &id);
    }

    #[test]
    fn valid_configuration_moves_runtime_to_configured() {
        let mut lifecycle = RuntimeLifecycle::new(runtime_id());

        lifecycle
            .configure(
                &RuntimeConfig::default(),
                &RuntimePolicy::default(),
            )
            .expect("valid configuration");

        assert_eq!(lifecycle.state(), RuntimeState::Configured);
    }

    #[test]
    fn configuration_cannot_be_applied_twice() {
        let mut lifecycle = configured_lifecycle();

        let result = lifecycle.configure(
            &RuntimeConfig::default(),
            &RuntimePolicy::default(),
        );

        assert!(result.is_err());
        assert_eq!(lifecycle.state(), RuntimeState::Configured);
    }

    #[test]
    fn initialization_requires_configuration() {
        let mut lifecycle = RuntimeLifecycle::new(runtime_id());

        assert!(lifecycle.begin_initialization().is_err());
        assert_eq!(lifecycle.state(), RuntimeState::Created);
    }

    #[test]
    fn initialization_can_reach_ready() {
        let mut lifecycle = configured_lifecycle();

        lifecycle
            .begin_initialization()
            .expect("initialization begins");

        assert_eq!(lifecycle.state(), RuntimeState::Initializing);

        lifecycle
            .mark_ready()
            .expect("initialization completes");

        assert_eq!(lifecycle.state(), RuntimeState::Ready);
        assert!(lifecycle.is_ready());
    }

    #[test]
    fn runtime_execution_transitions_are_valid() {
        let mut lifecycle = configured_lifecycle();

        lifecycle.begin_initialization().unwrap();
        lifecycle.mark_ready().unwrap();

        lifecycle.begin_execution().unwrap();

        assert_eq!(lifecycle.state(), RuntimeState::Running);
        assert!(lifecycle.is_ready());

        lifecycle.execution_quiesced().unwrap();

        assert_eq!(lifecycle.state(), RuntimeState::Ready);
    }

    #[test]
    fn additional_execution_does_not_duplicate_running_transition() {
        let mut lifecycle = configured_lifecycle();

        lifecycle.begin_initialization().unwrap();
        lifecycle.mark_ready().unwrap();
        lifecycle.begin_execution().unwrap();

        lifecycle
            .begin_execution()
            .expect("additional execution is accepted");

        assert_eq!(lifecycle.state(), RuntimeState::Running);
    }

    #[test]
    fn execution_cannot_start_before_readiness() {
        let mut lifecycle = configured_lifecycle();

        assert!(lifecycle.begin_execution().is_err());
        assert_eq!(lifecycle.state(), RuntimeState::Configured);
    }

    #[test]
    fn execution_cannot_start_after_shutdown_begins() {
        let mut lifecycle = configured_lifecycle();

        lifecycle.begin_shutdown().unwrap();

        assert!(lifecycle.begin_execution().is_err());
        assert_eq!(lifecycle.state(), RuntimeState::Stopping);
    }

    #[test]
    fn failed_initialization_can_be_cleaned_up() {
        let mut lifecycle = configured_lifecycle();

        lifecycle.begin_initialization().unwrap();
        lifecycle.mark_failed().unwrap();

        assert_eq!(lifecycle.state(), RuntimeState::Failed);
        assert!(!lifecycle.is_ready());

        lifecycle.begin_shutdown().unwrap();
        lifecycle.mark_stopped().unwrap();

        assert_eq!(lifecycle.state(), RuntimeState::Stopped);
    }

    #[test]
    fn failure_is_idempotent() {
        let mut lifecycle = configured_lifecycle();

        lifecycle.mark_failed().unwrap();
        lifecycle.mark_failed().unwrap();

        assert_eq!(lifecycle.state(), RuntimeState::Failed);
    }

    #[test]
    fn stopped_runtime_cannot_be_failed_again() {
        let mut lifecycle = configured_lifecycle();

        lifecycle.begin_shutdown().unwrap();
        lifecycle.mark_stopped().unwrap();

        assert!(lifecycle.mark_failed().is_err());
        assert_eq!(lifecycle.state(), RuntimeState::Stopped);
    }

    #[test]
    fn shutdown_is_idempotent() {
        let mut lifecycle = configured_lifecycle();

        lifecycle.begin_shutdown().unwrap();
        lifecycle.begin_shutdown().unwrap();

        assert_eq!(lifecycle.state(), RuntimeState::Stopping);

        lifecycle.mark_stopped().unwrap();
        lifecycle.mark_stopped().unwrap();

        assert_eq!(lifecycle.state(), RuntimeState::Stopped);
    }

    #[test]
    fn runtime_can_be_stopped_before_initialization() {
        let mut lifecycle = RuntimeLifecycle::new(runtime_id());

        lifecycle.begin_shutdown().unwrap();
        lifecycle.mark_stopped().unwrap();

        assert_eq!(lifecycle.state(), RuntimeState::Stopped);
    }

    #[test]
    fn runtime_cannot_be_marked_stopped_without_cleanup_phase() {
        let mut lifecycle = configured_lifecycle();

        assert!(lifecycle.mark_stopped().is_err());
        assert_eq!(lifecycle.state(), RuntimeState::Configured);
    }

    #[test]
    fn runtime_state_text_is_stable() {
        assert_eq!(RuntimeState::Created.as_str(), "created");
        assert_eq!(RuntimeState::Configured.as_str(), "configured");
        assert_eq!(RuntimeState::Initializing.as_str(), "initializing");
        assert_eq!(RuntimeState::Ready.as_str(), "ready");
        assert_eq!(RuntimeState::Running.as_str(), "running");
        assert_eq!(RuntimeState::Stopping.as_str(), "stopping");
        assert_eq!(RuntimeState::Stopped.as_str(), "stopped");
        assert_eq!(RuntimeState::Failed.as_str(), "failed");
    }
}
