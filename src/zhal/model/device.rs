
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! Hardware Device Model
//!
//! Path: `src/zhal/model/device.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the canonical, provider-independent description of a
//! hardware device exposed through ZHAL.
//!
//! A device may represent physical, virtual, remote, simulated,
//! embedded, reconfigurable or future computational hardware.
//!
//! Supported hardware domains include:
//!
//! - Classical processors and memory systems.
//! - Quantum processors and control systems.
//! - FPGA and ASIC devices.
//! - GPUs, TPUs, NPUs and DSPs.
//! - Embedded processors and microcontrollers.
//! - Sensors and actuators.
//! - Distributed and remotely accessible hardware.
//! - Neuromorphic and photonic systems.
//! - Analog and in-memory computing systems.
//! - Future computational technologies.
//!
//! This model describes hardware. It does not execute computation.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - HardwareDeviceDescriptor.
//! - DeviceAvailability.
//! - DeviceAvailabilityState.
//! - HardwareDeviceProvenance.
//! - Device descriptor validation.
//! - Device descriptor serialization.
//! - Device descriptor deserialization.
//! - Device-to-resource identity references.
//! - Device-to-interface identity references.
//! - Capability and resource snapshot references.
//!
//! This module does not own:
//!
//! - Provider identity.
//! - Generic device identity.
//! - Device discovery.
//! - Provider registration.
//! - Provider lifecycle.
//! - Capability semantics.
//! - Capability evidence evaluation.
//! - Resource allocation.
//! - Resource reservation.
//! - Resource accounting.
//! - Hardware interface implementation.
//! - Physical device communication.
//! - Device sessions.
//! - Hardware scheduling.
//! - Quantum routing.
//! - HDL realization.
//! - Distributed execution.
//! - Hybrid execution.
//! - Compiler or IR representations.
//! - Runtime orchestration.
//!
//! These responsibilities remain with their existing authorities.
//!
//! ## Canonical repository authorities
//!
//! Provider identity:
//!     crate::stdlib::provider::ProviderId
//!
//! Device identity:
//!     crate::stdlib::io::devices::DeviceId
//!
//! Device kind:
//!     crate::stdlib::io::devices::DeviceKindId
//!
//! Composite hardware identity:
//!     crate::zhal::identity::HardwareDeviceIdentity
//!
//! Resource identity:
//!     crate::zhal::identity::HardwareResourceId
//!
//! Interface identity:
//!     crate::zhal::identity::HardwareInterfaceId
//!
//! Snapshot identity:
//!     crate::zhal::identity::HardwareSnapshotId
//!
//! Hardware metadata:
//!     crate::zhal::model::metadata
//!
//! Capability descriptions and snapshots:
//!     crate::stdlib::capability
//!
//! Generic device I/O:
//!     crate::stdlib::io::devices
//!
//! Quantum hardware:
//!     crate::quantum::hardware
//!
//! HDL realization:
//!     crate::hdl::realization
//!
//! Runtime allocation and orchestration:
//!     crate::zur
//!
//! ## Identity
//!
//! A device identity is provider-scoped.
//!
//! The composite identity consists of the canonical ProviderId and
//! canonical DeviceId through HardwareDeviceIdentity.
//!
//! Two providers may expose the same local DeviceId without
//! identifying the same device.
//!
//! ## Hardware categories
//!
//! Hardware categories are open-world DeviceKindId values.
//!
//! No closed enumeration of vendors, architectures, device types,
//! technologies or computational domains is maintained here.
//!
//! ## Availability
//!
//! Availability is an observation, not a reservation.
//!
//! Unknown is not equivalent to available.
//!
//! A previously available device may become unavailable before access.
//!
//! Providers must revalidate availability when opening a device.
//!
//! ## Capability snapshots
//!
//! A capability snapshot reference identifies an external,
//! immutable capability observation.
//!
//! It does not prove that a requirement is satisfied.
//!
//! Capability semantics and evidence remain owned by
//! crate::stdlib::capability and the relevant domain authorities.
//!
//! ## Resource references
//!
//! This module references hardware resource identities and resource
//! observation snapshots.
//!
//! It does not describe or allocate physical resources.
//!
//! Resource descriptions and observations belong to the resource model.
//!
//! Reservation and allocation belong to the runtime.
//!
//! ## Interfaces
//!
//! This module references hardware interface identities.
//!
//! Interface contracts and implementation details belong to
//! the interface and provider subsystems.
//!
//! Generic byte-oriented transport remains owned by
//! crate::stdlib::io::devices.
//!
//! Domain-specific interfaces remain owned by their respective domains.
//!
//! ## Metadata
//!
//! Hardware metadata is descriptive information.
//!
//! It must not be interpreted as proof of capability, authenticity,
//! authorization, availability or successful execution.
//!
//! Secret rejection and metadata validation are delegated to
//! crate::zhal::model::metadata.
//!
//! ## Provenance
//!
//! Device provenance describes the origin of a hardware observation.
//!
//! It does not independently authenticate the provider or observation.
//!
//! Provider trust and verification remain security responsibilities.
//!
//! ## Serialization
//!
//! Device descriptors serialize as structured objects.
//!
//! Deserialization validates all fields before constructing a descriptor.
//!
//! Unknown hardware categories remain representable.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Device identifier length.
//! - Hardware category length.
//! - Number of devices.
//! - Number of interfaces.
//! - Number of resource references.
//! - Metadata entry count.
//! - Provider count.
//! - Hardware categories.
//! - Computational domains.
//! - Hardware technologies.
//!
//! Collections grow according to available memory and platform limits.
//!
//! Operational budgets must be explicit configuration or runtime policy.
//!
//! Literal infinite physical storage or computation is not promised.
//!
//! ## Grammar boundary
//!
//! The hardware grammar describes source-level hardware intent.
//!
//! The resource grammar describes source-level resource intent.
//!
//! This module describes provider-reported hardware facts.
//!
//! It does not parse grammar, create AST nodes or interpret source-level
//! requirements.
//!
//! ## Integration
//!
//! Provider discovery produces validated device descriptors.
//!
//! The device inventory stores observed descriptors.
//!
//! Capability models provide capability snapshots.
//!
//! Resource models provide resource observations.
//!
//! Interface models provide hardware interface descriptions.
//!
//! Hardware snapshots preserve immutable observations.
//!
//! Quantum and HDL adapters translate their existing specialized
//! descriptions into this generic device representation.
//!
//! ZUR consumes these descriptions when making runtime decisions.
//!
//! ## Safety
//!
//! Unsafe Rust is forbidden.
//!
//! This module performs no physical hardware access.
//!
//! It does not create global mutable state.
//!
//! It does not silently allocate or reserve resources.

use core::fmt;

use std::collections::BTreeSet;

use serde::de::Error as SerdeDeError;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::stdlib::io::devices::DeviceKindId;

use crate::zhal::identity::{
    HardwareDeviceIdentity,
    HardwareInterfaceId,
    HardwareResourceId,
    HardwareSnapshotId,
};

use crate::zhal::model::metadata::HardwareMetadata;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable module identity.
pub const COMPONENT: &str = "zhal.model.device";

/// Device descriptor schema version.
///
/// This identifies the serialized device descriptor contract.
/// It is independent of the Cargo package version, language version
/// and ZHAL API version.
pub const DEVICE_SCHEMA_VERSION: &str = "1";

// ============================================================================
// DEVICE MODEL ERRORS
// ============================================================================

/// Classification of device-model validation failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DeviceModelErrorKind {
    /// A required descriptive field is empty.
    EmptyField,

    /// A field contains surrounding whitespace.
    SurroundingWhitespace,

    /// A field contains control characters.
    ControlCharacter,

    /// A field contains invalid whitespace.
    InvalidWhitespace,

    /// A device availability state is invalid.
    InvalidAvailability,

    /// A provenance description is invalid.
    InvalidProvenance,

    /// A hardware metadata collection is invalid.
    InvalidMetadata,

    /// A device descriptor is internally inconsistent.
    InconsistentDescriptor,
}

impl fmt::Display for DeviceModelErrorKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let description = match self {
            Self::EmptyField => "required field is empty",
            Self::SurroundingWhitespace => {
                "field contains surrounding whitespace"
            }
            Self::ControlCharacter => {
                "field contains control characters"
            }
            Self::InvalidWhitespace => {
                "field contains invalid whitespace"
            }
            Self::InvalidAvailability => {
                "device availability state is invalid"
            }
            Self::InvalidProvenance => {
                "device provenance is invalid"
            }
            Self::InvalidMetadata => {
                "device metadata is invalid"
            }
            Self::InconsistentDescriptor => {
                "device descriptor is internally inconsistent"
            }
        };

        formatter.write_str(description)
    }
}

/// Structured device-model validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceModelError {
    kind: DeviceModelErrorKind,
    field: String,
    message: String,
}

impl DeviceModelError {
    fn new(
        kind: DeviceModelErrorKind,
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            field: field.into(),
            message: message.into(),
        }
    }

    /// Returns the error classification.
    #[must_use]
    pub const fn kind(&self) -> DeviceModelErrorKind {
        self.kind
    }

    /// Returns the affected field.
    #[must_use]
    pub fn field(&self) -> &str {
        &self.field
    }

    /// Returns the descriptive error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for DeviceModelError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}: {}: {}",
            COMPONENT,
            self.field,
            self.message
        )
    }
}

impl std::error::Error for DeviceModelError {}

/// Result type for device-model operations.
pub type DeviceModelResult<T> = Result<T, DeviceModelError>;

// ============================================================================
// COMMON VALIDATION
// ============================================================================

fn validate_reference(
    value: &str,
    field: &str,
) -> DeviceModelResult<()> {
    if value.is_empty() {
        return Err(DeviceModelError::new(
            DeviceModelErrorKind::EmptyField,
            field,
            "value must not be empty",
        ));
    }

    if value.trim() != value {
        return Err(DeviceModelError::new(
            DeviceModelErrorKind::SurroundingWhitespace,
            field,
            "value must not contain surrounding whitespace",
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(DeviceModelError::new(
            DeviceModelErrorKind::ControlCharacter,
            field,
            "value must not contain control characters",
        ));
    }

    Ok(())
}

fn validate_display_name(
    value: &str,
) -> DeviceModelResult<()> {
    if value.trim().is_empty() {
        return Err(DeviceModelError::new(
            DeviceModelErrorKind::EmptyField,
            "display_name",
            "device display name must not be empty",
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(DeviceModelError::new(
            DeviceModelErrorKind::ControlCharacter,
            "display_name",
            "device display name must not contain control characters",
        ));
    }

    Ok(())
}

// ============================================================================
// DEVICE AVAILABILITY
// ============================================================================

/// Standard availability-state identifiers.
///
/// These constants provide common values without making the state
/// representation a closed enumeration.
pub mod availability_state {
    /// Availability has not been established.
    pub const UNKNOWN: &str = "unknown";

    /// The provider reports that the device is available.
    pub const AVAILABLE: &str = "available";

    /// The provider reports that the device is unavailable.
    pub const UNAVAILABLE: &str = "unavailable";

    /// The provider reports that the device is operational but degraded.
    pub const DEGRADED: &str = "degraded";

    /// Availability depends on additional conditions.
    pub const CONDITIONAL: &str = "conditional";

    /// The device is undergoing initialization.
    pub const INITIALIZING: &str = "initializing";

    /// The device is undergoing shutdown.
    pub const STOPPING: &str = "stopping";

    /// The device has entered a failed state.
    pub const FAILED: &str = "failed";
}

/// Open-world device availability observation.
///
/// The state is an extensible string rather than a closed enum.
/// This permits providers and future hardware technologies to
/// introduce additional availability classifications.
///
/// Availability is observational. It is not a reservation,
/// authorization decision or guarantee of future access.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DeviceAvailability {
    state: String,
    reason: Option<String>,
}

impl DeviceAvailability {
    /// Constructs an availability observation.
    ///
    /// Standard state identifiers are available through
    /// [`availability_state`].
    ///
    /// Unknown provider-defined states are preserved.
    ///
    /// # Errors
    ///
    /// Returns an error if the state or optional reason is invalid.
    pub fn new(
        state: impl Into<String>,
        reason: Option<String>,
    ) -> DeviceModelResult<Self> {
        let state = state.into();

        validate_reference(&state, "availability.state")
            .map_err(|_| {
                DeviceModelError::new(
                    DeviceModelErrorKind::InvalidAvailability,
                    "availability.state",
                    "availability state must be non-empty and contain no whitespace or control characters",
                )
            })?;

        if let Some(reason) = &reason {
            if reason.trim().is_empty() {
                return Err(DeviceModelError::new(
                    DeviceModelErrorKind::InvalidAvailability,
                    "availability.reason",
                    "availability reason must not be empty",
                ));
            }

            if reason.chars().any(char::is_control) {
                return Err(DeviceModelError::new(
                    DeviceModelErrorKind::ControlCharacter,
                    "availability.reason",
                    "availability reason must not contain control characters",
                ));
            }
        }

        Ok(Self {
            state,
            reason,
        })
    }

    /// Constructs an unknown availability observation.
    #[must_use]
    pub fn unknown() -> Self {
        Self {
            state: availability_state::UNKNOWN.to_owned(),
            reason: None,
        }
    }

    /// Constructs an available observation.
    #[must_use]
    pub fn available() -> Self {
        Self {
            state: availability_state::AVAILABLE.to_owned(),
            reason: None,
        }
    }

    /// Constructs an unavailable observation.
    ///
    /// The reason is optional.
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied reason is invalid.
    pub fn unavailable(
        reason: Option<String>,
    ) -> DeviceModelResult<Self> {
        Self::new(
            availability_state::UNAVAILABLE,
            reason,
        )
    }

    /// Constructs a degraded observation.
    ///
    /// The reason is optional.
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied reason is invalid.
    pub fn degraded(
        reason: Option<String>,
    ) -> DeviceModelResult<Self> {
        Self::new(
            availability_state::DEGRADED,
            reason,
        )
    }

    /// Returns the exact availability state.
    #[must_use]
    pub fn state(&self) -> &str {
        &self.state
    }

    /// Returns the optional availability explanation.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Returns whether the state explicitly reports availability.
    ///
    /// This is only an observation. It does not authorize access,
    /// reserve the device or guarantee that it remains available.
    #[must_use]
    pub fn is_available(&self) -> bool {
        self.state == availability_state::AVAILABLE
    }

    /// Returns whether the state explicitly reports unavailability.
    #[must_use]
    pub fn is_unavailable(&self) -> bool {
        self.state == availability_state::UNAVAILABLE
    }

    /// Returns whether the state is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.state == availability_state::UNKNOWN
    }

    /// Validates the complete availability observation.
    ///
    /// # Errors
    ///
    /// Returns a structured error if a field is invalid.
    pub fn validate(&self) -> DeviceModelResult<()> {
        validate_reference(
            &self.state,
            "availability.state",
        )?;

        if let Some(reason) = &self.reason {
            if reason.trim().is_empty() {
                return Err(DeviceModelError::new(
                    DeviceModelErrorKind::InvalidAvailability,
                    "availability.reason",
                    "availability reason must not be empty",
                ));
            }

            if reason.chars().any(char::is_control) {
                return Err(DeviceModelError::new(
                    DeviceModelErrorKind::ControlCharacter,
                    "availability.reason",
                    "availability reason must not contain control characters",
                ));
            }
        }

        Ok(())
    }
}

impl Default for DeviceAvailability {
    fn default() -> Self {
        Self::unknown()
    }
}

impl<'de> Deserialize<'de> for DeviceAvailability {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            state: String,
            reason: Option<String>,
        }

        let wire = Wire::deserialize(deserializer)?;

        Self::new(wire.state, wire.reason)
            .map_err(D::Error::custom)
    }
}

// ============================================================================
// DEVICE PROVENANCE
// ============================================================================

/// Describes the origin of a hardware-device observation.
///
/// Provenance is descriptive evidence about where a device description
/// came from. It does not authenticate the source or prove that the
/// observation is accurate.
///
/// Timestamps are retained as provider-supplied strings so that this
/// model does not impose a particular clock or timestamp authority.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HardwareDeviceProvenance {
    source: String,
    observed_at: Option<String>,
    evidence_reference: Option<String>,
}

impl HardwareDeviceProvenance {
    /// Constructs a device-provenance description.
    ///
    /// `source` identifies the observation source.
    ///
    /// `observed_at` is an optional provider-supplied timestamp.
    ///
    /// `evidence_reference` is an optional reference to external
    /// evidence or a provider observation record.
    ///
    /// # Errors
    ///
    /// Returns an error when a supplied field is invalid.
    pub fn new(
        source: impl Into<String>,
        observed_at: Option<String>,
        evidence_reference: Option<String>,
    ) -> DeviceModelResult<Self> {
        let source = source.into();

        validate_reference(
            &source,
            "provenance.source",
        )
        .map_err(|_| {
            DeviceModelError::new(
                DeviceModelErrorKind::InvalidProvenance,
                "provenance.source",
                "provenance source must be non-empty and contain no whitespace or control characters",
            )
        })?;

        if let Some(value) = &observed_at {
            validate_reference(
                value,
                "provenance.observed_at",
            )
            .map_err(|_| {
                DeviceModelError::new(
                    DeviceModelErrorKind::InvalidProvenance,
                    "provenance.observed_at",
                    "observation timestamp must be non-empty and contain no control characters",
                )
            })?;
        }

        if let Some(value) = &evidence_reference {
            validate_reference(
                value,
                "provenance.evidence_reference",
            )
            .map_err(|_| {
                DeviceModelError::new(
                    DeviceModelErrorKind::InvalidProvenance,
                    "provenance.evidence_reference",
                    "evidence reference must be non-empty and contain no whitespace or control characters",
                )
            })?;
        }

        Ok(Self {
            source,
            observed_at,
            evidence_reference,
        })
    }

    /// Returns the observation source.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Returns the provider-supplied observation timestamp.
    #[must_use]
    pub fn observed_at(&self) -> Option<&str> {
        self.observed_at.as_deref()
    }

    /// Returns the external evidence reference.
    #[must_use]
    pub fn evidence_reference(&self) -> Option<&str> {
        self.evidence_reference.as_deref()
    }

    /// Validates the complete provenance record.
    ///
    /// # Errors
    ///
    /// Returns a structured error when a field is invalid.
    pub fn validate(&self) -> DeviceModelResult<()> {
        Self::new(
            self.source.clone(),
            self.observed_at.clone(),
            self.evidence_reference.clone(),
        )
        .map(|_| ())
    }
}

impl<'de> Deserialize<'de> for HardwareDeviceProvenance {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            source: String,
            observed_at: Option<String>,
            evidence_reference: Option<String>,
        }

        let wire = Wire::deserialize(deserializer)?;

        Self::new(
            wire.source,
            wire.observed_at,
            wire.evidence_reference,
        )
        .map_err(D::Error::custom)
    }
}

// ============================================================================
// CANONICAL HARDWARE DEVICE DESCRIPTOR
// ============================================================================

/// Complete, provider-independent description of a hardware device.
///
/// This descriptor is the common physical hardware representation
/// consumed by ZHAL inventory, discovery, provider and runtime adapters.
///
/// It combines existing canonical identities with descriptive
/// observations and references to separately owned models.
///
/// It does not own hardware resources, capability snapshots,
/// interfaces or provider implementations.
///
/// # Identity
///
/// The identity is a provider-scoped HardwareDeviceIdentity.
///
/// # Category
///
/// The category is the existing open-world DeviceKindId.
///
/// # Observations
///
/// Availability, capability snapshots, resource snapshots,
/// metadata and provenance describe observations at a particular
/// point in the provider's reporting lifecycle.
///
/// # Resource and interface references
///
/// Resource and interface identities refer to objects owned by
/// their respective ZHAL model and provider authorities.
///
/// They do not imply ownership, allocation or reservation.
///
/// # Immutability
///
/// The descriptor exposes read-only accessors.
/// Updates are performed by constructing a new descriptor and
/// replacing the corresponding inventory observation.
///
/// This prevents callers from mutating a descriptor already
/// shared as an observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HardwareDeviceDescriptor {
    identity: HardwareDeviceIdentity,
    category: DeviceKindId,
    display_name: String,
    availability: DeviceAvailability,
    capability_snapshot: Option<HardwareSnapshotId>,
    resource_snapshot: Option<HardwareSnapshotId>,
    resources: BTreeSet<HardwareResourceId>,
    interfaces: BTreeSet<HardwareInterfaceId>,
    metadata: HardwareMetadata,
    provenance: Option<HardwareDeviceProvenance>,
}

impl HardwareDeviceDescriptor {
    /// Constructs a hardware-device descriptor.
    ///
    /// New descriptors default to unknown availability.
    ///
    /// No capability or resource observation is fabricated.
    ///
    /// # Errors
    ///
    /// Returns an error when the display name or category is invalid.
    pub fn new(
        identity: HardwareDeviceIdentity,
        category: DeviceKindId,
        display_name: impl Into<String>,
    ) -> DeviceModelResult<Self> {
        let display_name = display_name.into();

        validate_display_name(&display_name)?;

        Ok(Self {
            identity,
            category,
            display_name,
            availability: DeviceAvailability::unknown(),
            capability_snapshot: None,
            resource_snapshot: None,
            resources: BTreeSet::new(),
            interfaces: BTreeSet::new(),
            metadata: HardwareMetadata::new(),
            provenance: None,
        })
    }

    /// Returns the device schema version.
    #[must_use]
    pub const fn schema_version(&self) -> &'static str {
        DEVICE_SCHEMA_VERSION
    }

    /// Returns the provider-scoped hardware identity.
    #[must_use]
    pub fn identity(&self) -> &HardwareDeviceIdentity {
        &self.identity
    }

    /// Returns the canonical provider identity.
    #[must_use]
    pub fn provider_id(
        &self,
    ) -> &crate::stdlib::provider::ProviderId {
        self.identity.provider_id()
    }

    /// Returns the canonical provider-scoped device identity.
    #[must_use]
    pub fn device_id(
        &self,
    ) -> &crate::stdlib::io::devices::DeviceId {
        self.identity.device_id()
    }

    /// Returns the open-world device category.
    #[must_use]
    pub fn category(&self) -> &DeviceKindId {
        &self.category
    }

    /// Returns the human-readable device name.
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Returns the availability observation.
    #[must_use]
    pub fn availability(&self) -> &DeviceAvailability {
        &self.availability
    }

    /// Returns the capability snapshot identity, if one was supplied.
    ///
    /// The referenced snapshot is owned by the capability subsystem.
    #[must_use]
    pub fn capability_snapshot(
        &self,
    ) -> Option<&HardwareSnapshotId> {
        self.capability_snapshot.as_ref()
    }

    /// Returns the resource observation snapshot identity, if supplied.
    ///
    /// The referenced snapshot is owned by the resource subsystem.
    #[must_use]
    pub fn resource_snapshot(
        &self,
    ) -> Option<&HardwareSnapshotId> {
        self.resource_snapshot.as_ref()
    }

    /// Returns referenced hardware resources in deterministic order.
    ///
    /// These are identities, not allocations or reservations.
    #[must_use]
    pub fn resources(
        &self,
    ) -> &BTreeSet<HardwareResourceId> {
        &self.resources
    }

    /// Returns referenced hardware interfaces in deterministic order.
    #[must_use]
    pub fn interfaces(
        &self,
    ) -> &BTreeSet<HardwareInterfaceId> {
        &self.interfaces
    }

    /// Returns the descriptive hardware metadata.
    #[must_use]
    pub fn metadata(&self) -> &HardwareMetadata {
        &self.metadata
    }

    /// Returns the optional observation provenance.
    #[must_use]
    pub fn provenance(
        &self,
    ) -> Option<&HardwareDeviceProvenance> {
        self.provenance.as_ref()
    }

    /// Replaces the availability observation.
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied observation is invalid.
    pub fn with_availability(
        mut self,
        availability: DeviceAvailability,
    ) -> DeviceModelResult<Self> {
        availability.validate()?;

        self.availability = availability;

        Ok(self)
    }

    /// Associates an external capability snapshot.
    ///
    /// This does not evaluate capability requirements.
    #[must_use]
    pub fn with_capability_snapshot(
        mut self,
        snapshot: HardwareSnapshotId,
    ) -> Self {
        self.capability_snapshot = Some(snapshot);
        self
    }

    /// Associates an external resource observation snapshot.
    ///
    /// This does not allocate or reserve resources.
    #[must_use]
    pub fn with_resource_snapshot(
        mut self,
        snapshot: HardwareSnapshotId,
    ) -> Self {
        self.resource_snapshot = Some(snapshot);
        self
    }

    /// Adds a referenced hardware resource identity.
    ///
    /// Duplicate identities are naturally ignored.
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied descriptor is invalid.
    pub fn with_resource(
        mut self,
        resource: HardwareResourceId,
    ) -> Self {
        self.resources.insert(resource);
        self
    }

    /// Adds multiple referenced hardware resource identities.
    ///
    /// The collection is deterministic and deduplicated.
    #[must_use]
    pub fn with_resources(
        mut self,
        resources: impl IntoIterator<Item = HardwareResourceId>,
    ) -> Self {
        self.resources.extend(resources);
        self
    }

    /// Adds a referenced hardware interface identity.
    ///
    /// Duplicate identities are naturally ignored.
    #[must_use]
    pub fn with_interface(
        mut self,
        interface: HardwareInterfaceId,
    ) -> Self {
        self.interfaces.insert(interface);
        self
    }

    /// Adds multiple hardware interface identities.
    ///
    /// The collection is deterministic and deduplicated.
    #[must_use]
    pub fn with_interfaces(
        mut self,
        interfaces: impl IntoIterator<Item = HardwareInterfaceId>,
    ) -> Self {
        self.interfaces.extend(interfaces);
        self
    }

    /// Associates validated descriptive hardware metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if metadata validation fails.
    pub fn with_metadata(
        mut self,
        metadata: HardwareMetadata,
    ) -> DeviceModelResult<Self> {
        metadata
            .validate()
            .map_err(|error| {
                DeviceModelError::new(
                    DeviceModelErrorKind::InvalidMetadata,
                    error.field(),
                    error.message(),
                )
            })?;

        self.metadata = metadata;

        Ok(self)
    }

    /// Associates hardware observation provenance.
    ///
    /// # Errors
    ///
    /// Returns an error if provenance is invalid.
    pub fn with_provenance(
        mut self,
        provenance: HardwareDeviceProvenance,
    ) -> DeviceModelResult<Self> {
        provenance.validate()?;

        self.provenance = Some(provenance);

        Ok(self)
    }

    /// Validates the complete device descriptor.
    ///
    /// This method should be called at provider, inventory and
    /// serialization boundaries.
    ///
    /// # Errors
    ///
    /// Returns the first encountered validation error.
    pub fn validate(&self) -> DeviceModelResult<()> {
        validate_display_name(&self.display_name)?;

        self.availability.validate()?;

        self.metadata
            .validate()
            .map_err(|error| {
                DeviceModelError::new(
                    DeviceModelErrorKind::InvalidMetadata,
                    error.field(),
                    error.message(),
                )
            })?;

        if let Some(provenance) = &self.provenance {
            provenance.validate()?;
        }

        Ok(())
    }

    /// Returns whether the descriptor explicitly reports availability.
    ///
    /// This is an observation only.
    #[must_use]
    pub fn is_available(&self) -> bool {
        self.availability.is_available()
    }
}

// ============================================================================
// SERIALIZATION
// ============================================================================

impl Serialize for HardwareDeviceDescriptor {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct(
            "HardwareDeviceDescriptor",
            10,
        )?;

        state.serialize_field(
            "schema_version",
            DEVICE_SCHEMA_VERSION,
        )?;

        state.serialize_field(
            "identity",
            &self.identity,
        )?;

        state.serialize_field(
            "category",
            self.category.as_str(),
        )?;

        state.serialize_field(
            "display_name",
            &self.display_name,
        )?;

        state.serialize_field(
            "availability",
            &self.availability,
        )?;

        state.serialize_field(
            "capability_snapshot",
            &self.capability_snapshot,
        )?;

        state.serialize_field(
            "resource_snapshot",
            &self.resource_snapshot,
        )?;

        state.serialize_field(
            "resources",
            &self.resources,
        )?;

        state.serialize_field(
            "interfaces",
            &self.interfaces,
        )?;

        state.serialize_field(
            "metadata",
            &self.metadata,
        )?;

        state.serialize_field(
            "provenance",
            &self.provenance,
        )?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for HardwareDeviceDescriptor {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            schema_version: Option<String>,
            identity: HardwareDeviceIdentity,
            category: String,
            display_name: String,
            availability: DeviceAvailability,

            capability_snapshot: Option<HardwareSnapshotId>,
            resource_snapshot: Option<HardwareSnapshotId>,

            #[serde(default)]
            resources: BTreeSet<HardwareResourceId>,

            #[serde(default)]
            interfaces: BTreeSet<HardwareInterfaceId>,

            #[serde(default)]
            metadata: HardwareMetadata,

            provenance: Option<HardwareDeviceProvenance>,
        }

        let wire = Wire::deserialize(deserializer)?;

        if let Some(version) = &wire.schema_version {
            if version != DEVICE_SCHEMA_VERSION {
                return Err(D::Error::custom(format!(
                    "unsupported device schema version: {version}"
                )));
            }
        }

        let category = DeviceKindId::new(
            wire.category,
        )
        .map_err(D::Error::custom)?;

        let descriptor = Self {
            identity: wire.identity,
            category,
            display_name: wire.display_name,
            availability: wire.availability,
            capability_snapshot: wire.capability_snapshot,
            resource_snapshot: wire.resource_snapshot,
            resources: wire.resources,
            interfaces: wire.interfaces,
            metadata: wire.metadata,
            provenance: wire.provenance,
        };

        descriptor
            .validate()
            .map_err(D::Error::custom)?;

        Ok(descriptor)
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

    fn identity() -> HardwareDeviceIdentity {
        HardwareDeviceIdentity::new(
            ProviderId::new("org.example.hardware")
                .expect("valid provider identity"),
            DeviceId::new("device/processor-01")
                .expect("valid device identity"),
        )
    }

    fn category() -> DeviceKindId {
        DeviceKindId::new("classical.processor")
            .expect("valid device category")
    }

    fn descriptor() -> HardwareDeviceDescriptor {
        HardwareDeviceDescriptor::new(
            identity(),
            category(),
            "Example processor",
        )
        .expect("valid descriptor")
    }

    #[test]
    fn creates_valid_device_descriptor() {
        let device = descriptor();

        assert_eq!(
            device.display_name(),
            "Example processor"
        );

        assert_eq!(
            device.provider_id().as_str(),
            "org.example.hardware"
        );

        assert_eq!(
            device.device_id().as_str(),
            "device/processor-01"
        );

        assert!(device.availability().is_unknown());
    }

    #[test]
    fn rejects_empty_display_name() {
        assert!(
            HardwareDeviceDescriptor::new(
                identity(),
                category(),
                "   ",
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_control_characters_in_display_name() {
        assert!(
            HardwareDeviceDescriptor::new(
                identity(),
                category(),
                "processor\ninjected",
            )
            .is_err()
        );
    }

    #[test]
    fn availability_does_not_assume_unknown_is_available() {
        let device = descriptor();

        assert!(!device.is_available());

        assert!(device.availability().is_unknown());
    }

    #[test]
    fn supports_available_observation() {
        let device = descriptor()
            .with_availability(
                DeviceAvailability::available(),
            )
            .expect("valid availability");

        assert!(device.is_available());
    }

    #[test]
    fn supports_unavailable_observation() {
        let availability = DeviceAvailability::unavailable(
            Some("device disconnected".to_owned()),
        )
        .expect("valid unavailable state");

        assert!(availability.is_unavailable());
        assert!(!availability.is_available());
    }

    #[test]
    fn supports_provider_defined_availability_states() {
        let availability = DeviceAvailability::new(
            "provider_specific.maintenance",
            None,
        )
        .expect("open-world state");

        assert_eq!(
            availability.state(),
            "provider_specific.maintenance"
        );

        assert!(!availability.is_available());
    }

    #[test]
    fn rejects_invalid_availability_state() {
        assert!(
            DeviceAvailability::new(
                "invalid state",
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn resources_are_deterministic_and_deduplicated() {
        let first = HardwareResourceId::new(
            "resource.processor",
        )
        .expect("valid resource");

        let second = HardwareResourceId::new(
            "resource.memory",
        )
        .expect("valid resource");

        let device = descriptor()
            .with_resources([
                first.clone(),
                second.clone(),
                first.clone(),
            ]);

        assert_eq!(device.resources().len(), 2);

        assert!(device.resources().contains(&first));
        assert!(device.resources().contains(&second));
    }

    #[test]
    fn interfaces_are_deterministic_and_deduplicated() {
        let interface = HardwareInterfaceId::new(
            "interface.pcie",
        )
        .expect("valid interface");

        let device = descriptor()
            .with_interfaces([
                interface.clone(),
                interface.clone(),
            ]);

        assert_eq!(device.interfaces().len(), 1);
    }

    #[test]
    fn preserves_snapshot_references() {
        let capability_snapshot = HardwareSnapshotId::new(
            "capability.snapshot.001",
        )
        .expect("valid snapshot identity");

        let resource_snapshot = HardwareSnapshotId::new(
            "resource.snapshot.001",
        )
        .expect("valid snapshot identity");

        let device = descriptor()
            .with_capability_snapshot(
                capability_snapshot.clone(),
            )
            .with_resource_snapshot(
                resource_snapshot.clone(),
            );

        assert_eq!(
            device.capability_snapshot(),
            Some(&capability_snapshot)
        );

        assert_eq!(
            device.resource_snapshot(),
            Some(&resource_snapshot)
        );
    }

    #[test]
    fn preserves_provenance() {
        let provenance = HardwareDeviceProvenance::new(
            "provider.discovery",
            Some("2026-11-14T12:00:00Z".to_owned()),
            Some("observation-001".to_owned()),
        )
        .expect("valid provenance");

        let device = descriptor()
            .with_provenance(provenance)
            .expect("valid device provenance");

        assert_eq!(
            device.provenance()
                .and_then(HardwareDeviceProvenance::source),
            None
        );

        assert_eq!(
            device.provenance()
                .map(HardwareDeviceProvenance::source),
            Some("provider.discovery")
        );
    }

    #[test]
    fn rejects_invalid_provenance() {
        assert!(
            HardwareDeviceProvenance::new(
                " ",
                None,
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn validates_complete_descriptor() {
        let device = descriptor()
            .with_availability(
                DeviceAvailability::available(),
            )
            .expect("valid availability");

        assert!(device.validate().is_ok());
    }

    #[test]
    fn serializes_and_deserializes_device_descriptor() {
        let device = descriptor()
            .with_availability(
                DeviceAvailability::available(),
            )
            .expect("valid availability")
            .with_interface(
                HardwareInterfaceId::new("interface.pcie")
                    .expect("valid interface"),
            )
            .with_resource(
                HardwareResourceId::new("resource.compute")
                    .expect("valid resource"),
            )
            .with_provenance(
                HardwareDeviceProvenance::new(
                    "provider.discovery",
                    Some("2026-11-14T12:00:00Z".to_owned()),
                    Some("observation-001".to_owned()),
                )
                .expect("valid provenance"),
            )
            .expect("valid provenance");

        let encoded = serde_json::to_string(&device)
            .expect("serialization succeeds");

        let decoded: HardwareDeviceDescriptor =
            serde_json::from_str(&encoded)
                .expect("deserialization succeeds");

        assert_eq!(device, decoded);
    }

    #[test]
    fn rejects_unsupported_schema_versions() {
        let encoded = serde_json::json!({
            "schema_version": "unsupported",
            "identity": {
                "provider_id": "org.example.hardware",
                "device_id": "device/processor-01"
            },
            "category": "classical.processor",
            "display_name": "Example processor",
            "availability": {
                "state": "unknown",
                "reason": null
            }
        });

        assert!(
            serde_json::from_value::<HardwareDeviceDescriptor>(
                encoded
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_invalid_deserialized_display_name() {
        let encoded = serde_json::json!({
            "schema_version": DEVICE_SCHEMA_VERSION,
            "identity": {
                "provider_id": "org.example.hardware",
                "device_id": "device/processor-01"
            },
            "category": "classical.processor",
            "display_name": "invalid\nname",
            "availability": {
                "state": "unknown",
                "reason": null
            }
        });

        assert!(
            serde_json::from_value::<HardwareDeviceDescriptor>(
                encoded
            )
            .is_err()
        );
    }

    #[test]
    fn provider_scoped_identity_distinguishes_same_local_device_id() {
        let first = HardwareDeviceIdentity::new(
            ProviderId::new("org.example.provider_a").unwrap(),
            DeviceId::new("device-01").unwrap(),
        );

        let second = HardwareDeviceIdentity::new(
            ProviderId::new("org.example.provider_b").unwrap(),
            DeviceId::new("device-01").unwrap(),
        );

        assert_ne!(first, second);
    }

    #[test]
    fn descriptor_schema_version_is_stable() {
        assert_eq!(
            descriptor().schema_version(),
            DEVICE_SCHEMA_VERSION
        );
    }
}
