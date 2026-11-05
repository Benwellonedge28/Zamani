
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Backend Discovery
//!
//! Path: src/compiler/backends/discovery.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Integrate the canonical backend registry with compiler-side backend
//! inventory and the existing canonical target-discovery subsystem.
//!
//! This module provides:
//!
//! 1. Discovery of registered backend descriptors.
//! 2. Filtering of backend inventories using caller-supplied criteria.
//! 3. Explicit backend implementation probing.
//! 4. An adapter allowing registered backends to contribute target
//!    descriptors to the canonical target-discovery subsystem.
//!
//! # Architectural ownership
//!
//! Canonical backend registration:
//!     crate::backend::registry
//!
//! Compiler-facing backend registry:
//!     crate::compiler::backends::registry
//!
//! Target descriptors:
//!     crate::compiler::configuration::target
//!
//! Target discovery:
//!     crate::compiler::planning::target_discovery
//!
//! Capability analysis:
//!     crate::compiler::planning::capability_matching
//!
//! Target selection:
//!     crate::backend::selection
//!
//! Resource feasibility:
//!     crate::compiler::planning
//!
//! Runtime execution:
//!     crate::runtime
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Backend discovery requests.
//! - Backend discovery reports.
//! - Backend inventory filtering.
//! - Explicit backend implementation probing.
//! - Backend-to-target discovery adaptation.
//! - Discovery integration errors.
//!
//! This module does not own:
//!
//! - Backend registration.
//! - Backend factories.
//! - Target descriptor definitions.
//! - Target discovery provider registration.
//! - Capability definitions.
//! - Capability evidence.
//! - Hardware inspection.
//! - Physical resource allocation.
//! - Resource feasibility.
//! - Backend selection.
//! - IR lowering.
//! - Artifact generation.
//! - Runtime execution.
//!
//! # Important distinction
//!
//! A registered backend is not necessarily an available physical device.
//!
//! A constructible backend is not necessarily an executable toolchain.
//!
//! A discovered target is not necessarily compatible with a workload.
//!
//! A compatible target is not necessarily resource-feasible.
//!
//! A feasible plan is not proof that physical execution succeeded.
//!
//! These distinctions must remain explicit throughout compilation.
//!
//! # Scalability
//!
//! No fixed backend count, architecture count, accelerator count,
//! provider count, or target count is introduced here.
//!
//! Collections grow according to available resources.
//!
//! Operational budgets belong to the appropriate compilation and
//! discovery orchestration contracts.
//!
//! Literal infinite computation is not claimed.
//!
//! # Determinism
//!
//! Backend descriptors are obtained from the canonical registry,
//! which provides deterministic identifier ordering.
//!
//! Filtering preserves that ordering.
//!
//! # Thread safety
//!
//! Canonical backend registry synchronization is inherited.
//!
//! This module introduces no global mutable registry.
//!
//! Target discovery remains instance-owned.
//!
//! # Failure behavior
//!
//! Registry failures are propagated.
//!
//! Factory construction failures are never interpreted as successful
//! physical hardware discovery.
//!
//! Missing target metadata is never fabricated.
//!
//! Unknown capability information is never interpreted as support.
//!
//! Conflicting target identities are handled by the canonical
//! target-discovery subsystem.
//!
//! # Integration
//!
//! Parent module:
//!     src/compiler/backends/mod.rs
//!
//! Required declaration:
//!     pub mod discovery;
//!
//! Required sibling:
//!     pub mod registry;
//!
//! Existing planning module:
//!     crate::compiler::planning::target_discovery
//!
//! No competing registry or discovery engine is introduced.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::backend::registry::{
    BackendDescriptor,
    BackendRegistry,
    RegistryError,
};

use crate::compiler::planning::target_discovery::{
    DiscoveryError,
    DiscoveryProviderId,
    DiscoveryRequest,
    ProviderFailure,
    TargetDiscoveryProvider,
};

use crate::compiler::configuration::target::{
    TargetDescriptor,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable backend discovery module identity.
pub const BACKEND_DISCOVERY_MODULE_NAME: &str =
    "zamani.compiler.backends.discovery";

/// Backend discovery API version.
pub const BACKEND_DISCOVERY_API_VERSION: u16 = 1;

// ============================================================================
// DISCOVERY REQUEST
// ============================================================================

/// Criteria used to discover registered backend implementations.
///
/// An empty filter means unrestricted discovery for that dimension.
///
/// When multiple filters are supplied, all filters must match.
///
/// Identifiers are compared using their canonical, case-sensitive
/// representations.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BackendDiscoveryRequest {
    backend_ids: BTreeSet<String>,
    file_extensions: BTreeSet<String>,
}

impl BackendDiscoveryRequest {
    /// Create an unrestricted backend discovery request.
    #[must_use]
    pub fn all() -> Self {
        Self::default()
    }

    /// Restrict discovery to specified backend identifiers.
    pub fn with_backend_ids<I, S>(
        mut self,
        identifiers: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.backend_ids = identifiers
            .into_iter()
            .map(Into::into)
            .collect();

        self
    }

    /// Restrict discovery to specified output extensions.
    pub fn with_file_extensions<I, S>(
        mut self,
        extensions: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.file_extensions = extensions
            .into_iter()
            .map(Into::into)
            .collect();

        self
    }

    /// Return requested backend identifiers.
    #[must_use]
    pub fn backend_ids(&self) -> &BTreeSet<String> {
        &self.backend_ids
    }

    /// Return requested output extensions.
    #[must_use]
    pub fn file_extensions(&self) -> &BTreeSet<String> {
        &self.file_extensions
    }

    /// Determine whether a descriptor matches the request.
    #[must_use]
    pub fn accepts(
        &self,
        descriptor: &BackendDescriptor,
    ) -> bool {
        let identifier_matches =
            self.backend_ids.is_empty()
                || self.backend_ids.contains(&descriptor.id);

        let extension_matches =
            self.file_extensions.is_empty()
                || self.file_extensions.contains(
                    &descriptor.file_extension,
                );

        identifier_matches && extension_matches
    }

    /// Whether the request contains no restrictions.
    #[must_use]
    pub fn is_unrestricted(&self) -> bool {
        self.backend_ids.is_empty()
            && self.file_extensions.is_empty()
    }
}

// ============================================================================
// DISCOVERY REPORT
// ============================================================================

/// Immutable snapshot of discovered registered backend metadata.
///
/// The snapshot contains cloned descriptors, not backend instances.
///
/// It therefore does not hold registry locks or backend implementation
/// resources after discovery completes.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BackendDiscoveryReport {
    descriptors: Vec<BackendDescriptor>,

    scanned: usize,
}

impl BackendDiscoveryReport {
    /// Create an empty report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return discovered backend descriptors.
    ///
    /// Results are in canonical backend identifier order.
    #[must_use]
    pub fn descriptors(&self) -> &[BackendDescriptor] {
        &self.descriptors
    }

    /// Return a discovered descriptor by canonical identifier.
    #[must_use]
    pub fn find(
        &self,
        identifier: &str,
    ) -> Option<&BackendDescriptor> {
        self.descriptors
            .binary_search_by(|descriptor| {
                descriptor.id.as_str().cmp(identifier)
            })
            .ok()
            .map(|index| &self.descriptors[index])
    }

    /// Return the number of registered descriptors examined.
    #[must_use]
    pub fn scanned_count(&self) -> usize {
        self.scanned
    }

    /// Return the number of matching backends.
    #[must_use]
    pub fn matched_count(&self) -> usize {
        self.descriptors.len()
    }

    /// Whether the discovery returned no matching backends.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.descriptors.is_empty()
    }
}

// ============================================================================
// DISCOVERY ERRORS
// ============================================================================

/// Errors produced by backend discovery and probing.
#[derive(Debug)]
#[non_exhaustive]
pub enum BackendDiscoveryError {
    /// Canonical registry operation failed.
    Registry(RegistryError),

    /// Backend factory output did not match its registered metadata.
    ImplementationMetadataMismatch {
        backend_id: String,
        registered_extension: String,
        implementation_extension: String,
    },
}

impl fmt::Display for BackendDiscoveryError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Registry(error) => {
                write!(
                    formatter,
                    "backend discovery registry failure: {error}"
                )
            }

            Self::ImplementationMetadataMismatch {
                backend_id,
                registered_extension,
                implementation_extension,
            } => {
                write!(
                    formatter,
                    "backend `{backend_id}` declares extension \
                     `{registered_extension}` but its implementation \
                     reports `{implementation_extension}`"
                )
            }
        }
    }
}

impl Error for BackendDiscoveryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Registry(error) => Some(error),
            Self::ImplementationMetadataMismatch { .. } => None,
        }
    }
}

impl From<RegistryError> for BackendDiscoveryError {
    fn from(error: RegistryError) -> Self {
        Self::Registry(error)
    }
}

// ============================================================================
// BACKEND DISCOVERY SERVICE
// ============================================================================

/// Stateless service for inspecting the canonical backend registry.
///
/// The service never creates another registry.
///
/// All discovery operations use the registry supplied by the caller.
pub struct BackendDiscovery;

impl BackendDiscovery {
    /// Return the stable module identity.
    #[must_use]
    pub const fn module_name() -> &'static str {
        BACKEND_DISCOVERY_MODULE_NAME
    }

    /// Return the API version.
    #[must_use]
    pub const fn api_version() -> u16 {
        BACKEND_DISCOVERY_API_VERSION
    }

    /// Discover registered backend descriptors.
    ///
    /// This operation only inspects registered metadata.
    ///
    /// It does not instantiate implementations, invoke toolchains,
    /// inspect physical devices, or claim target availability.
    ///
    /// # Errors
    ///
    /// Returns the original canonical registry error if its state
    /// cannot be inspected.
    pub fn discover(
        registry: &BackendRegistry,
        request: &BackendDiscoveryRequest,
    ) -> Result<
        BackendDiscoveryReport,
        BackendDiscoveryError,
    > {
        let descriptors = registry.descriptors()?;

        let scanned = descriptors.len();

        let descriptors = descriptors
            .into_iter()
            .filter(|descriptor| request.accepts(descriptor))
            .collect();

        Ok(BackendDiscoveryReport {
            descriptors,
            scanned,
        })
    }

    /// Discover one backend by canonical identifier or alias.
    ///
    /// Alias resolution is delegated to the canonical registry.
    pub fn discover_one(
        registry: &BackendRegistry,
        identifier: &str,
    ) -> Result<BackendDescriptor, BackendDiscoveryError> {
        Ok(registry.descriptor(identifier)?)
    }

    /// Explicitly construct and inspect a registered backend.
    ///
    /// This invokes the registered factory.
    ///
    /// Factory construction is not equivalent to:
    ///
    /// - Physical device discovery.
    /// - Toolchain availability verification.
    /// - Compilation success.
    /// - Artifact verification.
    /// - Execution availability.
    ///
    /// The canonical registry verifies the backend identity.
    /// This function additionally verifies output-extension agreement.
    ///
    /// Callers must invoke this method only when factory construction
    /// is an appropriate operation for the backend.
    pub fn probe(
        registry: &BackendRegistry,
        identifier: &str,
    ) -> Result<BackendProbeReport, BackendDiscoveryError> {
        let descriptor = registry.descriptor(identifier)?;

        let implementation = registry.create(identifier)?;

        let implementation_extension =
            implementation.file_extension().to_owned();

        if implementation_extension != descriptor.file_extension {
            return Err(
                BackendDiscoveryError::ImplementationMetadataMismatch {
                    backend_id: descriptor.id.clone(),
                    registered_extension:
                        descriptor.file_extension.clone(),
                    implementation_extension,
                },
            );
        }

        Ok(BackendProbeReport {
            backend_id: descriptor.id,
            target_name: implementation.target_name().to_owned(),
            file_extension: implementation.file_extension().to_owned(),
        })
    }
}

// ============================================================================
// BACKEND PROBE RESULT
// ============================================================================

/// Result of explicitly constructing a registered backend.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BackendProbeReport {
    /// Canonical backend identifier.
    pub backend_id: String,

    /// Implementation-reported target name.
    pub target_name: String,

    /// Implementation-reported output extension.
    pub file_extension: String,
}

// ============================================================================
// EXPLICIT BACKEND-TO-TARGET MAPPING
// ============================================================================

/// Explicit metadata adapter for a registered backend.
///
/// Backend registry metadata does not contain enough information to
/// manufacture a valid target descriptor.
///
/// Consequently, target information must be supplied by an authoritative
/// backend integration.
///
/// This callback must return only target descriptors supported by
/// verifiable metadata from the backend, toolchain, hardware inventory,
/// or another trusted source.
///
/// It must not invent architecture, ABI, capabilities, or resources.
pub type BackendTargetMapper = Arc<
    dyn Fn(
            &BackendDescriptor,
            &DiscoveryRequest,
        ) -> Result<Vec<TargetDescriptor>, String>
        + Send
        + Sync
        + 'static,
>;

/// Discovery provider adapting registered backends into the canonical
/// compiler target-discovery subsystem.
///
/// This provider does not own or duplicate target discovery.
///
/// It implements the existing `TargetDiscoveryProvider` contract.
pub struct RegisteredBackendTargetProvider {
    id: DiscoveryProviderId,

    registry: BackendRegistry,

    mapper: BackendTargetMapper,
}

impl RegisteredBackendTargetProvider {
    /// Construct an adapter using an explicit backend-to-target mapper.
    ///
    /// The backend registry is shared with its original owner.
    ///
    /// No target descriptor is fabricated by this constructor.
    pub fn new<F>(
        provider_id: impl Into<String>,
        registry: BackendRegistry,
        mapper: F,
    ) -> Result<Self, DiscoveryError>
    where
        F: Fn(
                &BackendDescriptor,
                &DiscoveryRequest,
            ) -> Result<Vec<TargetDescriptor>, String>
            + Send
            + Sync
            + 'static,
    {
        Ok(Self {
            id: DiscoveryProviderId::new(provider_id)?,
            registry,
            mapper: Arc::new(mapper),
        })
    }
}

impl TargetDiscoveryProvider for RegisteredBackendTargetProvider {
    fn id(&self) -> &DiscoveryProviderId {
        &self.id
    }

    fn discover(
        &self,
        request: &DiscoveryRequest,
    ) -> Result<Vec<TargetDescriptor>, ProviderFailure> {
        let descriptors = self
            .registry
            .descriptors()
            .map_err(|error| {
                ProviderFailure::new(
                    self.id.clone(),
                    format!(
                        "canonical backend registry inspection failed: {error}"
                    ),
                )
            })?;

        let mut targets = Vec::new();

        for descriptor in &descriptors {
            let discovered = (self.mapper)(descriptor, request)
                .map_err(|error| {
                    ProviderFailure::new(
                        self.id.clone(),
                        format!(
                            "target metadata mapping failed for backend \
                             `{}`: {error}",
                            descriptor.id
                        ),
                    )
                })?;

            targets.extend(discovered);
        }

        Ok(targets)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            BackendDiscovery::module_name(),
            BACKEND_DISCOVERY_MODULE_NAME,
        );

        assert_ne!(
            BackendDiscovery::api_version(),
            0,
        );
    }

    #[test]
    fn unrestricted_request_accepts_arbitrary_backend_metadata() {
        let request = BackendDiscoveryRequest::all();

        assert!(request.is_unrestricted());
    }

    #[test]
    fn backend_identifier_filter_is_exact() {
        let request = BackendDiscoveryRequest::all()
            .with_backend_ids(["vendor.custom"]);

        let matching = BackendDescriptor::new(
            "vendor.custom",
            ".custom",
        );

        let nonmatching = BackendDescriptor::new(
            "vendor.other",
            ".custom",
        );

        assert!(request.accepts(&matching));
        assert!(!request.accepts(&nonmatching));
    }

    #[test]
    fn extension_filter_is_exact() {
        let request = BackendDiscoveryRequest::all()
            .with_file_extensions([".custom"]);

        let matching = BackendDescriptor::new(
            "vendor.custom",
            ".custom",
        );

        let nonmatching = BackendDescriptor::new(
            "vendor.custom",
            ".other",
        );

        assert!(request.accepts(&matching));
        assert!(!request.accepts(&nonmatching));
    }

    #[test]
    fn multiple_filters_are_combined() {
        let request = BackendDiscoveryRequest::all()
            .with_backend_ids(["vendor.custom"])
            .with_file_extensions([".custom"]);

        let matching = BackendDescriptor::new(
            "vendor.custom",
            ".custom",
        );

        let wrong_extension = BackendDescriptor::new(
            "vendor.custom",
            ".other",
        );

        let wrong_identifier = BackendDescriptor::new(
            "vendor.other",
            ".custom",
        );

        assert!(request.accepts(&matching));
        assert!(!request.accepts(&wrong_extension));
        assert!(!request.accepts(&wrong_identifier));
    }

    #[test]
    fn empty_registry_produces_valid_empty_report() {
        let registry = BackendRegistry::new();

        let report = BackendDiscovery::discover(
            &registry,
            &BackendDiscoveryRequest::all(),
        )
        .expect("empty registry discovery must succeed");

        assert!(report.is_empty());
        assert_eq!(report.scanned_count(), 0);
        assert_eq!(report.matched_count(), 0);
    }

    #[test]
    fn discovery_report_supports_empty_inventory() {
        let report = BackendDiscoveryReport::new();

        assert!(report.is_empty());
        assert_eq!(report.matched_count(), 0);
        assert_eq!(report.scanned_count(), 0);
        assert!(report.find("unknown").is_none());
    }

    #[test]
    fn canonical_backend_registry_is_shared() {
        let first = BackendRegistry::new();
        let second = first.clone();

        assert_eq!(
            first.len().expect("registry must be readable"),
            second.len().expect("registry must be readable"),
        );
    }

    #[test]
    fn discovery_does_not_claim_physical_availability() {
        let registry = BackendRegistry::new();

        let report = BackendDiscovery::discover(
            &registry,
            &BackendDiscoveryRequest::all(),
        )
        .expect("discovery should succeed");

        // An empty backend inventory is a valid discovery result.
        // It does not assert that no hardware exists.
        assert!(report.is_empty());
    }
}
