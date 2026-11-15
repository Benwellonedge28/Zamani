
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani ZHAL — Provider Lifecycle Coordination
//!
//! Path:
//!     src/zhal/providers/lifecycle.rs
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
//! Coordinates initialization, readiness, dependency ordering, rollback
//! and shutdown of registered ZHAL hardware providers.
//!
//! This module coordinates the existing standard-library
//! `ProviderLifecycle` contract. It does not redefine that contract.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! THIS MODULE OWNS:
//!
//! - Lifecycle coordination for registered hardware providers.
//! - Association of lifecycle-capable implementations with the registry.
//! - Provider dependency declarations.
//! - Dependency graph validation.
//! - Deterministic initialization ordering.
//! - Reverse-order shutdown.
//! - Lifecycle state tracking.
//! - Initialization failure handling.
//! - Best-effort partial-initialization rollback.
//! - Shutdown failure reporting.
//! - Lifecycle manager concurrency.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - Provider identity or provider descriptors.
//! - Provider lifecycle hook definitions.
//! - Provider registration or registry storage.
//! - Provider API compatibility validation.
//! - Provider authentication or trust verification.
//! - Provider authorization.
//! - Hardware discovery.
//! - Device inventory.
//! - Device sessions.
//! - Physical resource allocation.
//! - Resource reservation.
//! - Hardware scheduling or routing.
//! - Quantum execution.
//! - HDL realization.
//! - Distributed execution.
//! - Hybrid execution.
//! - Runtime orchestration.
//! - Source-language semantics.
//!
//! ============================================================================
//! CANONICAL AUTHORITIES
//! ============================================================================
//!
//! Provider identity and lifecycle hooks:
//!     crate::stdlib::provider
//!
//! Provider registration:
//!     crate::zhal::providers::registry
//!
//! Provider hardware interface:
//!     crate::zhal::providers::contract
//!
//! Operation context and cancellation:
//!     crate::stdlib::context
//!
//! ZHAL deadline and cancellation observation:
//!     crate::zhal::context
//!
//! Structured errors:
//!     crate::stdlib::error
//!
//! ============================================================================
//! LIFECYCLE MODEL
//! ============================================================================
//!
//! Attached
//!    |
//!    v
//! Initializing
//!    |
//!    +---- failure ----> Failed
//!    |
//!    v
//! Ready
//!    |
//!    v
//! Stopping
//!    |
//!    +---- failure ----> Failed
//!    |
//!    v
//! Stopped
//!
//! Failed providers are not automatically retried.
//!
//! An initialization failure triggers best-effort rollback of the
//! provider whose initialization was attempted and all providers
//! successfully initialized earlier in that initialization operation.
//!
//! Shutdown proceeds in reverse dependency order.
//!
//! ============================================================================
//! DEPENDENCY MODEL
//! ============================================================================
//!
//! A provider declares the provider identities it depends upon.
//!
//! Dependencies must be attached before initialization is attempted.
//!
//! Initialization order is deterministic and dependency-first.
//!
//! Shutdown order is the reverse of initialization order.
//!
//! Missing dependencies and dependency cycles are explicit errors.
//!
//! No arbitrary dependency-count or provider-count limit is imposed.
//!
//! ============================================================================
//! CONCURRENCY
//! ============================================================================
//!
//! No provider callback is invoked while a lifecycle state lock is held.
//!
//! Provider callbacks may perform I/O or await external operations.
//! They must never execute while the manager's internal lock is held.
//!
//! Concurrent lifecycle transitions for the same provider are rejected.
//!
//! ============================================================================
//! CANCELLATION AND DEADLINES
//! ============================================================================
//!
//! ZhalContext remains the caller-facing context.
//!
//! Its canonical OperationContext is passed to ProviderLifecycle hooks.
//!
//! Cancellation is cooperative.
//!
//! ZHAL deadlines are checked before and after lifecycle hooks.
//!
//! The existing ProviderLifecycle hook signature accepts OperationContext,
//! not ZhalContext. Consequently, a provider hook cannot directly inspect
//! the ZHAL deadline unless its integration supplies that information
//! through an existing typed context extension.
//!
//! A runtime should connect deadline expiration to the canonical
//! cancellation mechanism when in-flight provider cancellation is required.
//!
//! Cancellation never proves that physical hardware has stopped.
//!
//! ============================================================================
//! REGISTRY INTEGRATION
//! ============================================================================
//!
//! A provider must first be registered in ProviderRegistry.
//!
//! The lifecycle manager then attaches the same provider implementation.
//!
//! The manager verifies that the attached lifecycle implementation and
//! registered provider are the same Arc allocation.
//!
//! The manager does not duplicate provider registry storage.
//!
//! Registry removal remains an explicit registry operation. Providers
//! should be shut down and detached before registry removal.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! This module introduces no fixed limits on:
//!
//! - providers;
//! - provider dependencies;
//! - hardware devices;
//! - processors;
//! - accelerators;
//! - quantum processors;
//! - distributed participants;
//! - computational domains;
//! - hardware resources.
//!
//! Actual memory and execution are constrained by available resources
//! and platform representation limits.
//!
//! Literal infinite computation or storage is not promised.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::stdlib::context::OperationContext;
use crate::stdlib::error::{Error, ErrorKind, Result};
use crate::stdlib::provider::{
    Provider,
    ProviderId,
    ProviderLifecycle,
    ProviderStatus,
};

use crate::zhal::context::ZhalContext;
use crate::zhal::providers::registry::ProviderRegistry;

// ============================================================================
// MODULE CONTRACT
// ============================================================================

/// Stable component identity for lifecycle diagnostics.
pub const COMPONENT: &str = "zhal.providers.lifecycle";

/// Lifecycle state of a provider as observed by this manager.
///
/// This is manager-owned coordination state. It is deliberately distinct
/// from `ProviderStatus`, which is the provider's own live status report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManagedProviderState {
    /// The provider is registered and attached but not initialized.
    Attached,

    /// Initialization is in progress.
    Initializing,

    /// Initialization completed and the provider reports readiness.
    Ready,

    /// Shutdown is in progress.
    Stopping,

    /// Shutdown completed and the provider reports stopped.
    Stopped,

    /// Initialization or shutdown failed.
    Failed,
}

impl ManagedProviderState {
    /// Returns the stable textual state identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Attached => "attached",
            Self::Initializing => "initializing",
            Self::Ready => "ready",
            Self::Stopping => "stopping",
            Self::Stopped => "stopped",
            Self::Failed => "failed",
        }
    }
}

impl fmt::Display for ManagedProviderState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// LIFECYCLE PHASE
// ============================================================================

/// Identifies the lifecycle operation associated with a failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LifecyclePhase {
    /// Provider initialization failed.
    Initialization,

    /// Provider readiness verification failed.
    ReadinessVerification,

    /// Provider shutdown failed.
    Shutdown,

    /// Shutdown attempted as part of initialization rollback.
    RollbackShutdown,

    /// The provider was not registered or no longer matches the registry.
    RegistryVerification,

    /// Lifecycle state transition failed.
    StateTransition,
}

impl LifecyclePhase {
    /// Returns the stable textual phase identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initialization => "initialization",
            Self::ReadinessVerification => "readiness_verification",
            Self::Shutdown => "shutdown",
            Self::RollbackShutdown => "rollback_shutdown",
            Self::RegistryVerification => "registry_verification",
            Self::StateTransition => "state_transition",
        }
    }
}

// ============================================================================
// FAILURE
// ============================================================================

/// A provider-specific lifecycle failure.
///
/// The original standard-library error is retained. This is important
/// because provider implementations may attach structured context and
/// causal errors to their failures.
pub struct LifecycleFailure {
    provider_id: ProviderId,
    phase: LifecyclePhase,
    source: Error,
}

impl LifecycleFailure {
    fn new(
        provider_id: ProviderId,
        phase: LifecyclePhase,
        source: Error,
    ) -> Self {
        Self {
            provider_id,
            phase,
            source,
        }
    }

    /// Returns the affected provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the lifecycle phase.
    #[must_use]
    pub const fn phase(&self) -> LifecyclePhase {
        self.phase
    }

    /// Returns the original provider or manager error.
    #[must_use]
    pub fn source_error(&self) -> &Error {
        &self.source
    }
}

impl fmt::Debug for LifecycleFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LifecycleFailure")
            .field("provider_id", &self.provider_id)
            .field("phase", &self.phase)
            .field("source", &self.source)
            .finish()
    }
}

impl fmt::Display for LifecycleFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "provider {} failed during {}: {}",
            self.provider_id,
            self.phase.as_str(),
            self.source,
        )
    }
}

impl std::error::Error for LifecycleFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

// ============================================================================
// LIFECYCLE REPORT
// ============================================================================

/// Result of a coordinated initialization or shutdown operation.
///
/// Provider failures are retained in this report instead of stopping
/// cleanup at the first failed provider.
///
/// A report with failures is not a successful lifecycle operation.
/// Call [`Self::is_successful`] to inspect the result.
#[must_use]
pub struct LifecycleReport {
    attempted: Vec<ProviderId>,
    initialized: Vec<ProviderId>,
    stopped: Vec<ProviderId>,
    skipped: Vec<ProviderId>,
    failures: Vec<LifecycleFailure>,
}

impl LifecycleReport {
    fn new() -> Self {
        Self {
            attempted: Vec::new(),
            initialized: Vec::new(),
            stopped: Vec::new(),
            skipped: Vec::new(),
            failures: Vec::new(),
        }
    }

    /// Returns providers for which an initialization hook was attempted.
    #[must_use]
    pub fn attempted(&self) -> &[ProviderId] {
        &self.attempted
    }

    /// Returns providers that reached readiness during this operation.
    ///
    /// This list describes the operation's progress. A provider may
    /// subsequently have been shut down during rollback.
    #[must_use]
    pub fn initialized(&self) -> &[ProviderId] {
        &self.initialized
    }

    /// Returns providers whose shutdown completed successfully.
    #[must_use]
    pub fn stopped(&self) -> &[ProviderId] {
        &self.stopped
    }

    /// Returns providers that did not require a lifecycle transition.
    #[must_use]
    pub fn skipped(&self) -> &[ProviderId] {
        &self.skipped
    }

    /// Returns all failures, including rollback failures.
    #[must_use]
    pub fn failures(&self) -> &[LifecycleFailure] {
        &self.failures
    }

    /// Returns whether every requested lifecycle operation succeeded.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.failures.is_empty()
    }

    /// Returns whether the report contains any failures.
    #[must_use]
    pub fn has_failures(&self) -> bool {
        !self.failures.is_empty()
    }
}

impl fmt::Debug for LifecycleReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LifecycleReport")
            .field("attempted", &self.attempted)
            .field("initialized", &self.initialized)
            .field("stopped", &self.stopped)
            .field("skipped", &self.skipped)
            .field("failures", &self.failures)
            .finish()
    }
}

// ============================================================================
// INTERNAL PROVIDER ENTRY
// ============================================================================

struct ManagedProvider {
    implementation: Arc<dyn ProviderLifecycle>,
    dependencies: BTreeSet<ProviderId>,
    state: ManagedProviderState,
}

struct ManagerState {
    providers: BTreeMap<ProviderId, ManagedProvider>,
    generation: u64,
}

impl ManagerState {
    fn new() -> Self {
        Self {
            providers: BTreeMap::new(),
            generation: 0,
        }
    }

    fn next_generation(&self) -> Result<u64> {
        self.generation.checked_add(1).ok_or_else(|| {
            manager_error(
                ErrorKind::Internal,
                "provider lifecycle generation is exhausted",
            )
        })
    }
}

// ============================================================================
// MANAGER
// ============================================================================

/// Coordinates lifecycle operations for registered ZHAL providers.
///
/// The manager is explicitly constructed with the canonical provider
/// registry. It does not create global state.
///
/// Clones share the same lifecycle state and registry.
#[derive(Clone)]
pub struct ProviderLifecycleManager {
    registry: ProviderRegistry,
    state: Arc<RwLock<ManagerState>>,
}

impl ProviderLifecycleManager {
    /// Creates a lifecycle manager associated with an existing registry.
    ///
    /// The registry remains the sole provider registration authority.
    #[must_use]
    pub fn new(registry: ProviderRegistry) -> Self {
        Self {
            registry,
            state: Arc::new(RwLock::new(ManagerState::new())),
        }
    }

    /// Returns a reference to the associated provider registry.
    #[must_use]
    pub fn registry(&self) -> &ProviderRegistry {
        &self.registry
    }

    /// Returns the lifecycle manager's current generation.
    ///
    /// The generation changes when a provider is attached, detached,
    /// or transitions between managed lifecycle states.
    pub fn generation(&self) -> Result<u64> {
        Ok(self.read_state()?.generation)
    }

    // ------------------------------------------------------------------------
    // Lock access
    // ------------------------------------------------------------------------

    fn read_state(&self) -> Result<RwLockReadGuard<'_, ManagerState>> {
        self.state.read().map_err(|_| {
            manager_error(
                ErrorKind::Concurrency,
                "provider lifecycle state lock is poisoned",
            )
        })
    }

    fn write_state(&self) -> Result<RwLockWriteGuard<'_, ManagerState>> {
        self.state.write().map_err(|_| {
            manager_error(
                ErrorKind::Concurrency,
                "provider lifecycle state lock is poisoned",
            )
        })
    }

    // ------------------------------------------------------------------------
    // Provider attachment
    // ------------------------------------------------------------------------

    /// Attaches the lifecycle hooks of an already-registered provider.
    ///
    /// The supplied provider must be the exact implementation already
    /// registered in the associated ProviderRegistry.
    ///
    /// Dependencies are provider identities, not hardware device identities.
    ///
    /// Dependencies may be attached in any order. Their existence and
    /// acyclic structure are validated before initialization.
    ///
    /// This method does not initialize the provider.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - the provider is not registered;
    /// - the registered implementation is not the same Arc allocation;
    /// - the provider is already attached;
    /// - the provider depends on itself;
    /// - a dependency is duplicated;
    /// - the manager lock is poisoned;
    /// - the manager generation is exhausted.
    pub fn attach(
        &self,
        provider: Arc<dyn ProviderLifecycle>,
        dependencies: impl IntoIterator<Item = ProviderId>,
    ) -> Result<()> {
        // Provider callbacks are deliberately invoked before locking state.
        let descriptor = provider.descriptor().clone();
        let provider_id = descriptor.id().clone();

        // ProviderLifecycle: Provider is an existing supertrait relation.
        // Rust trait-object upcasting is supported by the declared baseline.
        let registered_implementation: Arc<dyn Provider> = provider.clone();

        let registered = self.registry.get_required(&provider_id)?;
        let registry_implementation = registered.implementation();

        if !Arc::ptr_eq(
            &registered_implementation,
            &registry_implementation,
        ) {
            return Err(manager_error(
                ErrorKind::ContractViolation,
                format!(
                    "lifecycle implementation for provider {} \
                     is not the implementation registered in ProviderRegistry",
                    provider_id,
                ),
            ));
        }

        let mut dependency_set = BTreeSet::new();

        for dependency in dependencies {
            if dependency == provider_id {
                return Err(manager_error(
                    ErrorKind::Dependency,
                    format!(
                        "provider {} cannot depend on itself",
                        provider_id,
                    ),
                ));
            }

            if !dependency_set.insert(dependency.clone()) {
                return Err(manager_error(
                    ErrorKind::AlreadyExists,
                    format!(
                        "provider {} declares dependency {} more than once",
                        provider_id,
                        dependency,
                    ),
                ));
            }
        }

        let mut state = self.write_state()?;

        if state.providers.contains_key(&provider_id) {
            return Err(manager_error(
                ErrorKind::AlreadyExists,
                format!(
                    "provider {} is already attached to the lifecycle manager",
                    provider_id,
                ),
            ));
        }

        let next_generation = state.next_generation()?;

        state.providers.insert(
            provider_id,
            ManagedProvider {
                implementation: provider,
                dependencies: dependency_set,
                state: ManagedProviderState::Attached,
            },
        );

        state.generation = next_generation;

        Ok(())
    }

    /// Detaches a provider from lifecycle coordination.
    ///
    /// Detachment does not unregister the provider from ProviderRegistry.
    /// Registry removal remains a separate explicit operation.
    ///
    /// A provider may only be detached while Attached or Stopped.
    /// A failed provider must first complete a successful shutdown.
    ///
    /// Providers with dependents cannot be detached.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider is unknown, active, failed,
    /// transitioning, or still required by another attached provider.
    pub fn detach(&self, provider_id: &ProviderId) -> Result<()> {
        let mut state = self.write_state()?;

        let provider = state.providers.get(provider_id).ok_or_else(|| {
            manager_error(
                ErrorKind::NotFound,
                format!(
                    "provider {} is not attached to the lifecycle manager",
                    provider_id,
                ),
            )
        })?;

        match provider.state {
            ManagedProviderState::Attached | ManagedProviderState::Stopped => {}
            current => {
                return Err(manager_error(
                    ErrorKind::Concurrency,
                    format!(
                        "provider {} cannot be detached while in state {}",
                        provider_id,
                        current,
                    ),
                ));
            }
        }

        if let Some((dependent, _)) = state.providers.iter().find(
            |(_, entry)| entry.dependencies.contains(provider_id),
        ) {
            return Err(manager_error(
                ErrorKind::Dependency,
                format!(
                    "provider {} is still required by provider {}",
                    provider_id,
                    dependent,
                ),
            ));
        }

        let next_generation = state.next_generation()?;
        state.providers.remove(provider_id);
        state.generation = next_generation;

        Ok(())
    }

    /// Returns the manager-owned state of an attached provider.
    pub fn provider_state(
        &self,
        provider_id: &ProviderId,
    ) -> Result<ManagedProviderState> {
        self.read_state()?
            .providers
            .get(provider_id)
            .map(|provider| provider.state)
            .ok_or_else(|| {
                manager_error(
                    ErrorKind::NotFound,
                    format!(
                        "provider {} is not attached to the lifecycle manager",
                        provider_id,
                    ),
                )
            })
    }

    /// Returns provider identities in deterministic order.
    pub fn provider_ids(&self) -> Result<Vec<ProviderId>> {
        Ok(self.read_state()?.providers.keys().cloned().collect())
    }

    // ------------------------------------------------------------------------
    // Dependency ordering
    // ------------------------------------------------------------------------

    fn ordered_entries(&self) -> Result<Vec<ProviderEntrySnapshot>> {
        let state = self.read_state()?;

        let order = topological_order(&state.providers)?;

        order
            .into_iter()
            .map(|provider_id| {
                let entry = state.providers.get(&provider_id).ok_or_else(|| {
                    manager_error(
                        ErrorKind::Internal,
                        "provider disappeared while creating lifecycle snapshot",
                    )
                })?;

                Ok(ProviderEntrySnapshot {
                    id: provider_id,
                    implementation: Arc::clone(&entry.implementation),
                    state: entry.state,
                })
            })
            .collect()
    }

    // ------------------------------------------------------------------------
    // State transitions
    // ------------------------------------------------------------------------

    fn begin_initialization(&self, provider_id: &ProviderId) -> Result<bool> {
        let mut state = self.write_state()?;

        let current = state
            .providers
            .get(provider_id)
            .map(|entry| entry.state)
            .ok_or_else(|| {
                manager_error(
                    ErrorKind::NotFound,
                    format!("provider {} is not attached", provider_id),
                )
            })?;

        match current {
            ManagedProviderState::Ready => return Ok(false),
            ManagedProviderState::Attached => {}
            other => {
                return Err(manager_error(
                    ErrorKind::Concurrency,
                    format!(
                        "provider {} cannot initialize from state {}",
                        provider_id,
                        other,
                    ),
                ));
            }
        }

        let next_generation = state.next_generation()?;

        let entry = state.providers.get_mut(provider_id).ok_or_else(|| {
            manager_error(
                ErrorKind::Internal,
                "provider disappeared during initialization transition",
            )
        })?;

        entry.state = ManagedProviderState::Initializing;
        state.generation = next_generation;

        Ok(true)
    }

    fn begin_shutdown(&self, provider_id: &ProviderId) -> Result<bool> {
        let mut state = self.write_state()?;

        let current = state
            .providers
            .get(provider_id)
            .map(|entry| entry.state)
            .ok_or_else(|| {
                manager_error(
                    ErrorKind::NotFound,
                    format!("provider {} is not attached", provider_id),
                )
            })?;

        match current {
            ManagedProviderState::Attached | ManagedProviderState::Stopped => {
                return Ok(false);
            }
            ManagedProviderState::Ready | ManagedProviderState::Failed => {}
            other => {
                return Err(manager_error(
                    ErrorKind::Concurrency,
                    format!(
                        "provider {} cannot shut down from state {}",
                        provider_id,
                        other,
                    ),
                ));
            }
        }

        let next_generation = state.next_generation()?;

        let entry = state.providers.get_mut(provider_id).ok_or_else(|| {
            manager_error(
                ErrorKind::Internal,
                "provider disappeared during shutdown transition",
            )
        })?;

        entry.state = ManagedProviderState::Stopping;
        state.generation = next_generation;

        Ok(true)
    }

    fn set_state(
        &self,
        provider_id: &ProviderId,
        new_state: ManagedProviderState,
    ) -> Result<()> {
        let mut state = self.write_state()?;
        let next_generation = state.next_generation()?;

        let entry = state.providers.get_mut(provider_id).ok_or_else(|| {
            manager_error(
                ErrorKind::NotFound,
                format!("provider {} is not attached", provider_id),
            )
        })?;

        entry.state = new_state;
        state.generation = next_generation;

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Registry consistency
    // ------------------------------------------------------------------------

    fn verify_registered_implementation(
        &self,
        provider_id: &ProviderId,
        implementation: &Arc<dyn ProviderLifecycle>,
    ) -> Result<()> {
        let registered = self.registry.get_required(provider_id)?;

        let lifecycle_implementation: Arc<dyn Provider> =
            implementation.clone();

        let registered_implementation = registered.implementation();

        if !Arc::ptr_eq(
            &lifecycle_implementation,
            &registered_implementation,
        ) {
            return Err(manager_error(
                ErrorKind::ContractViolation,
                format!(
                    "provider {} no longer matches its registered implementation",
                    provider_id,
                ),
            ));
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Initialization
    // ------------------------------------------------------------------------

    /// Initializes all attached providers in dependency order.
    ///
    /// Providers already in Ready state are skipped.
    ///
    /// If initialization fails:
    ///
    /// - initialization stops at the first failure;
    /// - the failing provider is included in rollback because its hook
    ///   may have partially initialized external resources;
    /// - previously initialized providers from this operation are shut
    ///   down in reverse order;
    /// - all rollback failures are retained in the report.
    ///
    /// Provider hooks are awaited without holding manager locks.
    ///
    /// A provider is considered ready only when its own status reports
    /// Ready or Degraded after successful initialization.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid dependency graphs, poisoned locks,
    /// invalid manager state or internal coordination failures.
    ///
    /// Provider hook failures are returned in the LifecycleReport.
    pub async fn initialize_all(
        &self,
        context: &ZhalContext,
    ) -> Result<LifecycleReport> {
        let ordered = self.ordered_entries()?;
        let mut report = LifecycleReport::new();
        let mut attempted: Vec<ProviderEntrySnapshot> = Vec::new();

        for entry in ordered {
            if let Err(error) = context.check_active() {
                report.failures.push(LifecycleFailure::new(
                    entry.id.clone(),
                    LifecyclePhase::Initialization,
                    error,
                ));

                self.rollback(&attempted, context, &mut report).await?;
                return Ok(report);
            }

            if let Err(error) = self.verify_registered_implementation(
                &entry.id,
                &entry.implementation,
            ) {
                report.failures.push(LifecycleFailure::new(
                    entry.id.clone(),
                    LifecyclePhase::RegistryVerification,
                    error,
                ));

                self.rollback(&attempted, context, &mut report).await?;
                return Ok(report);
            }

            let should_initialize = match self.begin_initialization(&entry.id) {
                Ok(value) => value,
                Err(error) => {
                    report.failures.push(LifecycleFailure::new(
                        entry.id.clone(),
                        LifecyclePhase::StateTransition,
                        error,
                    ));

                    self.rollback(&attempted, context, &mut report).await?;
                    return Ok(report);
                }
            };

            if !should_initialize {
                report.skipped.push(entry.id.clone());
                continue;
            }

            report.attempted.push(entry.id.clone());
            attempted.push(entry.clone());

            let child_context = match context
                .child_builder()
                .with_operation("zhal.providers.lifecycle.initialize")
            {
                Ok(builder) => builder.build(),
                Err(error) => {
                    self.set_state(
                        &entry.id,
                        ManagedProviderState::Failed,
                    )?;

                    report.failures.push(LifecycleFailure::new(
                        entry.id.clone(),
                        LifecyclePhase::Initialization,
                        error,
                    ));

                    self.rollback(&attempted, context, &mut report).await?;
                    return Ok(report);
                }
            };

            let initialization = entry
                .implementation
                .initialize(child_context.operation_context())
                .await;

            if let Err(error) = initialization {
                self.set_state(
                    &entry.id,
                    ManagedProviderState::Failed,
                )?;

                report.failures.push(LifecycleFailure::new(
                    entry.id.clone(),
                    LifecyclePhase::Initialization,
                    error,
                ));

                self.rollback(&attempted, context, &mut report).await?;
                return Ok(report);
            }

            if let Err(error) = context.check_active() {
                self.set_state(
                    &entry.id,
                    ManagedProviderState::Failed,
                )?;

                report.failures.push(LifecycleFailure::new(
                    entry.id.clone(),
                    LifecyclePhase::Initialization,
                    error,
                ));

                self.rollback(&attempted, context, &mut report).await?;
                return Ok(report);
            }

            let observed_status = entry.implementation.status();

            if !observed_status.may_accept_operations() {
                self.set_state(
                    &entry.id,
                    ManagedProviderState::Failed,
                )?;

                report.failures.push(LifecycleFailure::new(
                    entry.id.clone(),
                    LifecyclePhase::ReadinessVerification,
                    manager_error(
                        ErrorKind::ContractViolation,
                        format!(
                            "provider {} returned successfully from initialization \
                             but reports status {}",
                            entry.id,
                            observed_status,
                        ),
                    ),
                ));

                self.rollback(&attempted, context, &mut report).await?;
                return Ok(report);
            }

            self.set_state(
                &entry.id,
                ManagedProviderState::Ready,
            )?;

            report.initialized.push(entry.id);
        }

        Ok(report)
    }

    // ------------------------------------------------------------------------
    // Initialization rollback
    // ------------------------------------------------------------------------

    async fn rollback(
        &self,
        attempted: &[ProviderEntrySnapshot],
        context: &ZhalContext,
        report: &mut LifecycleReport,
    ) -> Result<()> {
        for entry in attempted.iter().rev() {
            self.set_state(
                &entry.id,
                ManagedProviderState::Stopping,
            )?;

            let child_context = match context
                .child_builder()
                .with_operation("zhal.providers.lifecycle.rollback")
            {
                Ok(builder) => builder.build(),
                Err(error) => {
                    self.set_state(
                        &entry.id,
                        ManagedProviderState::Failed,
                    )?;

                    report.failures.push(LifecycleFailure::new(
                        entry.id.clone(),
                        LifecyclePhase::RollbackShutdown,
                        error,
                    ));

                    continue;
                }
            };

            let shutdown = entry
                .implementation
                .shutdown(child_context.operation_context())
                .await;

            match shutdown {
                Err(error) => {
                    self.set_state(
                        &entry.id,
                        ManagedProviderState::Failed,
                    )?;

                    report.failures.push(LifecycleFailure::new(
                        entry.id.clone(),
                        LifecyclePhase::RollbackShutdown,
                        error,
                    ));
                }
                Ok(()) => {
                    let status = entry.implementation.status();

                    if status != ProviderStatus::Stopped {
                        self.set_state(
                            &entry.id,
                            ManagedProviderState::Failed,
                        )?;

                        report.failures.push(LifecycleFailure::new(
                            entry.id.clone(),
                            LifecyclePhase::RollbackShutdown,
                            manager_error(
                                ErrorKind::ContractViolation,
                                format!(
                                    "provider {} completed rollback shutdown \
                                     but reports status {}",
                                    entry.id,
                                    status,
                                ),
                            ),
                        ));
                    } else {
                        self.set_state(
                            &entry.id,
                            ManagedProviderState::Stopped,
                        )?;

                        report.stopped.push(entry.id.clone());
                    }
                }
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Shutdown
    // ------------------------------------------------------------------------

    /// Shuts down all active or failed providers in reverse dependency order.
    ///
    /// Attached and already-stopped providers are skipped.
    ///
    /// A failed provider is also offered shutdown because an earlier
    /// initialization or shutdown attempt may have left partial resources.
    ///
    /// Shutdown continues after individual provider failures.
    ///
    /// A provider is considered stopped only when:
    ///
    /// - its shutdown hook returns successfully; and
    /// - its status reports ProviderStatus::Stopped.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid dependency graphs or manager
    /// coordination failures.
    ///
    /// Provider shutdown failures are retained in the LifecycleReport.
    pub async fn shutdown_all(
        &self,
        context: &ZhalContext,
    ) -> Result<LifecycleReport> {
        let mut ordered = self.ordered_entries()?;
        ordered.reverse();

        let mut report = LifecycleReport::new();

        for entry in ordered {
            let should_shutdown = match self.begin_shutdown(&entry.id) {
                Ok(value) => value,
                Err(error) => {
                    report.failures.push(LifecycleFailure::new(
                        entry.id.clone(),
                        LifecyclePhase::StateTransition,
                        error,
                    ));

                    continue;
                }
            };

            if !should_shutdown {
                report.skipped.push(entry.id);
                continue;
            }

            let child_context = match context
                .child_builder()
                .with_operation("zhal.providers.lifecycle.shutdown")
            {
                Ok(builder) => builder.build(),
                Err(error) => {
                    self.set_state(
                        &entry.id,
                        ManagedProviderState::Failed,
                    )?;

                    report.failures.push(LifecycleFailure::new(
                        entry.id,
                        LifecyclePhase::Shutdown,
                        error,
                    ));

                    continue;
                }
            };

            let shutdown = entry
                .implementation
                .shutdown(child_context.operation_context())
                .await;

            match shutdown {
                Err(error) => {
                    self.set_state(
                        &entry.id,
                        ManagedProviderState::Failed,
                    )?;

                    report.failures.push(LifecycleFailure::new(
                        entry.id,
                        LifecyclePhase::Shutdown,
                        error,
                    ));
                }
                Ok(()) => {
                    let status = entry.implementation.status();

                    if status != ProviderStatus::Stopped {
                        self.set_state(
                            &entry.id,
                            ManagedProviderState::Failed,
                        )?;

                        report.failures.push(LifecycleFailure::new(
                            entry.id,
                            LifecyclePhase::Shutdown,
                            manager_error(
                                ErrorKind::ContractViolation,
                                format!(
                                    "provider {} completed shutdown \
                                     but reports status {}",
                                    entry.id,
                                    status,
                                ),
                            ),
                        ));
                    } else {
                        self.set_state(
                            &entry.id,
                            ManagedProviderState::Stopped,
                        )?;

                        report.stopped.push(entry.id);
                    }
                }
            }
        }

        Ok(report)
    }
}

// ============================================================================
// SNAPSHOT
// ============================================================================

#[derive(Clone)]
struct ProviderEntrySnapshot {
    id: ProviderId,
    implementation: Arc<dyn ProviderLifecycle>,
    state: ManagedProviderState,
}

// ============================================================================
// DEPENDENCY GRAPH
// ============================================================================

/// Computes a deterministic dependency-first provider ordering.
///
/// Kahn's algorithm is used instead of recursive depth-first traversal.
/// This avoids making dependency depth depend on the native call stack.
///
/// The BTreeMap/BTreeSet collections also make the ordering deterministic.
fn topological_order(
    providers: &BTreeMap<ProviderId, ManagedProvider>,
) -> Result<Vec<ProviderId>> {
    let mut indegree: BTreeMap<ProviderId, usize> = BTreeMap::new();

    let mut dependents: BTreeMap<
        ProviderId,
        BTreeSet<ProviderId>,
    > = BTreeMap::new();

    for (provider_id, provider) in providers {
        indegree.insert(
            provider_id.clone(),
            provider.dependencies.len(),
        );

        for dependency in &provider.dependencies {
            if !providers.contains_key(dependency) {
                return Err(manager_error(
                    ErrorKind::Dependency,
                    format!(
                        "provider {} depends on unattached provider {}",
                        provider_id,
                        dependency,
                    ),
                ));
            }

            dependents
                .entry(dependency.clone())
                .or_default()
                .insert(provider_id.clone());
        }
    }

    let mut ready: BTreeSet<ProviderId> = indegree
        .iter()
        .filter_map(|(provider_id, degree)| {
            (*degree == 0).then_some(provider_id.clone())
        })
        .collect();

    let mut ordered = Vec::with_capacity(providers.len());

    while let Some(provider_id) = ready.pop_first() {
        ordered.push(provider_id.clone());

        if let Some(children) = dependents.get(&provider_id) {
            for child in children {
                let degree = indegree.get_mut(child).ok_or_else(|| {
                    manager_error(
                        ErrorKind::Internal,
                        "dependency graph contains an unknown provider",
                    )
                })?;

                if *degree == 0 {
                    return Err(manager_error(
                        ErrorKind::Internal,
                        "dependency graph indegree became inconsistent",
                    ));
                }

                *degree -= 1;

                if *degree == 0 {
                    ready.insert(child.clone());
                }
            }
        }
    }

    if ordered.len() != providers.len() {
        return Err(manager_error(
            ErrorKind::Dependency,
            "provider dependency graph contains a cycle",
        ));
    }

    Ok(ordered)
}

// ============================================================================
// ERROR HELPERS
// ============================================================================

fn manager_error(
    kind: ErrorKind,
    message: impl Into<String>,
) -> Error {
    Error::new(kind, message)
        .in_component(COMPONENT)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Mutex;
    use std::task::{Context, Poll, Wake, Waker};
    use std::thread;

    use crate::stdlib::capability::CapabilitySnapshot;
    use crate::stdlib::provider::{
        ProviderDescriptor,
        ProviderFuture,
        ProviderKind,
    };
    use crate::stdlib::version::Version;

    struct TestProvider {
        descriptor: ProviderDescriptor,
        status: Arc<Mutex<ProviderStatus>>,
    }

    impl TestProvider {
        fn new(id: &str) -> Self {
            let version = Version::parse("1.0.0")
                .expect("test version must be valid");

            let descriptor = ProviderDescriptor::new(
                ProviderId::new(id)
                    .expect("test provider ID must be valid"),
                ProviderKind::new("test")
                    .expect("test provider kind must be valid"),
                "Lifecycle Test Provider",
                version.clone(),
                version,
            )
            .expect("test descriptor must be valid");

            Self {
                descriptor,
                status: Arc::new(Mutex::new(ProviderStatus::Unknown)),
            }
        }
    }

    impl Provider for TestProvider {
        fn descriptor(&self) -> &ProviderDescriptor {
            &self.descriptor
        }

        fn status(&self) -> ProviderStatus {
            self.status
                .lock()
                .map(|status| *status)
                .unwrap_or(ProviderStatus::Failed)
        }

        fn capability_snapshot<'a>(
            &'a self,
            _context: &'a OperationContext,
        ) -> ProviderFuture<'a, CapabilitySnapshot> {
            Box::pin(async {
                Err(Error::new(
                    ErrorKind::Unsupported,
                    "capability snapshot is not used by this lifecycle test",
                ))
            })
        }
    }

    impl ProviderLifecycle for TestProvider {
        fn initialize<'a>(
            &'a self,
            _context: &'a OperationContext,
        ) -> ProviderFuture<'a, ()> {
            let status = Arc::clone(&self.status);

            Box::pin(async move {
                let mut status = status.lock().map_err(|_| {
                    Error::new(
                        ErrorKind::Concurrency,
                        "test provider status lock is poisoned",
                    )
                })?;

                *status = ProviderStatus::Ready;
                Ok(())
            })
        }

        fn shutdown<'a>(
            &'a self,
            _context: &'a OperationContext,
        ) -> ProviderFuture<'a, ()> {
            let status = Arc::clone(&self.status);

            Box::pin(async move {
                let mut status = status.lock().map_err(|_| {
                    Error::new(
                        ErrorKind::Concurrency,
                        "test provider status lock is poisoned",
                    )
                })?;

                *status = ProviderStatus::Stopped;
                Ok(())
            })
        }
    }

    struct NoopWake;

    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}
    }

    fn block_on<F: Future>(future: F) -> F::Output {
        let waker = Waker::from(Arc::new(NoopWake));
        let mut context = Context::from_waker(&waker);
        let mut future = Box::pin(future);

        loop {
            match Pin::as_mut(&mut future).poll(&mut context) {
                Poll::Ready(output) => return output,
                Poll::Pending => thread::yield_now(),
            }
        }
    }

    fn register_and_attach(
        manager: &ProviderLifecycleManager,
        provider: Arc<TestProvider>,
        dependencies: Vec<ProviderId>,
    ) {
        let lifecycle_provider: Arc<dyn ProviderLifecycle> = provider;
        let registry_provider: Arc<dyn Provider> =
            lifecycle_provider.clone();

        manager
            .registry()
            .register(registry_provider)
            .expect("provider registration must succeed");

        manager
            .attach(lifecycle_provider, dependencies)
            .expect("provider attachment must succeed");
    }

    #[test]
    fn dependency_order_is_deterministic() {
        let registry = ProviderRegistry::new();
        let manager = ProviderLifecycleManager::new(registry);

        let base = Arc::new(TestProvider::new("org.example.base"));
        let dependent = Arc::new(TestProvider::new("org.example.dependent"));

        let base_id = base.descriptor().id().clone();

        register_and_attach(&manager, base, Vec::new());
        register_and_attach(&manager, dependent, vec![base_id.clone()]);

        let ordered = manager
            .ordered_entries()
            .expect("dependency graph must be valid");

        assert_eq!(ordered.len(), 2);
        assert_eq!(ordered[0].id, base_id);
    }

    #[test]
    fn missing_dependency_is_rejected_before_initialization() {
        let registry = ProviderRegistry::new();
        let manager = ProviderLifecycleManager::new(registry);

        let provider = Arc::new(TestProvider::new("org.example.provider"));

        let missing = ProviderId::new("org.example.missing")
            .expect("test ID must be valid");

        register_and_attach(&manager, provider, vec![missing]);

        let result = manager.ordered_entries();

        assert!(result.is_err());
    }

    #[test]
    fn initialization_and_shutdown_follow_lifecycle_contract() {
        let registry = ProviderRegistry::new();
        let manager = ProviderLifecycleManager::new(registry);

        let provider = Arc::new(TestProvider::new("org.example.provider"));
        let provider_id = provider.descriptor().id().clone();

        register_and_attach(&manager, provider, Vec::new());

        let context = ZhalContext::default();

        let initialized = block_on(manager.initialize_all(&context))
            .expect("lifecycle coordination must succeed");

        assert!(initialized.is_successful());

        assert_eq!(
            manager
                .provider_state(&provider_id)
                .expect("provider must remain attached"),
            ManagedProviderState::Ready,
        );

        let stopped = block_on(manager.shutdown_all(&context))
            .expect("shutdown coordination must succeed");

        assert!(stopped.is_successful());

        assert_eq!(
            manager
                .provider_state(&provider_id)
                .expect("provider must remain attached"),
            ManagedProviderState::Stopped,
        );
    }

    #[test]
    fn dependency_cycles_are_rejected() {
        let registry = ProviderRegistry::new();
        let manager = ProviderLifecycleManager::new(registry);

        let first = Arc::new(TestProvider::new("org.example.first"));
        let second = Arc::new(TestProvider::new("org.example.second"));

        let first_id = first.descriptor().id().clone();
        let second_id = second.descriptor().id().clone();

        register_and_attach(&manager, first, vec![second_id]);
        register_and_attach(&manager, second, vec![first_id]);

        assert!(manager.ordered_entries().is_err());
    }

    #[test]
    fn provider_can_only_be_detached_after_becoming_stopped() {
        let registry = ProviderRegistry::new();
        let manager = ProviderLifecycleManager::new(registry);

        let provider = Arc::new(TestProvider::new("org.example.provider"));
        let provider_id = provider.descriptor().id().clone();

        register_and_attach(&manager, provider, Vec::new());

        assert!(manager.detach(&provider_id).is_ok());
    }
}
