//! Zamani Quantum Noise (ZQN) — Erasure Fault Semantics.
//!
//! # Ownership
//!
//! This module owns the specialized semantic API for an **erasure fault**.
//!
//! An erasure is a realized fault whose semantics explicitly indicate that
//! the information/resource at a fault location has become unavailable or
//! erased from the intended computational representation.
//!
//! This module owns:
//!
//! - the `Erasure` semantic wrapper;
//! - construction of canonical erasure faults;
//! - validation that an existing `Fault` is genuinely an erasure;
//! - ergonomic constructors for canonical logical and physical qubits;
//! - conversion between `Erasure` and the canonical `Fault`;
//! - erasure-specific predicates;
//! - erasure-specific inspection APIs.
//!
//! # Does NOT own
//!
//! This module does NOT own:
//!
//! - the canonical `Fault` representation;
//! - `FaultId` allocation;
//! - logical qubit identity;
//! - physical qubit identity;
//! - probability distributions;
//! - stochastic sampling;
//! - noise-model generation;
//! - quantum channels;
//! - loss semantics;
//! - leakage semantics;
//! - QEC decoding;
//! - syndrome extraction;
//! - logical correction;
//! - routing;
//! - scheduling;
//! - calibration;
//! - characterization;
//! - hardware APIs;
//! - backend execution;
//! - serialization formats;
//! - resource-policy limits;
//! - global RNG state.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir::qubit
//!        │
//!        ├── QubitId
//!        └── PhysicalQubitId
//!                 │
//!                 ▼
//!         zqn::fault::fault
//!                 │
//!                 └── Fault
//!                      │
//!                      ├── classification
//!                      ├── location
//!                      └── effect
//!                             │
//!                             ▼
//!                    zqn::fault::erasure
//!                             │
//!                             └── Erasure
//!                             │
//!             ┌───────────────┼────────────────┐
//!             ▼               ▼                ▼
//!            QEC          simulation       benchmarking
//! ```
//!
//! `Fault` remains the canonical fault representation.
//!
//! `Erasure` is a semantic specialization that guarantees:
//!
//! ```text
//! FaultClassification::Erasure
//! +
//! FaultEffect::Erasure
//! ```
//!
//! It therefore prevents downstream code from repeatedly performing unchecked
//! classification/effect matching.
//!
//! # Canonical identity
//!
//! Erasure locations use the canonical Quantum IR identities through
//! `FaultLocation`:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module MUST NOT define:
//!
//! ```text
//! ErasureQubitId
//! ZqnQubitId
//! ErasurePhysicalQubitId
//! ```
//!
//! or any equivalent duplicate identity type.
//!
//! `FaultLocation::logical_qubit` and `FaultLocation::physical_qubit` remain
//! the canonical construction boundaries.
//!
//! # Erasure versus loss
//!
//! Erasure and loss are deliberately different semantic concepts.
//!
//! ```text
//! Erasure
//!     Information/resource is explicitly represented as erased.
//!
//! Loss
//!     A physical resource or excitation is lost.
//! ```
//!
//! A particular physical process may be modeled as loss and subsequently
//! represented as an erasure at another abstraction level, but this module
//! MUST NOT silently convert one semantic category into the other.
//!
//! # Erasure versus leakage
//!
//! Leakage means that a state leaves the intended computational subspace.
//!
//! Erasure means that the information/resource is explicitly unavailable or
//! erased from the intended representation.
//!
//! These are not interchangeable:
//!
//! ```text
//! Leakage != Erasure
//! ```
//!
//! A device may experience leakage without the system treating the resource as
//! erased, and an erasure may be declared by an execution protocol without
//! requiring a microscopic leakage mechanism.
//!
//! # Erasure versus probability
//!
//! `Erasure` represents a **realized fault**.
//!
//! It therefore does not contain an erasure probability.
//!
//! Probability belongs to:
//!
//! ```text
//! probability/*
//! noise/*
//! channel/*
//! ```
//!
//! A noise model may produce an `Erasure` with some probability, but the
//! realized `Erasure` itself is not a probability distribution.
//!
//! This distinction prevents the same probability from being represented in
//! multiple incompatible places.
//!
//! # Write once, scale everywhere
//!
//! This module contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_ERASURES
//! MAX_CORRELATED_QUBITS
//! MAX_FAULTS
//! MAX_DEVICES
//! ```
//!
//! There is no semantic upper bound on the number of erasures that a Zamani
//! computation may represent.
//!
//! A single `Erasure` represents one canonical fault. Large collections of
//! erasures must be streamed or batched by their owning subsystem rather than
//! being forced through this type.
//!
//! The practical maximum is determined by:
//!
//! - available memory;
//! - storage;
//! - execution resources;
//! - distributed capacity;
//! - caller-provided `ZqnLimits`;
//! - target capabilities;
//! - runtime policy.
//!
//! Therefore "infinity" means:
//!
//! > no artificial finite machine-size ceiling is encoded in the semantic
//! > erasure abstraction.
//!
//! It does not claim that a physical machine has infinite resources.
//!
//! # Determinism
//!
//! `Erasure` contains no hidden randomness.
//!
//! It does not:
//!
//! - access a global RNG;
//! - create a random identifier;
//! - read wall-clock time;
//! - inspect memory addresses;
//! - depend on thread scheduling;
//! - depend on hash-map iteration order;
//! - maintain global mutable state.
//!
//! Deterministic sampling belongs to `noise/*` and `simulation/*`.
//!
//! # Resource safety
//!
//! This file does not introduce resource limits because semantic validation and
//! resource policy are separate concerns.
//!
//! Construction of an `Erasure` delegates structural validation to the
//! canonical `Fault::erasure` implementation.
//!
//! Composite locations are caller-owned. This module does not secretly expand,
//! duplicate, enumerate, or materialize additional resources.
//!
//! Callers processing untrusted input MUST enforce explicit resource policies
//! before materializing arbitrarily large composite locations or collections
//! of erasures.
//!
//! # Numerical safety
//!
//! This module performs no floating-point arithmetic.
//!
//! Erasure probabilities, rates, confidence intervals and statistical
//! estimates belong to the probability, noise and characterization layers.
//!
//! # Serialization
//!
//! This module deliberately does not define a wire format.
//!
//! `io/*` owns:
//!
//! - schema;
//! - serialization;
//! - deserialization;
//! - canonical encoding;
//! - compatibility;
//! - migration.
//!
//! Serialization of an `Erasure` MUST preserve the complete underlying
//! canonical `Fault`, including:
//!
//! - fault identity;
//! - classification;
//! - location domain;
//! - location identity;
//! - effect;
//! - timing;
//! - optional metadata/annotations;
//! - schema/version information.
//!
//! Logical and physical qubit identities MUST remain distinguishable in the
//! serialized representation.
//!
//! # Thread safety
//!
//! `Erasure` is immutable.
//!
//! It contains no interior mutability and no global state.
//!
//! It is therefore suitable for concurrent use when placed in appropriate
//! thread-safe containers.
//!
//! # Security
//!
//! An erasure is data, not authority.
//!
//! Possession of an `Erasure`, `FaultId`, `QubitId` or `PhysicalQubitId` MUST
//! NOT grant:
//!
//! - QPU access;
//! - hardware control;
//! - credentials;
//! - calibration write access;
//! - execution authorization.
//!
//! Authorization belongs to the surrounding capability/security system.
//!
//! # Rust compatibility
//!
//! This file targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` is deliberately used as a compile-time guarantee.
//!
//! # Integration contract
//!
//! ```text
//! quantum::ir::qubit
//!       │
//!       ├── QubitId
//!       └── PhysicalQubitId
//!              │
//!              ▼
//! zqn::fault::fault
//!       │
//!       ├── Fault
//!       ├── FaultId
//!       ├── FaultLocation
//!       ├── FaultClassification
//!       └── FaultEffect
//!              │
//!              ▼
//! zqn::fault::erasure
//!       │
//!       └── Erasure
//!              │
//!       ├──────────────┬──────────────┬───────────────┐
//!       ▼              ▼              ▼               ▼
//!     QEC          simulation    benchmarking      runtime
//! ```
//!
//! The dependency direction is intentionally one-way:
//!
//! ```text
//! Fault
//!   ↓
//! Erasure
//! ```
//!
//! `fault.rs` does not depend on this module.
//!
//! This avoids a circular dependency and permits `fault.rs` to remain the
//! semantic owner of `Fault`.
//!
//! # Integration with QEC
//!
//! `integration/qec.rs` may consume `Erasure` and convert it into a
//! QEC-specific physical-fault representation.
//!
//! This module does not know:
//!
//! - which QEC code is being used;
//! - how a syndrome is extracted;
//! - how an erasure decoder operates;
//! - how correction is performed;
//! - whether the erasure is physical or logical from the QEC perspective.
//!
//! # Integration with simulation
//!
//! Simulation may consume:
//!
//! ```text
//! Erasure -> simulation-specific state transition
//! ```
//!
//! This module does not modify a quantum state.
//!
//! # Integration with noise
//!
//! `noise/*` may generate an erasure according to a stochastic model:
//!
//! ```text
//! NoiseModel
//!      │
//!      ▼
//! stochastic realization
//!      │
//!      ▼
//! Erasure
//! ```
//!
//! Sampling policy, probability and RNG state remain outside this file.
//!
//! # Integration with channels
//!
//! A quantum channel may represent an erasure process probabilistically.
//!
//! The channel remains the mathematical transformation.
//!
//! `Erasure` represents one realized event.
//!
//! Therefore:
//!
//! ```text
//! channel::erasure_process
//!          │
//!          ▼
//!       sampling
//!          │
//!          ▼
//!       Erasure
//! ```
//!
//! This module does not define an erasure channel.
//!
//! # Integration with characterization
//!
//! Characterization may estimate an erasure probability/rate and subsequently
//! construct a noise model that realizes `Erasure` values.
//!
//! The dependency is:
//!
//! ```text
//! observations
//!      ↓
//! characterization
//!      ↓
//! probability/parameter
//!      ↓
//! noise model
//!      ↓
//! Erasure
//! ```
//!
//! # Integration with benchmarking
//!
//! Benchmarking may count or classify erasures through:
//!
//! ```text
//! Erasure
//!     ↓
//! FaultClassification
//!     ↓
//! benchmark statistics
//! ```
//!
//! This file does not own those statistics.
//!
//! # Integration with routing and scheduling
//!
//! Routing and scheduling may consume erasure-related risk/cost information
//! through ZQN integration APIs.
//!
//! They must not modify the semantic meaning of `Erasure`.
//!
//! # Integration with canonical fault.rs
//!
//! `fault.rs` is the semantic owner of:
//!
//! ```text
//! Fault
//! FaultLocation
//! FaultClassification
//! FaultEffect
//! ```
//!
//! This module therefore delegates construction to:
//!
//! ```text
//! Fault::erasure(...)
//! ```
//!
//! and validates incoming `Fault` values by checking both:
//!
//! ```text
//! FaultClassification::Erasure
//! FaultEffect::Erasure
//! ```
//!
//! Checking both dimensions is intentional.
//!
//! A classification/effect mismatch must never be silently accepted.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. `Erasure` cannot contain a non-erasure `Fault`;
//! 2. `Fault` remains the canonical fault representation;
//! 3. canonical IR `QubitId`/`PhysicalQubitId` are used indirectly through
//!    `FaultLocation`;
//! 4. no duplicate qubit identity exists;
//! 5. no probability is duplicated in the realized fault;
//! 6. no machine-size ceiling exists;
//! 7. no global RNG/state exists;
//! 8. no unsafe code exists;
//! 9. construction is deterministic;
//! 10. validation is explicit;
//! 11. composite locations remain caller-controlled;
//! 12. QEC/simulation/noise integrations remain decoupled;
//! 13. serialization remains owned by `io/*`;
//! 14. future erasure-related analysis can be added without changing the
//!     canonical fault representation;
//! 15. the file does not need to be reopened merely because a downstream
//!     consumer is implemented.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::core::errors::{ZqnError, ZqnResult};
use crate::quantum::zqn::core::ids::FaultId;
use crate::quantum::zqn::fault::fault::{
    Fault,
    FaultClassification,
    FaultEffect,
    FaultLocation,
};

// ============================================================================
// Erasure
// ============================================================================

/// A validated, canonical ZQN erasure fault.
///
/// `Erasure` is a semantic specialization of [`Fault`].
///
/// The wrapped fault is guaranteed to satisfy both:
///
/// ```text
/// FaultClassification::Erasure
/// FaultEffect::Erasure
/// ```
///
/// The type deliberately does not duplicate any of the underlying fault
/// fields. This keeps `Fault` as the single canonical representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Erasure {
    fault: Fault,
}

impl Erasure {
    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    /// Constructs an erasure at an arbitrary canonical ZQN fault location.
    ///
    /// The location may be:
    ///
    /// - logical;
    /// - physical;
    /// - operation-associated;
    /// - measurement-associated;
    /// - preparation-associated;
    /// - reset-associated;
    /// - transport-associated;
    /// - composite;
    /// - another location explicitly supported by `FaultLocation`.
    ///
    /// No location-size limit is imposed here.
    ///
    /// Structural validation is delegated to the canonical `Fault::erasure`
    /// constructor so that there is exactly one owner of canonical fault
    /// validation.
    pub fn new(id: FaultId, location: FaultLocation) -> ZqnResult<Self> {
        let fault = Fault::erasure(id, location)?;
        Self::from_validated_fault(fault)
    }

    /// Constructs an erasure affecting a canonical logical qubit.
    ///
    /// The supplied `QubitId` is the authoritative Quantum IR logical-qubit
    /// identity.
    pub fn logical_qubit(id: FaultId, qubit: QubitId) -> ZqnResult<Self> {
        Self::new(id, FaultLocation::logical_qubit(qubit))
    }

    /// Constructs an erasure affecting a canonical physical qubit.
    ///
    /// The supplied `PhysicalQubitId` is the authoritative Quantum IR
    /// physical-qubit identity.
    pub fn physical_qubit(
        id: FaultId,
        qubit: PhysicalQubitId,
    ) -> ZqnResult<Self> {
        Self::new(id, FaultLocation::physical_qubit(qubit))
    }

    /// Constructs an erasure affecting a caller-provided composite location.
    ///
    /// The caller owns the collection and therefore controls its resource
    /// policy. This function does not impose an arbitrary maximum number of
    /// locations.
    ///
    /// Use this for correlated or multi-resource erasures when the canonical
    /// fault semantics require a composite location.
    pub fn composite(
        id: FaultId,
        locations: Vec<FaultLocation>,
    ) -> ZqnResult<Self> {
        Self::new(id, FaultLocation::Composite(locations))
    }

    /// Converts an existing canonical `Fault` into an `Erasure`.
    ///
    /// This is the primary integration boundary for:
    ///
    /// - noise generation;
    /// - QEC adapters;
    /// - simulation;
    /// - deserialization;
    /// - hardware observations.
    ///
    /// The input must already represent an erasure in both semantic
    /// dimensions:
    ///
    /// ```text
    /// classification == Erasure
    /// effect == Erasure
    /// ```
    pub fn from_fault(fault: Fault) -> ZqnResult<Self> {
        fault.validate()?;
        Self::from_validated_fault(fault)
    }

    /// Validates and wraps a fault that has already been constructed through
    /// the canonical fault API.
    ///
    /// This helper is private intentionally: callers should use `from_fault`
    /// unless they are inside this module and already know that structural
    /// validation has happened.
    fn from_validated_fault(fault: Fault) -> ZqnResult<Self> {
        let classification_is_erasure =
            matches!(fault.classification(), FaultClassification::Erasure);

        let effect_is_erasure =
            matches!(fault.effect(), FaultEffect::Erasure);

        if !classification_is_erasure || !effect_is_erasure {
            return Err(ZqnError::invalid_fault(
                "erasure requires FaultClassification::Erasure and FaultEffect::Erasure",
            ));
        }

        Ok(Self { fault })
    }

    // ------------------------------------------------------------------------
    // Validation
    // ------------------------------------------------------------------------

    /// Validates the wrapped canonical fault and erasure invariants.
    ///
    /// This method is deterministic and does not allocate additional
    /// structures.
    pub fn validate(&self) -> ZqnResult<()> {
        self.fault.validate()?;

        if !matches!(
            self.fault.classification(),
            FaultClassification::Erasure
        ) {
            return Err(ZqnError::invalid_fault(
                "erasure fault has a non-erasure classification",
            ));
        }

        if !matches!(self.fault.effect(), FaultEffect::Erasure) {
            return Err(ZqnError::invalid_fault(
                "erasure fault has a non-erasure effect",
            ));
        }

        Ok(())
    }

    /// Returns `true` when the wrapped fault satisfies the complete erasure
    /// invariant.
    ///
    /// This is an inspection predicate and therefore does not allocate.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    // ------------------------------------------------------------------------
    // Canonical fault access
    // ------------------------------------------------------------------------

    /// Returns the canonical underlying fault.
    ///
    /// The returned value remains the authoritative ZQN representation.
    #[must_use]
    pub const fn fault(&self) -> &Fault {
        &self.fault
    }

    /// Consumes the specialization and returns the canonical fault.
    #[must_use]
    pub fn into_fault(self) -> Fault {
        self.fault
    }

    /// Returns the canonical fault location.
    #[must_use]
    pub fn location(&self) -> &FaultLocation {
        self.fault.location()
    }

    /// Returns the canonical fault classification.
    ///
    /// For a valid `Erasure`, this is always
    /// `FaultClassification::Erasure`.
    #[must_use]
    pub fn classification(&self) -> &FaultClassification {
        self.fault.classification()
    }

    /// Returns the canonical fault effect.
    ///
    /// For a valid `Erasure`, this is always `FaultEffect::Erasure`.
    #[must_use]
    pub fn effect(&self) -> &FaultEffect {
        self.fault.effect()
    }

    // ------------------------------------------------------------------------
    // Location inspection
    // ------------------------------------------------------------------------

    /// Returns the logical qubit when this erasure is directly located on one.
    ///
    /// Composite locations return `None`; callers should inspect the canonical
    /// `FaultLocation` for composite semantics.
    #[must_use]
    pub fn logical_qubit_id(&self) -> Option<QubitId> {
        match self.location() {
            FaultLocation::LogicalQubit(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns the physical qubit when this erasure is directly located on
    /// one.
    ///
    /// Composite locations return `None`.
    #[must_use]
    pub fn physical_qubit_id(&self) -> Option<PhysicalQubitId> {
        match self.location() {
            FaultLocation::PhysicalQubit(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns whether this erasure directly targets a logical qubit.
    #[must_use]
    pub fn is_logical_qubit_erasure(&self) -> bool {
        matches!(self.location(), FaultLocation::LogicalQubit(_))
    }

    /// Returns whether this erasure directly targets a physical qubit.
    #[must_use]
    pub fn is_physical_qubit_erasure(&self) -> bool {
        matches!(self.location(), FaultLocation::PhysicalQubit(_))
    }

    /// Returns whether this erasure has a composite location.
    #[must_use]
    pub fn is_composite(&self) -> bool {
        matches!(self.location(), FaultLocation::Composite(_))
    }

    /// Returns the number of top-level resources in a composite erasure.
    ///
    /// This does not recursively count nested composite locations.
    ///
    /// Returning `Option<usize>` avoids assigning a fabricated count to
    /// non-composite locations.
    #[must_use]
    pub fn top_level_location_count(&self) -> Option<usize> {
        match self.location() {
            FaultLocation::Composite(locations) => Some(locations.len()),
            _ => None,
        }
    }

    // ------------------------------------------------------------------------
    // Semantic predicates
    // ------------------------------------------------------------------------

    /// Returns whether this is semantically an erasure.
    ///
    /// Because `Erasure` cannot normally contain a non-erasure fault, this
    /// predicate is expected to remain `true` for a successfully constructed
    /// value.
    #[must_use]
    pub fn is_erasure(&self) -> bool {
        matches!(
            (
                self.fault.classification(),
                self.fault.effect()
            ),
            (
                FaultClassification::Erasure,
                FaultEffect::Erasure
            )
        )
    }
}

impl AsRef<Fault> for Erasure {
    fn as_ref(&self) -> &Fault {
        self.fault()
    }
}

impl From<Erasure> for Fault {
    fn from(erasure: Erasure) -> Self {
        erasure.into_fault()
    }
}

impl TryFrom<Fault> for Erasure {
    type Error = ZqnError;

    fn try_from(fault: Fault) -> Result<Self, Self::Error> {
        Self::from_fault(fault)
    }
}

impl fmt::Display for Erasure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "erasure fault at {}",
            self.location()
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn fault_id(value: u64) -> FaultId {
        FaultId::new(value)
    }

    #[test]
    fn logical_qubit_constructor_uses_canonical_ir_identity() {
        let qubit = QubitId::new(7);

        let result = Erasure::logical_qubit(fault_id(1), qubit);

        assert!(result.is_ok());

        let erasure = result.expect("logical-qubit erasure should construct");

        assert!(erasure.is_erasure());
        assert!(erasure.is_logical_qubit_erasure());
        assert_eq!(erasure.logical_qubit_id(), Some(qubit));
        assert_eq!(
            erasure.classification(),
            &FaultClassification::Erasure
        );
        assert_eq!(erasure.effect(), &FaultEffect::Erasure);
    }

    #[test]
    fn physical_qubit_constructor_uses_canonical_ir_identity() {
        let qubit = PhysicalQubitId::new(11);

        let result = Erasure::physical_qubit(fault_id(2), qubit);

        assert!(result.is_ok());

        let erasure =
            result.expect("physical-qubit erasure should construct");

        assert!(erasure.is_erasure());
        assert!(erasure.is_physical_qubit_erasure());
        assert_eq!(erasure.physical_qubit_id(), Some(qubit));
    }

    #[test]
    fn canonical_fault_round_trip_preserves_erasure_semantics() {
        let original =
            Erasure::physical_qubit(fault_id(3), PhysicalQubitId::new(5))
                .expect("erasure should construct");

        let fault = original.clone().into_fault();

        let restored =
            Erasure::from_fault(fault).expect("erasure should round-trip");

        assert_eq!(original, restored);
        assert!(restored.is_erasure());
    }

    #[test]
    fn try_from_fault_uses_same_validation_contract() {
        let fault =
            Fault::erasure(
                fault_id(4),
                FaultLocation::physical_qubit(
                    PhysicalQubitId::new(13),
                ),
            )
            .expect("canonical erasure should construct");

        let erasure =
            Erasure::try_from(fault)
                .expect("canonical erasure should be accepted");

        assert!(erasure.is_valid());
        assert_eq!(
            erasure.physical_qubit_id(),
            Some(PhysicalQubitId::new(13))
        );
    }

    #[test]
    fn composite_erasure_does_not_encode_a_fixed_resource_limit() {
        let locations = vec![
            FaultLocation::physical_qubit(PhysicalQubitId::new(1)),
            FaultLocation::physical_qubit(PhysicalQubitId::new(2)),
            FaultLocation::physical_qubit(PhysicalQubitId::new(3)),
        ];

        let erasure =
            Erasure::composite(fault_id(5), locations)
                .expect("composite erasure should construct");

        assert!(erasure.is_composite());
        assert_eq!(erasure.top_level_location_count(), Some(3));
        assert!(erasure.is_erasure());
    }

    #[test]
    fn validation_is_deterministic() {
        let erasure =
            Erasure::logical_qubit(fault_id(6), QubitId::new(9))
                .expect("erasure should construct");

        assert!(erasure.validate().is_ok());
        assert!(erasure.validate().is_ok());
        assert!(erasure.is_valid());
        assert!(erasure.is_valid());
    }

    #[test]
    fn conversion_to_fault_preserves_the_canonical_location() {
        let qubit = PhysicalQubitId::new(17);

        let erasure =
            Erasure::physical_qubit(fault_id(7), qubit)
                .expect("erasure should construct");

        let fault: Fault = erasure.into();

        assert_eq!(
            fault.location(),
            &FaultLocation::PhysicalQubit(qubit)
        );
        assert_eq!(
            fault.classification(),
            &FaultClassification::Erasure
        );
        assert_eq!(fault.effect(), &FaultEffect::Erasure);
    }
}