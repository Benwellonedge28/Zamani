//! Zamani Quantum IR — Scheduling Placement
//!
//! `src/quantum/ir/scheduling/placement.rs`
//!
//! Production-grade, hardware-independent representation of qubit placement
//! state at the scheduling boundary.
//!
//! # Architectural role
//!
//! This module represents WHERE logical quantum resources are located for a
//! scheduling context. It does not decide the placement and it does not
//! schedule operations.
//!
//! The architectural separation is:
//!
//! ```text
//! quantum::ir::qubit
//!     |
//!     | canonical logical/physical identities
//!     v
//! resources::mapping
//!     |
//!     | logical -> physical mapping
//!     v
//! scheduling::placement
//!     |
//!     | placement state/snapshot used by scheduling
//!     v
//! scheduling
//!     |
//!     | decides WHEN operations execute
//!     v
//! backend / hardware lowering
//! ```
//!
//! Routing/layout algorithms may construct or update a placement.
//!
//! The scheduler consumes a placement.
//!
//! Hardware validation occurs outside this module.
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - immutable placement snapshots;
//! - mutable placement state;
//! - logical-to-physical lookup;
//! - physical-to-logical lookup;
//! - placement membership;
//! - placement occupancy;
//! - atomic placement changes;
//! - logical-qubit movement between physical resources;
//! - logical-qubit exchange;
//! - placement validation;
//! - deterministic iteration;
//! - placement epochs/generations;
//! - placement change records;
//! - placement state comparison;
//! - construction from canonical `QubitMapping`;
//! - conversion back to canonical `QubitMapping`;
//! - explicit partial/complete placement semantics;
//! - resource-independent placement contracts.
//!
//! # Non-responsibilities
//!
//! This module MUST NOT:
//!
//! - inspect hardware;
//! - discover topology;
//! - select a routing algorithm;
//! - calculate shortest paths;
//! - insert SWAP operations;
//! - optimize circuits;
//! - schedule operations;
//! - calculate operation durations;
//! - generate pulses;
//! - access calibration data;
//! - execute a QPU;
//! - simulate quantum state;
//! - perform QEC decoding;
//! - communicate with a backend;
//! - assume a vendor;
//! - assume a topology;
//! - impose a fixed machine qubit count;
//! - define a second `QubitId`;
//! - define a second `PhysicalQubitId`.
//!
//! # Canonical identity boundary
//!
//! The ONLY canonical qubit identity types used here are:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module intentionally imports those types rather than creating
//! scheduling-local identifiers.
//!
//! # Placement versus mapping
//!
//! `resources::mapping::QubitMapping` represents a logical-to-physical
//! assignment as a resource-level semantic mapping.
//!
//! `Placement` represents that assignment as the placement state visible to
//! the scheduling layer.
//!
//! A placement therefore does not replace `QubitMapping`.
//!
//! ```text
//! QubitMapping
//!     = canonical mapping value
//!
//! Placement
//!     = scheduling placement state
//! ```
//!
//! The distinction allows scheduling to track state changes without forcing
//! the resource-mapping layer to know anything about scheduling.
//!
//! # Dynamic placement
//!
//! A placement can change during a compilation/execution plan.
//!
//! Example:
//!
//! ```text
//! initial:
//!     q0 -> p0
//!     q1 -> p1
//!
//! transition:
//!     q0 -> p1
//!     q1 -> p0
//! ```
//!
//! Such a transition is represented as placement state. This module does NOT
//! insert a SWAP instruction. A routing/lowering layer may interpret the
//! transition and generate the appropriate operation sequence.
//!
//! # Scalability
//!
//! There is NO architectural maximum such as:
//!
//! ```text
//! 32
//! 64
//! 127
//! 4096
//! 1_000_000
//! ```
//!
//! A placement for one qubit and a placement for the largest finite mapping
//! representable by the execution environment use the same data model.
//!
//! Sparse identifiers are supported.
//!
//! The practical limits are:
//!
//! 1. the canonical identifier representation;
//! 2. available memory;
//! 3. explicit caller/resource/security policies.
//!
//! This module never interprets a numeric identifier as a machine capacity.
//!
//! # Determinism
//!
//! Placement entries are maintained in `BTreeMap` through the canonical
//! `QubitMapping` implementation.
//!
//! Therefore:
//!
//! - logical iteration is deterministic;
//! - physical iteration is deterministic;
//! - snapshots are deterministic;
//! - validation is deterministic;
//! - exported mappings are deterministic.
//!
//! No `HashMap` iteration order is observable through this API.
//!
//! # Atomicity
//!
//! Placement mutation is transactional at the public API boundary.
//!
//! A failed operation leaves the placement unchanged.
//!
//! In particular, relocation and exchange perform conflict validation before
//! modifying the underlying mapping.
//!
//! # Epoch semantics
//!
//! Every successful state-changing operation increments the placement epoch.
//!
//! The epoch is:
//!
//! - monotonic within one `Placement` instance;
//! - useful for detecting stale scheduler views;
//! - not a globally unique identifier;
//! - not a timestamp;
//! - not a cryptographic identity.
//!
//! Epoch overflow is rejected instead of wrapping.
//!
//! # Version independence
//!
//! This file intentionally does not depend on IR version, serialization,
//! hashing, routing, topology, hardware, or scheduler implementation types.
//!
//! Those systems may consume this API without requiring changes to this file.
//!
//! # Serialization boundary
//!
//! This module does not define a serialization format.
//!
//! Consumers should serialize:
//!
//! - epoch when semantically required;
//! - placement entries from `iter()`;
//! - completeness/domain information when required by the surrounding IR.
//!
//! Serialization must not depend on map iteration outside this API.
//!
//! # Hashing boundary
//!
//! This module does not choose a cryptographic hash algorithm.
//!
//! Canonical hashing should consume `iter()` in deterministic order and hash
//! semantic placement information only.
//!
//! # Security
//!
//! The module:
//!
//! - contains no unsafe code;
//! - performs checked epoch arithmetic;
//! - never trusts physical existence;
//! - never trusts physical availability;
//! - never assumes topology validity;
//! - never silently resolves mapping collisions;
//! - never silently discards placement information.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust.
//!
//! No nightly features.
//! No external dependencies.
//! No unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! ```text
//! quantum::ir::qubit
//! quantum::ir::resources::mapping
//! ```
//!
//! Downstream:
//!
//! ```text
//! quantum::ir::scheduling
//! quantum::routing
//! quantum::hardware
//! quantum::ir::validation
//! quantum::ir::analysis
//! quantum::backend
//! ```
//!
//! None of the downstream systems are imported here.
//!
//! Consequently, changes to routing, topology, hardware, scheduling,
//! optimization, backend lowering, serialization, or hashing do not require
//! changes to this file merely because those implementations evolve.
//!
//! # Completion invariant
//!
//! This file is complete when:
//!
//! 1. logical and physical identities come exclusively from `qubit.rs`;
//! 2. canonical mapping comes exclusively from `resources::mapping`;
//! 3. placement can represent partial mappings;
//! 4. placement can represent complete mappings;
//! 5. sparse identifiers work;
//! 6. no machine-size constant exists;
//! 7. relocation is atomic;
//! 8. exchange is atomic;
//! 9. collisions are rejected;
//! 10. deterministic iteration is guaranteed;
//! 11. epoch overflow is rejected;
//! 12. stale-placement detection is possible;
//! 13. mapping conversion is lossless;
//! 14. no topology is assumed;
//! 15. no routing algorithm is embedded;
//! 16. no scheduler policy is embedded;
//! 17. no hardware implementation is embedded;
//! 18. no unsafe Rust exists;
//! 19. future scheduling/routing implementations can consume this API without
//!     changing its semantic contracts.
//!
//! # Example
//!
//! ```ignore
//! use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
//! use crate::quantum::ir::scheduling::placement::Placement;
//!
//! let mut placement = Placement::new();
//!
//! placement.assign(
//!     QubitId::new(0),
//!     PhysicalQubitId::new(7),
//! )?;
//!
//! assert_eq!(
//!     placement.physical_for(QubitId::new(0)),
//!     Some(PhysicalQubitId::new(7)),
//! );
//!
//! # Ok::<(), crate::quantum::ir::scheduling::placement::PlacementError>(())
//! ```

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use super::super::qubit::{PhysicalQubitId, QubitId};
use super::super::resources::mapping::{
    MappingDomain,
    MappingEntry,
    QubitMapping,
};

// =============================================================================
// Placement epoch
// =============================================================================

/// Monotonically increasing placement-state generation.
///
/// An epoch identifies a version of one placement instance.
///
/// It is NOT:
//!
//! - a timestamp;
//! - a globally unique identifier;
//! - a cryptographic digest;
//! - a machine identifier.
///
/// Epoch zero represents the initial state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlacementEpoch(u64);

impl PlacementEpoch {
    /// Initial placement epoch.
    pub const ZERO: Self = Self(0);

    /// Creates an epoch from a raw value.
    ///
    /// This constructor is primarily useful at controlled integration
    /// boundaries and during deserialization.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying epoch value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the next epoch.
    ///
    /// Overflow is reported instead of wrapping.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl Default for PlacementEpoch {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for PlacementEpoch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// =============================================================================
// Placement completeness
// =============================================================================

/// Describes whether a placement is partial or complete for its declared
/// logical domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlacementCompleteness {
    /// The placement does not claim that every logical qubit is mapped.
    Partial,

    /// Every logical qubit in the declared domain has a physical location.
    Complete,
}

impl PlacementCompleteness {
    /// Returns whether this represents a complete placement.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Returns whether this represents a partial placement.
    #[must_use]
    pub const fn is_partial(self) -> bool {
        matches!(self, Self::Partial)
    }
}

impl Default for PlacementCompleteness {
    fn default() -> Self {
        Self::Partial
    }
}

// =============================================================================
// Placement change kind
// =============================================================================

/// Semantic kind of a successful placement change.
///
/// These values describe state changes only. They do not prescribe which
/// quantum instruction, if any, must realize the change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlacementChangeKind {
    /// A new logical-to-physical assignment was created.
    Assignment,

    /// A logical qubit changed physical location.
    Relocation,

    /// Two logical qubits exchanged physical locations.
    Exchange,

    /// An existing assignment was removed.
    Removal,

    /// The placement was replaced by another mapping state.
    Replacement,
}

impl fmt::Display for PlacementChangeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assignment => formatter.write_str("assignment"),
            Self::Relocation => formatter.write_str("relocation"),
            Self::Exchange => formatter.write_str("exchange"),
            Self::Removal => formatter.write_str("removal"),
            Self::Replacement => formatter.write_str("replacement"),
        }
    }
}

// =============================================================================
// Placement change
// =============================================================================

/// Immutable description of one successful placement-state transition.
///
/// A change record is semantic metadata. It does not contain a scheduling
/// operation or routing instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlacementChange {
    epoch: PlacementEpoch,
    kind: PlacementChangeKind,
    logical: QubitId,
    previous: Option<PhysicalQubitId>,
    current: Option<PhysicalQubitId>,
}

impl PlacementChange {
    /// Creates a placement change record.
    #[must_use]
    pub const fn new(
        epoch: PlacementEpoch,
        kind: PlacementChangeKind,
        logical: QubitId,
        previous: Option<PhysicalQubitId>,
        current: Option<PhysicalQubitId>,
    ) -> Self {
        Self {
            epoch,
            kind,
            logical,
            previous,
            current,
        }
    }

    /// Returns the resulting epoch.
    #[must_use]
    pub const fn epoch(self) -> PlacementEpoch {
        self.epoch
    }

    /// Returns the change kind.
    #[must_use]
    pub const fn kind(self) -> PlacementChangeKind {
        self.kind
    }

    /// Returns the logical qubit affected by this record.
    #[must_use]
    pub const fn logical(self) -> QubitId {
        self.logical
    }

    /// Returns the previous physical location.
    #[must_use]
    pub const fn previous(self) -> Option<PhysicalQubitId> {
        self.previous
    }

    /// Returns the resulting physical location.
    #[must_use]
    pub const fn current(self) -> Option<PhysicalQubitId> {
        self.current
    }
}

// =============================================================================
// Placement errors
// =============================================================================

/// Errors produced by placement operations.
///
/// This error type is deliberately local to this file. Higher-level IR error
/// aggregation may wrap it without creating a dependency cycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlacementError {
    /// The logical qubit is already assigned to the requested physical
    /// resource.
    LogicalAlreadyAssigned {
        logical: QubitId,
        physical: PhysicalQubitId,
    },

    /// The logical qubit already has a different physical assignment.
    LogicalAlreadyMapped {
        logical: QubitId,
        existing: PhysicalQubitId,
        requested: PhysicalQubitId,
    },

    /// The physical resource is already occupied.
    PhysicalAlreadyOccupied {
        physical: PhysicalQubitId,
        existing: QubitId,
        requested: QubitId,
    },

    /// The requested logical qubit has no current physical placement.
    LogicalNotPlaced {
        logical: QubitId,
    },

    /// The requested physical resource has no logical owner.
    PhysicalNotOccupied {
        physical: PhysicalQubitId,
    },

    /// The two logical qubits supplied for an exchange are identical.
    SameLogicalQubit {
        logical: QubitId,
    },

    /// A supplied logical domain is not completely represented by the
    /// placement.
    IncompletePlacement {
        expected: usize,
        placed: usize,
    },

    /// The placement contains an internal consistency error.
    InvariantViolation {
        detail: String,
    },

    /// Epoch advancement would overflow.
    EpochOverflow,

    /// The caller supplied a stale epoch.
    StaleEpoch {
        expected: PlacementEpoch,
        actual: PlacementEpoch,
    },

    /// A supplied mapping could not be converted into a valid placement.
    MappingFailure {
        detail: String,
    },
}

impl fmt::Display for PlacementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LogicalAlreadyAssigned { logical, physical } => write!(
                formatter,
                "logical qubit {logical} is already assigned to physical qubit {physical}"
            ),

            Self::LogicalAlreadyMapped {
                logical,
                existing,
                requested,
            } => write!(
                formatter,
                "logical qubit {logical} is already mapped to {existing}; \
                 cannot assign it to {requested}"
            ),

            Self::PhysicalAlreadyOccupied {
                physical,
                existing,
                requested,
            } => write!(
                formatter,
                "physical qubit {physical} is occupied by logical qubit {existing}; \
                 cannot assign logical qubit {requested}"
            ),

            Self::LogicalNotPlaced { logical } => {
                write!(formatter, "logical qubit {logical} has no placement")
            }

            Self::PhysicalNotOccupied { physical } => {
                write!(formatter, "physical qubit {physical} has no logical owner")
            }

            Self::SameLogicalQubit { logical } => {
                write!(formatter, "cannot exchange logical qubit {logical} with itself")
            }

            Self::IncompletePlacement { expected, placed } => write!(
                formatter,
                "placement is incomplete: expected {expected} logical qubits, \
                 but {placed} are placed"
            ),

            Self::InvariantViolation { detail } => {
                write!(formatter, "placement invariant violation: {detail}")
            }

            Self::EpochOverflow => {
                write!(formatter, "placement epoch overflow")
            }

            Self::StaleEpoch { expected, actual } => write!(
                formatter,
                "stale placement epoch: expected {expected}, actual {actual}"
            ),

            Self::MappingFailure { detail } => {
                write!(formatter, "placement mapping failure: {detail}")
            }
        }
    }
}

impl Error for PlacementError {}

// =============================================================================
// Placement snapshot
// =============================================================================

/// Immutable placement snapshot.
///
/// A snapshot is cheap to clone relative to the underlying mapping ownership
/// model and is suitable for passing a stable placement view to a scheduler,
/// validator, or backend integration.
///
/// The snapshot cannot mutate the placement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacementSnapshot {
    mapping: QubitMapping,
    epoch: PlacementEpoch,
}

impl PlacementSnapshot {
    /// Creates a snapshot from an existing canonical mapping.
    #[must_use]
    pub const fn from_mapping(
        mapping: QubitMapping,
        epoch: PlacementEpoch,
    ) -> Self {
        Self { mapping, epoch }
    }

    /// Returns the placement epoch represented by this snapshot.
    #[must_use]
    pub const fn epoch(&self) -> PlacementEpoch {
        self.epoch
    }

    /// Returns the number of placed logical qubits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.mapping.len()
    }

    /// Returns whether no logical qubits are placed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.mapping.is_empty()
    }

    /// Looks up the physical resource for a logical qubit.
    #[must_use]
    pub fn physical_for(
        &self,
        logical: QubitId,
    ) -> Option<PhysicalQubitId> {
        self.mapping.physical_for(logical)
    }

    /// Looks up the logical qubit occupying a physical resource.
    #[must_use]
    pub fn logical_for(
        &self,
        physical: PhysicalQubitId,
    ) -> Option<QubitId> {
        self.mapping.logical_for(physical)
    }

    /// Returns whether a logical qubit is currently placed.
    #[must_use]
    pub fn contains_logical(&self, logical: QubitId) -> bool {
        self.mapping.contains_logical(logical)
    }

    /// Returns whether a physical resource is occupied.
    #[must_use]
    pub fn contains_physical(&self, physical: PhysicalQubitId) -> bool {
        self.mapping.contains_physical(physical)
    }

    /// Returns deterministic logical-to-physical entries.
    pub fn iter(&self) -> impl Iterator<Item = MappingEntry> + '_ {
        self.mapping.iter()
    }

    /// Returns the underlying canonical mapping.
    ///
    /// The mapping is cloned so callers cannot mutate the snapshot.
    #[must_use]
    pub fn to_mapping(&self) -> QubitMapping {
        self.mapping.clone()
    }

    /// Checks completeness against an explicit logical domain.
    pub fn validate_complete(
        &self,
        domain: MappingDomain,
    ) -> Result<(), PlacementError> {
        self.mapping
            .validate_complete(domain)
            .map_err(|error| PlacementError::MappingFailure {
                detail: error.to_string(),
            })
    }

    /// Returns whether the placement is complete for a supplied domain.
    #[must_use]
    pub fn is_complete_for(&self, domain: MappingDomain) -> bool {
        self.mapping.is_complete_for(domain)
    }
}

// =============================================================================
// Placement
// =============================================================================

/// Mutable scheduling placement state.
///
/// `Placement` is the scheduling-facing state of a logical-to-physical
/// placement.
///
/// It is intentionally independent from topology and routing.
///
/// # Invariants
///
/// A valid `Placement` always satisfies:
///
/// ```text
/// one logical qubit -> at most one physical qubit
/// one physical qubit -> at most one logical qubit
/// ```
///
/// These invariants are delegated to the canonical `QubitMapping`.
///
/// # No hardware assumptions
///
/// A physical ID stored here does not prove that a physical qubit exists,
/// is online, is calibrated, or supports an operation.
///
/// Hardware validation remains downstream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Placement {
    mapping: QubitMapping,
    epoch: PlacementEpoch,
}

impl Default for Placement {
    fn default() -> Self {
        Self::new()
    }
}

impl Placement {
    /// Creates an empty placement.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            mapping: QubitMapping::new(),
            epoch: PlacementEpoch::ZERO,
        }
    }

    /// Creates a placement from a canonical mapping.
    ///
    /// The supplied mapping is validated before construction.
    pub fn from_mapping(mapping: QubitMapping) -> Result<Self, PlacementError> {
        mapping
            .validate()
            .map_err(|error| PlacementError::MappingFailure {
                detail: error.to_string(),
            })?;

        Ok(Self {
            mapping,
            epoch: PlacementEpoch::ZERO,
        })
    }

    /// Creates a placement from a canonical mapping and epoch.
    ///
    /// This constructor is intended for controlled restoration/deserialization
    /// boundaries where the epoch has already been validated externally.
    pub fn from_mapping_at_epoch(
        mapping: QubitMapping,
        epoch: PlacementEpoch,
    ) -> Result<Self, PlacementError> {
        mapping
            .validate()
            .map_err(|error| PlacementError::MappingFailure {
                detail: error.to_string(),
            })?;

        Ok(Self { mapping, epoch })
    }

    /// Returns the current placement epoch.
    #[must_use]
    pub const fn epoch(&self) -> PlacementEpoch {
        self.epoch
    }

    /// Returns the number of placed logical qubits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.mapping.len()
    }

    /// Returns whether the placement contains no assignments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.mapping.is_empty()
    }

    /// Returns the current immutable snapshot.
    #[must_use]
    pub fn snapshot(&self) -> PlacementSnapshot {
        PlacementSnapshot::from_mapping(self.mapping.clone(), self.epoch)
    }

    /// Returns the underlying canonical mapping as an owned value.
    #[must_use]
    pub fn to_mapping(&self) -> QubitMapping {
        self.mapping.clone()
    }

    /// Returns the physical resource assigned to a logical qubit.
    #[must_use]
    pub fn physical_for(
        &self,
        logical: QubitId,
    ) -> Option<PhysicalQubitId> {
        self.mapping.physical_for(logical)
    }

    /// Returns the logical qubit occupying a physical resource.
    #[must_use]
    pub fn logical_for(
        &self,
        physical: PhysicalQubitId,
    ) -> Option<QubitId> {
        self.mapping.logical_for(physical)
    }

    /// Returns whether a logical qubit is placed.
    #[must_use]
    pub fn contains_logical(&self, logical: QubitId) -> bool {
        self.mapping.contains_logical(logical)
    }

    /// Returns whether a physical resource is occupied.
    #[must_use]
    pub fn contains_physical(&self, physical: PhysicalQubitId) -> bool {
        self.mapping.contains_physical(physical)
    }

    /// Returns deterministic mapping entries.
    pub fn iter(&self) -> impl Iterator<Item = MappingEntry> + '_ {
        self.mapping.iter()
    }

    /// Returns all occupied physical resources in deterministic order.
    pub fn occupied_physical(
        &self,
    ) -> impl Iterator<Item = PhysicalQubitId> + '_ {
        self.mapping
            .iter()
            .map(|entry| entry.physical())
    }

    /// Returns all placed logical qubits in deterministic order.
    pub fn placed_logical(
        &self,
    ) -> impl Iterator<Item = QubitId> + '_ {
        self.mapping.iter().map(|entry| entry.logical())
    }

    /// Assigns a previously unplaced logical qubit to an unoccupied physical
    /// resource.
    ///
    /// The operation is atomic.
    pub fn assign(
        &mut self,
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> Result<PlacementChange, PlacementError> {
        if let Some(existing) = self.mapping.physical_for(logical) {
            if existing == physical {
                return Err(PlacementError::LogicalAlreadyAssigned {
                    logical,
                    physical,
                });
            }

            return Err(PlacementError::LogicalAlreadyMapped {
                logical,
                existing,
                requested: physical,
            });
        }

        if let Some(existing) = self.mapping.logical_for(physical) {
            return Err(PlacementError::PhysicalAlreadyOccupied {
                physical,
                existing,
                requested: logical,
            });
        }

        self.mapping
            .insert(logical, physical)
            .map_err(|error| PlacementError::MappingFailure {
                detail: error.to_string(),
            })?;

        let epoch = self.advance_epoch()?;

        Ok(PlacementChange::new(
            epoch,
            PlacementChangeKind::Assignment,
            logical,
            None,
            Some(physical),
        ))
    }

    /// Relocates an already placed logical qubit to an unoccupied physical
    /// resource.
    ///
    /// This changes placement state only. It does not generate a SWAP,
    /// transport operation, teleportation, or routing sequence.
    pub fn relocate(
        &mut self,
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> Result<PlacementChange, PlacementError> {
        let previous = self
            .mapping
            .physical_for(logical)
            .ok_or(PlacementError::LogicalNotPlaced { logical })?;

        if previous == physical {
            return Err(PlacementError::LogicalAlreadyAssigned {
                logical,
                physical,
            });
        }

        if let Some(existing) = self.mapping.logical_for(physical) {
            return Err(PlacementError::PhysicalAlreadyOccupied {
                physical,
                existing,
                requested: logical,
            });
        }

        // The mapping implementation validates the replacement atomically.
        self.mapping
            .replace(logical, physical)
            .map_err(|error| PlacementError::MappingFailure {
                detail: error.to_string(),
            })?;

        let epoch = self.advance_epoch()?;

        Ok(PlacementChange::new(
            epoch,
            PlacementChangeKind::Relocation,
            logical,
            Some(previous),
            Some(physical),
        ))
    }

    /// Exchanges the physical locations of two logical qubits.
    ///
    /// The operation is atomic.
    ///
    /// If either logical qubit is not placed, the placement is unchanged.
    pub fn exchange(
        &mut self,
        first: QubitId,
        second: QubitId,
    ) -> Result<PlacementChange, PlacementError> {
        if first == second {
            return Err(PlacementError::SameLogicalQubit { logical: first });
        }

        let first_physical = self
            .mapping
            .physical_for(first)
            .ok_or(PlacementError::LogicalNotPlaced { logical: first })?;

        let second_physical = self
            .mapping
            .physical_for(second)
            .ok_or(PlacementError::LogicalNotPlaced { logical: second })?;

        self.mapping
            .swap(first, second)
            .map_err(|error| PlacementError::MappingFailure {
                detail: error.to_string(),
            })?;

        let epoch = self.advance_epoch()?;

        Ok(PlacementChange::new(
            epoch,
            PlacementChangeKind::Exchange,
            first,
            Some(first_physical),
            Some(second_physical),
        ))
    }

    /// Removes the placement of a logical qubit.
    ///
    /// The physical resource becomes unoccupied in this placement state.
    ///
    /// The physical resource itself is not destroyed or released from
    /// hardware. Hardware resource management belongs elsewhere.
    pub fn remove(
        &mut self,
        logical: QubitId,
    ) -> Result<PlacementChange, PlacementError> {
        let previous = self
            .mapping
            .physical_for(logical)
            .ok_or(PlacementError::LogicalNotPlaced { logical })?;

        self.mapping
            .remove_logical(logical)
            .map_err(|error| PlacementError::MappingFailure {
                detail: error.to_string(),
            })?;

        let epoch = self.advance_epoch()?;

        Ok(PlacementChange::new(
            epoch,
            PlacementChangeKind::Removal,
            logical,
            Some(previous),
            None,
        ))
    }

    /// Replaces the complete placement mapping atomically.
    ///
    /// If validation or epoch advancement fails, the existing placement is
    /// preserved.
    pub fn replace_mapping(
        &mut self,
        mapping: QubitMapping,
    ) -> Result<PlacementChange, PlacementError> {
        mapping
            .validate()
            .map_err(|error| PlacementError::MappingFailure {
                detail: error.to_string(),
            })?;

        let old_mapping = self.mapping.clone();

        // Validate epoch before changing state.
        let next_epoch = self
            .epoch
            .checked_next()
            .ok_or(PlacementError::EpochOverflow)?;

        self.mapping = mapping;
        self.epoch = next_epoch;

        let logical = old_mapping
            .iter()
            .next()
            .map(|entry| entry.logical())
            .or_else(|| self.mapping.iter().next().map(|entry| entry.logical()))
            .unwrap_or_else(|| QubitId::new(0));

        Ok(PlacementChange::new(
            self.epoch,
            PlacementChangeKind::Replacement,
            logical,
            old_mapping.physical_for(logical),
            self.mapping.physical_for(logical),
        ))
    }

    /// Validates placement completeness against an explicitly supplied logical
    /// domain.
    pub fn validate_complete(
        &self,
        domain: MappingDomain,
    ) -> Result<(), PlacementError> {
        self.mapping
            .validate_complete(domain)
            .map_err(|error| PlacementError::MappingFailure {
                detail: error.to_string(),
            })
    }

    /// Returns whether every logical qubit in the supplied domain is placed.
    #[must_use]
    pub fn is_complete_for(&self, domain: MappingDomain) -> bool {
        self.mapping.is_complete_for(domain)
    }

    /// Returns the placement completeness classification for a domain.
    #[must_use]
    pub fn completeness_for(
        &self,
        domain: MappingDomain,
    ) -> PlacementCompleteness {
        if self.is_complete_for(domain) {
            PlacementCompleteness::Complete
        } else {
            PlacementCompleteness::Partial
        }
    }

    /// Checks all internal placement invariants.
    pub fn validate(&self) -> Result<(), PlacementError> {
        self.mapping
            .validate()
            .map_err(|error| PlacementError::InvariantViolation {
                detail: error.to_string(),
            })
    }

    /// Verifies that a caller is operating against the current placement
    /// epoch.
    ///
    /// This is useful when a scheduler or another compiler phase retains a
    /// snapshot and later attempts to apply a state-dependent operation.
    pub fn require_epoch(
        &self,
        expected: PlacementEpoch,
    ) -> Result<(), PlacementError> {
        if self.epoch != expected {
            return Err(PlacementError::StaleEpoch {
                expected,
                actual: self.epoch,
            });
        }

        Ok(())
    }

    /// Returns the number of occupied physical resources.
    #[must_use]
    pub fn occupied_count(&self) -> usize {
        self.mapping.len()
    }

    /// Returns the number of placed logical qubits.
    #[must_use]
    pub fn placed_count(&self) -> usize {
        self.mapping.len()
    }

    fn advance_epoch(&mut self) -> Result<PlacementEpoch, PlacementError> {
        let next = self
            .epoch
            .checked_next()
            .ok_or(PlacementError::EpochOverflow)?;

        self.epoch = next;

        Ok(next)
    }
}

// =============================================================================
// Placement comparison
// =============================================================================

/// Compares two placement snapshots without imposing a routing policy.
///
/// This is useful for scheduler cache invalidation and deterministic
/// compilation diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlacementDifference {
    /// Both placements contain exactly the same logical-to-physical mapping.
    Identical,

    /// A logical qubit was newly placed.
    Added {
        logical: QubitId,
        physical: PhysicalQubitId,
    },

    /// A logical qubit changed physical location.
    Moved {
        logical: QubitId,
        previous: PhysicalQubitId,
        current: PhysicalQubitId,
    },

    /// A logical qubit lost its placement.
    Removed {
        logical: QubitId,
        previous: PhysicalQubitId,
    },
}

impl PlacementDifference {
    /// Returns the logical qubit associated with this difference.
    #[must_use]
    pub const fn logical(&self) -> Option<QubitId> {
        match self {
            Self::Identical => None,
            Self::Added { logical, .. }
            | Self::Moved { logical, .. }
            | Self::Removed { logical, .. } => Some(*logical),
        }
    }
}

/// Produces deterministic differences between two placement snapshots.
///
/// When several logical qubits differ, the returned records are ordered by
/// logical-qubit identifier.
pub fn diff(
    before: &PlacementSnapshot,
    after: &PlacementSnapshot,
) -> Vec<PlacementDifference> {
    let mut logicals = BTreeSet::new();

    for entry in before.iter() {
        logicals.insert(entry.logical());
    }

    for entry in after.iter() {
        logicals.insert(entry.logical());
    }

    let mut differences = Vec::new();

    for logical in logicals {
        match (
            before.physical_for(logical),
            after.physical_for(logical),
        ) {
            (None, None) => {}

            (None, Some(current)) => {
                differences.push(PlacementDifference::Added {
                    logical,
                    physical: current,
                });
            }

            (Some(previous), None) => {
                differences.push(PlacementDifference::Removed {
                    logical,
                    previous,
                });
            }

            (Some(previous), Some(current)) if previous == current => {}

            (Some(previous), Some(current)) => {
                differences.push(PlacementDifference::Moved {
                    logical,
                    previous,
                    current,
                });
            }
        }
    }

    if differences.is_empty() {
        vec![PlacementDifference::Identical]
    } else {
        differences
    }
}

// =============================================================================
// Placement validation helpers
// =============================================================================

/// Validates that all entries in a placement are injective and internally
/// consistent.
///
/// This is intentionally a standalone function so validators and integration
/// tests can validate snapshots without obtaining mutable placement state.
pub fn validate_snapshot(
    snapshot: &PlacementSnapshot,
) -> Result<(), PlacementError> {
    snapshot
        .mapping
        .validate()
        .map_err(|error| PlacementError::InvariantViolation {
            detail: error.to_string(),
        })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn logical(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn physical(index: usize) -> PhysicalQubitId {
        PhysicalQubitId::new(index)
    }

    #[test]
    fn empty_placement_is_valid() {
        let placement = Placement::new();

        assert!(placement.is_empty());
        assert_eq!(placement.len(), 0);
        assert_eq!(placement.epoch(), PlacementEpoch::ZERO);
        assert!(placement.validate().is_ok());
    }

    #[test]
    fn assignment_uses_canonical_qubit_ids() {
        let mut placement = Placement::new();

        let change = placement
            .assign(logical(0), physical(7))
            .expect("assignment must succeed");

        assert_eq!(change.kind(), PlacementChangeKind::Assignment);
        assert_eq!(change.epoch(), PlacementEpoch::new(1));
        assert_eq!(
            placement.physical_for(logical(0)),
            Some(physical(7))
        );
        assert_eq!(
            placement.logical_for(physical(7)),
            Some(logical(0))
        );
    }

    #[test]
    fn duplicate_logical_assignment_is_rejected() {
        let mut placement = Placement::new();

        placement
            .assign(logical(0), physical(1))
            .expect("first assignment must succeed");

        let error = placement
            .assign(logical(0), physical(2))
            .expect_err("second assignment must fail");

        assert!(matches!(
            error,
            PlacementError::LogicalAlreadyMapped { .. }
        ));

        assert_eq!(
            placement.physical_for(logical(0)),
            Some(physical(1))
        );
    }

    #[test]
    fn physical_collision_is_rejected() {
        let mut placement = Placement::new();

        placement
            .assign(logical(0), physical(1))
            .expect("first assignment must succeed");

        let error = placement
            .assign(logical(1), physical(1))
            .expect_err("collision must fail");

        assert!(matches!(
            error,
            PlacementError::PhysicalAlreadyOccupied { .. }
        ));

        assert_eq!(
            placement.physical_for(logical(0)),
            Some(physical(1))
        );
        assert_eq!(
            placement.physical_for(logical(1)),
            None
        );
    }

    #[test]
    fn relocation_is_atomic_and_changes_epoch() {
        let mut placement = Placement::new();

        placement
            .assign(logical(0), physical(1))
            .expect("assignment must succeed");

        let change = placement
            .relocate(logical(0), physical(2))
            .expect("relocation must succeed");

        assert_eq!(change.kind(), PlacementChangeKind::Relocation);
        assert_eq!(change.previous(), Some(physical(1)));
        assert_eq!(change.current(), Some(physical(2)));
        assert_eq!(placement.epoch(), PlacementEpoch::new(2));

        assert_eq!(
            placement.physical_for(logical(0)),
            Some(physical(2))
        );
        assert_eq!(
            placement.logical_for(physical(1)),
            None
        );
    }

    #[test]
    fn relocation_collision_does_not_mutate_state() {
        let mut placement = Placement::new();

        placement
            .assign(logical(0), physical(1))
            .expect("assignment must succeed");

        placement
            .assign(logical(1), physical(2))
            .expect("assignment must succeed");

        let epoch = placement.epoch();

        let error = placement
            .relocate(logical(0), physical(2))
            .expect_err("collision must fail");

        assert!(matches!(
            error,
            PlacementError::PhysicalAlreadyOccupied { .. }
        ));

        assert_eq!(placement.epoch(), epoch);
        assert_eq!(
            placement.physical_for(logical(0)),
            Some(physical(1))
        );
        assert_eq!(
            placement.physical_for(logical(1)),
            Some(physical(2))
        );
    }

    #[test]
    fn exchange_is_atomic() {
        let mut placement = Placement::new();

        placement
            .assign(logical(0), physical(1))
            .expect("assignment must succeed");

        placement
            .assign(logical(1), physical(2))
            .expect("assignment must succeed");

        placement
            .exchange(logical(0), logical(1))
            .expect("exchange must succeed");

        assert_eq!(
            placement.physical_for(logical(0)),
            Some(physical(2))
        );
        assert_eq!(
            placement.physical_for(logical(1)),
            Some(physical(1))
        );
    }

    #[test]
    fn exchange_of_missing_qubit_does_not_mutate_state() {
        let mut placement = Placement::new();

        placement
            .assign(logical(0), physical(1))
            .expect("assignment must succeed");

        let epoch = placement.epoch();

        let error = placement
            .exchange(logical(0), logical(1))
            .expect_err("missing logical qubit must fail");

        assert!(matches!(
            error,
            PlacementError::LogicalNotPlaced { .. }
        ));

        assert_eq!(placement.epoch(), epoch);
        assert_eq!(
            placement.physical_for(logical(0)),
            Some(physical(1))
        );
    }

    #[test]
    fn removal_releases_placement_state() {
        let mut placement = Placement::new();

        placement
            .assign(logical(0), physical(1))
            .expect("assignment must succeed");

        let change = placement
            .remove(logical(0))
            .expect("removal must succeed");

        assert_eq!(change.kind(), PlacementChangeKind::Removal);
        assert_eq!(change.previous(), Some(physical(1)));
        assert_eq!(change.current(), None);

        assert_eq!(placement.physical_for(logical(0)), None);
        assert_eq!(placement.logical_for(physical(1)), None);
    }

    #[test]
    fn snapshots_are_immutable_views() {
        let mut placement = Placement::new();

        placement
            .assign(logical(0), physical(1))
            .expect("assignment must succeed");

        let snapshot = placement.snapshot();

        placement
            .relocate(logical(0), physical(2))
            .expect("relocation must succeed");

        assert_eq!(
            snapshot.physical_for(logical(0)),
            Some(physical(1))
        );
        assert_eq!(
            placement.physical_for(logical(0)),
            Some(physical(2))
        );
    }

    #[test]
    fn completeness_is_domain_based() {
        let mut placement = Placement::new();

        placement
            .assign(logical(0), physical(1))
            .expect("assignment must succeed");

        placement
            .assign(logical(1), physical(2))
            .expect("assignment must succeed");

        let domain = MappingDomain::new(logical(0), 2)
            .expect("domain must be valid");

        assert!(placement.is_complete_for(domain));
        assert_eq!(
            placement.completeness_for(domain),
            PlacementCompleteness::Complete
        );
    }

    #[test]
    fn sparse_identifiers_are_supported() {
        let mut placement = Placement::new();

        let large = usize::MAX - 1;

        placement
            .assign(logical(large), physical(large))
            .expect("sparse identifiers must work");

        assert_eq!(
            placement.physical_for(logical(large)),
            Some(physical(large))
        );
    }

    #[test]
    fn stale_epoch_is_detected() {
        let mut placement = Placement::new();

        let initial = placement.epoch();

        placement
            .assign(logical(0), physical(0))
            .expect("assignment must succeed");

        let error = placement
            .require_epoch(initial)
            .expect_err("old epoch must be rejected");

        assert!(matches!(
            error,
            PlacementError::StaleEpoch { .. }
        ));
    }

    #[test]
    fn diff_is_deterministic() {
        let mut before = Placement::new();

        before
            .assign(logical(0), physical(0))
            .expect("assignment must succeed");

        before
            .assign(logical(1), physical(1))
            .expect("assignment must succeed");

        let before = before.snapshot();

        let mut after = Placement::new();

        after
            .assign(logical(0), physical(2))
            .expect("assignment must succeed");

        after
            .assign(logical(2), physical(3))
            .expect("assignment must succeed");

        let after = after.snapshot();

        let differences = diff(&before, &after);

        assert_eq!(
            differences,
            vec![
                PlacementDifference::Moved {
                    logical: logical(0),
                    previous: physical(0),
                    current: physical(2),
                },
                PlacementDifference::Removed {
                    logical: logical(1),
                    previous: physical(1),
                },
                PlacementDifference::Added {
                    logical: logical(2),
                    physical: physical(3),
                },
            ]
        );
    }

    #[test]
    fn identical_diff_has_explicit_identity_result() {
        let placement = Placement::new();
        let snapshot = placement.snapshot();

        assert_eq!(
            diff(&snapshot, &snapshot),
            vec![PlacementDifference::Identical]
        );
    }

    #[test]
    fn placement_validation_is_independent_of_hardware() {
        let mut placement = Placement::new();

        placement
            .assign(
                logical(1_000_000),
                physical(9_000_000),
            )
            .expect("arbitrary finite identifiers are valid IR identities");

        assert!(placement.validate().is_ok());
    }
}