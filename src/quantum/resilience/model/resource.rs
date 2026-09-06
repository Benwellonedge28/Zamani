//! Zamani Quantum Resilience — Resource Model
//!
//! This module defines the hardware-independent resource vocabulary used by
//! the resilience subsystem.
//!
//! # Architectural role
//!
//! `resilience::model::resource` answers:
//!
//! > "Which computational or execution resource is being observed, degraded,
//! > constrained, recovered, migrated, or otherwise referenced by resilience?"
//!
//! It does NOT:
//!
//! - allocate hardware;
//! - discover hardware;
//! - perform routing;
//! - perform scheduling;
//! - execute quantum programs;
//! - own hardware calibration;
//! - define quantum fault semantics;
//! - define QEC semantics;
//! - define backend/provider identities;
//! - define canonical IR identities;
//! - define resource-capability matching;
//! - define recovery policy;
//! - define resilience actions.
//!
//! Those responsibilities remain owned by their respective subsystems.
//!
//! # Canonical identity rule
//!
//! This file MUST reuse canonical Zamani IR identities.
//!
//! In particular:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! are authoritative for logical and physical qubit identity.
//!
//! Generic IR resources use:
//!
//! ```text
//! quantum::ir::core::identity::ResourceId
//! ```
//!
//! This module MUST NOT define:
//!
//! ```text
//! ResilienceQubitId
//! LogicalQubitId
//! ResiliencePhysicalQubitId
//! ResilienceResourceId
//! ```
//!
//! or equivalent competing identity types.
//!
//! The repository explicitly establishes canonical qubit identity ownership
//! in the IR qubit subsystem and requires downstream systems to reuse it.
//!
//! # Universal-program principle
//!
//! A resilience resource is an observation/reference to a resource, not a
//! declaration of machine size.
//!
//! Nothing here assumes:
//!
//! ```text
//! 1 qubit
//! 127 qubits
//! 1000 qubits
//! 1_000_000 qubits
//! ```
//!
//! The same model must represent a single resource, a large QPU, a
//! distributed system, or a heterogeneous execution fabric.
//!
//! Practical limits arise only from:
//!
//! - representable identifier space;
//! - available host memory;
//! - explicit policy;
//! - target capabilities;
//! - execution resources.
//!
//! Those are not semantic limits imposed by this module.
//!
//! # Resource / capability / allocation separation
//!
//! These concepts are deliberately separate:
//!
//! ```text
//! RESOURCE
//!     What resource is being discussed?
//!
//! CAPABILITY
//!     What can that resource do?
//!
//! AVAILABILITY
//!     How much of that resource is currently available?
//!
//! DEGRADATION
//!     How has the resource's useful condition changed?
//!
//! ALLOCATION
//!     Who/what has been assigned the resource?
//!
//! ROUTING
//!     How are logical resources mapped to physical resources?
//!
//! SCHEDULING
//!     When are resources used?
//!
//! HARDWARE
//!     What actually exists on a target?
//! ```
//!
//! This file owns only the first three concepts insofar as resilience needs
//! normalized observations.
//!
//! # Determinism
//!
//! Resource identity types must remain deterministic.
//!
//! Collections exposed here therefore use ordered containers where ordering
//! is semantically observable.
//!
//! No hash-map iteration order may influence resilience decisions.
//!
//! # Unknown versus unavailable
//!
//! These states are intentionally different:
//!
//! ```text
//! Unknown
//!     The system does not know the current state.
//!
//! Unavailable
//!     The system knows that the resource cannot currently be used.
//!
//! Available
//!     The resource is currently usable.
//!
//! Unbounded
//!     The semantic model explicitly has no finite upper bound.
//! ```
//!
//! In particular, `Unknown` must never be silently converted into
//! `Unavailable`, and `Unbounded` must never be represented by
//! `u64::MAX` or `usize::MAX`.
//!
//! # No hardware assumptions
//!
//! This module contains no provider names, device names, topology assumptions,
//! qubit-count constants, retry counts, fidelity thresholds, or backend-specific
//! branches.
//!
//! # Serialization boundary
//!
//! This domain model intentionally does not define a wire format.
//!
//! Canonical resilience serialization belongs to:
//!
//! ```text
//! quantum::resilience::serialization
//! ```
//!
//! Serialization code may encode these types without changing their semantic
//! ownership.
//!
//! # Error boundary
//!
//! Local constructor validation belongs here where necessary.
//!
//! Higher-level resilience errors remain owned by:
//!
//! ```text
//! quantum::resilience::errors
//! ```
//!
//! This module therefore exposes a small local error type for malformed
//! resource-domain values and allows the resilience error layer to translate
//! it later without changing the underlying resource model.
//!
//! # Security boundary
//!
//! This module does not trust resource observations merely because they are
//! represented by a valid Rust value.
//!
//! Authenticity, provenance, authorization, freshness, and trust level belong
//! to telemetry/diagnosis/security layers.
//!
//! A `Resource` therefore represents a normalized semantic value, not proof
//! that an external actor's observation is trustworthy.
//!
//! # Rust requirements
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! The module explicitly forbids unsafe code.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use crate::quantum::ir::core::identity::ResourceId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// ============================================================================
// Resource kind
// ============================================================================

/// Extensible semantic kind of resource.
///
/// The value is intentionally represented as an owned string rather than a
/// closed enum. Resilience must be able to reason about future resource types
/// without requiring a new Zamani release merely because a new hardware or
/// execution resource appeared.
///
/// Examples of valid semantic kinds include:
///
/// ```text
/// qubit
/// logical_qubit
/// physical_qubit
/// coupling
/// control_channel
/// measurement_channel
/// reset_channel
/// execution_slot
/// classical_memory
/// quantum_memory
/// communication_link
/// cryogenic_control
/// custom.domain.resource
/// ```
///
/// These are labels, not hardware-provider identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceKind(String);

impl ResourceKind {
    /// Creates a resource kind.
    ///
    /// The value must not be empty or consist entirely of whitespace.
    ///
    /// No fixed vocabulary is imposed here so that the model remains
    /// extensible.
    pub fn new(value: impl Into<String>) -> Result<Self, ResourceError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ResourceError::EmptyKind);
        }

        Ok(Self(value))
    }

    /// Returns the semantic resource-kind label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the kind and returns its owned label.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<&str> for ResourceKind {
    type Error = ResourceError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for ResourceKind {
    type Error = ResourceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Resource identity
// ============================================================================

/// Canonical identity of a resource relevant to resilience.
///
/// The variants deliberately preserve the semantic distinction between:
///
/// - generic IR resources;
/// - logical qubits;
/// - physical qubits.
///
/// A numerical equality between a logical and physical identifier must never
/// make them interchangeable.
///
/// Generic resources use the canonical IR `ResourceId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceIdentity {
    /// Canonical generic IR resource identity.
    Ir(ResourceId),

    /// Canonical logical-qubit identity.
    LogicalQubit(QubitId),

    /// Canonical physical-qubit identity.
    PhysicalQubit(PhysicalQubitId),
}

impl ResourceIdentity {
    /// Creates a generic IR resource identity.
    #[must_use]
    pub const fn ir(id: ResourceId) -> Self {
        Self::Ir(id)
    }

    /// Creates a logical-qubit resource identity.
    #[must_use]
    pub const fn logical_qubit(id: QubitId) -> Self {
        Self::LogicalQubit(id)
    }

    /// Creates a physical-qubit resource identity.
    #[must_use]
    pub const fn physical_qubit(id: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(id)
    }

    /// Returns `true` when this identity represents a logical qubit.
    #[must_use]
    pub const fn is_logical_qubit(self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns `true` when this identity represents a physical qubit.
    #[must_use]
    pub const fn is_physical_qubit(self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }

    /// Returns `true` when this identity is a generic IR resource.
    #[must_use]
    pub const fn is_ir_resource(self) -> bool {
        matches!(self, Self::Ir(_))
    }

    /// Returns the logical qubit identity, if this is a logical-qubit
    /// resource.
    #[must_use]
    pub const fn logical_qubit_id(self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the physical qubit identity, if this is a physical-qubit
    /// resource.
    #[must_use]
    pub const fn physical_qubit_id(self) -> Option<PhysicalQubitId> {
        match self {
            Self::PhysicalQubit(id) => Some(id),
            _ => None,
        }
    }

    /// Returns the generic IR resource identity, if applicable.
    #[must_use]
    pub const fn ir_id(self) -> Option<ResourceId> {
        match self {
            Self::Ir(id) => Some(id),
            _ => None,
        }
    }
}

impl fmt::Display for ResourceIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ir(id) => write!(formatter, "ir:{id:?}"),
            Self::LogicalQubit(id) => write!(formatter, "logical-qubit:{id:?}"),
            Self::PhysicalQubit(id) => write!(formatter, "physical-qubit:{id:?}"),
        }
    }
}

// ============================================================================
// Resource scope
// ============================================================================

/// Semantic scope of a resource reference.
///
/// Scope prevents resilience code from accidentally treating a physical
/// resource observation as though it were a logical resource observation.
///
/// The enum is intentionally small because the identity itself carries the
/// authoritative logical/physical distinction. This type expresses how the
/// resource is being used by resilience.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceScope {
    /// Resource is considered at the logical program level.
    Logical,

    /// Resource is considered at the physical execution level.
    Physical,

    /// Resource is considered as a generic IR/execution resource.
    Generic,
}

impl ResourceScope {
    /// Returns the scope appropriate for an identity.
    #[must_use]
    pub const fn for_identity(identity: ResourceIdentity) -> Self {
        match identity {
            ResourceIdentity::LogicalQubit(_) => Self::Logical,
            ResourceIdentity::PhysicalQubit(_) => Self::Physical,
            ResourceIdentity::Ir(_) => Self::Generic,
        }
    }

    /// Returns whether the scope is logical.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::Logical)
    }

    /// Returns whether the scope is physical.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::Physical)
    }

    /// Returns whether the scope is generic.
    #[must_use]
    pub const fn is_generic(self) -> bool {
        matches!(self, Self::Generic)
    }
}

// ============================================================================
// Resource quantity
// ============================================================================

/// Semantic quantity of a resource.
///
/// This type deliberately distinguishes finite quantities from semantic
/// unboundedness and unknown state.
///
/// `u128` is used for the finite representation so arithmetic performed by
/// resilience does not unnecessarily narrow the quantity domain.
///
/// `Unbounded` is semantic and is never represented by an integer sentinel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceQuantity {
    /// A known finite non-negative quantity.
    Finite(u128),

    /// The resource is semantically unbounded.
    Unbounded,

    /// The quantity is currently unknown.
    Unknown,
}

impl ResourceQuantity {
    /// Creates a finite quantity.
    #[must_use]
    pub const fn finite(value: u128) -> Self {
        Self::Finite(value)
    }

    /// Creates an explicitly unbounded quantity.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self::Unbounded
    }

    /// Creates an unknown quantity.
    #[must_use]
    pub const fn unknown() -> Self {
        Self::Unknown
    }

    /// Returns the finite quantity, if known and finite.
    #[must_use]
    pub const fn as_finite(self) -> Option<u128> {
        match self {
            Self::Finite(value) => Some(value),
            Self::Unbounded | Self::Unknown => None,
        }
    }

    /// Returns whether the quantity is known to be finite.
    #[must_use]
    pub const fn is_finite(self) -> bool {
        matches!(self, Self::Finite(_))
    }

    /// Returns whether the quantity is explicitly unbounded.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        matches!(self, Self::Unbounded)
    }

    /// Returns whether the quantity is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns whether the quantity is known to be zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        matches!(self, Self::Finite(0))
    }
}

// ============================================================================
// Resource availability
// ============================================================================

/// Current semantic availability of a resource.
///
/// This is deliberately distinct from health.
///
/// A resource may be healthy but temporarily unavailable because it is
/// reserved. Conversely, a resource may be degraded while still partially
/// available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceAvailability {
    /// The resource is known to be available.
    Available,

    /// The resource is known to be unavailable.
    Unavailable,

    /// The current availability cannot be established.
    Unknown,
}

impl ResourceAvailability {
    /// Returns whether the resource is known to be available.
    #[must_use]
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }

    /// Returns whether the resource is known to be unavailable.
    #[must_use]
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }

    /// Returns whether availability is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

// ============================================================================
// Resource observation
// ============================================================================

/// Normalized resilience resource observation.
///
/// This type is intentionally a value object.
///
/// It does not claim that an observation is trustworthy; provenance and
/// authentication are handled by telemetry/diagnosis layers.
///
/// `capacity` describes the known total capacity of the resource.
/// `available` describes the currently available amount.
///
/// The two values are intentionally independent so that unknown and
/// unbounded semantics can be represented without sentinel values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Resource {
    identity: ResourceIdentity,
    kind: ResourceKind,
    scope: ResourceScope,
    capacity: ResourceQuantity,
    available: ResourceQuantity,
}

impl Resource {
    /// Creates a resource observation.
    ///
    /// The scope is derived from the canonical resource identity, preventing
    /// callers from accidentally constructing a logical resource with a
    /// physical scope or vice versa.
    pub fn new(
        identity: ResourceIdentity,
        kind: ResourceKind,
        capacity: ResourceQuantity,
        available: ResourceQuantity,
    ) -> Result<Self, ResourceError> {
        Self::validate_quantity_relationship(capacity, available)?;

        Ok(Self {
            identity,
            kind,
            scope: ResourceScope::for_identity(identity),
            capacity,
            available,
        })
    }

    /// Creates a resource observation from an IR resource identity.
    pub fn from_ir_resource(
        id: ResourceId,
        kind: ResourceKind,
        capacity: ResourceQuantity,
        available: ResourceQuantity,
    ) -> Result<Self, ResourceError> {
        Self::new(
            ResourceIdentity::Ir(id),
            kind,
            capacity,
            available,
        )
    }

    /// Creates a logical-qubit resource observation.
    pub fn from_logical_qubit(
        id: QubitId,
        kind: ResourceKind,
        capacity: ResourceQuantity,
        available: ResourceQuantity,
    ) -> Result<Self, ResourceError> {
        Self::new(
            ResourceIdentity::LogicalQubit(id),
            kind,
            capacity,
            available,
        )
    }

    /// Creates a physical-qubit resource observation.
    pub fn from_physical_qubit(
        id: PhysicalQubitId,
        kind: ResourceKind,
        capacity: ResourceQuantity,
        available: ResourceQuantity,
    ) -> Result<Self, ResourceError> {
        Self::new(
            ResourceIdentity::PhysicalQubit(id),
            kind,
            capacity,
            available,
        )
    }

    /// Returns the canonical resource identity.
    #[must_use]
    pub const fn identity(&self) -> ResourceIdentity {
        self.identity
    }

    /// Returns the semantic resource kind.
    #[must_use]
    pub fn kind(&self) -> &ResourceKind {
        &self.kind
    }

    /// Returns the semantic resource scope.
    #[must_use]
    pub const fn scope(&self) -> ResourceScope {
        self.scope
    }

    /// Returns the known capacity.
    #[must_use]
    pub const fn capacity(&self) -> ResourceQuantity {
        self.capacity
    }

    /// Returns the currently available quantity.
    #[must_use]
    pub const fn available(&self) -> ResourceQuantity {
        self.available
    }

    /// Returns whether the resource has a known finite capacity.
    #[must_use]
    pub const fn has_finite_capacity(&self) -> bool {
        self.capacity.is_finite()
    }

    /// Returns whether the resource is explicitly unbounded.
    #[must_use]
    pub const fn is_unbounded(&self) -> bool {
        self.capacity.is_unbounded()
    }

    /// Returns whether the capacity is unknown.
    #[must_use]
    pub const fn capacity_is_unknown(&self) -> bool {
        self.capacity.is_unknown()
    }

    /// Returns whether the currently available quantity is known to be zero.
    #[must_use]
    pub const fn is_exhausted(&self) -> bool {
        self.available.is_zero()
    }

    /// Returns the finite remaining capacity when both capacity and
    /// availability are finite.
    ///
    /// This method never invents a value for unbounded or unknown quantities.
    #[must_use]
    pub const fn finite_available(&self) -> Option<u128> {
        self.available.as_finite()
    }

    /// Returns the number of unavailable units when both quantities are
    /// finite and the relationship is valid.
    ///
    /// A `None` result means that an exact finite value cannot be derived.
    #[must_use]
    pub const fn finite_unavailable(&self) -> Option<u128> {
        match (self.capacity, self.available) {
            (ResourceQuantity::Finite(capacity), ResourceQuantity::Finite(available)) => {
                match capacity.checked_sub(available) {
                    Some(value) => Some(value),
                    None => None,
                }
            }
            _ => None,
        }
    }

    fn validate_quantity_relationship(
        capacity: ResourceQuantity,
        available: ResourceQuantity,
    ) -> Result<(), ResourceError> {
        match (capacity, available) {
            (
                ResourceQuantity::Finite(capacity),
                ResourceQuantity::Finite(available),
            ) if available > capacity => Err(ResourceError::AvailableExceedsCapacity {
                capacity,
                available,
            }),

            (ResourceQuantity::Finite(_), ResourceQuantity::Unbounded) => {
                Err(ResourceError::UnboundedAvailabilityWithFiniteCapacity)
            }

            _ => Ok(()),
        }
    }
}

// ============================================================================
// Resource reference
// ============================================================================

/// Lightweight reference to a resource.
///
/// This type is useful when resilience algorithms need identity but do not
/// need to carry the entire resource observation.
///
/// It is intentionally copyable and contains no mutable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceRef {
    identity: ResourceIdentity,
}

impl ResourceRef {
    /// Creates a resource reference.
    #[must_use]
    pub const fn new(identity: ResourceIdentity) -> Self {
        Self { identity }
    }

    /// Returns the referenced identity.
    #[must_use]
    pub const fn identity(self) -> ResourceIdentity {
        self.identity
    }

    /// Creates a reference to a generic IR resource.
    #[must_use]
    pub const fn ir(id: ResourceId) -> Self {
        Self::new(ResourceIdentity::Ir(id))
    }

    /// Creates a reference to a logical qubit.
    #[must_use]
    pub const fn logical_qubit(id: QubitId) -> Self {
        Self::new(ResourceIdentity::LogicalQubit(id))
    }

    /// Creates a reference to a physical qubit.
    #[must_use]
    pub const fn physical_qubit(id: PhysicalQubitId) -> Self {
        Self::new(ResourceIdentity::PhysicalQubit(id))
    }
}

// ============================================================================
// Resource set
// ============================================================================

/// Deterministic collection of resource references.
///
/// This is intentionally backed by `BTreeSet` rather than `HashSet`.
///
/// Resilience planning, replay, serialization, and deterministic testing must
/// not depend on hash iteration order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ResourceSet {
    resources: std::collections::BTreeSet<ResourceRef>,
}

impl ResourceSet {
    /// Creates an empty resource set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a resource set from an iterator.
    pub fn from_iter<I>(resources: I) -> Self
    where
        I: IntoIterator<Item = ResourceRef>,
    {
        Self {
            resources: resources.into_iter().collect(),
        }
    }

    /// Inserts a resource reference.
    ///
    /// Returns `true` when the reference was not already present.
    pub fn insert(&mut self, resource: ResourceRef) -> bool {
        self.resources.insert(resource)
    }

    /// Removes a resource reference.
    ///
    /// Returns `true` when the reference existed.
    pub fn remove(&mut self, resource: &ResourceRef) -> bool {
        self.resources.remove(resource)
    }

    /// Returns whether the set contains a resource.
    #[must_use]
    pub fn contains(&self, resource: &ResourceRef) -> bool {
        self.resources.contains(resource)
    }

    /// Returns the number of unique resources.
    #[must_use]
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    /// Returns whether the set contains no resources.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Clears all resource references.
    pub fn clear(&mut self) {
        self.resources.clear();
    }

    /// Iterates resources in deterministic canonical order.
    pub fn iter(&self) -> impl Iterator<Item = &ResourceRef> {
        self.resources.iter()
    }
}

impl IntoIterator for ResourceSet {
    type Item = ResourceRef;
    type IntoIter = std::collections::btree_set::IntoIter<ResourceRef>;

    fn into_iter(self) -> Self::IntoIter {
        self.resources.into_iter()
    }
}

impl<'a> IntoIterator for &'a ResourceSet {
    type Item = &'a ResourceRef;
    type IntoIter = std::collections::btree_set::Iter<'a, ResourceRef>;

    fn into_iter(self) -> Self::IntoIter {
        self.resources.iter()
    }
}

// ============================================================================
// Resource errors
// ============================================================================

/// Errors produced while constructing or validating a resilience resource
/// value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceError {
    /// The semantic resource kind was empty.
    EmptyKind,

    /// A finite available quantity exceeded finite capacity.
    AvailableExceedsCapacity {
        /// Declared finite capacity.
        capacity: ResourceQuantity,

        /// Declared finite availability.
        available: ResourceQuantity,
    },

    /// A finite resource capacity cannot simultaneously have unbounded
    /// availability.
    UnboundedAvailabilityWithFiniteCapacity,
}

impl fmt::Display for ResourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKind => {
                formatter.write_str("resource kind must not be empty")
            }

            Self::AvailableExceedsCapacity {
                capacity,
                available,
            } => {
                write!(
                    formatter,
                    "resource availability {available:?} exceeds capacity {capacity:?}"
                )
            }

            Self::UnboundedAvailabilityWithFiniteCapacity => {
                formatter.write_str(
                    "unbounded availability cannot be combined with finite capacity",
                )
            }
        }
    }
}

impl std::error::Error for ResourceError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn kind(value: &str) -> ResourceKind {
        ResourceKind::new(value).expect("test resource kind must be valid")
    }

    #[test]
    fn resource_kind_rejects_empty_values() {
        assert_eq!(
            ResourceKind::new(""),
            Err(ResourceError::EmptyKind)
        );

        assert_eq!(
            ResourceKind::new("   "),
            Err(ResourceError::EmptyKind)
        );
    }

    #[test]
    fn resource_kind_preserves_extensibility() {
        let value = ResourceKind::new("future.quantum.resource").unwrap();

        assert_eq!(value.as_str(), "future.quantum.resource");
    }

    #[test]
    fn identity_preserves_logical_and_physical_domains() {
        let logical = QubitId::new(7);
        let physical = PhysicalQubitId::new(7);

        let logical_identity = ResourceIdentity::logical_qubit(logical);
        let physical_identity = ResourceIdentity::physical_qubit(physical);

        assert_ne!(logical_identity, physical_identity);
        assert!(logical_identity.is_logical_qubit());
        assert!(physical_identity.is_physical_qubit());
    }

    #[test]
    fn identity_scope_is_derived_from_identity() {
        let logical = ResourceIdentity::logical_qubit(QubitId::new(1));
        let physical = ResourceIdentity::physical_qubit(PhysicalQubitId::new(1));

        assert_eq!(
            ResourceScope::for_identity(logical),
            ResourceScope::Logical
        );

        assert_eq!(
            ResourceScope::for_identity(physical),
            ResourceScope::Physical
        );
    }

    #[test]
    fn finite_quantities_are_supported_without_machine_constants() {
        assert_eq!(
            ResourceQuantity::finite(1),
            ResourceQuantity::Finite(1)
        );

        assert_eq!(
            ResourceQuantity::finite(u128::MAX),
            ResourceQuantity::Finite(u128::MAX)
        );
    }

    #[test]
    fn unbounded_is_not_an_integer_sentinel() {
        assert!(ResourceQuantity::unbounded().is_unbounded());
        assert!(!ResourceQuantity::unbounded().is_finite());
        assert_ne!(
            ResourceQuantity::unbounded(),
            ResourceQuantity::finite(u128::MAX)
        );
    }

    #[test]
    fn unknown_is_distinct_from_unavailable() {
        assert!(ResourceQuantity::unknown().is_unknown());
        assert_ne!(
            ResourceAvailability::Unknown,
            ResourceAvailability::Unavailable
        );
    }

    #[test]
    fn resource_rejects_availability_above_capacity() {
        let result = Resource::new(
            ResourceIdentity::ir(ResourceId::from(1_u64)),
            kind("execution_slot"),
            ResourceQuantity::finite(4),
            ResourceQuantity::finite(5),
        );

        assert!(matches!(
            result,
            Err(ResourceError::AvailableExceedsCapacity { .. })
        ));
    }

    #[test]
    fn resource_rejects_unbounded_availability_with_finite_capacity() {
        let result = Resource::new(
            ResourceIdentity::ir(ResourceId::from(1_u64)),
            kind("execution_slot"),
            ResourceQuantity::finite(4),
            ResourceQuantity::unbounded(),
        );

        assert_eq!(
            result,
            Err(ResourceError::UnboundedAvailabilityWithFiniteCapacity)
        );
    }

    #[test]
    fn resource_accepts_unknown_availability() {
        let resource = Resource::new(
            ResourceIdentity::ir(ResourceId::from(1_u64)),
            kind("execution_slot"),
            ResourceQuantity::finite(4),
            ResourceQuantity::unknown(),
        )
        .unwrap();

        assert!(resource.available().is_unknown());
        assert_eq!(resource.finite_unavailable(), None);
    }

    #[test]
    fn resource_accepts_unbounded_capacity() {
        let resource = Resource::new(
            ResourceIdentity::ir(ResourceId::from(1_u64)),
            kind("logical_execution"),
            ResourceQuantity::unbounded(),
            ResourceQuantity::unknown(),
        )
        .unwrap();

        assert!(resource.is_unbounded());
        assert!(resource.capacity_is_unknown() == false);
    }

    #[test]
    fn finite_unavailable_is_exact() {
        let resource = Resource::new(
            ResourceIdentity::ir(ResourceId::from(42_u64)),
            kind("qubit_capacity"),
            ResourceQuantity::finite(100),
            ResourceQuantity::finite(73),
        )
        .unwrap();

        assert_eq!(resource.finite_available(), Some(73));
        assert_eq!(resource.finite_unavailable(), Some(27));
    }

    #[test]
    fn exhausted_resource_is_detected_without_thresholds() {
        let resource = Resource::new(
            ResourceIdentity::ir(ResourceId::from(9_u64)),
            kind("execution_slot"),
            ResourceQuantity::finite(8),
            ResourceQuantity::finite(0),
        )
        .unwrap();

        assert!(resource.is_exhausted());
    }

    #[test]
    fn resource_ref_preserves_identity() {
        let id = QubitId::new(11);
        let reference = ResourceRef::logical_qubit(id);

        assert_eq!(
            reference.identity(),
            ResourceIdentity::LogicalQubit(id)
        );
    }

    #[test]
    fn resource_set_is_unique_and_deterministic() {
        let first = ResourceRef::ir(ResourceId::from(2_u64));
        let second = ResourceRef::ir(ResourceId::from(1_u64));

        let mut set = ResourceSet::new();

        assert!(set.insert(first));
        assert!(!set.insert(first));
        assert!(set.insert(second));

        assert_eq!(set.len(), 2);

        let values: Vec<_> = set.iter().copied().collect();

        assert_eq!(values[0], second);
        assert_eq!(values[1], first);
    }

    #[test]
    fn resource_set_can_scale_without_materializing_identifier_ranges() {
        let mut set = ResourceSet::new();

        set.insert(ResourceRef::logical_qubit(QubitId::new(0)));
        set.insert(ResourceRef::logical_qubit(QubitId::new(1_000_000)));
        set.insert(ResourceRef::logical_qubit(QubitId::new(usize::MAX)));

        assert_eq!(set.len(), 3);
    }

    #[test]
    fn resource_scope_matches_identity() {
        let logical = Resource::from_logical_qubit(
            QubitId::new(3),
            kind("logical_qubit"),
            ResourceQuantity::finite(1),
            ResourceQuantity::finite(1),
        )
        .unwrap();

        let physical = Resource::from_physical_qubit(
            PhysicalQubitId::new(3),
            kind("physical_qubit"),
            ResourceQuantity::finite(1),
            ResourceQuantity::finite(1),
        )
        .unwrap();

        assert_eq!(logical.scope(), ResourceScope::Logical);
        assert_eq!(physical.scope(), ResourceScope::Physical);
    }
}