
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Provider Contract
//!
//! Path: src/zhal/providers/contract.rs
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Defines the universal hardware-provider integration contract
//! for Zamani.
//!
//! A hardware provider exposes physical, virtual, simulated,
//! remote, embedded, reconfigurable or future computational
//! hardware through the canonical ZHAL boundary.
//!
//! Supported computational domains include:
//!
//! - Classical computing.
//! - Quantum computing.
//! - Hardware description and realization.
//! - GPUs and other accelerators.
//! - AI and tensor acceleration.
//! - Hybrid computing.
//! - Distributed computing.
//! - Embedded computing.
//! - Networking and interconnects.
//! - Neuromorphic computing.
//! - Photonic computing.
//! - Analog computing.
//! - Future computational technologies.
//!
//! These domains are open-ended.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - HardwareProvider.
//! - HardwareSession.
//! - HardwareProviderFuture.
//! - DeviceDiscoveryCursor.
//! - DeviceDiscoveryRequest.
//! - DeviceDiscoveryPage.
//! - Provider response validation.
//! - Provider-scoped discovery contracts.
//! - Provider-scoped device inspection.
//! - Provider-scoped interface inspection.
//! - Provider-scoped device session creation.
//!
//! This module does NOT own:
//!
//! - Provider identity.
//! - Provider lifecycle.
//! - Provider registration.
//! - Provider discovery or dynamic library loading.
//! - Provider authentication.
//! - Provider trust verification.
//! - Provider authorization policy.
//! - Capability identity or capability evaluation.
//! - Semantic requirements.
//! - Semantic resource requirements.
//! - Hardware feasibility.
//! - Resource allocation.
//! - Resource reservation.
//! - Resource accounting.
//! - Device inventory storage.
//! - Hardware topology.
//! - Hardware scheduling.
//! - Hardware routing.
//! - Classical execution semantics.
//! - Quantum execution semantics.
//! - HDL realization semantics.
//! - Distributed execution.
//! - Hybrid execution.
//! - Runtime orchestration.
//! - Compiler or IR definitions.
//! - Generic byte-oriented device transport.
//!
//! These responsibilities remain with their canonical
//! repository authorities.
//!
//! # Canonical dependencies
//!
//! Provider identity and lifecycle:
//!     crate::stdlib::provider
//!
//! Capability contracts:
//!     crate::stdlib::capability
//!
//! Operation context:
//!     crate::stdlib::context
//!
//! Hardware context:
//!     crate::zhal::context
//!
//! Hardware identities:
//!     crate::zhal::identity
//!
//! Hardware device descriptions:
//!     crate::zhal::model::device
//!
//! Hardware interface descriptions:
//!     crate::zhal::model::interface
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! # Provider model
//!
//! A hardware provider is an implementation of the existing
//! standard-library Provider contract plus the ZHAL-specific
//! hardware integration contract.
//!
//! A provider must expose:
//!
//! - Immutable canonical provider metadata.
//! - Current provider status.
//! - Evidence-backed capability observations.
//! - Paginated device discovery.
//! - Device inspection.
//! - Hardware interface inspection.
//! - Device session creation.
//!
//! Domain-specific operations remain owned by their respective
//! provider interfaces.
//!
//! # Execution boundary
//!
//! ZHAL does not introduce a universal execute_instruction()
//! method.
//!
//! CPU instructions, quantum circuits, quantum pulses,
//! HDL realization operations, accelerator workloads and
//! distributed operations have different semantics.
//!
//! Their specialized interfaces remain authoritative.
//!
//! A hardware session establishes a provider-managed access
//! lifecycle. It does not replace the specialized operation
//! interfaces.
//!
//! # Discovery
//!
//! Device discovery is observational.
//!
//! Discovery:
//!
//! - Does not allocate hardware.
//! - Does not reserve resources.
//! - Does not authorize access.
//! - Does not guarantee future availability.
//! - Does not select a target.
//! - Does not guarantee successful execution.
//!
//! Discovery supports pagination so providers do not need
//! to materialize their complete hardware inventory in memory.
//!
//! # POCO-REAF
//!
//! Provider identities, device categories, hardware interfaces,
//! operation identifiers and discovery cursors remain open-world.
//!
//! No arbitrary maximum is imposed on:
//!
//! - Hardware providers.
//! - Hardware devices.
//! - Discovery pages.
//! - Discovery operations.
//! - Hardware categories.
//! - Hardware interfaces.
//! - Provider operations.
//! - Computational domains.
//! - Hardware technologies.
//! - Device identifier lengths.
//! - Discovery cursor lengths.
//!
//! A caller may supply an operational page-size budget.
//! The provider may also apply its explicitly configured
//! operational policies.
//!
//! Such budgets are not language-level hardware limitations.
//!
//! Actual execution remains constrained by available resources,
//! operating-system capabilities and provider implementations.
//!
//! Literal infinite physical computation is not promised.
//!
//! # Security
//!
//! Provider descriptors and discovery requests must not
//! contain credentials.
//!
//! Provider implementations must enforce applicable
//! authentication and authorization requirements.
//!
//! Registration does not establish trust.
//!
//! Discovery does not grant hardware access.
//!
//! Hardware sessions must not expose credentials through
//! Debug implementations or ordinary diagnostic output.
//!
//! # Grammar boundary
//!
//! Relevant grammar:
//!
//! - grammar/effects/hardware.g4
//! - grammar/execution/execution-context.g4
//! - grammar/execution/distributed-execution.g4
//!
//! Relevant normative specifications:
//!
//! - grammar/specification/semantic-model.md
//! - grammar/validation/semantic-boundaries.md
//! - grammar/specification/execution-model.md
//!
//! The grammar and semantic model express computational intent.
//!
//! This contract exposes physical hardware observations.
//!
//! It does not parse or reinterpret source-level requirements,
//! capabilities, preferences, constraints, hints or feasibility.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.

use core::fmt;
use core::future::Future;
use core::num::NonZeroUsize;
use core::pin::Pin;

use std::collections::BTreeSet;

use crate::stdlib::provider::{
    Provider,
    ProviderId,
};

use crate::zhal::context::ZhalContext;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::identity::{
    HardwareDeviceIdentity,
    HardwareInterfaceIdentity,
    HardwareSessionIdentity,
};

use crate::zhal::model::{
    HardwareDeviceDescriptor,
    HardwareInterfaceDescriptor,
};

// ============================================================================
// ASYNCHRONOUS HARDWARE PROVIDER OPERATIONS
// ============================================================================

/// Future returned by a ZHAL hardware-provider operation.
///
/// The future is Send so that providers can participate in
/// multithreaded runtime environments.
///
/// Its lifetime is tied to the provider and operation context.
///
/// ZHAL does not provide an asynchronous executor.
///
/// Execution is delegated to the canonical runtime.
pub type HardwareProviderFuture<'a, T> =
    Pin<Box<dyn Future<Output = ZhalResult<T>> + Send + 'a>>;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable component identifier.
pub const COMPONENT: &str = "zhal.providers.contract";

// ============================================================================
// COMMON VALIDATION
// ============================================================================

fn validate_text(
    value: &str,
    field: &'static str,
) -> ZhalResult<()> {
    if value.is_empty() || value.trim() != value {
        return Err(
            ZhalError::new(
                ZhalErrorKind::InvalidInput,
                format!("{field} must not be empty or contain surrounding whitespace"),
            )
            .in_component(COMPONENT),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ZhalError::new(
                ZhalErrorKind::InvalidInput,
                format!("{field} must not contain control characters"),
            )
            .in_component(COMPONENT),
        );
    }

    Ok(())
}

/// Checks cancellation and deadline conditions.
///
/// Cancellation delegates to the existing ZHAL error adapter.
///
/// Deadline observation delegates to ZhalContext.
///
/// No competing cancellation or deadline mechanism is created.
fn check_active(
    context: &ZhalContext,
) -> ZhalResult<()> {
    ZhalError::check_cancelled(
        context.operation_context(),
    )?;

    if context.is_deadline_expired() {
        return Err(
            ZhalError::new(
                ZhalErrorKind::OperationTimedOut,
                "the hardware-provider operation deadline has expired",
            )
            .in_component(COMPONENT),
        );
    }

    Ok(())
}

fn contract_violation(
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(
        ZhalErrorKind::ContractViolation,
        message,
    )
    .in_component(COMPONENT)
}

// ============================================================================
// DISCOVERY CURSOR
// ============================================================================

/// Opaque continuation token for paginated device discovery.
///
/// The token is owned by the provider that created it.
///
/// Consumers must not interpret, modify, manufacture or
/// transfer a cursor between unrelated providers.
///
/// Cursor contents must not contain credentials or secrets.
///
/// No arbitrary cursor-length limit is imposed.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DeviceDiscoveryCursor(String);

impl DeviceDiscoveryCursor {
    /// Creates a validated provider-owned discovery cursor.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty cursor, surrounding
    /// whitespace or control characters.
    pub fn new(
        value: impl Into<String>,
    ) -> ZhalResult<Self> {
        let value = value.into();

        validate_text(
            &value,
            "device discovery cursor",
        )?;

        Ok(Self(value))
    }

    /// Returns the opaque cursor contents.
    ///
    /// Consumers must treat this value as provider-owned data.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DeviceDiscoveryCursor {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// DISCOVERY REQUEST
// ============================================================================

/// Request for a provider-owned hardware discovery page.
///
/// The optional page size is an operational request, not
/// a universal hardware limit.
///
/// When omitted, the provider chooses its configured
/// discovery page size.
///
/// The provider may return fewer devices than requested.
///
/// A cursor must have been returned by the same provider
/// under the applicable discovery consistency contract.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DeviceDiscoveryRequest {
    cursor: Option<DeviceDiscoveryCursor>,
    page_size: Option<NonZeroUsize>,
}

impl DeviceDiscoveryRequest {
    /// Creates a discovery request.
    ///
    /// `cursor` continues an existing discovery operation.
    ///
    /// `page_size` optionally requests a nonzero page size.
    ///
    /// No default page size is imposed by ZHAL.
    #[must_use]
    pub fn new(
        cursor: Option<DeviceDiscoveryCursor>,
        page_size: Option<NonZeroUsize>,
    ) -> Self {
        Self {
            cursor,
            page_size,
        }
    }

    /// Creates the initial discovery request.
    #[must_use]
    pub fn initial() -> Self {
        Self::default()
    }

    /// Returns the continuation cursor, if present.
    #[must_use]
    pub fn cursor(&self) -> Option<&DeviceDiscoveryCursor> {
        self.cursor.as_ref()
    }

    /// Returns the requested page size, if supplied.
    #[must_use]
    pub const fn page_size(&self) -> Option<NonZeroUsize> {
        self.page_size
    }

    /// Returns whether this request continues an existing
    /// discovery operation.
    #[must_use]
    pub fn is_continuation(&self) -> bool {
        self.cursor.is_some()
    }
}

// ============================================================================
// DISCOVERY PAGE
// ============================================================================

/// One page of hardware-device discovery results.
///
/// A page contains:
///
/// - Provider-owned device descriptions.
/// - An optional continuation cursor.
/// - An optional provider-defined snapshot revision.
///
/// Absence of a continuation cursor means that the provider
/// declares discovery complete for the requested consistency scope.
///
/// A provider must not return a continuation cursor identical
/// to the request cursor because that would prevent progress.
///
/// The provider must define the consistency behavior of its
/// cursor and revision.
///
/// A page is not a reservation or allocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceDiscoveryPage {
    devices: Vec<HardwareDeviceDescriptor>,
    next_cursor: Option<DeviceDiscoveryCursor>,
    snapshot_revision: Option<String>,
}

impl DeviceDiscoveryPage {
    /// Constructs a discovery page.
    ///
    /// # Errors
    ///
    /// Returns an error when the optional snapshot revision
    /// is invalid.
    pub fn new(
        devices: Vec<HardwareDeviceDescriptor>,
        next_cursor: Option<DeviceDiscoveryCursor>,
        snapshot_revision: Option<String>,
    ) -> ZhalResult<Self> {
        if let Some(revision) = snapshot_revision.as_deref() {
            validate_text(
                revision,
                "discovery snapshot revision",
            )?;
        }

        Ok(Self {
            devices,
            next_cursor,
            snapshot_revision,
        })
    }

    /// Returns the devices contained in this page.
    ///
    /// Device order is provider-defined.
    ///
    /// Consumers requiring deterministic ordering should
    /// use canonical hardware identities.
    #[must_use]
    pub fn devices(&self) -> &[HardwareDeviceDescriptor] {
        &self.devices
    }

    /// Returns the continuation cursor, if more results exist.
    #[must_use]
    pub fn next_cursor(&self) -> Option<&DeviceDiscoveryCursor> {
        self.next_cursor.as_ref()
    }

    /// Returns the provider-defined snapshot revision.
    ///
    /// A revision may be absent when the provider cannot
    /// establish a consistent inventory revision.
    #[must_use]
    pub fn snapshot_revision(&self) -> Option<&str> {
        self.snapshot_revision.as_deref()
    }

    /// Returns the number of devices in this page.
    #[must_use]
    pub fn len(&self) -> usize {
        self.devices.len()
    }

    /// Returns whether this page contains no devices.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    /// Returns whether discovery is complete.
    ///
    /// This describes the provider's declared discovery result.
    /// It does not prove that no hardware can appear later.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.next_cursor.is_none()
    }

    /// Validates the page against its owning provider
    /// and originating request.
    ///
    /// Validation checks:
    ///
    /// - Every device belongs to the expected provider.
    /// - Every device descriptor is valid.
    /// - Device identities are unique within the page.
    /// - The continuation cursor makes progress.
    ///
    /// This method does not establish authorization,
    /// availability guarantees or resource feasibility.
    ///
    /// # Errors
    ///
    /// Returns a structured contract-violation error
    /// when the provider returns inconsistent data.
    pub fn validate_for(
        &self,
        provider_id: &ProviderId,
        request: &DeviceDiscoveryRequest,
    ) -> ZhalResult<()> {
        let mut identities = BTreeSet::new();

        for device in &self.devices {
            device.validate().map_err(|error| {
                ZhalError::new(
                    ZhalErrorKind::InvalidData,
                    format!(
                        "provider returned an invalid device descriptor: {error}"
                    ),
                )
                .in_component(COMPONENT)
                .with_provider(provider_id.clone())
            })?;

            if device.provider_id() != provider_id {
                return Err(
                    contract_violation(
                        "provider returned a device owned by another provider",
                    )
                    .with_provider(provider_id.clone()),
                );
            }

            if !identities.insert(device.identity().clone()) {
                return Err(
                    contract_violation(
                        "provider returned duplicate device identities within a discovery page",
                    )
                    .with_provider(provider_id.clone()),
                );
            }
        }

        if let Some(next_cursor) = self.next_cursor() {
            if request.cursor() == Some(next_cursor) {
                return Err(
                    contract_violation(
                        "provider returned an unchanged discovery continuation cursor",
                    )
                    .with_provider(provider_id.clone()),
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// HARDWARE SESSION
// ============================================================================

/// Provider-managed hardware access session.
///
/// A session represents an established provider-managed
/// access lifecycle for one hardware device.
///
/// It does not define a universal instruction set.
///
/// Domain-specific execution interfaces remain authoritative.
///
/// For example:
///
/// - Classical execution uses its existing execution contracts.
/// - Quantum execution uses quantum hardware contracts.
/// - HDL realization uses realization-provider contracts.
/// - Generic byte-oriented device I/O uses stdlib device I/O.
///
/// A session may internally manage connections, device handles,
/// remote channels or provider-owned resources.
///
/// Sessions must not expose credentials through ordinary
/// diagnostics.
///
/// Implementations must make successful repeated closure
/// harmless.
///
/// A session is exclusively managed by its owner unless its
/// specialized interface explicitly provides concurrency support.
pub trait HardwareSession: Send {
    /// Returns the provider-scoped session identity.
    fn identity(&self) -> &HardwareSessionIdentity;

    /// Returns the hardware device associated with this session.
    fn device_identity(&self) -> &HardwareDeviceIdentity;

    /// Returns whether the session has been successfully closed.
    fn is_closed(&self) -> bool;

    /// Closes the provider-managed hardware session.
    ///
    /// Closing is a lifecycle operation, not proof that an
    /// already submitted physical operation has stopped.
    ///
    /// Implementations should support idempotent closure.
    ///
    /// The operation must observe cancellation and applicable
    /// deadlines where safe.
    ///
    /// Cleanup policy may require a provider to complete
    /// essential release work even after cancellation.
    ///
    /// # Errors
    ///
    /// Returns a structured hardware error when closure fails.
    fn close<'a>(
        &'a mut self,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, ()>;
}

// ============================================================================
// UNIVERSAL HARDWARE PROVIDER
// ============================================================================

/// Universal ZHAL hardware-provider integration contract.
///
/// Every hardware provider implements the canonical standard-library
/// Provider contract and this specialized hardware contract.
///
/// The standard-library Provider remains authoritative for:
///
/// - Provider identity.
/// - Provider kind.
/// - Provider descriptor.
/// - Provider implementation version.
/// - Provider API version.
/// - Declared capabilities.
/// - Current provider status.
/// - Evidence-backed capability snapshots.
///
/// This trait adds hardware-specific discovery and access.
///
/// # Object safety
///
/// The trait is object-safe.
///
/// Its asynchronous methods return boxed futures rather than
/// exposing generic associated future types.
///
/// Providers can therefore be stored behind trait objects by
/// the provider registry.
///
/// # Thread safety
///
/// The inherited Provider contract requires Send + Sync.
///
/// Hardware provider implementations must therefore support
/// the concurrency model of the canonical runtime.
///
/// Internal hardware sessions are separately represented
/// through HardwareSession.
///
/// # Provider identity
///
/// Every returned device and session must belong to the
/// identity returned by descriptor().
///
/// # Lifecycle
///
/// Providers requiring initialization or shutdown implement
/// the existing ProviderLifecycle trait separately.
///
/// This trait does not create another lifecycle authority.
///
/// # Domain-specific extension
///
/// Providers may additionally implement specialized interfaces.
///
/// Examples:
///
/// - Quantum hardware provider interfaces.
/// - HDL realization provider interfaces.
/// - Classical execution provider interfaces.
/// - Accelerator execution provider interfaces.
/// - Distributed hardware provider interfaces.
///
/// A new domain must not require modifications to this trait
/// merely to introduce a new operation family.
pub trait HardwareProvider: Provider {
    /// Discovers one page of provider-owned hardware devices.
    ///
    /// Implementations must:
    ///
    /// - Respect the supplied discovery cursor.
    /// - Respect the optional requested page size.
    /// - Preserve provider-scoped device identities.
    /// - Return valid device descriptors.
    /// - Preserve unknown availability.
    /// - Avoid fabricating capability evidence.
    /// - Avoid allocating or reserving resources.
    /// - Observe cancellation.
    /// - Observe applicable deadlines.
    /// - Return a continuation cursor when additional results
    ///   remain within the discovery operation.
    /// - Avoid returning the same continuation cursor.
    ///
    /// Providers should support discovery without requiring
    /// the complete hardware inventory to be held in memory.
    ///
    /// Discovery ordering is provider-defined.
    ///
    /// # Errors
    ///
    /// Returns structured errors for discovery failures,
    /// communication failures, cancellation and invalid data.
    fn discover_devices_page<'a>(
        &'a self,
        request: &'a DeviceDiscoveryRequest,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, DeviceDiscoveryPage>;

    /// Inspects an individual hardware device.
    ///
    /// The requested identity must belong to this provider.
    ///
    /// The returned descriptor must identify exactly the
    /// requested device.
    ///
    /// Inspection is observational.
    ///
    /// It does not reserve the device or establish a session.
    ///
    /// # Errors
    ///
    /// Returns an error if the device is unknown, inaccessible,
    /// unavailable, invalid or owned by another provider.
    fn inspect_device<'a>(
        &'a self,
        identity: &'a HardwareDeviceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, HardwareDeviceDescriptor>;

    /// Describes one provider-owned hardware interface.
    ///
    /// Interface identity is device-scoped.
    ///
    /// The returned descriptor must identify exactly the
    /// requested interface.
    ///
    /// Interface availability is observational.
    ///
    /// This method does not open or reserve the interface.
    ///
    /// # Errors
    ///
    /// Returns an error if the interface is unknown,
    /// inaccessible or belongs to another provider.
    fn describe_interface<'a>(
        &'a self,
        identity: &'a HardwareInterfaceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, HardwareInterfaceDescriptor>;

    /// Opens a provider-managed session for a hardware device.
    ///
    /// The provider must:
    ///
    /// - Verify that the device belongs to itself.
    /// - Revalidate current device availability.
    /// - Enforce applicable access-control requirements.
    /// - Establish a provider-managed session.
    /// - Return a session with the correct provider identity.
    /// - Associate the session with the requested device.
    /// - Preserve access failures.
    /// - Observe cancellation and applicable deadlines.
    ///
    /// Discovery or inspection does not grant authorization.
    ///
    /// Resource reservation remains subject to the runtime
    /// and provider-specific reservation contracts.
    ///
    /// # Errors
    ///
    /// Returns a structured error if access is denied,
    /// the device is unavailable, the provider fails or
    /// session creation cannot be completed.
    fn open_session<'a>(
        &'a self,
        device: &'a HardwareDeviceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, Box<dyn HardwareSession>>;

    // ------------------------------------------------------------------------
    // VALIDATED DISCOVERY
    // ------------------------------------------------------------------------

    /// Performs discovery and validates the provider's response.
    ///
    /// This default implementation provides common validation
    /// for all hardware providers.
    ///
    /// # Errors
    ///
    /// Returns an error for cancellation, timeout, invalid
    /// descriptors, foreign devices or duplicate identities.
    fn discover_devices<'a>(
        &'a self,
        request: &'a DeviceDiscoveryRequest,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, DeviceDiscoveryPage> {
        Box::pin(async move {
            check_active(context)?;

            let page = self
                .discover_devices_page(request, context)
                .await?;

            check_active(context)?;

            page.validate_for(
                self.descriptor().id(),
                request,
            )?;

            Ok(page)
        })
    }

    // ------------------------------------------------------------------------
    // VALIDATED DEVICE INSPECTION
    // ------------------------------------------------------------------------

    /// Inspects and validates a device descriptor.
    ///
    /// The returned descriptor must have exactly the requested
    /// provider-scoped identity.
    ///
    /// # Errors
    ///
    /// Returns a contract violation if the provider returns
    /// another device or invalid descriptor data.
    fn inspect_device_validated<'a>(
        &'a self,
        identity: &'a HardwareDeviceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, HardwareDeviceDescriptor> {
        Box::pin(async move {
            check_active(context)?;

            if identity.provider_id() != self.descriptor().id() {
                return Err(
                    ZhalError::new(
                        ZhalErrorKind::InvalidInput,
                        "requested device belongs to another provider",
                    )
                    .in_component(COMPONENT),
                );
            }

            let descriptor = self
                .inspect_device(identity, context)
                .await?;

            check_active(context)?;

            descriptor.validate().map_err(|error| {
                ZhalError::new(
                    ZhalErrorKind::InvalidData,
                    format!(
                        "provider returned an invalid device descriptor: {error}"
                    ),
                )
                .in_component(COMPONENT)
            })?;

            if descriptor.identity() != identity {
                return Err(
                    contract_violation(
                        "provider returned a descriptor for a different device",
                    )
                );
            }

            Ok(descriptor)
        })
    }

    // ------------------------------------------------------------------------
    // VALIDATED INTERFACE DESCRIPTION
    // ------------------------------------------------------------------------

    /// Describes and validates a hardware interface.
    ///
    /// The interface must belong to this provider.
    ///
    /// # Errors
    ///
    /// Returns a contract violation when the provider returns
    /// a different interface identity.
    fn describe_interface_validated<'a>(
        &'a self,
        identity: &'a HardwareInterfaceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, HardwareInterfaceDescriptor> {
        Box::pin(async move {
            check_active(context)?;

            if identity.device().provider_id()
                != self.descriptor().id()
            {
                return Err(
                    ZhalError::new(
                        ZhalErrorKind::InvalidInput,
                        "requested hardware interface belongs to another provider",
                    )
                    .in_component(COMPONENT),
                );
            }

            let descriptor = self
                .describe_interface(identity, context)
                .await?;

            check_active(context)?;

            if descriptor.identity() != identity {
                return Err(
                    contract_violation(
                        "provider returned a descriptor for a different hardware interface",
                    )
                );
            }

            Ok(descriptor)
        })
    }

    // ------------------------------------------------------------------------
    // VALIDATED SESSION CREATION
    // ------------------------------------------------------------------------

    /// Opens a hardware session and validates its identity.
    ///
    /// The provider must perform actual access authorization
    /// and revalidate the device before returning a session.
    ///
    /// # Errors
    ///
    /// Returns a contract violation if the session belongs
    /// to another provider or device.
    fn open_session_validated<'a>(
        &'a self,
        device: &'a HardwareDeviceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, Box<dyn HardwareSession>> {
        Box::pin(async move {
            check_active(context)?;

            if device.provider_id() != self.descriptor().id() {
                return Err(
                    ZhalError::new(
                        ZhalErrorKind::InvalidInput,
                        "requested hardware device belongs to another provider",
                    )
                    .in_component(COMPONENT),
                );
            }

            let session = self
                .open_session(device, context)
                .await?;

            check_active(context)?;

            if session.identity().provider_id()
                != self.descriptor().id()
            {
                return Err(
                    contract_violation(
                        "provider returned a session owned by another provider",
                    )
                );
            }

            if session.device_identity() != device {
                return Err(
                    contract_violation(
                        "provider returned a session for a different hardware device",
                    )
                );
            }

            if session.identity().provider_id()
                != session.device_identity().provider_id()
            {
                return Err(
                    contract_violation(
                        "hardware session identity and device identity have different providers",
                    )
                );
            }

            Ok(session)
        })
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::io::devices::{
        DeviceId,
        DeviceKindId,
    };

    use crate::stdlib::provider::ProviderId;

    use crate::zhal::identity::{
        HardwareDeviceIdentity,
    };

    fn provider_id() -> ProviderId {
        ProviderId::new("test.hardware.provider")
            .expect("valid provider identity")
    }

    fn device_identity(
        provider: ProviderId,
        device: &str,
    ) -> HardwareDeviceIdentity {
        HardwareDeviceIdentity::new(
            provider,
            DeviceId::new(device)
                .expect("valid device identity"),
        )
    }

    fn device_descriptor(
        provider: ProviderId,
        device: &str,
    ) -> HardwareDeviceDescriptor {
        HardwareDeviceDescriptor::new(
            device_identity(provider, device),
            DeviceKindId::new("test.hardware.device")
                .expect("valid device kind"),
            "Test hardware device",
        )
        .expect("valid device descriptor")
    }

    #[test]
    fn discovery_cursor_preserves_opaque_provider_data() {
        let cursor = DeviceDiscoveryCursor::new(
            "provider/partition/continuation-1",
        )
        .expect("valid cursor");

        assert_eq!(
            cursor.as_str(),
            "provider/partition/continuation-1",
        );
    }

    #[test]
    fn discovery_cursor_rejects_empty_values() {
        assert!(
            DeviceDiscoveryCursor::new("").is_err()
        );
    }

    #[test]
    fn discovery_cursor_rejects_surrounding_whitespace() {
        assert!(
            DeviceDiscoveryCursor::new(" cursor ").is_err()
        );
    }

    #[test]
    fn discovery_cursor_rejects_control_characters() {
        assert!(
            DeviceDiscoveryCursor::new("cursor\nnext").is_err()
        );
    }

    #[test]
    fn initial_discovery_request_has_no_assumptions() {
        let request = DeviceDiscoveryRequest::initial();

        assert!(request.cursor().is_none());
        assert!(request.page_size().is_none());
        assert!(!request.is_continuation());
    }

    #[test]
    fn discovery_request_accepts_caller_defined_page_size() {
        let request = DeviceDiscoveryRequest::new(
            None,
            NonZeroUsize::new(4096),
        );

        assert_eq!(
            request.page_size().map(NonZeroUsize::get),
            Some(4096),
        );
    }

    #[test]
    fn discovery_page_can_represent_empty_inventory() {
        let page = DeviceDiscoveryPage::new(
            Vec::new(),
            None,
            Some("inventory-revision-1".to_owned()),
        )
        .expect("valid discovery page");

        assert!(page.is_empty());
        assert!(page.is_complete());
        assert_eq!(page.len(), 0);
    }

    #[test]
    fn discovery_page_supports_continuation() {
        let cursor = DeviceDiscoveryCursor::new(
            "provider/next",
        )
        .expect("valid cursor");

        let page = DeviceDiscoveryPage::new(
            Vec::new(),
            Some(cursor),
            None,
        )
        .expect("valid discovery page");

        assert!(!page.is_complete());
        assert!(page.next_cursor().is_some());
    }

    #[test]
    fn discovery_page_rejects_invalid_revision() {
        assert!(
            DeviceDiscoveryPage::new(
                Vec::new(),
                None,
                Some("invalid\nrevision".to_owned()),
            )
            .is_err()
        );
    }

    #[test]
    fn discovery_page_accepts_future_device_categories() {
        let descriptor = HardwareDeviceDescriptor::new(
            device_identity(
                provider_id(),
                "future-device-1",
            ),
            DeviceKindId::new(
                "future.photonic.molecular.quantum.device",
            )
            .expect("future hardware category is valid"),
            "Future computational device",
        )
        .expect("valid device descriptor");

        assert_eq!(
            descriptor.category().as_str(),
            "future.photonic.molecular.quantum.device",
        );
    }

    #[test]
    fn discovery_page_validates_provider_ownership() {
        let expected = provider_id();

        let descriptor = device_descriptor(
            expected.clone(),
            "device-1",
        );

        let page = DeviceDiscoveryPage::new(
            vec![descriptor],
            None,
            None,
        )
        .expect("valid page");

        assert!(
            page.validate_for(
                &expected,
                &DeviceDiscoveryRequest::initial(),
            )
            .is_ok()
        );
    }

    #[test]
    fn discovery_page_rejects_foreign_provider_devices() {
        let expected = provider_id();

        let foreign = ProviderId::new(
            "another.hardware.provider",
        )
        .expect("valid provider identity");

        let descriptor = device_descriptor(
            foreign,
            "device-1",
        );

        let page = DeviceDiscoveryPage::new(
            vec![descriptor],
            None,
            None,
        )
        .expect("structurally valid page");

        assert!(
            page.validate_for(
                &expected,
                &DeviceDiscoveryRequest::initial(),
            )
            .is_err()
        );
    }

    #[test]
    fn discovery_page_rejects_duplicate_devices() {
        let provider = provider_id();

        let first = device_descriptor(
            provider.clone(),
            "device-1",
        );

        let second = device_descriptor(
            provider.clone(),
            "device-1",
        );

        let page = DeviceDiscoveryPage::new(
            vec![first, second],
            None,
            None,
        )
        .expect("page construction succeeds");

        assert!(
            page.validate_for(
                &provider,
                &DeviceDiscoveryRequest::initial(),
            )
            .is_err()
        );
    }

    #[test]
    fn discovery_page_rejects_non_advancing_cursor() {
        let provider = provider_id();

        let cursor = DeviceDiscoveryCursor::new(
            "continuation-1",
        )
        .expect("valid cursor");

        let request = DeviceDiscoveryRequest::new(
            Some(cursor.clone()),
            None,
        );

        let page = DeviceDiscoveryPage::new(
            Vec::new(),
            Some(cursor),
            None,
        )
        .expect("page construction succeeds");

        assert!(
            page.validate_for(
                &provider,
                &request,
            )
            .is_err()
        );
    }

    #[test]
    fn discovery_supports_large_provider_inventories() {
        let provider = provider_id();

        let devices = (0usize..1024)
            .map(|index| {
                device_descriptor(
                    provider.clone(),
                    &format!("device-{index}"),
                )
            })
            .collect();

        let page = DeviceDiscoveryPage::new(
            devices,
            None,
            None,
        )
        .expect("valid large discovery page");

        assert_eq!(page.len(), 1024);

        assert!(
            page.validate_for(
                &provider,
                &DeviceDiscoveryRequest::initial(),
            )
            .is_ok()
        );
    }
}
