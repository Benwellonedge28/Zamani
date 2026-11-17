
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Registry Discovery
//!
//! Path: `src/toolchain/registries/discovery.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Discovers configured registry instances and resolves their registered
//! provider implementations against explicit compatibility requirements.
//!
//! Supports:
//!
//! - Public registries.
//! - Private registries.
//! - Enterprise registries.
//! - Self-hosted registries.
//! - Federated registry instances.
//! - Local registry implementations.
//! - Third-party provider implementations.
//! - Future registry protocols and computational domains.
//!
//! No registry vendor, endpoint scheme, authentication mechanism,
//! protocol implementation, or computational domain is hard-coded.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - The in-process registry-provider catalog.
//! - Registry discovery requests.
//! - Provider implementation lookup.
//! - Provider and registry compatibility classification.
//! - Discovery result and diagnostic reporting.
//!
//! It does not own:
//!
//! - Registry configuration or lifecycle.
//! - Registry identity allocation.
//! - Manifest parsing.
//! - Registry persistence.
//! - Network discovery or transport.
//! - Authentication or authorization.
//! - Package resolution.
//! - Package downloading or publishing.
//! - Signature verification or trust decisions.
//! - Registry federation algorithms.
//! - Hardware discovery or allocation.
//!
//! ## CANONICAL AUTHORITIES
//!
//! Registry configuration:
//!     super::registry
//!
//! Provider contracts:
//!     super::provider
//!
//! Registry identities:
//!     crate::toolchain::ids
//!
//! Version contracts:
//!     crate::toolchain::version
//!
//! Structured errors:
//!     crate::toolchain::errors
//!
//! Manifest configuration:
//!     crate::zamani_project_config
//!
//! ## DISCOVERY GUARANTEES
//!
//! Discovery distinguishes:
//!
//! - Configured registry existence.
//! - Registry lifecycle eligibility.
//! - Provider implementation availability.
//! - Provider identity compatibility.
//! - Protocol compatibility.
//! - Declared operation support.
//!
//! Discovery does not claim that a remote registry is reachable,
//! authenticated, trusted, authorized, or operational.
//!
//! An available provider means only that a registered implementation
//! satisfies the locally declared discovery requirements.
//!
//! ## SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Registry instances.
//! - Provider implementations.
//! - Protocol versions.
//! - Operations.
//! - Computational domains.
//!
//! Ordered maps provide deterministic enumeration.
//! Shared immutable provider handles avoid unnecessary implementation
//! cloning.
//!
//! Actual memory and execution costs remain subject to available
//! resources and caller-configured operational budgets.
//!
//! Literal infinite computation or storage is not promised.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, RwLock};

use crate::toolchain::errors::{
    Result,
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::ids::ProviderId;

use crate::toolchain::version::RegistryProtocolVersion;

use super::provider::{
    RegistryOperationId,
    RegistryProvider,
};

use super::registry::{
    RegistryCatalog,
    RegistrySnapshot,
    RegistryStatus,
};

// ============================================================================
// OPERATION IDENTITIES
// ============================================================================

const OPERATION_PROVIDER_REGISTER: &str =
    "registry.discovery.provider.register";

const OPERATION_PROVIDER_LOOKUP: &str =
    "registry.discovery.provider.lookup";

const OPERATION_DISCOVER: &str =
    "registry.discovery.discover";

// ============================================================================
// SHARED PROVIDER HANDLE
// ============================================================================

/// Shared, thread-safe registry-provider implementation.
pub type SharedRegistryProvider = Arc<dyn RegistryProvider>;

// ============================================================================
// PROVIDER CATALOG
// ============================================================================

/// In-process catalog of registered registry-provider implementations.
///
/// This catalog is deliberately distinct from [`RegistryCatalog`].
///
/// `RegistryCatalog` stores configured registry instances.
///
/// `RegistryProviderCatalog` stores executable provider implementations.
///
/// A provider may serve multiple registry instances, and a registry
/// instance identifies exactly one configured provider.
///
/// The catalog does not persist across process restarts.
///
/// Provider execution is never performed while the catalog lock is held.
#[derive(Default)]
pub struct RegistryProviderCatalog {
    entries: RwLock<BTreeMap<ProviderId, SharedRegistryProvider>>,
}

impl fmt::Debug for RegistryProviderCatalog {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.entries.read() {
            Ok(entries) => formatter
                .debug_struct("RegistryProviderCatalog")
                .field("provider_count", &entries.len())
                .field("providers", &entries.keys().collect::<Vec<_>>())
                .finish(),
            Err(_) => formatter
                .debug_struct("RegistryProviderCatalog")
                .field("state", &"lock poisoned")
                .finish(),
        }
    }
}

impl RegistryProviderCatalog {
    /// Create an empty provider catalog.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a provider implementation.
    ///
    /// The provider descriptor is validated before registration.
    ///
    /// Duplicate provider identities are rejected.
    ///
    /// The provider implementation is retained through an `Arc`.
    ///
    /// Registration does not establish remote availability,
    /// authentication, authorization, or trust.
    pub fn register(
        &self,
        provider: SharedRegistryProvider,
    ) -> Result<()> {
        let descriptor = provider.descriptor();

        descriptor
            .validate()
            .map_err(|error| {
                error
                    .with_operation(OPERATION_PROVIDER_REGISTER)
                    .with_subject(
                        descriptor.provider_id().to_string(),
                    )
            })?;

        let id = descriptor.provider_id().clone();

        let mut entries = self.entries.write().map_err(|_| {
            provider_catalog_lock_error("register")
        })?;

        if entries.contains_key(&id) {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::AlreadyExists,
                    "A registry provider with this identity is already registered.",
                )
                .with_operation(OPERATION_PROVIDER_REGISTER)
                .with_subject(id.to_string()),
            );
        }

        entries.insert(id, provider);

        Ok(())
    }

    /// Retrieve a provider implementation by its canonical identity.
    ///
    /// Returns an independent shared handle. The catalog lock is released
    /// before the caller can execute provider operations.
    pub fn get(
        &self,
        id: &ProviderId,
    ) -> Result<Option<SharedRegistryProvider>> {
        let entries = self.entries.read().map_err(|_| {
            provider_catalog_lock_error("get")
        })?;

        Ok(entries.get(id).cloned())
    }

    /// Return all registered providers in deterministic identity order.
    pub fn list(&self) -> Result<Vec<SharedRegistryProvider>> {
        let entries = self.entries.read().map_err(|_| {
            provider_catalog_lock_error("list")
        })?;

        Ok(entries.values().cloned().collect())
    }

    /// Remove a provider implementation from future discovery.
    ///
    /// Existing shared provider handles remain valid.
    ///
    /// This operation does not remove or modify registry configurations
    /// that refer to the provider.
    ///
    /// Such registries will subsequently be reported as having a
    /// missing provider.
    pub fn remove(
        &self,
        id: &ProviderId,
    ) -> Result<Option<SharedRegistryProvider>> {
        let mut entries = self.entries.write().map_err(|_| {
            provider_catalog_lock_error("remove")
        })?;

        Ok(entries.remove(id))
    }

    /// Return the number of registered provider implementations.
    pub fn len(&self) -> Result<usize> {
        let entries = self.entries.read().map_err(|_| {
            provider_catalog_lock_error("len")
        })?;

        Ok(entries.len())
    }

    /// Determine whether the provider catalog is empty.
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }

    /// Determine whether an implementation is registered.
    pub fn contains(
        &self,
        id: &ProviderId,
    ) -> Result<bool> {
        let entries = self.entries.read().map_err(|_| {
            provider_catalog_lock_error("contains")
        })?;

        Ok(entries.contains_key(id))
    }
}

// ============================================================================
// DISCOVERY REQUIREMENTS
// ============================================================================

/// Explicit requirements applied during registry discovery.
///
/// An absent protocol or operation means that discovery does not filter
/// on that particular requirement.
///
/// These requirements concern declared provider compatibility only.
///
/// Authentication, authorization, trust and remote availability remain
/// separate checks.
#[derive(Debug, Clone, Default)]
pub struct RegistryDiscoveryRequirements {
    /// Required registry protocol version, if one is requested.
    required_protocol: Option<RegistryProtocolVersion>,

    /// Required provider operation, if one is requested.
    required_operation: Option<RegistryOperationId>,
}

impl RegistryDiscoveryRequirements {
    /// Create empty discovery requirements.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Require a particular registry protocol version.
    #[must_use]
    pub fn protocol(
        mut self,
        version: RegistryProtocolVersion,
    ) -> Self {
        self.required_protocol = Some(version);
        self
    }

    /// Require a particular provider operation.
    #[must_use]
    pub fn operation(
        mut self,
        operation: RegistryOperationId,
    ) -> Self {
        self.required_operation = Some(operation);
        self
    }

    /// Return the required protocol version.
    #[must_use]
    pub const fn required_protocol(
        &self,
    ) -> Option<RegistryProtocolVersion> {
        self.required_protocol
    }

    /// Return the required operation.
    #[must_use]
    pub fn required_operation(
        &self,
    ) -> Option<&RegistryOperationId> {
        self.required_operation.as_ref()
    }
}

// ============================================================================
// DISCOVERY REQUEST
// ============================================================================

/// Request controlling a registry discovery operation.
///
/// By default, disabled registries are included in the report so that
/// callers can understand the complete configured registry state.
///
/// They are never considered selectable.
#[derive(Debug, Clone)]
pub struct RegistryDiscoveryRequest {
    /// Whether disabled registry configurations should appear in results.
    include_disabled: bool,

    /// Required provider compatibility conditions.
    requirements: RegistryDiscoveryRequirements,
}

impl Default for RegistryDiscoveryRequest {
    fn default() -> Self {
        Self {
            include_disabled: true,
            requirements: RegistryDiscoveryRequirements::new(),
        }
    }
}

impl RegistryDiscoveryRequest {
    /// Create a discovery request with default behavior.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Include or exclude disabled registries.
    ///
    /// Excluded entries are counted in `omitted_disabled`.
    #[must_use]
    pub fn include_disabled(
        mut self,
        include: bool,
    ) -> Self {
        self.include_disabled = include;
        self
    }

    /// Set provider compatibility requirements.
    #[must_use]
    pub fn requirements(
        mut self,
        requirements: RegistryDiscoveryRequirements,
    ) -> Self {
        self.requirements = requirements;
        self
    }

    /// Return whether disabled registries should be included.
    #[must_use]
    pub const fn includes_disabled(&self) -> bool {
        self.include_disabled
    }

    /// Return the discovery requirements.
    #[must_use]
    pub const fn discovery_requirements(
        &self,
    ) -> &RegistryDiscoveryRequirements {
        &self.requirements
    }
}

// ============================================================================
// PROVIDER AVAILABILITY
// ============================================================================

/// Local provider-compatibility classification.
///
/// These values do not claim remote service availability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderAvailability {
    /// A registered implementation satisfies the requested checks.
    Available,

    /// No implementation with the configured provider identity exists.
    Missing,

    /// The provider identity or configured protocol ranges are incompatible.
    Incompatible,

    /// The explicitly required protocol version is unsupported.
    ProtocolUnsupported,

    /// The explicitly required operation is not declared by the provider.
    OperationUnsupported,
}

impl ProviderAvailability {
    /// Determine whether the provider satisfies local discovery checks.
    #[must_use]
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }
}

// ============================================================================
// DISCOVERY ENTRY
// ============================================================================

/// One registry and its discovery classification.
///
/// Registry configuration is retained as an immutable snapshot.
///
/// A provider handle is supplied only when a compatible registered
/// implementation satisfies the requested discovery conditions.
///
/// The handle does not imply authentication, authorization, trust,
/// reachability or successful operation.
#[derive(Clone)]
pub struct RegistryDiscoveryEntry {
    registry: RegistrySnapshot,
    provider: Option<SharedRegistryProvider>,
    provider_availability: ProviderAvailability,
    diagnostic: Option<ToolchainError>,
}

impl fmt::Debug for RegistryDiscoveryEntry {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("RegistryDiscoveryEntry")
            .field("registry", self.registry.registry())
            .field(
                "provider_availability",
                &self.provider_availability,
            )
            .field(
                "provider_registered",
                &self.provider.is_some(),
            )
            .field("diagnostic", &self.diagnostic)
            .finish()
    }
}

impl RegistryDiscoveryEntry {
    fn new(
        registry: RegistrySnapshot,
        provider: Option<SharedRegistryProvider>,
        provider_availability: ProviderAvailability,
        diagnostic: Option<ToolchainError>,
    ) -> Self {
        Self {
            registry,
            provider,
            provider_availability,
            diagnostic,
        }
    }

    /// Return the immutable registry configuration snapshot.
    #[must_use]
    pub fn registry(&self) -> &RegistrySnapshot {
        &self.registry
    }

    /// Return the registered provider handle, if compatible.
    #[must_use]
    pub fn provider(&self) -> Option<&SharedRegistryProvider> {
        self.provider.as_ref()
    }

    /// Return the local provider compatibility classification.
    #[must_use]
    pub const fn provider_availability(
        &self,
    ) -> ProviderAvailability {
        self.provider_availability
    }

    /// Return the associated discovery diagnostic, if any.
    #[must_use]
    pub fn diagnostic(&self) -> Option<&ToolchainError> {
        self.diagnostic.as_ref()
    }

    /// Determine whether the registry is eligible for selection.
    ///
    /// Eligibility requires:
    ///
    /// - Enabled registry lifecycle state.
    /// - A registered provider implementation.
    /// - Successful local compatibility validation.
    ///
    /// This does not establish operational availability.
    #[must_use]
    pub fn is_selectable(&self) -> bool {
        self.registry.registry().status() == RegistryStatus::Enabled
            && self.provider_availability.is_available()
            && self.provider.is_some()
    }
}

// ============================================================================
// DISCOVERY REPORT
// ============================================================================

/// Complete deterministic result of one discovery operation.
#[derive(Debug, Clone)]
pub struct RegistryDiscoveryReport {
    entries: Vec<RegistryDiscoveryEntry>,
    omitted_disabled: usize,
}

impl RegistryDiscoveryReport {
    fn new(
        entries: Vec<RegistryDiscoveryEntry>,
        omitted_disabled: usize,
    ) -> Self {
        Self {
            entries,
            omitted_disabled,
        }
    }

    /// Return all reported registry entries.
    ///
    /// Entries are ordered by canonical `RegistryId`.
    #[must_use]
    pub fn entries(&self) -> &[RegistryDiscoveryEntry] {
        &self.entries
    }

    /// Return only registries eligible for selection.
    ///
    /// The iterator preserves deterministic registry identity ordering.
    pub fn selectable(
        &self,
    ) -> impl Iterator<Item = &RegistryDiscoveryEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.is_selectable())
    }

    /// Return the number of reported registry entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Determine whether no registry entries were reported.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Return the number of disabled registries excluded by the request.
    #[must_use]
    pub const fn omitted_disabled(&self) -> usize {
        self.omitted_disabled
    }

    /// Return the number of selectable registries.
    #[must_use]
    pub fn selectable_count(&self) -> usize {
        self.selectable().count()
    }

    /// Find a reported registry by its canonical identity.
    #[must_use]
    pub fn find(
        &self,
        id: &crate::toolchain::ids::RegistryId,
    ) -> Option<&RegistryDiscoveryEntry> {
        self.entries
            .binary_search_by(|entry| {
                entry.registry.registry().id().cmp(id)
            })
            .ok()
            .map(|index| &self.entries[index])
    }
}

// ============================================================================
// REGISTRY DISCOVERY SERVICE
// ============================================================================

/// Registry discovery service.
///
/// The service coordinates the existing registry catalog and the
/// provider implementation catalog.
///
/// It is intentionally independent of networking and asynchronous
/// execution. Discovery performs local configuration and contract checks.
///
/// The catalogs are shared so that configuration and provider lifecycle
/// operations can be managed by their respective owners.
pub struct RegistryDiscovery {
    registries: Arc<RegistryCatalog>,
    providers: Arc<RegistryProviderCatalog>,
}

impl fmt::Debug for RegistryDiscovery {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("RegistryDiscovery")
            .field("registries", &self.registries)
            .field("providers", &self.providers)
            .finish()
    }
}

impl RegistryDiscovery {
    /// Construct a discovery service from canonical catalogs.
    #[must_use]
    pub fn new(
        registries: Arc<RegistryCatalog>,
        providers: Arc<RegistryProviderCatalog>,
    ) -> Self {
        Self {
            registries,
            providers,
        }
    }

    /// Return the registry catalog used by this service.
    #[must_use]
    pub fn registry_catalog(&self) -> &Arc<RegistryCatalog> {
        &self.registries
    }

    /// Return the provider catalog used by this service.
    #[must_use]
    pub fn provider_catalog(&self) -> &Arc<RegistryProviderCatalog> {
        &self.providers
    }

    /// Discover configured registry instances.
    ///
    /// Processing:
    ///
    /// 1. Obtain immutable registry snapshots.
    /// 2. Apply the disabled-registry inclusion policy.
    /// 3. Resolve the configured provider identity.
    /// 4. Validate the provider descriptor.
    /// 5. Validate registry/provider identity and protocol compatibility.
    /// 6. Apply explicitly requested protocol and operation requirements.
    /// 7. Return deterministic entries with structured diagnostics.
    ///
    /// No registry or provider lock is held while a provider is inspected
    /// or while compatibility checks are performed.
    ///
    /// No provider operation is executed.
    pub fn discover(
        &self,
        request: &RegistryDiscoveryRequest,
    ) -> Result<RegistryDiscoveryReport> {
        let snapshots = self
            .registries
            .list()
            .map_err(|error| {
                error
                    .with_operation(OPERATION_DISCOVER)
                    .with_context(
                        "Unable to obtain configured registry snapshots.",
                    )
            })?;

        let mut entries = Vec::new();
        let mut omitted_disabled = 0_usize;

        for snapshot in snapshots {
            let registry = snapshot.registry();

            if registry.status() == RegistryStatus::Disabled
                && !request.include_disabled
            {
                omitted_disabled = omitted_disabled
                    .checked_add(1)
                    .ok_or_else(|| {
                        ToolchainError::resource_exhausted(
                            "Disabled registry count exceeded the representable range.",
                        )
                    })?;

                continue;
            }

            let provider_id = registry.provider().provider_id();

            let provider = self
                .providers
                .get(provider_id)
                .map_err(|error| {
                    error
                        .with_operation(OPERATION_DISCOVER)
                        .with_subject(registry.id().to_string())
                        .with_context(
                            "Unable to resolve the configured provider.",
                        )
                })?;

            let entry = match provider {
                Some(provider) => {
                    self.inspect_provider(
                        snapshot,
                        provider,
                        request.discovery_requirements(),
                    )
                }

                None => {
                    let diagnostic = ToolchainError::new(
                        ToolchainErrorKind::ProviderFailed,
                        "The configured registry provider is not registered.",
                    )
                    .with_operation(OPERATION_PROVIDER_LOOKUP)
                    .with_subject(registry.id().to_string())
                    .with_context(provider_id.to_string());

                    RegistryDiscoveryEntry::new(
                        snapshot,
                        None,
                        ProviderAvailability::Missing,
                        Some(diagnostic),
                    )
                }
            };

            entries.push(entry);
        }

        // RegistryCatalog::list() already provides identity ordering.
        // Keep the ordering invariant explicit at the report boundary.
        entries.sort_by(|left, right| {
            left.registry
                .registry()
                .id()
                .cmp(right.registry.registry().id())
        });

        Ok(RegistryDiscoveryReport::new(
            entries,
            omitted_disabled,
        ))
    }

    fn inspect_provider(
        &self,
        snapshot: RegistrySnapshot,
        provider: SharedRegistryProvider,
        requirements: &RegistryDiscoveryRequirements,
    ) -> RegistryDiscoveryEntry {
        let registry = snapshot.registry();
        let descriptor = provider.descriptor();

        let failure = |availability, error| {
            RegistryDiscoveryEntry::new(
                snapshot.clone(),
                None,
                availability,
                Some(
                    error
                        .with_operation(OPERATION_DISCOVER)
                        .with_subject(registry.id().to_string()),
                ),
            )
        };

        if let Err(error) = descriptor.validate() {
            return failure(
                ProviderAvailability::Incompatible,
                error,
            );
        }

        if let Err(error) = registry.validate_provider(descriptor) {
            return failure(
                ProviderAvailability::Incompatible,
                error,
            );
        }

        if let Some(protocol) = requirements.required_protocol {
            if !registry.provider().supports_protocol(protocol)
                || !descriptor.supports_protocol(protocol)
            {
                let error = ToolchainError::new(
                    ToolchainErrorKind::IncompatibleVersion,
                    "The registry and provider do not both support the required protocol version.",
                );

                return failure(
                    ProviderAvailability::ProtocolUnsupported,
                    error,
                );
            }
        }

        if let Some(operation) = &requirements.required_operation {
            if !descriptor.supports_operation(operation) {
                let error = ToolchainError::new(
                    ToolchainErrorKind::UnsupportedOperation,
                    format!(
                        "The configured provider does not declare operation '{}'.",
                        operation.as_str(),
                    ),
                );

                return failure(
                    ProviderAvailability::OperationUnsupported,
                    error,
                );
            }
        }

        RegistryDiscoveryEntry::new(
            snapshot,
            Some(provider),
            ProviderAvailability::Available,
            None,
        )
    }
}

// ============================================================================
// ERROR HELPERS
// ============================================================================

fn provider_catalog_lock_error(
    operation: &str,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ProviderFailed,
        format!(
            "Registry provider catalog lock could not be acquired during {operation}.",
        ),
    )
    .with_operation("registry.discovery.provider_catalog")
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;
    use std::future::Future;
    use std::pin::Pin;

    use crate::toolchain::ids::{
        IdNamespace,
        ProviderId,
        RegistryId,
    };

    use crate::toolchain::version::{
        ProviderApiVersion,
        RegistryProtocolVersion,
        VersionRange,
    };

    use super::super::provider::{
        ProviderFuture,
        RegistryProviderDescriptor,
        RegistryRequest,
        RegistryResponse,
    };

    use super::super::registry::{
        Registry,
        RegistryLocation,
        RegistryName,
        RegistryProviderBinding,
    };

    const TEST_NAMESPACE: &str = "zamani.tests";

    fn registry_id(value: u128) -> RegistryId {
        RegistryId::new(
            IdNamespace::new(TEST_NAMESPACE).expect("namespace"),
            value,
        )
        .expect("registry ID")
    }

    fn provider_id(value: u128) -> ProviderId {
        ProviderId::new(
            IdNamespace::new(TEST_NAMESPACE).expect("namespace"),
            value,
        )
        .expect("provider ID")
    }

    fn protocol() -> RegistryProtocolVersion {
        RegistryProtocolVersion::new(1, 0, 0)
    }

    fn operation() -> RegistryOperationId {
        RegistryOperationId::new("packages.query")
            .expect("operation ID")
    }

    struct TestProvider {
        descriptor: RegistryProviderDescriptor,
    }

    impl TestProvider {
        fn new(id: ProviderId) -> Self {
            let descriptor = RegistryProviderDescriptor::new(
                id,
                "Test provider",
                ProviderApiVersion::new(1, 0, 0),
                vec![VersionRange::any()],
                BTreeSet::from([operation()]),
                BTreeSet::new(),
            )
            .expect("valid provider descriptor");

            Self { descriptor }
        }
    }

    impl RegistryProvider for TestProvider {
        fn descriptor(&self) -> &RegistryProviderDescriptor {
            &self.descriptor
        }

        fn execute<'a>(
            &'a self,
            request: RegistryRequest,
        ) -> ProviderFuture<'a> {
            Box::pin(async move {
                Ok(RegistryResponse::new(
                    request.registry_id().clone(),
                    request.protocol_version(),
                    Arc::<[u8]>::from([]),
                ))
            })
        }
    }

    fn registry(id: u128, provider: u128) -> Registry {
        let binding = RegistryProviderBinding::new(
            provider_id(provider),
            vec![VersionRange::any()],
        )
        .expect("valid provider binding");

        Registry::new(
            registry_id(id),
            RegistryName::new(format!("Registry {id}"))
                .expect("registry name"),
            RegistryLocation::new(format!("local:registry/{id}"))
                .expect("registry location"),
            binding,
        )
    }

    fn enabled_catalog(
        entries: impl IntoIterator<Item = Registry>,
    ) -> Arc<RegistryCatalog> {
        let catalog = Arc::new(RegistryCatalog::new());

        for registry in entries {
            let snapshot = catalog
                .register(registry)
                .expect("registry registration");

            catalog
                .set_status(
                    snapshot.registry().id(),
                    snapshot.registry().revision(),
                    RegistryStatus::Enabled,
                )
                .expect("enable registry");
        }

        catalog
    }

    fn provider_catalog(
        ids: impl IntoIterator<Item = u128>,
    ) -> Arc<RegistryProviderCatalog> {
        let catalog = Arc::new(RegistryProviderCatalog::new());

        for id in ids {
            catalog
                .register(Arc::new(TestProvider::new(provider_id(id))))
                .expect("provider registration");
        }

        catalog
    }

    #[test]
    fn provider_catalog_registers_and_resolves_implementations() {
        let catalog = provider_catalog([1]);

        assert_eq!(catalog.len().expect("count"), 1);
        assert!(catalog.contains(&provider_id(1)).expect("lookup"));
        assert!(catalog.get(&provider_id(1)).expect("lookup").is_some());
    }

    #[test]
    fn duplicate_provider_identity_is_rejected() {
        let catalog = provider_catalog([1]);

        let result = catalog.register(
            Arc::new(TestProvider::new(provider_id(1))),
        );

        assert!(result.is_err());
    }

    #[test]
    fn provider_removal_preserves_existing_shared_handles() {
        let catalog = provider_catalog([1]);
        let existing = catalog
            .get(&provider_id(1))
            .expect("lookup")
            .expect("provider");

        assert!(catalog.remove(&provider_id(1)).expect("remove").is_some());
        assert!(catalog.get(&provider_id(1)).expect("lookup").is_none());

        assert_eq!(
            existing.descriptor().provider_id(),
            &provider_id(1),
        );
    }

    #[test]
    fn discovers_compatible_enabled_registry() {
        let registries = enabled_catalog([registry(1, 1)]);
        let providers = provider_catalog([1]);

        let discovery = RegistryDiscovery::new(
            registries,
            providers,
        );

        let report = discovery
            .discover(&RegistryDiscoveryRequest::new())
            .expect("discovery");

        assert_eq!(report.len(), 1);
        assert_eq!(report.selectable_count(), 1);

        let entry = &report.entries()[0];

        assert_eq!(
            entry.provider_availability(),
            ProviderAvailability::Available,
        );

        assert!(entry.is_selectable());
        assert!(entry.provider().is_some());
        assert!(entry.diagnostic().is_none());
    }

    #[test]
    fn reports_missing_provider_without_fabricating_availability() {
        let registries = enabled_catalog([registry(1, 99)]);
        let providers = provider_catalog([]);

        let discovery = RegistryDiscovery::new(
            registries,
            providers,
        );

        let report = discovery
            .discover(&RegistryDiscoveryRequest::new())
            .expect("discovery");

        let entry = &report.entries()[0];

        assert_eq!(
            entry.provider_availability(),
            ProviderAvailability::Missing,
        );

        assert!(!entry.is_selectable());
        assert!(entry.provider().is_none());
        assert!(entry.diagnostic().is_some());
    }

    #[test]
    fn disabled_registries_are_reported_but_not_selectable() {
        let registries = Arc::new(RegistryCatalog::new());

        registries
            .register(registry(1, 1))
            .expect("registration");

        let discovery = RegistryDiscovery::new(
            registries,
            provider_catalog([1]),
        );

        let report = discovery
            .discover(&RegistryDiscoveryRequest::new())
            .expect("discovery");

        assert_eq!(report.len(), 1);
        assert_eq!(report.selectable_count(), 0);
        assert!(!report.entries()[0].is_selectable());
    }

    #[test]
    fn disabled_registries_can_be_excluded_explicitly() {
        let registries = Arc::new(RegistryCatalog::new());

        registries
            .register(registry(1, 1))
            .expect("registration");

        let discovery = RegistryDiscovery::new(
            registries,
            provider_catalog([1]),
        );

        let report = discovery
            .discover(
                &RegistryDiscoveryRequest::new()
                    .include_disabled(false),
            )
            .expect("discovery");

        assert!(report.is_empty());
        assert_eq!(report.omitted_disabled(), 1);
    }

    #[test]
    fn required_protocol_is_checked_against_both_contracts() {
        let discovery = RegistryDiscovery::new(
            enabled_catalog([registry(1, 1)]),
            provider_catalog([1]),
        );

        let report = discovery
            .discover(
                &RegistryDiscoveryRequest::new().requirements(
                    RegistryDiscoveryRequirements::new()
                        .protocol(protocol()),
                ),
            )
            .expect("discovery");

        assert_eq!(
            report.entries()[0].provider_availability(),
            ProviderAvailability::Available,
        );
    }

    #[test]
    fn unsupported_required_operation_is_reported() {
        let discovery = RegistryDiscovery::new(
            enabled_catalog([registry(1, 1)]),
            provider_catalog([1]),
        );

        let unsupported = RegistryOperationId::new(
            "packages.publish",
        )
        .expect("operation");

        let report = discovery
            .discover(
                &RegistryDiscoveryRequest::new().requirements(
                    RegistryDiscoveryRequirements::new()
                        .operation(unsupported),
                ),
            )
            .expect("discovery");

        let entry = &report.entries()[0];

        assert_eq!(
            entry.provider_availability(),
            ProviderAvailability::OperationUnsupported,
        );

        assert!(!entry.is_selectable());
    }

    #[test]
    fn provider_compatibility_failure_is_reported() {
        let binding = RegistryProviderBinding::new(
            provider_id(1),
            vec![VersionRange::new(
                RegistryProtocolVersion::new(2, 0, 0),
                RegistryProtocolVersion::new(3, 0, 0),
            )
            .expect("range")],
        )
        .expect("binding");

        let registry = Registry::new(
            registry_id(1),
            RegistryName::new("Incompatible")
                .expect("name"),
            RegistryLocation::new("local:incompatible")
                .expect("location"),
            binding,
        );

        let discovery = RegistryDiscovery::new(
            enabled_catalog([registry]),
            provider_catalog([1]),
        );

        let report = discovery
            .discover(&RegistryDiscoveryRequest::new())
            .expect("discovery");

        assert_eq!(
            report.entries()[0].provider_availability(),
            ProviderAvailability::Incompatible,
        );

        assert!(!report.entries()[0].is_selectable());
    }

    #[test]
    fn discovery_order_is_deterministic() {
        let discovery = RegistryDiscovery::new(
            enabled_catalog([
                registry(30, 1),
                registry(10, 1),
                registry(20, 1),
            ]),
            provider_catalog([1]),
        );

        let report = discovery
            .discover(&RegistryDiscoveryRequest::new())
            .expect("discovery");

        let ids: Vec<_> = report
            .entries()
            .iter()
            .map(|entry| entry.registry().registry().id().clone())
            .collect();

        assert_eq!(
            ids,
            vec![registry_id(10), registry_id(20), registry_id(30)],
        );
    }

    #[test]
    fn registry_lookup_uses_binary_search_over_sorted_results() {
        let discovery = RegistryDiscovery::new(
            enabled_catalog([
                registry(10, 1),
                registry(20, 1),
                registry(30, 1),
            ]),
            provider_catalog([1]),
        );

        let report = discovery
            .discover(&RegistryDiscoveryRequest::new())
            .expect("discovery");

        assert!(report.find(&registry_id(20)).is_some());
        assert!(report.find(&registry_id(99)).is_none());
    }

    #[test]
    fn multiple_registries_can_share_one_provider() {
        let discovery = RegistryDiscovery::new(
            enabled_catalog([
                registry(1, 1),
                registry(2, 1),
                registry(3, 1),
            ]),
            provider_catalog([1]),
        );

        let report = discovery
            .discover(&RegistryDiscoveryRequest::new())
            .expect("discovery");

        assert_eq!(report.len(), 3);
        assert_eq!(report.selectable_count(), 3);
    }

    #[test]
    fn provider_catalog_is_independent_of_registry_catalog() {
        let registries = enabled_catalog([registry(1, 1)]);
        let providers = provider_catalog([]);

        let discovery = RegistryDiscovery::new(
            Arc::clone(&registries),
            Arc::clone(&providers),
        );

        assert_eq!(registries.len().expect("registry count"), 1);
        assert_eq!(providers.len().expect("provider count"), 0);

        assert_eq!(
            discovery
                .discover(&RegistryDiscoveryRequest::new())
                .expect("discovery")
                .selectable_count(),
            0,
        );
    }

    #[test]
    fn empty_catalogs_produce_an_empty_report() {
        let discovery = RegistryDiscovery::new(
            Arc::new(RegistryCatalog::new()),
            Arc::new(RegistryProviderCatalog::new()),
        );

        let report = discovery
            .discover(&RegistryDiscoveryRequest::new())
            .expect("discovery");

        assert!(report.is_empty());
        assert_eq!(report.selectable_count(), 0);
        assert_eq!(report.omitted_disabled(), 0);
    }

    #[test]
    fn provider_discovery_does_not_execute_provider_operations() {
        // Discovery inspects descriptors only. The provider's execute
        // method is reserved for the registry client.
        let discovery = RegistryDiscovery::new(
            enabled_catalog([registry(1, 1)]),
            provider_catalog([1]),
        );

        let report = discovery
            .discover(&RegistryDiscoveryRequest::new())
            .expect("discovery");

        assert_eq!(report.selectable_count(), 1);
    }
}
