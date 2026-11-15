
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Device Discovery
//!
//! Path: src/zhal/devices/discovery.rs
//!
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Provides the device-facing discovery API for the universal ZHAL.
//!
//! This module coordinates discovery through the existing
//! provider-discovery authority.
//!
//! It exposes validated, filtered hardware-device observations
//! incrementally without accumulating the complete hardware inventory.
//!
//! Supported hardware domains are open-ended:
//!
//! - Classical computing.
//! - Quantum computing.
//! - HDL and programmable hardware.
//! - Accelerators.
//! - AI and tensor acceleration.
//! - Hybrid computing.
//! - Distributed computing.
//! - Embedded computing.
//! - Neuromorphic computing.
//! - Photonic computing.
//! - Analog computing.
//! - Future computational technologies.
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! THIS MODULE OWNS:
//!
//! - DeviceDiscoveryFilter.
//! - DeviceDiscoveryOptions.
//! - DeviceDiscoveryPage.
//! - DeviceDiscoverySession.
//! - Device-level filtering.
//! - Device-level discovery result composition.
//! - Device-facing discovery API.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - Provider identity.
//! - Hardware provider implementations.
//! - Provider registration.
//! - Provider lifecycle.
//! - Provider validation.
//! - Provider discovery coordination.
//! - Provider pagination.
//! - Device identity.
//! - Device descriptor definitions.
//! - Device inventory storage.
//! - Capability evaluation.
//! - Resource allocation.
//! - Resource reservation.
//! - Hardware authorization.
//! - Device sessions.
//! - Hardware communication.
//! - Quantum hardware discovery algorithms.
//! - HDL realization.
//! - Distributed execution.
//! - Hardware scheduling.
//! - Hardware routing.
//! - Runtime orchestration.
//!
//! These responsibilities remain with their canonical authorities.
//!
//! ============================================================================
//! CANONICAL AUTHORITIES
//! ============================================================================
//!
//! Provider identity:
//!     crate::stdlib::provider
//!
//! Device category:
//!     crate::stdlib::io::devices
//!
//! Hardware device identity:
//!     crate::zhal::identity
//!
//! Hardware device descriptor:
//!     crate::zhal::model::device
//!
//! Hardware provider:
//!     crate::zhal::providers::contract
//!
//! Provider registry:
//!     crate::zhal::providers::registry
//!
//! Provider discovery:
//!     crate::zhal::providers::discovery
//!
//! Operation context:
//!     crate::zhal::context
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! Device inventory:
//!     crate::zhal::devices::inventory
//!
//! Runtime resource management:
//!     crate::zur
//!
//! ============================================================================
//! DISCOVERY SEMANTICS
//! ============================================================================
//!
//! Discovery is observational.
//!
//! It does not:
//!
//! - Allocate physical resources.
//! - Reserve devices.
//! - Authorize access.
//! - Open device sessions.
//! - Select execution targets.
//! - Evaluate semantic feasibility.
//! - Guarantee future availability.
//! - Guarantee successful execution.
//!
//! A discovered device is an observation, not an allocation.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Providers.
//! - Devices.
//! - Hardware categories.
//! - Discovery pages.
//! - Device identifiers.
//! - Hardware technologies.
//! - Computational domains.
//!
//! Discovery is incremental.
//!
//! Only the current provider page and the state required by
//! the underlying provider-discovery coordinator are retained.
//!
//! Cross-page duplicate detection is delegated to the provider
//! discovery authority.
//!
//! Memory usage depends on the provider count, processed device
//! identities and the current discovery page.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ============================================================================
//! PROVIDER REGISTRY INTEGRATION
//! ============================================================================
//!
//! The existing ProviderRegistry stores Arc<dyn Provider>.
//!
//! This module requires Arc<dyn HardwareProvider>.
//!
//! These are distinct trait-object types.
//!
//! Callers must retain typed hardware-provider handles when
//! registering hardware providers, or introduce a future
//! explicitly typed registry integration.
//!
//! This module deliberately performs no unsafe downcasting.
//!
//! ============================================================================
//! GRAMMAR BOUNDARY
//! ============================================================================
//!
//! Relevant grammar:
//!
//! - grammar/hardware/devices.g4
//! - grammar/hardware/hardware.g4
//! - grammar/effects/hardware.g4
//! - grammar/execution/execution-context.g4
//! - grammar/execution/distributed-execution.g4
//!
//! Relevant specifications:
//!
//! - grammar/specification/semantic-model.md
//! - grammar/specification/execution-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/validation/semantic-boundaries.md
//!
//! Grammar and semantic analysis describe logical computational
//! intent and requirements.
//!
//! This module exposes observed physical hardware.
//!
//! It must never reinterpret source-language requirements,
//! capabilities, preferences, constraints or hints.
//!
//! ============================================================================
//! SAFETY
//! ============================================================================
//!
//! Unsafe Rust is forbidden.

use core::num::NonZeroUsize;

use std::sync::Arc;

use crate::stdlib::io::devices::DeviceKindId;

use crate::stdlib::provider::ProviderId;

use crate::zhal::context::ZhalContext;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::identity::HardwareDeviceIdentity;

use crate::zhal::model::device::{
    availability_state,
    HardwareDeviceDescriptor,
};

use crate::zhal::providers::contract::{
    DeviceDiscoveryCursor,
    HardwareProvider,
};

use crate::zhal::providers::discovery::{
    DiscoveryFailure,
    DiscoveryFailurePolicy,
    DiscoveryOptions,
    DiscoveryReport,
    DiscoverySession as ProviderDiscoverySession,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable device-discovery component identifier.
pub const COMPONENT: &str = "zhal.devices.discovery";

// ============================================================================
// DEVICE AVAILABILITY FILTER
// ============================================================================

/// Controls which provider-reported device availability states
/// are included in discovery results.
///
/// Availability remains an observation.
///
/// Selecting an availability filter does not establish a
/// reservation, authorization or execution guarantee.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeviceAvailabilityFilter {
    /// Include devices regardless of their reported availability.
    Any,

    /// Include devices explicitly reported as available.
    Available,

    /// Include devices explicitly reported as unavailable.
    Unavailable,

    /// Include devices whose availability is unknown.
    Unknown,

    /// Include one exact open-world provider-defined state.
    ///
    /// This permits future providers to introduce new availability
    /// classifications without modifying ZHAL.
    State(String),
}

impl Default for DeviceAvailabilityFilter {
    fn default() -> Self {
        Self::Any
    }
}

impl DeviceAvailabilityFilter {
    /// Returns whether a device matches this availability filter.
    #[must_use]
    pub fn matches(
        &self,
        device: &HardwareDeviceDescriptor,
    ) -> bool {
        let availability = device.availability();

        match self {
            Self::Any => true,

            Self::Available => {
                availability.is_available()
            }

            Self::Unavailable => {
                availability.is_unavailable()
            }

            Self::Unknown => {
                availability.is_unknown()
            }

            Self::State(expected) => {
                availability.state() == expected
            }
        }
    }

    /// Validates this filter.
    ///
    /// # Errors
    ///
    /// Returns an error if a custom state is empty,
    /// contains surrounding whitespace or contains
    /// control characters.
    pub fn validate(&self) -> ZhalResult<()> {
        if let Self::State(state) = self {
            if state.is_empty()
                || state.trim() != state
                || state.chars().any(char::is_control)
            {
                return Err(
                    discovery_error(
                        ZhalErrorKind::InvalidInput,
                        "device availability filter contains an invalid state",
                    ),
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// DEVICE DISCOVERY FILTER
// ============================================================================

/// Immutable device-discovery filtering criteria.
///
/// All populated criteria are combined using logical AND.
///
/// Provider and device-category identities reuse their
/// canonical repository definitions.
///
/// Categories remain open-world.
///
/// No physical hardware identity is required to discover
/// hardware satisfying a category filter.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DeviceDiscoveryFilter {
    provider: Option<ProviderId>,

    category: Option<DeviceKindId>,

    availability: DeviceAvailabilityFilter,
}

impl DeviceDiscoveryFilter {
    /// Creates an unrestricted device filter.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Restricts discovery to one provider.
    #[must_use]
    pub fn with_provider(
        mut self,
        provider: ProviderId,
    ) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Restricts discovery to one open-world device category.
    #[must_use]
    pub fn with_category(
        mut self,
        category: DeviceKindId,
    ) -> Self {
        self.category = Some(category);
        self
    }

    /// Sets the device availability filter.
    #[must_use]
    pub fn with_availability(
        mut self,
        availability: DeviceAvailabilityFilter,
    ) -> Self {
        self.availability = availability;
        self
    }

    /// Returns the optional provider restriction.
    #[must_use]
    pub fn provider(&self) -> Option<&ProviderId> {
        self.provider.as_ref()
    }

    /// Returns the optional device-category restriction.
    #[must_use]
    pub fn category(&self) -> Option<&DeviceKindId> {
        self.category.as_ref()
    }

    /// Returns the availability filter.
    #[must_use]
    pub fn availability(&self) -> &DeviceAvailabilityFilter {
        &self.availability
    }

    /// Validates the complete filter.
    ///
    /// # Errors
    ///
    /// Returns an error if an individual filter is invalid.
    pub fn validate(&self) -> ZhalResult<()> {
        self.availability.validate()
    }

    /// Returns whether a device satisfies every filter.
    #[must_use]
    pub fn matches(
        &self,
        device: &HardwareDeviceDescriptor,
    ) -> bool {
        if let Some(provider) = &self.provider {
            if device.provider_id() != provider {
                return false;
            }
        }

        if let Some(category) = &self.category {
            if device.category() != category {
                return false;
            }
        }

        self.availability.matches(device)
    }
}

// ============================================================================
// DISCOVERY OPTIONS
// ============================================================================

/// Immutable configuration for one device-discovery session.
///
/// This type composes the existing provider-discovery options.
///
/// It does not introduce a second provider-discovery policy.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DeviceDiscoveryOptions {
    provider_options: DiscoveryOptions,

    filter: DeviceDiscoveryFilter,
}

impl DeviceDiscoveryOptions {
    /// Creates default device-discovery options.
    ///
    /// Defaults:
    ///
    /// - Provider-defined page size.
    /// - Best-effort provider failure handling.
    /// - No device filtering.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets an optional operational page-size request.
    ///
    /// This is not a universal hardware or language-level limit.
    ///
    /// Providers may return fewer devices than requested.
    #[must_use]
    pub fn with_page_size(
        mut self,
        page_size: Option<NonZeroUsize>,
    ) -> Self {
        self.provider_options =
            self.provider_options.with_page_size(page_size);

        self
    }

    /// Sets the provider failure policy.
    #[must_use]
    pub fn with_failure_policy(
        mut self,
        policy: DiscoveryFailurePolicy,
    ) -> Self {
        self.provider_options =
            self.provider_options.with_failure_policy(policy);

        self
    }

    /// Sets device-level filtering criteria.
    #[must_use]
    pub fn with_filter(
        mut self,
        filter: DeviceDiscoveryFilter,
    ) -> Self {
        self.filter = filter;
        self
    }

    /// Returns the provider-discovery options.
    #[must_use]
    pub fn provider_options(&self) -> &DiscoveryOptions {
        &self.provider_options
    }

    /// Returns the device filter.
    #[must_use]
    pub fn filter(&self) -> &DeviceDiscoveryFilter {
        &self.filter
    }

    /// Validates the complete options.
    ///
    /// # Errors
    ///
    /// Returns an error if the device filter is invalid.
    pub fn validate(&self) -> ZhalResult<()> {
        self.filter.validate()
    }
}

// ============================================================================
// DEVICE DISCOVERY PAGE
// ============================================================================

/// One provider-backed, device-filtered discovery page.
///
/// Devices are ordered by canonical hardware identity.
///
/// The page contains only matching device observations.
///
/// An empty page does not necessarily mean discovery is complete:
/// additional matching devices may appear on subsequent pages.
///
/// The continuation cursor remains owned by the originating provider.
#[derive(Clone, Debug)]
pub struct DeviceDiscoveryPage {
    provider_id: ProviderId,

    page_index: usize,

    devices: Vec<HardwareDeviceDescriptor>,

    next_cursor: Option<DeviceDiscoveryCursor>,

    snapshot_revision: Option<String>,

    observed_devices: usize,
}

impl DeviceDiscoveryPage {
    fn from_provider_page(
        page: crate::zhal::providers::discovery::ProviderDiscoveryPage,
        filter: &DeviceDiscoveryFilter,
    ) -> Self {
        let observed_devices = page.devices().len();

        let mut devices: Vec<_> = page
            .devices()
            .iter()
            .filter(|device| filter.matches(device))
            .cloned()
            .collect();

        devices.sort_by(|left, right| {
            left.identity().cmp(right.identity())
        });

        Self {
            provider_id: page.provider_id().clone(),
            page_index: page.page_index(),
            devices,
            next_cursor: page.next_cursor().cloned(),
            snapshot_revision: page
                .snapshot_revision()
                .map(str::to_owned),
            observed_devices,
        }
    }

    /// Returns the provider that produced this page.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the zero-based page index for this provider.
    #[must_use]
    pub const fn page_index(&self) -> usize {
        self.page_index
    }

    /// Returns the matching device descriptors.
    #[must_use]
    pub fn devices(&self) -> &[HardwareDeviceDescriptor] {
        &self.devices
    }

    /// Returns the number of matching devices.
    #[must_use]
    pub fn len(&self) -> usize {
        self.devices.len()
    }

    /// Returns whether this page contains no matching devices.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Returns the number of devices observed before filtering.
    #[must_use]
    pub const fn observed_devices(&self) -> usize {
        self.observed_devices
    }

    /// Returns the number of devices excluded by filtering.
    #[must_use]
    pub const fn filtered_devices(&self) -> usize {
        self.observed_devices - self.devices.len()
    }

    /// Returns the provider-owned continuation cursor.
    #[must_use]
    pub fn next_cursor(&self) -> Option<&DeviceDiscoveryCursor> {
        self.next_cursor.as_ref()
    }

    /// Returns the optional provider-defined snapshot revision.
    #[must_use]
    pub fn snapshot_revision(&self) -> Option<&str> {
        self.snapshot_revision.as_deref()
    }

    /// Returns whether the provider has completed this discovery stream.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.next_cursor.is_none()
    }

    /// Returns matching device identities in canonical order.
    #[must_use]
    pub fn identities(&self) -> Vec<HardwareDeviceIdentity> {
        self.devices
            .iter()
            .map(|device| device.identity().clone())
            .collect()
    }
}

// ============================================================================
// DEVICE DISCOVERY SESSION
// ============================================================================

/// Incremental device-discovery coordinator.
///
/// This session composes the existing provider-discovery session.
///
/// It does not implement a second provider scheduler, pagination
/// algorithm, provider lifecycle or duplicate-detection mechanism.
///
/// Providers are retained through reference-counted typed handles.
///
/// Removing a provider from its registry does not invalidate
/// an already-started discovery session.
///
/// Provider initialization and shutdown remain the responsibility
/// of ProviderLifecycleManager.
///
/// # Thread safety
///
/// A session is mutable and should have one coordinating owner.
///
/// Provider implementations retain their existing Send + Sync
/// requirements.
///
/// The session does not create shared global mutable state.
pub struct DeviceDiscoverySession {
    inner: ProviderDiscoverySession,

    filter: DeviceDiscoveryFilter,
}

impl DeviceDiscoverySession {
    /// Creates a device-discovery session.
    ///
    /// The provider iterator must contain typed hardware-provider
    /// implementations.
    ///
    /// Provider identities are validated and ordered by the
    /// underlying provider-discovery authority.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - discovery options are invalid;
    /// - provider identities are duplicated;
    /// - provider discovery cannot be initialized.
    pub fn new(
        providers: impl IntoIterator<Item = Arc<dyn HardwareProvider>>,
        options: DeviceDiscoveryOptions,
    ) -> ZhalResult<Self> {
        options.validate()?;

        let inner = ProviderDiscoverySession::new(
            providers,
            options.provider_options,
        )?;

        Ok(Self {
            inner,
            filter: options.filter,
        })
    }

    /// Returns the device filter associated with this session.
    #[must_use]
    pub fn filter(&self) -> &DeviceDiscoveryFilter {
        &self.filter
    }

    /// Returns the underlying provider-discovery report.
    ///
    /// This report describes provider observations and failures.
    /// It does not count only matching devices.
    #[must_use]
    pub fn report(&self) -> &DiscoveryReport {
        self.inner.report()
    }

    /// Returns all provider-discovery failures.
    #[must_use]
    pub fn failures(&self) -> &[DiscoveryFailure] {
        self.inner.report().failures()
    }

    /// Returns whether any provider failed.
    #[must_use]
    pub fn has_failures(&self) -> bool {
        self.inner.report().has_failures()
    }

    /// Returns whether the underlying provider discovery is complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.inner.is_complete()
    }

    /// Returns whether strict failure handling terminated discovery.
    #[must_use]
    pub fn is_terminated(&self) -> bool {
        self.inner.is_terminated()
    }

    /// Consumes the session and returns its final discovery report.
    #[must_use]
    pub fn into_report(self) -> DiscoveryReport {
        self.inner.into_report()
    }

    /// Retrieves the next provider-backed device page.
    ///
    /// At most one provider page is delivered per call.
    ///
    /// The underlying provider-discovery coordinator handles:
    ///
    /// - provider readiness;
    /// - provider enumeration;
    /// - pagination;
    /// - continuation cursor validation;
    /// - cursor-cycle detection;
    /// - cross-page duplicate detection;
    /// - provider failure isolation;
    /// - deterministic round-robin progression;
    /// - discovery accounting.
    ///
    /// This method applies the device-level filter to the
    /// already validated page.
    ///
    /// A page may contain zero matching devices while still
    /// having a continuation cursor.
    ///
    /// The caller must continue until `None` is returned or
    /// the session otherwise reaches a terminal state.
    ///
    /// Cancellation and deadline expiration are propagated
    /// by the underlying provider-discovery coordinator.
    ///
    /// # Errors
    ///
    /// Returns cancellation, deadline, strict-policy provider
    /// failures and other underlying discovery errors.
    pub async fn next_page(
        &mut self,
        context: &ZhalContext,
    ) -> ZhalResult<Option<DeviceDiscoveryPage>> {
        context.check_active()?;

        let page = self.inner.next_page(context).await?;

        context.check_active()?;

        Ok(page.map(|page| {
            DeviceDiscoveryPage::from_provider_page(
                page,
                &self.filter,
            )
        }))
    }
}

// ============================================================================
// SERVICE
// ============================================================================

/// Stateless entry point for creating device-discovery sessions.
///
/// The service owns no provider registry, hardware inventory,
/// global cache or mutable device state.
///
/// Provider registration and lifecycle are explicitly managed
/// by their canonical authorities.
#[derive(Clone, Copy, Debug, Default)]
pub struct DeviceDiscoveryService;

impl DeviceDiscoveryService {
    /// Creates a stateless discovery service.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Starts a new device-discovery session.
    ///
    /// The supplied providers must retain their typed
    /// HardwareProvider trait-object handles.
    ///
    /// # Errors
    ///
    /// Returns errors from session construction or validation.
    pub fn start(
        &self,
        providers: impl IntoIterator<Item = Arc<dyn HardwareProvider>>,
        options: DeviceDiscoveryOptions,
    ) -> ZhalResult<DeviceDiscoverySession> {
        DeviceDiscoverySession::new(
            providers,
            options,
        )
    }
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn discovery_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(kind, message)
        .in_component(COMPONENT)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::io::devices::DeviceKindId;

    use crate::zhal::model::device::{
        DeviceAvailability,
        HardwareDeviceDescriptor,
    };

    use crate::zhal::identity::{
        HardwareDeviceId,
        HardwareDeviceIdentity,
    };

    fn provider_id() -> ProviderId {
        ProviderId::new("test.provider")
            .expect("valid provider identity")
    }

    fn device(
        id: &str,
        category: &str,
        availability: &str,
    ) -> HardwareDeviceDescriptor {
        let provider = provider_id();

        let identity = HardwareDeviceIdentity::new(
            provider,
            HardwareDeviceId::new(id)
                .expect("valid device identity"),
        );

        HardwareDeviceDescriptor::new(
            identity,
            DeviceKindId::new(category)
                .expect("valid category"),
            id,
        )
        .expect("valid device descriptor")
        .with_availability(
            DeviceAvailability::new(
                availability,
                None,
            )
            .expect("valid availability"),
        )
        .expect("valid availability assignment")
    }

    #[test]
    fn unrestricted_filter_matches_every_device() {
        let filter = DeviceDiscoveryFilter::new();

        assert!(filter.matches(
            &device(
                "device.one",
                "classical.cpu",
                availability_state::AVAILABLE,
            )
        ));

        assert!(filter.matches(
            &device(
                "device.two",
                "quantum.qpu",
                availability_state::UNKNOWN,
            )
        ));
    }

    #[test]
    fn provider_filter_uses_canonical_identity() {
        let filter = DeviceDiscoveryFilter::new()
            .with_provider(provider_id());

        assert!(filter.matches(
            &device(
                "device.one",
                "classical.cpu",
                availability_state::AVAILABLE,
            )
        ));

        let other_provider =
            ProviderId::new("other.provider")
                .expect("valid provider");

        let identity = HardwareDeviceIdentity::new(
            other_provider,
            HardwareDeviceId::new("device.two")
                .expect("valid device"),
        );

        let other = HardwareDeviceDescriptor::new(
            identity,
            DeviceKindId::new("classical.cpu")
                .expect("valid category"),
            "Other device",
        )
        .expect("valid descriptor");

        assert!(!filter.matches(&other));
    }

    #[test]
    fn category_filter_supports_open_world_categories() {
        let filter = DeviceDiscoveryFilter::new()
            .with_category(
                DeviceKindId::new("future.photonic.processor")
                    .expect("valid category"),
            );

        assert!(filter.matches(
            &device(
                "device.one",
                "future.photonic.processor",
                availability_state::AVAILABLE,
            )
        ));

        assert!(!filter.matches(
            &device(
                "device.two",
                "classical.cpu",
                availability_state::AVAILABLE,
            )
        ));
    }

    #[test]
    fn unknown_availability_is_not_available() {
        let filter = DeviceDiscoveryFilter::new()
            .with_availability(
                DeviceAvailabilityFilter::Available,
            );

        assert!(!filter.matches(
            &device(
                "device.one",
                "classical.cpu",
                availability_state::UNKNOWN,
            )
        ));

        assert!(filter.matches(
            &device(
                "device.two",
                "classical.cpu",
                availability_state::AVAILABLE,
            )
        ));
    }

    #[test]
    fn provider_defined_availability_is_supported() {
        let filter = DeviceDiscoveryFilter::new()
            .with_availability(
                DeviceAvailabilityFilter::State(
                    "provider.maintenance_window".to_owned(),
                ),
            );

        assert!(filter.matches(
            &device(
                "device.one",
                "classical.cpu",
                "provider.maintenance_window",
            )
        ));
    }

    #[test]
    fn invalid_custom_availability_is_rejected() {
        let filter = DeviceDiscoveryFilter::new()
            .with_availability(
                DeviceAvailabilityFilter::State(
                    " invalid ".to_owned(),
                ),
            );

        assert!(filter.validate().is_err());
    }

    #[test]
    fn combined_filters_use_logical_and() {
        let filter = DeviceDiscoveryFilter::new()
            .with_provider(provider_id())
            .with_category(
                DeviceKindId::new("quantum.qpu")
                    .expect("category"),
            )
            .with_availability(
                DeviceAvailabilityFilter::Available,
            );

        assert!(filter.matches(
            &device(
                "device.one",
                "quantum.qpu",
                availability_state::AVAILABLE,
            )
        ));

        assert!(!filter.matches(
            &device(
                "device.two",
                "classical.cpu",
                availability_state::AVAILABLE,
            )
        ));

        assert!(!filter.matches(
            &device(
                "device.three",
                "quantum.qpu",
                availability_state::UNKNOWN,
            )
        ));
    }

    #[test]
    fn page_size_is_optional() {
        let options = DeviceDiscoveryOptions::new();

        assert_eq!(
            options.provider_options().page_size(),
            None,
        );
    }

    #[test]
    fn page_size_is_an_operational_budget() {
        let options = DeviceDiscoveryOptions::new()
            .with_page_size(
                NonZeroUsize::new(128),
            );

        assert_eq!(
            options.provider_options().page_size(),
            NonZeroUsize::new(128),
        );
    }

    #[test]
    fn best_effort_is_the_default_failure_policy() {
        let options = DeviceDiscoveryOptions::new();

        assert_eq!(
            options.provider_options().failure_policy(),
            DiscoveryFailurePolicy::BestEffort,
        );
    }

    #[test]
    fn availability_filter_defaults_to_any() {
        let filter = DeviceDiscoveryFilter::new();

        assert_eq!(
            filter.availability(),
            &DeviceAvailabilityFilter::Any,
        );
    }

    #[test]
    fn discovery_service_has_no_hidden_global_state() {
        let first = DeviceDiscoveryService::new();
        let second = DeviceDiscoveryService::new();

        assert_eq!(first, second);
    }

    #[test]
    fn device_filter_does_not_impose_inventory_limits() {
        let filter = DeviceDiscoveryFilter::new();

        for index in 0..4096 {
            let descriptor = device(
                &format!("device.{index}"),
                "classical.cpu",
                availability_state::AVAILABLE,
            );

            assert!(filter.matches(&descriptor));
        }
    }
}
