
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Task Provider Registry
//!
//! Path: `src/toolchain/tasks/registry.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Registers and resolves implementations capable of handling toolchain
//! task kinds. The registry is generic over the implementation contract
//! supplied by its consumer. The task executor can therefore use its own
//! authoritative provider trait without this module defining a competing
//! execution API.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Task-provider registration and unregistration.
//! - Provider descriptors and supported task-kind declarations.
//! - Task-kind-to-provider indexing.
//! - Explicit and unambiguous provider resolution.
//! - Thread-safe shared registry state.
//! - Deterministic provider enumeration.
//!
//! This module does not own:
//!
//! - Task definitions, identities, inputs, outputs or lifecycle state.
//! - Task dependency graphs or topological ordering.
//! - Task execution, worker pools or scheduling.
//! - Cancellation, retries, recovery or operational budgets.
//! - Compiler, backend, runtime or hardware provider implementations.
//! - Package registries or package dependency resolution.
//! - Provider persistence, dynamic library loading or network discovery.
//!
//! ## Canonical authorities
//!
//! - `tasks::model` owns `TaskKind` and `TaskDefinition`.
//! - `ids` owns `ProviderId`.
//! - `errors` owns toolchain error classification.
//! - `result` owns the public toolchain result aliases.
//! - `tasks::graph` reuses the canonical build graph.
//! - `tasks::executor` owns task execution and its provider execution trait.
//!
//! ## Resolution rules
//!
//! - A provider may declare support for multiple task kinds.
//! - Multiple providers may support the same task kind.
//! - An explicitly selected provider must exist and support the requested
//!   task kind.
//! - An implicit resolution succeeds only when exactly one provider
//!   supports the requested task kind.
//! - Ambiguous or missing providers are errors; the registry never silently
//!   chooses a fallback.
//! - Provider IDs and task kinds are ordered canonically for deterministic
//!   enumeration.
//!
//! ## Concurrency and lifecycle
//!
//! Cloned registry handles share the same state. The lock protects only
//! registry metadata. Implementations are returned as `Arc` handles and
//! must be invoked after the registry operation has returned.
//!
//! Registration validates the complete descriptor and checks uniqueness
//! before modifying the indexes. Unregistration removes the provider and
//! all of its task-kind index entries in one write-locked operation.
//!
//! ## Scalability
//!
//! No fixed provider count, task-kind count, domain count, target count,
//! worker count, device count or hardware-capacity limit is imposed.
//! Storage and processing are bounded only by the process's actual
//! representational and resource constraints. Operational budgets belong
//! to configuration and execution, not to this registry.
//!
//! ## Security
//!
//! Provider IDs, task kinds and metadata are identifiers, not executable
//! paths or shell commands. This registry does not load code, invoke
//! processes, access the network or establish provider trust. Those actions
//! require their own authorization and verification boundaries.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use crate::toolchain::errors::{ToolchainError, ToolchainErrorKind};
use crate::toolchain::ids::ProviderId;
use crate::toolchain::result::ToolchainResult;

use super::model::{TaskDefinition, TaskKind};

// ============================================================================
// PROVIDER DESCRIPTOR
// ============================================================================

/// Immutable descriptive registration data for one task provider.
///
/// A descriptor associates one canonical provider identity with the task
/// kinds that the provider declares it can handle. It does not prove that
/// the provider is feasible for a particular task or target. The executor
/// and the relevant capability authorities must establish that separately.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskProviderDescriptor {
    provider_id: ProviderId,
    task_kinds: Vec<TaskKind>,
    metadata: BTreeMap<String, String>,
}

impl TaskProviderDescriptor {
    /// Create a provider descriptor.
    ///
    /// Task kinds are stored in canonical order. Duplicate task kinds and
    /// an empty task-kind declaration are rejected.
    ///
    /// # Errors
    ///
    /// Returns an error if no task kinds are declared, a task kind is
    /// duplicated, or the descriptor cannot allocate its task-kind list.
    pub fn new<I>(
        provider_id: ProviderId,
        task_kinds: I,
    ) -> ToolchainResult<Self>
    where
        I: IntoIterator<Item = TaskKind>,
    {
        let mut kinds = Vec::new();

        for kind in task_kinds {
            kinds.try_reserve(1).map_err(|error| {
                ToolchainError::new(
                    ToolchainErrorKind::ResourceExhausted,
                    format!(
                        "unable to allocate task-provider task-kind list: {error}"
                    ),
                )
                .with_operation("tasks.registry.descriptor")
            })?;

            kinds.push(kind);
        }

        if kinds.is_empty() {
            return Err(registry_error(
                ToolchainErrorKind::InvalidInput,
                "tasks.registry.descriptor",
                provider_id.to_string(),
                "a task provider must declare at least one supported task kind",
            ));
        }

        kinds.sort();

        if let Some(duplicate) = kinds
            .windows(2)
            .find(|pair| pair[0] == pair[1])
        {
            return Err(registry_error(
                ToolchainErrorKind::InvalidInput,
                "tasks.registry.descriptor",
                provider_id.to_string(),
                format!(
                    "provider declares task kind `{}` more than once",
                    duplicate[0]
                ),
            ));
        }

        Ok(Self {
            provider_id,
            task_kinds: kinds,
            metadata: BTreeMap::new(),
        })
    }

    /// Add descriptive metadata to the provider descriptor.
    ///
    /// Metadata is not used to establish task compatibility, trust,
    /// authorization or execution feasibility.
    ///
    /// Keys must be non-empty and contain no whitespace or control
    /// characters. Values may contain ordinary whitespace but may not
    /// contain control characters. Duplicate keys are rejected.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid or duplicate metadata key, a
    /// control character in the value, or an allocation failure reported
    /// by the underlying collection.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let key = key.into();
        let value = value.into();

        validate_metadata_key(&key)?;

        if value.chars().any(char::is_control) {
            return Err(registry_error(
                ToolchainErrorKind::InvalidInput,
                "tasks.registry.metadata",
                self.provider_id.to_string(),
                "provider metadata values must not contain control characters",
            ));
        }

        if self.metadata.contains_key(&key) {
            return Err(registry_error(
                ToolchainErrorKind::AlreadyExists,
                "tasks.registry.metadata",
                self.provider_id.to_string(),
                format!("provider metadata key `{key}` is already present"),
            ));
        }

        self.metadata.insert(key, value);
        Ok(self)
    }

    /// Return the canonical provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Return the supported task kinds in canonical order.
    #[must_use]
    pub fn task_kinds(&self) -> &[TaskKind] {
        &self.task_kinds
    }

    /// Return descriptive metadata in canonical key order.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Return whether this provider declares support for a task kind.
    #[must_use]
    pub fn supports(&self, kind: &TaskKind) -> bool {
        self.task_kinds
            .binary_search(kind)
            .is_ok()
    }
}

// ============================================================================
// REGISTRY STORAGE
// ============================================================================

/// One provider registration and its implementation handle.
struct Registration<P: ?Sized> {
    descriptor: TaskProviderDescriptor,
    implementation: Arc<P>,
}

/// Shared registry state.
///
/// The provider map is ordered by canonical provider identity. The reverse
/// index is ordered by task kind and then provider identity.
struct RegistryState<P: ?Sized> {
    providers: BTreeMap<ProviderId, Registration<P>>,
    providers_by_kind: BTreeMap<TaskKind, BTreeSet<ProviderId>>,
}

impl<P: ?Sized> Default for RegistryState<P> {
    fn default() -> Self {
        Self {
            providers: BTreeMap::new(),
            providers_by_kind: BTreeMap::new(),
        }
    }
}

// ============================================================================
// TASK PROVIDER REGISTRY
// ============================================================================

/// Thread-safe registry of task-provider implementations.
///
/// `P` is the implementation contract supplied by the consumer. In the
/// completed task executor, this is intended to be the executor-owned
/// provider trait, for example `TaskRegistry<dyn TaskExecutorProvider>`.
///
/// The registry deliberately does not define that execution trait. Its
/// request, context, cancellation, output and failure contracts belong to
/// `tasks::executor`.
///
/// Cloning a registry shares its registrations; it does not copy them.
pub struct TaskRegistry<P: ?Sized + Send + Sync + 'static> {
    state: Arc<RwLock<RegistryState<P>>>,
}

impl<P: ?Sized + Send + Sync + 'static> Clone for TaskRegistry<P> {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

impl<P: ?Sized + Send + Sync + 'static> Default for TaskRegistry<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: ?Sized + Send + Sync + 'static> TaskRegistry<P> {
    /// Create an empty task-provider registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(RegistryState::default())),
        }
    }

    /// Register a provider descriptor and implementation atomically.
    ///
    /// Multiple providers may declare the same task kind. This is
    /// intentional: the executor may need to distinguish providers by
    /// explicit selection, policy, compatibility evidence or capability
    /// assessment.
    ///
    /// A provider ID may be registered only once. A failed registration
    /// leaves the registry unchanged.
    ///
    /// The implementation is stored as an `Arc`; it is not called while
    /// the registry lock is held.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider ID is already registered or if
    /// the registry lock is poisoned.
    pub fn register(
        &self,
        descriptor: TaskProviderDescriptor,
        implementation: Arc<P>,
    ) -> ToolchainResult<()> {
        let provider_id = descriptor.provider_id.clone();

        // Descriptor construction validates task kinds and metadata.
        // Recheck the invariant at the registry boundary so future
        // construction paths cannot bypass it.
        if descriptor.task_kinds.is_empty() {
            return Err(registry_error(
                ToolchainErrorKind::InvalidInput,
                "tasks.registry.register",
                provider_id.to_string(),
                "provider descriptor contains no supported task kinds",
            ));
        }

        if descriptor
            .task_kinds
            .windows(2)
            .any(|pair| pair[0] >= pair[1])
        {
            return Err(registry_error(
                ToolchainErrorKind::InvalidInput,
                "tasks.registry.register",
                provider_id.to_string(),
                "provider task kinds must be unique and canonically ordered",
            ));
        }

        let mut state = self.state.write().map_err(|_| {
            lock_poisoned("register")
        })?;

        if state.providers.contains_key(&provider_id) {
            return Err(registry_error(
                ToolchainErrorKind::AlreadyExists,
                "tasks.registry.register",
                provider_id.to_string(),
                format!("task provider `{provider_id}` is already registered"),
            ));
        }

        // All logical validation is complete before the indexes change.
        // Multiple providers for one task kind are supported.
        for kind in &descriptor.task_kinds {
            state
                .providers_by_kind
                .entry(kind.clone())
                .or_default()
                .insert(provider_id.clone());
        }

        state.providers.insert(
            provider_id,
            Registration {
                descriptor,
                implementation,
            },
        );

        Ok(())
    }

    /// Return whether a provider identity is registered.
    ///
    /// # Errors
    ///
    /// Returns an error if the registry lock is poisoned.
    pub fn contains(
        &self,
        provider_id: &ProviderId,
    ) -> ToolchainResult<bool> {
        let state = self.state.read().map_err(|_| {
            lock_poisoned("contains")
        })?;

        Ok(state.providers.contains_key(provider_id))
    }

    /// Return the number of registered providers.
    ///
    /// This is an observation of the current registry state, not a
    /// capacity limit.
    ///
    /// # Errors
    ///
    /// Returns an error if the registry lock is poisoned.
    pub fn len(&self) -> ToolchainResult<usize> {
        let state = self.state.read().map_err(|_| {
            lock_poisoned("len")
        })?;

        Ok(state.providers.len())
    }

    /// Return whether no providers are registered.
    ///
    /// # Errors
    ///
    /// Returns an error if the registry lock is poisoned.
    pub fn is_empty(&self) -> ToolchainResult<bool> {
        Ok(self.len()? == 0)
    }

    /// Return a provider descriptor by canonical provider identity.
    ///
    /// The implementation is not constructed or invoked.
    ///
    /// # Errors
    ///
    /// Returns `NotFound` if the provider is not registered, or an error
    /// if the registry lock is poisoned.
    pub fn descriptor(
        &self,
        provider_id: &ProviderId,
    ) -> ToolchainResult<TaskProviderDescriptor> {
        let state = self.state.read().map_err(|_| {
            lock_poisoned("descriptor")
        })?;

        state
            .providers
            .get(provider_id)
            .map(|registration| registration.descriptor.clone())
            .ok_or_else(|| {
                registry_error(
                    ToolchainErrorKind::NotFound,
                    "tasks.registry.descriptor",
                    provider_id.to_string(),
                    format!("task provider `{provider_id}` is not registered"),
                )
            })
    }

    /// Return a cloned implementation handle by provider identity.
    ///
    /// The returned `Arc` is independent of the registry lock. Unregistering
    /// the provider later does not invalidate implementation handles already
    /// returned to callers.
    ///
    /// # Errors
    ///
    /// Returns `NotFound` if the provider is not registered, or an error
    /// if the registry lock is poisoned.
    pub fn implementation(
        &self,
        provider_id: &ProviderId,
    ) -> ToolchainResult<Arc<P>> {
        let state = self.state.read().map_err(|_| {
            lock_poisoned("implementation")
        })?;

        state
            .providers
            .get(provider_id)
            .map(|registration| Arc::clone(&registration.implementation))
            .ok_or_else(|| {
                registry_error(
                    ToolchainErrorKind::NotFound,
                    "tasks.registry.implementation",
                    provider_id.to_string(),
                    format!("task provider `{provider_id}` is not registered"),
                )
            })
    }

    /// Return all provider IDs declaring support for a task kind.
    ///
    /// The result is sorted by canonical provider identity. An empty
    /// collection means no provider has declared support; it does not
    /// imply that the task kind is invalid or impossible in every future
    /// environment.
    ///
    /// # Errors
    ///
    /// Returns an error if the registry lock is poisoned.
    pub fn provider_ids_for(
        &self,
        kind: &TaskKind,
    ) -> ToolchainResult<Vec<ProviderId>> {
        let state = self.state.read().map_err(|_| {
            lock_poisoned("provider_ids_for")
        })?;

        Ok(state
            .providers_by_kind
            .get(kind)
            .map(|providers| providers.iter().cloned().collect())
            .unwrap_or_default())
    }

    /// Return descriptors for all providers supporting a task kind.
    ///
    /// Descriptors are returned in canonical provider-ID order.
    ///
    /// # Errors
    ///
    /// Returns an error if the registry lock is poisoned or an index
    /// invariant is inconsistent.
    pub fn providers_for(
        &self,
        kind: &TaskKind,
    ) -> ToolchainResult<Vec<TaskProviderDescriptor>> {
        let state = self.state.read().map_err(|_| {
            lock_poisoned("providers_for")
        })?;

        let Some(provider_ids) = state.providers_by_kind.get(kind) else {
            return Ok(Vec::new());
        };

        let mut descriptors = Vec::new();

        descriptors
            .try_reserve_exact(provider_ids.len())
            .map_err(|error| {
                ToolchainError::new(
                    ToolchainErrorKind::ResourceExhausted,
                    format!(
                        "unable to allocate task-provider descriptor list: {error}"
                    ),
                )
                .with_operation("tasks.registry.providers_for")
                .with_subject(kind.to_string())
            })?;

        for provider_id in provider_ids {
            let registration = state.providers.get(provider_id).ok_or_else(|| {
                registry_error(
                    ToolchainErrorKind::Internal,
                    "tasks.registry.providers_for",
                    provider_id.to_string(),
                    "task-kind index references a missing provider registration",
                )
            })?;

            descriptors.push(registration.descriptor.clone());
        }

        Ok(descriptors)
    }

    /// Resolve the sole registered provider for a task kind.
    ///
    /// This method intentionally refuses to choose among multiple
    /// implementations. Callers must use explicit provider selection or
    /// perform their own policy and feasibility assessment before
    /// requesting a specific provider.
    ///
    /// # Errors
    ///
    /// Returns `NotFound` when no provider supports the task kind.
    /// Returns `ValidationFailed` when more than one provider supports it.
    /// Returns an error if the registry lock is poisoned or its index is
    /// inconsistent.
    pub fn resolve_for_kind(
        &self,
        kind: &TaskKind,
    ) -> ToolchainResult<(TaskProviderDescriptor, Arc<P>)> {
        let state = self.state.read().map_err(|_| {
            lock_poisoned("resolve_for_kind")
        })?;

        let provider_ids = state
            .providers_by_kind
            .get(kind)
            .ok_or_else(|| {
                registry_error(
                    ToolchainErrorKind::NotFound,
                    "tasks.registry.resolve_for_kind",
                    kind.to_string(),
                    format!("no provider declares support for task kind `{kind}`"),
                )
            })?;

        if provider_ids.is_empty() {
            return Err(registry_error(
                ToolchainErrorKind::NotFound,
                "tasks.registry.resolve_for_kind",
                kind.to_string(),
                format!("no provider declares support for task kind `{kind}`"),
            ));
        }

        if provider_ids.len() != 1 {
            let candidates = provider_ids
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ");

            return Err(registry_error(
                ToolchainErrorKind::ValidationFailed,
                "tasks.registry.resolve_for_kind",
                kind.to_string(),
                format!(
                    "provider selection for task kind `{kind}` is ambiguous; \
                     explicitly select one of: {candidates}"
                ),
            ));
        }

        let provider_id = provider_ids
            .iter()
            .next()
            .ok_or_else(|| {
                registry_error(
                    ToolchainErrorKind::Internal,
                    "tasks.registry.resolve_for_kind",
                    kind.to_string(),
                    "provider index unexpectedly contains no provider",
                )
            })?;

        let registration = state.providers.get(provider_id).ok_or_else(|| {
            registry_error(
                ToolchainErrorKind::Internal,
                "tasks.registry.resolve_for_kind",
                provider_id.to_string(),
                "task-kind index references a missing provider registration",
            )
        })?;

        Ok((
            registration.descriptor.clone(),
            Arc::clone(&registration.implementation),
        ))
    }

    /// Resolve a specifically selected provider for a task kind.
    ///
    /// The provider must be registered and must declare support for the
    /// requested task kind. No fallback is attempted.
    ///
    /// # Errors
    ///
    /// Returns `NotFound` if the provider is not registered.
    /// Returns `UnsupportedOperation` if the provider does not declare
    /// support for the requested task kind.
    /// Returns an error if the registry lock is poisoned.
    pub fn resolve_for_provider(
        &self,
        kind: &TaskKind,
        provider_id: &ProviderId,
    ) -> ToolchainResult<(TaskProviderDescriptor, Arc<P>)> {
        let state = self.state.read().map_err(|_| {
            lock_poisoned("resolve_for_provider")
        })?;

        let registration = state
            .providers
            .get(provider_id)
            .ok_or_else(|| {
                registry_error(
                    ToolchainErrorKind::NotFound,
                    "tasks.registry.resolve_for_provider",
                    provider_id.to_string(),
                    format!("task provider `{provider_id}` is not registered"),
                )
            })?;

        if !registration.descriptor.supports(kind) {
            return Err(registry_error(
                ToolchainErrorKind::UnsupportedOperation,
                "tasks.registry.resolve_for_provider",
                provider_id.to_string(),
                format!(
                    "provider `{provider_id}` does not declare support \
                     for task kind `{kind}`"
                ),
            ));
        }

        Ok((
            registration.descriptor.clone(),
            Arc::clone(&registration.implementation),
        ))
    }

    /// Resolve a provider for a complete task definition.
    ///
    /// If the task explicitly names a provider, that exact provider must
    /// support its task kind. Otherwise, resolution succeeds only if one
    /// provider uniquely declares support for the task kind.
    ///
    /// Task-definition validation is performed before provider resolution.
    ///
    /// # Errors
    ///
    /// Returns task validation errors, missing-provider errors,
    /// unsupported-provider errors, ambiguity errors, or lock errors.
    pub fn resolve_for_task(
        &self,
        task: &TaskDefinition,
    ) -> ToolchainResult<(TaskProviderDescriptor, Arc<P>)> {
        task.validate()?;

        match task.provider() {
            Some(provider_id) => {
                self.resolve_for_provider(task.kind(), provider_id)
            }
            None => self.resolve_for_kind(task.kind()),
        }
    }

    /// Return all registered provider descriptors in canonical order.
    ///
    /// # Errors
    ///
    /// Returns an error if the registry lock is poisoned or an allocation
    /// required for the returned collection fails.
    pub fn descriptors(
        &self,
    ) -> ToolchainResult<Vec<TaskProviderDescriptor>> {
        let state = self.state.read().map_err(|_| {
            lock_poisoned("descriptors")
        })?;

        let mut descriptors = Vec::new();

        descriptors
            .try_reserve_exact(state.providers.len())
            .map_err(|error| {
                ToolchainError::new(
                    ToolchainErrorKind::ResourceExhausted,
                    format!(
                        "unable to allocate task-provider descriptor list: {error}"
                    ),
                )
                .with_operation("tasks.registry.descriptors")
            })?;

        descriptors.extend(
            state
                .providers
                .values()
                .map(|registration| registration.descriptor.clone()),
        );

        Ok(descriptors)
    }

    /// Unregister a provider and remove all of its task-kind index entries.
    ///
    /// Implementation handles previously returned as `Arc` values remain
    /// valid for their existing owners. Unregistration prevents subsequent
    /// registry lookups from discovering the provider.
    ///
    /// # Errors
    ///
    /// Returns `NotFound` if the provider is not registered, or an error
    /// if the registry lock is poisoned or an index invariant is broken.
    pub fn unregister(
        &self,
        provider_id: &ProviderId,
    ) -> ToolchainResult<TaskProviderDescriptor> {
        let mut state = self.state.write().map_err(|_| {
            lock_poisoned("unregister")
        })?;

        let registration = state
            .providers
            .remove(provider_id)
            .ok_or_else(|| {
                registry_error(
                    ToolchainErrorKind::NotFound,
                    "tasks.registry.unregister",
                    provider_id.to_string(),
                    format!("task provider `{provider_id}` is not registered"),
                )
            })?;

        for kind in &registration.descriptor.task_kinds {
            let remove_kind = {
                let provider_ids = state
                    .providers_by_kind
                    .get_mut(kind)
                    .ok_or_else(|| {
                        registry_error(
                            ToolchainErrorKind::Internal,
                            "tasks.registry.unregister",
                            kind.to_string(),
                            "provider registration is missing its task-kind index",
                        )
                    })?;

                if !provider_ids.remove(provider_id) {
                    return Err(registry_error(
                        ToolchainErrorKind::Internal,
                        "tasks.registry.unregister",
                        provider_id.to_string(),
                        "provider is missing from its task-kind index",
                    ));
                }

                provider_ids.is_empty()
            };

            if remove_kind {
                state.providers_by_kind.remove(kind);
            }
        }

        Ok(registration.descriptor)
    }
}

// ============================================================================
// VALIDATION AND ERROR HELPERS
// ============================================================================

fn validate_metadata_key(key: &str) -> ToolchainResult<()> {
    if key.trim().is_empty()
        || key.trim() != key
        || key.chars().any(char::is_control)
        || key.chars().any(char::is_whitespace)
    {
        return Err(registry_error(
            ToolchainErrorKind::InvalidInput,
            "tasks.registry.metadata",
            "metadata",
            "provider metadata keys must be non-empty and contain no whitespace or control characters",
        ));
    }

    Ok(())
}

fn registry_error(
    kind: ToolchainErrorKind,
    operation: &'static str,
    subject: impl Into<String>,
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(kind, message)
        .with_operation(operation)
        .with_subject(subject)
}

fn lock_poisoned(operation: &'static str) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::Internal,
        format!("task-provider registry lock is poisoned during `{operation}`"),
    )
    .with_operation("tasks.registry")
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::ids::IdNamespace;

    #[derive(Debug)]
    struct TestProvider {
        label: &'static str,
    }

    fn provider_id(value: u128) -> ProviderId {
        ProviderId::new(
            IdNamespace::new("test").expect("valid namespace"),
            value,
        )
        .expect("valid provider ID")
    }

    fn task_kind(name: &str) -> TaskKind {
        TaskKind::new("test", name, "1")
            .expect("valid task kind")
    }

    fn descriptor(
        id: u128,
        kinds: Vec<TaskKind>,
    ) -> TaskProviderDescriptor {
        TaskProviderDescriptor::new(provider_id(id), kinds)
            .expect("valid provider descriptor")
    }

    fn registry() -> TaskRegistry<TestProvider> {
        TaskRegistry::new()
    }

    #[test]
    fn registers_and_resolves_a_provider() {
        let registry = registry();
        let kind = task_kind("compile");
        let provider = provider_id(1);

        registry
            .register(
                descriptor(1, vec![kind.clone()]),
                Arc::new(TestProvider { label: "compiler" }),
            )
            .expect("registration should succeed");

        let (resolved, implementation) = registry
            .resolve_for_provider(&kind, &provider)
            .expect("provider should resolve");

        assert_eq!(resolved.provider_id(), &provider);
        assert!(resolved.supports(&kind));
        assert_eq!(implementation.label, "compiler");
    }

    #[test]
    fn a_single_provider_can_be_resolved_without_explicit_selection() {
        let registry = registry();
        let kind = task_kind("compile");

        registry
            .register(
                descriptor(1, vec![kind.clone()]),
                Arc::new(TestProvider { label: "only" }),
            )
            .unwrap();

        let (resolved, implementation) = registry
            .resolve_for_kind(&kind)
            .expect("one provider is unambiguous");

        assert_eq!(resolved.provider_id(), &provider_id(1));
        assert_eq!(implementation.label, "only");
    }

    #[test]
    fn multiple_providers_are_not_silently_selected() {
        let registry = registry();
        let kind = task_kind("compile");

        for (id, label) in [(2, "second"), (1, "first")] {
            registry
                .register(
                    descriptor(id, vec![kind.clone()]),
                    Arc::new(TestProvider { label }),
                )
                .unwrap();
        }

        let error = registry
            .resolve_for_kind(&kind)
            .expect_err("multiple providers require explicit selection");

        assert_eq!(error.kind(), ToolchainErrorKind::ValidationFailed);

        let ids = registry.provider_ids_for(&kind).unwrap();

        assert_eq!(ids, vec![provider_id(1), provider_id(2)]);
    }

    #[test]
    fn explicit_selection_resolves_one_of_multiple_providers() {
        let registry = registry();
        let kind = task_kind("compile");

        for id in [1, 2] {
            registry
                .register(
                    descriptor(id, vec![kind.clone()]),
                    Arc::new(TestProvider { label: "provider" }),
                )
                .unwrap();
        }

        let (resolved, _) = registry
            .resolve_for_provider(&kind, &provider_id(2))
            .expect("explicit provider should resolve");

        assert_eq!(resolved.provider_id(), &provider_id(2));
    }

    #[test]
    fn provider_cannot_resolve_an_undeclared_task_kind() {
        let registry = registry();

        registry
            .register(
                descriptor(1, vec![task_kind("compile")]),
                Arc::new(TestProvider { label: "compiler" }),
            )
            .unwrap();

        let error = registry
            .resolve_for_provider(
                &task_kind("synthesize"),
                &provider_id(1),
            )
            .expect_err("undeclared task kind must be rejected");

        assert_eq!(error.kind(), ToolchainErrorKind::UnsupportedOperation);
    }

    #[test]
    fn missing_task_kind_has_an_explicit_not_found_error() {
        let registry = registry();

        let error = registry
            .resolve_for_kind(&task_kind("unknown"))
            .expect_err("unknown task kind must not resolve");

        assert_eq!(error.kind(), ToolchainErrorKind::NotFound);
    }

    #[test]
    fn duplicate_provider_registration_is_rejected_without_replacement() {
        let registry = registry();
        let kind = task_kind("compile");

        registry
            .register(
                descriptor(1, vec![kind.clone()]),
                Arc::new(TestProvider { label: "original" }),
            )
            .unwrap();

        let result = registry.register(
            descriptor(1, vec![kind.clone()]),
            Arc::new(TestProvider { label: "replacement" }),
        );

        assert!(result.is_err());
        assert_eq!(registry.len().unwrap(), 1);

        let (_, implementation) = registry
            .resolve_for_kind(&kind)
            .expect("original registration remains");

        assert_eq!(implementation.label, "original");
    }

    #[test]
    fn descriptor_rejects_empty_task_kind_declarations() {
        let result = TaskProviderDescriptor::new(
            provider_id(1),
            Vec::<TaskKind>::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn descriptor_rejects_duplicate_task_kinds() {
        let kind = task_kind("compile");

        let result = TaskProviderDescriptor::new(
            provider_id(1),
            vec![kind.clone(), kind],
        );

        assert!(result.is_err());
    }

    #[test]
    fn descriptor_sorts_task_kinds_deterministically() {
        let descriptor = descriptor(
            1,
            vec![
                task_kind("synthesize"),
                task_kind("compile"),
                task_kind("package"),
            ],
        );

        let names = descriptor
            .task_kinds()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        assert_eq!(
            names,
            vec![
                "test::compile@1",
                "test::package@1",
                "test::synthesize@1",
            ]
        );
    }

    #[test]
    fn metadata_rejects_duplicate_or_invalid_keys() {
        let result = descriptor(1, vec![task_kind("compile")])
            .with_metadata("category", "compiler")
            .and_then(|value| {
                value.with_metadata("category", "replacement")
            });

        assert!(result.is_err());

        let invalid = descriptor(2, vec![task_kind("compile")])
            .with_metadata("invalid key", "value");

        assert!(invalid.is_err());
    }

    #[test]
    fn unregister_removes_provider_and_all_task_kind_indexes() {
        let registry = registry();
        let compile = task_kind("compile");
        let package = task_kind("package");

        registry
            .register(
                descriptor(1, vec![compile.clone(), package.clone()]),
                Arc::new(TestProvider { label: "provider" }),
            )
            .unwrap();

        let removed = registry
            .unregister(&provider_id(1))
            .expect("provider should unregister");

        assert_eq!(removed.provider_id(), &provider_id(1));
        assert!(!registry.contains(&provider_id(1)).unwrap());
        assert!(registry.provider_ids_for(&compile).unwrap().is_empty());
        assert!(registry.provider_ids_for(&package).unwrap().is_empty());
        assert!(registry.is_empty().unwrap());
    }

    #[test]
    fn unregister_does_not_remove_other_providers_for_the_same_kind() {
        let registry = registry();
        let kind = task_kind("compile");

        for id in [1, 2] {
            registry
                .register(
                    descriptor(id, vec![kind.clone()]),
                    Arc::new(TestProvider { label: "provider" }),
                )
                .unwrap();
        }

        registry.unregister(&provider_id(1)).unwrap();

        assert_eq!(
            registry.provider_ids_for(&kind).unwrap(),
            vec![provider_id(2)]
        );
    }

    #[test]
    fn previously_returned_implementation_handles_remain_valid() {
        let registry = registry();
        let kind = task_kind("compile");

        registry
            .register(
                descriptor(1, vec![kind.clone()]),
                Arc::new(TestProvider { label: "retained" }),
            )
            .unwrap();

        let implementation = registry
            .implementation(&provider_id(1))
            .unwrap();

        registry.unregister(&provider_id(1)).unwrap();

        assert_eq!(implementation.label, "retained");
        assert!(registry.implementation(&provider_id(1)).is_err());
    }

    #[test]
    fn cloned_registry_handles_share_the_same_state() {
        let registry = registry();
        let other = registry.clone();

        registry
            .register(
                descriptor(1, vec![task_kind("compile")]),
                Arc::new(TestProvider { label: "shared" }),
            )
            .unwrap();

        assert!(other.contains(&provider_id(1)).unwrap());
        assert_eq!(other.len().unwrap(), 1);
    }

    #[test]
    fn concurrent_registrations_preserve_all_unique_providers() {
        let registry = registry();
        let kind = task_kind("compile");

        let threads = (1_u128..=16)
            .map(|id| {
                let registry = registry.clone();
                let kind = kind.clone();

                std::thread::spawn(move || {
                    registry.register(
                        descriptor(id, vec![kind]),
                        Arc::new(TestProvider { label: "parallel" }),
                    )
                })
            })
            .collect::<Vec<_>>();

        for thread in threads {
            thread
                .join()
                .expect("registration thread should not panic")
                .expect("unique provider registration should succeed");
        }

        assert_eq!(registry.len().unwrap(), 16);
        assert_eq!(registry.provider_ids_for(&kind).unwrap().len(), 16);
    }
}
