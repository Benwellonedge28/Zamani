
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Registry configuration, identity, lifecycle and catalog management.
//!
//! This module owns the local representation and lifecycle of package
//! registries. It does not implement network transport, authentication,
//! package resolution, package storage, signatures or trust decisions.
//!
//! # Architectural boundaries
//!
//! - `provider.rs` owns provider implementation contracts.
//! - `protocol.rs` owns wire-protocol serialization and negotiation.
//! - `client.rs` owns provider selection and registry communication.
//! - `authentication.rs` owns credential acquisition and authentication.
//! - `discovery.rs` owns registry discovery.
//! - `federation.rs` owns multi-registry coordination.
//! - `dependencies/` owns dependency resolution.
//! - `packages/` owns package metadata and archive handling.
//! - `security/` owns integrity, signatures and trust policies.
//!
//! Registry configuration is transport-neutral. A registry location is
//! interpreted by its selected provider; this module does not assume HTTP,
//! a particular URL scheme, a vendor or a hosting service.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, RwLock};

use crate::toolchain::errors::{
    Result,
    ToolchainError,
    ToolchainErrorKind,
};
use crate::toolchain::ids::{
    ProviderId,
    RegistryId,
};
use crate::toolchain::version::{
    RegistryProtocolVersion,
    VersionRange,
};

use super::provider::RegistryProviderDescriptor;

/// Shared registry configuration handle.
pub type SharedRegistry = Arc<Registry>;

/// Shared, thread-safe registry catalog handle.
pub type SharedRegistryCatalog = Arc<RegistryCatalog>;

/// Registry display name.
///
/// This is descriptive metadata, not a stable registry identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegistryName(Arc<str>);

impl RegistryName {
    /// Creates a validated registry name.
    ///
    /// Names must be non-empty and contain no control characters.
    /// No fixed maximum length is imposed by this type.
    pub fn new(value: impl AsRef<str>) -> Result<Self> {
        let value = value.as_ref().trim();

        if value.is_empty() {
            return Err(invalid_registry(
                "Registry name must not be empty.",
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(invalid_registry(
                "Registry name must not contain control characters.",
            ));
        }

        Ok(Self(Arc::from(value)))
    }

    /// Returns the registry name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RegistryName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Provider-interpreted registry location.
///
/// The location is deliberately opaque to the registry catalog. Its
/// interpretation and validation belong to the selected provider.
///
/// Do not place credentials, access tokens, passwords or private keys
/// in this value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegistryLocation(Arc<str>);

impl RegistryLocation {
    /// Creates a registry location.
    ///
    /// This performs generic validation only. Scheme-specific validation
    /// belongs to the selected provider.
    pub fn new(value: impl AsRef<str>) -> Result<Self> {
        let value = value.as_ref().trim();

        if value.is_empty() {
            return Err(invalid_registry(
                "Registry location must not be empty.",
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(invalid_registry(
                "Registry location must not contain control characters.",
            ));
        }

        Ok(Self(Arc::from(value)))
    }

    /// Returns the opaque location.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Operational lifecycle state of a registry.
///
/// `Enabled` means eligible for selection by local policy. It does not
/// establish network availability, successful authentication, provider
/// trust or package compatibility.
///
/// A retired registry cannot be re-enabled. Register a new registry
/// identity if a retired registry must be replaced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegistryStatus {
    /// Configured but not eligible for selection.
    Disabled,

    /// Eligible for selection, subject to validation and policy.
    Enabled,

    /// Permanently retired from this registry identity.
    Retired,
}

impl RegistryStatus {
    /// Returns whether the registry may be selected.
    pub fn is_selectable(self) -> bool {
        matches!(self, Self::Enabled)
    }

    /// Returns whether the registry is permanently retired.
    pub fn is_retired(self) -> bool {
        matches!(self, Self::Retired)
    }
}

/// Provider binding for a registry.
///
/// A binding identifies the provider implementation and the protocol
/// versions that the registry configuration permits.
///
/// It does not contain provider code, authentication material or
/// transport-specific state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryProviderBinding {
    provider_id: ProviderId,
    supported_protocols: Vec<VersionRange<RegistryProtocolVersion>>,
}

impl RegistryProviderBinding {
    /// Creates a provider binding.
    ///
    /// An empty protocol range collection is rejected because it would
    /// make protocol compatibility impossible to establish.
    pub fn new(
        provider_id: ProviderId,
        supported_protocols: Vec<
            VersionRange<RegistryProtocolVersion>,
        >,
    ) -> Result<Self> {
        if supported_protocols.is_empty() {
            return Err(invalid_registry(
                "At least one registry protocol version range is required.",
            ));
        }

        Ok(Self {
            provider_id,
            supported_protocols,
        })
    }

    /// Returns the selected provider identity.
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the configured protocol ranges.
    pub fn supported_protocols(
        &self,
    ) -> &[VersionRange<RegistryProtocolVersion>] {
        &self.supported_protocols
    }

    /// Checks whether a protocol version is permitted.
    pub fn supports_protocol(
        &self,
        version: RegistryProtocolVersion,
    ) -> bool {
        self.supported_protocols
            .iter()
            .any(|range| range.contains(version))
    }

    /// Validates this binding against a registered provider descriptor.
    pub fn validate_provider(
        &self,
        descriptor: &RegistryProviderDescriptor,
    ) -> Result<()> {
        if descriptor.provider_id() != &self.provider_id {
            return Err(ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                "Registry provider identity does not match its configured binding.",
            ));
        }

        let compatible = self.supported_protocols.iter().any(
            |configured_range| {
                descriptor
                    .supported_protocols()
                    .iter()
                    .any(|provider_range| {
                        ranges_overlap(configured_range, provider_range)
                    })
            },
        );

        if !compatible {
            return Err(ToolchainError::new(
                ToolchainErrorKind::ProtocolMismatch,
                "Registry and provider have no overlapping protocol versions.",
            ));
        }

        Ok(())
    }
}

/// Registry configuration.
///
/// Registry configuration contains identity, descriptive metadata,
/// provider binding, location and lifecycle state.
///
/// Credentials and secrets must be supplied through the authentication
/// subsystem and must never be stored in this structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Registry {
    id: RegistryId,
    name: RegistryName,
    location: RegistryLocation,
    provider: RegistryProviderBinding,
    status: RegistryStatus,
    revision: u64,
}

impl Registry {
    /// Creates a new registry configuration.
    ///
    /// New registries start disabled. Enabling is an explicit lifecycle
    /// operation performed through `RegistryCatalog`.
    pub fn new(
        id: RegistryId,
        name: RegistryName,
        location: RegistryLocation,
        provider: RegistryProviderBinding,
    ) -> Self {
        Self {
            id,
            name,
            location,
            provider,
            status: RegistryStatus::Disabled,
            revision: 0,
        }
    }

    /// Returns the stable registry identity.
    pub fn id(&self) -> &RegistryId {
        &self.id
    }

    /// Returns the descriptive registry name.
    pub fn name(&self) -> &RegistryName {
        &self.name
    }

    /// Returns the opaque provider-interpreted location.
    pub fn location(&self) -> &RegistryLocation {
        &self.location
    }

    /// Returns the provider binding.
    pub fn provider(&self) -> &RegistryProviderBinding {
        &self.provider
    }

    /// Returns the lifecycle state.
    pub fn status(&self) -> RegistryStatus {
        self.status
    }

    /// Returns the current configuration revision.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Returns whether the registry is eligible for selection.
    pub fn is_selectable(&self) -> bool {
        self.status.is_selectable()
    }

    /// Validates the configured provider against a provider descriptor.
    pub fn validate_provider(
        &self,
        descriptor: &RegistryProviderDescriptor,
    ) -> Result<()> {
        self.provider.validate_provider(descriptor)
    }

    /// Creates a revised registry configuration.
    ///
    /// This operation is internal to catalog lifecycle management.
    fn revise(
        &self,
        name: RegistryName,
        location: RegistryLocation,
        provider: RegistryProviderBinding,
    ) -> Result<Self> {
        if self.status.is_retired() {
            return Err(invalid_transition(
                "A retired registry cannot be reconfigured.",
            ));
        }

        let revision = self
            .revision
            .checked_add(1)
            .ok_or_else(|| invalid_registry(
                "Registry revision counter is exhausted.",
            ))?;

        Ok(Self {
            id: self.id.clone(),
            name,
            location,
            provider,
            status: self.status,
            revision,
        })
    }

    /// Creates a lifecycle state revision.
    fn revise_status(
        &self,
        status: RegistryStatus,
    ) -> Result<Self> {
        if self.status.is_retired() && status != RegistryStatus::Retired {
            return Err(invalid_transition(
                "A retired registry cannot be reactivated.",
            ));
        }

        let revision = self
            .revision
            .checked_add(1)
            .ok_or_else(|| invalid_registry(
                "Registry revision counter is exhausted.",
            ))?;

        Ok(Self {
            id: self.id.clone(),
            name: self.name.clone(),
            location: self.location.clone(),
            provider: self.provider.clone(),
            status,
            revision,
        })
    }
}

/// Immutable registry configuration snapshot.
///
/// Consumers should use snapshots rather than hold catalog locks while
/// performing provider, authentication or network operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistrySnapshot {
    registry: Arc<Registry>,
}

impl RegistrySnapshot {
    /// Creates a snapshot from a registry configuration.
    fn new(registry: Arc<Registry>) -> Self {
        Self { registry }
    }

    /// Returns the registry configuration.
    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    /// Returns the shared registry configuration.
    pub fn shared_registry(&self) -> Arc<Registry> {
        Arc::clone(&self.registry)
    }
}

/// Thread-safe in-process registry catalog.
///
/// The catalog owns registry configuration and lifecycle state. It does
/// not persist data across process restarts and does not provide
/// distributed consistency.
///
/// All mutations use optimistic revision checks. Callers must supply the
/// revision they previously observed, preventing silent lost updates.
///
/// No provider code is executed while a catalog lock is held.
#[derive(Debug, Default)]
pub struct RegistryCatalog {
    entries: RwLock<BTreeMap<RegistryId, Arc<Registry>>>,
}

impl RegistryCatalog {
    /// Creates an empty registry catalog.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a new registry.
    ///
    /// Duplicate registry identities are rejected.
    pub fn register(
        &self,
        registry: Registry,
    ) -> Result<RegistrySnapshot> {
        let id = registry.id.clone();
        let mut entries = self.entries.write().map_err(|_| {
            catalog_lock_error("register")
        })?;

        if entries.contains_key(&id) {
            return Err(ToolchainError::new(
                ToolchainErrorKind::ValidationFailed,
                "A registry with this identity is already registered.",
            ));
        }

        let registry = Arc::new(registry);
        entries.insert(id, Arc::clone(&registry));

        Ok(RegistrySnapshot::new(registry))
    }

    /// Retrieves a registry snapshot by identity.
    pub fn get(
        &self,
        id: &RegistryId,
    ) -> Result<Option<RegistrySnapshot>> {
        let entries = self.entries.read().map_err(|_| {
            catalog_lock_error("get")
        })?;

        Ok(entries
            .get(id)
            .cloned()
            .map(RegistrySnapshot::new))
    }

    /// Returns all registry snapshots in deterministic identity order.
    pub fn list(&self) -> Result<Vec<RegistrySnapshot>> {
        let entries = self.entries.read().map_err(|_| {
            catalog_lock_error("list")
        })?;

        Ok(entries
            .values()
            .cloned()
            .map(RegistrySnapshot::new)
            .collect())
    }

    /// Returns enabled registry snapshots.
    pub fn list_selectable(
        &self,
    ) -> Result<Vec<RegistrySnapshot>> {
        let entries = self.entries.read().map_err(|_| {
            catalog_lock_error("list_selectable")
        })?;

        Ok(entries
            .values()
            .filter(|registry| registry.is_selectable())
            .cloned()
            .map(RegistrySnapshot::new)
            .collect())
    }

    /// Changes a registry's lifecycle state.
    ///
    /// `expected_revision` must equal the revision previously observed by
    /// the caller. The returned snapshot contains the new revision.
    pub fn set_status(
        &self,
        id: &RegistryId,
        expected_revision: u64,
        status: RegistryStatus,
    ) -> Result<RegistrySnapshot> {
        let mut entries = self.entries.write().map_err(|_| {
            catalog_lock_error("set_status")
        })?;

        let current = entries.get(id).ok_or_else(|| {
            registry_not_found(id)
        })?;

        check_revision(current, expected_revision)?;

        let revised = Arc::new(current.revise_status(status)?);

        entries.insert(id.clone(), Arc::clone(&revised));

        Ok(RegistrySnapshot::new(revised))
    }

    /// Reconfigures an existing registry.
    ///
    /// Registry identity cannot be changed through reconfiguration.
    /// Lifecycle state is preserved.
    pub fn reconfigure(
        &self,
        id: &RegistryId,
        expected_revision: u64,
        name: RegistryName,
        location: RegistryLocation,
        provider: RegistryProviderBinding,
    ) -> Result<RegistrySnapshot> {
        let mut entries = self.entries.write().map_err(|_| {
            catalog_lock_error("reconfigure")
        })?;

        let current = entries.get(id).ok_or_else(|| {
            registry_not_found(id)
        })?;

        check_revision(current, expected_revision)?;

        let revised = Arc::new(current.revise(
            name,
            location,
            provider,
        )?);

        entries.insert(id.clone(), Arc::clone(&revised));

        Ok(RegistrySnapshot::new(revised))
    }

    /// Removes a registry from the catalog.
    ///
    /// Removal is an administrative operation. Existing snapshots remain
    /// valid because they own shared immutable configuration.
    ///
    /// `expected_revision` prevents deleting a configuration that has
    /// changed since it was inspected.
    pub fn remove(
        &self,
        id: &RegistryId,
        expected_revision: u64,
    ) -> Result<RegistrySnapshot> {
        let mut entries = self.entries.write().map_err(|_| {
            catalog_lock_error("remove")
        })?;

        let current = entries.get(id).ok_or_else(|| {
            registry_not_found(id)
        })?;

        check_revision(current, expected_revision)?;

        let removed = entries.remove(id).ok_or_else(|| {
            registry_not_found(id)
        })?;

        Ok(RegistrySnapshot::new(removed))
    }

    /// Returns the number of registered registries.
    pub fn len(&self) -> Result<usize> {
        let entries = self.entries.read().map_err(|_| {
            catalog_lock_error("len")
        })?;

        Ok(entries.len())
    }

    /// Returns whether the catalog is empty.
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }
}

fn check_revision(
    current: &Registry,
    expected_revision: u64,
) -> Result<()> {
    if current.revision != expected_revision {
        return Err(ToolchainError::new(
            ToolchainErrorKind::ValidationFailed,
            "Registry configuration changed since it was last inspected.",
        ));
    }

    Ok(())
}

fn ranges_overlap(
    left: &VersionRange<RegistryProtocolVersion>,
    right: &VersionRange<RegistryProtocolVersion>,
) -> bool {
    // Compatibility is established by testing whether either range's
    // declared bounds can be contained in the other range.
    //
    // The exact comparison implementation must follow the canonical
    // VersionRange representation in version.rs.
    //
    // This conservative implementation checks the range boundaries
    // exposed by VersionRange::contains.
    match (left.minimum(), left.maximum(),
           right.minimum(), right.maximum()) {
        (Some(left_min), Some(left_max),
         Some(right_min), Some(right_max)) => {
            left.contains(right_min)
                || left.contains(right_max)
                || right.contains(left_min)
                || right.contains(left_max)
        }
        _ => true,
    }
}

fn invalid_registry(message: &str) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::InvalidInput,
        message,
    )
}

fn invalid_transition(message: &str) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ValidationFailed,
        message,
    )
}

fn registry_not_found(id: &RegistryId) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::InvalidInput,
        format!("Registry identity {id} is not registered."),
    )
}

fn catalog_lock_error(operation: &str) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ProviderFailure,
        format!(
            "Registry catalog lock could not be acquired during {operation}."
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registry_id(value: &str) -> RegistryId {
        RegistryId::new(value).expect("valid registry ID")
    }

    fn provider_id(value: &str) -> ProviderId {
        ProviderId::new(value).expect("valid provider ID")
    }

    fn registry() -> Registry {
        let provider = RegistryProviderBinding::new(
            provider_id("test-provider"),
            vec![
                VersionRange::any(),
            ],
        )
        .expect("valid provider binding");

        Registry::new(
            registry_id("test-registry"),
            RegistryName::new("Test Registry").expect("valid name"),
            RegistryLocation::new("test:registry").expect("valid location"),
            provider,
        )
    }

    #[test]
    fn new_registry_starts_disabled() {
        let registry = registry();

        assert_eq!(registry.status(), RegistryStatus::Disabled);
        assert_eq!(registry.revision(), 0);
        assert!(!registry.is_selectable());
    }

    #[test]
    fn registry_name_rejects_empty_value() {
        assert!(RegistryName::new("  ").is_err());
    }

    #[test]
    fn registry_location_rejects_control_characters() {
        assert!(RegistryLocation::new("registry\nlocation").is_err());
    }

    #[test]
    fn duplicate_registry_identity_is_rejected() {
        let catalog = RegistryCatalog::new();

        catalog.register(registry()).expect("first registration");

        assert!(
            catalog.register(registry()).is_err(),
            "duplicate identity must be rejected"
        );
    }

    #[test]
    fn registry_can_be_enabled() {
        let catalog = RegistryCatalog::new();
        let registered = catalog.register(registry()).expect("register");

        let enabled = catalog
            .set_status(
                registered.registry().id(),
                registered.registry().revision(),
                RegistryStatus::Enabled,
            )
            .expect("enable");

        assert_eq!(enabled.registry().status(), RegistryStatus::Enabled);
        assert!(enabled.registry().is_selectable());
        assert_eq!(enabled.registry().revision(), 1);
    }

    #[test]
    fn stale_revision_is_rejected() {
        let catalog = RegistryCatalog::new();
        let registered = catalog.register(registry()).expect("register");

        catalog
            .set_status(
                registered.registry().id(),
                0,
                RegistryStatus::Enabled,
            )
            .expect("enable");

        let result = catalog.set_status(
            registered.registry().id(),
            0,
            RegistryStatus::Disabled,
        );

        assert!(result.is_err());
    }

    #[test]
    fn retired_registry_is_terminal() {
        let catalog = RegistryCatalog::new();
        let registered = catalog.register(registry()).expect("register");
        let id = registered.registry().id().clone();

        let enabled = catalog
            .set_status(
                &id,
                registered.registry().revision(),
                RegistryStatus::Enabled,
            )
            .expect("enable");

        let retired = catalog
            .set_status(
                &id,
                enabled.registry().revision(),
                RegistryStatus::Retired,
            )
            .expect("retire");

        assert_eq!(
            retired.registry().status(),
            RegistryStatus::Retired
        );

        let result = catalog.set_status(
            &id,
            retired.registry().revision(),
            RegistryStatus::Enabled,
        );

        assert!(result.is_err());
    }

    #[test]
    fn retired_registry_cannot_be_reconfigured() {
        let catalog = RegistryCatalog::new();
        let registered = catalog.register(registry()).expect("register");
        let id = registered.registry().id().clone();

        let retired = catalog
            .set_status(
                &id,
                registered.registry().revision(),
                RegistryStatus::Retired,
            )
            .expect("retire");

        let result = catalog.reconfigure(
            &id,
            retired.registry().revision(),
            RegistryName::new("Replacement").expect("name"),
            RegistryLocation::new("replacement:registry").expect("location"),
            RegistryProviderBinding::new(
                provider_id("test-provider"),
                vec![VersionRange::any()],
            )
            .expect("provider"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn removal_does_not_invalidate_existing_snapshot() {
        let catalog = RegistryCatalog::new();
        let registered = catalog.register(registry()).expect("register");
        let id = registered.registry().id().clone();

        catalog
            .remove(
                &id,
                registered.registry().revision(),
            )
            .expect("remove");

        assert!(catalog.get(&id).expect("lookup").is_none());

        assert_eq!(
            registered.registry().name().as_str(),
            "Test Registry"
        );
    }

    #[test]
    fn list_is_deterministic() {
        let catalog = RegistryCatalog::new();

        let first = registry();

        let second = Registry::new(
            registry_id("another-registry"),
            RegistryName::new("Another Registry").expect("name"),
            RegistryLocation::new("another:registry").expect("location"),
            RegistryProviderBinding::new(
                provider_id("test-provider"),
                vec![VersionRange::any()],
            )
            .expect("provider"),
        );

        catalog.register(first).expect("register first");
        catalog.register(second).expect("register second");

        let listed = catalog.list().expect("list");

        assert_eq!(listed.len(), 2);
        assert!(
            listed[0].registry().id()
                <= listed[1].registry().id()
        );
    }

    #[test]
    fn empty_catalog_reports_empty() {
        let catalog = RegistryCatalog::new();

        assert!(catalog.is_empty().expect("empty"));
        assert_eq!(catalog.len().expect("length"), 0);
    }
}
