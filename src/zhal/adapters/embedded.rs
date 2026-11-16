
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Embedded Hardware Adapter
//!
//! Canonical path:
//!     src/zhal/adapters/embedded.rs
//!
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Integrates embedded and device-oriented hardware providers with
//! the universal ZHAL hardware-provider architecture.
//!
//! The adapter composes two existing contracts implemented by the
//! same provider:
//!
//! - `crate::zhal::providers::contract::HardwareProvider`
//! - `crate::stdlib::io::devices::DeviceProvider`
//!
//! The first contract supplies provider-neutral hardware discovery,
//! device inspection, interface inspection and hardware sessions.
//!
//! The second contract supplies generic byte-oriented device
//! discovery and communication.
//!
//! The adapter delegates to both contracts without translating
//! between incompatible discovery models or inventing hardware
//! behavior.
//!
//! # Supported hardware
//!
//! Providers may expose:
//!
//! - Microcontrollers.
//! - Embedded processors.
//! - Sensors.
//! - Actuators.
//! - Robotics hardware.
//! - Industrial controllers.
//! - Embedded accelerators.
//! - Communication and interconnect devices.
//! - Edge-computing devices.
//! - Remote embedded devices.
//! - Virtual and simulated embedded devices.
//! - Future embedded and physical-computing technologies.
//!
//! These are examples, not a closed enumeration.
//!
//! This adapter does not require an operating system.
//!
//! # Architectural ownership
//!
//! ZHAL owns:
//!
//! - Universal hardware-provider integration.
//! - Provider-neutral hardware discovery contracts.
//! - Hardware identity and descriptor contracts.
//! - Hardware interface descriptions.
//! - Provider-managed hardware session contracts.
//!
//! `stdlib::io::devices` owns:
//!
//! - Generic device identity.
//! - Generic device descriptors.
//! - Device access modes.
//! - Byte-oriented device transport.
//! - Cancellation-aware device connections.
//! - Generic device-provider contracts.
//!
//! ZUR owns:
//!
//! - Runtime device-I/O orchestration.
//! - Resource allocation and coordination.
//! - Runtime policy and execution integration.
//!
//! Embedded providers own:
//!
//! - Physical device discovery.
//! - Microcontroller-specific communication.
//! - Bus and peripheral access.
//! - Sensor protocols.
//! - Actuator protocols.
//! - Device-specific command and event semantics.
//! - Driver implementation.
//! - Provider-specific hardware sessions.
//! - Device-specific failure handling.
//!
//! # Does not own
//!
//! This adapter does not implement:
//!
//! - Device drivers.
//! - Operating-system device enumeration.
//! - A new device registry.
//! - A new provider registry.
//! - A new provider lifecycle manager.
//! - A new device identity system.
//! - A new hardware identity system.
//! - A new byte-stream implementation.
//! - A new sensor event model.
//! - A new actuator command model.
//! - A new microcontroller instruction set.
//! - A new hardware protocol.
//! - Physical resource allocation.
//! - Hardware scheduling.
//! - Hardware routing.
//! - Distributed execution.
//! - Quantum execution.
//! - HDL synthesis or realization.
//! - Compiler lowering.
//! - Source-language semantic analysis.
//! - A new intermediate representation.
//!
//! These responsibilities remain with their canonical authorities.
//!
//! # Composite-provider contract
//!
//! `EmbeddedHardwareAdapter<P>` requires `P` to implement both
//! `HardwareProvider` and `DeviceProvider`.
//!
//! Both contracts inherit the same canonical `Provider` identity.
//! Consequently, the adapter cannot accidentally combine a hardware
//! provider and a byte-I/O provider with different provider identities.
//!
//! The underlying provider remains responsible for keeping its
//! hardware discovery and generic device-I/O observations consistent.
//!
//! A provider should derive both views from its authoritative device
//! inventory or discovery source. It must not maintain competing
//! inventories merely to satisfy the two interfaces.
//!
//! # Discovery and scalability
//!
//! ZHAL hardware discovery is delegated to
//! `HardwareProvider::discover_devices_page`.
//!
//! This preserves:
//!
//! - Provider-owned continuation cursors.
//! - Optional caller-requested page sizes.
//! - Provider-defined inventory revisions.
//! - Incremental discovery.
//! - Provider-scoped hardware identities.
//! - Provider-reported availability.
//! - Discovery errors and cancellation.
//!
//! Generic `DeviceProvider` discovery retains its existing
//! standard-library contract. Its `discover_devices` convenience
//! method returns a complete `Vec<DeviceDescriptor>`.
//!
//! Consumers requiring scalable or incremental hardware discovery
//! should use the paginated ZHAL discovery interface rather than
//! materializing a complete generic device-I/O inventory.
//!
//! This adapter introduces no arbitrary maximum for:
//!
//! - Embedded devices.
//! - Microcontrollers.
//! - Sensors.
//! - Actuators.
//! - Providers.
//! - Hardware interfaces.
//! - Device connections.
//! - Hardware resources.
//! - Device identifier lengths.
//! - Provider-defined device categories.
//! - Supported protocols.
//! - Computational domains.
//!
//! Actual limits are determined by physical hardware, provider
//! implementations, available memory, address-space representation,
//! communication facilities and explicitly configured operational
//! budgets.
//!
//! Literal infinite physical resources are not promised.
//!
//! # Device communication
//!
//! Generic byte-oriented communication is delegated to
//! `stdlib::io::devices`.
//!
//! A provider may implement that interface for devices for which
//! byte-stream semantics are appropriate.
//!
//! Not every sensor or actuator is a byte stream.
//!
//! Event-oriented sensors, sampled-data interfaces, typed actuator
//! commands, timing-sensitive control and other specialized protocols
//! must retain their provider-specific or domain-specific interfaces.
//!
//! The adapter does not reinterpret those protocols as byte streams.
//!
//! # Identity and lifecycle
//!
//! Provider identity is inherited from the wrapped provider.
//!
//! Hardware device and interface identities are delegated unchanged.
//!
//! Hardware sessions are delegated unchanged.
//!
//! Generic device connections are delegated unchanged.
//!
//! The adapter does not manufacture physical device identities,
//! hardware sessions, access rights, reservations or availability.
//!
//! # Cancellation and errors
//!
//! The supplied `OperationContext` and `ZhalContext` are forwarded
//! unchanged to the corresponding provider contract.
//!
//! The adapter does not create a second cancellation mechanism,
//! deadline mechanism, retry policy or error hierarchy.
//!
//! Provider errors are returned unchanged.
//!
//! Cancellation remains cooperative and provider-aware.
//!
//! Closing a generic device connection remains possible according
//! to the existing `DeviceConnection` cleanup contract.
//!
//! # Security
//!
//! This adapter stores no credentials.
//!
//! It does not authenticate callers, authorize device access,
//! establish provider trust or implement isolation.
//!
//! Those responsibilities remain with the existing provider,
//! ZHAL security and ZUR security authorities.
//!
//! Device discovery does not grant permission to open a device.
//!
//! # Thread safety
//!
//! The adapter holds its provider through `Arc`.
//!
//! The canonical provider contracts require `Send + Sync`.
//!
//! Individual `DeviceConnection` and `HardwareSession` instances
//! retain the ownership and concurrency semantics of their
//! respective contracts.
//!
//! The adapter introduces no global mutable state.
//!
//! # Grammar boundary
//!
//! Relevant source grammar:
//!
//! - `grammar/hardware/devices.g4`
//! - `grammar/effects/hardware.g4`
//! - `grammar/effects/io.g4`
//! - `grammar/execution/execution-context.g4`
//! - `grammar/hybrid/host-device.g4`
//!
//! Relevant normative specifications:
//!
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/execution-model.md`
//!
//! These sources describe logical intent and execution contracts.
//! This adapter operates after parsing and semantic validation.
//!
//! It does not parse source, interpret requirements, select hardware,
//! allocate resources or alter program semantics.
//!
//! # Integration
//!
//! Provider identity, metadata and lifecycle:
//!     `crate::stdlib::provider`
//!
//! Generic device I/O:
//!     `crate::stdlib::io::devices`
//!
//! Operation context and cancellation:
//!     `crate::stdlib::context`
//!
//! ZHAL hardware provider:
//!     `crate::zhal::providers::contract`
//!
//! Hardware identity:
//!     `crate::zhal::identity`
//!
//! Hardware descriptions:
//!     `crate::zhal::model`
//!
//! Hardware errors:
//!     `crate::zhal::errors`
//!
//! ZUR device I/O:
//!     `crate::zur::io::devices`
//!
//! Runtime orchestration:
//!     `crate::zur`
//!
//! # Completion invariants
//!
//! This adapter is complete when:
//!
//! 1. It delegates the canonical Provider contract.
//! 2. It delegates the canonical HardwareProvider contract.
//! 3. It delegates the canonical DeviceProvider contract.
//! 4. Both device contracts use the same provider identity.
//! 5. Paginated hardware discovery remains available.
//! 6. Generic device connections retain their original semantics.
//! 7. Hardware sessions retain their original semantics.
//! 8. Cancellation and errors are propagated unchanged.
//! 9. Provider lifecycle is delegated only when supported.
//! 10. No OS or vendor assumption is introduced.
//! 11. No arbitrary hardware-capacity ceiling is introduced.
//! 12. No duplicate registry, allocator, scheduler or protocol is added.
//! 13. No unsafe Rust is used.
//! 14. Public API and integration tests pass.

use core::fmt;

use std::sync::Arc;

use crate::stdlib::capability::CapabilitySnapshot;

use crate::stdlib::context::OperationContext;

use crate::stdlib::io::devices::{
    DeviceAccess,
    DeviceConnection,
    DeviceDescriptor,
    DeviceProvider,
    DeviceTransport,
};

use crate::stdlib::provider::{
    Provider,
    ProviderDescriptor,
    ProviderFuture,
    ProviderLifecycle,
    ProviderStatus,
};

use crate::zhal::context::ZhalContext;

use crate::zhal::identity::{
    HardwareDeviceIdentity,
    HardwareInterfaceIdentity,
};

use crate::zhal::model::{
    HardwareDeviceDescriptor,
    HardwareInterfaceDescriptor,
};

use crate::zhal::providers::contract::{
    DeviceDiscoveryPage,
    DeviceDiscoveryRequest,
    HardwareProvider,
    HardwareProviderFuture,
    HardwareSession,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable component identifier.
pub const COMPONENT: &str = "zhal.adapters.embedded";

/// Stable embedded-adapter schema identifier.
pub const EMBEDDED_ADAPTER_SCHEMA_ID: &str =
    "zamani.zhal.adapters.embedded";

/// Embedded-adapter integration schema version.
///
/// This is not the language version, Cargo package version,
/// provider API version, firmware version or hardware revision.
pub const EMBEDDED_ADAPTER_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// ADAPTER
// ============================================================================

/// Integrates one canonical embedded provider with ZHAL and
/// the standard-library device-I/O interface.
///
/// The wrapped provider must implement both:
///
/// - [`HardwareProvider`], for universal hardware discovery,
///   inspection, interface descriptions and hardware sessions.
/// - [`DeviceProvider`], for generic byte-oriented device discovery
///   and communication.
///
/// Both contracts inherit the same canonical [`Provider`] identity.
///
/// The adapter shares ownership of the provider through `Arc`.
/// It does not initialize, register, authenticate, authorize or
/// shut down the provider during construction.
///
/// # Type parameters
///
/// `P` may be a concrete embedded provider or an existing trait object
/// implementing both required contracts.
///
/// # Scalability
///
/// The adapter adds no provider or device collection of its own.
/// Paginated hardware discovery is delegated directly to the provider.
/// No arbitrary device, interface, connection or resource ceiling
/// is introduced.
#[derive(Clone)]
pub struct EmbeddedHardwareAdapter<
    P: HardwareProvider + DeviceProvider + ?Sized,
> {
    provider: Arc<P>,
}

impl<P> EmbeddedHardwareAdapter<P>
where
    P: HardwareProvider + DeviceProvider + ?Sized,
{
    /// Constructs an embedded hardware adapter around an existing
    /// provider implementation.
    ///
    /// Construction is intentionally side-effect free.
    ///
    /// It performs no:
    ///
    /// - Hardware discovery.
    /// - Device enumeration.
    /// - Provider initialization.
    /// - Provider registration.
    /// - Authentication.
    /// - Authorization.
    /// - Device access.
    /// - Resource reservation.
    /// - Runtime allocation.
    ///
    /// The provider implements both hardware and generic device-I/O
    /// contracts, so both interfaces necessarily share its canonical
    /// provider identity.
    ///
    /// The caller remains responsible for provider admission,
    /// lifecycle management, trust verification and authorization.
    #[must_use]
    pub fn new(provider: Arc<P>) -> Self {
        Self { provider }
    }

    /// Returns a borrowed reference to the wrapped provider.
    ///
    /// The provider remains the authority for hardware discovery,
    /// generic device I/O, capability observations and implementation
    /// behavior.
    #[must_use]
    pub fn provider(&self) -> &P {
        self.provider.as_ref()
    }

    /// Returns a shared ownership handle to the wrapped provider.
    ///
    /// Cloning this handle does not clone the physical provider or
    /// create another provider identity.
    #[must_use]
    pub fn provider_handle(&self) -> Arc<P> {
        Arc::clone(&self.provider)
    }

    /// Returns the canonical provider descriptor.
    ///
    /// The descriptor is provider metadata. It is not live evidence
    /// that a particular device is available.
    #[must_use]
    pub fn descriptor(&self) -> &ProviderDescriptor {
        self.provider.descriptor()
    }

    /// Returns the provider's current observed lifecycle status.
    ///
    /// This is an observation and does not guarantee that a subsequent
    /// hardware or device-I/O operation will succeed.
    #[must_use]
    pub fn status(&self) -> ProviderStatus {
        self.provider.status()
    }

    /// Returns whether the provider status currently permits operations.
    ///
    /// This checks only the canonical provider lifecycle status.
    /// It does not establish device availability, capability
    /// compatibility, authorization or resource feasibility.
    #[must_use]
    pub fn may_accept_operations(&self) -> bool {
        self.status().may_accept_operations()
    }

    /// Discovers generic byte-oriented devices through the canonical
    /// standard-library device-provider contract.
    ///
    /// The underlying standard-library method returns a complete
    /// `Vec<DeviceDescriptor>`. It validates provider ownership and
    /// duplicate device identities.
    ///
    /// This convenience method is appropriate when the provider's
    /// generic device inventory can reasonably be materialized.
    ///
    /// For incremental or large-scale hardware discovery, use the
    /// paginated [`HardwareProvider::discover_devices_page`] contract
    /// instead.
    ///
    /// This method does not reserve devices or grant access.
    ///
    /// # Cancellation
    ///
    /// The supplied operation context is passed directly to the
    /// canonical device provider.
    ///
    /// # Errors
    ///
    /// Returns the original standard-library provider error.
    pub fn discover_io_devices<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, Vec<DeviceDescriptor>> {
        DeviceProvider::discover_devices(
            self.provider.as_ref(),
            context,
        )
    }

    /// Opens a generic byte-oriented device connection.
    ///
    /// The canonical `DeviceProvider::open` method validates provider
    /// ownership, checks cancellation and delegates the actual access
    /// request to the provider.
    ///
    /// The provider must revalidate that the device is still available
    /// and enforce its applicable access policy.
    ///
    /// The returned connection retains the standard-library access,
    /// cancellation, transfer and closure semantics.
    ///
    /// This method is not a substitute for specialized sensor,
    /// actuator, robotics or microcontroller protocol interfaces.
    ///
    /// # Errors
    ///
    /// Returns the original standard-library error if the descriptor
    /// is invalid, access is denied, the device is unavailable,
    /// cancellation is requested or opening fails.
    pub fn open_io_device<'a>(
        &'a self,
        descriptor: &'a DeviceDescriptor,
        access: DeviceAccess,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, DeviceConnection> {
        DeviceProvider::open(
            self.provider.as_ref(),
            descriptor,
            access,
            context,
        )
    }
}

// ============================================================================
// COMMON PROVIDER INTEGRATION
// ============================================================================

/// Delegates provider identity, status and capability observations
/// to the wrapped canonical provider.
///
/// No second provider identity or capability authority is introduced.
impl<P> Provider for EmbeddedHardwareAdapter<P>
where
    P: HardwareProvider + DeviceProvider + ?Sized,
{
    fn descriptor(&self) -> &ProviderDescriptor {
        self.provider.descriptor()
    }

    fn status(&self) -> ProviderStatus {
        self.provider.status()
    }

    fn capability_snapshot<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, CapabilitySnapshot> {
        self.provider.capability_snapshot(context)
    }
}

// ============================================================================
// UNIVERSAL HARDWARE PROVIDER INTEGRATION
// ============================================================================

/// Delegates universal hardware operations to the existing provider.
///
/// Hardware descriptors, identities, discovery pages, interface
/// descriptions, errors and sessions are returned unchanged.
///
/// In particular, this implementation does not convert the
/// standard-library full-inventory discovery API into a fabricated
/// paginated API. The provider's native ZHAL pagination contract
/// remains authoritative.
impl<P> HardwareProvider for EmbeddedHardwareAdapter<P>
where
    P: HardwareProvider + DeviceProvider + ?Sized,
{
    fn discover_devices_page<'a>(
        &'a self,
        request: &'a DeviceDiscoveryRequest,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, DeviceDiscoveryPage> {
        self.provider
            .discover_devices_page(request, context)
    }

    fn inspect_device<'a>(
        &'a self,
        identity: &'a HardwareDeviceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, HardwareDeviceDescriptor> {
        self.provider
            .inspect_device(identity, context)
    }

    fn describe_interface<'a>(
        &'a self,
        identity: &'a HardwareInterfaceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, HardwareInterfaceDescriptor> {
        self.provider
            .describe_interface(identity, context)
    }

    fn open_session<'a>(
        &'a self,
        device: &'a HardwareDeviceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, Box<dyn HardwareSession>> {
        self.provider
            .open_session(device, context)
    }
}

// ============================================================================
// GENERIC DEVICE-PROVIDER INTEGRATION
// ============================================================================

/// Delegates generic device discovery and transport creation to
/// the canonical standard-library device provider.
///
/// `DeviceProvider` remains the authority for:
///
/// - Provider-scoped generic device descriptors.
/// - Byte-oriented device transports.
/// - Access-mode enforcement.
/// - Device availability revalidation.
/// - Device connection creation.
/// - Device-specific I/O errors.
///
/// The adapter does not wrap, copy or reinterpret a transport.
impl<P> DeviceProvider for EmbeddedHardwareAdapter<P>
where
    P: HardwareProvider + DeviceProvider + ?Sized,
{
    fn list_devices<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, Vec<DeviceDescriptor>> {
        self.provider.list_devices(context)
    }

    fn open_transport<'a>(
        &'a self,
        descriptor: &'a DeviceDescriptor,
        access: DeviceAccess,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, Box<dyn DeviceTransport>> {
        self.provider
            .open_transport(descriptor, access, context)
    }
}

// ============================================================================
// EXISTING PROVIDER LIFECYCLE INTEGRATION
// ============================================================================

/// Delegates initialization and shutdown only when the wrapped provider
/// already implements the canonical lifecycle contract.
///
/// This conditional implementation does not create a second lifecycle
/// state machine or lifecycle manager.
impl<P> ProviderLifecycle for EmbeddedHardwareAdapter<P>
where
    P: HardwareProvider + DeviceProvider + ProviderLifecycle + ?Sized,
{
    fn initialize<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, ()> {
        self.provider.initialize(context)
    }

    fn shutdown<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, ()> {
        self.provider.shutdown(context)
    }
}

// ============================================================================
// DEBUG
// ============================================================================

/// Formats the adapter without exposing provider internals.
///
/// Provider implementations may hold device handles, transport state,
/// credentials or other sensitive implementation details. Consequently,
/// the adapter reports only canonical provider identity and status.
impl<P> fmt::Debug for EmbeddedHardwareAdapter<P>
where
    P: HardwareProvider + DeviceProvider + ?Sized,
{
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("EmbeddedHardwareAdapter")
            .field(
                "provider_id",
                &self.provider.descriptor().id(),
            )
            .field(
                "provider_status",
                &self.provider.status(),
            )
            .finish()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_schema_identity_is_stable() {
        assert_eq!(
            EMBEDDED_ADAPTER_SCHEMA_ID,
            "zamani.zhal.adapters.embedded",
        );

        assert_eq!(
            EMBEDDED_ADAPTER_SCHEMA_VERSION,
            1,
        );
    }

    #[test]
    fn component_identity_is_stable() {
        assert_eq!(
            COMPONENT,
            "zhal.adapters.embedded",
        );
    }
}
