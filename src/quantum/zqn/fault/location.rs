//! Zamani Quantum Noise (ZQN) — Fault locations.
//!
//! This module defines the canonical, backend-independent description of
//! *where* a physical or logical fault/noise event is attached.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - `FaultLocation`;
//! - `FaultLocationKind`;
//! - canonical qubit-location collections;
//! - deterministic normalization and validation of qubit collections;
//! - location classification and identity-domain queries;
//! - location-local errors;
//! - resource-independent global locations;
//! - stable, allocation-aware iteration over location members.
//!
//! This file does NOT own:
//!
//! - fault semantics;
//! - fault probability;
//! - quantum channels;
//! - noise models;
//! - calibration;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - execution;
//! - simulation state;
//! - QEC decoding;
//! - measurement results;
//! - vendor/backend APIs;
//! - quantum-state representations;
//! - resource availability.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Architectural role
//!
//! ```text
//! canonical Quantum IR
//!         │
//!         │ QubitId / PhysicalQubitId / QubitRef
//!         ▼
//!      ZQN location
//!         │
//!         ├──► fault
//!         ├──► correlated fault
//!         ├──► noise model
//!         ├──► simulation
//!         ├──► QEC adapter
//!         ├──► routing cost
//!         └──► hardware execution
//! ```
//!
//! A location answers:
//!
//! > "Which semantic resource(s), if any, does this fault/noise event affect?"
//!
//! It does NOT answer:
//!
//! > "Is this resource available on a particular machine?"
//!
//! > "How are logical qubits mapped to physical qubits?"
//!
//! > "What topology connects these resources?"
//!
//! > "What calibration applies?"
//!
//! Those questions belong elsewhere.
//!
//! # Canonical qubit identity
//!
//! This module MUST use the canonical quantum IR identities:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! crate::quantum::ir::qubit::QubitRef
//! ```
//!
//! It MUST NOT define:
//!
//! ```text
//! ZqnQubitId
//! NoiseQubitId
//! FaultQubitId
//! ResourceQubitId
//! PhysicalResourceId
//! ```
//!
//! or any equivalent replacement for the canonical IR identities.
//!
//! The repository-wide architecture explicitly establishes the IR qubit
//! module as the identity owner. Downstream systems are consumers of those
//! identities, not competing owners. The canonical IR implementation also
//! deliberately keeps logical and physical identities as different Rust types.
//!
//! # Logical versus physical locations
//!
//! A fault may be described before routing, after routing, or at a boundary
//! where both domains are meaningful.
//!
//! Therefore the location model preserves the identity domain:
//!
//! ```text
//! QubitId
//!     logical semantic resource
//!
//! PhysicalQubitId
//!     physical target resource
//!
//! QubitRef::Logical(...)
//! QubitRef::Physical(...)
//! ```
//!
//! A logical identifier is NEVER implicitly converted into a physical
//! identifier merely because both have an integer representation.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once and may be lowered to compatible targets
//! of different sizes and technologies.
//!
//! Consequently this file contains:
//!
//! - no maximum qubit count;
//! - no maximum number of resources in a location;
//! - no maximum correlation arity;
//! - no vendor-specific resource numbering;
//! - no fixed hardware topology;
//! - no fixed gate arity;
//! - no fixed machine size.
//!
//! An implementation may impose an explicit runtime/resource admission limit,
//! but such a limit must come from the caller's resource policy rather than
//! from this semantic location type.
//!
//! "Infinity" in this context means:
//!
//! > there is no artificial finite machine-size ceiling in the semantic API;
//! > actual execution remains bounded by address space, memory, CPU/GPU,
//! > distributed resources, runtime policy, and target capabilities.
//!
//! # Determinism
//!
//! Location semantics are deterministic.
//!
//! A multi-qubit location is canonicalized into a deterministic ordering.
//!
//! Therefore:
//!
//! ```text
//! [p3, p1, p2]
//! ```
//!
//! and
//!
//! ```text
//! [p2, p3, p1]
//! ```
//!
//! represent the same normalized location when they contain the same unique
//! resources.
//!
//! Duplicate resources are rejected rather than silently changing the user's
//! requested fault semantics.
//!
//! No RNG, clock, thread identity, hash-randomization state, filesystem state,
//! or global mutable state participates in location construction.
//!
//! # Resource safety
//!
//! This module:
//!
//! - contains no `unsafe`;
//! - explicitly forbids unsafe code;
//! - performs no I/O;
//! - performs no network access;
//! - performs no allocation merely by importing the module;
//! - imposes no architectural machine-size limit;
//! - does not recursively traverse unbounded structures;
//! - does not create a global cache;
//! - does not own global mutable state.
//!
//! Collection construction necessarily allocates storage proportional to the
//! number of explicitly materialized location members. This is an ordinary
//! resource constraint, not a semantic limit.
//!
//! Callers processing extremely large locations should use streaming/chunked
//! construction at a higher layer rather than expecting one allocation to
//! represent an unbounded collection.
//!
//! # Why `Box<[QubitRef]>`
//!
//! A normalized multi-resource location is immutable after construction.
//!
//! `Box<[QubitRef]>` therefore provides an appropriate owned representation:
//!
//! - contiguous;
//! - immutable;
//! - no spare `Vec` capacity;
//! - deterministic iteration;
//! - cheap cloning when the contained resources are `Copy`;
//! - no hidden global storage.
//!
//! The module does not require callers to materialize huge locations unless
//! they actually need an owned multi-resource location.
//!
//! # Duplicate semantics
//!
//! A multi-resource location represents a set of distinct resources.
//!
//! Therefore:
//!
//! ```text
//! [q0, q1, q2]
//! ```!
//!
//! is valid, while:
//!
//! ```text
//! [q0, q1, q0]
//! ```!
//!
//! is rejected.
//!
//! This is intentionally stricter than silently applying a set operation.
//! Silent deduplication could conceal an upstream compiler, routing, or noise
//! model bug.
//!
//! # Empty locations
//!
//! An empty explicit qubit collection is invalid.
//!
//! An empty collection does not identify a fault target.
//!
//! A global/system-wide fault is represented explicitly by:
//!
//! ```text
//! FaultLocation::Global
//! ```
//!
//! rather than by an empty collection.
//!
//! # Ordering
//!
//! Multi-resource locations are ordered according to the canonical
//! `QubitRef: Ord` implementation supplied by the IR.
//!
//! This gives deterministic equality/hash/iteration semantics without
//! introducing a ZQN-specific identity ordering.
//!
//! # Extensibility
//!
//! The enum deliberately separates:
//!
//! - one-qubit location;
//! - multi-resource location;
//! - global location.
//!
//! It does not invent an artificial hierarchy for future modalities.
//!
//! When Zamani gains canonical identity types for other quantum resources,
//! such as modes, qudits, links, bosonic modes, or distributed quantum
//! resources, those should be introduced by the appropriate canonical IR
//! resource layer first.
//!
//! This file must not invent those identities prematurely.
//!
//! # Integration contracts
//!
//! ## `fault.rs`
//!
//! `Fault` should contain or reference a `FaultLocation`.
//!
//! `FaultLocation` does not contain probability or fault kind.
//!
//! ```text
//! Fault
//! ├── kind
//! ├── location: FaultLocation
//! └── parameters
//! ```
//!
//! ## `correlated.rs`
//!
//! Correlated faults should use `FaultLocation::Qubits` for arbitrary
//! multi-resource correlations. No `TwoQubitLocation`, `ThreeQubitLocation`,
//! or other fixed-arity type should be introduced.
//!
//! ## `batch.rs`
//!
//! Fault batches may iterate over `FaultLocation::Qubits` without modifying
//! the location contract.
//!
//! ## `noise/*`
//!
//! Noise models may map operations or noise events to locations. The location
//! type remains independent of how the noise was generated.
//!
//! ## `calibration/*`
//!
//! Calibration is responsible for determining whether a referenced physical
//! resource exists and what parameters apply. `FaultLocation` performs no
//! hardware lookup.
//!
//! ## `routing/*`
//!
//! Routing may transform logical references into physical references, but the
//! location type does not perform routing itself.
//!
//! ## `scheduling/*`
//!
//! Scheduling may attach time intervals to an existing location at a higher
//! layer. Time is intentionally not embedded here because location and time
//! are independent semantic dimensions.
//!
//! ## `error_correction/*`
//!
//! QEC may convert physical fault locations into syndrome/decoder resources.
//! The QEC layer remains responsible for decoder semantics.
//!
//! ## `hardware/*`
//!
//! Hardware adapters may validate physical locations against target
//! capabilities/topology. This module deliberately does not know hardware.
//!
//! ## `simulation/*`
//!
//! Simulators may consume locations to apply channels/faults to state or
//! trajectory representations. This module contains no simulation state.
//!
//! # Serialization
//!
//! This module does not define an external serialization format.
//!
//! Versioned ZQN serialization belongs under:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! The Rust representation is therefore not itself a wire-format contract.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependency required by this file;
//! - no `unsafe`.
//!
//! The implementation intentionally uses only stable standard-library
//! facilities available to the required toolchain.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. it uses canonical IR qubit identities;
//! 2. it defines no competing qubit identity;
//! 3. it has no hardware assumptions;
//! 4. it has no machine-size constants;
//! 5. it rejects duplicate explicitly supplied resources;
//! 6. it rejects empty explicit multi-resource locations;
//! 7. it provides deterministic canonical ordering;
//! 8. it supports one or arbitrarily many explicitly materialized resources;
//! 9. it supports global/system-wide locations without abusing an empty set;
//! 10. it performs no routing;
//! 11. it performs no calibration lookup;
//! 12. it performs no simulation;
//! 13. it has no global mutable state;
//! 14. it contains no unsafe code;
//! 15. it can be consumed by future ZQN modules without modification;
//! 16. its invariants are locally testable;
//! 17. unrelated future ZQN changes do not require reopening this file.
//!
//! # Tests
//!
//! This file contains focused unit tests for:
//!
//! - logical locations;
//! - physical locations;
//! - global locations;
//! - deterministic normalization;
//! - duplicate rejection;
//! - empty collection rejection;
//! - mixed logical/physical collections;
//! - iteration;
//! - equality and ordering;
//! - resource-domain queries;
//! - boundary values such as `usize::MAX`.
//!
//! No test encodes an architectural maximum qubit count.

// -----------------------------------------------------------------------------
// Safety contract
// -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::error::Error;
use std::fmt;
use std::iter::FromIterator;

// -----------------------------------------------------------------------------
// Canonical quantum identity boundary
// -----------------------------------------------------------------------------
//
// The repository's canonical IR owns these identities.
//
// Do not replace these imports with ZQN-local aliases or new identifier types.

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId, QubitRef};

// ============================================================================
// Fault location
// ============================================================================

/// Canonical location of a ZQN fault/noise event.
///
/// A location is one of:
///
/// - one explicitly identified quantum resource;
/// - multiple explicitly identified quantum resources;
/// - a global/system-wide location.
///
/// `QubitRef` preserves the distinction between logical and physical
/// identities.
///
/// The type is intentionally independent of:
///
/// - fault kind;
/// - probability;
/// - channel;
/// - operation semantics;
/// - hardware topology;
/// - calibration;
/// - time.
///
/// Those concepts can be composed with a location by higher-level modules.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FaultLocation {
    /// A single logical or physical quantum resource.
    Qubit(QubitRef),

    /// A canonical ordered set of two or more distinct logical/physical
    /// quantum resources.
    ///
    /// The collection is guaranteed to be:
    ///
    /// - non-empty;
    /// - duplicate-free;
    /// - sorted according to `QubitRef::Ord`.
    Qubits(Box<[QubitRef]>),

    /// A system/global location.
    ///
    /// This is used when a fault or noise process is not attached to one
    /// explicitly enumerated quantum resource set.
    ///
    /// Examples may include:
    ///
    /// - global environment noise;
    /// - system-wide control noise;
    /// - externally imposed global disturbance.
    ///
    /// The meaning of a global fault is defined by the consuming noise/fault
    /// model, not by this location type.
    Global,
}

impl FaultLocation {
    // ------------------------------------------------------------------------
    // Constructors
    // ------------------------------------------------------------------------

    /// Creates a location for one canonical qubit reference.
    #[must_use]
    pub const fn qubit(resource: QubitRef) -> Self {
        Self::Qubit(resource)
    }

    /// Creates a logical-qubit location.
    #[must_use]
    pub const fn logical(qubit: QubitId) -> Self {
        Self::Qubit(QubitRef::Logical(qubit))
    }

    /// Creates a physical-qubit location.
    #[must_use]
    pub const fn physical(qubit: PhysicalQubitId) -> Self {
        Self::Qubit(QubitRef::Physical(qubit))
    }

    /// Creates a global/system-wide location.
    #[must_use]
    pub const fn global() -> Self {
        Self::Global
    }

    /// Creates a normalized multi-resource location.
    ///
    /// Requirements:
    ///
    /// - at least one resource must be supplied;
    /// - resources must be distinct;
    /// - logical and physical identities remain distinct;
    /// - no implicit logical-to-physical conversion occurs.
    ///
    /// One resource is normalized to `FaultLocation::Qubit` rather than
    /// allocating a collection.
    pub fn qubits<I>(resources: I) -> Result<Self, FaultLocationError>
    where
        I: IntoIterator<Item = QubitRef>,
    {
        let mut resources: Vec<QubitRef> = resources.into_iter().collect();

        if resources.is_empty() {
            return Err(FaultLocationError::EmptyResourceSet);
        }

        resources.sort_unstable();

        if let Some((duplicate, _)) = resources
            .windows(2)
            .find_map(|window| match window {
                [left, right] if left == right => Some((*left, *right)),
                _ => None,
            })
        {
            return Err(FaultLocationError::DuplicateResource { resource: duplicate });
        }

        if resources.len() == 1 {
            return Ok(Self::Qubit(resources[0]));
        }

        Ok(Self::Qubits(resources.into_boxed_slice()))
    }

    /// Creates a normalized multi-resource location from a slice.
    pub fn from_slice(resources: &[QubitRef]) -> Result<Self, FaultLocationError> {
        Self::qubits(resources.iter().copied())
    }

    /// Creates a normalized multi-resource location from logical qubits.
    ///
    /// This is a convenience constructor over the canonical `QubitId` type.
    pub fn logical_qubits<I>(qubits: I) -> Result<Self, FaultLocationError>
    where
        I: IntoIterator<Item = QubitId>,
    {
        Self::qubits(qubits.into_iter().map(QubitRef::Logical))
    }

    /// Creates a normalized multi-resource location from physical qubits.
    ///
    /// This is a convenience constructor over the canonical
    /// `PhysicalQubitId` type.
    pub fn physical_qubits<I>(qubits: I) -> Result<Self, FaultLocationError>
    where
        I: IntoIterator<Item = PhysicalQubitId>,
    {
        Self::qubits(qubits.into_iter().map(QubitRef::Physical))
    }

    // ------------------------------------------------------------------------
    // Classification
    // ------------------------------------------------------------------------

    /// Returns the location kind without exposing internal representation
    /// details.
    #[must_use]
    pub const fn kind(&self) -> FaultLocationKind {
        match self {
            Self::Qubit(_) => FaultLocationKind::Qubit,
            Self::Qubits(_) => FaultLocationKind::Qubits,
            Self::Global => FaultLocationKind::Global,
        }
    }

    /// Returns `true` for a global/system-wide location.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self, Self::Global)
    }

    /// Returns `true` when the location identifies exactly one qubit.
    #[must_use]
    pub const fn is_single_qubit(&self) -> bool {
        matches!(self, Self::Qubit(_))
    }

    /// Returns `true` when the location explicitly identifies multiple
    /// qubits.
    #[must_use]
    pub const fn is_multi_qubit(&self) -> bool {
        matches!(self, Self::Qubits(_))
    }

    /// Returns the number of explicitly identified resources.
    ///
    /// `Global` has no explicitly enumerated resource and therefore returns
    /// zero.
    ///
    /// This is a cardinality of materialized location membership, not a
    /// statement about the size of the underlying quantum machine.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        match self {
            Self::Qubit(_) => 1,
            Self::Qubits(resources) => resources.len(),
            Self::Global => 0,
        }
    }

    /// Returns whether at least one explicit resource is present.
    #[must_use]
    pub fn has_explicit_resources(&self) -> bool {
        self.resource_count() != 0
    }

    // ------------------------------------------------------------------------
    // Identity-domain queries
    // ------------------------------------------------------------------------

    /// Returns `true` if every explicitly identified resource is logical.
    ///
    /// For `Global`, this returns `false` because no explicit resource domain
    /// is available.
    #[must_use]
    pub fn is_logical(&self) -> bool {
        match self {
            Self::Qubit(QubitRef::Logical(_)) => true,
            Self::Qubit(QubitRef::Physical(_)) => false,
            Self::Qubits(resources) => resources
                .iter()
                .all(|resource| resource.is_logical()),
            Self::Global => false,
        }
    }

    /// Returns `true` if every explicitly identified resource is physical.
    ///
    /// For `Global`, this returns `false` because no explicit resource domain
    /// is available.
    #[must_use]
    pub fn is_physical(&self) -> bool {
        match self {
            Self::Qubit(QubitRef::Physical(_)) => true,
            Self::Qubit(QubitRef::Logical(_)) => false,
            Self::Qubits(resources) => resources
                .iter()
                .all(|resource| resource.is_physical()),
            Self::Global => false,
        }
    }

    /// Returns `true` when the location contains both logical and physical
    /// resource references.
    ///
    /// A mixed location is legal because there are legitimate compiler
    /// integration boundaries where both identity domains must be preserved.
    ///
    /// This method does not imply that the logical resource maps to the
    /// physical resource. Mapping belongs to routing.
    #[must_use]
    pub fn is_mixed_identity_domain(&self) -> bool {
        match self {
            Self::Qubit(_) | Self::Global => false,
            Self::Qubits(resources) => {
                let mut has_logical = false;
                let mut has_physical = false;

                for resource in resources.iter().copied() {
                    has_logical |= resource.is_logical();
                    has_physical |= resource.is_physical();

                    if has_logical && has_physical {
                        return true;
                    }
                }

                false
            }
        }
    }

    /// Returns `true` when the location contains at least one logical
    /// resource.
    #[must_use]
    pub fn contains_logical(&self) -> bool {
        match self {
            Self::Qubit(resource) => resource.is_logical(),
            Self::Qubits(resources) => resources.iter().any(|resource| resource.is_logical()),
            Self::Global => false,
        }
    }

    /// Returns `true` when the location contains at least one physical
    /// resource.
    #[must_use]
    pub fn contains_physical(&self) -> bool {
        match self {
            Self::Qubit(resource) => resource.is_physical(),
            Self::Qubits(resources) => resources.iter().any(|resource| resource.is_physical()),
            Self::Global => false,
        }
    }

    // ------------------------------------------------------------------------
    // Membership
    // ------------------------------------------------------------------------

    /// Returns whether the given canonical resource belongs to this location.
    ///
    /// `Global` never reports explicit membership because it does not enumerate
    /// a resource set.
    #[must_use]
    pub fn contains(&self, resource: QubitRef) -> bool {
        match self {
            Self::Qubit(existing) => *existing == resource,
            Self::Qubits(resources) => resources.binary_search(&resource).is_ok(),
            Self::Global => false,
        }
    }

    /// Returns whether the location contains the given logical qubit.
    #[must_use]
    pub fn contains_logical_qubit(&self, qubit: QubitId) -> bool {
        self.contains(QubitRef::Logical(qubit))
    }

    /// Returns whether the location contains the given physical qubit.
    #[must_use]
    pub fn contains_physical_qubit(&self, qubit: PhysicalQubitId) -> bool {
        self.contains(QubitRef::Physical(qubit))
    }

    // ------------------------------------------------------------------------
    // Iteration
    // ------------------------------------------------------------------------

    /// Returns an iterator over explicitly enumerated resources.
    ///
    /// `Global` produces an empty iterator.
    ///
    /// The iterator borrows the location and does not allocate.
    pub fn resources(&self) -> FaultLocationResources<'_> {
        match self {
            Self::Qubit(resource) => FaultLocationResources::Single(Some(resource)),
            Self::Qubits(resources) => FaultLocationResources::Many(resources.iter()),
            Self::Global => FaultLocationResources::Global,
        }
    }

    /// Returns the single resource if this is a single-resource location.
    #[must_use]
    pub const fn single_resource(&self) -> Option<QubitRef> {
        match self {
            Self::Qubit(resource) => Some(*resource),
            Self::Qubits(_) | Self::Global => None,
        }
    }

    /// Returns the canonical multi-resource representation if this is a
    /// multi-resource location.
    ///
    /// A single-resource location deliberately returns `None`; callers can
    /// use `resources()` when they want a uniform iteration API.
    #[must_use]
    pub fn resource_slice(&self) -> Option<&[QubitRef]> {
        match self {
            Self::Qubit(_) | Self::Global => None,
            Self::Qubits(resources) => Some(resources),
        }
    }

    /// Returns the first explicit resource in canonical order.
    ///
    /// Global locations have no explicit first resource.
    #[must_use]
    pub fn first_resource(&self) -> Option<QubitRef> {
        match self {
            Self::Qubit(resource) => Some(*resource),
            Self::Qubits(resources) => resources.first().copied(),
            Self::Global => None,
        }
    }

    /// Returns the last explicit resource in canonical order.
    ///
    /// Global locations have no explicit last resource.
    #[must_use]
    pub fn last_resource(&self) -> Option<QubitRef> {
        match self {
            Self::Qubit(resource) => Some(*resource),
            Self::Qubits(resources) => resources.last().copied(),
            Self::Global => None,
        }
    }

    // ------------------------------------------------------------------------
    // Domain-specific views
    // ------------------------------------------------------------------------

    /// Returns an iterator containing only logical qubits.
    ///
    /// No allocation is performed.
    pub fn logical_resources(&self) -> impl Iterator<Item = QubitId> + '_ {
        self.resources().filter_map(QubitRef::logical)
    }

    /// Returns an iterator containing only physical qubits.
    ///
    /// No allocation is performed.
    pub fn physical_resources(&self) -> impl Iterator<Item = PhysicalQubitId> + '_ {
        self.resources().filter_map(QubitRef::physical)
    }

    // ------------------------------------------------------------------------
    // Validation
    // ------------------------------------------------------------------------

    /// Validates the internal invariants of this location.
    ///
    /// This is intentionally cheap for already-normalized locations.
    ///
    /// The method exists so higher-level modules can validate values at trust
    /// boundaries without knowing the internal representation.
    pub fn validate(&self) -> Result<(), FaultLocationError> {
        match self {
            Self::Qubit(_) | Self::Global => Ok(()),
            Self::Qubits(resources) => {
                if resources.is_empty() {
                    return Err(FaultLocationError::EmptyResourceSet);
                }

                if resources.windows(2).any(|window| window[0] >= window[1]) {
                    return Err(FaultLocationError::NonCanonicalOrder);
                }

                Ok(())
            }
        }
    }

    /// Creates a normalized clone of this location.
    ///
    /// A valid location is already normalized, so this is primarily useful at
    /// an API boundary where the caller wants an explicit normalization step.
    pub fn normalized(&self) -> Result<Self, FaultLocationError> {
        match self {
            Self::Qubit(resource) => Ok(Self::Qubit(*resource)),
            Self::Global => Ok(Self::Global),
            Self::Qubits(resources) => Self::qubits(resources.iter().copied()),
        }
    }
}

// ============================================================================
// Location kind
// ============================================================================

/// Structural category of a fault location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FaultLocationKind {
    /// Exactly one explicitly identified resource.
    Qubit,

    /// Multiple explicitly identified resources.
    Qubits,

    /// No explicit resource enumeration; system/global scope.
    Global,
}

impl FaultLocationKind {
    /// Returns `true` when this kind explicitly identifies resources.
    #[must_use]
    pub const fn has_explicit_resources(self) -> bool {
        !matches!(self, Self::Global)
    }

    /// Returns `true` when this kind represents multiple resources.
    #[must_use]
    pub const fn is_multi_resource(self) -> bool {
        matches!(self, Self::Qubits)
    }

    /// Returns `true` when this kind represents one resource.
    #[must_use]
    pub const fn is_single_resource(self) -> bool {
        matches!(self, Self::Qubit)
    }

    /// Returns `true` when this kind is global/system-wide.
    #[must_use]
    pub const fn is_global(self) -> bool {
        matches!(self, Self::Global)
    }
}

// ============================================================================
// Iterator
// ============================================================================

/// Borrowing iterator over explicit resources in a `FaultLocation`.
///
/// The iterator performs no allocation.
///
/// `Global` produces no resources because it is intentionally not an
/// enumeration of the entire machine.
pub enum FaultLocationResources<'a> {
    /// Iterator containing one borrowed resource.
    Single(Option<&'a QubitRef>),

    /// Iterator over a canonical resource slice.
    Many(std::slice::Iter<'a, QubitRef>),

    /// Global location with no explicit resource enumeration.
    Global,
}

impl<'a> Iterator for FaultLocationResources<'a> {
    type Item = QubitRef;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Single(resource) => resource.take().copied(),
            Self::Many(iter) => iter.next().copied(),
            Self::Global => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = match self {
            Self::Single(resource) => usize::from(resource.is_some()),
            Self::Many(iter) => iter.len(),
            Self::Global => 0,
        };

        (size, Some(size))
    }
}

impl ExactSizeIterator for FaultLocationResources<'_> {}

impl std::iter::FusedIterator for FaultLocationResources<'_> {}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced while constructing or validating a `FaultLocation`.
///
/// This error is deliberately local to this file so `location.rs` can be
/// implemented and tested independently of the future aggregate ZQN error
/// hierarchy.
///
/// A later `core::error` module may provide an adapter from this error into the
/// aggregate ZQN error type. That adapter must not require changing the
/// location semantics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FaultLocationError {
    /// An explicit multi-resource location contained no resources.
    EmptyResourceSet,

    /// The same canonical resource was supplied more than once.
    DuplicateResource {
        /// The duplicated canonical resource.
        resource: QubitRef,
    },

    /// A manually constructed/internal location is not canonically ordered.
    ///
    /// Public constructors normalize automatically, so callers normally only
    /// encounter this error when validating values crossing an internal or
    /// deserialization boundary.
    NonCanonicalOrder,
}

impl fmt::Display for FaultLocationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyResourceSet => {
                write!(formatter, "fault location resource set must not be empty")
            }
            Self::DuplicateResource { resource } => {
                write!(formatter, "fault location contains duplicate resource {resource}")
            }
            Self::NonCanonicalOrder => {
                write!(
                    formatter,
                    "fault location resources are not in canonical strict order"
                )
            }
        }
    }
}

impl Error for FaultLocationError {}

// ============================================================================
// Standard conversions
// ============================================================================

impl From<QubitRef> for FaultLocation {
    fn from(resource: QubitRef) -> Self {
        Self::qubit(resource)
    }
}

impl From<QubitId> for FaultLocation {
    fn from(qubit: QubitId) -> Self {
        Self::logical(qubit)
    }
}

impl From<PhysicalQubitId> for FaultLocation {
    fn from(qubit: PhysicalQubitId) -> Self {
        Self::physical(qubit)
    }
}

impl TryFrom<Vec<QubitRef>> for FaultLocation {
    type Error = FaultLocationError;

    fn try_from(resources: Vec<QubitRef>) -> Result<Self, Self::Error> {
        Self::qubits(resources)
    }
}

impl TryFrom<Box<[QubitRef]>> for FaultLocation {
    type Error = FaultLocationError;

    fn try_from(resources: Box<[QubitRef]>) -> Result<Self, Self::Error> {
        Self::qubits(resources.into_vec())
    }
}

impl FromIterator<QubitRef> for Result<FaultLocation, FaultLocationError> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = QubitRef>,
    {
        FaultLocation::qubits(iter)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn logical(index: usize) -> QubitRef {
        QubitRef::Logical(QubitId::new(index))
    }

    fn physical(index: usize) -> QubitRef {
        QubitRef::Physical(PhysicalQubitId::new(index))
    }

    #[test]
    fn single_logical_location_is_canonical() {
        let location = FaultLocation::logical(QubitId::new(7));

        assert_eq!(
            location,
            FaultLocation::Qubit(QubitRef::Logical(QubitId::new(7)))
        );
        assert_eq!(location.kind(), FaultLocationKind::Qubit);
        assert!(location.is_single_qubit());
        assert!(!location.is_multi_qubit());
        assert!(location.is_logical());
        assert!(!location.is_physical());
        assert_eq!(location.resource_count(), 1);
        assert_eq!(location.single_resource(), Some(logical(7)));
        assert!(location.validate().is_ok());
    }

    #[test]
    fn single_physical_location_is_canonical() {
        let location = FaultLocation::physical(PhysicalQubitId::new(11));

        assert_eq!(
            location,
            FaultLocation::Qubit(QubitRef::Physical(PhysicalQubitId::new(11)))
        );
        assert!(location.is_single_qubit());
        assert!(location.is_physical());
        assert!(!location.is_logical());
        assert_eq!(location.resource_count(), 1);
        assert_eq!(location.single_resource(), Some(physical(11)));
        assert!(location.validate().is_ok());
    }

    #[test]
    fn global_location_has_no_explicit_resources() {
        let location = FaultLocation::global();

        assert!(location.is_global());
        assert_eq!(location.kind(), FaultLocationKind::Global);
        assert_eq!(location.resource_count(), 0);
        assert!(!location.has_explicit_resources());
        assert!(!location.contains_logical(logical(0)));
        assert!(location.resources().next().is_none());
        assert!(location.validate().is_ok());
    }

    #[test]
    fn multi_resource_location_is_sorted_deterministically() {
        let location = FaultLocation::qubits([
            logical(9),
            logical(2),
            logical(5),
            logical(1),
        ])
        .expect("location should be valid");

        let resources: Vec<_> = location.resources().collect();

        assert_eq!(
            resources,
            vec![logical(1), logical(2), logical(5), logical(9)]
        );

        assert_eq!(location.resource_count(), 4);
        assert!(location.is_multi_qubit());
        assert!(location.is_logical());
        assert!(!location.is_physical());
        assert!(location.validate().is_ok());
    }

    #[test]
    fn one_element_collection_becomes_single_location() {
        let location =
            FaultLocation::qubits([logical(4)]).expect("single-resource collection is valid");

        assert_eq!(location, FaultLocation::logical(QubitId::new(4)));
        assert_eq!(location.resource_count(), 1);
        assert!(location.is_single_qubit());
        assert!(!location.is_multi_qubit());
    }

    #[test]
    fn empty_collection_is_rejected() {
        let result = FaultLocation::qubits(std::iter::empty());

        assert_eq!(result, Err(FaultLocationError::EmptyResourceSet));
    }

    #[test]
    fn duplicate_resource_is_rejected() {
        let result = FaultLocation::qubits([
            logical(1),
            logical(2),
            logical(1),
        ]);

        assert_eq!(
            result,
            Err(FaultLocationError::DuplicateResource {
                resource: logical(1),
            })
        );
    }

    #[test]
    fn physical_duplicate_is_rejected() {
        let result = FaultLocation::qubits([
            physical(3),
            physical(3),
        ]);

        assert_eq!(
            result,
            Err(FaultLocationError::DuplicateResource {
                resource: physical(3),
            })
        );
    }

    #[test]
    fn logical_and_physical_same_numeric_index_are_distinct() {
        let logical = logical(5);
        let physical = physical(5);

        assert_ne!(logical, physical);

        let location =
            FaultLocation::qubits([logical, physical]).expect("identity domains are distinct");

        assert_eq!(location.resource_count(), 2);
        assert!(location.is_mixed_identity_domain());
        assert!(location.contains_logical_qubit(QubitId::new(5)));
        assert!(location.contains_physical_qubit(PhysicalQubitId::new(5)));
    }

    #[test]
    fn membership_is_deterministic() {
        let location =
            FaultLocation::qubits([physical(9), physical(1), physical(7)])
                .expect("location should be valid");

        assert!(location.contains_physical_qubit(PhysicalQubitId::new(1)));
        assert!(location.contains_physical_qubit(PhysicalQubitId::new(7)));
        assert!(location.contains_physical_qubit(PhysicalQubitId::new(9)));
        assert!(!location.contains_physical_qubit(PhysicalQubitId::new(8)));

        assert!(location.contains(physical(7)));
        assert!(!location.contains(logical(7)));
    }

    #[test]
    fn logical_and_physical_iterators_are_non_allocating_views() {
        let location = FaultLocation::qubits([
            logical(1),
            physical(2),
            logical(3),
            physical(4),
        ])
        .expect("mixed location should be valid");

        let logicals: Vec<_> = location.logical_resources().collect();
        let physicals: Vec<_> = location.physical_resources().collect();

        assert_eq!(
            logicals,
            vec![QubitId::new(1), QubitId::new(3)]
        );
        assert_eq!(
            physicals,
            vec![PhysicalQubitId::new(2), PhysicalQubitId::new(4)]
        );
    }

    #[test]
    fn first_and_last_resource_follow_canonical_order() {
        let location =
            FaultLocation::qubits([logical(8), logical(2), logical(5)])
                .expect("location should be valid");

        assert_eq!(location.first_resource(), Some(logical(2)));
        assert_eq!(location.last_resource(), Some(logical(8)));
    }

    #[test]
    fn global_has_no_first_or_last_resource() {
        let location = FaultLocation::global();

        assert_eq!(location.first_resource(), None);
        assert_eq!(location.last_resource(), None);
        assert_eq!(location.single_resource(), None);
        assert_eq!(location.resource_slice(), None);
    }

    #[test]
    fn canonical_multi_resource_slice_is_available_without_copy() {
        let location =
            FaultLocation::qubits([logical(7), logical(3), logical(9)])
                .expect("location should be valid");

        let slice = location
            .resource_slice()
            .expect("multi-resource location has a slice");

        assert_eq!(
            slice,
            &[logical(3), logical(7), logical(9)]
        );
    }

    #[test]
    fn normalized_location_is_idempotent() {
        let original =
            FaultLocation::qubits([logical(8), logical(1), logical(4)])
                .expect("location should be valid");

        let normalized = original
            .normalized()
            .expect("already canonical location should remain valid");

        assert_eq!(original, normalized);
    }

    #[test]
    fn resource_iterator_is_exact_size() {
        let location =
            FaultLocation::qubits([logical(1), logical(2), logical(3)])
                .expect("location should be valid");

        let mut iter = location.resources();

        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next(), Some(logical(1)));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some(logical(2)));
        assert_eq!(iter.next(), Some(logical(3)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn single_resource_iterator_does_not_allocate() {
        let location = FaultLocation::logical(QubitId::new(12));

        let resources: Vec<_> = location.resources().collect();

        assert_eq!(resources, vec![logical(12)]);
    }

    #[test]
    fn global_resource_iterator_is_empty() {
        let location = FaultLocation::global();

        assert_eq!(location.resources().count(), 0);
    }

    #[test]
    fn conversion_from_canonical_id_is_typed() {
        let logical_location: FaultLocation = QubitId::new(4).into();
        let physical_location: FaultLocation = PhysicalQubitId::new(4).into();

        assert_eq!(logical_location, FaultLocation::logical(QubitId::new(4)));
        assert_eq!(
            physical_location,
            FaultLocation::physical(PhysicalQubitId::new(4))
        );
    }

    #[test]
    fn vec_conversion_is_normalized() {
        let location = FaultLocation::try_from(vec![
            logical(6),
            logical(1),
            logical(4),
        ])
        .expect("vector should produce a valid location");

        assert_eq!(
            location.resources().collect::<Vec<_>>(),
            vec![logical(1), logical(4), logical(6)]
        );
    }

    #[test]
    fn slice_conversion_is_normalized() {
        let resources = [physical(10), physical(2), physical(7)];

        let location =
            FaultLocation::from_slice(&resources).expect("slice should produce a valid location");

        assert_eq!(
            location.resources().collect::<Vec<_>>(),
            vec![physical(2), physical(7), physical(10)]
        );
    }

    #[test]
    fn boundary_identifier_values_are_supported() {
        let logical_location = FaultLocation::logical(QubitId::new(usize::MAX));
        let physical_location =
            FaultLocation::physical(PhysicalQubitId::new(usize::MAX));

        assert_eq!(
            logical_location.single_resource(),
            Some(logical(usize::MAX))
        );
        assert_eq!(
            physical_location.single_resource(),
            Some(physical(usize::MAX))
        );
    }

    #[test]
    fn boundary_identifiers_do_not_overflow_location_logic() {
        let location = FaultLocation::qubits([
            logical(usize::MAX - 1),
            logical(usize::MAX),
        ])
        .expect("maximum representable identifiers should be valid");

        assert_eq!(location.resource_count(), 2);
        assert!(location.contains_logical_qubit(QubitId::new(usize::MAX)));
        assert!(location.contains_logical_qubit(QubitId::new(usize::MAX - 1)));
    }

    #[test]
    fn ordering_is_canonical() {
        let a =
            FaultLocation::qubits([logical(1), physical(2)])
                .expect("location should be valid");

        let b =
            FaultLocation::qubits([physical(2), logical(1)])
                .expect("location should normalize");

        assert_eq!(a, b);
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Equal);
    }

    #[test]
    fn global_is_distinct_from_empty_explicit_location() {
        assert_eq!(
            FaultLocation::qubits([]),
            Err(FaultLocationError::EmptyResourceSet)
        );

        assert_ne!(
            FaultLocation::global(),
            FaultLocation::qubit(logical(0))
        );
    }

    #[test]
    fn error_messages_are_stable_and_actionable() {
        assert_eq!(
            FaultLocationError::EmptyResourceSet.to_string(),
            "fault location resource set must not be empty"
        );

        assert_eq!(
            FaultLocationError::DuplicateResource {
                resource: logical(3)
            }
            .to_string(),
            "fault location contains duplicate resource q3"
        );
    }

    #[test]
    fn from_iterator_contract_works() {
        let result: Result<FaultLocation, FaultLocationError> =
            [logical(5), logical(1), logical(3)]
                .into_iter()
                .collect();

        let location = result.expect("iterator should construct a location");

        assert_eq!(
            location.resources().collect::<Vec<_>>(),
            vec![logical(1), logical(3), logical(5)]
        );
    }
}