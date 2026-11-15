
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer — Resource Model
//!
//! Path: src/zhal/model/resource.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.1
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Defines the provider-independent representation of physical
//! computational resources and their observations.
//!
//! Supports classical, quantum, HDL, accelerator, embedded,
//! distributed, hybrid and future computational technologies.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Physical resource descriptions.
//! - Extensible resource-kind identities.
//! - Extensible resource-unit identities.
//! - Exact non-negative resource quantities.
//! - Resource ownership references.
//! - Resource capacity observations.
//! - Resource availability observations.
//! - Resource observation provenance.
//! - Resource observation validation.
//!
//! This module does NOT own:
//!
//! - Source-language resource declarations.
//! - AST resource representations.
//! - Semantic resource requirements.
//! - Symbolic expression evaluation.
//! - Capability discovery.
//! - Provider registration.
//! - Hardware discovery algorithms.
//! - Resource feasibility.
//! - Resource allocation.
//! - Resource scheduling.
//! - Resource placement.
//! - Resource routing.
//! - Resource reservation.
//! - Resource lease lifecycle.
//! - Memory allocation.
//! - Quantum execution.
//! - HDL realization.
//! - Distributed execution.
//!
//! # Canonical repository authorities
//!
//! Hardware resource identity:
//!     crate::zhal::identity::HardwareResourceIdentity
//!
//! Provider identity:
//!     crate::stdlib::provider::ProviderId
//!
//! Device identity:
//!     crate::stdlib::io::devices::DeviceId
//!
//! Capability identity:
//!     crate::stdlib::capability::CapabilityId
//!
//! Source-level resource semantics:
//!     crate::frontend::semantic::resources
//!
//! Runtime resource management:
//!     crate::zur::resources
//!
//! Quantum hardware:
//!     crate::quantum::hardware
//!
//! HDL realization:
//!     crate::hdl::realization
//!
//! Distributed execution:
//!     crate::distributed
//!
//! # Fundamental invariants
//!
//! Resource description != resource observation.
//! Resource observation != resource availability guarantee.
//! Resource availability != resource reservation.
//! Resource reservation != resource allocation.
//! Resource allocation != resource execution.
//!
//! ZHAL describes physical resources. It never claims to have
//! acquired or reserved them.
//!
//! # Scalability
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Resource quantities.
//! - Resource identifier lengths.
//! - Resource categories.
//! - Resource units.
//! - Resource descriptions.
//! - Resource observations.
//! - Hardware devices.
//! - Computational domains.
//!
//! Quantities use exact decimal strings rather than u64, u128,
//! usize or floating-point representations.
//!
//! Actual storage remains constrained by available memory,
//! address-space representation and configured operational budgets.
//!
//! Literal infinite physical resources are not promised.
//!
//! # Symbolic quantities
//!
//! Symbolic and source-derived quantities remain owned by:
//!
//!     crate::frontend::semantic::resources
//!
//! ZHAL represents physical quantities reported by providers.
//! Dynamic changes are represented by new observations.
//!
//! # Concurrency
//!
//! Resource descriptions and observations are immutable values.
//! This module maintains no global mutable state.
//!
//! # Security
//!
//! Descriptive metadata and ownership references are not credentials.
//! Never store passwords, private keys, access tokens, session secrets
//! or authorization headers in these structures.

use core::cmp::Ordering;
use core::fmt;
use std::time::SystemTime;

use serde::de::Error as SerdeDeError;
use serde::{Deserialize, Deserializer, Serialize};

use crate::stdlib::provider::ProviderId;
use crate::zhal::identity::HardwareResourceIdentity;

// ============================================================================
// MODULE CONTRACT
// ============================================================================

/// Stable component identifier.
pub const COMPONENT: &str = "zhal.resource";

/// Resource-model schema version.
///
/// This is not the language, package, provider API or firmware version.
pub const RESOURCE_SCHEMA_VERSION: &str = "1";

// ============================================================================
// ERRORS
// ============================================================================

/// Classification of resource-model validation errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ResourceErrorKind {
    /// An identifier is empty.
    EmptyIdentifier,

    /// An identifier contains whitespace.
    InvalidWhitespace,

    /// An identifier contains control characters.
    ControlCharacter,

    /// A resource quantity is malformed.
    InvalidQuantity,

    /// A resource unit is invalid.
    InvalidUnit,

    /// A resource kind is invalid.
    InvalidKind,

    /// An observed quantity is inconsistent.
    InconsistentObservation,

    /// Provider provenance does not match resource ownership.
    InvalidProvider,

    /// A resource ownership reference is invalid.
    InvalidOwnership,

    /// A descriptive value is invalid.
    InvalidDescription,
}

impl fmt::Display for ResourceErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::EmptyIdentifier => "identifier must not be empty",
            Self::InvalidWhitespace => "identifier contains whitespace",
            Self::ControlCharacter => "identifier contains control characters",
            Self::InvalidQuantity => "invalid resource quantity",
            Self::InvalidUnit => "invalid resource unit",
            Self::InvalidKind => "invalid resource kind",
            Self::InconsistentObservation => "inconsistent resource observation",
            Self::InvalidProvider => "invalid provider provenance",
            Self::InvalidOwnership => "invalid resource ownership",
            Self::InvalidDescription => "invalid resource description",
        };

        f.write_str(message)
    }
}

/// Structured resource-model error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceError {
    kind: ResourceErrorKind,
    component: &'static str,
    message: String,
}

impl ResourceError {
    fn new(
        kind: ResourceErrorKind,
        component: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            component,
            message: message.into(),
        }
    }

    /// Returns the error classification.
    #[must_use]
    pub const fn kind(&self) -> ResourceErrorKind {
        self.kind
    }

    /// Returns the affected component.
    #[must_use]
    pub const fn component(&self) -> &'static str {
        self.component
    }

    /// Returns the diagnostic explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}: {}",
            self.component,
            self.kind,
            self.message
        )
    }
}

impl std::error::Error for ResourceError {}

/// Result type for resource-model operations.
pub type ResourceResult<T> = Result<T, ResourceError>;

// ============================================================================
// COMMON VALIDATION
// ============================================================================

fn resource_error(
    kind: ResourceErrorKind,
    component: &'static str,
    message: impl Into<String>,
) -> ResourceError {
    ResourceError::new(kind, component, message)
}

fn validate_identifier(
    value: &str,
    component: &'static str,
) -> ResourceResult<()> {
    if value.is_empty() {
        return Err(resource_error(
            ResourceErrorKind::EmptyIdentifier,
            component,
            "identifier cannot be empty",
        ));
    }

    if value.trim() != value {
        return Err(resource_error(
            ResourceErrorKind::InvalidWhitespace,
            component,
            "leading or trailing whitespace is not permitted",
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(resource_error(
            ResourceErrorKind::ControlCharacter,
            component,
            "control characters are not permitted",
        ));
    }

    if value.chars().any(char::is_whitespace) {
        return Err(resource_error(
            ResourceErrorKind::InvalidWhitespace,
            component,
            "whitespace is not permitted",
        ));
    }

    Ok(())
}

fn validate_description(
    value: &str,
    component: &'static str,
) -> ResourceResult<()> {
    if value.trim().is_empty()
        || value.chars().any(char::is_control)
    {
        return Err(resource_error(
            ResourceErrorKind::InvalidDescription,
            component,
            "description must contain meaningful text and no control characters",
        ));
    }

    Ok(())
}

// ============================================================================
// OPEN-WORLD RESOURCE KIND
// ============================================================================

/// Extensible identity of a physical resource category.
///
/// This is intentionally not a closed enum.
///
/// Examples:
///
/// - classical.processor
/// - classical.memory
/// - quantum.qubit
/// - quantum.control_channel
/// - hdl.logic_element
/// - hdl.memory_block
/// - accelerator.compute_unit
/// - network.bandwidth
/// - embedded.sensor
/// - photonic.optical_channel
/// - neuromorphic.neuron
///
/// New technologies must not require modifying this type.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct ResourceKindId(String);

impl ResourceKindId {
    /// Constructs a validated resource-kind identity.
    pub fn new(value: impl Into<String>) -> ResourceResult<Self> {
        let value = value.into();

        validate_identifier(&value, "resource kind")?;

        Ok(Self(value))
    }

    /// Returns the exact resource-kind identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for ResourceKindId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(D::Error::custom)
    }
}

impl fmt::Display for ResourceKindId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// OPEN-WORLD RESOURCE UNIT
// ============================================================================

/// Extensible physical resource-unit identity.
///
/// This type identifies a unit but does not define conversion factors.
///
/// Unit dimensionality and conversion belong to the appropriate
/// canonical unit-registry or domain authority.
///
/// Examples:
///
/// - information:byte
/// - information:bit
/// - frequency:hertz
/// - time:second
/// - compute:operation
/// - quantum:physical_qubit
/// - network:byte_per_second
/// - power:watt
/// - storage:sector
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ResourceUnitId {
    namespace: String,
    name: String,
}

impl ResourceUnitId {
    /// Constructs a validated resource unit.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> ResourceResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identifier(&namespace, "unit namespace")?;
        validate_identifier(&name, "unit name")?;

        if namespace.contains(':') || name.contains(':') {
            return Err(resource_error(
                ResourceErrorKind::InvalidUnit,
                "unit",
                "unit components cannot contain the canonical separator",
            ));
        }

        Ok(Self {
            namespace,
            name,
        })
    }

    /// Returns the unit namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the unit name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the canonical textual identity.
    #[must_use]
    pub fn canonical_name(&self) -> String {
        format!("{}:{}", self.namespace, self.name)
    }
}

impl<'de> Deserialize<'de> for ResourceUnitId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireUnit {
            namespace: String,
            name: String,
        }

        let wire = WireUnit::deserialize(deserializer)?;

        Self::new(wire.namespace, wire.name)
            .map_err(D::Error::custom)
    }
}

impl fmt::Display for ResourceUnitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.name)
    }
}

// ============================================================================
// EXACT RESOURCE QUANTITY
// ============================================================================

/// Exact, non-negative decimal resource quantity.
///
/// The canonical representation is an owned decimal string.
///
/// It is not restricted to u64, u128, usize or floating-point range.
///
/// Accepted syntax:
///
///     DIGITS
///     DIGITS "." DIGITS
///
/// Scientific notation and negative quantities are not accepted.
///
/// Examples:
///
///     0
///     1
///     1024
///     184467440737095516160000000000000000000
///     0.125
///
/// A quantity is meaningful only in conjunction with its resource unit.
///
/// This representation does not promise infinite storage: the quantity
/// remains bounded by available memory and address-space limits.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ResourceAmount {
    canonical: String,
}

impl ResourceAmount {
    /// Parses and canonicalizes a non-negative decimal quantity.
    pub fn new(value: impl AsRef<str>) -> ResourceResult<Self> {
        let value = value.as_ref();

        if value.is_empty() {
            return Err(resource_error(
                ResourceErrorKind::InvalidQuantity,
                "quantity",
                "quantity cannot be empty",
            ));
        }

        let mut parts = value.split('.');

        let integer = parts.next().unwrap_or_default();
        let fraction = parts.next();

        if parts.next().is_some()
            || integer.is_empty()
            || !integer.bytes().all(|b| b.is_ascii_digit())
        {
            return Err(resource_error(
                ResourceErrorKind::InvalidQuantity,
                "quantity",
                "expected unsigned decimal digits with at most one decimal point",
            ));
        }

        if let Some(fraction) = fraction {
            if fraction.is_empty()
                || !fraction.bytes().all(|b| b.is_ascii_digit())
            {
                return Err(resource_error(
                    ResourceErrorKind::InvalidQuantity,
                    "quantity",
                    "fractional digits must be present and numeric",
                ));
            }
        }

        let integer = integer.trim_start_matches('0');
        let integer = if integer.is_empty() {
            "0"
        } else {
            integer
        };

        let canonical = match fraction {
            Some(fraction) => {
                let fraction = fraction.trim_end_matches('0');

                if fraction.is_empty() {
                    integer.to_owned()
                } else {
                    let mut result =
                        String::with_capacity(
                            integer.len() + 1 + fraction.len(),
                        );

                    result.push_str(integer);
                    result.push('.');
                    result.push_str(fraction);

                    result
                }
            }

            None => integer.to_owned(),
        };

        Ok(Self { canonical })
    }

    /// Returns the canonical decimal representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.canonical
    }

    /// Returns whether the quantity is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.canonical == "0"
    }

    /// Compares quantities without machine-integer conversion.
    #[must_use]
    pub fn numeric_cmp(&self, other: &Self) -> Ordering {
        let (left_integer, left_fraction) =
            split_decimal(&self.canonical);

        let (right_integer, right_fraction) =
            split_decimal(&other.canonical);

        left_integer
            .len()
            .cmp(&right_integer.len())
            .then_with(|| left_integer.cmp(right_integer))
            .then_with(|| {
                compare_fraction(left_fraction, right_fraction)
            })
    }

    /// Adds two exact non-negative quantities.
    ///
    /// No machine-sized integer conversion is performed.
    pub fn checked_add(&self, other: &Self) -> ResourceResult<Self> {
        let (left_integer, left_fraction) =
            split_decimal(&self.canonical);

        let (right_integer, right_fraction) =
            split_decimal(&other.canonical);

        let fraction_width =
            left_fraction.len().max(right_fraction.len());

        let mut left = left_integer.as_bytes().to_vec();
        let mut right = right_integer.as_bytes().to_vec();

        left.extend_from_slice(
            left_fraction.as_bytes(),
        );

        right.extend_from_slice(
            right_fraction.as_bytes(),
        );

        let left_scale = left_fraction.len();
        let right_scale = right_fraction.len();

        if left_scale < fraction_width {
            left.extend(
                std::iter::repeat_n(
                    b'0',
                    fraction_width - left_scale,
                ),
            );
        }

        if right_scale < fraction_width {
            right.extend(
                std::iter::repeat_n(
                    b'0',
                    fraction_width - right_scale,
                ),
            );
        }

        let width = left.len().max(right.len());

        let mut result = Vec::with_capacity(width + 1);
        let mut carry = 0u8;

        for index in (0..width).rev() {
            let a = left
                .get(index + left.len().saturating_sub(width))
                .copied()
                .unwrap_or(b'0') - b'0';

            let b = right
                .get(index + right.len().saturating_sub(width))
                .copied()
                .unwrap_or(b'0') - b'0';

            let sum = a + b + carry;

            result.push(b'0' + sum % 10);
            carry = sum / 10;
        }

        if carry != 0 {
            result.push(b'0' + carry);
        }

        result.reverse();

        let mut digits = String::from_utf8(result)
            .map_err(|_| resource_error(
                ResourceErrorKind::InvalidQuantity,
                "quantity arithmetic",
                "decimal addition produced invalid digits",
            ))?;

        if fraction_width > 0 {
            let integer_width = digits.len().saturating_sub(fraction_width);

            if integer_width == 0 {
                let mut padded = String::with_capacity(
                    digits.len() + 1,
                );

                padded.push('0');
                padded.extend(
                    std::iter::repeat_n(
                        '0',
                        fraction_width - digits.len(),
                    ),
                );
                padded.push_str(&digits);

                digits = padded;
            }

            let split = digits.len() - fraction_width;
            digits.insert(split, '.');
        }

        Self::new(digits)
    }
}

fn split_decimal(value: &str) -> (&str, &str) {
    match value.split_once('.') {
        Some((integer, fraction)) => (integer, fraction),
        None => (value, ""),
    }
}

fn compare_fraction(left: &str, right: &str) -> Ordering {
    let width = left.len().max(right.len());

    for index in 0..width {
        let a = left.as_bytes().get(index).copied().unwrap_or(b'0');
        let b = right.as_bytes().get(index).copied().unwrap_or(b'0');

        let ordering = a.cmp(&b);

        if ordering != Ordering::Equal {
            return ordering;
        }
    }

    Ordering::Equal
}

impl Ord for ResourceAmount {
    fn cmp(&self, other: &Self) -> Ordering {
        self.numeric_cmp(other)
    }
}

impl PartialOrd for ResourceAmount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for ResourceAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.canonical)
    }
}

impl<'de> Deserialize<'de> for ResourceAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(D::Error::custom)
    }
}

// ============================================================================
// RESOURCE AVAILABILITY
// ============================================================================

/// Observed physical resource availability.
///
/// This is evidence about a resource at an observation time.
///
/// It is not a reservation or availability guarantee.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub enum ResourceAvailability {
    /// Evidence establishes that the resource is available.
    Available,

    /// Evidence establishes that the resource is unavailable.
    Unavailable,

    /// Availability depends on described conditions.
    Conditional {
        /// Conditions governing availability.
        conditions: String,
    },

    /// Availability cannot currently be established.
    Unknown,

    /// The observation is known to be stale.
    Stale,
}

impl ResourceAvailability {
    /// Creates conditional availability with validated conditions.
    pub fn conditional(
        conditions: impl Into<String>,
    ) -> ResourceResult<Self> {
        let conditions = conditions.into();

        validate_description(
            &conditions,
            "availability conditions",
        )?;

        Ok(Self::Conditional { conditions })
    }

    /// Returns whether availability is explicitly established.
    #[must_use]
    pub fn is_available(&self) -> bool {
        matches!(self, Self::Available)
    }
}

impl<'de> Deserialize<'de> for ResourceAvailability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        enum WireAvailability {
            Available,
            Unavailable,
            Conditional {
                conditions: String,
            },
            Unknown,
            Stale,
        }

        match WireAvailability::deserialize(deserializer)? {
            WireAvailability::Available => Ok(Self::Available),
            WireAvailability::Unavailable => Ok(Self::Unavailable),
            WireAvailability::Unknown => Ok(Self::Unknown),
            WireAvailability::Stale => Ok(Self::Stale),
            WireAvailability::Conditional { conditions } => {
                Self::conditional(conditions).map_err(D::Error::custom)
            }
        }
    }
}

// ============================================================================
// RESOURCE OWNERSHIP REFERENCE
// ============================================================================

/// Descriptive reference to an entity associated with a resource.
///
/// This does not establish allocation, reservation or authorization.
///
/// The referenced entity may be a device, provider, deployment,
/// execution, session or another extensible entity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ResourceOwnerReference {
    scope: String,
    identity: String,
}

impl ResourceOwnerReference {
    /// Constructs a validated ownership reference.
    pub fn new(
        scope: impl Into<String>,
        identity: impl Into<String>,
    ) -> ResourceResult<Self> {
        let scope = scope.into();
        let identity = identity.into();

        validate_identifier(&scope, "owner scope")?;
        validate_identifier(&identity, "owner identity")?;

        Ok(Self {
            scope,
            identity,
        })
    }

    /// Returns the ownership scope.
    #[must_use]
    pub fn scope(&self) -> &str {
        &self.scope
    }

    /// Returns the owner identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }
}

impl<'de> Deserialize<'de> for ResourceOwnerReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireOwner {
            scope: String,
            identity: String,
        }

        let wire = WireOwner::deserialize(deserializer)?;

        Self::new(wire.scope, wire.identity)
            .map_err(D::Error::custom)
    }
}

// ============================================================================
// HARDWARE RESOURCE DESCRIPTION
// ============================================================================

/// Provider-independent description of a physical resource.
///
/// A description establishes identity and classification.
///
/// It does not establish current capacity or availability.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HardwareResource {
    identity: HardwareResourceIdentity,
    kind: ResourceKindId,
    unit: ResourceUnitId,
    owner: Option<ResourceOwnerReference>,
    description: Option<String>,
}

impl HardwareResource {
    /// Creates a hardware resource description.
    ///
    /// The identity, resource kind and unit must already be validated
    /// by their respective canonical constructors.
    #[must_use]
    pub fn new(
        identity: HardwareResourceIdentity,
        kind: ResourceKindId,
        unit: ResourceUnitId,
    ) -> Self {
        Self {
            identity,
            kind,
            unit,
            owner: None,
            description: None,
        }
    }

    /// Attaches a descriptive ownership reference.
    #[must_use]
    pub fn with_owner(
        mut self,
        owner: ResourceOwnerReference,
    ) -> Self {
        self.owner = Some(owner);
        self
    }

    /// Attaches a validated human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> ResourceResult<Self> {
        let description = description.into();

        validate_description(
            &description,
            "resource description",
        )?;

        self.description = Some(description);

        Ok(self)
    }

    /// Returns the canonical hardware resource identity.
    #[must_use]
    pub fn identity(&self) -> &HardwareResourceIdentity {
        &self.identity
    }

    /// Returns the resource kind.
    #[must_use]
    pub fn kind(&self) -> &ResourceKindId {
        &self.kind
    }

    /// Returns the resource unit.
    #[must_use]
    pub fn unit(&self) -> &ResourceUnitId {
        &self.unit
    }

    /// Returns the optional descriptive ownership reference.
    #[must_use]
    pub fn owner(&self) -> Option<&ResourceOwnerReference> {
        self.owner.as_ref()
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl<'de> Deserialize<'de> for HardwareResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireResource {
            identity: HardwareResourceIdentity,
            kind: ResourceKindId,
            unit: ResourceUnitId,
            owner: Option<ResourceOwnerReference>,
            description: Option<String>,
        }

        let wire = WireResource::deserialize(deserializer)?;

        let mut resource = Self::new(
            wire.identity,
            wire.kind,
            wire.unit,
        );

        if let Some(owner) = wire.owner {
            resource = resource.with_owner(owner);
        }

        if let Some(description) = wire.description {
            resource = resource
                .with_description(description)
                .map_err(D::Error::custom)?;
        }

        Ok(resource)
    }
}

// ============================================================================
// RESOURCE CAPACITY
// ============================================================================

/// Observed quantities associated with a physical resource.
///
/// Every quantity uses the unit declared by its HardwareResource.
///
/// Unknown values remain unknown.
///
/// Available and in-use quantities describe distinct portions of
/// the resource capacity.
///
/// The model permits unaccounted capacity, including reserved,
/// unavailable or otherwise unreported capacity.
///
/// It does not perform reservation or allocation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ResourceCapacity {
    total: Option<ResourceAmount>,
    available: Option<ResourceAmount>,
    in_use: Option<ResourceAmount>,
}

impl ResourceCapacity {
    /// Creates and validates a capacity observation.
    ///
    /// When total, available and in-use are known:
    ///
    ///     available + in_use <= total
    ///
    /// Unaccounted capacity is permitted.
    pub fn new(
        total: Option<ResourceAmount>,
        available: Option<ResourceAmount>,
        in_use: Option<ResourceAmount>,
    ) -> ResourceResult<Self> {
        if let Some(total) = &total {
            if let Some(available) = &available {
                if available > total {
                    return Err(resource_error(
                        ResourceErrorKind::InconsistentObservation,
                        "capacity",
                        "available quantity exceeds total capacity",
                    ));
                }
            }

            if let Some(in_use) = &in_use {
                if in_use > total {
                    return Err(resource_error(
                        ResourceErrorKind::InconsistentObservation,
                        "capacity",
                        "in-use quantity exceeds total capacity",
                    ));
                }
            }

            if let (Some(available), Some(in_use)) =
                (&available, &in_use)
            {
                let combined = available.checked_add(in_use)?;

                if &combined > total {
                    return Err(resource_error(
                        ResourceErrorKind::InconsistentObservation,
                        "capacity",
                        "available plus in-use quantity exceeds total capacity",
                    ));
                }
            }
        }

        Ok(Self {
            total,
            available,
            in_use,
        })
    }

    /// Returns the observed total capacity.
    #[must_use]
    pub fn total(&self) -> Option<&ResourceAmount> {
        self.total.as_ref()
    }

    /// Returns the observed available quantity.
    #[must_use]
    pub fn available(&self) -> Option<&ResourceAmount> {
        self.available.as_ref()
    }

    /// Returns the observed in-use quantity.
    #[must_use]
    pub fn in_use(&self) -> Option<&ResourceAmount> {
        self.in_use.as_ref()
    }

    /// Returns whether total capacity is known.
    #[must_use]
    pub fn has_known_total(&self) -> bool {
        self.total.is_some()
    }
}

impl<'de> Deserialize<'de> for ResourceCapacity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireCapacity {
            total: Option<ResourceAmount>,
            available: Option<ResourceAmount>,
            in_use: Option<ResourceAmount>,
        }

        let wire = WireCapacity::deserialize(deserializer)?;

        Self::new(
            wire.total,
            wire.available,
            wire.in_use,
        )
        .map_err(D::Error::custom)
    }
}

// ============================================================================
// RESOURCE OBSERVATION PROVENANCE
// ============================================================================

/// Provenance of a physical resource observation.
///
/// Provenance identifies the provider that supplied the observation.
///
/// It does not independently establish provider trust or authorization.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ResourceProvenance {
    provider_id: ProviderId,
    source_revision: Option<String>,
}

impl ResourceProvenance {
    /// Creates resource observation provenance.
    #[must_use]
    pub fn new(provider_id: ProviderId) -> Self {
        Self {
            provider_id,
            source_revision: None,
        }
    }

    /// Attaches a validated provider-defined source revision.
    pub fn with_source_revision(
        mut self,
        revision: impl Into<String>,
    ) -> ResourceResult<Self> {
        let revision = revision.into();

        validate_identifier(
            &revision,
            "source revision",
        )?;

        self.source_revision = Some(revision);

        Ok(self)
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the optional provider revision.
    #[must_use]
    pub fn source_revision(&self) -> Option<&str> {
        self.source_revision.as_deref()
    }
}

impl<'de> Deserialize<'de> for ResourceProvenance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireProvenance {
            provider_id: ProviderId,
            source_revision: Option<String>,
        }

        let wire = WireProvenance::deserialize(deserializer)?;

        let mut provenance = Self::new(wire.provider_id);

        if let Some(revision) = wire.source_revision {
            provenance = provenance
                .with_source_revision(revision)
                .map_err(D::Error::custom)?;
        }

        Ok(provenance)
    }
}

// ============================================================================
// COMPLETE RESOURCE OBSERVATION
// ============================================================================

/// Immutable observation of a physical hardware resource.
///
/// Combines:
///
/// - Resource description.
/// - Capacity information.
/// - Availability information.
/// - Observation provenance.
/// - Observation time.
///
/// This is a snapshot of provider-reported facts.
///
/// It is not a resource reservation, allocation or execution result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceObservation {
    resource: HardwareResource,
    capacity: ResourceCapacity,
    availability: ResourceAvailability,
    provenance: ResourceProvenance,
    observed_at: SystemTime,
}

impl ResourceObservation {
    /// Constructs a validated resource observation.
    ///
    /// The provider identity in provenance must match the provider
    /// that owns the physical resource identity.
    pub fn new(
        resource: HardwareResource,
        capacity: ResourceCapacity,
        availability: ResourceAvailability,
        provenance: ResourceProvenance,
        observed_at: SystemTime,
    ) -> ResourceResult<Self> {
        let resource_provider =
            resource.identity().device().provider_id();

        if resource_provider != provenance.provider_id() {
            return Err(resource_error(
                ResourceErrorKind::InvalidProvider,
                "provenance",
                "observation provider does not own the resource identity",
            ));
        }

        if let ResourceAvailability::Conditional {
            conditions,
        } = &availability
        {
            validate_description(
                conditions,
                "availability conditions",
            )?;
        }

        Ok(Self {
            resource,
            capacity,
            availability,
            provenance,
            observed_at,
        })
    }

    /// Returns the physical resource description.
    #[must_use]
    pub fn resource(&self) -> &HardwareResource {
        &self.resource
    }

    /// Returns the capacity observation.
    #[must_use]
    pub fn capacity(&self) -> &ResourceCapacity {
        &self.capacity
    }

    /// Returns the availability observation.
    #[must_use]
    pub fn availability(&self) -> &ResourceAvailability {
        &self.availability
    }

    /// Returns the observation provenance.
    #[must_use]
    pub fn provenance(&self) -> &ResourceProvenance {
        &self.provenance
    }

    /// Returns the observation timestamp.
    #[must_use]
    pub const fn observed_at(&self) -> SystemTime {
        self.observed_at
    }

    /// Returns whether the resource was explicitly observed as available.
    ///
    /// This does not guarantee continuing availability.
    ///
    /// It does not establish reservation or allocation.
    #[must_use]
    pub fn is_observed_available(&self) -> bool {
        self.availability.is_available()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::io::devices::DeviceId;
    use crate::zhal::identity::{
        HardwareDeviceIdentity,
        HardwareResourceId,
    };

    fn provider() -> ProviderId {
        ProviderId::new("test.provider")
            .expect("valid provider")
    }

    fn identity() -> HardwareResourceIdentity {
        HardwareResourceIdentity::new(
            HardwareDeviceIdentity::new(
                provider(),
                DeviceId::new("device.001")
                    .expect("valid device"),
            ),
            HardwareResourceId::new("resource.001")
                .expect("valid resource"),
        )
    }

    fn amount(value: &str) -> ResourceAmount {
        ResourceAmount::new(value)
            .expect("valid quantity")
    }

    fn resource() -> HardwareResource {
        HardwareResource::new(
            identity(),
            ResourceKindId::new("classical.memory")
                .expect("valid kind"),
            ResourceUnitId::new("information", "byte")
                .expect("valid unit"),
        )
    }

    // ------------------------------------------------------------------------
    // Exact quantities
    // ------------------------------------------------------------------------

    #[test]
    fn quantity_canonicalizes_integer_zeroes() {
        assert_eq!(amount("0000123").as_str(), "123");
    }

    #[test]
    fn quantity_canonicalizes_fractional_zeroes() {
        assert_eq!(amount("00012.3400").as_str(), "12.34");
    }

    #[test]
    fn quantity_canonicalizes_zero() {
        assert_eq!(amount("0000.000").as_str(), "0");
    }

    #[test]
    fn quantity_supports_values_beyond_u128() {
        let value = amount(
            "340282366920938463463374607431768211456000000000000000000"
        );

        assert!(!value.is_zero());
    }

    #[test]
    fn quantity_compares_exactly() {
        assert!(amount("100000000000000000000")
            > amount("99999999999999999999"));

        assert!(amount("1.21") > amount("1.2"));

        assert_eq!(
            amount("1.200"),
            amount("1.2")
        );
    }

    #[test]
    fn quantity_adds_exactly() {
        assert_eq!(
            amount("999.99")
                .checked_add(&amount("0.01"))
                .unwrap()
                .as_str(),
            "1000"
        );

        assert_eq!(
            amount("0.09")
                .checked_add(&amount("0.02"))
                .unwrap()
                .as_str(),
            "0.11"
        );

        assert_eq!(
            amount("999999999999999999999")
                .checked_add(&amount("1"))
                .unwrap()
                .as_str(),
            "1000000000000000000000"
        );
    }

    #[test]
    fn quantity_rejects_negative_values() {
        assert!(ResourceAmount::new("-1").is_err());
    }

    #[test]
    fn quantity_rejects_scientific_notation() {
        assert!(ResourceAmount::new("1e100").is_err());
    }

    #[test]
    fn quantity_rejects_multiple_decimal_points() {
        assert!(ResourceAmount::new("1.2.3").is_err());
    }

    #[test]
    fn quantity_rejects_empty_fraction() {
        assert!(ResourceAmount::new("1.").is_err());
    }

    // ------------------------------------------------------------------------
    // Extensible identifiers
    // ------------------------------------------------------------------------

    #[test]
    fn resource_kinds_are_open_ended() {
        let kind = ResourceKindId::new(
            "future.photonic.optical_resource"
        ).unwrap();

        assert_eq!(
            kind.as_str(),
            "future.photonic.optical_resource"
        );
    }

    #[test]
    fn resource_units_are_open_ended() {
        let unit = ResourceUnitId::new(
            "future.domain",
            "specialized_unit",
        ).unwrap();

        assert_eq!(
            unit.canonical_name(),
            "future.domain:specialized_unit"
        );
    }

    #[test]
    fn resource_units_reject_ambiguous_separators() {
        assert!(
            ResourceUnitId::new(
                "information:extra",
                "byte",
            ).is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Capacity consistency
    // ------------------------------------------------------------------------

    #[test]
    fn capacity_accepts_consistent_observations() {
        let capacity = ResourceCapacity::new(
            Some(amount("1000")),
            Some(amount("300")),
            Some(amount("500")),
        ).unwrap();

        assert_eq!(
            capacity.total().unwrap().as_str(),
            "1000"
        );
    }

    #[test]
    fn capacity_rejects_available_above_total() {
        assert!(
            ResourceCapacity::new(
                Some(amount("100")),
                Some(amount("101")),
                None,
            ).is_err()
        );
    }

    #[test]
    fn capacity_rejects_combined_available_and_usage_above_total() {
        assert!(
            ResourceCapacity::new(
                Some(amount("100")),
                Some(amount("80")),
                Some(amount("80")),
            ).is_err()
        );
    }

    #[test]
    fn capacity_accepts_unaccounted_resources() {
        assert!(
            ResourceCapacity::new(
                Some(amount("100")),
                Some(amount("20")),
                Some(amount("30")),
            ).is_ok()
        );
    }

    #[test]
    fn capacity_preserves_unknown_values() {
        let capacity = ResourceCapacity::new(
            None,
            Some(amount("100")),
            None,
        ).unwrap();

        assert!(capacity.total().is_none());
        assert!(capacity.available().is_some());
    }

    // ------------------------------------------------------------------------
    // Resource descriptions
    // ------------------------------------------------------------------------

    #[test]
    fn resource_description_preserves_identity() {
        let resource = resource();

        assert_eq!(
            resource.kind().as_str(),
            "classical.memory"
        );

        assert_eq!(
            resource.unit().canonical_name(),
            "information:byte"
        );
    }

    #[test]
    fn resource_description_rejects_invalid_text() {
        assert!(
            resource()
                .with_description(" \n ")
                .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Observations
    // ------------------------------------------------------------------------

    #[test]
    fn observation_requires_matching_provider() {
        let result = ResourceObservation::new(
            resource(),
            ResourceCapacity::new(None, None, None).unwrap(),
            ResourceAvailability::Unknown,
            ResourceProvenance::new(
                ProviderId::new("different.provider").unwrap()
            ),
            SystemTime::now(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn unknown_availability_is_not_available() {
        let observation = ResourceObservation::new(
            resource(),
            ResourceCapacity::new(None, None, None).unwrap(),
            ResourceAvailability::Unknown,
            ResourceProvenance::new(provider()),
            SystemTime::now(),
        ).unwrap();

        assert!(!observation.is_observed_available());
    }

    #[test]
    fn available_observation_is_explicit() {
        let observation = ResourceObservation::new(
            resource(),
            ResourceCapacity::new(
                Some(amount("100")),
                Some(amount("50")),
                Some(amount("30")),
            ).unwrap(),
            ResourceAvailability::Available,
            ResourceProvenance::new(provider()),
            SystemTime::now(),
        ).unwrap();

        assert!(observation.is_observed_available());
    }

    #[test]
    fn conditional_availability_requires_explanation() {
        assert!(
            ResourceAvailability::conditional("")
                .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Serialization validation
    // ------------------------------------------------------------------------

    #[test]
    fn resource_kind_deserialization_validates() {
        assert!(
            serde_json::from_str::<ResourceKindId>("\"\"")
                .is_err()
        );
    }

    #[test]
    fn resource_unit_deserialization_validates() {
        assert!(
            serde_json::from_str::<ResourceUnitId>(
                r#"{"namespace":"","name":"byte"}"#
            ).is_err()
        );
    }

    #[test]
    fn capacity_deserialization_validates() {
        assert!(
            serde_json::from_str::<ResourceCapacity>(
                r#"{"total":"100","available":"80","in_use":"80"}"#
            ).is_err()
        );
    }

    #[test]
    fn quantity_serialization_round_trips() {
        let original = amount(
            "123456789012345678901234567890.125"
        );

        let encoded = serde_json::to_string(&original).unwrap();

        let restored: ResourceAmount =
            serde_json::from_str(&encoded).unwrap();

        assert_eq!(original, restored);
    }

    #[test]
    fn resource_kind_serialization_round_trips() {
        let original = ResourceKindId::new(
            "future.hardware.resource"
        ).unwrap();

        let encoded = serde_json::to_string(&original).unwrap();

        let restored: ResourceKindId =
            serde_json::from_str(&encoded).unwrap();

        assert_eq!(original, restored);
    }
}
