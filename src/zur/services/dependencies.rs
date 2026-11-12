
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Service Dependencies
//!
//! Path: src/zur/services/dependencies.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97+
//!
//! PURPOSE
//!
//! Provides the service-dependency planning boundary for ZUR.
//!
//! This module transforms the canonical dependency graph produced
//! by ServiceRegistry into an immutable, dependency-aware plan
//! suitable for consumption by ServiceManager.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Dependency-plan construction.
//! - Parent-to-dependency relationship preservation.
//! - Required dependency verification.
//! - Optional dependency absence reporting.
//! - Dependency-plan consistency validation.
//! - Dependency-first ordering verification.
//! - Read-only dependency-plan inspection.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - Service contract definitions.
//! - Service descriptor registration.
//! - Service instance identity allocation.
//! - Dependency graph discovery.
//! - Dependency cycle detection.
//! - Service version resolution.
//! - Service initialization.
//! - Service activation.
//! - Service shutdown.
//! - Service implementation registration.
//! - Service leases.
//! - Hardware discovery.
//! - Physical resource reservation.
//! - Backend selection.
//! - Domain-provider registration.
//! - Distributed service discovery.
//! - Source-language semantics.
//! - Grammar parsing.
//! - Compiler planning.
//!
//! Those responsibilities remain with their existing
//! authoritative subsystems.
//!
//! ARCHITECTURAL AUTHORITIES
//!
//! contracts/service.rs:
//!     ServiceDependency
//!     DependencyRequirement
//!
//! core/service_registry.rs:
//!     ServiceRegistry
//!     RegisteredService
//!     ResolvedServiceGraph
//!
//! core/service_manager.rs:
//!     Actual service activation and lifecycle.
//!
//! config.rs:
//!     ServiceKey
//!     ServiceActivation
//!
//! version.rs:
//!     ServiceContractVersion
//!     VersionRange
//!
//! error.rs:
//!     ZurError
//!     ZurResult
//!
//! GRAMMAR BOUNDARY
//!
//! Source-level service and runtime intent is owned by:
//!
//!     grammar/distributed/services.g4
//!     grammar/networking/service-discovery.g4
//!     grammar/execution/runtime.g4
//!     grammar/execution/runtime-capabilities.g4
//!     grammar/resources/requirements.g4
//!
//! Those grammars are consumed by the frontend and semantic
//! authorities. This module never parses grammar or interprets
//! source declarations directly.
//!
//! INVARIANTS
//!
//! 1. Registry resolution remains authoritative.
//! 2. Required dependencies cannot be silently omitted.
//! 3. Optional dependency absence is explicitly represented.
//! 4. Every dependency retains its actual parent service.
//! 5. Dependency-first initialization ordering is preserved.
//! 6. Incompatible versions are never silently accepted.
//! 7. Planning never activates a service.
//! 8. Planning never reserves physical resources.
//! 9. Planning never claims operational readiness.
//! 10. No global mutable state.
//! 11. No unsafe Rust.
//! 12. No arbitrary dependency or service-count ceilings.
//!
//! SCALABILITY
//!
//! Dynamic collections grow according to actual requirements
//! and available memory.
//!
//! No artificial maximum is imposed on:
//!
//! - Services.
//! - Service versions.
//! - Dependencies.
//! - Dependency relationships.
//! - Computational domains.
//! - Providers.
//! - Hardware resources.
//!
//! This does not promise literal infinite physical memory
//! or infinite computation.
//!
//! THREAD SAFETY
//!
//! Dependency plans are immutable snapshots.
//!
//! They can be shared through ordinary ownership mechanisms,
//! including Arc, without introducing mutable global state.
//!
//! INTEGRATION
//!
//! ServiceRegistry:
//!     Produces ResolvedServiceGraph.
//!
//! This module:
//!     Produces ServiceDependencyPlan.
//!
//! ServiceManager:
//!     Consumes the plan to perform activation.
//!
//! The registry remains the single authority for selecting
//! compatible service versions.

use std::collections::BTreeMap;

use crate::zur::config::ServiceKey;

use crate::zur::contracts::service::{
    DependencyRequirement,
    ServiceDependency,
};

use crate::zur::core::service_registry::{
    RegisteredService,
    ResolvedServiceGraph,
    ServiceRegistry,
};

use crate::zur::error::{
    ErrorKind,
    ZurError,
    ZurResult,
};

use crate::zur::version::ServiceContractVersion;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.services.dependencies";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

/// Constructs a structured dependency-planning error.
fn dependency_error(
    code: &'static str,
    message: impl Into<String>,
) -> ZurError {
    ZurError::with_code(
        ErrorKind::Dependency,
        code,
        message,
    )
    .unwrap_or_else(|_| {
        ZurError::invalid_data(
            "unable to construct a dependency-planning error",
        )
    })
    .with_component(COMPONENT)
    .with_operation("resolve")
}

// ============================================================================
// RESOLVED DEPENDENCY
// ============================================================================

/// One dependency relationship belonging to a particular service.
///
/// The parent is represented by the containing
/// DependencyPlanEntry.
///
/// The selected service is present when a compatible registration
/// was resolved.
///
/// An absent selected service is permitted only for an optional
/// dependency.
///
/// This representation deliberately preserves optional
/// dependencies instead of silently deleting them from the plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedDependency {
    identity: ServiceKey,
    declaration: ServiceDependency,
    selected: Option<RegisteredService>,
}

impl ResolvedDependency {
    /// Constructs a resolved dependency.
    fn new(
        identity: ServiceKey,
        declaration: ServiceDependency,
        selected: Option<RegisteredService>,
    ) -> Self {
        Self {
            identity,
            declaration,
            selected,
        }
    }

    /// Returns the dependency service identity.
    #[must_use]
    pub fn identity(&self) -> &ServiceKey {
        &self.identity
    }

    /// Returns the original dependency declaration.
    #[must_use]
    pub fn declaration(&self) -> &ServiceDependency {
        &self.declaration
    }

    /// Returns whether this dependency is required.
    #[must_use]
    pub fn is_required(&self) -> bool {
        self.declaration.requirement()
            == DependencyRequirement::Required
    }

    /// Returns whether this dependency is optional.
    #[must_use]
    pub fn is_optional(&self) -> bool {
        self.declaration.requirement()
            == DependencyRequirement::Optional
    }

    /// Returns the selected registered service, if available.
    ///
    /// The selected service is a registration snapshot.
    ///
    /// It is not proof of service initialization or readiness.
    #[must_use]
    pub fn selected(&self) -> Option<&RegisteredService> {
        self.selected.as_ref()
    }

    /// Returns whether a compatible registration was resolved.
    #[must_use]
    pub fn is_resolved(&self) -> bool {
        self.selected.is_some()
    }

    /// Returns whether this optional dependency is unavailable.
    #[must_use]
    pub fn is_missing_optional(&self) -> bool {
        self.is_optional() && self.selected.is_none()
    }
}

// ============================================================================
// DEPENDENCY PLAN ENTRY
// ============================================================================

/// Dependency information for one registered service.
///
/// Every dependency belongs to the service represented by
/// `service`.
///
/// This prevents nested dependencies from being incorrectly
/// classified using the root service descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyPlanEntry {
    service: RegisteredService,
    dependencies: Vec<ResolvedDependency>,
}

impl DependencyPlanEntry {
    /// Constructs one dependency-plan entry.
    fn new(
        service: RegisteredService,
        dependencies: Vec<ResolvedDependency>,
    ) -> Self {
        Self {
            service,
            dependencies,
        }
    }

    /// Returns the registered service.
    #[must_use]
    pub fn service(&self) -> &RegisteredService {
        &self.service
    }

    /// Returns all dependencies declared by this service.
    ///
    /// Both required and optional dependencies are included.
    #[must_use]
    pub fn dependencies(&self) -> &[ResolvedDependency] {
        &self.dependencies
    }

    /// Returns the number of declared dependencies.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Returns whether the service declares dependencies.
    #[must_use]
    pub fn has_dependencies(&self) -> bool {
        !self.dependencies.is_empty()
    }

    /// Returns the dependency declaration for an identity.
    #[must_use]
    pub fn dependency(
        &self,
        identity: &ServiceKey,
    ) -> Option<&ResolvedDependency> {
        self.dependencies
            .iter()
            .find(|dependency| {
                dependency.identity() == identity
            })
    }

    /// Returns dependencies that have compatible registrations.
    #[must_use]
    pub fn resolved_dependencies(
        &self,
    ) -> impl Iterator<Item = &ResolvedDependency> {
        self.dependencies
            .iter()
            .filter(|dependency| dependency.is_resolved())
    }

    /// Returns optional dependencies that were not registered
    /// with a compatible version.
    #[must_use]
    pub fn missing_optional_dependencies(
        &self,
    ) -> impl Iterator<Item = &ResolvedDependency> {
        self.dependencies
            .iter()
            .filter(|dependency| {
                dependency.is_missing_optional()
            })
    }
}

// ============================================================================
// COMPLETE DEPENDENCY PLAN
// ============================================================================

/// Immutable service dependency plan.
///
/// Entries are ordered so that dependencies appear before their
/// dependent services.
///
/// The root service is the final entry.
///
/// A plan is a declarative snapshot. It does not activate services,
/// initialize implementations, allocate resources, or execute code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceDependencyPlan {
    root: RegisteredService,
    entries: Vec<DependencyPlanEntry>,
}

impl ServiceDependencyPlan {
    /// Constructs a validated dependency plan.
    fn new(
        root: RegisteredService,
        entries: Vec<DependencyPlanEntry>,
    ) -> Self {
        Self {
            root,
            entries,
        }
    }

    /// Returns the requested root service.
    #[must_use]
    pub fn root(&self) -> &RegisteredService {
        &self.root
    }

    /// Returns the complete dependency-first plan.
    ///
    /// The root service is the final entry.
    #[must_use]
    pub fn initialization_order(
        &self,
    ) -> &[DependencyPlanEntry] {
        &self.entries
    }

    /// Returns the number of registered services in the plan.
    #[must_use]
    pub fn service_count(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the plan contains no services.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the root service's dependency entry.
    #[must_use]
    pub fn root_entry(&self) -> Option<&DependencyPlanEntry> {
        self.entries.last().filter(|entry| {
            entry.service().instance_id()
                == self.root.instance_id()
        })
    }

    /// Returns the plan entry for an exact service instance.
    #[must_use]
    pub fn entry(
        &self,
        identity: &crate::zur::ids::ServiceInstanceId,
    ) -> Option<&DependencyPlanEntry> {
        self.entries.iter().find(|entry| {
            entry.service().instance_id() == identity
        })
    }

    /// Returns the number of dependency declarations in the plan.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.entries
            .iter()
            .map(DependencyPlanEntry::dependency_count)
            .sum()
    }

    /// Returns the number of unresolved optional dependencies.
    #[must_use]
    pub fn missing_optional_count(&self) -> usize {
        self.entries
            .iter()
            .map(|entry| {
                entry.missing_optional_dependencies().count()
            })
            .sum()
    }

    /// Returns all missing optional dependencies together
    /// with their actual parent services.
    pub fn missing_optional_dependencies(
        &self,
    ) -> impl Iterator<
        Item = (&RegisteredService, &ResolvedDependency),
    > {
        self.entries.iter().flat_map(|entry| {
            entry
                .missing_optional_dependencies()
                .map(move |dependency| {
                    (entry.service(), dependency)
                })
        })
    }

    /// Returns whether every required dependency has been resolved.
    ///
    /// This describes registration/version resolution only.
    ///
    /// It does not prove operational readiness.
    #[must_use]
    pub fn required_dependencies_resolved(&self) -> bool {
        self.entries.iter().all(|entry| {
            entry.dependencies().iter().all(|dependency| {
                !dependency.is_required()
                    || dependency.is_resolved()
            })
        })
    }
}

// ============================================================================
// SERVICE DEPENDENCY RESOLVER
// ============================================================================

/// Stateless adapter between ServiceRegistry and service management.
///
/// The registry remains the authority for dependency graph
/// construction, cycle detection and version selection.
///
/// This type does not maintain another service registry.
///
/// It does not retain mutable runtime state.
///
/// It can be freely constructed whenever a dependency plan
/// is required.
#[derive(Debug, Default, Clone, Copy)]
pub struct ServiceDependencyResolver;

impl ServiceDependencyResolver {
    /// Creates a dependency resolver.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Resolves a service dependency graph into an immutable plan.
    ///
    /// Resolution delegates to the canonical ServiceRegistry.
    ///
    /// This method then preserves each service's own dependency
    /// declarations and validates the resulting relationships.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The root service is not registered.
    /// - A required dependency cannot be resolved.
    /// - The registry produces an inconsistent graph.
    /// - A selected dependency violates its declared version range.
    /// - A dependency appears after its dependent in initialization
    ///   order.
    /// - The root service is missing from the graph.
    /// - Registry synchronization or resolution fails.
    ///
    /// Optional dependencies without compatible registrations
    /// are retained as explicitly unresolved entries.
    pub fn resolve(
        &self,
        registry: &ServiceRegistry,
        identity: &ServiceKey,
        version: &ServiceContractVersion,
    ) -> ZurResult<ServiceDependencyPlan> {
        let graph = registry.resolve(identity, version)?;

        Self::from_resolved_graph(graph)
    }

    /// Resolves the latest registered root-service version.
    ///
    /// Version selection remains the responsibility of the
    /// canonical ServiceRegistry.
    pub fn resolve_latest(
        &self,
        registry: &ServiceRegistry,
        identity: &ServiceKey,
    ) -> ZurResult<ServiceDependencyPlan> {
        let graph = registry.resolve_latest(identity)?;

        Self::from_resolved_graph(graph)
    }

    /// Converts an authoritative registry resolution result
    /// into a parent-aware dependency plan.
    ///
    /// This is also useful to callers that already possess
    /// a ResolvedServiceGraph.
    pub fn from_resolved_graph(
        graph: ResolvedServiceGraph,
    ) -> ZurResult<ServiceDependencyPlan> {
        let root = graph.root().clone();

        let order = graph.initialization_order();

        if order.is_empty() {
            return Err(dependency_error(
                "zur.dependencies.empty_graph",
                "the registry returned an empty dependency graph",
            ));
        }

        // Build an index of the exact registered service versions
        // included in the authoritative resolution.
        //
        // The registry has already selected compatible versions.
        // This index does not discover or register additional services.
        let mut candidates: BTreeMap<
            ServiceKey,
            Vec<RegisteredService>,
        > = BTreeMap::new();

        let mut positions: BTreeMap<
            (ServiceKey, ServiceContractVersion),
            usize,
        > = BTreeMap::new();

        for (position, service) in order.iter().enumerate() {
            let key = (
                service.identity().clone(),
                service.version().clone(),
            );

            if positions.insert(key, position).is_some() {
                return Err(dependency_error(
                    "zur.dependencies.duplicate_graph_node",
                    format!(
                        "dependency graph contains duplicate service \
                         `{}` version `{}`",
                        service.identity(),
                        service.version(),
                    ),
                ));
            }

            candidates
                .entry(service.identity().clone())
                .or_default()
                .push(service.clone());
        }

        // Order candidate versions deterministically.
        for versions in candidates.values_mut() {
            versions.sort_by(|left, right| {
                left.version().cmp(right.version())
            });
        }

        // The root must be represented by the final graph entry.
        let final_service = order.last().ok_or_else(|| {
            dependency_error(
                "zur.dependencies.missing_root",
                "dependency graph does not contain a root service",
            )
        })?;

        if final_service.instance_id() != root.instance_id() {
            return Err(dependency_error(
                "zur.dependencies.invalid_root_order",
                "the registry did not place the root service last",
            ));
        }

        let mut entries = Vec::with_capacity(order.len());

        // The registry already provides dependency-first ordering.
        // We preserve that order without performing another DFS.
        for (position, service) in order.iter().enumerate() {
            let mut dependencies = Vec::with_capacity(
                service.descriptor().dependency_count(),
            );

            // IMPORTANT:
            //
            // Dependencies are read from THIS service descriptor,
            // not from the root descriptor.
            //
            // This preserves nested dependency semantics.
            for (dependency_identity, declaration)
                in service.descriptor().dependencies()
            {
                let selected = candidates
                    .get(dependency_identity)
                    .and_then(|versions| {
                        versions
                            .iter()
                            .rev()
                            .find(|candidate| {
                                declaration.accepts_version(
                                    candidate.version(),
                                )
                            })
                    })
                    .cloned();

                match &selected {
                    Some(dependency_service) => {
                        let dependency_key = (
                            dependency_service.identity().clone(),
                            dependency_service.version().clone(),
                        );

                        let dependency_position =
                            positions.get(&dependency_key).copied()
                                .ok_or_else(|| {
                                    dependency_error(
                                        "zur.dependencies.unindexed_service",
                                        "selected dependency is absent \
                                         from the graph index",
                                    )
                                })?;

                        if dependency_position >= position {
                            return Err(dependency_error(
                                "zur.dependencies.invalid_order",
                                format!(
                                    "dependency `{}` of `{}` does not \
                                     precede its dependent",
                                    dependency_identity,
                                    service.identity(),
                                ),
                            ));
                        }
                    }

                    None => {
                        if declaration.requirement()
                            == DependencyRequirement::Required
                        {
                            return Err(dependency_error(
                                "zur.dependencies.required_missing",
                                format!(
                                    "required dependency `{}` of service \
                                     `{}` is absent or incompatible",
                                    dependency_identity,
                                    service.identity(),
                                ),
                            ));
                        }
                    }
                }

                dependencies.push(
                    ResolvedDependency::new(
                        dependency_identity.clone(),
                        declaration.clone(),
                        selected,
                    ),
                );
            }

            entries.push(
                DependencyPlanEntry::new(
                    service.clone(),
                    dependencies,
                ),
            );
        }

        let plan = ServiceDependencyPlan::new(
            root,
            entries,
        );

        if !plan.required_dependencies_resolved() {
            return Err(dependency_error(
                "zur.dependencies.required_unresolved",
                "the resulting plan contains an unresolved \
                 required dependency",
            ));
        }

        Ok(plan)
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

    use crate::zur::core::service_registry::ServiceRegistry;

    use crate::zur::ids::IdNamespace;

    use crate::zur::version::{
        ServiceContractVersion,
        VersionRange,
    };

    fn registry() -> ServiceRegistry {
        ServiceRegistry::new(
            IdNamespace::new(
                "zamani.test.zur.dependencies.instances",
            )
            .expect("valid identity namespace"),
        )
    }

    fn key(value: &str) -> ServiceKey {
        ServiceKey::new(value)
            .expect("valid service key")
    }

    fn version(value: &str) -> ServiceContractVersion {
        ServiceContractVersion::from_str(value)
            .expect("valid contract version")
    }

    fn range(
        minimum: &str,
        maximum: &str,
    ) -> VersionRange<ServiceContractVersion> {
        VersionRange::new(
            version(minimum),
            version(maximum),
        )
        .expect("valid version range")
    }

    fn descriptor(
        identity: &str,
        contract_version: &str,
    ) -> crate::zur::contracts::service::ServiceDescriptor {
        crate::zur::contracts::service::ServiceDescriptor::new(
            key(identity),
            version(contract_version),
        )
    }

    fn register(
        registry: &ServiceRegistry,
        descriptor: crate::zur::contracts::service::ServiceDescriptor,
    ) -> RegisteredService {
        registry
            .register(descriptor)
            .expect("service registration succeeds")
    }

    // ------------------------------------------------------------------------
    // BASIC RESOLUTION
    // ------------------------------------------------------------------------

    #[test]
    fn resolves_independent_service() {
        let registry = registry();

        let service = register(
            &registry,
            descriptor(
                "zamani.test.executor",
                "1.0.0",
            ),
        );

        let plan = ServiceDependencyResolver::new()
            .resolve(
                &registry,
                service.identity(),
                service.version(),
            )
            .unwrap();

        assert_eq!(plan.service_count(), 1);
        assert_eq!(plan.dependency_count(), 0);
        assert!(plan.required_dependencies_resolved());

        assert_eq!(
            plan.root().instance_id(),
            service.instance_id(),
        );
    }

    #[test]
    fn resolves_required_dependency() {
        let registry = registry();

        let dependency = register(
            &registry,
            descriptor(
                "zamani.test.memory",
                "1.0.0",
            ),
        );

        let root_descriptor = descriptor(
            "zamani.test.executor",
            "1.0.0",
        )
        .with_dependency(
            dependency.identity().clone(),
            ServiceDependency::required(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap();

        let root = register(&registry, root_descriptor);

        let plan = ServiceDependencyResolver::new()
            .resolve(
                &registry,
                root.identity(),
                root.version(),
            )
            .unwrap();

        assert_eq!(plan.service_count(), 2);

        assert_eq!(
            plan.initialization_order()[0]
                .service()
                .identity(),
            dependency.identity(),
        );

        assert_eq!(
            plan.initialization_order()[1]
                .service()
                .identity(),
            root.identity(),
        );

        assert!(plan.required_dependencies_resolved());
    }

    // ------------------------------------------------------------------------
    // NESTED DEPENDENCIES
    // ------------------------------------------------------------------------

    #[test]
    fn preserves_nested_parent_dependency_relationships() {
        let registry = registry();

        let storage = register(
            &registry,
            descriptor(
                "zamani.test.storage",
                "1.0.0",
            ),
        );

        let memory_descriptor = descriptor(
            "zamani.test.memory",
            "1.0.0",
        )
        .with_dependency(
            storage.identity().clone(),
            ServiceDependency::required(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap();

        let memory = register(
            &registry,
            memory_descriptor,
        );

        let executor_descriptor = descriptor(
            "zamani.test.executor",
            "1.0.0",
        )
        .with_dependency(
            memory.identity().clone(),
            ServiceDependency::required(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap();

        let executor = register(
            &registry,
            executor_descriptor,
        );

        let plan = ServiceDependencyResolver::new()
            .resolve(
                &registry,
                executor.identity(),
                executor.version(),
            )
            .unwrap();

        assert_eq!(plan.service_count(), 3);

        let memory_entry = plan
            .entry(memory.instance_id())
            .unwrap();

        let nested = memory_entry
            .dependency(storage.identity())
            .unwrap();

        assert!(nested.is_required());
        assert!(nested.is_resolved());

        assert_eq!(
            nested.selected().unwrap().instance_id(),
            storage.instance_id(),
        );

        let root_entry = plan.root_entry().unwrap();

        let root_dependency = root_entry
            .dependency(memory.identity())
            .unwrap();

        assert!(root_dependency.is_required());

        assert_eq!(
            root_dependency.selected().unwrap().instance_id(),
            memory.instance_id(),
        );
    }

    // ------------------------------------------------------------------------
    // OPTIONAL DEPENDENCIES
    // ------------------------------------------------------------------------

    #[test]
    fn preserves_missing_optional_dependencies() {
        let registry = registry();

        let root_descriptor = descriptor(
            "zamani.test.executor",
            "1.0.0",
        )
        .with_dependency(
            key("zamani.test.optional_cache"),
            ServiceDependency::optional(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap();

        let root = register(&registry, root_descriptor);

        let plan = ServiceDependencyResolver::new()
            .resolve(
                &registry,
                root.identity(),
                root.version(),
            )
            .unwrap();

        assert_eq!(plan.service_count(), 1);

        assert_eq!(
            plan.missing_optional_count(),
            1,
        );

        let missing = plan
            .root_entry()
            .unwrap()
            .dependency(
                &key("zamani.test.optional_cache"),
            )
            .unwrap();

        assert!(missing.is_optional());
        assert!(!missing.is_resolved());
        assert!(missing.is_missing_optional());

        assert!(plan.required_dependencies_resolved());
    }

    #[test]
    fn preserves_optional_dependency_parent_when_nested() {
        let registry = registry();

        let memory_descriptor = descriptor(
            "zamani.test.memory",
            "1.0.0",
        )
        .with_dependency(
            key("zamani.test.cache"),
            ServiceDependency::optional(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap();

        let memory = register(
            &registry,
            memory_descriptor,
        );

        let root_descriptor = descriptor(
            "zamani.test.executor",
            "1.0.0",
        )
        .with_dependency(
            memory.identity().clone(),
            ServiceDependency::required(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap();

        let root = register(&registry, root_descriptor);

        let plan = ServiceDependencyResolver::new()
            .resolve(
                &registry,
                root.identity(),
                root.version(),
            )
            .unwrap();

        assert_eq!(plan.missing_optional_count(), 1);

        let memory_entry = plan
            .entry(memory.instance_id())
            .unwrap();

        assert!(
            memory_entry
                .dependency(&key("zamani.test.cache"))
                .unwrap()
                .is_missing_optional()
        );

        assert_eq!(
            plan.root_entry()
                .unwrap()
                .dependency(&key("zamani.test.cache")),
            None,
        );
    }

    // ------------------------------------------------------------------------
    // VERSION COMPATIBILITY
    // ------------------------------------------------------------------------

    #[test]
    fn selects_highest_compatible_registered_version() {
        let registry = registry();

        register(
            &registry,
            descriptor(
                "zamani.test.memory",
                "1.0.0",
            ),
        );

        let newer = register(
            &registry,
            descriptor(
                "zamani.test.memory",
                "1.5.0",
            ),
        );

        register(
            &registry,
            descriptor(
                "zamani.test.memory",
                "3.0.0",
            ),
        );

        let root_descriptor = descriptor(
            "zamani.test.executor",
            "1.0.0",
        )
        .with_dependency(
            key("zamani.test.memory"),
            ServiceDependency::required(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap();

        let root = register(&registry, root_descriptor);

        let plan = ServiceDependencyResolver::new()
            .resolve(
                &registry,
                root.identity(),
                root.version(),
            )
            .unwrap();

        let selected = plan
            .root_entry()
            .unwrap()
            .dependency(
                &key("zamani.test.memory"),
            )
            .unwrap()
            .selected()
            .unwrap();

        assert_eq!(
            selected.instance_id(),
            newer.instance_id(),
        );
    }

    // ------------------------------------------------------------------------
    // DETERMINISM AND GRAPH INTEGRITY
    // ------------------------------------------------------------------------

    #[test]
    fn produces_deterministic_service_order() {
        let registry = registry();

        let first = register(
            &registry,
            descriptor(
                "zamani.test.first",
                "1.0.0",
            ),
        );

        let second = register(
            &registry,
            descriptor(
                "zamani.test.second",
                "1.0.0",
            ),
        );

        let root_descriptor = descriptor(
            "zamani.test.root",
            "1.0.0",
        )
        .with_dependency(
            first.identity().clone(),
            ServiceDependency::required(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap()
        .with_dependency(
            second.identity().clone(),
            ServiceDependency::required(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap();

        let root = register(&registry, root_descriptor);

        let resolver = ServiceDependencyResolver::new();

        let first_plan = resolver
            .resolve(
                &registry,
                root.identity(),
                root.version(),
            )
            .unwrap();

        let second_plan = resolver
            .resolve(
                &registry,
                root.identity(),
                root.version(),
            )
            .unwrap();

        assert_eq!(first_plan, second_plan);
    }

    #[test]
    fn reports_missing_optional_dependencies_with_parent() {
        let registry = registry();

        let child_descriptor = descriptor(
            "zamani.test.child",
            "1.0.0",
        )
        .with_dependency(
            key("zamani.test.optional"),
            ServiceDependency::optional(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap();

        let child = register(
            &registry,
            child_descriptor,
        );

        let root_descriptor = descriptor(
            "zamani.test.root",
            "1.0.0",
        )
        .with_dependency(
            child.identity().clone(),
            ServiceDependency::required(
                range("1.0.0", "2.0.0"),
            ),
        )
        .unwrap();

        let root = register(&registry, root_descriptor);

        let plan = ServiceDependencyResolver::new()
            .resolve(
                &registry,
                root.identity(),
                root.version(),
            )
            .unwrap();

        let missing: Vec<_> = plan
            .missing_optional_dependencies()
            .collect();

        assert_eq!(missing.len(), 1);

        assert_eq!(
            missing[0].0.identity(),
            child.identity(),
        );

        assert_eq!(
            missing[0].1.identity(),
            &key("zamani.test.optional"),
        );
    }

    // ------------------------------------------------------------------------
    // LATEST ROOT VERSION
    // ------------------------------------------------------------------------

    #[test]
    fn resolves_latest_root_version() {
        let registry = registry();

        register(
            &registry,
            descriptor(
                "zamani.test.executor",
                "1.0.0",
            ),
        );

        let latest = register(
            &registry,
            descriptor(
                "zamani.test.executor",
                "2.0.0",
            ),
        );

        let plan = ServiceDependencyResolver::new()
            .resolve_latest(
                &registry,
                latest.identity(),
            )
            .unwrap();

        assert_eq!(
            plan.root().version(),
            latest.version(),
        );
    }
}
