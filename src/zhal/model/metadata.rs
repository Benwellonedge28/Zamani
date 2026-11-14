
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! Hardware Metadata Model
//!
//! Path: `src/zhal/model/metadata.rs`
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Defines the provider-independent descriptive metadata model
//! used by the Zamani Universal Hardware Abstraction Layer.
//!
//! Metadata describes hardware, hardware revisions, firmware,
//! implementation characteristics and provider-defined descriptive
//! properties.
//!
//! The model is independent of physical hardware technology.
//!
//! It supports metadata originating from:
//!
//! - Classical processors.
//! - Memory systems.
//! - Quantum processors.
//! - Quantum-control systems.
//! - FPGA devices.
//! - ASIC devices.
//! - GPUs.
//! - TPUs.
//! - NPUs.
//! - DSPs.
//! - Embedded devices.
//! - Sensors and actuators.
//! - Distributed hardware.
//! - Neuromorphic systems.
//! - Photonic systems.
//! - Analog computing systems.
//! - Future hardware technologies.
//!
//! # OWNERSHIP
//!
//! This module owns:
//!
//! - MetadataKey.
//! - MetadataValue.
//! - HardwareMetadata.
//! - HardwareVersionInfo.
//! - FirmwareDescriptor.
//! - Metadata validation.
//! - Metadata serialization.
//! - Metadata secret-key rejection.
//!
//! # DOES NOT OWN
//!
//! This module does not own:
//!
//! - Provider identity.
//! - Device identity.
//! - Device discovery.
//! - Hardware capability semantics.
//! - Capability evidence.
//! - Hardware resource allocation.
//! - Hardware topology.
//! - Hardware scheduling.
//! - Hardware routing.
//! - Hardware execution.
//! - Firmware installation.
//! - Firmware updates.
//! - Device authentication.
//! - Provider authorization.
//! - Source-language metadata semantics.
//! - AST or IR metadata.
//! - Hardware calibration.
//! - Quantum execution.
//! - HDL realization.
//!
//! These responsibilities remain with their canonical
//! repository authorities.
//!
//! # CANONICAL AUTHORITIES
//!
//! ZHAL identity:
//!     crate::zhal::identity
//!
//! ZHAL errors:
//!     crate::zhal::errors
//!
//! ZHAL version:
//!     crate::zhal::version
//!
//! Standard-library semantic version:
//!     crate::stdlib::version
//!
//! Provider identity:
//!     crate::stdlib::provider
//!
//! Capability descriptions:
//!     crate::stdlib::capability
//!
//! Generic device identity:
//!     crate::stdlib::io::devices
//!
//! Quantum hardware:
//!     crate::quantum::hardware
//!
//! HDL realization:
//!     crate::hdl::realization
//!
//! Runtime:
//!     crate::zur
//!
//! # METADATA MODEL
//!
//! Metadata is descriptive information.
//!
//! It is not proof of hardware capability, availability,
//! authenticity, authorization or resource reservation.
//!
//! Metadata must be interpreted together with its provenance,
//! observation time and owning hardware descriptor.
//!
//! # NAMESPACE
//!
//! Metadata keys use qualified namespaces.
//!
//! Examples:
//!
//!     org.zamani.hardware.architecture
//!     org.zamani.hardware.model
//!     org.zamani.hardware.revision
//!     org.zamani.firmware.version
//!     org.vendor.accelerator.compute_units
//!     org.vendor.quantum.technology
//!
//! Keys are case-sensitive.
//!
//! Namespaces are open-ended.
//!
//! # SECURITY
//!
//! Metadata is descriptive and MUST NOT contain:
//!
//! - Passwords.
//! - Private keys.
//! - API keys.
//! - Access tokens.
//! - Refresh tokens.
//! - Authentication credentials.
//! - Authorization headers.
//! - Session secrets.
//! - Cookies.
//! - Client secrets.
//!
//! Keys that explicitly identify these values are rejected.
//!
//! Nested metadata objects are validated recursively.
//!
//! Recognizable private-key and bearer-token material is rejected.
//!
//! Applications must still avoid supplying secrets under innocent
//! descriptive keys. No general string inspection algorithm can
//! reliably recognize every possible secret.
//!
//! Authentication material belongs to the security subsystem.
//!
//! # VERSIONING
//!
//! Hardware and firmware version strings are opaque provider-defined
//! descriptions.
//!
//! They are deliberately not forced into Semantic Versioning.
//!
//! For example, these are all valid descriptions:
//!
//!     1.2.3
//!     2026.04
//!     firmware-release-17
//!     vendor-build-abc
//!     revision-A3
//!
//! A consumer requiring semantic version comparison must explicitly
//! parse a compatible version through:
//!
//!     crate::stdlib::version::Version
//!
//! # DETERMINISM
//!
//! Metadata attributes are stored in BTreeMap.
//!
//! Iteration and serialization ordering are deterministic for
//! a fixed metadata value.
//!
//! # SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Metadata entry count.
//! - Identifier length.
//! - Namespace depth.
//! - Metadata string length.
//! - Hardware version length.
//! - Firmware description length.
//! - Nested metadata collection size.
//! - Number of hardware technologies.
//! - Number of providers.
//! - Number of devices.
//!
//! Actual growth is limited by available memory, address-space
//! representation and explicitly configured operational budgets.
//!
//! This module does not promise literal infinite memory.
//!
//! # GRAMMAR INTEGRATION
//!
//! Hardware source-level metadata intent is described through
//! the canonical hardware grammar.
//!
//! Resource metadata intent belongs to the resource grammar.
//!
//! This module represents runtime/provider-side hardware facts.
//!
//! It does not parse Zamani source code.
//!
//! # INTEGRATION
//!
//! Hardware device descriptors may contain HardwareMetadata.
//!
//! Hardware resource descriptors may contain HardwareMetadata.
//!
//! Provider descriptors may expose HardwareMetadata through
//! their provider-owned descriptive contracts.
//!
//! Hardware snapshots may retain immutable metadata snapshots.
//!
//! Each integration must preserve the metadata's provenance
//! and distinguish observations from verified evidence.

use core::fmt;
use core::str::FromStr;

use serde::de::Error as SerdeDeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use serde_json::Value;

use std::collections::BTreeMap;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable component identifier.
pub const COMPONENT: &str = "zhal.model.metadata";

/// Metadata model schema identifier.
///
/// This identifies the metadata representation contract.
/// It is independent of the Cargo package version and ZHAL API version.
pub const METADATA_SCHEMA_VERSION: &str = "1";

// ============================================================================
// ERROR CLASSIFICATION
// ============================================================================

/// Classification of metadata validation failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum MetadataErrorKind {
    /// Metadata key is empty.
    EmptyKey,

    /// Metadata key is not qualified with a namespace.
    UnqualifiedKey,

    /// Metadata key contains invalid syntax.
    InvalidKey,

    /// Metadata key identifies sensitive information.
    SensitiveKey,

    /// Metadata contains recognizable secret material.
    SensitiveValue,

    /// Metadata contains invalid descriptive information.
    InvalidValue,

    /// Metadata version information is invalid.
    InvalidVersion,

    /// Metadata contains an invalid firmware description.
    InvalidFirmware,

    /// Metadata contains an invalid hardware revision.
    InvalidRevision,

    /// Metadata contains invalid nested attributes.
    InvalidNestedMetadata,
}

impl fmt::Display for MetadataErrorKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let description = match self {
            Self::EmptyKey => "metadata key is empty",
            Self::UnqualifiedKey => {
                "metadata key must contain a namespace"
            }
            Self::InvalidKey => {
                "metadata key contains invalid syntax"
            }
            Self::SensitiveKey => {
                "metadata key identifies sensitive information"
            }
            Self::SensitiveValue => {
                "metadata value contains recognizable secret material"
            }
            Self::InvalidValue => {
                "metadata value is invalid"
            }
            Self::InvalidVersion => {
                "hardware version description is invalid"
            }
            Self::InvalidFirmware => {
                "firmware description is invalid"
            }
            Self::InvalidRevision => {
                "hardware revision description is invalid"
            }
            Self::InvalidNestedMetadata => {
                "nested metadata is invalid"
            }
        };

        formatter.write_str(description)
    }
}

/// Structured metadata validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetadataError {
    kind: MetadataErrorKind,
    field: String,
    message: String,
}

impl MetadataError {
    fn new(
        kind: MetadataErrorKind,
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
    pub const fn kind(&self) -> MetadataErrorKind {
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

impl fmt::Display for MetadataError {
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

impl std::error::Error for MetadataError {}

/// Result type for metadata operations.
pub type MetadataResult<T> = Result<T, MetadataError>;

// ============================================================================
// COMMON VALIDATION
// ============================================================================

fn validate_description(
    value: &str,
    field: &str,
    kind: MetadataErrorKind,
) -> MetadataResult<()> {
    if value.is_empty() {
        return Err(MetadataError::new(
            kind,
            field,
            "value must not be empty",
        ));
    }

    if value.trim() != value {
        return Err(MetadataError::new(
            kind,
            field,
            "value must not contain surrounding whitespace",
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(MetadataError::new(
            kind,
            field,
            "value must not contain control characters",
        ));
    }

    Ok(())
}

// ============================================================================
// SECRET DETECTION
// ============================================================================

/// Determines whether a metadata key identifies sensitive information.
///
/// Matching uses complete key segments rather than arbitrary substring
/// matching.
///
/// Consequently, a descriptive key such as `tokenizer.version` is not
/// rejected merely because it contains the letters "token".
fn is_sensitive_key(value: &str) -> bool {
    value
        .split(['.', '_', '-'])
        .any(|segment| {
            matches!(
                segment.to_ascii_lowercase().as_str(),
                "password"
                    | "passwd"
                    | "secret"
                    | "secrets"
                    | "credential"
                    | "credentials"
                    | "apikey"
                    | "accesskey"
                    | "privatekey"
                    | "token"
                    | "accesstoken"
                    | "refreshtoken"
                    | "clientsecret"
                    | "authorization"
                    | "cookie"
                    | "cookies"
            )
        })
}

/// Rejects recognizable secret material without retaining or displaying
/// the potentially sensitive value.
fn contains_obvious_secret_material(value: &str) -> bool {
    let trimmed = value.trim();

    let lower = trimmed.to_ascii_lowercase();

    let pem_private_key =
        lower.contains("-----begin ")
            && lower.contains("private key-----");

    let bearer_token =
        lower.starts_with("bearer ");

    let basic_credentials =
        lower.starts_with("basic ");

    let authorization_header =
        lower.starts_with("authorization:");

    pem_private_key
        || bearer_token
        || basic_credentials
        || authorization_header
}

// ============================================================================
// METADATA KEY
// ============================================================================

/// A validated, namespaced hardware metadata key.
///
/// Metadata keys are open-world identifiers.
///
/// Their namespaces prevent unrelated providers and hardware domains
/// from accidentally assigning conflicting meanings to the same key.
///
/// Examples:
///
/// - `org.zamani.hardware.model`
/// - `org.zamani.hardware.revision`
/// - `org.zamani.firmware.version`
/// - `org.vendor.quantum.technology`
/// - `org.vendor.accelerator.architecture`
///
/// Keys are case-sensitive.
///
/// Namespace segments permit:
///
/// - ASCII lowercase letters.
/// - ASCII uppercase letters.
/// - ASCII digits.
/// - Underscores.
/// - Hyphens.
///
/// Segments must not be empty.
///
/// No arbitrary maximum key length or namespace depth is imposed.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MetadataKey(String);

impl MetadataKey {
    /// Constructs a validated metadata key from a qualified string.
    ///
    /// At least one namespace separator is required.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the key is invalid or sensitive.
    pub fn new(
        value: impl Into<String>,
    ) -> MetadataResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(MetadataError::new(
                MetadataErrorKind::EmptyKey,
                "key",
                "metadata key must not be empty",
            ));
        }

        if value.trim() != value {
            return Err(MetadataError::new(
                MetadataErrorKind::InvalidKey,
                &value,
                "metadata key must not contain surrounding whitespace",
            ));
        }

        if !value.contains('.') {
            return Err(MetadataError::new(
                MetadataErrorKind::UnqualifiedKey,
                &value,
                "metadata key must be namespace-qualified",
            ));
        }

        for segment in value.split('.') {
            if segment.is_empty()
                || !segment.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric()
                        || byte == b'_'
                        || byte == b'-'
                })
            {
                return Err(MetadataError::new(
                    MetadataErrorKind::InvalidKey,
                    &value,
                    "metadata key contains an invalid namespace segment",
                ));
            }
        }

        if is_sensitive_key(&value) {
            return Err(MetadataError::new(
                MetadataErrorKind::SensitiveKey,
                &value,
                "sensitive information must use the security subsystem",
            ));
        }

        Ok(Self(value))
    }

    /// Constructs a metadata key from namespace and local name.
    ///
    /// # Errors
    ///
    /// Returns a metadata validation error when either component
    /// or the resulting qualified identifier is invalid.
    pub fn qualified(
        namespace: &str,
        name: &str,
    ) -> MetadataResult<Self> {
        Self::new(format!("{namespace}.{name}"))
    }

    /// Returns the complete qualified key.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the namespace portion.
    #[must_use]
    pub fn namespace(&self) -> &str {
        self.0
            .rsplit_once('.')
            .map(|(namespace, _)| namespace)
            .unwrap_or("")
    }

    /// Returns the local key name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.0
            .rsplit('.')
            .next()
            .unwrap_or(self.0.as_str())
    }
}

impl fmt::Display for MetadataKey {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for MetadataKey {
    type Err = MetadataError;

    fn from_str(value: &str) -> MetadataResult<Self> {
        Self::new(value)
    }
}

impl TryFrom<&str> for MetadataKey {
    type Error = MetadataError;

    fn try_from(value: &str) -> MetadataResult<Self> {
        Self::new(value)
    }
}

impl TryFrom<String> for MetadataKey {
    type Error = MetadataError;

    fn try_from(value: String) -> MetadataResult<Self> {
        Self::new(value)
    }
}

impl Serialize for MetadataKey {
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

impl<'de> Deserialize<'de> for MetadataKey {
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

// ============================================================================
// METADATA VALUE
// ============================================================================

/// Extensible typed metadata value.
///
/// The canonical representation is JSON-compatible.
///
/// Supported values include:
///
/// - Strings.
/// - Boolean values.
/// - Numbers.
/// - Arrays.
/// - Nested objects.
/// - Null values.
///
/// Nested objects are recursively validated before insertion.
///
/// Metadata values must remain descriptive. They must not contain
/// credentials or authentication material.
///
/// JSON values are used to avoid imposing a closed hardware-specific
/// metadata schema.
pub type MetadataValue = Value;

/// Validates a metadata value recursively.
///
/// Object keys are checked as sensitive metadata keys.
///
/// String values are checked for recognizable credential material.
///
/// No arbitrary collection-size or nesting-count limit is introduced.
fn validate_metadata_value(
    value: &Value,
    field: &str,
) -> MetadataResult<()> {
    match value {
        Value::String(value) => {
            if contains_obvious_secret_material(value) {
                return Err(MetadataError::new(
                    MetadataErrorKind::SensitiveValue,
                    field,
                    "recognizable secret material is not permitted",
                ));
            }
        }

        Value::Array(values) => {
            for (index, value) in values.iter().enumerate() {
                validate_metadata_value(
                    value,
                    &format!("{field}[{index}]"),
                )
                .map_err(|error| {
                    MetadataError::new(
                        MetadataErrorKind::InvalidNestedMetadata,
                        error.field,
                        error.message,
                    )
                })?;
            }
        }

        Value::Object(values) => {
            for (key, value) in values {
                if is_sensitive_key(key) {
                    return Err(MetadataError::new(
                        MetadataErrorKind::SensitiveKey,
                        key,
                        "nested metadata contains a sensitive key",
                    ));
                }

                validate_metadata_value(
                    value,
                    &format!("{field}.{key}"),
                )
                .map_err(|error| {
                    MetadataError::new(
                        MetadataErrorKind::InvalidNestedMetadata,
                        error.field,
                        error.message,
                    )
                })?;
            }
        }

        Value::Null
        | Value::Bool(_)
        | Value::Number(_) => {}
    }

    Ok(())
}

// ============================================================================
// FIRMWARE DESCRIPTION
// ============================================================================

/// Describes firmware associated with a hardware implementation.
///
/// Firmware version strings are opaque provider-defined values.
///
/// They are not assumed to follow Semantic Versioning.
///
/// A firmware descriptor is descriptive information, not proof that
/// the firmware is authentic, installed, current or secure.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FirmwareDescriptor {
    name: Option<String>,
    version: String,
    vendor: Option<String>,
    build: Option<String>,
}

impl FirmwareDescriptor {
    /// Constructs a firmware descriptor.
    ///
    /// # Errors
    ///
    /// Returns an error when a required or optional field is invalid.
    pub fn new(
        version: impl Into<String>,
    ) -> MetadataResult<Self> {
        let version = version.into();

        validate_description(
            &version,
            "firmware.version",
            MetadataErrorKind::InvalidFirmware,
        )?;

        Ok(Self {
            name: None,
            version,
            vendor: None,
            build: None,
        })
    }

    /// Sets the firmware name.
    ///
    /// # Errors
    ///
    /// Returns an error if the name is invalid.
    pub fn with_name(
        mut self,
        name: impl Into<String>,
    ) -> MetadataResult<Self> {
        let name = name.into();

        validate_description(
            &name,
            "firmware.name",
            MetadataErrorKind::InvalidFirmware,
        )?;

        self.name = Some(name);

        Ok(self)
    }

    /// Sets the firmware vendor description.
    ///
    /// # Errors
    ///
    /// Returns an error if the description is invalid.
    pub fn with_vendor(
        mut self,
        vendor: impl Into<String>,
    ) -> MetadataResult<Self> {
        let vendor = vendor.into();

        validate_description(
            &vendor,
            "firmware.vendor",
            MetadataErrorKind::InvalidFirmware,
        )?;

        self.vendor = Some(vendor);

        Ok(self)
    }

    /// Sets the firmware build description.
    ///
    /// # Errors
    ///
    /// Returns an error if the description is invalid.
    pub fn with_build(
        mut self,
        build: impl Into<String>,
    ) -> MetadataResult<Self> {
        let build = build.into();

        validate_description(
            &build,
            "firmware.build",
            MetadataErrorKind::InvalidFirmware,
        )?;

        self.build = Some(build);

        Ok(self)
    }

    /// Returns the firmware name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the opaque firmware version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Returns the firmware vendor description.
    #[must_use]
    pub fn vendor(&self) -> Option<&str> {
        self.vendor.as_deref()
    }

    /// Returns the firmware build description.
    #[must_use]
    pub fn build(&self) -> Option<&str> {
        self.build.as_deref()
    }

    fn validate(&self) -> MetadataResult<()> {
        validate_description(
            &self.version,
            "firmware.version",
            MetadataErrorKind::InvalidFirmware,
        )?;

        if let Some(name) = &self.name {
            validate_description(
                name,
                "firmware.name",
                MetadataErrorKind::InvalidFirmware,
            )?;
        }

        if let Some(vendor) = &self.vendor {
            validate_description(
                vendor,
                "firmware.vendor",
                MetadataErrorKind::InvalidFirmware,
            )?;
        }

        if let Some(build) = &self.build {
            validate_description(
                build,
                "firmware.build",
                MetadataErrorKind::InvalidFirmware,
            )?;
        }

        Ok(())
    }
}

impl<'de> Deserialize<'de> for FirmwareDescriptor {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            name: Option<String>,
            version: String,
            vendor: Option<String>,
            build: Option<String>,
        }

        let wire = Wire::deserialize(deserializer)?;

        let value = Self {
            name: wire.name,
            version: wire.version,
            vendor: wire.vendor,
            build: wire.build,
        };

        value.validate()
            .map_err(D::Error::custom)?;

        Ok(value)
    }
}

// ============================================================================
// HARDWARE VERSION INFORMATION
// ============================================================================

/// Describes the version and revision of a hardware implementation.
///
/// Hardware version information is independent of software package
/// versioning and ZHAL API versioning.
///
/// Hardware revisions and version strings are opaque descriptive
/// values supplied by the relevant hardware provider.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HardwareVersionInfo {
    product: Option<String>,
    version: Option<String>,
    revision: Option<String>,
    firmware: Option<FirmwareDescriptor>,
}

impl HardwareVersionInfo {
    /// Creates an empty hardware version description.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            product: None,
            version: None,
            revision: None,
            firmware: None,
        }
    }

    /// Sets the hardware product description.
    ///
    /// # Errors
    ///
    /// Returns an error if the description is invalid.
    pub fn with_product(
        mut self,
        product: impl Into<String>,
    ) -> MetadataResult<Self> {
        let product = product.into();

        validate_description(
            &product,
            "hardware.product",
            MetadataErrorKind::InvalidVersion,
        )?;

        self.product = Some(product);

        Ok(self)
    }

    /// Sets the hardware version description.
    ///
    /// # Errors
    ///
    /// Returns an error if the description is invalid.
    pub fn with_version(
        mut self,
        version: impl Into<String>,
    ) -> MetadataResult<Self> {
        let version = version.into();

        validate_description(
            &version,
            "hardware.version",
            MetadataErrorKind::InvalidVersion,
        )?;

        self.version = Some(version);

        Ok(self)
    }

    /// Sets the hardware revision description.
    ///
    /// # Errors
    ///
    /// Returns an error if the revision is invalid.
    pub fn with_revision(
        mut self,
        revision: impl Into<String>,
    ) -> MetadataResult<Self> {
        let revision = revision.into();

        validate_description(
            &revision,
            "hardware.revision",
            MetadataErrorKind::InvalidRevision,
        )?;

        self.revision = Some(revision);

        Ok(self)
    }

    /// Associates a firmware description.
    #[must_use]
    pub fn with_firmware(
        mut self,
        firmware: FirmwareDescriptor,
    ) -> Self {
        self.firmware = Some(firmware);

        self
    }

    /// Returns the product description.
    #[must_use]
    pub fn product(&self) -> Option<&str> {
        self.product.as_deref()
    }

    /// Returns the hardware version description.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns the hardware revision.
    #[must_use]
    pub fn revision(&self) -> Option<&str> {
        self.revision.as_deref()
    }

    /// Returns the firmware description.
    #[must_use]
    pub const fn firmware(&self) -> Option<&FirmwareDescriptor> {
        self.firmware.as_ref()
    }

    fn validate(&self) -> MetadataResult<()> {
        if let Some(product) = &self.product {
            validate_description(
                product,
                "hardware.product",
                MetadataErrorKind::InvalidVersion,
            )?;
        }

        if let Some(version) = &self.version {
            validate_description(
                version,
                "hardware.version",
                MetadataErrorKind::InvalidVersion,
            )?;
        }

        if let Some(revision) = &self.revision {
            validate_description(
                revision,
                "hardware.revision",
                MetadataErrorKind::InvalidRevision,
            )?;
        }

        if let Some(firmware) = &self.firmware {
            firmware.validate()?;
        }

        Ok(())
    }
}

impl Default for HardwareVersionInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HardwareVersionInfo {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            product: Option<String>,
            version: Option<String>,
            revision: Option<String>,
            firmware: Option<FirmwareDescriptor>,
        }

        let wire = Wire::deserialize(deserializer)?;

        let value = Self {
            product: wire.product,
            version: wire.version,
            revision: wire.revision,
            firmware: wire.firmware,
        };

        value.validate()
            .map_err(D::Error::custom)?;

        Ok(value)
    }
}

// ============================================================================
// HARDWARE METADATA
// ============================================================================

/// Complete descriptive metadata associated with hardware.
///
/// Metadata is provider-independent and technology-neutral.
///
/// It contains optional hardware version information and an extensible,
/// deterministically ordered collection of namespaced descriptive
/// attributes.
///
/// Metadata does not establish capability, availability, authenticity,
/// authorization or successful execution.
///
/// # Examples
///
/// ```
/// use zamani_compiler::zhal::model::metadata::{
///     HardwareMetadata,
///     HardwareVersionInfo,
///     MetadataKey,
/// };
///
/// use serde_json::json;
///
/// let version = HardwareVersionInfo::new()
///     .with_product("Example accelerator")?
///     .with_version("revision-12")?
///     .with_revision("B3")?;
///
/// let mut metadata = HardwareMetadata::new();
///
/// metadata.set_hardware_version(version)?;
///
/// metadata.insert(
///     MetadataKey::new("org.example.hardware.architecture")?,
///     json!("heterogeneous-compute"),
/// )?;
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct HardwareMetadata {
    hardware_version: Option<HardwareVersionInfo>,

    attributes: BTreeMap<MetadataKey, MetadataValue>,
}

impl HardwareMetadata {
    /// Creates an empty metadata collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            hardware_version: None,
            attributes: BTreeMap::new(),
        }
    }

    /// Returns the metadata schema version.
    #[must_use]
    pub const fn schema_version(&self) -> &'static str {
        METADATA_SCHEMA_VERSION
    }

    /// Sets the hardware version information.
    ///
    /// # Errors
    ///
    /// Returns an error if any supplied description is invalid.
    pub fn set_hardware_version(
        &mut self,
        version: HardwareVersionInfo,
    ) -> MetadataResult<()> {
        version.validate()?;

        self.hardware_version = Some(version);

        Ok(())
    }

    /// Replaces the hardware version information.
    ///
    /// # Errors
    ///
    /// Returns an error if any supplied description is invalid.
    pub fn with_hardware_version(
        mut self,
        version: HardwareVersionInfo,
    ) -> MetadataResult<Self> {
        self.set_hardware_version(version)?;

        Ok(self)
    }

    /// Returns the hardware version information.
    #[must_use]
    pub const fn hardware_version(
        &self,
    ) -> Option<&HardwareVersionInfo> {
        self.hardware_version.as_ref()
    }

    /// Inserts a validated metadata attribute.
    ///
    /// Existing values with the same key are replaced.
    ///
    /// Returns the previous value, if any.
    ///
    /// # Errors
    ///
    /// Returns an error when the value contains invalid or sensitive
    /// information.
    pub fn insert(
        &mut self,
        key: MetadataKey,
        value: MetadataValue,
    ) -> MetadataResult<Option<MetadataValue>> {
        validate_metadata_value(
            &value,
            key.as_str(),
        )?;

        Ok(self.attributes.insert(key, value))
    }

    /// Inserts an attribute using a qualified string key.
    ///
    /// # Errors
    ///
    /// Returns an error if the key or value is invalid.
    pub fn insert_qualified(
        &mut self,
        key: &str,
        value: MetadataValue,
    ) -> MetadataResult<Option<MetadataValue>> {
        self.insert(
            MetadataKey::new(key)?,
            value,
        )
    }

    /// Returns an attribute by its validated key.
    #[must_use]
    pub fn get(
        &self,
        key: &MetadataKey,
    ) -> Option<&MetadataValue> {
        self.attributes.get(key)
    }

    /// Returns an attribute using its qualified string key.
    ///
    /// Invalid keys return `None`.
    #[must_use]
    pub fn get_qualified(
        &self,
        key: &str,
    ) -> Option<&MetadataValue> {
        let key = MetadataKey::new(key).ok()?;

        self.get(&key)
    }

    /// Removes an attribute.
    ///
    /// Returns the removed value when present.
    #[must_use]
    pub fn remove(
        &mut self,
        key: &MetadataKey,
    ) -> Option<MetadataValue> {
        self.attributes.remove(key)
    }

    /// Returns whether an attribute exists.
    #[must_use]
    pub fn contains_key(
        &self,
        key: &MetadataKey,
    ) -> bool {
        self.attributes.contains_key(key)
    }

    /// Returns the number of metadata attributes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.attributes.len()
    }

    /// Returns whether the metadata contains no attributes or version
    /// information.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
            && self.hardware_version.is_none()
    }

    /// Returns attributes in deterministic key order.
    #[must_use]
    pub fn attributes(
        &self,
    ) -> &BTreeMap<MetadataKey, MetadataValue> {
        &self.attributes
    }

    /// Returns an iterator over deterministically ordered attributes.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&MetadataKey, &MetadataValue)> {
        self.attributes.iter()
    }

    /// Validates the complete metadata collection.
    ///
    /// This method is useful at provider and serialization boundaries.
    ///
    /// # Errors
    ///
    /// Returns the first encountered validation error.
    pub fn validate(&self) -> MetadataResult<()> {
        if let Some(version) = &self.hardware_version {
            version.validate()?;
        }

        for (key, value) in &self.attributes {
            MetadataKey::new(key.as_str())?;

            validate_metadata_value(
                value,
                key.as_str(),
            )?;
        }

        Ok(())
    }

    /// Merges another metadata collection into this collection.
    ///
    /// Values supplied by `other` replace existing values with the
    /// same key.
    ///
    /// Hardware version information from `other` replaces existing
    /// hardware version information when present.
    ///
    /// Validation occurs before mutation so an invalid input does not
    /// partially modify this collection.
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied metadata is invalid.
    pub fn merge(
        &mut self,
        other: &HardwareMetadata,
    ) -> MetadataResult<()> {
        other.validate()?;

        let mut merged = self.attributes.clone();

        for (key, value) in &other.attributes {
            merged.insert(
                key.clone(),
                value.clone(),
            );
        }

        self.attributes = merged;

        if let Some(version) = &other.hardware_version {
            self.hardware_version = Some(version.clone());
        }

        Ok(())
    }

    /// Returns an independent validated metadata snapshot.
    ///
    /// This is a descriptive data snapshot. It does not imply that
    /// the underlying hardware remains unchanged.
    ///
    /// # Errors
    ///
    /// Returns an error if the metadata is invalid.
    pub fn snapshot(&self) -> MetadataResult<Self> {
        self.validate()?;

        Ok(self.clone())
    }
}

impl<'de> Deserialize<'de> for HardwareMetadata {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            hardware_version: Option<HardwareVersionInfo>,

            #[serde(default)]
            attributes: BTreeMap<MetadataKey, MetadataValue>,
        }

        let wire = Wire::deserialize(deserializer)?;

        let metadata = Self {
            hardware_version: wire.hardware_version,
            attributes: wire.attributes,
        };

        metadata.validate()
            .map_err(D::Error::custom)?;

        Ok(metadata)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    // ------------------------------------------------------------------------
    // IDENTIFIERS
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_namespaced_metadata_keys() {
        let key = MetadataKey::new(
            "org.zamani.hardware.architecture",
        )
        .unwrap();

        assert_eq!(
            key.namespace(),
            "org.zamani.hardware"
        );

        assert_eq!(
            key.name(),
            "architecture"
        );
    }

    #[test]
    fn accepts_provider_defined_namespaces() {
        let key = MetadataKey::new(
            "org.vendor.quantum.technology",
        )
        .unwrap();

        assert_eq!(
            key.as_str(),
            "org.vendor.quantum.technology"
        );
    }

    #[test]
    fn rejects_empty_metadata_keys() {
        assert!(
            MetadataKey::new("").is_err()
        );
    }

    #[test]
    fn rejects_unqualified_metadata_keys() {
        assert!(
            MetadataKey::new("architecture").is_err()
        );
    }

    #[test]
    fn rejects_empty_namespace_segments() {
        assert!(
            MetadataKey::new("org..hardware.model").is_err()
        );
    }

    #[test]
    fn rejects_invalid_namespace_characters() {
        assert!(
            MetadataKey::new(
                "org.vendor.hardware model"
            )
            .is_err()
        );
    }

    #[test]
    fn accepts_long_metadata_namespaces() {
        let value = format!(
            "org.{}.hardware.description",
            "provider".repeat(128)
        );

        assert!(
            MetadataKey::new(value).is_ok()
        );
    }

    // ------------------------------------------------------------------------
    // SECRET PROTECTION
    // ------------------------------------------------------------------------

    #[test]
    fn rejects_password_metadata_keys() {
        assert!(
            MetadataKey::new(
                "org.vendor.device.password"
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_private_key_metadata_keys() {
        assert!(
            MetadataKey::new(
                "org.vendor.device.private_key"
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_access_token_metadata_keys() {
        assert!(
            MetadataKey::new(
                "org.vendor.access_token"
            )
            .is_err()
        );
    }

    #[test]
    fn permits_descriptive_tokenizer_metadata() {
        assert!(
            MetadataKey::new(
                "org.vendor.tokenizer.version"
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_bearer_token_values() {
        let mut metadata = HardwareMetadata::new();

        let result = metadata.insert_qualified(
            "org.vendor.hardware.description",
            json!("Bearer example-secret"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_private_key_material() {
        let mut metadata = HardwareMetadata::new();

        let result = metadata.insert_qualified(
            "org.vendor.hardware.description",
            json!(
                "-----BEGIN PRIVATE KEY-----\nexample"
            ),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_nested_sensitive_keys() {
        let mut metadata = HardwareMetadata::new();

        let result = metadata.insert_qualified(
            "org.vendor.hardware.details",
            json!({
                "architecture": "example",
                "credentials": "sensitive"
            }),
        );

        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // METADATA VALUES
    // ------------------------------------------------------------------------

    #[test]
    fn stores_typed_descriptive_values() {
        let mut metadata = HardwareMetadata::new();

        metadata.insert_qualified(
            "org.vendor.hardware.enabled",
            json!(true),
        )
        .unwrap();

        metadata.insert_qualified(
            "org.vendor.hardware.description",
            json!("heterogeneous processor"),
        )
        .unwrap();

        metadata.insert_qualified(
            "org.vendor.hardware.properties",
            json!({
                "architecture": "example",
                "generation": 3
            }),
        )
        .unwrap();

        assert_eq!(metadata.len(), 3);
    }

    #[test]
    fn retrieves_metadata_values() {
        let mut metadata = HardwareMetadata::new();

        metadata.insert_qualified(
            "org.vendor.hardware.architecture",
            json!("example"),
        )
        .unwrap();

        assert_eq!(
            metadata.get_qualified(
                "org.vendor.hardware.architecture"
            ),
            Some(&json!("example"))
        );
    }

    #[test]
    fn replaces_existing_metadata_values() {
        let mut metadata = HardwareMetadata::new();

        metadata.insert_qualified(
            "org.vendor.hardware.generation",
            json!(1),
        )
        .unwrap();

        let previous = metadata.insert_qualified(
            "org.vendor.hardware.generation",
            json!(2),
        )
        .unwrap();

        assert_eq!(
            previous,
            Some(json!(1))
        );

        assert_eq!(
            metadata.get_qualified(
                "org.vendor.hardware.generation"
            ),
            Some(&json!(2))
        );
    }

    #[test]
    fn removes_metadata_values() {
        let mut metadata = HardwareMetadata::new();

        let key = MetadataKey::new(
            "org.vendor.hardware.model",
        )
        .unwrap();

        metadata.insert(
            key.clone(),
            json!("example"),
        )
        .unwrap();

        assert!(
            metadata.remove(&key).is_some()
        );

        assert!(metadata.is_empty());
    }

    // ------------------------------------------------------------------------
    // HARDWARE VERSION
    // ------------------------------------------------------------------------

    #[test]
    fn stores_hardware_version_information() {
        let version = HardwareVersionInfo::new()
            .with_product("Example processor")
            .unwrap()
            .with_version("generation-12")
            .unwrap()
            .with_revision("B3")
            .unwrap();

        let mut metadata = HardwareMetadata::new();

        metadata.set_hardware_version(version)
            .unwrap();

        let stored = metadata.hardware_version()
            .unwrap();

        assert_eq!(
            stored.product(),
            Some("Example processor")
        );

        assert_eq!(
            stored.version(),
            Some("generation-12")
        );

        assert_eq!(
            stored.revision(),
            Some("B3")
        );
    }

    #[test]
    fn stores_firmware_descriptions() {
        let firmware = FirmwareDescriptor::new(
            "2026.04",
        )
        .unwrap()
        .with_name("Device firmware")
        .unwrap()
        .with_vendor("Example vendor")
        .unwrap()
        .with_build("build-42")
        .unwrap();

        let version = HardwareVersionInfo::new()
            .with_firmware(firmware);

        let mut metadata = HardwareMetadata::new();

        metadata.set_hardware_version(version)
            .unwrap();

        let firmware = metadata
            .hardware_version()
            .unwrap()
            .firmware()
            .unwrap();

        assert_eq!(
            firmware.version(),
            "2026.04"
        );

        assert_eq!(
            firmware.build(),
            Some("build-42")
        );
    }

    #[test]
    fn permits_provider_specific_firmware_versions() {
        let firmware = FirmwareDescriptor::new(
            "vendor-build-abc",
        )
        .unwrap();

        assert_eq!(
            firmware.version(),
            "vendor-build-abc"
        );
    }

    #[test]
    fn rejects_invalid_hardware_revisions() {
        let result = HardwareVersionInfo::new()
            .with_revision("invalid\nrevision");

        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // MERGING AND SNAPSHOTS
    // ------------------------------------------------------------------------

    #[test]
    fn merges_metadata_deterministically() {
        let mut first = HardwareMetadata::new();

        first.insert_qualified(
            "org.vendor.hardware.generation",
            json!(1),
        )
        .unwrap();

        let mut second = HardwareMetadata::new();

        second.insert_qualified(
            "org.vendor.hardware.architecture",
            json!("example"),
        )
        .unwrap();

        first.merge(&second).unwrap();

        assert_eq!(
            first.len(),
            2
        );
    }

    #[test]
    fn metadata_snapshot_is_independent() {
        let mut original = HardwareMetadata::new();

        original.insert_qualified(
            "org.vendor.hardware.generation",
            json!(1),
        )
        .unwrap();

        let snapshot = original.snapshot().unwrap();

        original.insert_qualified(
            "org.vendor.hardware.generation",
            json!(2),
        )
        .unwrap();

        assert_eq!(
            snapshot.get_qualified(
                "org.vendor.hardware.generation"
            ),
            Some(&json!(1))
        );
    }

    // ------------------------------------------------------------------------
    // SERIALIZATION
    // ------------------------------------------------------------------------

    #[test]
    fn metadata_round_trips_through_json() {
        let mut original = HardwareMetadata::new();

        original.insert_qualified(
            "org.vendor.hardware.architecture",
            json!("example"),
        )
        .unwrap();

        let serialized = serde_json::to_string(&original)
            .unwrap();

        let restored: HardwareMetadata =
            serde_json::from_str(&serialized)
                .unwrap();

        assert_eq!(
            original,
            restored
        );
    }

    #[test]
    fn serialization_preserves_hardware_versions() {
        let version = HardwareVersionInfo::new()
            .with_product("Example")
            .unwrap()
            .with_revision("A1")
            .unwrap();

        let original = HardwareMetadata::new()
            .with_hardware_version(version)
            .unwrap();

        let serialized = serde_json::to_string(&original)
            .unwrap();

        let restored: HardwareMetadata =
            serde_json::from_str(&serialized)
                .unwrap();

        assert_eq!(
            original,
            restored
        );
    }

    #[test]
    fn deserialization_rejects_sensitive_keys() {
        let serialized = r#"
        {
            "hardware_version": null,
            "attributes": {
                "org.vendor.hardware.password": "example"
            }
        }
        "#;

        let restored: Result<HardwareMetadata, _> =
            serde_json::from_str(serialized);

        assert!(restored.is_err());
    }

    #[test]
    fn deserialization_rejects_nested_sensitive_keys() {
        let serialized = r#"
        {
            "hardware_version": null,
            "attributes": {
                "org.vendor.hardware.details": {
                    "private_key": "example"
                }
            }
        }
        "#;

        let restored: Result<HardwareMetadata, _> =
            serde_json::from_str(serialized);

        assert!(restored.is_err());
    }

    #[test]
    fn metadata_has_no_hardware_category_ceiling() {
        let mut metadata = HardwareMetadata::new();

        for index in 0..1024 {
            let key = format!(
                "org.example.hardware.property_{index}"
            );

            metadata.insert_qualified(
                &key,
                json!(index),
            )
            .unwrap();
        }

        assert_eq!(
            metadata.len(),
            1024
        );
    }

    #[test]
    fn metadata_iteration_is_deterministic() {
        let mut metadata = HardwareMetadata::new();

        metadata.insert_qualified(
            "org.vendor.hardware.z",
            json!(3),
        )
        .unwrap();

        metadata.insert_qualified(
            "org.vendor.hardware.a",
            json!(1),
        )
        .unwrap();

        let keys: Vec<&str> = metadata
            .iter()
            .map(|(key, _)| key.as_str())
            .collect();

        assert_eq!(
            keys,
            vec![
                "org.vendor.hardware.a",
                "org.vendor.hardware.z"
            ]
        );
    }
}
