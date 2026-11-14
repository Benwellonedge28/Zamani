
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! Path: `src/zhal/identity.rs`
//!
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Defines stable, provider-independent hardware identity composition
//! and strongly typed identifiers used by ZHAL.
//!
//! This module establishes identity contracts for:
//!
//! - Physical hardware devices.
//! - Hardware interfaces.
//! - Hardware resources.
//! - Hardware sessions.
//! - Hardware operations.
//! - Hardware inventories.
//! - Hardware topologies.
//! - Hardware observation snapshots.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - ZHAL-specific identity types.
//! - Device identity composition.
//! - Hardware interface identity composition.
//! - Hardware resource identity composition.
//! - Hardware identifier validation.
//! - Stable identity comparison.
//! - Identity serialization and validated deserialization.
//!
//! This module does NOT own:
//!
//! - Provider identity.
//! - Device identity implementation.
//! - Device-kind identity.
//! - Capability identity.
//! - Semantic source-language names.
//! - Quantum backend identity.
//! - Compiler target identity.
//! - Physical resource allocation.
//! - Hardware discovery.
//! - Hardware scheduling.
//! - Hardware routing.
//! - Hardware execution.
//! - Hardware address interpretation.
//!
//! These responsibilities remain with their existing authorities.
//!
//! # Canonical repository authorities
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
//! Capability identity:
//!     crate::stdlib::capability::CapabilityId
//!
//! Version representation:
//!     crate::stdlib::version::Version
//!
//! Quantum hardware:
//!     crate::quantum::hardware
//!
//! Runtime device integration:
//!     crate::zur::io::devices
//!
//! # Identity invariants
//!
//! 1. An identity must not be empty.
//! 2. An identity must not contain whitespace.
//! 3. An identity must not contain control characters.
//! 4. An identity is case-sensitive.
//! 5. An identity is not normalized or silently rewritten.
//! 6. An identity is independent of physical location.
//! 7. An identity is independent of hardware vendor.
//! 8. An identity is independent of operating system.
//! 9. An identity is independent of computational domain.
//! 10. Deserialization must validate identifiers.
//! 11. Equality must preserve exact identity.
//! 12. Hashing must be consistent with equality.
//! 13. Ordering must be deterministic.
//!
//! # Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Identifier length.
//! - Namespace depth.
//! - Number of devices.
//! - Number of interfaces.
//! - Number of resources.
//! - Number of sessions.
//! - Number of operations.
//! - Number of inventories.
//! - Number of topologies.
//! - Number of snapshots.
//! - Hardware categories.
//! - Computing domains.
//!
//! Actual requirements are constrained by available memory,
//! address-space representation, platform capabilities and
//! explicitly configured operational budgets.
//!
//! Literal infinite physical storage or computation is not promised.
//!
//! # POCO-REAF
//!
//! Hardware identities must remain independent of the hardware
//! realization selected for a portable Zamani program.
//!
//! A program must not need to embed a vendor-specific physical
//! device identity merely to express a hardware requirement.
//!
//! # Serialization
//!
//! Individual identifiers serialize as strings.
//!
//! Composite identities serialize as structured objects with
//! stable field names.
//!
//! Every deserialized identifier is validated before construction.
//!
//! Serialization is independent of physical hardware availability.
//!
//! # Security
//!
//! Identifiers are descriptive identity data.
//!
//! They must never be used as containers for:
//!
//! - Passwords.
//! - Private keys.
//! - Access tokens.
//! - Authentication credentials.
//! - Authorization headers.
//! - Session secrets.
//!
//! Identity possession does not establish authorization.
//!
//! # Dependencies
//!
//! - Rust standard library.
//! - serde.
//! - crate::stdlib::provider.
//! - crate::stdlib::io::devices.
//!
//! No hardware SDK is required.

use core::fmt;
use core::str::FromStr;

use serde::de::Error as SerdeDeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::stdlib::io::devices::DeviceId;
use crate::stdlib::provider::ProviderId;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable ZHAL identity component name.
pub const COMPONENT: &str = "zhal.identity";

/// Identity contract version.
///
/// This identifies the identity schema contract, not the
/// Cargo package version or Zamani language version.
pub const IDENTITY_SCHEMA_VERSION: &str = "1";

// ============================================================================
// IDENTITY ERRORS
// ============================================================================

/// Classification of hardware identity validation failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdentityErrorKind {
    /// The supplied identifier is empty.
    Empty,

    /// The identifier contains leading or trailing whitespace.
    SurroundingWhitespace,

    /// The identifier contains whitespace.
    Whitespace,

    /// The identifier contains a control character.
    ControlCharacter,

    /// A provider identity is invalid.
    InvalidProviderIdentity,

    /// A device identity is invalid.
    InvalidDeviceIdentity,
}

impl fmt::Display for IdentityErrorKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let message = match self {
            Self::Empty => "identifier must not be empty",
            Self::SurroundingWhitespace => {
                "identifier must not contain surrounding whitespace"
            }
            Self::Whitespace => {
                "identifier must not contain whitespace"
            }
            Self::ControlCharacter => {
                "identifier must not contain control characters"
            }
            Self::InvalidProviderIdentity => {
                "provider identity is invalid"
            }
            Self::InvalidDeviceIdentity => {
                "device identity is invalid"
            }
        };

        formatter.write_str(message)
    }
}

/// Structured hardware identity validation error.
///
/// The error deliberately identifies the invalid component
/// without requiring callers to parse formatted error messages.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityError {
    kind: IdentityErrorKind,
    component: &'static str,
}

impl IdentityError {
    fn new(
        kind: IdentityErrorKind,
        component: &'static str,
    ) -> Self {
        Self {
            kind,
            component,
        }
    }

    /// Returns the error classification.
    #[must_use]
    pub const fn kind(&self) -> IdentityErrorKind {
        self.kind
    }

    /// Returns the component that failed validation.
    #[must_use]
    pub const fn component(&self) -> &'static str {
        self.component
    }
}

impl fmt::Display for IdentityError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}: {}",
            self.component,
            self.kind
        )
    }
}

impl std::error::Error for IdentityError {}

/// Result type for ZHAL identity construction.
pub type IdentityResult<T> = Result<T, IdentityError>;

// ============================================================================
// COMMON VALIDATION
// ============================================================================

/// Validates an opaque provider-defined identity.
///
/// Identity values are intentionally not restricted to source-language
/// identifier syntax, filesystem naming rules, URL syntax or vendor
/// naming conventions.
///
/// Exact spelling is preserved.
///
/// No arbitrary maximum length or namespace depth is imposed.
fn validate_identifier(
    value: &str,
    component: &'static str,
) -> IdentityResult<()> {
    if value.is_empty() {
        return Err(IdentityError::new(
            IdentityErrorKind::Empty,
            component,
        ));
    }

    if value.trim() != value {
        return Err(IdentityError::new(
            IdentityErrorKind::SurroundingWhitespace,
            component,
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(IdentityError::new(
            IdentityErrorKind::ControlCharacter,
            component,
        ));
    }

    if value.chars().any(char::is_whitespace) {
        return Err(IdentityError::new(
            IdentityErrorKind::Whitespace,
            component,
        ));
    }

    Ok(())
}

// ============================================================================
// OPEN-WORLD IDENTITY TYPES
// ============================================================================

/// Defines an opaque, strongly typed hardware identifier.
///
/// All generated identifiers:
///
/// - Preserve their exact supplied spelling.
/// - Are case-sensitive.
/// - Support deterministic ordering.
/// - Support hashing.
/// - Support validated deserialization.
/// - Have no arbitrary identifier-length ceiling.
///
/// The identifier's interpretation belongs to its owning
/// provider or hardware abstraction contract.
macro_rules! define_hardware_identity {
    (
        $name:ident,
        $description:literal,
        $component:literal
    ) => {
        #[doc = $description]
        #[derive(
            Clone,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            Serialize,
        )]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Constructs a validated hardware identifier.
            ///
            /// No arbitrary maximum length is imposed.
            ///
            /// # Errors
            ///
            /// Returns an identity validation error if the value
            /// is empty or contains whitespace or control characters.
            pub fn new(
                value: impl Into<String>,
            ) -> IdentityResult<Self> {
                let value = value.into();

                validate_identifier(
                    &value,
                    $component,
                )?;

                Ok(Self(value))
            }

            /// Returns the exact identifier.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consumes the identifier and returns its owned string.
            #[must_use]
            pub fn into_string(self) -> String {
                self.0
            }

            /// Returns the identifier's byte length.
            #[must_use]
            pub fn len(&self) -> usize {
                self.0.len()
            }

            /// Returns whether the identifier is empty.
            ///
            /// Valid instances cannot be empty.
            #[must_use]
            pub const fn is_empty(&self) -> bool {
                false
            }
        }

        impl fmt::Display for $name {
            fn fmt(
                &self,
                formatter: &mut fmt::Formatter<'_>,
            ) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl FromStr for $name {
            type Err = IdentityError;

            fn from_str(value: &str) -> IdentityResult<Self> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = IdentityError;

            fn try_from(value: &str) -> IdentityResult<Self> {
                Self::new(value)
            }
        }

        impl TryFrom<String> for $name {
            type Error = IdentityError;

            fn try_from(value: String) -> IdentityResult<Self> {
                Self::new(value)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(
                deserializer: D,
            ) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;

                Self::new(value)
                    .map_err(D::Error::custom)
            }
        }
    };
}

// ============================================================================
// HARDWARE IDENTIFIERS
// ============================================================================

/// Identity of a hardware interface.
///
/// This identifier is interpreted within its owning hardware device.
/// It does not describe an interface protocol or guarantee availability.
define_hardware_identity!(
    HardwareInterfaceId,
    "Opaque identity of a hardware interface.",
    "hardware interface identity"
);

/// Identity of a hardware resource.
///
/// Examples include provider-defined processing units, memory regions,
/// physical control resources and specialized computational resources.
///
/// The identifier does not represent an allocation or reservation.
define_hardware_identity!(
    HardwareResourceId,
    "Opaque identity of a hardware resource.",
    "hardware resource identity"
);

/// Identity of a hardware session.
///
/// A session identifier identifies one provider-defined interaction
/// session. It is not an authentication credential.
define_hardware_identity!(
    HardwareSessionId,
    "Identity of a hardware interaction session.",
    "hardware session identity"
);

/// Identity of a hardware operation instance.
///
/// This is distinct from ProviderOperationId, which identifies an
/// operation contract supported by a provider.
///
/// This identity identifies an individual submitted operation.
define_hardware_identity!(
    HardwareOperationId,
    "Identity of an individual hardware operation.",
    "hardware operation identity"
);

/// Identity of a hardware inventory.
///
/// An inventory identity identifies an inventory maintained by
/// an inventory owner. It does not imply that inventory contents
/// are permanently available.
define_hardware_identity!(
    HardwareInventoryId,
    "Identity of a hardware inventory.",
    "hardware inventory identity"
);

/// Identity of a physical hardware topology.
///
/// A topology identity identifies a provider- or ZHAL-managed
/// topology description, not a particular routing algorithm.
define_hardware_identity!(
    HardwareTopologyId,
    "Identity of a hardware topology.",
    "hardware topology identity"
);

/// Identity of a hardware observation snapshot.
///
/// A snapshot identity identifies an immutable observation record.
/// It does not establish that the observed hardware remains available.
define_hardware_identity!(
    HardwareSnapshotId,
    "Identity of a hardware observation snapshot.",
    "hardware snapshot identity"
);

// ============================================================================
// CANONICAL DEVICE ID REUSE
// ============================================================================

/// Canonical hardware device identifier.
///
/// This is deliberately an alias of the existing standard-library
/// DeviceId.
///
/// ZHAL must not create a competing physical device identifier.
///
/// Device identity is scoped by ProviderId when used in a
/// HardwareDeviceIdentity.
pub type HardwareDeviceId = DeviceId;

// ============================================================================
// PROVIDER-SCOPED DEVICE IDENTITY
// ============================================================================

/// Composite identity of a physical or provider-backed hardware device.
///
/// A device is identified by the combination of:
///
/// - Its canonical provider identity.
/// - Its canonical provider-scoped device identity.
///
/// Two different providers may legitimately expose the same DeviceId.
/// Their composite identities remain distinct.
///
/// This identity is independent of:
///
/// - Physical location.
/// - Operating system.
/// - Vendor.
/// - Hardware architecture.
/// - Computational domain.
/// - Device availability.
///
/// It is not a reservation or authorization token.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct HardwareDeviceIdentity {
    provider_id: ProviderId,
    device_id: HardwareDeviceId,
}

impl HardwareDeviceIdentity {
    /// Constructs a provider-scoped hardware device identity.
    ///
    /// Both components must already satisfy their canonical
    /// standard-library identity validation contracts.
    #[must_use]
    pub fn new(
        provider_id: ProviderId,
        device_id: HardwareDeviceId,
    ) -> Self {
        Self {
            provider_id,
            device_id,
        }
    }

    /// Returns the canonical provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the canonical device identity.
    #[must_use]
    pub fn device_id(&self) -> &HardwareDeviceId {
        &self.device_id
    }

    /// Consumes the composite identity and returns both components.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (ProviderId, HardwareDeviceId) {
        (self.provider_id, self.device_id)
    }
}

impl fmt::Display for HardwareDeviceIdentity {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}::{}",
            self.provider_id,
            self.device_id
        )
    }
}

impl Serialize for HardwareDeviceIdentity {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct(
            "HardwareDeviceIdentity",
            2,
        )?;

        state.serialize_field(
            "provider_id",
            self.provider_id.as_str(),
        )?;

        state.serialize_field(
            "device_id",
            self.device_id.as_str(),
        )?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for HardwareDeviceIdentity {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireIdentity {
            provider_id: String,
            device_id: String,
        }

        let wire = WireIdentity::deserialize(deserializer)?;

        let provider_id = ProviderId::new(wire.provider_id)
            .map_err(D::Error::custom)?;

        let device_id = DeviceId::new(wire.device_id)
            .map_err(D::Error::custom)?;

        Ok(Self::new(
            provider_id,
            device_id,
        ))
    }
}

// ============================================================================
// DEVICE-SCOPED INTERFACE IDENTITY
// ============================================================================

/// Composite identity of a hardware interface.
///
/// Interface identifiers are scoped to their owning device.
///
/// This prevents interfaces belonging to different devices from
/// accidentally being treated as identical.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct HardwareInterfaceIdentity {
    device: HardwareDeviceIdentity,
    interface_id: HardwareInterfaceId,
}

impl HardwareInterfaceIdentity {
    /// Creates a device-scoped interface identity.
    #[must_use]
    pub fn new(
        device: HardwareDeviceIdentity,
        interface_id: HardwareInterfaceId,
    ) -> Self {
        Self {
            device,
            interface_id,
        }
    }

    /// Returns the owning hardware device.
    #[must_use]
    pub fn device(&self) -> &HardwareDeviceIdentity {
        &self.device
    }

    /// Returns the interface identifier.
    #[must_use]
    pub fn interface_id(&self) -> &HardwareInterfaceId {
        &self.interface_id
    }

    /// Consumes the identity and returns its components.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (HardwareDeviceIdentity, HardwareInterfaceId) {
        (self.device, self.interface_id)
    }
}

// ============================================================================
// DEVICE-SCOPED RESOURCE IDENTITY
// ============================================================================

/// Composite identity of a hardware resource.
///
/// Resource identity is scoped to a hardware device.
///
/// A resource identity is descriptive. It does not establish:
///
/// - Resource ownership.
/// - Resource availability.
/// - Resource reservation.
/// - Resource allocation.
/// - Exclusive access.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct HardwareResourceIdentity {
    device: HardwareDeviceIdentity,
    resource_id: HardwareResourceId,
}

impl HardwareResourceIdentity {
    /// Creates a device-scoped resource identity.
    #[must_use]
    pub fn new(
        device: HardwareDeviceIdentity,
        resource_id: HardwareResourceId,
    ) -> Self {
        Self {
            device,
            resource_id,
        }
    }

    /// Returns the owning hardware device.
    #[must_use]
    pub fn device(&self) -> &HardwareDeviceIdentity {
        &self.device
    }

    /// Returns the resource identifier.
    #[must_use]
    pub fn resource_id(&self) -> &HardwareResourceId {
        &self.resource_id
    }

    /// Consumes the identity and returns its components.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (HardwareDeviceIdentity, HardwareResourceId) {
        (self.device, self.resource_id)
    }
}

// ============================================================================
// PROVIDER-SCOPED SESSION IDENTITY
// ============================================================================

/// Composite identity of a provider-backed hardware session.
///
/// Session identity is scoped to its provider.
///
/// The identifier does not contain credentials or authorization data.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct HardwareSessionIdentity {
    provider_id: ProviderId,
    session_id: HardwareSessionId,
}

impl HardwareSessionIdentity {
    /// Creates a provider-scoped session identity.
    #[must_use]
    pub fn new(
        provider_id: ProviderId,
        session_id: HardwareSessionId,
    ) -> Self {
        Self {
            provider_id,
            session_id,
        }
    }

    /// Returns the owning provider.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the session identifier.
    #[must_use]
    pub fn session_id(&self) -> &HardwareSessionId {
        &self.session_id
    }
}

// ============================================================================
// PROVIDER-SCOPED OPERATION IDENTITY
// ============================================================================

/// Composite identity of an individual provider-backed operation.
///
/// This identifies an operation instance rather than the provider's
/// declared operation contract.
///
/// ProviderOperationId remains the authority for operation contracts.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct HardwareOperationIdentity {
    provider_id: ProviderId,
    operation_id: HardwareOperationId,
}

impl HardwareOperationIdentity {
    /// Creates a provider-scoped hardware operation identity.
    #[must_use]
    pub fn new(
        provider_id: ProviderId,
        operation_id: HardwareOperationId,
    ) -> Self {
        Self {
            provider_id,
            operation_id,
        }
    }

    /// Returns the owning provider.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the operation instance identity.
    #[must_use]
    pub fn operation_id(&self) -> &HardwareOperationId {
        &self.operation_id
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    // ------------------------------------------------------------------------
    // Test helpers
    // ------------------------------------------------------------------------

    fn provider_id(value: &str) -> ProviderId {
        ProviderId::new(value)
            .expect("valid provider identity")
    }

    fn device_id(value: &str) -> HardwareDeviceId {
        DeviceId::new(value)
            .expect("valid device identity")
    }

    fn device_identity(
        provider: &str,
        device: &str,
    ) -> HardwareDeviceIdentity {
        HardwareDeviceIdentity::new(
            provider_id(provider),
            device_id(device),
        )
    }

    // ------------------------------------------------------------------------
    // Identifier validation
    // ------------------------------------------------------------------------

    #[test]
    fn identifiers_reject_empty_values() {
        assert_eq!(
            HardwareResourceId::new("").unwrap_err().kind(),
            IdentityErrorKind::Empty
        );
    }

    #[test]
    fn identifiers_reject_surrounding_whitespace() {
        assert_eq!(
            HardwareResourceId::new(" resource ")
                .unwrap_err()
                .kind(),
            IdentityErrorKind::SurroundingWhitespace
        );
    }

    #[test]
    fn identifiers_reject_internal_whitespace() {
        assert_eq!(
            HardwareInterfaceId::new("interface one")
                .unwrap_err()
                .kind(),
            IdentityErrorKind::Whitespace
        );
    }

    #[test]
    fn identifiers_reject_control_characters() {
        assert_eq!(
            HardwareOperationId::new("operation\none")
                .unwrap_err()
                .kind(),
            IdentityErrorKind::ControlCharacter
        );
    }

    #[test]
    fn identifiers_preserve_exact_spelling() {
        let id = HardwareResourceId::new(
            "Vendor/Quantum/Resource-A_01"
        )
        .expect("valid opaque identity");

        assert_eq!(
            id.as_str(),
            "Vendor/Quantum/Resource-A_01"
        );
    }

    #[test]
    fn identifiers_are_case_sensitive() {
        let first = HardwareResourceId::new("Resource-A")
            .unwrap();

        let second = HardwareResourceId::new("resource-a")
            .unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn identifiers_support_unicode_without_normalization() {
        let id = HardwareInterfaceId::new("接口/量子")
            .expect("valid opaque Unicode identity");

        assert_eq!(id.as_str(), "接口/量子");
    }

    #[test]
    fn identifiers_support_deep_namespaces() {
        let value = format!(
            "{}.{}",
            "domain".repeat(128),
            "resource".repeat(128)
        );

        let id = HardwareResourceId::new(value.clone())
            .expect("valid extended identifier");

        assert_eq!(id.as_str(), value);
    }

    // ------------------------------------------------------------------------
    // Ordering and hashing
    // ------------------------------------------------------------------------

    #[test]
    fn identifiers_have_deterministic_ordering() {
        let first = HardwareResourceId::new("resource.a")
            .unwrap();

        let second = HardwareResourceId::new("resource.b")
            .unwrap();

        assert!(first < second);
    }

    #[test]
    fn identifiers_support_hash_collections() {
        let mut identities = HashSet::new();

        identities.insert(
            HardwareInterfaceId::new("interface.a")
                .unwrap(),
        );

        identities.insert(
            HardwareInterfaceId::new("interface.b")
                .unwrap(),
        );

        assert_eq!(identities.len(), 2);
    }

    #[test]
    fn identical_identifiers_are_equal() {
        let first = HardwareSnapshotId::new("snapshot.1")
            .unwrap();

        let second = HardwareSnapshotId::new("snapshot.1")
            .unwrap();

        assert_eq!(first, second);
    }

    // ------------------------------------------------------------------------
    // Device identity
    // ------------------------------------------------------------------------

    #[test]
    fn same_device_id_under_different_providers_is_distinct() {
        let first = device_identity(
            "provider.alpha",
            "device.001",
        );

        let second = device_identity(
            "provider.beta",
            "device.001",
        );

        assert_ne!(first, second);
    }

    #[test]
    fn identical_provider_and_device_are_equal() {
        let first = device_identity(
            "provider.alpha",
            "device.001",
        );

        let second = device_identity(
            "provider.alpha",
            "device.001",
        );

        assert_eq!(first, second);
    }

    #[test]
    fn device_identity_exposes_canonical_components() {
        let identity = device_identity(
            "provider.alpha",
            "device.001",
        );

        assert_eq!(
            identity.provider_id().as_str(),
            "provider.alpha"
        );

        assert_eq!(
            identity.device_id().as_str(),
            "device.001"
        );
    }

    // ------------------------------------------------------------------------
    // Interface identity
    // ------------------------------------------------------------------------

    #[test]
    fn interface_identity_is_device_scoped() {
        let first = HardwareInterfaceIdentity::new(
            device_identity("provider.alpha", "device.001"),
            HardwareInterfaceId::new("interface.001")
                .unwrap(),
        );

        let second = HardwareInterfaceIdentity::new(
            device_identity("provider.alpha", "device.002"),
            HardwareInterfaceId::new("interface.001")
                .unwrap(),
        );

        assert_ne!(first, second);
    }

    // ------------------------------------------------------------------------
    // Resource identity
    // ------------------------------------------------------------------------

    #[test]
    fn resource_identity_is_device_scoped() {
        let first = HardwareResourceIdentity::new(
            device_identity("provider.alpha", "device.001"),
            HardwareResourceId::new("resource.001")
                .unwrap(),
        );

        let second = HardwareResourceIdentity::new(
            device_identity("provider.alpha", "device.002"),
            HardwareResourceId::new("resource.001")
                .unwrap(),
        );

        assert_ne!(first, second);
    }

    // ------------------------------------------------------------------------
    // Session identity
    // ------------------------------------------------------------------------

    #[test]
    fn session_identity_is_provider_scoped() {
        let first = HardwareSessionIdentity::new(
            provider_id("provider.alpha"),
            HardwareSessionId::new("session.001")
                .unwrap(),
        );

        let second = HardwareSessionIdentity::new(
            provider_id("provider.beta"),
            HardwareSessionId::new("session.001")
                .unwrap(),
        );

        assert_ne!(first, second);
    }

    // ------------------------------------------------------------------------
    // Operation identity
    // ------------------------------------------------------------------------

    #[test]
    fn operation_identity_is_provider_scoped() {
        let first = HardwareOperationIdentity::new(
            provider_id("provider.alpha"),
            HardwareOperationId::new("operation.001")
                .unwrap(),
        );

        let second = HardwareOperationIdentity::new(
            provider_id("provider.beta"),
            HardwareOperationId::new("operation.001")
                .unwrap(),
        );

        assert_ne!(first, second);
    }

    // ------------------------------------------------------------------------
    // Serialization
    // ------------------------------------------------------------------------

    #[test]
    fn individual_identity_serializes_as_string() {
        let id = HardwareResourceId::new("resource.001")
            .unwrap();

        let serialized = serde_json::to_string(&id)
            .unwrap();

        assert_eq!(
            serialized,
            "\"resource.001\""
        );
    }

    #[test]
    fn individual_identity_round_trips() {
        let original = HardwareResourceId::new(
            "provider/resource.001"
        )
        .unwrap();

        let serialized = serde_json::to_string(&original)
            .unwrap();

        let restored: HardwareResourceId =
            serde_json::from_str(&serialized)
                .unwrap();

        assert_eq!(original, restored);
    }

    #[test]
    fn invalid_serialized_identity_is_rejected() {
        let result: Result<HardwareResourceId, _> =
            serde_json::from_str("\"invalid identity\"");

        assert!(result.is_err());
    }

    #[test]
    fn empty_serialized_identity_is_rejected() {
        let result: Result<HardwareResourceId, _> =
            serde_json::from_str("\"\"");

        assert!(result.is_err());
    }

    #[test]
    fn device_identity_round_trips() {
        let original = device_identity(
            "provider.alpha",
            "device.001",
        );

        let serialized = serde_json::to_string(&original)
            .unwrap();

        let restored: HardwareDeviceIdentity =
            serde_json::from_str(&serialized)
                .unwrap();

        assert_eq!(original, restored);
    }

    #[test]
    fn invalid_provider_identity_in_serialized_device_is_rejected() {
        let result: Result<HardwareDeviceIdentity, _> =
            serde_json::from_str(
                r#"{
                    "provider_id": "",
                    "device_id": "device.001"
                }"#,
            );

        assert!(result.is_err());
    }

    #[test]
    fn invalid_device_identity_in_serialized_device_is_rejected() {
        let result: Result<HardwareDeviceIdentity, _> =
            serde_json::from_str(
                r#"{
                    "provider_id": "provider.alpha",
                    "device_id": "invalid device"
                }"#,
            );

        assert!(result.is_err());
    }

    #[test]
    fn interface_identity_round_trips() {
        let original = HardwareInterfaceIdentity::new(
            device_identity("provider.alpha", "device.001"),
            HardwareInterfaceId::new("interface.001")
                .unwrap(),
        );

        let serialized = serde_json::to_string(&original)
            .unwrap();

        let restored: HardwareInterfaceIdentity =
            serde_json::from_str(&serialized)
                .unwrap();

        assert_eq!(original, restored);
    }

    #[test]
    fn resource_identity_round_trips() {
        let original = HardwareResourceIdentity::new(
            device_identity("provider.alpha", "device.001"),
            HardwareResourceId::new("resource.001")
                .unwrap(),
        );

        let serialized = serde_json::to_string(&original)
            .unwrap();

        let restored: HardwareResourceIdentity =
            serde_json::from_str(&serialized)
                .unwrap();

        assert_eq!(original, restored);
    }

    // ------------------------------------------------------------------------
    // Identity ownership
    // ------------------------------------------------------------------------

    #[test]
    fn hardware_device_id_is_canonical_device_id() {
        let original: HardwareDeviceId =
            DeviceId::new("device.001").unwrap();

        assert_eq!(
            original.as_str(),
            "device.001"
        );
    }

    #[test]
    fn device_identity_display_is_deterministic() {
        let identity = device_identity(
            "provider.alpha",
            "device.001",
        );

        assert_eq!(
            identity.to_string(),
            "provider.alpha::device.001"
        );
    }
}
