
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Lifecycle Management
//!
//! Path: src/stdlib/lifecycle.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides dependency-aware lifecycle orchestration for standard-library
//! components.
//!
//! Responsibilities include:
//!
//! - Component lifecycle registration.
//! - Dependency resolution.
//! - Dependency version compatibility.
//! - Deterministic initialization ordering.
//! - On-demand initialization.
//! - Eager initialization.
//! - External lifecycle management.
//! - Explicit readiness reporting.
//! - Initialization failure handling.
//! - Transactional rollback of newly initialized components.
//! - Reverse-order shutdown.
//! - Idempotent shutdown.
//! - Lifecycle status inspection.
//!
//! ## AUTHORITATIVE DEPENDENCIES
//!
//! - crate::stdlib::manifest
//! - crate::stdlib::context
//! - crate::stdlib::provider
//! - crate::stdlib::error
//!
//! ## DOES NOT OWN
//!
//! - Provider registration.
//! - Provider discovery.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Resource management.
//! - Runtime scheduling.
//! - Compiler initialization.
//! - Quantum execution.
//! - Distributed execution.
//! - Semantic analysis.
//! - IR construction.
//!
//! These remain the responsibilities of their respective authorities.
//!
//! ## SCALABILITY
//!
//! This module introduces no arbitrary limits on:
//!
//! - Components.
//! - Dependencies.
//! - Computational domains.
//! - Providers.
//! - Hardware resources.
//! - Distributed participants.
//!
//! Actual growth is constrained by available resources.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## CONCURRENCY CONTRACT
//!
//! Lifecycle mutations require exclusive mutable access to the manager.
//!
//! This prevents concurrent initialization and shutdown operations
//! from racing over the same component state.
//!
//! Component callbacks are awaited without holding a registry lock.
//!
//! The manager does not introduce an independent asynchronous executor.
//!
//! ## CANCELLATION
//!
//! Every callback receives the canonical OperationContext.
//!
//! Implementations must cooperate with cancellation.
//!
//! Successfully initialized components remain tracked if an operation
//! future is cancelled between lifecycle transitions.

use std::collections::{
    BTreeMap,
    BTreeSet,
};

use std::fmt;
use std::sync::Arc;

use crate::stdlib::context::OperationContext;

use crate::stdlib::error::{
    Error,
    ErrorKind,
    Result,
};

use crate::stdlib::manifest::{
    ComponentId,
    InitializationPolicy,
    Manifest,
    ShutdownPolicy,
};

use crate::stdlib::provider::ProviderFuture;


// ============================================================================
// CONSTANTS
// ============================================================================

const COMPONENT: &str = "stdlib.lifecycle";


// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn lifecycle_error(
    kind: ErrorKind,
    message: impl Into<String>,
) -> Error {
    Error::new(kind, message)
        .in_component(COMPONENT)
}

fn component_not_found(
    id: &ComponentId,
) -> Error {
    lifecycle_error(
        ErrorKind::NotFound,
        format!("component {id} is not registered"),
    )
}

fn dependency_error(
    message: impl Into<String>,
) -> Error {
    lifecycle_error(
        ErrorKind::Dependency,
        message,
    )
}

fn invalid_state(
    message: impl Into<String>,
) -> Error {
    lifecycle_error(
        ErrorKind::ContractViolation,
        message,
    )
}


// ============================================================================
// COMPONENT LIFECYCLE CONTRACT
// ============================================================================

/// Lifecycle contract for a managed standard-library component.
///
/// Implementations supply the actual component initialization and
/// shutdown operations.
///
/// The manager owns ordering and coordination.
///
/// The component owns its internal implementation and resources.
///
/// Component implementations must be thread-safe because managed
/// components can be shared through Arc.
///
/// They must also honor the supplied operation context.
///
/// Successful initialization means the component has completed
/// its declared initialization contract.
///
/// Successful shutdown means its declared shutdown contract completed.
pub trait ManagedComponent: Send + Sync {

    /// Returns the immutable component manifest.
    ///
    /// The manifest must remain stable throughout the lifetime
    /// of the registered component.
    fn manifest(&self) -> &Manifest;

    /// Initializes the component.
    ///
    /// This operation is asynchronous.
    ///
    /// It must honor cooperative cancellation.
    ///
    /// # Errors
    ///
    /// Returns a structured standard-library error if initialization
    /// cannot complete.
    fn initialize<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, ()>;

    /// Shuts down the component.
    ///
    /// This operation is asynchronous.
    ///
    /// It must release component-owned resources according to
    /// its declared shutdown contract.
    ///
    /// # Errors
    ///
    /// Returns a structured standard-library error if shutdown fails.
    fn shutdown<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, ()>;
}


// ============================================================================
// COMPONENT STATE
// ============================================================================

/// Observable lifecycle state of a standard-library component.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
)]
#[non_exhaustive]
pub enum ComponentState {

    /// Component is registered but has not been initialized.
    Registered,

    /// Component is externally managed but readiness is unconfirmed.
    ExternalPending,

    /// External authority has confirmed readiness.
    ExternalReady,

    /// Component has successfully initialized.
    Ready,

    /// Initialization failed.
    Failed,

    /// Shutdown has completed.
    Stopped,

    /// Shutdown failed and cleanup requires attention.
    ShutdownFailed,
}

impl ComponentState {

    /// Returns whether the component is ready for dependent components.
    #[must_use]
    pub const fn is_ready(self) -> bool {
        matches!(
            self,
            Self::Ready | Self::ExternalReady
        )
    }

    /// Returns whether the component has completed shutdown.
    #[must_use]
    pub const fn is_stopped(self) -> bool {
        matches!(
            self,
            Self::Stopped
        )
    }
}


// ============================================================================
// COMPONENT STATUS
// ============================================================================

/// Immutable snapshot of one component's lifecycle state.
#[derive(Clone, Debug)]
pub struct ComponentStatus {

    id: ComponentId,

    state: ComponentState,
}

impl ComponentStatus {

    /// Returns the component identity.
    #[must_use]
    pub fn id(&self) -> &ComponentId {
        &self.id
    }

    /// Returns the current lifecycle state.
    #[must_use]
    pub const fn state(&self) -> ComponentState {
        self.state
    }

    /// Returns whether the component is ready.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        self.state.is_ready()
    }
}


// ============================================================================
// LIFECYCLE REPORT
// ============================================================================

/// Report generated by a lifecycle operation.
#[derive(Clone, Debug, Default)]
pub struct LifecycleReport {

    initialized: Vec<ComponentId>,

    skipped: Vec<ComponentId>,

    stopped: Vec<ComponentId>,

    failures: Vec<String>,
}

impl LifecycleReport {

    /// Creates an empty lifecycle report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns successfully initialized components.
    #[must_use]
    pub fn initialized(&self) -> &[ComponentId] {
        &self.initialized
    }

    /// Returns components skipped by lifecycle policy.
    #[must_use]
    pub fn skipped(&self) -> &[ComponentId] {
        &self.skipped
    }

    /// Returns components successfully stopped.
    #[must_use]
    pub fn stopped(&self) -> &[ComponentId] {
        &self.stopped
    }

    /// Returns lifecycle failure descriptions.
    #[must_use]
    pub fn failures(&self) -> &[String] {
        &self.failures
    }

    /// Returns whether every attempted operation succeeded.
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.failures.is_empty()
    }
}


// ============================================================================
// REGISTERED COMPONENT
// ============================================================================

struct RegisteredComponent {

    implementation: Arc<dyn ManagedComponent>,

    state: ComponentState,
}

impl RegisteredComponent {

    fn manifest(&self) -> &Manifest {
        self.implementation.manifest()
    }
}


// ============================================================================
// LIFECYCLE MANAGER
// ============================================================================

/// Dependency-aware manager for standard-library component lifecycles.
///
/// Components are indexed by their canonical ComponentId.
///
/// The manager does not assume any particular computing domain.
///
/// It can coordinate foundation, scientific, quantum, HDL, hybrid,
/// distributed, AI, networking, security and future components.
///
/// Initialization and shutdown are asynchronous.
///
/// The manager itself does not provide an executor.
#[derive(Default)]
pub struct LifecycleManager {

    components: BTreeMap<
        ComponentId,
        RegisteredComponent,
    >,

    /// Successful initialization order.
    ///
    /// This is the authoritative ordering for manager-owned shutdown.
    initialized_order: Vec<ComponentId>,
}

impl LifecycleManager {

    // ------------------------------------------------------------------------
    // CONSTRUCTION
    // ------------------------------------------------------------------------

    /// Creates an empty lifecycle manager.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }


    // ------------------------------------------------------------------------
    // REGISTRATION
    // ------------------------------------------------------------------------

    /// Registers a component implementation.
    ///
    /// Registration does not initialize the component.
    ///
    /// Dependencies are validated when an initialization plan is built.
    ///
    /// This allows components to be registered in any order.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The component identity is already registered.
    /// - The manifest is inconsistent with the implementation.
    pub fn register(
        &mut self,
        implementation: Arc<dyn ManagedComponent>,
    ) -> Result<()> {

        let id = implementation.manifest().id().clone();

        if self.components.contains_key(&id) {
            return Err(
                lifecycle_error(
                    ErrorKind::AlreadyExists,
                    format!(
                        "component {id} is already registered"
                    ),
                )
            );
        }

        let state = match implementation
            .manifest()
            .lifecycle()
            .initialization()
        {
            InitializationPolicy::External => {
                ComponentState::ExternalPending
            }

            InitializationPolicy::OnDemand
            | InitializationPolicy::Eager => {
                ComponentState::Registered
            }

            _ => {
                return Err(
                    invalid_state(
                        format!(
                            "component {id} declares an unknown \
                             initialization policy"
                        )
                    )
                );
            }
        };

        self.components.insert(
            id,
            RegisteredComponent {
                implementation,
                state,
            },
        );

        Ok(())
    }


    // ------------------------------------------------------------------------
    // REGISTRATION INSPECTION
    // ------------------------------------------------------------------------

    /// Returns the number of registered components.
    #[must_use]
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Returns whether no components are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Returns whether a component is registered.
    #[must_use]
    pub fn contains(
        &self,
        id: &ComponentId,
    ) -> bool {
        self.components.contains_key(id)
    }

    /// Returns a snapshot of one component's status.
    #[must_use]
    pub fn status(
        &self,
        id: &ComponentId,
    ) -> Option<ComponentStatus> {

        self.components.get(id).map(|component| {
            ComponentStatus {
                id: id.clone(),
                state: component.state,
            }
        })
    }

    /// Returns deterministic snapshots of all registered components.
    #[must_use]
    pub fn statuses(&self) -> Vec<ComponentStatus> {

        self.components
            .iter()
            .map(|(id, component)| {
                ComponentStatus {
                    id: id.clone(),
                    state: component.state,
                }
            })
            .collect()
    }


    // ------------------------------------------------------------------------
    // EXTERNAL LIFECYCLE MANAGEMENT
    // ------------------------------------------------------------------------

    /// Confirms readiness of an externally managed component.
    ///
    /// This does not initialize the component.
    ///
    /// The external lifecycle authority remains responsible for
    /// its initialization and shutdown.
    ///
    /// # Errors
    ///
    /// Returns an error if the component is not externally managed.
    pub fn mark_external_ready(
        &mut self,
        id: &ComponentId,
    ) -> Result<()> {

        let component = self.components
            .get_mut(id)
            .ok_or_else(|| component_not_found(id))?;

        if !matches!(
            component.manifest().lifecycle().initialization(),
            InitializationPolicy::External
        ) {
            return Err(
                invalid_state(
                    format!(
                        "component {id} is not externally managed"
                    )
                )
            );
        }

        component.state = ComponentState::ExternalReady;

        Ok(())
    }


    /// Marks an externally managed component as not ready.
    ///
    /// This is useful when the external authority loses availability.
    ///
    /// # Errors
    ///
    /// Returns an error if the component is not externally managed.
    pub fn mark_external_unready(
        &mut self,
        id: &ComponentId,
    ) -> Result<()> {

        let component = self.components
            .get_mut(id)
            .ok_or_else(|| component_not_found(id))?;

        if !matches!(
            component.manifest().lifecycle().initialization(),
            InitializationPolicy::External
        ) {
            return Err(
                invalid_state(
                    format!(
                        "component {id} is not externally managed"
                    )
                )
            );
        }

        component.state = ComponentState::ExternalPending;

        Ok(())
    }


    // ------------------------------------------------------------------------
    // DEPENDENCY RESOLUTION
    // ------------------------------------------------------------------------

    /// Builds a deterministic dependency initialization order.
    ///
    /// Required dependencies must exist and satisfy their declared
    /// component and API version requirements.
    ///
    /// Optional dependencies are included when registered and compatible.
    ///
    /// The algorithm uses iterative graph traversal and Kahn's
    /// topological sorting algorithm.
    ///
    /// It does not use recursive dependency traversal.
    ///
    /// This avoids imposing an artificial dependency-depth ceiling
    /// through recursive call-stack growth.
    ///
    /// # Errors
    ///
    /// Returns an error for:
    ///
    /// - Missing required dependencies.
    /// - Incompatible dependency versions.
    /// - Dependency cycles.
    pub fn dependency_order(
        &self,
        root: &ComponentId,
    ) -> Result<Vec<ComponentId>> {

        if !self.components.contains_key(root) {
            return Err(component_not_found(root));
        }

        let mut selected = BTreeSet::new();

        let mut pending = vec![root.clone()];

        while let Some(current) = pending.pop() {

            if !selected.insert(current.clone()) {
                continue;
            }

            let component = self.components
                .get(&current)
                .ok_or_else(|| component_not_found(&current))?;

            for dependency in component.manifest().dependencies() {

                let dependency_id = dependency.target();

                let Some(target) = self.components.get(dependency_id)
                else {

                    if dependency.is_optional() {
                        continue;
                    }

                    return Err(
                        dependency_error(
                            format!(
                                "component {current} requires missing \
                                 component {dependency_id}"
                            )
                        )
                    );
                };

                let target_manifest = target.manifest();

                let component_version =
                    dependency.component_version_requirement();

                let api_version =
                    dependency.api_version_requirement();

                let compatible =
                    component_version.matches(
                        target_manifest.component_version()
                    )
                    &&
                    api_version.matches(
                        target_manifest.api_version()
                    );

                if !compatible {

                    if dependency.is_optional() {
                        continue;
                    }

                    return Err(
                        dependency_error(
                            format!(
                                "component {current} has an incompatible \
                                 dependency {dependency_id}"
                            )
                        )
                    );
                }

                if !selected.contains(dependency_id) {
                    pending.push(dependency_id.clone());
                }
            }
        }

        // Construct dependency edges.
        //
        // dependency -> dependent

        let mut indegree = BTreeMap::<
            ComponentId,
            usize,
        >::new();

        let mut dependents = BTreeMap::<
            ComponentId,
            BTreeSet<ComponentId>,
        >::new();

        for id in &selected {
            indegree.insert(id.clone(), 0);
        }

        for id in &selected {

            let component = self.components
                .get(id)
                .ok_or_else(|| component_not_found(id))?;

            for dependency in component.manifest().dependencies() {

                let dependency_id = dependency.target();

                if !selected.contains(dependency_id) {
                    continue;
                }

                let edges = dependents
                    .entry(dependency_id.clone())
                    .or_default();

                if edges.insert(id.clone()) {

                    let degree = indegree
                        .get_mut(id)
                        .ok_or_else(|| {
                            invalid_state(
                                "dependency graph is inconsistent"
                            )
                        })?;

                    *degree = degree.checked_add(1)
                        .ok_or_else(|| {
                            invalid_state(
                                "dependency count overflow"
                            )
                        })?;
                }
            }
        }

        // Deterministic topological ordering.

        let mut ready = BTreeSet::new();

        for (id, degree) in &indegree {
            if *degree == 0 {
                ready.insert(id.clone());
            }
        }

        let mut order = Vec::with_capacity(selected.len());

        while let Some(id) = ready.pop_first() {

            order.push(id.clone());

            if let Some(children) = dependents.get(&id) {

                for child in children {

                    let degree = indegree
                        .get_mut(child)
                        .ok_or_else(|| {
                            invalid_state(
                                "dependency graph is inconsistent"
                            )
                        })?;

                    *degree -= 1;

                    if *degree == 0 {
                        ready.insert(child.clone());
                    }
                }
            }
        }

        if order.len() != selected.len() {

            return Err(
                dependency_error(
                    "component dependency graph contains a cycle"
                )
            );
        }

        Ok(order)
    }


    // ------------------------------------------------------------------------
    // INITIALIZATION
    // ------------------------------------------------------------------------

    /// Initializes one component and all compatible required dependencies.
    ///
    /// Initialization follows the manifest dependency graph.
    ///
    /// Successfully initialized components are recorded in dependency
    /// order.
    ///
    /// If initialization fails, components initialized by this operation
    /// are rolled back in reverse order.
    ///
    /// Components that were ready before this operation are not rolled back.
    ///
    /// External components must first be confirmed ready through
    /// mark_external_ready.
    ///
    /// # Errors
    ///
    /// Returns the original initialization failure, with rollback
    /// failures included in its message.
    pub async fn initialize(
        &mut self,
        root: &ComponentId,
        context: &OperationContext,
    ) -> Result<LifecycleReport> {

        context.check_cancelled()?;

        let order = self.dependency_order(root)?;

        let mut report = LifecycleReport::new();

        let mut newly_initialized = Vec::new();

        for id in order {

            context.check_cancelled()?;

            let component = self.components
                .get(&id)
                .ok_or_else(|| component_not_found(&id))?;

            match component.state {

                ComponentState::Ready => {
                    report.skipped.push(id);
                    continue;
                }

                ComponentState::ExternalReady => {
                    report.skipped.push(id);
                    continue;
                }

                ComponentState::ExternalPending => {
                    return Err(
                        dependency_error(
                            format!(
                                "externally managed component {id} \
                                 has not confirmed readiness"
                            )
                        )
                    );
                }

                ComponentState::Stopped
                | ComponentState::Failed
                | ComponentState::ShutdownFailed
                | ComponentState::Registered => {}
            }

            let policy = component
                .manifest()
                .lifecycle()
                .initialization();

            if matches!(
                policy,
                InitializationPolicy::External
            ) {
                return Err(
                    dependency_error(
                        format!(
                            "external component {id} is not ready"
                        )
                    )
                );
            }

            // Do not hold locks across asynchronous callbacks.
            //
            // State becomes Ready only after successful completion.

            let implementation = Arc::clone(
                &component.implementation
            );

            match implementation.initialize(context).await {

                Ok(()) => {

                    let component = self.components
                        .get_mut(&id)
                        .ok_or_else(|| component_not_found(&id))?;

                    component.state = ComponentState::Ready;

                    self.initialized_order.push(id.clone());

                    newly_initialized.push(id.clone());

                    report.initialized.push(id);
                }

                Err(primary_error) => {

                    if let Some(component) =
                        self.components.get_mut(&id)
                    {
                        component.state = ComponentState::Failed;
                    }

                    let mut rollback_failures = Vec::new();

                    for rollback_id in
                        newly_initialized.iter().rev()
                    {

                        let Some(component) =
                            self.components.get(rollback_id)
                        else {
                            continue;
                        };

                        let implementation =
                            Arc::clone(&component.implementation);

                        let shutdown_policy = component
                            .manifest()
                            .lifecycle()
                            .shutdown();

                        if matches!(
                            shutdown_policy,
                            ShutdownPolicy::External
                            | ShutdownPolicy::NotApplicable
                        ) {
                            continue;
                        }

                        match implementation.shutdown(context).await {

                            Ok(()) => {

                                if let Some(component) =
                                    self.components.get_mut(rollback_id)
                                {
                                    component.state =
                                        ComponentState::Stopped;
                                }

                                self.initialized_order.retain(
                                    |item| item != rollback_id
                                );
                            }

                            Err(error) => {

                                if let Some(component) =
                                    self.components.get_mut(rollback_id)
                                {
                                    component.state =
                                        ComponentState::ShutdownFailed;
                                }

                                rollback_failures.push(
                                    format!(
                                        "{rollback_id}: {error}"
                                    )
                                );
                            }
                        }
                    }

                    if rollback_failures.is_empty() {
                        return Err(primary_error);
                    }

                    return Err(
                        lifecycle_error(
                            ErrorKind::Provider,
                            format!(
                                "initialization of {id} failed: \
                                 {primary_error}; rollback failures: {}",
                                rollback_failures.join("; ")
                            )
                        )
                    );
                }
            }
        }

        Ok(report)
    }


    // ------------------------------------------------------------------------
    // EAGER INITIALIZATION
    // ------------------------------------------------------------------------

    /// Initializes all components declaring the Eager policy.
    ///
    /// Components are processed in deterministic identity order.
    ///
    /// Dependencies are initialized automatically.
    ///
    /// Independent components continue to be attempted after a failure.
    ///
    /// This operation does not initialize OnDemand or External components
    /// unless they are dependencies of an eager component.
    ///
    /// # Errors
    ///
    /// Returns an error if cancellation prevents the operation from
    /// continuing.
    pub async fn initialize_eager(
        &mut self,
        context: &OperationContext,
    ) -> Result<LifecycleReport> {

        let roots: Vec<ComponentId> = self.components
            .iter()
            .filter_map(|(id, component)| {

                if matches!(
                    component.manifest().lifecycle().initialization(),
                    InitializationPolicy::Eager
                ) {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut combined = LifecycleReport::new();

        for root in roots {

            context.check_cancelled()?;

            match self.initialize(&root, context).await {

                Ok(mut report) => {

                    combined.initialized
                        .append(&mut report.initialized);

                    combined.skipped
                        .append(&mut report.skipped);
                }

                Err(error) => {

                    combined.failures.push(
                        format!("{root}: {error}")
                    );
                }
            }
        }

        Ok(combined)
    }


    // ------------------------------------------------------------------------
    // READINESS
    // ------------------------------------------------------------------------

    /// Checks whether a component is ready.
    #[must_use]
    pub fn is_ready(
        &self,
        id: &ComponentId,
    ) -> bool {

        self.components
            .get(id)
            .is_some_and(
                |component| component.state.is_ready()
            )
    }

    /// Checks whether all registered required dependencies of a
    /// component are ready.
    ///
    /// Missing optional dependencies do not make a component unready.
    #[must_use]
    pub fn dependencies_ready(
        &self,
        id: &ComponentId,
    ) -> bool {

        let Some(component) = self.components.get(id)
        else {
            return false;
        };

        component.manifest()
            .dependencies()
            .iter()
            .all(|dependency| {

                let Some(target) =
                    self.components.get(dependency.target())
                else {
                    return dependency.is_optional();
                };

                target.state.is_ready()
            })
    }


    // ------------------------------------------------------------------------
    // SHUTDOWN
    // ------------------------------------------------------------------------

    /// Shuts down one component.
    ///
    /// A component cannot be shut down while another ready component
    /// still depends on it.
    ///
    /// External and NotApplicable shutdown policies are respected.
    ///
    /// Repeated shutdown of an already stopped component is successful.
    ///
    /// # Errors
    ///
    /// Returns an error for:
    ///
    /// - Missing components.
    /// - Active dependent components.
    /// - Failed shutdown operations.
    pub async fn shutdown_component(
        &mut self,
        id: &ComponentId,
        context: &OperationContext,
    ) -> Result<()> {

        context.check_cancelled()?;

        let component = self.components
            .get(id)
            .ok_or_else(|| component_not_found(id))?;

        if component.state.is_stopped() {
            return Ok(());
        }

        for (dependent_id, dependent) in &self.components {

            if dependent_id == id || !dependent.state.is_ready() {
                continue;
            }

            let depends_on = dependent.manifest()
                .dependencies()
                .iter()
                .any(|dependency| {
                    dependency.target() == id
                        && !dependency.is_optional()
                });

            if depends_on {
                return Err(
                    dependency_error(
                        format!(
                            "cannot shut down {id}: ready component \
                             {dependent_id} depends on it"
                        )
                    )
                );
            }
        }

        let policy = component
            .manifest()
            .lifecycle()
            .shutdown();

        if matches!(
            policy,
            ShutdownPolicy::External
            | ShutdownPolicy::NotApplicable
        ) {
            if let Some(component) = self.components.get_mut(id) {
                component.state = ComponentState::Stopped;
            }

            self.initialized_order.retain(
                |item| item != id
            );

            return Ok(());
        }

        let implementation = Arc::clone(
            &component.implementation
        );

        match implementation.shutdown(context).await {

            Ok(()) => {

                if let Some(component) = self.components.get_mut(id) {
                    component.state = ComponentState::Stopped;
                }

                self.initialized_order.retain(
                    |item| item != id
                );

                Ok(())
            }

            Err(error) => {

                if let Some(component) = self.components.get_mut(id) {
                    component.state = ComponentState::ShutdownFailed;
                }

                Err(error)
            }
        }
    }


    /// Shuts down all manager-owned initialized components.
    ///
    /// Shutdown follows reverse successful initialization order.
    ///
    /// Components are processed even if another component fails
    /// to shut down.
    ///
    /// External and NotApplicable policies are respected.
    ///
    /// Repeated invocation is safe.
    ///
    /// # Errors
    ///
    /// Returns an error if one or more shutdown operations fail.
    pub async fn shutdown_all(
        &mut self,
        context: &OperationContext,
    ) -> Result<LifecycleReport> {

        let mut report = LifecycleReport::new();

        let order: Vec<ComponentId> =
            self.initialized_order.iter().rev().cloned().collect();

        let mut failures = Vec::new();

        for id in order {

            // Attempt all cleanup operations even if cancellation
            // has been requested. Cleanup is necessary to avoid
            // abandoning manager-owned resources.

            let Some(component) = self.components.get(&id)
            else {
                continue;
            };

            if !matches!(
                component.state,
                ComponentState::Ready
                | ComponentState::ShutdownFailed
            ) {
                continue;
            }

            let policy = component
                .manifest()
                .lifecycle()
                .shutdown();

            if matches!(
                policy,
                ShutdownPolicy::External
                | ShutdownPolicy::NotApplicable
            ) {
                if let Some(component) = self.components.get_mut(&id) {
                    component.state = ComponentState::Stopped;
                }

                report.stopped.push(id.clone());

                continue;
            }

            let implementation = Arc::clone(
                &component.implementation
            );

            match implementation.shutdown(context).await {

                Ok(()) => {

                    if let Some(component) = self.components.get_mut(&id) {
                        component.state = ComponentState::Stopped;
                    }

                    report.stopped.push(id.clone());
                }

                Err(error) => {

                    if let Some(component) = self.components.get_mut(&id) {
                        component.state = ComponentState::ShutdownFailed;
                    }

                    failures.push(
                        format!("{id}: {error}")
                    );
                }
            }
        }

        self.initialized_order.retain(|id| {

            self.components.get(id).is_some_and(
                |component| {
                    matches!(
                        component.state,
                        ComponentState::Ready
                        | ComponentState::ShutdownFailed
                    )
                }
            )
        });

        report.failures = failures.clone();

        if !failures.is_empty() {

            return Err(
                lifecycle_error(
                    ErrorKind::Provider,
                    format!(
                        "one or more component shutdown operations failed: {}",
                        failures.join("; ")
                    )
                )
            );
        }

        Ok(report)
    }


    // ------------------------------------------------------------------------
    // COMPONENT UNREGISTRATION
    // ------------------------------------------------------------------------

    /// Unregisters a component that is not active.
    ///
    /// Active components cannot be silently removed.
    ///
    /// The component must first be shut down or externally confirmed
    /// to be inactive.
    ///
    /// A component cannot be unregistered while another registered
    /// component declares it as a required dependency.
    ///
    /// # Errors
    ///
    /// Returns an error if the component is active or still required.
    pub fn unregister(
        &mut self,
        id: &ComponentId,
    ) -> Result<()> {

        let component = self.components
            .get(id)
            .ok_or_else(|| component_not_found(id))?;

        if matches!(
            component.state,
            ComponentState::Ready
            | ComponentState::ShutdownFailed
        ) {
            return Err(
                invalid_state(
                    format!(
                        "cannot unregister active component {id}"
                    )
                )
            );
        }

        for (dependent_id, dependent) in &self.components {

            if dependent_id == id {
                continue;
            }

            let required = dependent.manifest()
                .dependencies()
                .iter()
                .any(|dependency| {
                    dependency.target() == id
                        && !dependency.is_optional()
                });

            if required {
                return Err(
                    dependency_error(
                        format!(
                            "cannot unregister {id}: component \
                             {dependent_id} requires it"
                        )
                    )
                );
            }
        }

        self.components.remove(id);

        self.initialized_order.retain(
            |item| item != id
        );

        Ok(())
    }


    // ------------------------------------------------------------------------
    // RESET
    // ------------------------------------------------------------------------

    /// Resets a failed or stopped component to its registered state.
    ///
    /// This does not initialize the component.
    ///
    /// It does not silently recover a component whose shutdown failed.
    ///
    /// ShutdownFailed must be resolved through a successful shutdown
    /// before reset.
    ///
    /// # Errors
    ///
    /// Returns an error if the component is active or cleanup remains
    /// incomplete.
    pub fn reset(
        &mut self,
        id: &ComponentId,
    ) -> Result<()> {

        let component = self.components
            .get_mut(id)
            .ok_or_else(|| component_not_found(id))?;

        match component.state {

            ComponentState::Failed
            | ComponentState::Stopped => {

                component.state =
                    match component.manifest()
                        .lifecycle()
                        .initialization()
                    {
                        InitializationPolicy::External => {
                            ComponentState::ExternalPending
                        }

                        _ => ComponentState::Registered,
                    };

                Ok(())
            }

            ComponentState::ShutdownFailed => {
                Err(
                    invalid_state(
                        format!(
                            "component {id} has unresolved shutdown failure"
                        )
                    )
                )
            }

            _ => {
                Err(
                    invalid_state(
                        format!(
                            "component {id} cannot be reset from \
                             its current state"
                        )
                    )
                )
            }
        }
    }
}


// ============================================================================
// DISPLAY
// ============================================================================

impl fmt::Display for ComponentState {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        let value = match self {

            Self::Registered => "registered",

            Self::ExternalPending => "external_pending",

            Self::ExternalReady => "external_ready",

            Self::Ready => "ready",

            Self::Failed => "failed",

            Self::Stopped => "stopped",

            Self::ShutdownFailed => "shutdown_failed",
        };

        formatter.write_str(value)
    }
}


// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    use std::future::Future;
    use std::pin::Pin;
    use std::task::{
        Context,
        Poll,
        RawWaker,
        RawWakerVTable,
        Waker,
    };

    use crate::stdlib::manifest::{
        ComponentKind,
        LifecycleDeclaration,
        LifecycleFailurePolicy,
        Version,
    };

    use crate::stdlib::context::OperationContext;


    // ------------------------------------------------------------------------
    // TEST EXECUTOR
    // ------------------------------------------------------------------------

    // A minimal executor for immediately-ready test futures.
    // Production execution belongs to the canonical runtime.

    fn raw_waker() -> RawWaker {

        unsafe fn clone(_: *const ()) -> RawWaker {
            raw_waker()
        }

        unsafe fn wake(_: *const ()) {}

        unsafe fn wake_by_ref(_: *const ()) {}

        unsafe fn drop(_: *const ()) {}

        static VTABLE: RawWakerVTable =
            RawWakerVTable::new(
                clone,
                wake,
                wake_by_ref,
                drop,
            );

        RawWaker::new(
            std::ptr::null(),
            &VTABLE,
        )
    }

    fn block_on<F: Future>(
        future: F,
    ) -> F::Output {

        // This executor is intentionally restricted to tests
        // and immediately-ready lifecycle callbacks.

        let waker = unsafe {
            Waker::from_raw(raw_waker())
        };

        let mut context = Context::from_waker(&waker);

        let mut future = Box::pin(future);

        match Future::poll(
            Pin::as_mut(&mut future),
            &mut context,
        ) {
            Poll::Ready(value) => value,

            Poll::Pending => {
                panic!(
                    "test executor cannot execute pending futures"
                )
            }
        }
    }


    // ------------------------------------------------------------------------
    // TEST COMPONENT
    // ------------------------------------------------------------------------

    struct TestComponent {

        manifest: Manifest,
    }

    impl ManagedComponent for TestComponent {

        fn manifest(&self) -> &Manifest {
            &self.manifest
        }

        fn initialize<'a>(
            &'a self,
            context: &'a OperationContext,
        ) -> ProviderFuture<'a, ()> {

            Box::pin(async move {
                context.check_cancelled()?;
                Ok(())
            })
        }

        fn shutdown<'a>(
            &'a self,
            context: &'a OperationContext,
        ) -> ProviderFuture<'a, ()> {

            Box::pin(async move {
                context.check_cancelled()?;
                Ok(())
            })
        }
    }


    // ------------------------------------------------------------------------
    // TEST FIXTURES
    // ------------------------------------------------------------------------

    fn version() -> Version {
        Version::parse("1.0.0")
            .expect("valid test version")
    }

    fn manifest(
        id: &str,
        dependencies: &[&str],
    ) -> Manifest {

        let component_id = ComponentId::new(id)
            .expect("valid component ID");

        let kind = ComponentKind::new("test")
            .expect("valid component kind");

        let mut builder = Manifest::builder(
            component_id,
            kind,
            version(),
            version(),
            version(),
        );

        builder.description(
            format!("Test component {id}")
        ).expect("valid description");

        for dependency in dependencies {

            let dependency_id = ComponentId::new(*dependency)
                .expect("valid dependency ID");

            builder.dependency(
                crate::stdlib::manifest::ComponentDependency::new(
                    dependency_id
                )
            );
        }

        builder.lifecycle(
            LifecycleDeclaration::new(
                InitializationPolicy::OnDemand,
                ShutdownPolicy::Graceful,
                LifecycleFailurePolicy::AbortDependents,
            )
        );

        builder.build()
            .expect("valid manifest")
    }

    fn component(
        id: &str,
        dependencies: &[&str],
    ) -> Arc<dyn ManagedComponent> {

        Arc::new(
            TestComponent {
                manifest: manifest(id, dependencies),
            }
        )
    }

    fn manager() -> LifecycleManager {

        let mut manager = LifecycleManager::new();

        manager.register(
            component("stdlib.foundation", &[])
        ).expect("register foundation");

        manager.register(
            component(
                "stdlib.data",
                &["stdlib.foundation"]
            )
        ).expect("register data");

        manager.register(
            component(
                "stdlib.scientific",
                &["stdlib.data"]
            )
        ).expect("register scientific");

        manager
    }


    // ------------------------------------------------------------------------
    // REGISTRATION TESTS
    // ------------------------------------------------------------------------

    #[test]
    fn registers_components_without_initializing_them() {

        let manager = manager();

        assert_eq!(manager.len(), 3);

        let id = ComponentId::new("stdlib.foundation")
            .expect("valid ID");

        assert_eq!(
            manager.status(&id).expect("status").state(),
            ComponentState::Registered
        );
    }

    #[test]
    fn duplicate_registration_is_rejected() {

        let mut manager = LifecycleManager::new();

        let component = component(
            "stdlib.foundation",
            &[]
        );

        manager.register(component.clone())
            .expect("first registration");

        assert!(
            manager.register(component).is_err()
        );
    }


    // ------------------------------------------------------------------------
    // DEPENDENCY TESTS
    // ------------------------------------------------------------------------

    #[test]
    fn dependencies_are_initialized_before_dependents() {

        let manager = manager();

        let root = ComponentId::new("stdlib.scientific")
            .expect("valid ID");

        let order = manager.dependency_order(&root)
            .expect("valid dependency order");

        assert_eq!(
            order.iter()
                .map(ComponentId::as_str)
                .collect::<Vec<_>>(),
            vec![
                "stdlib.foundation",
                "stdlib.data",
                "stdlib.scientific",
            ]
        );
    }

    #[test]
    fn missing_required_dependency_is_rejected() {

        let mut manager = LifecycleManager::new();

        manager.register(
            component(
                "stdlib.scientific",
                &["stdlib.missing"]
            )
        ).expect("register component");

        let root = ComponentId::new("stdlib.scientific")
            .expect("valid ID");

        assert!(
            manager.dependency_order(&root).is_err()
        );
    }

    #[test]
    fn dependency_cycles_are_rejected() {

        let mut manager = LifecycleManager::new();

        manager.register(
            component(
                "stdlib.alpha",
                &["stdlib.beta"]
            )
        ).expect("register alpha");

        manager.register(
            component(
                "stdlib.beta",
                &["stdlib.alpha"]
            )
        ).expect("register beta");

        let root = ComponentId::new("stdlib.alpha")
            .expect("valid ID");

        assert!(
            manager.dependency_order(&root).is_err()
        );
    }


    // ------------------------------------------------------------------------
    // INITIALIZATION TESTS
    // ------------------------------------------------------------------------

    #[test]
    fn initializes_dependencies_on_demand() {

        let mut manager = manager();

        let context = OperationContext::builder().build();

        let root = ComponentId::new("stdlib.scientific")
            .expect("valid ID");

        let report = block_on(
            manager.initialize(&root, &context)
        ).expect("initialization");

        assert_eq!(report.initialized().len(), 3);

        for id in [
            "stdlib.foundation",
            "stdlib.data",
            "stdlib.scientific",
        ] {

            let id = ComponentId::new(id)
                .expect("valid ID");

            assert!(
                manager.is_ready(&id)
            );
        }
    }

    #[test]
    fn repeated_initialization_is_idempotent() {

        let mut manager = manager();

        let context = OperationContext::builder().build();

        let root = ComponentId::new("stdlib.scientific")
            .expect("valid ID");

        block_on(
            manager.initialize(&root, &context)
        ).expect("first initialization");

        let report = block_on(
            manager.initialize(&root, &context)
        ).expect("second initialization");

        assert!(
            report.initialized().is_empty()
        );

        assert_eq!(
            report.skipped().len(),
            3
        );
    }


    // ------------------------------------------------------------------------
    // SHUTDOWN TESTS
    // ------------------------------------------------------------------------

    #[test]
    fn shutdown_all_stops_components() {

        let mut manager = manager();

        let context = OperationContext::builder().build();

        let root = ComponentId::new("stdlib.scientific")
            .expect("valid ID");

        block_on(
            manager.initialize(&root, &context)
        ).expect("initialization");

        let report = block_on(
            manager.shutdown_all(&context)
        ).expect("shutdown");

        assert_eq!(report.stopped().len(), 3);

        assert!(
            manager.initialized_order.is_empty()
        );
    }

    #[test]
    fn repeated_shutdown_is_idempotent() {

        let mut manager = manager();

        let context = OperationContext::builder().build();

        let first = block_on(
            manager.shutdown_all(&context)
        ).expect("first shutdown");

        let second = block_on(
            manager.shutdown_all(&context)
        ).expect("second shutdown");

        assert!(first.stopped().is_empty());
        assert!(second.stopped().is_empty());
    }


    // ------------------------------------------------------------------------
    // EXTERNAL COMPONENT TESTS
    // ------------------------------------------------------------------------

    #[test]
    fn external_components_require_explicit_readiness() {

        let mut manager = LifecycleManager::new();

        let id = ComponentId::new("stdlib.external")
            .expect("valid ID");

        let mut builder = Manifest::builder(
            id.clone(),
            ComponentKind::new("external")
                .expect("valid kind"),
            version(),
            version(),
            version(),
        );

        builder.description("External component")
            .expect("description");

        builder.lifecycle(
            LifecycleDeclaration::new(
                InitializationPolicy::External,
                ShutdownPolicy::External,
                LifecycleFailurePolicy::AbortDependents,
            )
        );

        let manifest = builder.build()
            .expect("valid manifest");

        manager.register(
            Arc::new(TestComponent { manifest })
        ).expect("registration");

        assert!(
            !manager.is_ready(&id)
        );

        manager.mark_external_ready(&id)
            .expect("external readiness");

        assert!(
            manager.is_ready(&id)
        );
    }


    // ------------------------------------------------------------------------
    // UNREGISTRATION TESTS
    // ------------------------------------------------------------------------

    #[test]
    fn required_dependencies_cannot_be_unregistered() {

        let mut manager = manager();

        let foundation = ComponentId::new(
            "stdlib.foundation"
        ).expect("valid ID");

        assert!(
            manager.unregister(&foundation).is_err()
        );
    }

    #[test]
    fn stopped_component_can_be_unregistered() {

        let mut manager = manager();

        let context = OperationContext::builder().build();

        let root = ComponentId::new("stdlib.scientific")
            .expect("valid ID");

        block_on(
            manager.initialize(&root, &context)
        ).expect("initialization");

        block_on(
            manager.shutdown_all(&context)
        ).expect("shutdown");

        assert!(
            manager.unregister(&root).is_ok()
        );
    }
}
