
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Service Contracts
//!
//! Path: `src/zur/contracts/service.rs`
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//!
//! PURPOSE
//!
//! Defines the canonical contracts describing optional ZUR services.
//!
//! A service contract identifies a runtime service, its version,
//! its dependencies, and its lifecycle requirements.
//!
//! It describes what a service requires from the runtime service
//! environment. It does not implement or perform service execution.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Stable runtime service identity aliases.
//! - Runtime service descriptors.
//! - Runtime service contract dependencies.
//! - Dependency compatibility requirements.
//! - Required and optional dependency classifications.
//! - Service lifecycle requirements.
//! - Descriptor validation.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - Service implementation.
//! - Service registration or discovery.
//! - Service instance allocation.
//! - Service startup or shutdown execution.
//! - Runtime lifecycle state.
//! - Dependency graph execution.
//! - Dependency cycle resolution.
//! - Physical resource allocation.
//! - Resource availability.
//! - Capability discovery.
//! - Authorization or security enforcement.
//! - Hardware discovery.
//! - Backend registration.
//! - Source-language service declarations.
//! - Distributed service semantics.
//! - Network service protocols.
//! - Compiler planning.
//! - Domain-specific execution.
//!
//! Those responsibilities belong to their respective ZUR,
//! compiler, semantic, networking, distributed and domain modules.
//!
//! SCALABILITY
//!
//! Service identifiers are namespaced and extensible.
//!
//! Dependencies use dynamically sized ordered collections.
//!
//! Version components are represented by the existing ZUR
//! arbitrary-width version infrastructure.
//!
//! No fixed service count, dependency count, domain count,
//! provider count, or hardware count is imposed.
//!
//! Actual operation remains subject to available resources.
//!
//! SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No unsafe implementations.
//! No global mutable state.
//! No implicit service activation.
//! No fabricated service availability.
//!
//! INTEGRATION
//!
//! - config.rs owns ServiceKey and ServiceActivation.
//! - version.rs owns ServiceContractVersion and VersionRange.
//! - ids.rs owns ServiceInstanceId.
//! - error.rs owns ZurError and ZurResult.
//! - core/service_registry.rs consumes ServiceDescriptor.
//! - core/service_manager.rs realizes lifecycle requirements.
//! - core/context.rs provides runtime configuration.
//! - execution/ consumes the services required by executable artifacts.
//! - security/ governs authorization.
//! - observability/ monitors actual service operations.
//!
//! The descriptor is declarative. It never claims that a service
//! is registered, available, initialized, or operational.

use std::collections::BTreeMap;
use std::fmt;

use crate::zur::config::ServiceKey;
use crate::zur::error::{ZurError, ZurResult};
use crate::zur::version::{
    ServiceContractVersion,
    VersionRange,
};

// ============================================================================
// PUBLIC IDENTITY
// ============================================================================

/// Stable identity of a ZUR runtime service.
///
/// This aliases the existing canonical ServiceKey.
///
/// A service identity identifies the service contract, not a particular
/// running instance.
///
/// Service instances use ServiceInstanceId from zur::ids.
pub type ServiceId = ServiceKey;

// ============================================================================
// DEPENDENCY REQUIREMENTS
// ============================================================================

/// Describes whether a service dependency is mandatory or optional.
///
/// A required dependency must be satisfied before the dependent service
/// can become operational.
///
/// An optional dependency may be unavailable without automatically
/// making the dependent service invalid.
///
/// The service implementation must explicitly handle the absence
/// of an optional dependency.
///
/// Optional does not mean that failures may be silently ignored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum DependencyRequirement {
    /// The dependency must be available and version-compatible.
    Required,

    /// The dependency may be unavailable.
    ///
    /// The dependent service must provide an explicitly valid
    /// behavior when the dependency is absent.
    Optional,
}

impl DependencyRequirement {
    /// Returns whether the dependency is mandatory.
    #[must_use]
    pub const fn is_required(self) -> bool {
        matches!(self, Self::Required)
    }

    /// Returns whether the dependency may be unavailable.
    #[must_use]
    pub const fn is_optional(self) -> bool {
        matches!(self, Self::Optional)
    }
}

impl fmt::Display for DependencyRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Required => "required",
            Self::Optional => "optional",
        })
    }
}

// ============================================================================
// SERVICE DEPENDENCY
// ============================================================================

/// Describes one dependency of a runtime service.
///
/// A dependency specifies:
///
/// - The identity of another runtime service.
/// - The acceptable versions of its contract.
/// - Whether the dependency is required or optional.
///
/// Dependency versions are inclusive ranges.
///
/// A dependency declaration does not prove that the dependency
/// is registered, available, compatible in every other respect,
/// or successfully initialized.
///
/// The service registry and service manager establish those facts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceDependency {
    accepted_versions: VersionRange<ServiceContractVersion>,
    requirement: DependencyRequirement,
}

impl ServiceDependency {
    /// Creates a dependency with an explicit version range.
    ///
    /// # Arguments
    ///
    /// * `accepted_versions` - Inclusive acceptable contract versions.
    /// * `requirement` - Whether the dependency is required or optional.
    #[must_use]
    pub fn new(
        accepted_versions: VersionRange<ServiceContractVersion>,
        requirement: DependencyRequirement,
    ) -> Self {
        Self {
            accepted_versions,
            requirement,
        }
    }

    /// Creates a mandatory dependency.
    #[must_use]
    pub fn required(
        accepted_versions: VersionRange<ServiceContractVersion>,
    ) -> Self {
        Self::new(
            accepted_versions,
            DependencyRequirement::Required,
        )
    }

    /// Creates an optional dependency.
    #[must_use]
    pub fn optional(
        accepted_versions: VersionRange<ServiceContractVersion>,
    ) -> Self {
        Self::new(
            accepted_versions,
            DependencyRequirement::Optional,
        )
    }

    /// Returns the accepted service contract version range.
    #[must_use]
    pub fn accepted_versions(
        &self,
    ) -> &VersionRange<ServiceContractVersion> {
        &self.accepted_versions
    }

    /// Returns the dependency requirement classification.
    #[must_use]
    pub const fn requirement(&self) -> DependencyRequirement {
        self.requirement
    }

    /// Checks whether a supplied service contract version
    /// falls inside the declared acceptable range.
    ///
    /// This establishes version-range compatibility only.
    ///
    /// It does not establish ABI compatibility, capability
    /// compatibility, security authorization, or execution feasibility.
    #[must_use]
    pub fn accepts_version(
        &self,
        supplied: &ServiceContractVersion,
    ) -> bool {
        self.accepted_versions.contains(supplied)
    }
}

// ============================================================================
// SERVICE STARTUP REQUIREMENTS
// ============================================================================

/// Declares when a runtime service must be initialized.
///
/// This is a lifecycle requirement, not a service activation policy.
///
/// The runtime configuration may disable a service or impose
/// an operational activation policy.
///
/// A mandatory lifecycle requirement must not be silently violated
/// by the service manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum StartupRequirement {
    /// Initialize the service when an actual execution requires it.
    ///
    /// This is compatible with lazy service activation.
    #[default]
    OnDemand,

    /// Initialize the service before accepting execution that
    /// requires the service.
    BeforeExecution,
}

impl StartupRequirement {
    /// Returns whether initialization may be deferred until required.
    #[must_use]
    pub const fn permits_lazy_initialization(self) -> bool {
        matches!(self, Self::OnDemand)
    }
}

// ============================================================================
// SERVICE READINESS REQUIREMENTS
// ============================================================================

/// Declares how service readiness is established.
///
/// The service lifecycle implementation must honor this declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum ReadinessRequirement {
    /// The service must explicitly report operational readiness.
    ///
    /// Initialization alone does not establish readiness.
    #[default]
    Explicit,

    /// Successful initialization is sufficient to establish readiness.
    ///
    /// This is suitable only for services whose implementation
    /// can guarantee that initialization completes all required
    /// readiness checks.
    InitializationComplete,
}

// ============================================================================
// SERVICE SHUTDOWN REQUIREMENTS
// ============================================================================

/// Declares the shutdown guarantees required by a service.
///
/// This is not an implementation of shutdown behavior.
///
/// Actual shutdown belongs to the service manager and service
/// implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum ShutdownRequirement {
    /// The service requires an opportunity to perform graceful shutdown.
    ///
    /// The manager must honor this requirement when performing
    /// an orderly shutdown.
    #[default]
    Graceful,

    /// The service permits immediate termination.
    ///
    /// This does not authorize unsafe termination or bypass
    /// memory-safety and resource-ownership guarantees.
    ImmediateAllowed,
}

// ============================================================================
// SERVICE LIFECYCLE REQUIREMENTS
// ============================================================================

/// Describes the lifecycle requirements of a runtime service.
///
/// These requirements are consumed by the ZUR lifecycle manager.
///
/// They do not define the runtime's actual lifecycle state machine.
///
/// The authoritative lifecycle implementation belongs to:
///
/// `src/zur/core/lifecycle.rs`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ServiceLifecycleRequirements {
    startup: StartupRequirement,
    readiness: ReadinessRequirement,
    shutdown: ShutdownRequirement,
}

impl ServiceLifecycleRequirements {
    /// Creates explicit lifecycle requirements.
    #[must_use]
    pub const fn new(
        startup: StartupRequirement,
        readiness: ReadinessRequirement,
        shutdown: ShutdownRequirement,
    ) -> Self {
        Self {
            startup,
            readiness,
            shutdown,
        }
    }

    /// Returns the startup requirement.
    #[must_use]
    pub const fn startup(&self) -> StartupRequirement {
        self.startup
    }

    /// Returns the readiness requirement.
    #[must_use]
    pub const fn readiness(&self) -> ReadinessRequirement {
        self.readiness
    }

    /// Returns the shutdown requirement.
    #[must_use]
    pub const fn shutdown(&self) -> ShutdownRequirement {
        self.shutdown
    }
}

// ============================================================================
// SERVICE DESCRIPTOR
// ============================================================================

/// Immutable declarative contract for one ZUR service.
///
/// The descriptor contains the information required by the service
/// registry and service manager to validate and manage a service.
///
/// It does not contain:
///
/// - A service implementation object.
/// - A running service instance.
/// - An operating-system process.
/// - A physical device.
/// - An allocated resource.
/// - A service availability assertion.
///
/// A descriptor can exist even when its service implementation
/// is not installed or available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceDescriptor {
    identity: ServiceId,
    version: ServiceContractVersion,
    lifecycle: ServiceLifecycleRequirements,

    // Ordered representation ensures deterministic iteration,
    // validation and dependency-plan construction.
    dependencies: BTreeMap<ServiceId, ServiceDependency>,
}

impl ServiceDescriptor {
    /// Creates a service descriptor.
    ///
    /// The identity and version must already have passed their
    /// respective canonical validation.
    ///
    /// New services default to:
    ///
    /// - On-demand initialization.
    /// - Explicit readiness.
    /// - Graceful shutdown.
    #[must_use]
    pub fn new(
        identity: ServiceId,
        version: ServiceContractVersion,
    ) -> Self {
        Self {
            identity,
            version,
            lifecycle: ServiceLifecycleRequirements::default(),
            dependencies: BTreeMap::new(),
        }
    }

    /// Returns the stable service identity.
    #[must_use]
    pub fn identity(&self) -> &ServiceId {
        &self.identity
    }

    /// Returns the declared service contract version.
    #[must_use]
    pub fn version(&self) -> &ServiceContractVersion {
        &self.version
    }

    /// Returns the service lifecycle requirements.
    #[must_use]
    pub const fn lifecycle(
        &self,
    ) -> &ServiceLifecycleRequirements {
        &self.lifecycle
    }

    /// Returns all service dependencies.
    ///
    /// Dependencies are ordered by their canonical service identities.
    #[must_use]
    pub fn dependencies(
        &self,
    ) -> &BTreeMap<ServiceId, ServiceDependency> {
        &self.dependencies
    }

    /// Returns a dependency by service identity.
    #[must_use]
    pub fn dependency(
        &self,
        identity: &ServiceId,
    ) -> Option<&ServiceDependency> {
        self.dependencies.get(identity)
    }

    /// Returns the number of declared dependencies.
    ///
    /// This is an actual collection count, not an architecture limit.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Returns whether the descriptor has any dependencies.
    #[must_use]
    pub fn has_dependencies(&self) -> bool {
        !self.dependencies.is_empty()
    }

    /// Sets the service lifecycle requirements.
    ///
    /// This replaces the previous lifecycle declaration.
    #[must_use]
    pub fn with_lifecycle(
        mut self,
        lifecycle: ServiceLifecycleRequirements,
    ) -> Self {
        self.lifecycle = lifecycle;
        self
    }

    /// Adds a service dependency.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - A service depends on itself.
    /// - The same service dependency is declared more than once.
    ///
    /// Dependencies are never silently overwritten.
    pub fn with_dependency(
        mut self,
        identity: ServiceId,
        dependency: ServiceDependency,
    ) -> ZurResult<Self> {
        self.add_dependency(identity, dependency)?;
        Ok(self)
    }

    /// Adds a dependency to an existing descriptor.
    ///
    /// This method is useful when constructing descriptors
    /// incrementally.
    ///
    /// # Errors
    ///
    /// Returns a structured validation error for self-dependencies
    /// or duplicate dependencies.
    pub fn add_dependency(
        &mut self,
        identity: ServiceId,
        dependency: ServiceDependency,
    ) -> ZurResult<()> {
        if identity == self.identity {
            return Err(service_validation_error(
                format!(
                    "service `{}` cannot depend on itself",
                    self.identity
                ),
            ));
        }

        if self.dependencies.contains_key(&identity) {
            return Err(service_validation_error(
                format!(
                    "duplicate dependency `{identity}` in service `{}`",
                    self.identity
                ),
            ));
        }

        self.dependencies.insert(identity, dependency);

        Ok(())
    }

    /// Removes a previously declared dependency.
    ///
    /// Returns the removed dependency when present.
    ///
    /// Removal is explicit and does not modify another descriptor.
    pub fn remove_dependency(
        &mut self,
        identity: &ServiceId,
    ) -> Option<ServiceDependency> {
        self.dependencies.remove(identity)
    }

    /// Validates the internal consistency of this descriptor.
    ///
    /// This method verifies descriptor-local invariants.
    ///
    /// It does not:
    ///
    /// - Resolve dependency registration.
    /// - Detect cycles across multiple descriptors.
    /// - Verify actual service availability.
    /// - Verify physical resource availability.
    /// - Establish runtime compatibility beyond declared versions.
    /// - Initialize services.
    ///
    /// Those operations belong to the service registry and manager.
    ///
    /// # Errors
    ///
    /// Returns a structured error if a descriptor invariant fails.
    pub fn validate(&self) -> ZurResult<()> {
        for identity in self.dependencies.keys() {
            if identity == &self.identity {
                return Err(service_validation_error(
                    format!(
                        "service `{}` contains a self-dependency",
                        self.identity
                    ),
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// SERVICE CONTRACT VALIDATION
// ============================================================================

/// Constructs a structured service-contract validation error.
fn service_validation_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::invalid_input(message)
        .with_component("zur.contracts.service")
        .with_operation("validate")
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn service_key(value: &str) -> ServiceKey {
        ServiceKey::new(value)
            .expect("test service identity must be valid")
    }

    fn service_version(
        value: &str,
    ) -> ServiceContractVersion {
        value
            .parse()
            .expect("test service version must be valid")
    }

    fn version_range(
        minimum: &str,
        maximum: &str,
    ) -> VersionRange<ServiceContractVersion> {
        VersionRange::new(
            service_version(minimum),
            service_version(maximum),
        )
        .expect("test version range must be valid")
    }

    fn descriptor() -> ServiceDescriptor {
        ServiceDescriptor::new(
            service_key("zamani.test.executor"),
            service_version("1.0.0"),
        )
    }

    // ------------------------------------------------------------------------
    // IDENTITY
    // ------------------------------------------------------------------------

    #[test]
    fn service_identity_is_preserved() {
        let service = descriptor();

        assert_eq!(
            service.identity().as_str(),
            "zamani.test.executor"
        );
    }

    #[test]
    fn service_version_is_preserved() {
        let service = descriptor();

        assert_eq!(
            service.version().to_string(),
            "1.0.0"
        );
    }

    // ------------------------------------------------------------------------
    // LIFECYCLE
    // ------------------------------------------------------------------------

    #[test]
    fn default_lifecycle_is_on_demand() {
        let service = descriptor();

        assert_eq!(
            service.lifecycle().startup(),
            StartupRequirement::OnDemand
        );
    }

    #[test]
    fn default_readiness_is_explicit() {
        let service = descriptor();

        assert_eq!(
            service.lifecycle().readiness(),
            ReadinessRequirement::Explicit
        );
    }

    #[test]
    fn default_shutdown_is_graceful() {
        let service = descriptor();

        assert_eq!(
            service.lifecycle().shutdown(),
            ShutdownRequirement::Graceful
        );
    }

    #[test]
    fn lifecycle_can_be_configured() {
        let lifecycle = ServiceLifecycleRequirements::new(
            StartupRequirement::BeforeExecution,
            ReadinessRequirement::InitializationComplete,
            ShutdownRequirement::ImmediateAllowed,
        );

        let service = descriptor().with_lifecycle(lifecycle);

        assert_eq!(
            service.lifecycle().startup(),
            StartupRequirement::BeforeExecution
        );

        assert_eq!(
            service.lifecycle().readiness(),
            ReadinessRequirement::InitializationComplete
        );

        assert_eq!(
            service.lifecycle().shutdown(),
            ShutdownRequirement::ImmediateAllowed
        );
    }

    // ------------------------------------------------------------------------
    // DEPENDENCIES
    // ------------------------------------------------------------------------

    #[test]
    fn required_dependency_can_be_added() {
        let dependency = ServiceDependency::required(
            version_range("1.0.0", "2.0.0"),
        );

        let service = descriptor()
            .with_dependency(
                service_key("zamani.core.memory"),
                dependency,
            )
            .expect("dependency should be accepted");

        assert_eq!(service.dependency_count(), 1);

        assert!(
            service.has_dependencies()
        );
    }

    #[test]
    fn optional_dependency_can_be_added() {
        let dependency = ServiceDependency::optional(
            version_range("1.0.0", "1.9.0"),
        );

        let service = descriptor()
            .with_dependency(
                service_key("zamani.core.observability"),
                dependency,
            )
            .expect("optional dependency should be accepted");

        let key = service_key("zamani.core.observability");

        let registered = service
            .dependency(&key)
            .expect("dependency should exist");

        assert!(
            registered.requirement().is_optional()
        );
    }

    #[test]
    fn required_dependency_is_classified_correctly() {
        let dependency = ServiceDependency::required(
            version_range("1.0.0", "1.0.0"),
        );

        assert!(
            dependency.requirement().is_required()
        );
    }

    #[test]
    fn self_dependency_is_rejected() {
        let identity = service_key("zamani.test.executor");

        let mut service = ServiceDescriptor::new(
            identity.clone(),
            service_version("1.0.0"),
        );

        let dependency = ServiceDependency::required(
            version_range("1.0.0", "1.0.0"),
        );

        assert!(
            service.add_dependency(identity, dependency).is_err()
        );
    }

    #[test]
    fn duplicate_dependency_is_rejected() {
        let key = service_key("zamani.core.memory");

        let dependency = ServiceDependency::required(
            version_range("1.0.0", "2.0.0"),
        );

        let mut service = descriptor();

        service
            .add_dependency(
                key.clone(),
                dependency.clone(),
            )
            .expect("first dependency should be accepted");

        assert!(
            service.add_dependency(key, dependency).is_err()
        );
    }

    #[test]
    fn dependencies_are_deterministically_ordered() {
        let service = descriptor()
            .with_dependency(
                service_key("zamani.z.service"),
                ServiceDependency::required(
                    version_range("1.0.0", "1.0.0"),
                ),
            )
            .expect("dependency should be accepted")
            .with_dependency(
                service_key("zamani.a.service"),
                ServiceDependency::required(
                    version_range("1.0.0", "1.0.0"),
                ),
            )
            .expect("dependency should be accepted");

        let keys: Vec<&str> = service
            .dependencies()
            .keys()
            .map(ServiceKey::as_str)
            .collect();

        assert_eq!(
            keys,
            vec![
                "zamani.a.service",
                "zamani.z.service",
            ]
        );
    }

    #[test]
    fn dependency_can_be_removed() {
        let key = service_key("zamani.core.memory");

        let mut service = descriptor()
            .with_dependency(
                key.clone(),
                ServiceDependency::required(
                    version_range("1.0.0", "1.0.0"),
                ),
            )
            .expect("dependency should be accepted");

        assert!(
            service.remove_dependency(&key).is_some()
        );

        assert_eq!(service.dependency_count(), 0);
    }

    // ------------------------------------------------------------------------
    // VERSION COMPATIBILITY
    // ------------------------------------------------------------------------

    #[test]
    fn compatible_dependency_version_is_accepted() {
        let dependency = ServiceDependency::required(
            version_range("1.0.0", "2.0.0"),
        );

        assert!(
            dependency.accepts_version(
                &service_version("1.5.0")
            )
        );
    }

    #[test]
    fn incompatible_dependency_version_is_rejected() {
        let dependency = ServiceDependency::required(
            version_range("1.0.0", "2.0.0"),
        );

        assert!(
            !dependency.accepts_version(
                &service_version("3.0.0")
            )
        );
    }

    #[test]
    fn version_range_endpoints_are_inclusive() {
        let dependency = ServiceDependency::required(
            version_range("1.0.0", "2.0.0"),
        );

        assert!(
            dependency.accepts_version(
                &service_version("1.0.0")
            )
        );

        assert!(
            dependency.accepts_version(
                &service_version("2.0.0")
            )
        );
    }

    // ------------------------------------------------------------------------
    // DESCRIPTOR VALIDATION
    // ------------------------------------------------------------------------

    #[test]
    fn empty_descriptor_is_valid() {
        let service = descriptor();

        assert!(
            service.validate().is_ok()
        );
    }

    #[test]
    fn descriptor_with_dependencies_is_valid() {
        let service = descriptor()
            .with_dependency(
                service_key("zamani.core.memory"),
                ServiceDependency::required(
                    version_range("1.0.0", "2.0.0"),
                ),
            )
            .expect("dependency should be accepted");

        assert!(
            service.validate().is_ok()
        );
    }

    #[test]
    fn descriptor_can_be_cloned_without_losing_contract_information() {
        let service = descriptor()
            .with_dependency(
                service_key("zamani.core.memory"),
                ServiceDependency::required(
                    version_range("1.0.0", "2.0.0"),
                ),
            )
            .expect("dependency should be accepted");

        let cloned = service.clone();

        assert_eq!(service, cloned);
    }

    #[test]
    fn descriptor_validation_does_not_claim_availability() {
        let service = descriptor();

        // Descriptor validity only establishes internal consistency.
        // It does not establish service registration or availability.
        assert!(service.validate().is_ok());
    }
}
