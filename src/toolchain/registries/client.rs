
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Registry Client
//!
//! Path: src/toolchain/registries/client.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the provider-neutral orchestration boundary for configured
//! package registries.
//!
//! The client coordinates:
//!
//! - Registry catalog access.
//! - Provider registration and lookup.
//! - Registry/provider binding validation.
//! - Protocol compatibility checks.
//! - Registry lifecycle validation.
//! - Registry operation dispatch.
//! - Provider response-envelope validation.
//!
//! ## Ownership
//!
//! This module owns provider selection and registry operation orchestration.
//!
//! It does not own:
//!
//! - Registry identity or configuration.
//! - Provider implementation.
//! - Wire-protocol encoding or decoding.
//! - Network transport.
//! - Authentication or credential storage.
//! - Authorization policy.
//! - Package metadata interpretation.
//! - Dependency resolution.
//! - Package downloading or publishing.
//! - Package archives.
//! - Signatures, integrity or trust decisions.
//! - Registry discovery.
//! - Registry federation.
//! - Build execution.
//! - Hardware discovery.
//! - Runtime execution.
//!
//! Those responsibilities remain with their canonical subsystems.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on registry instances, providers,
//! operations, computational domains or package counts.
//!
//! The client uses dynamically growing collections and shared provider
//! handles. Actual memory, execution and concurrency remain subject to
//! available resources and caller-configured operational policies.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## Asynchronous execution
//!
//! The client uses the standard-library Future contract established by
//! provider.rs. It does not require Tokio, async-std or another executor.
//!
//! The caller owns executor integration, deadlines, cancellation,
//! retry policies and operational budgets.
//!
//! ## Integration
//!
//! - registry.rs supplies RegistryCatalog and RegistrySnapshot.
//! - provider.rs supplies RegistryProvider and execute_checked.
//! - ids.rs supplies RegistryId and ProviderId.
//! - version.rs supplies RegistryProtocolVersion.
//! - errors.rs supplies structured toolchain failures.
//! - authentication/security remain external authorization boundaries.
//! - protocol.rs remains responsible for payload encoding and decoding.
//!
//! A registry operation must be authorized before it is dispatched.
//!
//! Provider code is never executed while a registry-catalog or
//! provider-catalog lock is held.

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::toolchain::errors::{
    Result,
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::ids::ProviderId;

use super::provider::{
    execute_checked,
    RegistryProvider,
    RegistryRequest,
    RegistryResponse,
};

use super::registry::{
    RegistryCatalog,
    RegistrySnapshot,
};

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_CLIENT: &str = "registry.client";
const OPERATION_REGISTER: &str = "registry.client.register_provider";
const OPERATION_REMOVE: &str = "registry.client.remove_provider";
const OPERATION_EXECUTE: &str = "registry.client.execute";
const OPERATION_LOOKUP: &str = "registry.client.lookup_provider";

// ============================================================================
// SHARED CLIENT TYPES
// ============================================================================

/// Shared registry client handle.
///
/// Cloning this value shares the same registry catalog and provider
/// registrations.
pub type SharedRegistryClient = Arc<RegistryClient>;

/// Shared registry provider implementation.
pub type SharedRegistryProvider = Arc<dyn RegistryProvider>;

// ============================================================================
// REGISTRY CLIENT
// ============================================================================

/// Provider-neutral registry operation coordinator.
///
/// The registry catalog is supplied by the caller and remains the
/// canonical authority for registry configuration and lifecycle.
///
/// Provider implementations are registered independently of registry
/// instances. A single provider may serve multiple registry instances,
/// subject to the provider's own compatibility and configuration rules.
///
/// The client does not assume that a provider represents a particular
/// vendor, protocol, transport or computational domain.
pub struct RegistryClient {
    registries: Arc<RegistryCatalog>,

    providers: RwLock<BTreeMap<ProviderId, SharedRegistryProvider>>,
}

impl RegistryClient {
    /// Construct a registry client using an existing registry catalog.
    ///
    /// The supplied catalog remains the canonical registry authority.
    ///
    /// A new provider catalog is created for this client instance.
    #[must_use]
    pub fn new(registries: Arc<RegistryCatalog>) -> Self {
        Self {
            registries,
            providers: RwLock::new(BTreeMap::new()),
        }
    }

    /// Return the shared canonical registry catalog.
    #[must_use]
    pub fn registry_catalog(&self) -> &Arc<RegistryCatalog> {
        &self.registries
    }

    // ========================================================================
    // PROVIDER REGISTRATION
    // ========================================================================

    /// Register a registry provider implementation.
    ///
    /// Registration validates the provider's immutable descriptor before
    /// inserting the implementation.
    ///
    /// Provider identities must be unique within this client instance.
    ///
    /// Registering the same provider identity twice is rejected rather
    /// than silently replacing an existing implementation.
    ///
    /// Existing in-flight operations retain their shared provider handle.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The descriptor is invalid.
    /// - A provider with the same identity already exists.
    /// - The provider catalog lock is poisoned.
    pub fn register_provider(
        &self,
        provider: SharedRegistryProvider,
    ) -> Result<()> {
        let descriptor = provider.descriptor();

        descriptor.validate()?;

        let provider_id = descriptor.provider_id().clone();

        let mut providers = self.providers.write().map_err(|_| {
            lock_error(OPERATION_REGISTER)
        })?;

        if providers.contains_key(&provider_id) {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::AlreadyExists,
                    "A registry provider with this identity is already registered.",
                )
                .with_operation(OPERATION_REGISTER)
                .with_subject(provider_id.to_string()),
            );
        }

        providers.insert(provider_id, provider);

        Ok(())
    }

    /// Remove a registered provider implementation.
    ///
    /// Removing a provider prevents future operations from selecting it.
    ///
    /// Operations that already obtained a shared provider handle may
    /// complete independently.
    ///
    /// This method does not remove registry configurations referring to
    /// the provider. Such configurations remain in the canonical registry
    /// catalog and will report that their provider is unavailable.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider catalog lock is poisoned.
    pub fn remove_provider(
        &self,
        provider_id: &ProviderId,
    ) -> Result<bool> {
        let mut providers = self.providers.write().map_err(|_| {
            lock_error(OPERATION_REMOVE)
        })?;

        Ok(providers.remove(provider_id).is_some())
    }

    /// Return a registered provider descriptor.
    ///
    /// The returned descriptor is an independent immutable snapshot.
    ///
    /// Descriptor existence does not prove that a provider is reachable,
    /// authenticated, trusted or operationally available.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider does not exist or the provider
    /// catalog lock is poisoned.
    pub fn provider_descriptor(
        &self,
        provider_id: &ProviderId,
    ) -> Result<super::provider::RegistryProviderDescriptor> {
        let provider = self.provider_handle(provider_id)?;

        Ok(provider.descriptor().clone())
    }

    /// Return the number of registered provider implementations.
    ///
    /// This is observational metadata, not a configured capacity limit.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider catalog lock is poisoned.
    pub fn provider_count(&self) -> Result<usize> {
        let providers = self.providers.read().map_err(|_| {
            lock_error(OPERATION_LOOKUP)
        })?;

        Ok(providers.len())
    }

    // ========================================================================
    // REGISTRY LOOKUP
    // ========================================================================

    /// Retrieve a registry configuration snapshot.
    ///
    /// This operation does not perform provider execution or network access.
    ///
    /// # Errors
    ///
    /// Returns an error if the registry catalog cannot be read.
    pub fn registry_snapshot(
        &self,
        registry_id: &crate::toolchain::ids::RegistryId,
    ) -> Result<Option<RegistrySnapshot>> {
        self.registries.get(registry_id)
    }

    // ========================================================================
    // REGISTRY OPERATION EXECUTION
    // ========================================================================

    /// Execute one operation against a configured registry.
    ///
    /// The supplied request identifies:
    ///
    /// - The registry.
    /// - The registry protocol version.
    /// - The open-world operation identifier.
    /// - The opaque protocol payload.
    ///
    /// The client:
    ///
    /// 1. Resolves the configured registry.
    /// 2. Verifies that the registry is enabled.
    /// 3. Resolves the configured provider.
    /// 4. Validates the registry/provider binding.
    /// 5. Validates the requested protocol version against the registry.
    /// 6. Supplies the configured registry location to the request.
    /// 7. Executes the provider through execute_checked.
    /// 8. Returns the validated provider response.
    ///
    /// Provider execution occurs without holding either management lock.
    ///
    /// The operation must already have passed the caller's applicable
    /// authentication and authorization checks.
    ///
    /// This method does not retry failed operations automatically.
    /// In particular, it does not retry potentially non-idempotent
    /// publication or mutation operations.
    ///
    /// # Errors
    ///
    /// Returns structured errors for:
    ///
    /// - Unknown registry.
    /// - Disabled or retired registry.
    /// - Unknown provider.
    /// - Provider identity mismatch.
    /// - Incompatible protocol version.
    /// - Unsupported provider operation.
    /// - Provider execution failure.
    /// - Invalid provider response.
    /// - Poisoned management locks.
    ///
    /// Provider errors retain their original structured classification.
    pub async fn execute(
        &self,
        request: RegistryRequest,
    ) -> Result<RegistryResponse> {
        let registry_id = request.registry_id().clone();

        // --------------------------------------------------------------------
        // 1. Resolve an immutable registry snapshot.
        // --------------------------------------------------------------------

        let snapshot = self
            .registries
            .get(&registry_id)?
            .ok_or_else(|| {
                ToolchainError::new(
                    ToolchainErrorKind::NotFound,
                    "The requested registry is not registered.",
                )
                .with_operation(OPERATION_EXECUTE)
                .with_subject(registry_id.to_string())
            })?;

        let registry = snapshot.registry();

        // --------------------------------------------------------------------
        // 2. Enforce registry lifecycle.
        // --------------------------------------------------------------------

        if !registry.is_selectable() {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::InvalidConfiguration,
                    "The requested registry is not enabled for operations.",
                )
                .with_operation(OPERATION_EXECUTE)
                .with_subject(registry_id.to_string()),
            );
        }

        // --------------------------------------------------------------------
        // 3. Resolve the provider without retaining the provider lock.
        // --------------------------------------------------------------------

        let provider_id = registry.provider().provider_id().clone();

        let provider = self.provider_handle(&provider_id)?;

        // --------------------------------------------------------------------
        // 4. Validate the registry/provider relationship.
        // --------------------------------------------------------------------

        registry.validate_provider(provider.descriptor())?;

        // --------------------------------------------------------------------
        // 5. Validate the exact requested protocol version.
        // --------------------------------------------------------------------

        let protocol_version = request.protocol_version();

        if !registry.provider().supports_protocol(protocol_version) {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::IncompatibleVersion,
                    "The requested protocol version is not permitted by the registry configuration.",
                )
                .with_operation(OPERATION_EXECUTE)
                .with_subject(registry_id.to_string()),
            );
        }

        if !provider
            .descriptor()
            .supports_protocol(protocol_version)
        {
            return Err(
                ToolchainError::new(
                    ToolchainErrorKind::IncompatibleVersion,
                    "The selected provider does not support the requested protocol version.",
                )
                .with_operation(OPERATION_EXECUTE)
                .with_subject(provider_id.to_string()),
            );
        }

        // --------------------------------------------------------------------
        // 6. Attach the canonical configured registry location.
        //
        // RegistryRequest::with_location is an explicit integration
        // extension to provider.rs documented below.
        // --------------------------------------------------------------------

        let request = request.with_location(
            Arc::<str>::from(registry.location().as_str()),
        );

        // --------------------------------------------------------------------
        // 7. Execute without holding registry or provider catalog locks.
        // --------------------------------------------------------------------

        execute_checked(provider.as_ref(), request)
            .await
            .map_err(|error| {
                error
                    .with_context(
                        "executing a provider operation through RegistryClient",
                    )
                    .with_subject(provider_id.to_string())
            })
    }

    // ========================================================================
    // INTERNAL PROVIDER RESOLUTION
    // ========================================================================

    /// Resolve a shared provider implementation.
    ///
    /// The returned Arc allows the caller to execute the provider after
    /// releasing the provider catalog lock.
    fn provider_handle(
        &self,
        provider_id: &ProviderId,
    ) -> Result<SharedRegistryProvider> {
        let providers = self.providers.read().map_err(|_| {
            lock_error(OPERATION_LOOKUP)
        })?;

        providers.get(provider_id).cloned().ok_or_else(|| {
            ToolchainError::new(
                ToolchainErrorKind::NotFound,
                "The registry's configured provider is not registered.",
            )
            .with_operation(OPERATION_LOOKUP)
            .with_subject(provider_id.to_string())
        })
    }
}

// ============================================================================
// ERROR HELPERS
// ============================================================================

/// Construct a structured management-lock error.
fn lock_error(operation: &str) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::Internal,
        "A registry client management lock could not be acquired.",
    )
    .with_operation(operation)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    use crate::toolchain::ids::{
        IdNamespace,
        RegistryId,
    };

    use crate::toolchain::version::{
        ProviderApiVersion,
        RegistryProtocolVersion,
        VersionRange,
    };

    use super::super::provider::{
        ProviderFuture,
        RegistryOperationId,
        RegistryProviderDescriptor,
        RegistryRequest,
    };

    use super::super::registry::{
        Registry,
        RegistryLocation,
        RegistryName,
        RegistryProviderBinding,
        RegistryStatus,
    };

    struct TestProvider {
        descriptor: RegistryProviderDescriptor,
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
                    request.payload().to_vec(),
                ))
            })
        }
    }

    fn test_provider() -> SharedRegistryProvider {
        let namespace = IdNamespace::new("test")
            .expect("valid test namespace");

        let provider_id = ProviderId::new(
            namespace,
            1,
        )
        .expect("valid provider identity");

        let protocol = RegistryProtocolVersion::new(
            1,
            0,
            0,
        );

        let protocol_range = VersionRange::new(
            protocol,
            protocol,
        )
        .expect("valid protocol range");

        let operation = RegistryOperationId::new(
            "packages.query",
        )
        .expect("valid operation");

        let descriptor = RegistryProviderDescriptor::new(
            provider_id,
            "Test provider",
            ProviderApiVersion::new(1, 0, 0),
            vec![protocol_range],
            BTreeSet::from([operation]),
            BTreeSet::new(),
        )
        .expect("valid provider descriptor");

        Arc::new(TestProvider { descriptor })
    }

    fn test_registry() -> Registry {
        let namespace = IdNamespace::new("test")
            .expect("valid namespace");

        let registry_id = RegistryId::new(
            namespace,
            1,
        )
        .expect("valid registry identity");

        let provider = test_provider();

        let binding = RegistryProviderBinding::new(
            provider.descriptor().provider_id().clone(),
            provider.descriptor().supported_protocols().to_vec(),
        )
        .expect("valid provider binding");

        Registry::new(
            registry_id,
            RegistryName::new("Test registry")
                .expect("valid registry name"),
            RegistryLocation::new("local:test")
                .expect("valid registry location"),
            binding,
        )
    }

    #[test]
    fn registers_provider() {
        let client = RegistryClient::new(
            Arc::new(RegistryCatalog::new()),
        );

        client
            .register_provider(test_provider())
            .expect("provider registration succeeds");

        assert_eq!(
            client.provider_count().expect("provider count"),
            1,
        );
    }

    #[test]
    fn rejects_duplicate_provider_identity() {
        let client = RegistryClient::new(
            Arc::new(RegistryCatalog::new()),
        );

        client
            .register_provider(test_provider())
            .expect("first registration succeeds");

        let result = client.register_provider(test_provider());

        assert!(result.is_err());
        assert_eq!(
            client.provider_count().expect("provider count"),
            1,
        );
    }

    #[test]
    fn removes_registered_provider() {
        let client = RegistryClient::new(
            Arc::new(RegistryCatalog::new()),
        );

        let provider = test_provider();
        let provider_id = provider.descriptor().provider_id().clone();

        client
            .register_provider(provider)
            .expect("registration succeeds");

        assert!(
            client
                .remove_provider(&provider_id)
                .expect("removal succeeds")
        );

        assert_eq!(
            client.provider_count().expect("provider count"),
            0,
        );

        assert!(
            !client
                .remove_provider(&provider_id)
                .expect("second removal succeeds")
        );
    }

    #[test]
    fn registry_starts_disabled() {
        let registry = test_registry();

        assert_eq!(
            registry.status(),
            RegistryStatus::Disabled,
        );

        assert!(!registry.is_selectable());
    }
}
