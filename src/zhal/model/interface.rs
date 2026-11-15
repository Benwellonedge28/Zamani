
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! Hardware Interface Model.
//!
//! Path: `src/zhal/model/interface.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the canonical provider-independent description of a physical
//! or provider-backed hardware interface.
//!
//! An interface may belong to classical, quantum, HDL, accelerator,
//! embedded, distributed, hybrid or future computational hardware.
//!
//! Examples include:
//!
//! - Processor interfaces.
//! - Memory interfaces.
//! - Accelerator interfaces.
//! - Quantum-control interfaces.
//! - FPGA programming interfaces.
//! - ASIC programming interfaces.
//! - Interconnect interfaces.
//! - Device communication interfaces.
//! - Sensor and actuator interfaces.
//! - Remote hardware interfaces.
//! - Future provider-defined interfaces.
//!
//! This module describes interfaces. It does not implement or execute them.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - HardwareInterfaceDescriptor.
//! - HardwareInterfaceKind.
//! - InterfaceDirection.
//! - InterfaceAvailability.
//! - InterfaceAvailabilityState.
//! - CommunicationCharacteristics.
//! - SupportedOperation.
//! - Interface validation.
//! - Interface serialization and validated deserialization.
//!
//! This module does NOT own:
//!
//! - Hardware interface identity.
//! - Hardware device identity.
//! - Provider identity.
//! - Provider lifecycle.
//! - Provider registration.
//! - Capability identity or capability evaluation.
//! - Physical device discovery.
//! - Physical device communication.
//! - Byte-oriented device transport.
//! - Resource quantities or allocation.
//! - Resource reservation.
//! - Hardware execution.
//! - Hardware scheduling.
//! - Quantum control implementation.
//! - Quantum routing.
//! - HDL realization.
//! - FPGA synthesis.
//! - ASIC realization.
//! - Distributed transport.
//! - Hybrid execution.
//! - Runtime orchestration.
//! - Source grammar or semantic analysis.
//! - Intermediate representations.
//!
//! ## Canonical repository authorities
//!
//! Interface identity:
//!     crate::zhal::identity::HardwareInterfaceIdentity
//!
//! Interface identifier:
//!     crate::zhal::identity::HardwareInterfaceId
//!
//! Device identity:
//!     crate::zhal::identity::HardwareDeviceIdentity
//!
//! Provider identity:
//!     crate::stdlib::provider::ProviderId
//!
//! Provider operation identity:
//!     crate::stdlib::provider::ProviderOperationId
//!
//! Provider operation contracts:
//!     crate::stdlib::provider
//!
//! Semantic version:
//!     crate::stdlib::version::Version
//!
//! Generic device I/O:
//!     crate::stdlib::io::devices
//!
//! Capability authority:
//!     crate::stdlib::capability
//!
//! Quantum-specific interfaces:
//!     crate::quantum::hardware
//!
//! HDL realization interfaces:
//!     crate::hdl::realization
//!
//! Resource descriptions:
//!     crate::zhal::model::resource
//!
//! Runtime allocation and orchestration:
//!     crate::zur
//!
//! ## Identity
//!
//! Every interface has a device-scoped identity:
//!
//!     HardwareInterfaceIdentity
//!         |
//!         +-- HardwareDeviceIdentity
//!         |
//!         +-- HardwareInterfaceId
//!
//! An interface identifier is only unique within its owning device.
//!
//! Two different devices may expose the same local interface identifier.
//!
//! Their composite identities remain distinct.
//!
//! ## Open-world extensibility
//!
//! Interface kinds are open-ended identifiers.
//!
//! This module must not maintain a closed list of hardware interface types.
//!
//! A future provider must be able to introduce a new interface kind without
//! modifying this module.
//!
//! ## Communication characteristics
//!
//! Communication characteristics describe declared or observed interface
//! properties. They do not perform communication.
//!
//! Protocol and transport identifiers are open-ended.
//!
//! Numeric performance quantities, capacity and resource availability
//! remain owned by the resource and observation models.
//!
//! This module does not invent bandwidth, latency, throughput or reliability
//! measurements.
//!
//! ## Supported operations
//!
//! SupportedOperation references a canonical ProviderOperationId.
//!
//! It does not define a new operation language, request payload format,
//! execution engine or provider operation registry.
//!
//! A declared operation is not proof that the operation can currently
//! execute successfully.
//!
//! ## Availability
//!
//! Availability is an observation, not a reservation.
//!
//! Unknown is not equivalent to available.
//!
//! Availability may change after an observation.
//!
//! Consumers must revalidate availability before attempting physical access.
//!
//! Observation freshness and provenance are maintained by the appropriate
//! provider, inventory and snapshot authorities.
//!
//! ## Serialization
//!
//! Deserialization validates all locally owned identifiers and versions.
//!
//! Unknown interface kinds and operation identifiers remain representable.
//!
//! Serialization does not authenticate a provider or establish that
//! a physical interface exists.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Interface identifier length.
//! - Interface kind length.
//! - Interface count.
//! - Supported operation count.
//! - Property count.
//! - Provider count.
//! - Device count.
//! - Hardware categories.
//! - Communication protocols.
//! - Computational domains.
//!
//! Collections grow according to available memory and platform limits.
//!
//! Explicit operational budgets belong to configuration and runtime policy.
//!
//! Literal infinite physical storage or computation is not promised.
//!
//! ## Grammar boundary
//!
//! Source-level hardware declarations are owned by:
//!
//!     grammar/hardware/devices.g4
//!     grammar/hardware/topology.g4
//!
//! Those grammars describe logical hardware intent and relationships.
//!
//! This module describes provider-reported physical interface information.
//!
//! It must not parse grammar, construct AST nodes or interpret source-level
//! requirements.
//!
//! ## Integration
//!
//! The device model references HardwareInterfaceId values.
//!
//! The interface inventory stores HardwareInterfaceDescriptor values.
//!
//! Providers construct descriptors from their hardware observations.
//!
//! Device discovery aggregates descriptors into device observations.
//!
//! Hardware snapshots preserve the observation and its provenance.
//!
//! Quantum and HDL adapters translate specialized interface information
//! into this generic description without moving their domain-specific
//! authorities into ZHAL.
//!
//! ZUR consumes interface descriptions when evaluating physical access
//! and runtime execution requirements.
//!
//! ## Security
//!
//! Interface properties are descriptive metadata.
//!
//! They must not contain credentials, access tokens, passwords, private
//! keys, authorization headers or other secret material.
//!
//! Possession of an interface identity does not establish authorization.
//!
//! ## Determinism
//!
//! Ordered maps and sets are used for stable iteration and serialization.
//!
//! This module performs no network I/O, physical probing, random generation
//! or system-clock access.
//!
//! Availability timestamps and observation provenance belong to snapshots.

use core::fmt;
use core::str::FromStr;

use std::collections::{BTreeMap, BTreeSet};

use serde::de::Error as SerdeDeError;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::stdlib::provider::ProviderOperationId;
use crate::stdlib::version::Version;

use crate::zhal::identity::{
    HardwareDeviceIdentity,
    HardwareInterfaceId,
    HardwareInterfaceIdentity,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable architectural component identifier.
pub const COMPONENT: &str = "zhal.model.interface";

/// Serialized interface-model schema version.
///
/// This is independent of the Cargo package version, Zamani language
/// version, provider implementation version and hardware firmware version.
pub const INTERFACE_SCHEMA_VERSION: &str = "1";

// ============================================================================
// ERRORS
// ============================================================================

/// Classification of interface-model validation failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum InterfaceErrorKind {
    /// An identifier is empty.
    EmptyIdentifier,

    /// An identifier contains whitespace.
    InvalidIdentifier,

    /// An identifier contains a control character.
    ControlCharacter,

    /// A descriptive value is empty.
    EmptyValue,

    /// A descriptive value contains a control character.
    InvalidValue,

    /// A version string is not a valid semantic version.
    InvalidVersion,

    /// A property key is invalid.
    InvalidPropertyKey,

    /// A property contains an invalid value.
    InvalidPropertyValue,

    /// An operation identity is invalid.
    InvalidOperation,

    /// An operation identity occurs more than once.
    DuplicateOperation,

    /// An interface kind is invalid.
    InvalidInterfaceKind,

    /// An availability observation is inconsistent.
    InvalidAvailability,

    /// A communication characteristic is invalid.
    InvalidCommunicationCharacteristics,
}

impl fmt::Display for InterfaceErrorKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let message = match self {
            Self::EmptyIdentifier => "identifier must not be empty",
            Self::InvalidIdentifier => "identifier is invalid",
            Self::ControlCharacter => {
                "value must not contain control characters"
            }
            Self::EmptyValue => "value must not be empty",
            Self::InvalidValue => "value is invalid",
            Self::InvalidVersion => "version is invalid",
            Self::InvalidPropertyKey => "property key is invalid",
            Self::InvalidPropertyValue => "property value is invalid",
            Self::InvalidOperation => "operation identity is invalid",
            Self::DuplicateOperation => {
                "operation identity is duplicated"
            }
            Self::InvalidInterfaceKind => "interface kind is invalid",
            Self::InvalidAvailability => {
                "availability observation is invalid"
            }
            Self::InvalidCommunicationCharacteristics => {
                "communication characteristics are invalid"
            }
        };

        formatter.write_str(message)
    }
}

/// Structured interface-model validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterfaceError {
    kind: InterfaceErrorKind,
    field: &'static str,
    detail: Option<String>,
}

impl InterfaceError {
    fn new(
        kind: InterfaceErrorKind,
        field: &'static str,
    ) -> Self {
        Self {
            kind,
            field,
            detail: None,
        }
    }

    fn with_detail(
        kind: InterfaceErrorKind,
        field: &'static str,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            field,
            detail: Some(detail.into()),
        }
    }

    /// Returns the structured error classification.
    #[must_use]
    pub const fn kind(&self) -> InterfaceErrorKind {
        self.kind
    }

    /// Returns the field associated with the error.
    #[must_use]
    pub const fn field(&self) -> &'static str {
        self.field
    }

    /// Returns the optional explanatory detail.
    #[must_use]
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }
}

impl fmt::Display for InterfaceError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}: {}",
            self.field,
            self.kind
        )?;

        if let Some(detail) = &self.detail {
            write!(formatter, ": {detail}")?;
        }

        Ok(())
    }
}

impl std::error::Error for InterfaceError {}

/// Result type for interface-model operations.
pub type InterfaceResult<T> = Result<T, InterfaceError>;

// ============================================================================
// VALIDATION
// ============================================================================

/// Validates an open-world identifier.
///
/// Identifiers preserve their original spelling and case.
///
/// No arbitrary maximum length is imposed.
fn validate_identifier(
    value: &str,
    field: &'static str,
) -> InterfaceResult<()> {
    if value.is_empty() {
        return Err(InterfaceError::new(
            InterfaceErrorKind::EmptyIdentifier,
            field,
        ));
    }

    if value.trim() != value {
        return Err(InterfaceError::new(
            InterfaceErrorKind::InvalidIdentifier,
            field,
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(InterfaceError::new(
            InterfaceErrorKind::ControlCharacter,
            field,
        ));
    }

    if value.chars().any(char::is_whitespace) {
        return Err(InterfaceError::new(
            InterfaceErrorKind::InvalidIdentifier,
            field,
        ));
    }

    Ok(())
}

/// Validates a descriptive value.
///
/// Descriptive values may contain ordinary whitespace.
fn validate_description(
    value: &str,
    field: &'static str,
) -> InterfaceResult<()> {
    if value.trim().is_empty() {
        return Err(InterfaceError::new(
            InterfaceErrorKind::EmptyValue,
            field,
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(InterfaceError::new(
            InterfaceErrorKind::InvalidValue,
            field,
        ));
    }

    Ok(())
}

/// Parses and validates a semantic version.
fn validate_version(
    value: &str,
    field: &'static str,
) -> InterfaceResult<()> {
    Version::parse(value).map_err(|error| {
        InterfaceError::with_detail(
            InterfaceErrorKind::InvalidVersion,
            field,
            error.to_string(),
        )
    })?;

    Ok(())
}

// ============================================================================
// INTERFACE KIND
// ============================================================================

/// Open-world hardware interface kind.
///
/// Examples include:
///
/// - `processor.execution`
/// - `memory.addressing`
/// - `accelerator.compute`
/// - `quantum.control`
/// - `quantum.measurement`
/// - `hdl.configuration`
/// - `hdl.programming`
/// - `interconnect.fabric`
/// - `device.input`
/// - `device.output`
/// - `sensor.acquisition`
/// - `actuator.control`
///
/// These examples are not an exhaustive enumeration.
///
/// A provider may introduce additional kinds without changing ZHAL.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HardwareInterfaceKind(String);

impl HardwareInterfaceKind {
    /// Constructs an open-world interface kind.
    ///
    /// # Errors
    ///
    /// Returns an error when the identifier is empty or contains
    /// whitespace or control characters.
    pub fn new(
        value: impl Into<String>,
    ) -> InterfaceResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "interface kind",
        )?;

        Ok(Self(value))
    }

    /// Returns the exact interface-kind identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the kind and returns its owned identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for HardwareInterfaceKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for HardwareInterfaceKind {
    type Err = InterfaceError;

    fn from_str(value: &str) -> InterfaceResult<Self> {
        Self::new(value)
    }
}

impl Serialize for HardwareInterfaceKind {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for HardwareInterfaceKind {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(D::Error::custom)
    }
}

// ============================================================================
// INTERFACE DIRECTION
// ============================================================================

/// Describes the logical direction of information through an interface.
///
/// This is descriptive interface information. It does not implement
/// communication or determine access permissions.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum InterfaceDirection {
    /// The direction is not currently known.
    #[default]
    Unknown,

    /// Information enters the device through this interface.
    Input,

    /// Information leaves the device through this interface.
    Output,

    /// The interface supports both input and output.
    Bidirectional,

    /// The interface is internal to the described device.
    Internal,
}

// ============================================================================
// COMMUNICATION CHARACTERISTICS
// ============================================================================

/// Provider-independent descriptive communication characteristics.
///
/// Transport and protocol identifiers are open-world strings.
///
/// This type intentionally does not own numeric performance measurements.
/// Resource capacity, bandwidth, latency and physical measurements belong
/// to the resource and observation authorities.
///
/// Properties must not contain credentials or other secret material.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CommunicationCharacteristics {
    transport: Option<String>,
    protocol: Option<String>,
    direction: InterfaceDirection,
    properties: BTreeMap<String, String>,
}

impl CommunicationCharacteristics {
    /// Creates communication characteristics.
    ///
    /// `transport` and `protocol` are optional open-world identifiers.
    ///
    /// # Errors
    ///
    /// Returns an error if either supplied identifier is invalid.
    pub fn new(
        transport: Option<String>,
        protocol: Option<String>,
        direction: InterfaceDirection,
    ) -> InterfaceResult<Self> {
        if let Some(value) = &transport {
            validate_identifier(
                value,
                "communication transport",
            )?;
        }

        if let Some(value) = &protocol {
            validate_identifier(
                value,
                "communication protocol",
            )?;
        }

        Ok(Self {
            transport,
            protocol,
            direction,
            properties: BTreeMap::new(),
        })
    }

    /// Returns the optional transport identifier.
    #[must_use]
    pub fn transport(&self) -> Option<&str> {
        self.transport.as_deref()
    }

    /// Returns the optional protocol identifier.
    #[must_use]
    pub fn protocol(&self) -> Option<&str> {
        self.protocol.as_deref()
    }

    /// Returns the interface direction.
    #[must_use]
    pub const fn direction(&self) -> InterfaceDirection {
        self.direction
    }

    /// Returns descriptive properties in deterministic key order.
    #[must_use]
    pub fn properties(&self) -> &BTreeMap<String, String> {
        &self.properties
    }

    /// Inserts or replaces a descriptive communication property.
    ///
    /// Keys must be nonempty identifiers. Values may contain ordinary
    /// whitespace but cannot contain control characters.
    ///
    /// Properties are descriptive only. They must not be used to
    /// smuggle credentials, authorization material or secrets.
    ///
    /// # Errors
    ///
    /// Returns an error if the key or value is invalid.
    pub fn insert_property(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> InterfaceResult<Option<String>> {
        let key = key.into();
        let value = value.into();

        validate_identifier(
            &key,
            "communication property key",
        )
        .map_err(|_| {
            InterfaceError::new(
                InterfaceErrorKind::InvalidPropertyKey,
                "communication property key",
            )
        })?;

        validate_description(
            &value,
            "communication property value",
        )
        .map_err(|_| {
            InterfaceError::new(
                InterfaceErrorKind::InvalidPropertyValue,
                "communication property value",
            )
        })?;

        Ok(self.properties.insert(key, value))
    }

    /// Removes a descriptive property.
    ///
    /// Returns the previous value when present.
    pub fn remove_property(
        &mut self,
        key: &str,
    ) -> Option<String> {
        self.properties.remove(key)
    }

    /// Returns whether no communication properties are present.
    #[must_use]
    pub fn has_no_properties(&self) -> bool {
        self.properties.is_empty()
    }
}

impl Serialize for CommunicationCharacteristics {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct(
            "CommunicationCharacteristics",
            4,
        )?;

        state.serialize_field("transport", &self.transport)?;
        state.serialize_field("protocol", &self.protocol)?;
        state.serialize_field("direction", &self.direction)?;
        state.serialize_field("properties", &self.properties)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for CommunicationCharacteristics {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            transport: Option<String>,
            protocol: Option<String>,
            direction: InterfaceDirection,
            properties: BTreeMap<String, String>,
        }

        let wire = Wire::deserialize(deserializer)?;

        let mut result = Self::new(
            wire.transport,
            wire.protocol,
            wire.direction,
        )
        .map_err(D::Error::custom)?;

        for (key, value) in wire.properties {
            result
                .insert_property(key, value)
                .map_err(D::Error::custom)?;
        }

        Ok(result)
    }
}

// ============================================================================
// AVAILABILITY
// ============================================================================

/// Current observed interface availability state.
///
/// Availability is not authorization, reservation or guaranteed access.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum InterfaceAvailabilityState {
    /// No trustworthy current availability observation is available.
    ///
    /// Unknown must never be treated as available.
    #[default]
    Unknown,

    /// The interface was observed as available.
    Available,

    /// The interface was observed as unavailable.
    Unavailable,

    /// The interface was observed as available with limitations.
    Degraded,

    /// Availability depends on stated conditions.
    Conditional,
}

/// An interface availability observation.
///
/// This type records a state and optional explanation. Observation time,
/// evidence source, freshness and provenance belong to the enclosing
/// hardware observation/snapshot model.
///
/// The observation does not guarantee that the interface will remain
/// available when subsequently accessed.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InterfaceAvailability {
    state: InterfaceAvailabilityState,
    condition: Option<String>,
}

impl InterfaceAvailability {
    /// Creates an availability observation.
    ///
    /// A condition may explain degraded or conditional availability.
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied explanation is empty or invalid.
    pub fn new(
        state: InterfaceAvailabilityState,
        condition: Option<String>,
    ) -> InterfaceResult<Self> {
        if let Some(value) = &condition {
            validate_description(
                value,
                "availability condition",
            )?;
        }

        Ok(Self {
            state,
            condition,
        })
    }

    /// Creates an unknown availability observation.
    #[must_use]
    pub fn unknown() -> Self {
        Self::default()
    }

    /// Returns the observed availability state.
    #[must_use]
    pub const fn state(&self) -> InterfaceAvailabilityState {
        self.state
    }

    /// Returns the optional availability explanation.
    #[must_use]
    pub fn condition(&self) -> Option<&str> {
        self.condition.as_deref()
    }

    /// Returns whether availability is explicitly established.
    ///
    /// Degraded and conditional availability are not equivalent
    /// to unrestricted availability.
    #[must_use]
    pub const fn is_available(&self) -> bool {
        matches!(
            self.state,
            InterfaceAvailabilityState::Available
        )
    }

    /// Returns whether the state is unknown.
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        matches!(
            self.state,
            InterfaceAvailabilityState::Unknown
        )
    }
}

impl Serialize for InterfaceAvailability {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct(
            "InterfaceAvailability",
            2,
        )?;

        state.serialize_field("state", &self.state)?;
        state.serialize_field("condition", &self.condition)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for InterfaceAvailability {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            state: InterfaceAvailabilityState,
            condition: Option<String>,
        }

        let wire = Wire::deserialize(deserializer)?;

        Self::new(
            wire.state,
            wire.condition,
        )
        .map_err(D::Error::custom)
    }
}

// ============================================================================
// SUPPORTED OPERATIONS
// ============================================================================

/// A reference to an operation contract supported by an interface.
///
/// The operation identity reuses the standard-library ProviderOperationId.
/// An interface does not create a second operation identity namespace.
///
/// The optional version identifies the declared operation contract version.
/// It does not identify a particular operation execution.
///
/// A declaration is not proof of current availability or successful
/// execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupportedOperation {
    id: ProviderOperationId,
    version: Option<Version>,
}

impl SupportedOperation {
    /// Constructs a supported-operation reference.
    ///
    /// # Errors
    ///
    /// Returns an error if a supplied version is invalid.
    pub fn new(
        id: ProviderOperationId,
        version: Option<Version>,
    ) -> Self {
        Self {
            id,
            version,
        }
    }

    /// Constructs a supported-operation reference from a version string.
    ///
    /// # Errors
    ///
    /// Returns an error when the version is not valid semantic versioning.
    pub fn with_version(
        id: ProviderOperationId,
        version: &str,
    ) -> InterfaceResult<Self> {
        let version = Version::parse(version).map_err(|error| {
            InterfaceError::with_detail(
                InterfaceErrorKind::InvalidVersion,
                "supported operation version",
                error.to_string(),
            )
        })?;

        Ok(Self {
            id,
            version: Some(version),
        })
    }

    /// Returns the canonical provider operation identity.
    #[must_use]
    pub fn id(&self) -> &ProviderOperationId {
        &self.id
    }

    /// Returns the optional operation contract version.
    #[must_use]
    pub fn version(&self) -> Option<&Version> {
        self.version.as_ref()
    }
}

impl Serialize for SupportedOperation {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct(
            "SupportedOperation",
            2,
        )?;

        state.serialize_field(
            "id",
            self.id.as_str(),
        )?;

        let version = self.version.as_ref().map(ToString::to_string);

        state.serialize_field(
            "version",
            &version,
        )?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for SupportedOperation {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            id: String,
            version: Option<String>,
        }

        let wire = Wire::deserialize(deserializer)?;

        let id = ProviderOperationId::new(wire.id)
            .map_err(D::Error::custom)?;

        let version = match wire.version {
            Some(value) => Some(
                Version::parse(&value)
                    .map_err(D::Error::custom)?,
            ),
            None => None,
        };

        Ok(Self {
            id,
            version,
        })
    }
}

// ============================================================================
// HARDWARE INTERFACE DESCRIPTOR
// ============================================================================

/// Canonical provider-independent hardware interface descriptor.
///
/// The descriptor describes one interface belonging to one hardware device.
///
/// Its identity is device-scoped.
///
/// It is an immutable description from the perspective of consumers.
/// Providers may publish a newer descriptor when the hardware or its
/// observed state changes.
///
/// The descriptor does not provide direct access to the physical interface.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HardwareInterfaceDescriptor {
    identity: HardwareInterfaceIdentity,
    kind: HardwareInterfaceKind,
    version: Version,
    availability: InterfaceAvailability,
    communication: CommunicationCharacteristics,
    supported_operations: BTreeMap<ProviderOperationId, SupportedOperation>,
}

impl HardwareInterfaceDescriptor {
    /// Constructs a validated hardware interface descriptor.
    ///
    /// The interface identity must already contain a validated device
    /// identity and interface identifier.
    ///
    /// The interface version is a semantic version.
    ///
    /// Supported operations are indexed by their canonical operation
    /// identities. Duplicate operation identities are rejected.
    ///
    /// # Errors
    ///
    /// Returns an error if the version is invalid or duplicate operation
    /// identities are supplied.
    pub fn new(
        identity: HardwareInterfaceIdentity,
        kind: HardwareInterfaceKind,
        version: Version,
        availability: InterfaceAvailability,
        communication: CommunicationCharacteristics,
        supported_operations: impl IntoIterator<Item = SupportedOperation>,
    ) -> InterfaceResult<Self> {
        let mut operations = BTreeMap::new();

        for operation in supported_operations {
            let id = operation.id().clone();

            if operations.insert(id, operation).is_some() {
                return Err(InterfaceError::new(
                    InterfaceErrorKind::DuplicateOperation,
                    "supported operations",
                ));
            }
        }

        Ok(Self {
            identity,
            kind,
            version,
            availability,
            communication,
            supported_operations: operations,
        })
    }

    /// Constructs a descriptor from a semantic version string.
    ///
    /// # Errors
    ///
    /// Returns an error if the version is invalid or the descriptor
    /// contains duplicate operation identities.
    pub fn from_version(
        identity: HardwareInterfaceIdentity,
        kind: HardwareInterfaceKind,
        version: &str,
        availability: InterfaceAvailability,
        communication: CommunicationCharacteristics,
        supported_operations: impl IntoIterator<Item = SupportedOperation>,
    ) -> InterfaceResult<Self> {
        let version = Version::parse(version).map_err(|error| {
            InterfaceError::with_detail(
                InterfaceErrorKind::InvalidVersion,
                "interface version",
                error.to_string(),
            )
        })?;

        Self::new(
            identity,
            kind,
            version,
            availability,
            communication,
            supported_operations,
        )
    }

    /// Returns the complete device-scoped interface identity.
    #[must_use]
    pub fn identity(&self) -> &HardwareInterfaceIdentity {
        &self.identity
    }

    /// Returns the owning hardware device.
    #[must_use]
    pub fn device(&self) -> &HardwareDeviceIdentity {
        self.identity.device()
    }

    /// Returns the local interface identifier.
    #[must_use]
    pub fn interface_id(&self) -> &HardwareInterfaceId {
        self.identity.interface_id()
    }

    /// Returns the open-world interface kind.
    #[must_use]
    pub fn kind(&self) -> &HardwareInterfaceKind {
        &self.kind
    }

    /// Returns the interface contract version.
    #[must_use]
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Returns the availability observation.
    #[must_use]
    pub fn availability(&self) -> &InterfaceAvailability {
        &self.availability
    }

    /// Returns communication characteristics.
    #[must_use]
    pub fn communication(
        &self,
    ) -> &CommunicationCharacteristics {
        &self.communication
    }

    /// Returns supported operations in deterministic identity order.
    #[must_use]
    pub fn supported_operations(
        &self,
    ) -> impl Iterator<Item = &SupportedOperation> {
        self.supported_operations.values()
    }

    /// Returns the number of supported operation contracts.
    #[must_use]
    pub fn supported_operation_count(&self) -> usize {
        self.supported_operations.len()
    }

    /// Returns whether an operation is declared as supported.
    ///
    /// This does not establish that the operation is currently executable.
    #[must_use]
    pub fn supports_operation(
        &self,
        id: &ProviderOperationId,
    ) -> bool {
        self.supported_operations.contains_key(id)
    }

    /// Finds a supported operation by canonical identity.
    #[must_use]
    pub fn operation(
        &self,
        id: &ProviderOperationId,
    ) -> Option<&SupportedOperation> {
        self.supported_operations.get(id)
    }

    /// Returns whether the interface has explicitly observed availability.
    ///
    /// Only the Available state returns true.
    #[must_use]
    pub fn is_available(&self) -> bool {
        self.availability.is_available()
    }
}

// ============================================================================
// SERIALIZATION
// ============================================================================

impl Serialize for HardwareInterfaceDescriptor {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct(
            "HardwareInterfaceDescriptor",
            7,
        )?;

        state.serialize_field(
            "schema_version",
            INTERFACE_SCHEMA_VERSION,
        )?;

        state.serialize_field(
            "device",
            self.identity.device(),
        )?;

        state.serialize_field(
            "interface_id",
            self.identity.interface_id(),
        )?;

        state.serialize_field(
            "kind",
            &self.kind,
        )?;

        state.serialize_field(
            "version",
            &self.version.to_string(),
        )?;

        state.serialize_field(
            "availability",
            &self.availability,
        )?;

        state.serialize_field(
            "communication",
            &self.communication,
        )?;

        let operations: Vec<&SupportedOperation> =
            self.supported_operations.values().collect();

        state.serialize_field(
            "supported_operations",
            &operations,
        )?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for HardwareInterfaceDescriptor {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            schema_version: String,
            device: HardwareDeviceIdentity,
            interface_id: HardwareInterfaceId,
            kind: HardwareInterfaceKind,
            version: String,
            availability: InterfaceAvailability,
            communication: CommunicationCharacteristics,
            supported_operations: Vec<SupportedOperation>,
        }

        let wire = Wire::deserialize(deserializer)?;

        if wire.schema_version != INTERFACE_SCHEMA_VERSION {
            return Err(D::Error::custom(format!(
                "unsupported hardware interface schema version: {}",
                wire.schema_version
            )));
        }

        let identity = HardwareInterfaceIdentity::new(
            wire.device,
            wire.interface_id,
        );

        let version = Version::parse(&wire.version)
            .map_err(D::Error::custom)?;

        Self::new(
            identity,
            wire.kind,
            version,
            wire.availability,
            wire.communication,
            wire.supported_operations,
        )
        .map_err(D::Error::custom)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::io::devices::DeviceId;
    use crate::stdlib::provider::ProviderId;

    fn device_identity() -> HardwareDeviceIdentity {
        HardwareDeviceIdentity::new(
            ProviderId::new("example.provider")
                .expect("valid provider identity"),
            DeviceId::new("device-1")
                .expect("valid device identity"),
        )
    }

    fn interface_identity(
        id: &str,
    ) -> HardwareInterfaceIdentity {
        HardwareInterfaceIdentity::new(
            device_identity(),
            HardwareInterfaceId::new(id)
                .expect("valid interface identity"),
        )
    }

    fn basic_descriptor() -> HardwareInterfaceDescriptor {
        let operation = SupportedOperation::with_version(
            ProviderOperationId::new("device.transfer")
                .expect("valid operation identity"),
            "1.0.0",
        )
        .expect("valid operation version");

        let communication = CommunicationCharacteristics::new(
            Some("provider.transport".to_owned()),
            Some("provider.protocol".to_owned()),
            InterfaceDirection::Bidirectional,
        )
        .expect("valid communication characteristics");

        HardwareInterfaceDescriptor::from_version(
            interface_identity("interface-1"),
            HardwareInterfaceKind::new("memory.access")
                .expect("valid interface kind"),
            "1.0.0",
            InterfaceAvailability::new(
                InterfaceAvailabilityState::Available,
                None,
            )
            .expect("valid availability"),
            communication,
            [operation],
        )
        .expect("valid interface descriptor")
    }

    #[test]
    fn interface_identity_is_device_scoped() {
        let first = interface_identity("port-1");

        let second = HardwareInterfaceIdentity::new(
            HardwareDeviceIdentity::new(
                ProviderId::new("another.provider")
                    .expect("valid provider"),
                DeviceId::new("device-1")
                    .expect("valid device"),
            ),
            HardwareInterfaceId::new("port-1")
                .expect("valid interface"),
        );

        assert_ne!(first, second);
    }

    #[test]
    fn interface_kind_is_open_world() {
        let kind = HardwareInterfaceKind::new(
            "future.photonic.quantum.control",
        )
        .expect("future interface kind must be accepted");

        assert_eq!(
            kind.as_str(),
            "future.photonic.quantum.control"
        );
    }

    #[test]
    fn interface_kind_rejects_invalid_identifiers() {
        assert!(
            HardwareInterfaceKind::new("").is_err()
        );

        assert!(
            HardwareInterfaceKind::new("invalid kind").is_err()
        );

        assert!(
            HardwareInterfaceKind::new("invalid\nkind").is_err()
        );
    }

    #[test]
    fn communication_properties_are_deterministic() {
        let mut communication =
            CommunicationCharacteristics::default();

        communication
            .insert_property(
                "vendor.mode",
                "high precision",
            )
            .expect("valid property");

        communication
            .insert_property(
                "transport.mode",
                "reliable",
            )
            .expect("valid property");

        let keys: Vec<_> = communication
            .properties()
            .keys()
            .map(String::as_str)
            .collect();

        assert_eq!(
            keys,
            vec![
                "transport.mode",
                "vendor.mode",
            ]
        );
    }

    #[test]
    fn invalid_communication_properties_are_rejected() {
        let mut communication =
            CommunicationCharacteristics::default();

        assert!(
            communication
                .insert_property("", "value")
                .is_err()
        );

        assert!(
            communication
                .insert_property("valid.key", "")
                .is_err()
        );

        assert!(
            communication
                .insert_property("valid.key", "bad\nvalue")
                .is_err()
        );
    }

    #[test]
    fn unknown_availability_is_not_available() {
        let availability = InterfaceAvailability::unknown();

        assert!(availability.is_unknown());
        assert!(!availability.is_available());
    }

    #[test]
    fn degraded_availability_is_not_unrestricted_availability() {
        let availability = InterfaceAvailability::new(
            InterfaceAvailabilityState::Degraded,
            Some("Reduced operating capability".to_owned()),
        )
        .expect("valid degraded observation");

        assert!(!availability.is_available());
    }

    #[test]
    fn supported_operations_are_open_world() {
        let operation = SupportedOperation::new(
            ProviderOperationId::new(
                "future.quantum.control.operation",
            )
            .expect("valid operation"),
            None,
        );

        let descriptor = HardwareInterfaceDescriptor::new(
            interface_identity("control-1"),
            HardwareInterfaceKind::new("quantum.control")
                .expect("valid kind"),
            Version::parse("1.0.0").expect("valid version"),
            InterfaceAvailability::unknown(),
            CommunicationCharacteristics::default(),
            [operation],
        )
        .expect("valid descriptor");

        let operation_id = ProviderOperationId::new(
            "future.quantum.control.operation",
        )
        .expect("valid operation identity");

        assert!(descriptor.supports_operation(&operation_id));
    }

    #[test]
    fn duplicate_supported_operations_are_rejected() {
        let id = ProviderOperationId::new("device.transfer")
            .expect("valid operation");

        let first = SupportedOperation::new(
            id.clone(),
            None,
        );

        let second = SupportedOperation::new(
            id,
            None,
        );

        let result = HardwareInterfaceDescriptor::new(
            interface_identity("interface-1"),
            HardwareInterfaceKind::new("device.io")
                .expect("valid kind"),
            Version::parse("1.0.0").expect("valid version"),
            InterfaceAvailability::unknown(),
            CommunicationCharacteristics::default(),
            [first, second],
        );

        assert!(matches!(
            result,
            Err(InterfaceError {
                kind: InterfaceErrorKind::DuplicateOperation,
                ..
            })
        ));
    }

    #[test]
    fn invalid_interface_version_is_rejected() {
        assert!(
            HardwareInterfaceDescriptor::from_version(
                interface_identity("interface-1"),
                HardwareInterfaceKind::new("device.io")
                    .expect("valid kind"),
                "not-a-version",
                InterfaceAvailability::unknown(),
                CommunicationCharacteristics::default(),
                [],
            )
            .is_err()
        );
    }

    #[test]
    fn descriptor_serialization_round_trips() {
        let descriptor = basic_descriptor();

        let encoded = serde_json::to_string(&descriptor)
            .expect("serialization must succeed");

        let decoded: HardwareInterfaceDescriptor =
            serde_json::from_str(&encoded)
                .expect("validated deserialization must succeed");

        assert_eq!(descriptor, decoded);
    }

    #[test]
    fn invalid_serialized_interface_kind_is_rejected() {
        let encoded = serde_json::json!({
            "schema_version": INTERFACE_SCHEMA_VERSION,
            "device": {
                "provider_id": "example.provider",
                "device_id": "device-1"
            },
            "interface_id": "interface-1",
            "kind": "invalid kind",
            "version": "1.0.0",
            "availability": {
                "state": "Available",
                "condition": null
            },
            "communication": {
                "transport": null,
                "protocol": null,
                "direction": "Unknown",
                "properties": {}
            },
            "supported_operations": []
        });

        assert!(
            serde_json::from_value::<HardwareInterfaceDescriptor>(
                encoded
            )
            .is_err()
        );
    }

    #[test]
    fn serialized_schema_version_is_checked() {
        let mut encoded = serde_json::to_value(basic_descriptor())
            .expect("serialization must succeed");

        encoded["schema_version"] =
            serde_json::Value::String("999".to_owned());

        assert!(
            serde_json::from_value::<HardwareInterfaceDescriptor>(
                encoded
            )
            .is_err()
        );
    }

    #[test]
    fn interface_identifiers_support_unicode() {
        let kind = HardwareInterfaceKind::new(
            "接口.量子控制",
        )
        .expect("valid Unicode interface kind");

        assert_eq!(
            kind.as_str(),
            "接口.量子控制"
        );
    }

    #[test]
    fn descriptor_has_no_fixed_operation_ceiling() {
        let operations = (0usize..1024).map(|index| {
            SupportedOperation::new(
                ProviderOperationId::new(
                    format!("provider.operation.{index}"),
                )
                .expect("valid operation identity"),
                None,
            )
        });

        let descriptor = HardwareInterfaceDescriptor::new(
            interface_identity("large-interface"),
            HardwareInterfaceKind::new("provider.extensible")
                .expect("valid kind"),
            Version::parse("1.0.0").expect("valid version"),
            InterfaceAvailability::unknown(),
            CommunicationCharacteristics::default(),
            operations,
        )
        .expect("all distinct operations should be accepted");

        assert_eq!(
            descriptor.supported_operation_count(),
            1024
        );
    }
}
