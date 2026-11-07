
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Backend Discovery
//!
//! Path: `src/compiler/backends/discovery.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Provides compiler-facing discovery of registered backend metadata and
//! an adapter connecting the canonical backend registry to the canonical
//! compiler target-discovery subsystem.
//!
//! This module is an integration layer. It does not create or own another
//! backend registry, target registry, capability registry, compatibility
//! engine, or physical hardware inventory.
//!
//! ## Canonical authorities
//!
//! Backend implementation:
//!     crate::backend
//!
//! Backend registration:
//!     crate::backend::registry
//!
//! Backend capability evidence:
//!     crate::backend::capabilities
//!
//! Backend compatibility:
//!     crate::backend::compatibility
//!
//! Compiler target descriptors:
//!     crate::compiler::configuration::target
//!
//! Target discovery:
//!     crate::compiler::planning::target_discovery
//!
//! Backend selection:
//!     crate::compiler::backends::selection
//!
//! Resource feasibility:
//!     crate::compiler::planning
//!
//! Runtime execution:
//!     crate::runtime
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Backend discovery requests.
//! - Backend discovery reports.
//! - Backend descriptor filtering.
//! - Explicit backend implementation probing.
//! - Backend-to-target discovery adaptation.
//! - Discovery integration errors.
//!
//! This module does not own:
//!
//! - Backend registration or factories.
//! - Backend implementation traits.
//! - Target descriptor definitions.
//! - Target provider registration.
//! - Capability definitions or evidence.
//! - Compatibility evaluation.
//! - Physical device discovery.
//! - Physical resource allocation.
//! - Resource feasibility.
//! - Backend selection.
//! - IR construction or lowering.
//! - Artifact generation or verification.
//! - Runtime execution.
//!
//! ## Important distinctions
//!
//! Registration is not implementation availability.
//!
//! Implementation construction is not physical device discovery.
//!
//! Target discovery is not capability compatibility.
//!
//! Capability compatibility is not resource feasibility.
//!
//! Resource feasibility is not physical resource reservation.
//!
//! Successful code generation is not artifact verification.
//!
//! Artifact verification is not successful execution.
//!
//! Discovery must never promote unknown evidence into positive evidence.
//!
//! ## Scalability
//!
//! No fixed maximum is imposed on:
//!
//! - Backends.
//! - Providers.
//! - Targets.
//! - Architectures.
//! - Computational domains.
//! - Accelerators.
//! - Devices.
//! - Distributed participants.
//! - Resource quantities.
//!
//! Operational budgets belong to compiler configuration and orchestration.
//! Physical capacity belongs to the actual target and its providers.
//!
//! Literal infinite computation is not claimed.
//!
//! ## Determinism
//!
//! Backend descriptors are supplied by the canonical registry in canonical
//! identifier order. Discovery filtering preserves that order.
//!
//! Target descriptor aggregation and conflict handling belong to the
//! canonical target-discovery subsystem.
//!
//! ## Concurrency
//!
//! Backend registry synchronization is inherited from BackendRegistry.
//! Target provider registration is owned by TargetDiscoveryRegistry.
//!
//! This module introduces no global mutable state.
//!
//! ## Failure behavior
//!
//! Registry failures are propagated.
//! Factory construction failures are propagated.
//! Invalid mapped target descriptors are rejected.
//! Mapper failures are reported with their backend identity.
//! Missing metadata is never fabricated.
//! A failed probe is never interpreted as successful availability.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::backend::registry::{
    BackendDescriptor,
    BackendRegistry,
    RegistryError,
};

use crate::compiler::configuration::target::{
    TargetDescriptor,
};

use crate::compiler::planning::target_discovery::{
    DiscoveryError,
    DiscoveryProviderId,
    DiscoveryRequest,
    ProviderFailure,
    TargetDiscoveryProvider,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable backend discovery module identity.
pub const BACKEND_DISCOVERY_MODULE_NAME: &str =
    "zamani.compiler.backends.discovery";

/// Backend discovery API version.
pub const BACKEND_DISCOVERY_API_VERSION: u16 = 2;

// ============================================================================
// DISCOVERY REQUEST
// ============================================================================

/// Criteria for inspecting registered backend descriptors.
///
/// All configured filters are combined using logical AND.
///
/// Within an individual filter, matching uses exact, case-sensitive
/// canonical identifiers or extension strings.
///
/// An empty filter means unrestricted discovery for that dimension.
///
/// Backend identifiers in this request are canonical identifiers.
/// Alias resolution is available through [`BackendDiscovery::discover_one`].
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

    /// Restrict discovery to canonical backend identifiers.
    ///
    /// Duplicate identifiers are removed deterministically.
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

    /// Restrict discovery to advertised output extensions.
    ///
    /// Extension matching is exact and case-sensitive.
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

    /// Return requested canonical backend identifiers.
    #[must_use]
    pub fn backend_ids(&self) -> &BTreeSet<String> {
        &self.backend_ids
    }

    /// Return requested output extensions.
    #[must_use]
    pub fn file_extensions(&self) -> &BTreeSet<String> {
        &self.file_extensions
    }

    /// Determine whether a descriptor satisfies every configured filter.
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

    /// Return whether the request has no restrictions.
    #[must_use]
    pub fn is_unrestricted(&self) -> bool {
        self.backend_ids.is_empty()
            && self.file_extensions.is_empty()
    }
}

// ============================================================================
// DISCOVERY REPORT
// ============================================================================

/// Immutable snapshot of discovered backend metadata.
///
/// Descriptors are cloned from the canonical registry.
///
/// The report does not retain backend instances or registry locks.
///
/// The descriptor collection is ordered by canonical backend identifier.
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

    /// Return discovered descriptors in canonical identifier order.
    #[must_use]
    pub fn descriptors(&self) -> &[BackendDescriptor] {
        &self.descriptors
    }

    /// Find a descriptor by canonical identifier.
    ///
    /// Aliases are not searched here. Use the canonical registry for
    /// alias resolution.
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

    /// Return the number of matching descriptors.
    #[must_use]
    pub fn matched_count(&self) -> usize {
        self.descriptors.len()
    }

    /// Return whether no descriptors matched.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.descriptors.is_empty()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by backend discovery and implementation probing.
#[derive(Debug)]
#[non_exhaustive]
pub enum BackendDiscoveryError {
    /// An operation on the canonical backend registry failed.
    Registry(RegistryError),

    /// The implementation's extension differs from registered metadata.
    ImplementationMetadataMismatch {
        /// Canonical backend identifier.
        backend_id: String,

        /// Extension recorded in the canonical registry.
        registered_extension: String,

        /// Extension reported by the constructed implementation.
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
/// The service never creates or owns another registry.
///
/// Every operation receives the authoritative registry explicitly.
pub struct BackendDiscovery;

impl BackendDiscovery {
    /// Return the stable module identity.
    #[must_use]
    pub const fn module_name() -> &'static str {
        BACKEND_DISCOVERY_MODULE_NAME
    }

    /// Return the backend discovery API version.
    #[must_use]
    pub const fn api_version() -> u16 {
        BACKEND_DISCOVERY_API_VERSION
    }

    /// Discover registered backend descriptors.
    ///
    /// This operation inspects registered metadata only.
    ///
    /// It does not:
    ///
    /// - Construct backend implementations.
    /// - Invoke external toolchains.
    /// - Inspect physical devices.
    /// - Query runtime availability.
    /// - Evaluate capabilities.
    /// - Evaluate compatibility.
    /// - Allocate resources.
    ///
    /// The canonical registry supplies descriptors in deterministic
    /// identifier order. Filtering preserves that ordering.
    ///
    /// # Errors
    ///
    /// Returns the original canonical registry error if registry
    /// inspection fails.
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

    /// Discover one backend by canonical identifier or registered alias.
    ///
    /// Alias resolution is delegated to the canonical registry.
    ///
    /// The returned descriptor always contains its canonical identifier.
    ///
    /// # Errors
    ///
    /// Returns the canonical registry error when the identifier cannot
    /// be resolved or the registry cannot be inspected.
    pub fn discover_one(
        registry: &BackendRegistry,
        identifier: &str,
    ) -> Result<BackendDescriptor, BackendDiscoveryError> {
        Ok(registry.descriptor(identifier)?)
    }

    /// Explicitly construct and inspect a registered backend.
    ///
    /// This invokes the registered backend factory.
    ///
    /// The canonical registry verifies that the constructed implementation
    /// reports the registered canonical backend identity.
    ///
    /// This method additionally verifies output-extension agreement.
    ///
    /// Construction does not establish:
    ///
    /// - Physical device availability.
    /// - Toolchain availability.
    /// - Target compatibility.
    /// - Resource feasibility.
    /// - Compilation success.
    /// - Artifact validity.
    /// - Execution availability.
    ///
    /// Callers must use this method only when backend construction is
    /// appropriate. Factories should not perform unexpected external
    /// side effects.
    ///
    /// # Errors
    ///
    /// Returns registry errors or implementation metadata mismatches.
    pub fn probe(
        registry: &BackendRegistry,
        identifier: &str,
    ) -> Result<BackendProbeReport, BackendDiscoveryError> {
        // Resolve first so the report and subsequent operations use the
        // same canonical backend identity.
        let canonical_id = registry.resolve(identifier)?;

        let descriptor = registry.descriptor(&canonical_id)?;

        let implementation = registry.create(&canonical_id)?;

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
            file_extension: implementation_extension,
        })
    }
}

// ============================================================================
// PROBE REPORT
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
// BACKEND-TO-TARGET MAPPING
// ============================================================================

/// Callback that maps authoritative backend metadata into target metadata.
///
/// Backend registry descriptors do not contain enough information to
/// manufacture a complete target descriptor.
///
/// Implementations must obtain target information from authoritative
/// metadata such as:
///
/// - A backend target manifest.
/// - A verified toolchain description.
/// - A hardware inventory provider.
/// - A deployment inventory.
/// - A trusted remote target provider.
///
/// A mapper must not infer architecture, ABI, capabilities, or resources
/// from a backend name, alias, description, or output extension.
///
/// The callback receives the canonical backend descriptor and the
/// canonical target discovery request.
///
/// It must return only target descriptors supported by evidence.
pub type BackendTargetMapper = Arc<
    dyn Fn(
            &BackendDescriptor,
            &DiscoveryRequest,
        ) -> Result<Vec<TargetDescriptor>, String>
        + Send
        + Sync
        + 'static,
>;

// ============================================================================
// REGISTERED BACKEND TARGET PROVIDER
// ============================================================================

/// Adapts the canonical backend registry into the canonical target
/// discovery subsystem.
///
/// This is a provider adapter, not a second target discovery engine.
///
/// It does not own target descriptor identity, target provider registration,
/// target aggregation, duplicate detection, or target conflict resolution.
///
/// Those responsibilities remain with
/// `crate::compiler::planning::target_discovery`.
pub struct RegisteredBackendTargetProvider {
    id: DiscoveryProviderId,
    registry: BackendRegistry,
    mapper: BackendTargetMapper,
}

impl RegisteredBackendTargetProvider {
    /// Construct a backend-to-target discovery adapter.
    ///
    /// The supplied registry is cloned by shared ownership. No independent
    /// registry is created.
    ///
    /// The mapper is mandatory because backend registration metadata alone
    /// cannot establish a target architecture, ABI, capability profile,
    /// resource capacity, or physical device.
    ///
    /// # Errors
    ///
    /// Returns a target discovery error when the provider identity is
    /// invalid.
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

    /// Return this provider's stable identity.
    #[must_use]
    pub fn provider_id(&self) -> &DiscoveryProviderId {
        &self.id
    }
}

impl TargetDiscoveryProvider for RegisteredBackendTargetProvider {
    fn id(&self) -> &DiscoveryProviderId {
        &self.id
    }

    /// Discover targets described by registered backend integrations.
    ///
    /// The provider takes a snapshot of backend descriptors before invoking
    /// mapper callbacks. No backend registry lock is held while callbacks
    /// execute.
    ///
    /// The canonical target discovery subsystem remains responsible for
    /// request filtering, descriptor validation, duplicate detection,
    /// provenance aggregation, and conflict reporting.
    ///
    /// A mapper failure is returned as a provider failure. The existing
    /// TargetDiscoveryProvider contract reports failure at provider scope;
    /// per-backend partial failures require a future extension to that
    /// canonical contract.
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
                        "canonical backend registry inspection failed: \
                         {error}"
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
                            "target mapping failed for backend `{}`: {error}",
                            descriptor.id
                        ),
                    )
                })?;

            // Validate here as well as in the canonical target discovery
            // subsystem. This makes the adapter's output contract explicit
            // and prevents invalid mapper results crossing this boundary.
            for target in discovered {
                target.validate().map_err(|error| {
                    ProviderFailure::new(
                        self.id.clone(),
                        format!(
                            "backend `{}` supplied invalid target `{}`: \
                             {error}",
                            descriptor.id,
                            target.id()
                        ),
                    )
                })?;

                targets.push(target);
            }
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
    }

    #[test]
    fn api_version_is_nonzero() {
        assert_ne!(
            BackendDiscovery::api_version(),
            0,
        );
    }

    #[test]
    fn unrestricted_request_has_no_filters() {
        let request = BackendDiscoveryRequest::all();

        assert!(request.is_unrestricted());
        assert!(request.backend_ids().is_empty());
        assert!(request.file_extensions().is_empty());
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
    fn multiple_filters_are_combined_with_and() {
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
    fn duplicate_filters_are_deduplicated() {
        let request = BackendDiscoveryRequest::all()
            .with_backend_ids([
                "vendor.custom",
                "vendor.custom",
            ]);

        assert_eq!(
            request.backend_ids().len(),
            1,
        );
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

        // No registered backend is a valid empty inventory result.
        // It does not prove that no physical hardware exists.
        assert!(report.is_empty());
    }
}
