//! Zamani Quantum Resilience — Resilience Fault Model.
//!
//! This module defines the resilience-domain representation of a realized
//! quantum fault.
//!
//! # Architectural position
//!
//! The canonical physical/quantum fault semantics belong to:
//!
//! ```text
//! crate::quantum::zqn::fault::fault
//! ```
//!
//! This module MUST NOT create a competing quantum fault ontology.
//!
//! Instead, it provides the resilience layer with an immutable, validated
//! boundary object around the canonical ZQN `Fault`.
//!
//! The dependency direction is:
//!
//! ```text
//! quantum::ir::qubit
//!          │
//!          ▼
//! quantum::zqn::fault::fault::Fault
//!          │
//!          ▼
//! quantum::resilience::model::fault::ResilienceFault
//!          │
//!          ├── detection
//!          ├── diagnosis
//!          ├── policy
//!          ├── planning
//!          ├── adaptation
//!          ├── recovery
//!          └── verification
//! ```
//!
//! # Why this wrapper exists
//!
//! ZQN answers:
//!
//! > What fault occurred?
//!
//! Resilience answers:
//!
//! > What does that fault mean for continued execution, and what should the
//! > resilience system do about it?
//!
//! Those are different responsibilities.
//!
//! The resilience layer therefore preserves the complete canonical ZQN fault
//! while adding only resilience-owned context that is necessary for detection,
//! diagnosis, planning, recovery, and verification.
//!
//! # Canonical identity
//!
//! Resource identities remain owned by the Quantum IR:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module intentionally does not define either type.
//!
//! Fault identity remains owned by ZQN:
//!
//! ```text
//! crate::quantum::zqn::core::ids::FaultId
//! ```
//!
//! A `ResilienceFault` therefore cannot accidentally manufacture a second
//! fault identity for the same physical event.
//!
//! # Write once, scale everywhere
//!
//! This file contains no machine-specific constants and no artificial limits.
//!
//! In particular, it contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_FAULTS
//! MAX_CORRELATED_QUBITS
//! MAX_OPERATIONS
//! MAX_BACKENDS
//! ```
//!
//! A fault may refer to any canonical ZQN location representable by the
//! underlying ZQN model and permitted by the caller's resource policy.
//!
//! "Infinity" therefore means:
//!
//! > the semantic model does not impose an artificial finite machine-size
//! > ceiling.
//!
//! Actual execution remains constrained by available memory, storage,
//! processing resources, hardware capability, configured policy, deadlines,
//! and distributed capacity.
//!
//! # No hidden state
//!
//! `ResilienceFault` contains:
//!
//! - no global state;
//! - no mutable static state;
//! - no RNG;
//! - no clock access;
//! - no implicit telemetry collection;
//! - no backend access;
//! - no filesystem access;
//! - no network access;
//! - no authorization capability.
//!
//! Consequently construction is deterministic.
//!
//! # No unsafe Rust
//!
//! This module explicitly forbids unsafe Rust.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration contract
//!
//! This module integrates with the rest of the resilience subsystem as follows:
//!
//! ```text
//! ResilienceFault
//!      │
//!      ├── detector observations
//!      │
//!      ├── diagnosis evidence
//!      │
//!      ├── incident aggregation
//!      │
//!      ├── recovery planning
//!      │
//!      ├── adaptation decisions
//!      │
//!      └── verification/provenance
//! ```
//!
//! The object remains an immutable fact. It does not itself perform detection,
//! diagnosis, recovery, adaptation, mitigation, or verification.
//!
//! # Important semantic rule
//!
//! A resilience classification must never silently replace the underlying ZQN
//! classification.
//!
//! The canonical fault is always available through `canonical()`.
//!
//! Resilience-owned interpretation is represented separately.
//!
//! This makes it possible to distinguish:
//!
//! ```text
//! observed physical fault
//!         ≠
//! resilience interpretation
//!         ≠
//! recovery decision
//! ```
//!
//! That distinction is essential for deterministic replay and auditability.
//!
//! # Provenance
//!
//! A resilience system may receive the same canonical fault from multiple
//! execution contexts.
//!
//! Therefore the wrapper can carry an optional caller-supplied provenance
//! identifier without changing the ZQN fault identity.
//!
//! The provenance value is opaque to this module.
//!
//! No memory address, process identifier, current time, or random value may be
//! used to construct it implicitly.
//!
//! # Severity
//!
//! Severity is intentionally not duplicated here.
//!
//! `model::severity` owns resilience severity semantics.
//!
//! A fault is an observed fact; severity is a resilience interpretation that
//! may depend on execution context, policy, logical encoding, resource
//! redundancy, and current machine state.
//!
//! # Confidence
//!
//! Confidence is also not duplicated here.
//!
//! `model::confidence` owns confidence semantics.
//!
//! This prevents the canonical fault model and resilience inference model from
//! becoming coupled.
//!
//! # Resource domain
//!
//! Resource-domain information is derived from the canonical ZQN location.
//!
//! No resilience-specific `QubitId` or `PhysicalQubitId` is introduced.
//!
//! # Correlation
//!
//! Correlation is not duplicated here.
//!
//! Canonical correlated-fault semantics remain in ZQN. Resilience incident
//! aggregation may group multiple `ResilienceFault` values later.
//!
//! # Validation
//!
//! The canonical ZQN fault remains responsible for semantic validation of the
//! underlying fault.
//!
//! This wrapper validates only resilience-specific invariants:
//!
//! - the wrapper must contain a canonical fault;
//! - optional provenance must be valid according to this module's opaque-ID
//!   contract;
//! - construction must never alter the canonical fault.
//!
//! Because the canonical fault type is already validated by its own
//! constructor, resilience does not duplicate or weaken that validation.
//!
//! # Serialization
//!
//! This module does not define a wire format.
//!
//! The resilience serialization layer owns encoding, decoding, schema versions,
//! and migration.
//!
//! The canonical fault must remain recoverable during serialization.
//!
//! # Security
//!
//! A `ResilienceFault` is data, not a capability.
//!
//! Possession of one MUST NOT grant:
//!
//! - QPU access;
//! - backend access;
//! - credentials;
//! - calibration access;
//! - filesystem access;
//! - network access;
//! - recovery authorization.
//!
//! Recovery authorization belongs to the surrounding execution/security
//! subsystem.
//!
//! # Determinism
//!
//! Construction is deterministic.
//!
//! Accessors are deterministic.
//!
//! Equality is value based.
//!
//! No ordering based on arrival time is provided.
//!
//! No ordering based on memory address is provided.
//!
//! # Thread safety
//!
//! The type is immutable after construction and contains no interior
//! mutability.
//!
//! It can therefore be shared between concurrent readers when the surrounding
//! container/context satisfies the corresponding Rust thread-safety bounds.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. the canonical ZQN fault remains authoritative;
//! 2. no competing fault identity is introduced;
//! 3. canonical IR qubit identities remain authoritative;
//! 4. resilience metadata is separate from physical fault semantics;
//! 5. no fixed machine-size limit exists;
//! 6. no hidden randomness exists;
//! 7. no hidden time source exists;
//! 8. no global mutable state exists;
//! 9. no unsafe Rust exists;
//! 10. the type can be consumed by future resilience modules without changing
//!     its fundamental identity model;
//! 11. canonical ZQN provenance can be preserved through the resilience layer;
//! 12. deterministic replay can recover the exact canonical fault;
//! 13. the wrapper cannot mutate the underlying fault.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use crate::quantum::zqn::core::ids::FaultId;
use crate::quantum::zqn::fault::fault::Fault as ZqnFault;

/// An opaque, caller-supplied provenance identifier for a resilience
/// observation.
///
/// This is deliberately distinct from [`FaultId`].
///
/// `FaultId` identifies the canonical ZQN fault.
///
/// `ProvenanceId` identifies the resilience/execution context in which the
/// fault was observed or transported.
///
/// The value has no implicit meaning and grants no authority.
///
/// The caller is responsible for generating it according to the surrounding
/// deterministic identity policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProvenanceId(u64);

impl ProvenanceId {
    /// Creates a provenance identifier from a caller-owned stable value.
    ///
    /// This function performs no implicit allocation, hashing, clock access,
    /// or randomness.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying stable value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for ProvenanceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// Resilience-level representation of a realized quantum fault.
///
/// The canonical ZQN [`ZqnFault`] remains the semantic source of truth.
///
/// `ResilienceFault` adds only resilience-domain provenance. It deliberately
/// does not copy the ZQN fault's location, classification, effect, timing, or
/// operation fields because doing so would create duplicate state that could
/// diverge from the canonical value.
///
/// # Invariant
///
/// ```text
/// resilience_fault.canonical()
///     ==
/// canonical ZQN fault that was supplied at construction
/// ```
///
/// The canonical fault is never modified by this type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResilienceFault {
    canonical: ZqnFault,
    provenance: Option<ProvenanceId>,
}

impl ResilienceFault {
    /// Creates a resilience fault directly from an already validated canonical
    /// ZQN fault.
    ///
    /// This is the preferred constructor for normal integration because ZQN
    /// remains responsible for constructing and validating physical fault
    /// semantics.
    ///
    /// No information is copied out of the canonical fault.
    #[must_use]
    pub const fn from_canonical(canonical: ZqnFault) -> Self {
        Self {
            canonical,
            provenance: None,
        }
    }

    /// Creates a resilience fault with caller-supplied provenance.
    ///
    /// Provenance is intentionally opaque. This type does not interpret the
    /// value as a timestamp, memory address, backend identifier, or execution
    /// counter.
    #[must_use]
    pub const fn with_provenance(
        canonical: ZqnFault,
        provenance: ProvenanceId,
    ) -> Self {
        Self {
            canonical,
            provenance: Some(provenance),
        }
    }

    /// Returns the canonical ZQN fault by shared reference.
    ///
    /// Returning a reference rather than cloning the semantic components keeps
    /// one authoritative representation and avoids unnecessary allocation.
    #[must_use]
    pub const fn canonical(&self) -> &ZqnFault {
        &self.canonical
    }

    /// Consumes the wrapper and returns the canonical ZQN fault.
    ///
    /// This is useful at subsystem boundaries where the resilience layer has
    /// finished processing and ownership must be transferred back to another
    /// component.
    #[must_use]
    pub fn into_canonical(self) -> ZqnFault {
        self.canonical
    }

    /// Returns the canonical ZQN fault identity.
    ///
    /// This does not create a new resilience identity.
    #[must_use]
    pub const fn fault_id(&self) -> FaultId {
        self.canonical.id()
    }

    /// Returns the optional resilience provenance identifier.
    #[must_use]
    pub const fn provenance(&self) -> Option<ProvenanceId> {
        self.provenance
    }

    /// Returns whether this fault carries resilience provenance.
    #[must_use]
    pub const fn has_provenance(&self) -> bool {
        self.provenance.is_some()
    }

    /// Validates the resilience wrapper.
    ///
    /// The underlying ZQN fault is already the authoritative semantic object.
    /// Therefore resilience validation does not duplicate its structural
    /// validation rules.
    ///
    /// The current resilience wrapper has no additional fallible structural
    /// conditions, so a valid canonical fault always yields `Ok(())`.
    ///
    /// This method intentionally exists now as a stable integration boundary
    /// for future resilience-owned invariants without forcing callers to
    /// redesign their APIs later.
    pub fn validate(&self) -> crate::quantum::zqn::core::errors::ZqnResult<()> {
        Ok(())
    }

    /// Returns the canonical fault's deterministic identity.
    ///
    /// This is an explicit alias useful to planners and incident aggregation
    /// code where `fault_id()` is clearer than the generic term `id`.
    #[must_use]
    pub const fn id(&self) -> FaultId {
        self.fault_id()
    }

    /// Returns a stable semantic reference to the canonical fault.
    ///
    /// This method is intentionally equivalent to [`Self::canonical`] and
    /// exists as a descriptive integration API for verification/provenance
    /// code.
    #[must_use]
    pub const fn source_fault(&self) -> &ZqnFault {
        &self.canonical
    }

    /// Returns whether the supplied canonical fault has the same ZQN identity
    /// as this resilience fault.
    ///
    /// Identity comparison is deliberately based on `FaultId`; semantic
    /// equivalence must be decided using the canonical ZQN fault itself.
    #[must_use]
    pub const fn has_fault_id(&self, id: FaultId) -> bool {
        self.fault_id() == id
    }
}

impl From<ZqnFault> for ResilienceFault {
    fn from(canonical: ZqnFault) -> Self {
        Self::from_canonical(canonical)
    }
}

impl AsRef<ZqnFault> for ResilienceFault {
    fn as_ref(&self) -> &ZqnFault {
        self.canonical()
    }
}

impl From<ResilienceFault> for ZqnFault {
    fn from(fault: ResilienceFault) -> Self {
        fault.into_canonical()
    }
}

/// Borrows the canonical ZQN fault without changing ownership.
///
/// This trait is useful for generic resilience components that need to consume
/// any resilience fault-like value while keeping the canonical ZQN fault as
/// the common semantic contract.
pub trait CanonicalFaultRef {
    /// Returns the canonical ZQN fault.
    fn canonical_fault(&self) -> &ZqnFault;
}

impl CanonicalFaultRef for ResilienceFault {
    fn canonical_fault(&self) -> &ZqnFault {
        self.canonical()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::qubit::QubitId;
    use crate::quantum::zqn::fault::fault::{
        FaultClassification,
        FaultEffect,
        FaultLocation,
        FaultTiming,
    };

    fn test_fault(id: u64) -> ZqnFault {
        ZqnFault::new(
            FaultId::new(id),
            FaultLocation::logical_qubit(QubitId::new(0)),
            FaultClassification::Preparation,
            FaultEffect::None,
            FaultTiming::default(),
            None,
            None,
        )
        .expect("test fault construction must succeed")
    }

    #[test]
    fn wraps_canonical_fault_without_changing_identity() {
        let canonical = test_fault(1);
        let expected_id = canonical.id();

        let resilience = ResilienceFault::from_canonical(canonical);

        assert_eq!(resilience.fault_id(), expected_id);
        assert_eq!(resilience.id(), expected_id);
        assert!(resilience.validate().is_ok());
    }

    #[test]
    fn preserves_canonical_fault_by_reference() {
        let canonical = test_fault(2);
        let expected_id = canonical.id();

        let resilience = ResilienceFault::from_canonical(canonical);

        assert_eq!(resilience.canonical().id(), expected_id);
        assert_eq!(resilience.source_fault().id(), expected_id);
    }

    #[test]
    fn consumes_back_into_canonical_fault() {
        let canonical = test_fault(3);
        let expected_id = canonical.id();

        let resilience = ResilienceFault::from_canonical(canonical);
        let recovered = resilience.into_canonical();

        assert_eq!(recovered.id(), expected_id);
    }

    #[test]
    fn provenance_is_explicit_and_deterministic() {
        let canonical = test_fault(4);
        let provenance = ProvenanceId::new(42);

        let resilience =
            ResilienceFault::with_provenance(canonical, provenance);

        assert_eq!(resilience.provenance(), Some(provenance));
        assert!(resilience.has_provenance());
    }

    #[test]
    fn no_provenance_is_the_default() {
        let resilience = ResilienceFault::from_canonical(test_fault(5));

        assert_eq!(resilience.provenance(), None);
        assert!(!resilience.has_provenance());
    }

    #[test]
    fn conversion_from_canonical_is_lossless() {
        let canonical = test_fault(6);
        let expected_id = canonical.id();

        let resilience: ResilienceFault = canonical.into();
        let recovered: ZqnFault = resilience.into();

        assert_eq!(recovered.id(), expected_id);
    }

    #[test]
    fn canonical_reference_trait_preserves_identity() {
        let resilience = ResilienceFault::from_canonical(test_fault(7));

        let canonical = resilience.canonical_fault();

        assert_eq!(canonical.id(), resilience.fault_id());
    }

    #[test]
    fn different_provenance_does_not_change_fault_identity() {
        let canonical_a = test_fault(8);
        let canonical_b = test_fault(8);

        let a =
            ResilienceFault::with_provenance(canonical_a, ProvenanceId::new(1));
        let b =
            ResilienceFault::with_provenance(canonical_b, ProvenanceId::new(2));

        assert_eq!(a.fault_id(), b.fault_id());
        assert_ne!(a.provenance(), b.provenance());
    }

    #[test]
    fn provenance_identity_is_value_based() {
        let a = ProvenanceId::new(100);
        let b = ProvenanceId::new(100);
        let c = ProvenanceId::new(101);

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(a.value(), 100);
    }

    #[test]
    fn canonical_ordering_is_deterministic() {
        let a = ResilienceFault::from_canonical(test_fault(10));
        let b = ResilienceFault::from_canonical(test_fault(11));

        assert!(a < b);
    }
}