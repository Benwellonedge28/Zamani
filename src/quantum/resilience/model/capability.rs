//! Zamani Quantum Resilience — Effective Capability Model
//!
//! This module defines the resilience-layer representation of the capabilities
//! that are currently available to an execution target after accounting for
//! degradation, faults, resource loss, policy restrictions, and recovery state.
//!
//! # Architectural role
//!
//! The canonical capability vocabulary is owned by:
//!
//!     quantum::ir::resources::capability
//!
//! Resource quantities are owned by:
//!
//!     quantum::ir::resources::resource
//!
//! Canonical logical qubit identity is owned by:
//!
//!     quantum::ir::qubit
//!
//! This module MUST NOT redefine any of those concepts.
//!
//! The resilience capability model answers:
//!
//! > Given the target's canonical capabilities and its current execution
//! > state, what capability is effectively usable right now?
//!
//! It therefore sits between observation/diagnosis and planning/adaptation.
//!
//! ```text
//! canonical target capability
//!             │
//!             ▼
//!      degradation/faults
//!             │
//!             ▼
//!     policy restrictions
//!             │
//!             ▼
//! ┌─────────────────────────────┐
//! │ resilience::model::capability│
//! │                             │
//! │ effective capability state  │
//! └──────────────┬──────────────┘
//!                │
//!                ▼
//!        planning/feasibility
//!                │
//!        ┌───────┴────────┐
//!        ▼                ▼
//!     continue          adapt
//! ```
//!
//! # Important ownership rule
//!
//! This file does NOT:
//!
//! - discover hardware;
//! - allocate hardware;
//! - route logical qubits;
//! - schedule operations;
//! - compile circuits;
//! - implement QEC;
//! - implement error mitigation;
//! - execute a backend;
//! - decide recovery policy;
//! - redefine capability identifiers;
//! - redefine physical qubit identities.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Write once, scale everywhere
//!
//! No fixed:
//!
//! - qubit count;
//! - capability count;
//! - backend count;
//! - topology size;
//! - capability kind enum;
//! - resource type enum;
//! - hardware vendor;
//! - architecture;
//! - recovery strategy;
//! - degradation threshold
//!
//! is encoded here.
//!
//! Collections grow according to available memory and explicit execution or
//! security policies.
//!
//! "Infinity" means that no artificial finite machine-size ceiling is encoded
//! by this model. Actual executions remain finite because the host process,
//! memory, target and execution environment are finite.
//!
//! # Canonical qubit identity
//!
//! Logical-qubit-scoped state uses:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! Physical qubit identity is deliberately NOT stored here. Physical placement
//! belongs to routing and hardware layers.
//!
//! # Determinism
//!
//! Ordered maps and sets are used wherever collections are exposed as semantic
//! state. This makes iteration deterministic and prevents recovery planning
//! from depending on hash-map iteration order.
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe.
//!
//! # Serialization
//!
//! This module owns semantic state only. Canonical wire encoding remains the
//! responsibility of:
//!
//!     quantum::resilience::serialization
//!
//! # Integration
//!
//! Consumers:
//!
//! - `model::health`
//! - `model::degradation`
//! - `model::fault`
//! - `diagnosis`
//! - `policy`
//! - `planning::feasibility`
//! - `planning::planner`
//! - `adaptation`
//! - `recovery`
//! - `verification`
//! - `telemetry`
//!
//! Providers:
//!
//! - `quantum::ir::resources::capability`
//! - `quantum::ir::resources::resource`
//! - `quantum::ir::qubit`
//! - hardware HAL capability discovery
//! - QEC capability reporting
//! - routing/scheduling capability reporting
//!
//! The resilience layer consumes provider state and produces an effective
//! execution view. It does not mutate the provider's canonical capability
//! declaration.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::qubit::QubitId;
use crate::quantum::ir::resources::capability::{
    CapabilityId,
    CapabilitySupport,
    CapabilityVersion,
    VersionConstraint,
};
use crate::quantum::ir::resources::resource::ResourceQuantity;

use super::degradation::Degradation;

// =============================================================================
// Capability scope
// =============================================================================

/// Scope at which an effective capability applies.
///
/// This deliberately mirrors the semantic distinction required by resilience
/// without creating a second qubit identity type.
///
/// Physical placement is not represented here.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityScope {
    /// Capability applies to the complete execution target.
    Global,

    /// Capability applies to one logical qubit.
    LogicalQubit(QubitId),

    /// Capability applies to a deterministic set of logical qubits.
    LogicalQubits(Vec<QubitId>),
}

impl CapabilityScope {
    /// Creates a logical-qubit scope.
    #[must_use]
    pub const fn logical_qubit(qubit: QubitId) -> Self {
        Self::LogicalQubit(qubit)
    }

    /// Creates a normalized logical-qubit scope.
    ///
    /// The input is sorted and duplicate qubits are removed.
    ///
    /// An empty collection is rejected because it has no meaningful scope.
    pub fn logical_qubits<I>(qubits: I) -> Result<Self, CapabilityStateError>
    where
        I: IntoIterator<Item = QubitId>,
    {
        let mut values: Vec<QubitId> = qubits.into_iter().collect();

        values.sort();
        values.dedup();

        if values.is_empty() {
            return Err(CapabilityStateError::EmptyQubitScope);
        }

        Ok(Self::LogicalQubits(values))
    }

    /// Returns whether this is global state.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self, Self::Global)
    }

    /// Returns the logical qubit if this is a single-qubit scope.
    #[must_use]
    pub const fn as_logical_qubit(&self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(qubit) => Some(*qubit),
            _ => None,
        }
    }

    /// Returns the logical qubit collection if this is a multi-qubit scope.
    #[must_use]
    pub fn as_logical_qubits(&self) -> Option<&[QubitId]> {
        match self {
            Self::LogicalQubits(qubits) => Some(qubits),
            _ => None,
        }
    }
}

impl Default for CapabilityScope {
    fn default() -> Self {
        Self::Global
    }
}

// =============================================================================
// Effective support
// =============================================================================

/// Effective resilience-layer support state.
///
/// This is deliberately distinct from a raw hardware declaration.
///
/// A target may canonically support a capability while resilience temporarily
/// marks it degraded or unavailable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EffectiveSupport {
    /// The capability is currently usable without a resilience restriction.
    Available,

    /// The capability remains usable, but with reduced effective capacity or
    /// degraded operating conditions.
    Degraded,

    /// The capability is currently unavailable.
    Unavailable,
}

impl EffectiveSupport {
    /// Returns whether the capability can currently be used.
    #[must_use]
    pub const fn is_usable(self) -> bool {
        matches!(self, Self::Available | Self::Degraded)
    }

    /// Returns whether the capability is completely unavailable.
    #[must_use]
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }

    /// Returns whether the state represents degradation.
    #[must_use]
    pub const fn is_degraded(self) -> bool {
        matches!(self, Self::Degraded)
    }
}

// =============================================================================
// Capability quantity
// =============================================================================

/// Effective capacity associated with a capability.
///
/// `ResourceQuantity` is reused from the canonical IR resource model so that
/// resilience does not introduce a second finite/unbounded resource ontology.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CapabilityCapacity {
    /// Currently usable capacity.
    usable: ResourceQuantity,

    /// Optional total capacity known by the target.
    ///
    /// This is optional because some capabilities do not have a meaningful
    /// scalar capacity and some backends cannot expose their total capacity.
    total: Option<ResourceQuantity>,
}

impl CapabilityCapacity {
    /// Creates a capacity with a known usable amount and no total.
    #[must_use]
    pub const fn usable(usable: ResourceQuantity) -> Self {
        Self {
            usable,
            total: None,
        }
    }

    /// Creates a capacity with usable and total quantities.
    ///
    /// When both quantities are finite, `usable` must not exceed `total`.
    pub fn with_total(
        usable: ResourceQuantity,
        total: ResourceQuantity,
    ) -> Result<Self, CapabilityStateError> {
        if let (
            ResourceQuantity::Finite(usable_value),
            ResourceQuantity::Finite(total_value),
        ) = (usable, total)
        {
            if usable_value > total_value {
                return Err(CapabilityStateError::UsableExceedsTotal {
                    usable: usable_value,
                    total: total_value,
                });
            }
        }

        Ok(Self {
            usable,
            total: Some(total),
        })
    }

    /// Returns the currently usable quantity.
    #[must_use]
    pub const fn usable(self) -> ResourceQuantity {
        self.usable
    }

    /// Returns the total quantity, if known.
    #[must_use]
    pub const fn total(self) -> Option<ResourceQuantity> {
        self.total
    }

    /// Returns whether the usable quantity is unbounded.
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        self.usable.is_unbounded()
    }

    /// Returns the finite usable quantity, if available.
    #[must_use]
    pub const fn usable_finite(self) -> Option<u64> {
        self.usable.as_finite()
    }
}

// =============================================================================
// Capability key
// =============================================================================

/// Deterministic key for one effective capability state.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CapabilityKey {
    id: CapabilityId,
    scope: CapabilityScope,
}

impl CapabilityKey {
    /// Creates a capability key.
    #[must_use]
    pub fn new(id: CapabilityId, scope: CapabilityScope) -> Self {
        Self { id, scope }
    }

    /// Returns the canonical capability identifier.
    #[must_use]
    pub fn id(&self) -> &CapabilityId {
        &self.id
    }

    /// Returns the resilience scope.
    #[must_use]
    pub fn scope(&self) -> &CapabilityScope {
        &self.scope
    }
}

// =============================================================================
// Effective capability
// =============================================================================

/// Effective capability state observed by the resilience planner.
///
/// This type is deliberately a *view* over canonical capability semantics.
/// It does not replace the canonical IR `Capability`.
///
/// The state can therefore express:
///
/// ```text
/// canonical capability exists
///          │
///          ├── available
///          ├── degraded
///          └── unavailable
/// ```
///
/// without changing the program's semantic capability requirement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EffectiveCapability {
    key: CapabilityKey,
    version: CapabilityVersion,
    support: CapabilitySupport,
    effective_support: EffectiveSupport,
    capacity: Option<CapabilityCapacity>,
    degradation: Option<Degradation>,
}

impl EffectiveCapability {
    /// Creates an effective capability state.
    ///
    /// Validation ensures that an unavailable capability cannot simultaneously
    /// advertise usable capacity.
    pub fn new(
        id: CapabilityId,
        scope: CapabilityScope,
        version: CapabilityVersion,
        support: CapabilitySupport,
        effective_support: EffectiveSupport,
        capacity: Option<CapabilityCapacity>,
        degradation: Option<Degradation>,
    ) -> Result<Self, CapabilityStateError> {
        if matches!(effective_support, EffectiveSupport::Unavailable)
            && capacity
                .and_then(CapabilityCapacity::usable_finite)
                .is_some_and(|value| value > 0)
        {
            return Err(CapabilityStateError::UnavailableHasUsableCapacity);
        }

        if matches!(effective_support, EffectiveSupport::Available)
            && matches!(degradation, Some(ref value) if !value.is_zero())
        {
            return Err(CapabilityStateError::AvailableHasDegradation);
        }

        Ok(Self {
            key: CapabilityKey::new(id, scope),
            version,
            support,
            effective_support,
            capacity,
            degradation,
        })
    }

    /// Creates an unrestricted available capability.
    pub fn available(
        id: CapabilityId,
        scope: CapabilityScope,
        version: CapabilityVersion,
    ) -> Result<Self, CapabilityStateError> {
        Self::new(
            id,
            scope,
            version,
            CapabilitySupport::Supported,
            EffectiveSupport::Available,
            None,
            None,
        )
    }

    /// Creates an unavailable capability.
    pub fn unavailable(
        id: CapabilityId,
        scope: CapabilityScope,
        version: CapabilityVersion,
    ) -> Result<Self, CapabilityStateError> {
        Self::new(
            id,
            scope,
            version,
            CapabilitySupport::Unsupported,
            EffectiveSupport::Unavailable,
            None,
            None,
        )
    }

    /// Returns the canonical capability identifier.
    #[must_use]
    pub fn id(&self) -> &CapabilityId {
        self.key.id()
    }

    /// Returns the capability scope.
    #[must_use]
    pub fn scope(&self) -> &CapabilityScope {
        self.key.scope()
    }

    /// Returns the capability key.
    #[must_use]
    pub fn key(&self) -> &CapabilityKey {
        &self.key
    }

    /// Returns the declared canonical support state.
    #[must_use]
    pub const fn support(&self) -> CapabilitySupport {
        self.support
    }

    /// Returns the currently effective resilience state.
    #[must_use]
    pub const fn effective_support(&self) -> EffectiveSupport {
        self.effective_support
    }

    /// Returns the semantic capability version.
    #[must_use]
    pub const fn version(&self) -> CapabilityVersion {
        self.version
    }

    /// Returns the effective capacity, if one exists.
    #[must_use]
    pub const fn capacity(&self) -> Option<CapabilityCapacity> {
        self.capacity
    }

    /// Returns the degradation record, if one exists.
    #[must_use]
    pub fn degradation(&self) -> Option<&Degradation> {
        self.degradation.as_ref()
    }

    /// Returns whether the capability is currently usable.
    #[must_use]
    pub const fn is_usable(&self) -> bool {
        self.effective_support.is_usable()
    }

    /// Returns whether the capability is currently unavailable.
    #[must_use]
    pub const fn is_unavailable(&self) -> bool {
        self.effective_support.is_unavailable()
    }

    /// Returns whether the capability is currently degraded.
    #[must_use]
    pub const fn is_degraded(&self) -> bool {
        self.effective_support.is_degraded()
    }

    /// Returns whether the capability satisfies a version constraint.
    #[must_use]
    pub const fn matches_version(&self, constraint: VersionConstraint) -> bool {
        constraint.matches(self.version)
    }

    /// Returns a copy with a new effective state.
    ///
    /// The canonical capability identity/version remain unchanged.
    pub fn with_effective_state(
        self,
        effective_support: EffectiveSupport,
        capacity: Option<CapabilityCapacity>,
        degradation: Option<Degradation>,
    ) -> Result<Self, CapabilityStateError> {
        Self::new(
            self.key.id,
            self.key.scope,
            self.version,
            self.support,
            effective_support,
            capacity,
            degradation,
        )
    }
}

// =============================================================================
// Capability profile
// =============================================================================

/// Deterministic collection of effective capabilities.
///
/// A profile is the resilience-layer snapshot of what is currently usable.
///
/// It is intentionally not a hardware registry. Hardware discovery belongs to
/// the hardware subsystem.
///
/// A profile may contain arbitrarily many capabilities subject only to
/// available memory and caller policy.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CapabilityProfile {
    capabilities: BTreeMap<CapabilityKey, EffectiveCapability>,
}

impl CapabilityProfile {
    /// Creates an empty capability profile.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts or replaces an effective capability.
    ///
    /// Replacement is deterministic because the key is canonical.
    pub fn insert(&mut self, capability: EffectiveCapability) -> Option<EffectiveCapability> {
        self.capabilities
            .insert(capability.key.clone(), capability)
    }

    /// Removes a capability by key.
    pub fn remove(&mut self, key: &CapabilityKey) -> Option<EffectiveCapability> {
        self.capabilities.remove(key)
    }

    /// Returns a capability by exact key.
    #[must_use]
    pub fn get(&self, key: &CapabilityKey) -> Option<&EffectiveCapability> {
        self.capabilities.get(key)
    }

    /// Returns the capability with a global scope.
    #[must_use]
    pub fn get_global(&self, id: &CapabilityId) -> Option<&EffectiveCapability> {
        self.get(&CapabilityKey::new(
            id.clone(),
            CapabilityScope::Global,
        ))
    }

    /// Returns the capability scoped to one logical qubit.
    #[must_use]
    pub fn get_logical_qubit(
        &self,
        id: &CapabilityId,
        qubit: QubitId,
    ) -> Option<&EffectiveCapability> {
        self.get(&CapabilityKey::new(
            id.clone(),
            CapabilityScope::LogicalQubit(qubit),
        ))
    }

    /// Returns the number of capability entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.capabilities.len()
    }

    /// Returns whether the profile contains no capabilities.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
    }

    /// Iterates deterministically over all capability entries.
    pub fn iter(&self) -> impl Iterator<Item = (&CapabilityKey, &EffectiveCapability)> {
        self.capabilities.iter()
    }

    /// Returns whether an exact capability is currently usable.
    #[must_use]
    pub fn is_usable(&self, key: &CapabilityKey) -> bool {
        self.get(key)
            .is_some_and(EffectiveCapability::is_usable)
    }

    /// Returns all currently usable capabilities.
    ///
    /// The returned iterator follows deterministic key ordering.
    pub fn usable(
        &self,
    ) -> impl Iterator<Item = (&CapabilityKey, &EffectiveCapability)> {
        self.capabilities
            .iter()
            .filter(|(_, capability)| capability.is_usable())
    }

    /// Returns all currently degraded capabilities.
    pub fn degraded(
        &self,
    ) -> impl Iterator<Item = (&CapabilityKey, &EffectiveCapability)> {
        self.capabilities
            .iter()
            .filter(|(_, capability)| capability.is_degraded())
    }

    /// Returns all currently unavailable capabilities.
    pub fn unavailable(
        &self,
    ) -> impl Iterator<Item = (&CapabilityKey, &EffectiveCapability)> {
        self.capabilities
            .iter()
            .filter(|(_, capability)| capability.is_unavailable())
    }

    /// Applies a deterministic state transformation to one capability.
    ///
    /// The caller supplies the new effective state. This module deliberately
    /// does not decide *why* the state changed.
    pub fn update(
        &mut self,
        key: &CapabilityKey,
        effective_support: EffectiveSupport,
        capacity: Option<CapabilityCapacity>,
        degradation: Option<Degradation>,
    ) -> Result<(), CapabilityStateError> {
        let existing = self
            .capabilities
            .get(key)
            .ok_or_else(|| CapabilityStateError::CapabilityNotFound {
                capability: key.id.clone(),
            })?;

        let updated = existing.clone().with_effective_state(
            effective_support,
            capacity,
            degradation,
        )?;

        self.capabilities.insert(key.clone(), updated);

        Ok(())
    }

    /// Validates the complete profile.
    ///
    /// This checks internal invariants but deliberately does not decide whether
    /// the profile is sufficient for a particular program. That decision
    /// belongs to `planning::feasibility`.
    pub fn validate(&self) -> Result<(), CapabilityStateError> {
        for capability in self.capabilities.values() {
            validate_capability(capability)?;
        }

        Ok(())
    }
}

// =============================================================================
// Capability requirement check
// =============================================================================

/// Result of checking one capability requirement against an effective profile.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapabilityAvailability {
    /// The capability is directly usable.
    Available,

    /// The capability exists but is degraded.
    Degraded,

    /// The capability exists but is not usable.
    Unavailable,

    /// The capability is absent from the profile.
    Missing,

    /// The capability exists but its version is incompatible.
    VersionMismatch {
        /// Version currently provided.
        provided: CapabilityVersion,
    },
}

impl CapabilityAvailability {
    /// Returns whether the capability can be used.
    #[must_use]
    pub const fn is_usable(&self) -> bool {
        matches!(self, Self::Available | Self::Degraded)
    }

    /// Returns whether the result represents absence.
    #[must_use]
    pub const fn is_missing(&self) -> bool {
        matches!(self, Self::Missing)
    }
}

/// Checks whether a specific capability is available with a required version.
///
/// This function is intentionally small and policy-neutral. It does not decide
/// whether degraded support is acceptable; callers choose that through policy.
#[must_use]
pub fn check_capability(
    profile: &CapabilityProfile,
    id: &CapabilityId,
    scope: &CapabilityScope,
    version: VersionConstraint,
) -> CapabilityAvailability {
    let key = CapabilityKey::new(id.clone(), scope.clone());

    let Some(capability) = profile.get(&key) else {
        return CapabilityAvailability::Missing;
    };

    if !capability.matches_version(version) {
        return CapabilityAvailability::VersionMismatch {
            provided: capability.version(),
        };
    }

    match capability.effective_support() {
        EffectiveSupport::Available => CapabilityAvailability::Available,
        EffectiveSupport::Degraded => CapabilityAvailability::Degraded,
        EffectiveSupport::Unavailable => CapabilityAvailability::Unavailable,
    }
}

// =============================================================================
// Validation
// =============================================================================

fn validate_capability(
    capability: &EffectiveCapability,
) -> Result<(), CapabilityStateError> {
    if capability.is_unavailable() {
        if capability
            .capacity()
            .and_then(CapabilityCapacity::usable_finite)
            .is_some_and(|value| value > 0)
        {
            return Err(CapabilityStateError::UnavailableHasUsableCapacity);
        }
    }

    if capability.is_usable() && capability.support() == CapabilitySupport::Unsupported {
        return Err(CapabilityStateError::UsableCapabilityMarkedUnsupported);
    }

    if capability.effective_support() == EffectiveSupport::Available
        && capability
            .degradation()
            .is_some_and(|degradation| !degradation.is_zero())
    {
        return Err(CapabilityStateError::AvailableHasDegradation);
    }

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing or validating resilience capability state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapabilityStateError {
    /// A logical-qubit collection was empty.
    EmptyQubitScope,

    /// Usable capacity exceeds declared total capacity.
    UsableExceedsTotal {
        /// Effective usable quantity.
        usable: u64,

        /// Declared total quantity.
        total: u64,
    },

    /// An unavailable capability advertised usable capacity.
    UnavailableHasUsableCapacity,

    /// A capability marked available simultaneously contained degradation.
    AvailableHasDegradation,

    /// A usable capability was declared unsupported by its canonical provider.
    UsableCapabilityMarkedUnsupported,

    /// An update referenced a capability absent from the profile.
    CapabilityNotFound {
        /// Missing capability identifier.
        capability: CapabilityId,
    },
}

impl fmt::Display for CapabilityStateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyQubitScope => {
                formatter.write_str("logical-qubit capability scope cannot be empty")
            }

            Self::UsableExceedsTotal { usable, total } => {
                write!(
                    formatter,
                    "capability usable capacity {usable} exceeds total capacity {total}"
                )
            }

            Self::UnavailableHasUsableCapacity => {
                formatter.write_str(
                    "an unavailable capability cannot advertise usable capacity",
                )
            }

            Self::AvailableHasDegradation => {
                formatter.write_str(
                    "an available capability cannot carry a non-zero degradation",
                )
            }

            Self::UsableCapabilityMarkedUnsupported => {
                formatter.write_str(
                    "a usable capability cannot be marked unsupported by its provider",
                )
            }

            Self::CapabilityNotFound { capability } => {
                write!(
                    formatter,
                    "capability is not present in resilience profile: {capability}"
                )
            }
        }
    }
}

impl std::error::Error for CapabilityStateError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn capability_id() -> CapabilityId {
        CapabilityId::new(
            "zamani.quantum",
            "mid_circuit_measurement",
        )
        .expect("valid capability identifier")
    }

    fn version() -> CapabilityVersion {
        CapabilityVersion::new(1, 0, 0)
    }

    #[test]
    fn global_capability_is_available() {
        let capability = EffectiveCapability::available(
            capability_id(),
            CapabilityScope::Global,
            version(),
        )
        .expect("valid capability");

        assert!(capability.is_usable());
        assert!(!capability.is_degraded());
        assert!(!capability.is_unavailable());
    }

    #[test]
    fn logical_qubit_scope_is_normalized() {
        let scope = CapabilityScope::logical_qubits([
            QubitId::from(3_u64),
            QubitId::from(1_u64),
            QubitId::from(3_u64),
        ])
        .expect("non-empty scope");

        assert_eq!(
            scope.as_logical_qubits(),
            Some(&[QubitId::from(1_u64), QubitId::from(3_u64)][..])
        );
    }

    #[test]
    fn empty_logical_scope_is_rejected() {
        let result = CapabilityScope::logical_qubits(
            std::iter::empty::<QubitId>(),
        );

        assert_eq!(
            result,
            Err(CapabilityStateError::EmptyQubitScope)
        );
    }

    #[test]
    fn capacity_cannot_exceed_total() {
        let result = CapabilityCapacity::with_total(
            ResourceQuantity::Finite(11),
            ResourceQuantity::Finite(10),
        );

        assert_eq!(
            result,
            Err(CapabilityStateError::UsableExceedsTotal {
                usable: 11,
                total: 10,
            })
        );
    }

    #[test]
    fn unbounded_capacity_is_supported() {
        let capacity = CapabilityCapacity::with_total(
            ResourceQuantity::Unbounded,
            ResourceQuantity::Unbounded,
        )
        .expect("unbounded capacity is valid");

        assert!(capacity.is_unbounded());
        assert_eq!(capacity.usable_finite(), None);
    }

    #[test]
    fn profile_is_deterministic() {
        let mut profile = CapabilityProfile::new();

        let second = CapabilityId::new(
            "zamani.quantum",
            "b",
        )
        .expect("valid identifier");

        let first = CapabilityId::new(
            "zamani.quantum",
            "a",
        )
        .expect("valid identifier");

        profile.insert(
            EffectiveCapability::available(
                second,
                CapabilityScope::Global,
                version(),
            )
            .expect("valid capability"),
        );

        profile.insert(
            EffectiveCapability::available(
                first,
                CapabilityScope::Global,
                version(),
            )
            .expect("valid capability"),
        );

        let names: Vec<String> = profile
            .iter()
            .map(|(_, capability)| capability.id().qualified_name())
            .collect();

        assert_eq!(
            names,
            vec![
                "zamani.quantum.a".to_owned(),
                "zamani.quantum.b".to_owned(),
            ]
        );
    }

    #[test]
    fn missing_capability_is_distinguished_from_unavailable() {
        let profile = CapabilityProfile::new();
        let id = capability_id();

        assert_eq!(
            check_capability(
                &profile,
                &id,
                &CapabilityScope::Global,
                VersionConstraint::Any,
            ),
            CapabilityAvailability::Missing
        );
    }

    #[test]
    fn version_constraint_is_checked() {
        let mut profile = CapabilityProfile::new();

        profile.insert(
            EffectiveCapability::available(
                capability_id(),
                CapabilityScope::Global,
                version(),
            )
            .expect("valid capability"),
        );

        assert_eq!(
            check_capability(
                &profile,
                &capability_id(),
                &CapabilityScope::Global,
                VersionConstraint::Exact(CapabilityVersion::new(2, 0, 0)),
            ),
            CapabilityAvailability::VersionMismatch {
                provided: version(),
            }
        );
    }

    #[test]
    fn unavailable_capability_is_not_usable() {
        let capability = EffectiveCapability::unavailable(
            capability_id(),
            CapabilityScope::Global,
            version(),
        )
        .expect("valid capability");

        assert!(!capability.is_usable());
        assert!(capability.is_unavailable());
    }
}