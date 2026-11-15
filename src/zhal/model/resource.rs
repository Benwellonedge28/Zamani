
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer — Resources
//!
//! Path: `src/zhal/resource.rs`
//!
//! Edition: Rust 2021
//! Minimum supported Rust: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines the provider-independent representation of physical
//! computational resources and their observed characteristics.
//!
//! Supports resource descriptions and observations for:
//!
//! - Classical processors and processing units.
//! - Memory and storage resources.
//! - Quantum processors and physical quantum resources.
//! - FPGA and ASIC resources.
//! - GPUs and other accelerators.
//! - Embedded devices.
//! - Network and communication resources.
//! - Distributed computing resources.
//! - AI and tensor accelerators.
//! - Hybrid computing resources.
//! - Future computational technologies.
//!
//! Resource categories, quantities and units are extensible.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Physical hardware resource descriptions.
//! - Hardware resource kinds.
//! - Physical resource units.
//! - Exact resource quantities.
//! - Resource availability observations.
//! - Capacity observations.
//! - Available-resource observations.
//! - In-use resource observations.
//! - Resource ownership references.
//! - Resource observation provenance.
//! - Resource observation validation.
//!
//! This module does NOT own:
//!
//! - Source-language resource declarations.
//! - Resource AST representations.
//! - Semantic resource requirements.
//! - Resource preferences.
//! - Resource constraints.
//! - Resource feasibility.
//! - Capability discovery.
//! - Provider registration.
//! - Hardware discovery algorithms.
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
//! Those responsibilities remain with their canonical authorities.
//!
//! ## CANONICAL REPOSITORY AUTHORITIES
//!
//! Physical resource identity:
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
//! Semantic resource requirements:
//!     crate::frontend::semantic::resources
//!
//! Runtime resource acquisition:
//!     crate::zur::resources
//!
//! Runtime memory allocation:
//!     crate::zur::memory
//!
//! Quantum resources:
//!     crate::quantum
//!
//! HDL resources:
//!     crate::hdl
//!
//! Distributed resources:
//!     crate::distributed
//!
//! ## FUNDAMENTAL DISTINCTIONS
//!
//! Resource description != resource observation.
//! Resource observation != resource availability guarantee.
//! Resource availability != resource reservation.
//! Resource reservation != resource allocation.
//! Resource allocation != resource execution.
//!
//! This module describes physical resources.
//! It never claims to have acquired them.
//!
//! ## SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Resource quantity.
//! - Resource identifier length.
//! - Resource categories.
//! - Resource units.
//! - Resource observations.
//! - Resource descriptions.
//! - Hardware devices.
//! - Computational domains.
//!
//! Exact decimal quantities are stored as canonical strings.
//!
//! No u64, u128, usize or floating-point representation is
//! used as the universal resource quantity representation.
//!
//! Actual storage and computation remain constrained by
//! available memory and platform representation limits.
//!
//! Literal infinite physical resources are not promised.
//!
//! ## GRAMMAR BOUNDARY
//!
//! Source-level resource intent belongs to:
//!
//!     grammar/resources/
//!     grammar/types/resource.g4
//!
//! Its semantic interpretation belongs to:
//!
//!     crate::frontend::semantic::resources
//!
//! This module does not parse grammar productions or introduce
//! another resource-requirement representation.
//!
//! ## CONCURRENCY
//!
//! Resource observations are immutable values.
//!
//! Providers construct observations and publish them through
//! the appropriate ZHAL inventory and discovery interfaces.
//!
//! This module does not maintain global mutable state.
//!
//! ## SECURITY
//!
//! Resource metadata and ownership references are descriptive.
//!
//! They must not contain:
//!
//! - Passwords.
//! - Private keys.
//! - Access tokens.
//! - Authorization headers.
//! - Session credentials.
//!
//! Resource identity does not establish authorization.
//!
//! ## FAILURE BEHAVIOR
//!
//! Invalid quantities, identities and inconsistent observations
//! return structured errors.
//!
//! Unknown availability is never interpreted as available.
//!
//! Stale observations are never silently treated as current.
//!
//! Missing observations are not fabricated.
//!
//! ## INTEGRATION
//!
//! `src/zhal/identity.rs` provides the canonical resource identity.
//!
//! `src/zhal/model/device.rs` may reference resource descriptions.
//!
//! `src/zhal/model/snapshot.rs` may retain immutable observations.
//!
//! `src/zhal/resources/observation.rs` may adapt provider observations
//! to the appropriate inventory and observation interfaces.
//!
//! `src/zur/resources/lease.rs` remains authoritative for resource leases.
//!
//! No runtime allocator is introduced here.

use core::cmp::Ordering;
use core::fmt;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::stdlib::provider::ProviderId;

use crate::zhal::identity::HardwareResourceIdentity;

// ============================================================================
// MODULE CONTRACT
// ============================================================================

/// Stable component identifier.
pub const COMPONENT: &str = "zhal.resource";

/// Resource-model schema version.
///
/// This is not the Zamani language version, Cargo package version,
// provider API version or hardware firmware version.
pub const RESOURCE_SCHEMA_VERSION: &str = "1";

// ============================================================================
// ERRORS
// ============================================================================

/// Classification of resource-model validation errors.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
)]
#[non_exhaustive]
pub enum ResourceErrorKind {
    /// An identifier is empty.
    EmptyIdentifier,

    /// An identifier contains invalid whitespace.
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

    /// The observation contains an invalid timestamp.
    InvalidTimestamp,

    /// The provider identity is invalid.
    InvalidProvider,

    /// The resource ownership reference is invalid.
    InvalidOwnership,
}

impl fmt::Display for ResourceErrorKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let message = match self {
            Self::EmptyIdentifier => "identifier must not be empty",
            Self::InvalidWhitespace => "identifier contains whitespace",
            Self::ControlCharacter => "identifier contains control characters",
            Self::InvalidQuantity => "invalid resource quantity",
            Self::InvalidUnit => "invalid resource unit",
            Self::InvalidKind => "invalid resource kind",
            Self::InconsistentObservation => "inconsistent resource observation",
            Self::InvalidTimestamp => "invalid observation timestamp",
            Self::InvalidProvider => "invalid provider identity",
            Self::InvalidOwnership => "invalid resource ownership reference",
        };

        formatter.write_str(message)
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
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}: {}: {}",
            COMPONENT,
            self.component,
            self.message
        )
    }
}

impl std::error::Error for ResourceError {}

/// Result type for resource-model operations.
pub type ResourceResult<T> = Result<T, ResourceError>;

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_identifier(
    value: &str,
    component: &'static str,
) -> ResourceResult<()> {
    if value.is_empty() {
        return Err(ResourceError::new(
            ResourceErrorKind::EmptyIdentifier,
            component,
            "identifier cannot be empty",
        ));
    }

    if value.trim() != value {
        return Err(ResourceError::new(
            ResourceErrorKind::InvalidWhitespace,
            component,
            "leading or trailing whitespace is not permitted",
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(ResourceError::new(
            ResourceErrorKind::ControlCharacter,
            component,
            "control characters are not permitted",
        ));
    }

    if value.chars().any(char::is_whitespace) {
        return Err(ResourceError::new(
            ResourceErrorKind::InvalidWhitespace,
            component,
            "whitespace is not permitted",
        ));
    }

    Ok(())
}

// ============================================================================
// OPEN-WORLD RESOURCE KIND
// ============================================================================

/// Extensible physical resource-kind identity.
///
/// This is intentionally not a closed enum.
///
/// Examples:
///
/// - `classical.processor`
/// - `classical.memory`
/// - `quantum.qubit`
/// - `quantum.control_channel`
/// - `hdl.logic_element`
/// - `hdl.memory_block`
/// - `accelerator.compute_unit`
/// - `network.bandwidth`
/// - `embedded.sensor`
/// - `photonic.optical_channel`
/// - `neuromorphic.neuron`
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
#[serde(transparent)]
pub struct ResourceKindId(String);

impl ResourceKindId {
    /// Creates a validated resource-kind identity.
    pub fn new(
        value: impl Into<String>,
    ) -> ResourceResult<Self> {
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

impl fmt::Display for ResourceKindId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// OPEN-WORLD RESOURCE UNIT
// ============================================================================

/// Extensible resource-unit identity.
///
/// Unit interpretation and dimensional conversion belong to the
/// appropriate domain or unit-registry authority.
///
/// Examples:
///
/// - `information:byte`
/// - `information:bit`
/// - `frequency:hertz`
/// - `time:second`
/// - `compute:operation`
/// - `quantum:physical_qubit`
/// - `network:byte_per_second`
/// - `power:watt`
/// - `storage:sector`
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

impl fmt::Display for ResourceUnitId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}:{}",
            self.namespace,
            self.name
        )
    }
}

// ============================================================================
// ARBITRARY-PRECISION RESOURCE QUANTITY
// ============================================================================

/// Exact, non-negative decimal resource quantity.
///
/// Values are stored as canonical decimal strings rather than
/// bounded machine integers or floating-point numbers.
///
/// This permits resource quantities larger than the representable
/// range of u64, u128 or usize.
///
/// Accepted representation:
///
/// `DIGITS`
///
/// or:
///
/// `DIGITS "." DIGITS`
///
/// Scientific notation and negative values are deliberately excluded.
///
/// Examples:
///
/// - `0`
/// - `1`
/// - `1024`
/// - `184467440737095516160000000000000000000`
/// - `0.125`
///
/// This type represents quantities, not units.
///
/// A quantity is meaningful only together with its resource unit.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[serde(try_from = "String", into = "String")]
pub struct ResourceAmount {
    canonical: String,
}

impl ResourceAmount {
    /// Parses and canonicalizes a resource quantity.
    pub fn new(
        value: impl AsRef<str>,
    ) -> ResourceResult<Self> {
        let value = value.as_ref();

        if value.is_empty() {
            return Err(ResourceError::new(
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
            return Err(ResourceError::new(
                ResourceErrorKind::InvalidQuantity,
                "quantity",
                "expected unsigned decimal digits with at most one decimal point",
            ));
        }

        if let Some(fraction) = fraction {
            if fraction.is_empty()
                || !fraction.bytes().all(|b| b.is_ascii_digit())
            {
                return Err(ResourceError::new(
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
                    let mut result = String::with_capacity(
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

    /// Returns whether the quantity is exactly zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.canonical == "0"
    }

    /// Compares quantities numerically without machine-integer conversion.
    #[must_use]
    pub fn numeric_cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        let (left_integer, left_fraction) =
            split_decimal(&self.canonical);

        let (right_integer, right_fraction) =
            split_decimal(&other.canonical);

        left_integer
            .len()
            .cmp(&right_integer.len())
            .then_with(|| {
                left_integer.cmp(right_integer)
            })
            .then_with(|| {
                compare_fraction(
                    left_fraction,
                    right_fraction,
                )
            })
    }
}

fn split_decimal(
    value: &str,
) -> (&str, &str) {
    match value.split_once('.') {
        Some((integer, fraction)) => {
            (integer, fraction)
        }

        None => (value, ""),
    }
}

fn compare_fraction(
    left: &str,
    right: &str,
) -> Ordering {
    let width = left.len().max(right.len());

    for index in 0..width {
        let left_digit = left
            .as_bytes()
            .get(index)
            .copied()
            .unwrap_or(b'0');

        let right_digit = right
            .as_bytes()
            .get(index)
            .copied()
            .unwrap_or(b'0');

        let ordering = left_digit.cmp(&right_digit);

        if ordering != Ordering::Equal {
            return ordering;
        }
    }

    Ordering::Equal
}

impl Ord for ResourceAmount {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.numeric_cmp(other)
    }
}

impl PartialOrd for ResourceAmount {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for ResourceAmount {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.canonical)
    }
}

impl TryFrom<String> for ResourceAmount {
    type Error = ResourceError;

    fn try_from(value: String) -> ResourceResult<Self> {
        Self::new(value)
    }
}

impl From<ResourceAmount> for String {
    fn from(value: ResourceAmount) -> Self {
        value.canonical
    }
}

// ============================================================================
// RESOURCE AVAILABILITY
// ============================================================================

/// Observed physical resource availability.
///
/// Availability is observational, not a reservation guarantee.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
pub enum ResourceAvailability {
    /// Evidence establishes that the resource is available.
    Available,

    /// Evidence establishes that the resource is unavailable.
    Unavailable,

    /// Availability depends on explicitly described conditions.
    Conditional {
        /// Description of the conditions.
        conditions: String,
    },

    /// The provider cannot establish availability.
    Unknown,

    /// The observation is known to be stale.
    Stale,
}

// ============================================================================
// RESOURCE OWNERSHIP REFERENCE
// ============================================================================

/// Descriptive reference to an entity associated with a resource.
///
/// This is not an allocation, reservation or authorization.
///
/// The referenced owner may be an execution, session, provider,
/// deployment, device or another extensible entity.
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
pub struct ResourceOwnerReference {
    scope: String,
    identity: String,
}

impl ResourceOwnerReference {
    /// Creates a validated ownership reference.
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

// ============================================================================
// RESOURCE DESCRIPTION
// ============================================================================

/// Provider-independent description of a physical resource.
///
/// A description establishes resource identity and classification.
///
/// It does not establish current capacity or availability.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct HardwareResource {
    identity: HardwareResourceIdentity,
    kind: ResourceKindId,
    unit: ResourceUnitId,
    owner: Option<ResourceOwnerReference>,
    description: Option<String>,
}

impl HardwareResource {
    /// Creates a validated hardware resource description.
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

    /// Attaches a human-readable description.
    #[must_use]
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = Some(description.into());
        self
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

    /// Returns the optional ownership reference.
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

// ============================================================================
// RESOURCE CAPACITY
// ============================================================================

/// Observed quantities associated with a physical resource.
///
/// All quantities use the unit established by the resource description.
///
/// Unknown quantities remain unknown.
///
/// This structure never fabricates a capacity from missing information.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct ResourceCapacity {
    total: Option<ResourceAmount>,
    available: Option<ResourceAmount>,
    in_use: Option<ResourceAmount>,
}

impl ResourceCapacity {
    /// Creates and validates a resource-capacity observation.
    ///
    /// Available and in-use quantities must not individually exceed
    /// a known total.
    ///
    /// When both available and in-use quantities are known, their
    /// sum is not required to equal total because some capacity may
    /// be reserved, unavailable or otherwise unaccounted for.
    pub fn new(
        total: Option<ResourceAmount>,
        available: Option<ResourceAmount>,
        in_use: Option<ResourceAmount>,
    ) -> ResourceResult<Self> {
        if let Some(total) = &total {
            if let Some(available) = &available {
                if available > total {
                    return Err(ResourceError::new(
                        ResourceErrorKind::InconsistentObservation,
                        "capacity",
                        "available quantity exceeds total capacity",
                    ));
                }
            }

            if let Some(in_use) = &in_use {
                if in_use > total {
                    return Err(ResourceError::new(
                        ResourceErrorKind::InconsistentObservation,
                        "capacity",
                        "in-use quantity exceeds total capacity",
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

// ============================================================================
// RESOURCE OBSERVATION PROVENANCE
// ============================================================================

/// Provenance of a physical resource observation.
///
/// Observation provenance identifies the source and observation time.
///
/// It does not independently prove that the provider is trustworthy.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
)]
pub struct ResourceProvenance {
    provider_id: ProviderId,

    /// External provider observation revision.
    ///
    /// This is opaque provider-defined version information.
    source_revision: Option<String>,
}

impl ResourceProvenance {
    /// Creates resource observation provenance.
    pub fn new(
        provider_id: ProviderId,
    ) -> Self {
        Self {
            provider_id,
            source_revision: None,
        }
    }

    /// Attaches an external provider revision.
    #[must_use]
    pub fn with_source_revision(
        mut self,
        revision: impl Into<String>,
    ) -> Self {
        self.source_revision = Some(revision.into());
        self
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the optional source revision.
    #[must_use]
    pub fn source_revision(&self) -> Option<&str> {
        self.source_revision.as_deref()
    }
}

// ============================================================================
// COMPLETE RESOURCE OBSERVATION
// ============================================================================

/// Immutable observation of a physical hardware resource.
///
/// This structure combines:
///
/// - A resource description.
/// - Capacity information.
/// - Availability information.
/// - Observation provenance.
/// - Observation time.
///
/// It is not a reservation or allocation.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
)]
pub struct ResourceObservation {
    resource: HardwareResource,
    capacity: ResourceCapacity,
    availability: ResourceAvailability,
    provenance: ResourceProvenance,
    observed_at: SystemTime,
}

impl ResourceObservation {
    /// Constructs a resource observation.
    ///
    /// The provider identity in the provenance must match the
    /// provider identity of the hardware resource.
    pub fn new(
        resource: HardwareResource,
        capacity: ResourceCapacity,
        availability: ResourceAvailability,
        provenance: ResourceProvenance,
        observed_at: SystemTime,
    ) -> ResourceResult<Self> {
        if resource.identity().provider_id()
            != provenance.provider_id()
        {
            return Err(ResourceError::new(
                ResourceErrorKind::InvalidProvider,
                "provenance",
                "observation provider does not own the resource identity",
            ));
        }

        if observed_at.duration_since(SystemTime::UNIX_EPOCH).is_err() {
            return Err(ResourceError::new(
                ResourceErrorKind::InvalidTimestamp,
                "observation",
                "observation timestamp precedes the Unix epoch",
            ));
        }

        if let ResourceAvailability::Conditional {
            conditions,
        } = &availability
        {
            if conditions.trim().is_empty() {
                return Err(ResourceError::new(
                    ResourceErrorKind::InconsistentObservation,
                    "availability",
                    "conditional availability requires a description",
                ));
            }
        }

        Ok(Self {
            resource,
            capacity,
            availability,
            provenance,
            observed_at,
        })
    }

    /// Returns the hardware resource description.
    #[must_use]
    pub fn resource(&self) -> &HardwareResource {
        &self.resource
    }

    /// Returns the capacity observation.
    #[must_use]
    pub fn capacity(&self) -> &ResourceCapacity {
        &self.capacity
    }

    /// Returns the availability state.
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

    /// Returns whether the resource is explicitly observed as available.
    ///
    /// This does not guarantee that it remains available or that
    /// it has been reserved.
    #[must_use]
    pub fn is_observed_available(&self) -> bool {
        matches!(
            self.availability,
            ResourceAvailability::Available
        )
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
    use crate::zhal::identity::{
        HardwareDeviceIdentity,
        HardwareResourceId,
        HardwareResourceIdentity,
    };

    fn provider() -> ProviderId {
        ProviderId::new("test.provider")
            .expect("valid provider")
    }

    fn resource_identity() -> HardwareResourceIdentity {
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

    fn resource() -> HardwareResource {
        HardwareResource::new(
            resource_identity(),
            ResourceKindId::new("classical.memory")
                .expect("valid kind"),
            ResourceUnitId::new(
                "information",
                "byte",
            )
            .expect("valid unit"),
        )
    }

    fn amount(value: &str) -> ResourceAmount {
        ResourceAmount::new(value)
            .expect("valid quantity")
    }

    // ------------------------------------------------------------------------
    // Resource quantities
    // ------------------------------------------------------------------------

    #[test]
    fn quantity_canonicalizes_leading_zeroes() {
        assert_eq!(
            amount("0000123").as_str(),
            "123"
        );
    }

    #[test]
    fn quantity_canonicalizes_fractional_zeroes() {
        assert_eq!(
            amount("00012.3400").as_str(),
            "12.34"
        );
    }

    #[test]
    fn quantity_canonicalizes_zero() {
        assert_eq!(
            amount("0000.000").as_str(),
            "0"
        );
    }

    #[test]
    fn quantity_supports_values_beyond_u128() {
        let value = amount(
            "340282366920938463463374607431768211456000000000000000000"
        );

        assert!(!value.is_zero());
    }

    #[test]
    fn quantity_compares_without_machine_integer_conversion() {
        assert!(amount("100000000000000000000")
            > amount("99999999999999999999"));

        assert!(amount("1.21") > amount("1.2"));

        assert_eq!(
            amount("1.200").cmp(&amount("1.2")),
            Ordering::Equal
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
    // Resource identity and extensibility
    // ------------------------------------------------------------------------

    #[test]
    fn resource_kinds_are_open_ended() {
        let kind = ResourceKindId::new(
            "future.photonic.optical_resource"
        )
        .unwrap();

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
        )
        .unwrap();

        assert_eq!(
            unit.canonical_name(),
            "future.domain:specialized_unit"
        );
    }

    #[test]
    fn resource_kinds_reject_empty_values() {
        assert!(
            ResourceKindId::new("").is_err()
        );
    }

    #[test]
    fn resource_units_reject_control_characters() {
        assert!(
            ResourceUnitId::new(
                "information",
                "byte\n",
            )
            .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Resource capacity
    // ------------------------------------------------------------------------

    #[test]
    fn capacity_accepts_consistent_observations() {
        let capacity = ResourceCapacity::new(
            Some(amount("1000")),
            Some(amount("300")),
            Some(amount("500")),
        )
        .unwrap();

        assert_eq!(
            capacity.total().unwrap().as_str(),
            "1000"
        );
    }

    #[test]
    fn capacity_rejects_available_above_total() {
        let result = ResourceCapacity::new(
            Some(amount("100")),
            Some(amount("101")),
            None,
        );

        assert_eq!(
            result.unwrap_err().kind(),
            ResourceErrorKind::InconsistentObservation
        );
    }

    #[test]
    fn capacity_rejects_usage_above_total() {
        let result = ResourceCapacity::new(
            Some(amount("100")),
            None,
            Some(amount("101")),
        );

        assert!(result.is_err());
    }

    #[test]
    fn capacity_supports_unknown_total() {
        let capacity = ResourceCapacity::new(
            None,
            Some(amount("100")),
            None,
        )
        .unwrap();

        assert!(!capacity.has_known_total());
    }

    #[test]
    fn capacity_does_not_assume_available_plus_used_equals_total() {
        let capacity = ResourceCapacity::new(
            Some(amount("100")),
            Some(amount("20")),
            Some(amount("30")),
        );

        assert!(capacity.is_ok());
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
    fn resource_description_accepts_future_domains() {
        let resource = HardwareResource::new(
            resource_identity(),
            ResourceKindId::new(
                "future.molecular.computation"
            )
            .unwrap(),
            ResourceUnitId::new(
                "future",
                "molecular_unit",
            )
            .unwrap(),
        );

        assert_eq!(
            resource.kind().as_str(),
            "future.molecular.computation"
        );
    }

    // ------------------------------------------------------------------------
    // Resource observations
    // ------------------------------------------------------------------------

    #[test]
    fn observation_requires_matching_provider_identity() {
        let result = ResourceObservation::new(
            resource(),
            ResourceCapacity::new(
                None,
                None,
                None,
            )
            .unwrap(),
            ResourceAvailability::Unknown,
            ResourceProvenance::new(
                ProviderId::new("different.provider").unwrap()
            ),
            SystemTime::now(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn observation_preserves_unknown_availability() {
        let observation = ResourceObservation::new(
            resource(),
            ResourceCapacity::new(
                None,
                None,
                None,
            )
            .unwrap(),
            ResourceAvailability::Unknown,
            ResourceProvenance::new(provider()),
            SystemTime::now(),
        )
        .unwrap();

        assert!(
            !observation.is_observed_available()
        );
    }

    #[test]
    fn observation_preserves_available_state() {
        let observation = ResourceObservation::new(
            resource(),
            ResourceCapacity::new(
                Some(amount("100")),
                Some(amount("50")),
                Some(amount("30")),
            )
            .unwrap(),
            ResourceAvailability::Available,
            ResourceProvenance::new(provider()),
            SystemTime::now(),
        )
        .unwrap();

        assert!(
            observation.is_observed_available()
        );
    }

    #[test]
    fn conditional_availability_requires_explanation() {
        let result = ResourceObservation::new(
            resource(),
            ResourceCapacity::new(
                None,
                None,
                None,
            )
            .unwrap(),
            ResourceAvailability::Conditional {
                conditions: String::new(),
            },
            ResourceProvenance::new(provider()),
            SystemTime::now(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn observation_rejects_pre_epoch_timestamp() {
        let result = ResourceObservation::new(
            resource(),
            ResourceCapacity::new(
                None,
                None,
                None,
            )
            .unwrap(),
            ResourceAvailability::Unknown,
            ResourceProvenance::new(provider()),
            SystemTime::UNIX_EPOCH - std::time::Duration::from_secs(1),
        );

        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // Serialization
    // ------------------------------------------------------------------------

    #[test]
    fn resource_quantity_round_trips() {
        let original = amount(
            "123456789012345678901234567890.125"
        );

        let serialized = serde_json::to_string(&original)
            .unwrap();

        let restored: ResourceAmount =
            serde_json::from_str(&serialized)
                .unwrap();

        assert_eq!(original, restored);
    }

    #[test]
    fn resource_kind_round_trips() {
        let original = ResourceKindId::new(
            "future.hardware.resource"
        )
        .unwrap();

        let serialized = serde_json::to_string(&original)
            .unwrap();

        let restored: ResourceKindId =
            serde_json::from_str(&serialized)
                .unwrap();

        assert_eq!(original, restored);
    }

    #[test]
    fn resource_unit_round_trips() {
        let original = ResourceUnitId::new(
            "information",
            "byte",
        )
        .unwrap();

        let serialized = serde_json::to_string(&original)
            .unwrap();

        let restored: ResourceUnitId =
            serde_json::from_str(&serialized)
                .unwrap();

        assert_eq!(original, restored);
    }
}
