//! Zamani Quantum Noise (ZQN) — Leakage Faults.
//!
//! # Purpose
//!
//! This module provides the leakage-specific semantic layer for ZQN faults.
//!
//! A leakage fault represents population leaving the intended computational
//! subspace of a quantum resource and entering an explicitly identified
//! non-computational state, level, mode, manifold, or other leakage domain.
//!
//! This module is intentionally a specialization of the canonical
//! `crate::quantum::zqn::fault::fault::Fault` model.
//!
//! It MUST NOT introduce a competing fault representation.
//!
//! # Architectural ownership
//!
//! This file owns:
//!
//! - leakage destination semantics;
//! - leakage-specific validation;
//! - leakage fault construction;
//! - leakage inspection helpers;
//! - conversion between leakage semantics and the canonical `Fault` model;
//! - leakage-specific predicates.
//!
//! This file does NOT own:
//!
//! - canonical quantum-resource identity;
//! - canonical `QubitId`;
//! - canonical `PhysicalQubitId`;
//! - generic `Fault` semantics;
//! - generic `FaultLocation` semantics;
//! - probability distributions;
//! - random-number generation;
//! - noise-model generation;
//! - quantum channels;
//! - calibration;
//! - characterization;
//! - QEC decoding;
//! - routing;
//! - scheduling;
//! - hardware APIs;
//! - serialization;
//! - resource-policy limits.
//!
//! Those responsibilities remain in their respective modules.
//!
//! # Canonical fault integration
//!
//! The canonical fault layer defines leakage as:
//!
//! ```text
//! FaultEffect::Leakage { destination: ... }
//! ```
//!
//! Therefore this module MUST construct and inspect that canonical effect
//! rather than defining another `LeakageFault` object that duplicates `Fault`.
//!
//! The canonical fault layer also establishes that specialized
//! `fault/leakage.rs` is responsible for specialized leakage construction and
//! validation. This file implements that contract.
//!
//! # Canonical quantum identities
//!
//! When a leakage fault refers to a logical or physical qubit, the canonical
//! identities from:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! MUST be used.
//!
//! In particular:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No ZQN-specific qubit identifier is introduced here.
//!
//! # Write once, scale everywhere
//!
//! Leakage has no semantic maximum number of qubits, resources, levels, or
//! leakage destinations.
//!
//! This module deliberately contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_LEAKAGE_LEVELS
//! MAX_LEAKAGE_DESTINATIONS
//! MAX_FAULTS
//! ```
//!
//! Any actual resource limit belongs to the caller's explicit resource policy,
//! execution context, target capability, or `ZqnLimits` implementation.
//!
//! "Infinity" means that this semantic layer does not encode an artificial
//! finite machine-size ceiling. Actual execution remains bounded by available
//! resources.
//!
//! # Leakage is not loss
//!
//! Leakage and loss are distinct physical concepts.
//!
//! ```text
//! leakage
//!     population remains associated with the physical system but leaves
//!     the intended computational subspace.
//!
//! loss
//!     the relevant population/resource is physically lost or unavailable.
//! ```
//!
//! A backend or noise model MUST NOT silently convert one into the other.
//!
//! If a physical system can exhibit both effects, they should be represented
//! as separate fault semantics or through an explicitly defined composite
//! model.
//!
//! # Leakage is not erasure
//!
//! Leakage and erasure are also distinct.
//!
//! An erasure model may intentionally discard information about the original
//! state while preserving an erasure flag. Leakage describes the physical
//! departure from the computational subspace.
//!
//! A downstream QEC adapter may map leakage into an erasure-like representation
//! when the selected code and hardware model justify that transformation, but
//! this module must not make that assumption.
//!
//! # Leakage destination
//!
//! The leakage destination is represented as an opaque ZQN identifier rather
//! than as a fixed integer level or fixed two-level extension.
//!
//! This permits future representations such as:
//!
//! - a single non-computational level;
//! - an arbitrary energy level;
//! - a manifold;
//! - a mode;
//! - a bosonic occupation sector;
//! - a technology-defined state;
//! - an externally characterized leakage domain.
//!
//! The identifier itself does not establish that the destination physically
//! exists. Existence and target compatibility belong to the target/calibration
//! layers.
//!
//! # Determinism
//!
//! This module performs no random sampling.
//!
//! It does not:
//!
//! - create RNGs;
//! - access global RNG state;
//! - use thread-local randomness;
//! - inspect system time;
//! - depend on memory addresses;
//! - depend on hash-map iteration order;
//! - maintain mutable global state.
//!
//! A noise model may deterministically construct a leakage fault from a
//! supplied sampling context and seed policy.
//!
//! Once constructed, the leakage semantics are immutable.
//!
//! # Resource safety
//!
//! This module does not introduce implicit allocation proportional to machine
//! size.
//!
//! A single leakage destination is represented by a single opaque identifier.
//!
//! Validation is constant-time with respect to the number of quantum resources
//! represented elsewhere in the computation.
//!
//! Composite fault locations remain owned by `FaultLocation` and are subject
//! to the caller's explicit resource policy.
//!
//! # Numerical safety
//!
//! This module does not store leakage probabilities.
//!
//! Probabilities belong to the canonical ZQN probability subsystem and noise
//! models.
//!
//! If a leakage probability is required, it must be represented using the
//! canonical probability abstraction rather than a private `f64` field.
//!
//! # Serialization
//!
//! This module does not define a serialization format.
//!
//! `zqn::io` owns serialization, schema versioning, canonical encoding, and
//! compatibility.
//!
//! A serialized canonical leakage fault must preserve the distinction between:
//!
//! ```text
//! FaultEffect::Leakage
//! ```
//!
//! and other fault effects.
//!
//! The destination identity must remain typed by schema context.
//!
//! # Security
//!
//! Leakage destination identifiers are data, not capabilities.
//!
//! Constructing a leakage fault MUST NOT grant:
//!
//! - hardware access;
//! - QPU execution rights;
//! - calibration write access;
//! - credentials;
//! - filesystem access;
//! - network access.
//!
//! Untrusted leakage specifications must be validated before being materialized
//! into large composite fault structures by higher-level callers.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Integration graph
//!
//! ```text
//! quantum::ir::qubit
//!          │
//!          ├── QubitId
//!          └── PhysicalQubitId
//!
//! zqn::core::ids
//!          │
//!          └── ZqnIdValue
//!
//!          ▼
//! zqn::fault::fault
//!          │
//!          ├── Fault
//!          ├── FaultLocation
//!          └── FaultEffect::Leakage
//!                    ▲
//!                    │
//!          zqn::fault::leakage
//!                    │
//!          ┌─────────┼──────────┐
//!          ▼         ▼          ▼
//!        noise      QEC       simulation
//!          │         │          │
//!          └─────────┼──────────┘
//!                    ▼
//!                 runtime
//! ```
//!
//! The dependency direction is one-way:
//!
//! ```text
//! canonical Fault
//!       ▲
//!       │
//! leakage specialization
//! ```
//!
//! The generic fault module must not depend on this specialization.
//!
//! # Definition of done
//!
//! This module is complete when:
//!
//! 1. leakage uses the canonical `Fault` representation;
//! 2. leakage uses the canonical `FaultLocation` representation;
//! 3. leakage uses canonical Quantum IR qubit identities;
//! 4. leakage destination identity is not confused with a qubit identity;
//! 5. no finite machine-size ceiling is encoded;
//! 6. no RNG or global mutable state is used;
//! 7. invalid leakage destinations are rejected deterministically;
//! 8. leakage cannot silently become loss or erasure;
//! 9. construction does not require later changes to `leakage.rs` when
//!    downstream consumers are added;
//! 10. all behavior remains compatible with Rust 1.97/1.97.1;
//! 11. the implementation contains no unsafe code;
//! 12. the module can be consumed by noise, simulation, QEC, and runtime
//!     adapters without redefining leakage semantics.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::core::errors::{ZqnError, ZqnResult};
use crate::quantum::zqn::core::ids::ZqnIdValue;
use crate::quantum::zqn::fault::fault::{
    Fault,
    FaultEffect,
    FaultLocation,
};

// ============================================================================
// Leakage destination
// ============================================================================

/// Opaque identity of a non-computational leakage destination.
///
/// A leakage destination is deliberately not represented as a fixed numeric
/// energy level because quantum technologies differ in how non-computational
/// states are represented.
///
/// Examples include:
///
/// - a higher energy level;
/// - an unwanted state manifold;
/// - a bosonic occupation sector;
/// - a mode;
/// - a technology-defined leakage state.
///
/// The numeric value is an opaque identity and must not be interpreted as a
/// physical energy, array index, qubit count, or hardware address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LeakageDestination(ZqnIdValue);

impl LeakageDestination {
    /// Creates a leakage destination from an explicit opaque identity.
    ///
    /// This constructor does not assert that the destination physically
    /// exists. Target validation belongs to the target/capability layer.
    #[must_use]
    pub const fn new(value: ZqnIdValue) -> Self {
        Self(value)
    }

    /// Returns the opaque destination identity.
    #[must_use]
    pub const fn value(self) -> ZqnIdValue {
        self.0
    }

    /// Validates the destination identity.
    ///
    /// Zero is a valid opaque identity. The identity domain, rather than the
    /// numeric value, determines its meaning.
    pub const fn validate(self) -> ZqnResult<()> {
        Ok(())
    }
}

impl From<ZqnIdValue> for LeakageDestination {
    fn from(value: ZqnIdValue) -> Self {
        Self::new(value)
    }
}

impl From<LeakageDestination> for ZqnIdValue {
    fn from(destination: LeakageDestination) -> Self {
        destination.value()
    }
}

// ============================================================================
// Leakage fault helpers
// ============================================================================

/// Returns whether a canonical fault is a leakage fault.
///
/// This is intentionally a non-consuming predicate so callers can inspect
/// faults without cloning or converting them.
///
/// # Integration
///
/// Use this from:
///
/// - noise realization;
/// - simulation;
/// - QEC adapters;
/// - benchmarking;
/// - runtime telemetry.
///
/// The caller remains responsible for deciding what a leakage fault means for
/// its own subsystem.
#[must_use]
pub fn is_leakage(fault: &Fault) -> bool {
    matches!(fault.effect(), FaultEffect::Leakage { .. })
}

/// Extracts the leakage destination from a canonical fault.
///
/// Returns `None` when the supplied fault is not a leakage fault.
#[must_use]
pub fn destination(fault: &Fault) -> Option<LeakageDestination> {
    match fault.effect() {
        FaultEffect::Leakage { destination } => {
            Some(LeakageDestination::new(*destination))
        }
        _ => None,
    }
}

/// Validates the leakage-specific semantics of a canonical fault.
///
/// Generic structural validation remains owned by `Fault::validate()`.
/// This function adds leakage-specific validation only.
pub fn validate(fault: &Fault) -> ZqnResult<()> {
    fault.validate()?;

    match fault.effect() {
        FaultEffect::Leakage { destination } => {
            LeakageDestination::new(*destination).validate()
        }
        _ => Err(ZqnError::invalid_fault_effect(
            "fault is not a leakage effect",
        )),
    }
}

// ============================================================================
// Canonical leakage construction
// ============================================================================

/// Constructs a canonical leakage fault for a logical qubit.
///
/// The returned value is the repository's canonical `Fault`; no competing
/// leakage-fault structure is introduced.
///
/// The caller supplies the `FaultId` because ZQN identity allocation belongs
/// to the owning generator/registry rather than this helper.
///
/// # Integration
///
/// ```text
/// noise model
///     │
///     ├── chooses destination
///     ├── chooses FaultId
///     │
///     ▼
/// make_logical(...)
///     │
///     ▼
/// canonical Fault
///     │
///     ├── simulator
///     ├── QEC adapter
///     ├── runtime
///     └── benchmarking
/// ```
pub fn make_logical(
    id: crate::quantum::zqn::core::ids::FaultId,
    qubit: QubitId,
    leakage_destination: LeakageDestination,
) -> ZqnResult<Fault> {
    leakage_destination.validate()?;

    let fault = Fault::new(
        id,
        FaultLocation::logical_qubit(qubit),
        crate::quantum::zqn::fault::fault::FaultClassification::Leakage,
        FaultEffect::Leakage {
            destination: leakage_destination.value(),
        },
    )?;

    validate(&fault)?;
    Ok(fault)
}

/// Constructs a canonical leakage fault for a physical qubit.
///
/// The physical qubit identity comes directly from
/// `quantum::ir::qubit::PhysicalQubitId`.
pub fn make_physical(
    id: crate::quantum::zqn::core::ids::FaultId,
    qubit: PhysicalQubitId,
    leakage_destination: LeakageDestination,
) -> ZqnResult<Fault> {
    leakage_destination.validate()?;

    let fault = Fault::new(
        id,
        FaultLocation::physical_qubit(qubit),
        crate::quantum::zqn::fault::fault::FaultClassification::Leakage,
        FaultEffect::Leakage {
            destination: leakage_destination.value(),
        },
    )?;

    validate(&fault)?;
    Ok(fault)
}

/// Constructs a leakage fault at an arbitrary canonical fault location.
///
/// This is the most general constructor and should be preferred by technology
/// integrations that are not naturally represented as a qubit.
pub fn make_at(
    id: crate::quantum::zqn::core::ids::FaultId,
    location: FaultLocation,
    leakage_destination: LeakageDestination,
) -> ZqnResult<Fault> {
    leakage_destination.validate()?;
    location.validate()?;

    let fault = Fault::new(
        id,
        location,
        crate::quantum::zqn::fault::fault::FaultClassification::Leakage,
        FaultEffect::Leakage {
            destination: leakage_destination.value(),
        },
    )?;

    validate(&fault)?;
    Ok(fault)
}

// ============================================================================
// Leakage classification helpers
// ============================================================================

/// Returns `true` when the supplied fault represents leakage from the
/// computational subspace.
///
/// This function is equivalent to [`is_leakage`] and exists as a semantically
/// explicit API for consumers performing fault classification.
#[must_use]
pub fn is_computational_subspace_escape(fault: &Fault) -> bool {
    is_leakage(fault)
}

/// Returns the destination identity when the fault represents leakage.
///
/// This is an alias with domain-specific terminology useful to physical-noise
/// and characterization consumers.
#[must_use]
pub fn leaked_to(fault: &Fault) -> Option<LeakageDestination> {
    destination(fault)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::zqn::core::ids::FaultId;
    use crate::quantum::zqn::fault::fault::FaultClassification;

    fn fault_id() -> FaultId {
        FaultId::new(1)
    }

    fn destination_id() -> LeakageDestination {
        LeakageDestination::new(7)
    }

    #[test]
    fn destination_preserves_opaque_identity() {
        let destination = LeakageDestination::new(42);

        assert_eq!(destination.value(), 42);
    }

    #[test]
    fn destination_zero_is_valid_opaque_identity() {
        let destination = LeakageDestination::new(0);

        assert!(destination.validate().is_ok());
    }

    #[test]
    fn logical_leakage_uses_canonical_qubit_identity() {
        let qubit = QubitId::new(3);

        let fault = make_logical(fault_id(), qubit, destination_id())
            .expect("logical leakage fault should be valid");

        assert!(is_leakage(&fault));
        assert_eq!(destination(&fault), Some(destination_id()));
        assert_eq!(
            fault.location(),
            &FaultLocation::LogicalQubit(qubit)
        );
    }

    #[test]
    fn physical_leakage_uses_canonical_physical_identity() {
        let qubit = PhysicalQubitId::new(5);

        let fault = make_physical(fault_id(), qubit, destination_id())
            .expect("physical leakage fault should be valid");

        assert!(is_leakage(&fault));
        assert_eq!(destination(&fault), Some(destination_id()));
        assert_eq!(
            fault.location(),
            &FaultLocation::PhysicalQubit(qubit)
        );
    }

    #[test]
    fn arbitrary_location_can_represent_leakage() {
        let location = FaultLocation::zqn_resource(100);

        let fault = make_at(fault_id(), location, destination_id())
            .expect("arbitrary leakage location should be valid");

        assert!(is_leakage(&fault));
        assert_eq!(destination(&fault), Some(destination_id()));
    }

    #[test]
    fn non_leakage_fault_is_not_classified_as_leakage() {
        let fault = Fault::new(
            fault_id(),
            FaultLocation::logical_qubit(QubitId::new(1)),
            FaultClassification::Gate,
            FaultEffect::Custom("test".to_owned()),
        )
        .expect("test fault should be valid");

        assert!(!is_leakage(&fault));
        assert_eq!(destination(&fault), None);
    }

    #[test]
    fn leakage_validation_accepts_canonical_fault() {
        let fault = make_logical(
            fault_id(),
            QubitId::new(1),
            LeakageDestination::new(9),
        )
        .expect("leakage fault should be valid");

        assert!(validate(&fault).is_ok());
    }

    #[test]
    fn leakage_validation_rejects_non_leakage_fault() {
        let fault = Fault::new(
            fault_id(),
            FaultLocation::logical_qubit(QubitId::new(1)),
            FaultClassification::Gate,
            FaultEffect::Custom("test".to_owned()),
        )
        .expect("test fault should be valid");

        assert!(validate(&fault).is_err());
    }

    #[test]
    fn leakage_destination_is_copyable_and_orderable() {
        let first = LeakageDestination::new(1);
        let second = LeakageDestination::new(2);

        assert!(first < second);
        assert_eq!(first, LeakageDestination::new(1));
    }

    #[test]
    fn leakage_has_no_randomness() {
        let first = make_logical(
            FaultId::new(10),
            QubitId::new(4),
            LeakageDestination::new(8),
        )
        .expect("first fault should be valid");

        let second = make_logical(
            FaultId::new(10),
            QubitId::new(4),
            LeakageDestination::new(8),
        )
        .expect("second fault should be valid");

        assert_eq!(first, second);
    }
}