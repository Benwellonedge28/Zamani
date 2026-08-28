//! Zamani Quantum Routing — Logical/Physical Qubit Mapping
//!
//! Production-grade bidirectional logical-to-physical qubit mapping.
//!
//! # Responsibility
//!
//! `mapping.rs` owns the mutable relationship between:
//!
//! ```text
//! logical qubit  <──────────────>  physical qubit
//! ```
//!
//! It is deliberately independent of:
//!
//! - hardware topology;
//! - routing algorithms;
//! - layout algorithms;
//! - cost models;
//! - scheduling;
//! - gate decomposition;
//! - compiler IR implementation details;
//! - hardware providers;
//! - calibration;
//! - execution;
//! - QEC implementation.
//!
//! The mapping is a routing-state primitive. Higher-level routing components
//! decide *why* a mapping should change; this module guarantees that the
//! mapping change itself is correct, atomic, and internally consistent.
//!
//! # Production requirements
//!
//! This implementation provides:
//!
//! - strongly typed logical/physical identifiers;
//! - O(1) logical -> physical lookup on average;
//! - O(1) physical -> logical lookup on average;
//! - collision prevention;
//! - explicit unassignment;
//! - physical-location swaps;
//! - logical-qubit swaps;
//! - movement into an unoccupied physical location;
//! - multi-swap/permutation application;
//! - immutable snapshots;
//! - restoration from snapshots;
//! - transactional mutation;
//! - deterministic iteration;
//! - invariant validation;
//! - topology-independent validation hooks;
//! - capacity-independent mapping semantics;
//! - stable error vocabulary local to the mapping subsystem;
//! - no `unsafe`;
//! - no nightly features;
//! - Rust 1.97 / 1.97.1 compatibility.
//!
//! # Architectural rule
//!
//! Mapping does NOT own topology.
//!
//! A physical identifier can exist in a mapping without this module knowing
//! whether that physical qubit exists on a particular backend. Existence and
//! hardware availability belong to `topology.rs` / `hardware/`.
//!
//! This separation is intentional:
//!
//! ```text
//! mapping.rs
//!     │
//!     │ association correctness
//!     ▼
//! topology.rs
//!     │
//!     │ physical resource validity
//!     ▼
//! hardware/
//! ```
//!
//! # Integration contract
//!
//! `mapping.rs` consumes only routing identifiers from `types.rs`.
//!
//! Future routing modules can therefore depend on this file without creating
//! circular dependencies:
//!
//! ```text
//! types.rs
//!    │
//!    ▼
//! mapping.rs
//!    │
//!    ├──────────────► topology.rs
//!    ├──────────────► cost.rs
//!    ├──────────────► algorithms/*
//!    ├──────────────► layout.rs
//!    ├──────────────► router.rs
//!    ├──────────────► verification.rs
//!    └──────────────► transpiler.rs
//! ```
//!
//! The future `errors.rs` may wrap or convert `MappingError`, but `mapping.rs`
//! does not depend on `errors.rs`. This prevents a dependency cycle and means
//! this file remains independently compilable once `types.rs` is present.
//!
//! # No compiler-IR coupling
//!
//! The canonical Quantum IR already has its own `QubitId` and
//! `PhysicalQubitId`. Routing intentionally has routing-level identifiers in
//! `types.rs`. Conversion between the two namespaces belongs to the routing/
//! IR integration adapter, not this mapping primitive.
//!
//! # Important semantic distinction
//!
//! A mapping represents *where logical qubits currently reside*.
//!
//! A SWAP therefore changes the mapping even before a later lowering stage
//! decides whether the physical SWAP is implemented as:
//!
//! - a native SWAP;
//! - three CX gates;
//! - another decomposition;
//! - a backend-specific primitive.
//!
//! This module consequently provides `swap_physical()` as a mapping operation,
//! not as a gate-generation operation.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No external dependencies are required.
//! No `unsafe` code is used.

// =============================================================================
// Imports
// =============================================================================

use crate::quantum::routing::types::{LogicalQubitId, PhysicalQubitId};

use std::collections::HashMap;
use std::fmt;

// =============================================================================
// Mapping error
// =============================================================================

/// Errors produced by logical/physical mapping operations.
///
/// This error type is intentionally local to `mapping.rs`.
///
/// The future routing-wide `errors.rs` can wrap this type without requiring
/// this module to depend upward on the routing error aggregator.
///
/// All variants are deterministic and contain enough context for diagnostics,
/// tests, logging, and compiler integration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MappingError {
    /// The logical qubit is already assigned to a physical location.
    LogicalAlreadyMapped {
        logical: LogicalQubitId,
        physical: PhysicalQubitId,
    },

    /// The physical qubit is already occupied by another logical qubit.
    PhysicalAlreadyMapped {
        physical: PhysicalQubitId,
        logical: LogicalQubitId,
    },

    /// The requested logical qubit has no physical assignment.
    LogicalNotMapped {
        logical: LogicalQubitId,
    },

    /// The requested physical qubit has no logical occupant.
    PhysicalNotMapped {
        physical: PhysicalQubitId,
    },

    /// A mapping mutation would assign two logical qubits to one physical
    /// location.
    PhysicalCollision {
        physical: PhysicalQubitId,
        existing: LogicalQubitId,
        requested: LogicalQubitId,
    },

    /// A mapping invariant was violated.
    InvariantViolation {
        message: String,
    },

    /// A permutation contained the same physical endpoint more than once in a
    /// single atomic operation where that would make the requested operation
    /// ambiguous.
    InvalidPermutation {
        message: String,
    },

    /// A requested batch operation exceeded the caller's configured limit.
    ///
    /// The mapping itself does not impose an arbitrary global qubit limit.
    /// This variant exists so higher-level bounded APIs can use the same
    /// mapping error vocabulary without introducing a second type.
    OperationLimitExceeded {
        requested: usize,
        maximum: usize,
    },
}

impl fmt::Display for MappingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LogicalAlreadyMapped {
                logical,
                physical,
            } => write!(
                formatter,
                "logical qubit {logical} is already mapped to physical qubit {physical}"
            ),

            Self::PhysicalAlreadyMapped { physical, logical } => write!(
                formatter,
                "physical qubit {physical} is already occupied by logical qubit {logical}"
            ),

            Self::LogicalNotMapped { logical } => {
                write!(formatter, "logical qubit {logical} is not mapped")
            }

            Self::PhysicalNotMapped { physical } => {
                write!(formatter, "physical qubit {physical} is not mapped")
            }

            Self::PhysicalCollision {
                physical,
                existing,
                requested,
            } => write!(
                formatter,
                "physical qubit {physical} is occupied by logical qubit {existing}; \
                 cannot assign logical qubit {requested}"
            ),

            Self::InvariantViolation { message } => {
                write!(formatter, "mapping invariant violation: {message}")
            }

            Self::InvalidPermutation { message } => {
                write!(formatter, "invalid physical permutation: {message}")
            }

            Self::OperationLimitExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "mapping operation contains {requested} entries, exceeding maximum {maximum}"
            ),
        }
    }
}

impl std::error::Error for MappingError {}

// =============================================================================
// Mapping snapshot
// =============================================================================

/// Immutable snapshot of a [`QubitMapping`].
///
/// Snapshots are deliberately opaque: callers cannot mutate the snapshot's
/// internal maps and therefore cannot create an invalid snapshot.
///
/// Snapshots are used by:
///
/// - transactional routing;
/// - speculative SABRE candidates;
/// - lookahead;
/// - rollback;
/// - verification;
/// - debugging;
/// - deterministic test fixtures.
///
/// A snapshot contains both directions of the mapping so restoration is O(n)
/// rather than requiring reconstruction through repeated lookups.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QubitMappingSnapshot {
    logical_to_physical: HashMap<LogicalQubitId, PhysicalQubitId>,
    physical_to_logical: HashMap<PhysicalQubitId, LogicalQubitId>,
}

impl QubitMappingSnapshot {
    /// Returns the number of logical/physical assignments in the snapshot.
    #[must_use]
    pub fn len(&self) -> usize {
        self.logical_to_physical.len()
    }

    /// Returns whether the snapshot contains no assignments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.logical_to_physical.is_empty()
    }

    /// Looks up a logical qubit in the snapshot.
    #[must_use]
    pub fn physical_of(
        &self,
        logical: LogicalQubitId,
    ) -> Option<PhysicalQubitId> {
        self.logical_to_physical.get(&logical).copied()
    }

    /// Looks up a physical qubit in the snapshot.
    #[must_use]
    pub fn logical_at(
        &self,
        physical: PhysicalQubitId,
    ) -> Option<LogicalQubitId> {
        self.physical_to_logical.get(&physical).copied()
    }

    /// Returns deterministic logical-to-physical assignments.
    ///
    /// Sorting is performed at the API boundary rather than depending on
    /// `HashMap` iteration order.
    #[must_use]
    pub fn logical_to_physical(
        &self,
    ) -> Vec<(LogicalQubitId, PhysicalQubitId)> {
        let mut entries: Vec<_> = self
            .logical_to_physical
            .iter()
            .map(|(&logical, &physical)| (logical, physical))
            .collect();

        entries.sort_unstable_by_key(|(logical, _)| *logical);
        entries
    }

    /// Returns deterministic physical-to-logical assignments.
    #[must_use]
    pub fn physical_to_logical(
        &self,
    ) -> Vec<(PhysicalQubitId, LogicalQubitId)> {
        let mut entries: Vec<_> = self
            .physical_to_logical
            .iter()
            .map(|(&physical, &logical)| (physical, logical))
            .collect();

        entries.sort_unstable_by_key(|(physical, _)| *physical);
        entries
    }
}

// =============================================================================
// Qubit mapping
// =============================================================================

/// Bidirectional logical-to-physical qubit mapping.
///
/// # Core invariant
///
/// For every mapping:
///
/// ```text
/// logical_to_physical[L] = P
/// ```
///
/// there must be exactly one corresponding reverse entry:
///
/// ```text
/// physical_to_logical[P] = L
/// ```
///
/// Conversely, every reverse entry must have a matching forward entry.
///
/// Therefore:
///
/// ```text
/// len(logical_to_physical) == len(physical_to_logical)
/// ```
///
/// must always hold.
///
/// # Complexity
///
/// Average-case complexity:
///
/// | Operation | Complexity |
/// |---|---:|
/// | `physical_of` | O(1) |
/// | `logical_at` | O(1) |
/// | `contains_logical` | O(1) |
/// | `contains_physical` | O(1) |
/// | `assign` | O(1) |
/// | `unassign_logical` | O(1) |
/// | `unassign_physical` | O(1) |
/// | `swap_physical` | O(1) |
/// | `swap_logical` | O(1) |
/// | `snapshot` | O(n) |
/// | `restore` | O(n) |
/// | `validate` | O(n) |
///
/// This eliminates the O(n) reverse lookup used by the old transpiler
/// implementation when determining which logical qubit occupies a physical
/// location.
///
/// # Topology independence
///
/// `QubitMapping` deliberately does not contain a `Topology`.
///
/// This allows mapping to be used by:
///
/// - layout;
/// - routing;
/// - SABRE;
/// - lookahead;
/// - distributed routing;
/// - QEC-aware routing;
/// - simulation;
/// - benchmarking;
/// - compiler testing.
///
/// Hardware validity is checked by the topology layer.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct QubitMapping {
    logical_to_physical: HashMap<LogicalQubitId, PhysicalQubitId>,
    physical_to_logical: HashMap<PhysicalQubitId, LogicalQubitId>,
}

impl QubitMapping {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates an empty mapping.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty mapping with enough capacity for approximately
    /// `capacity` assignments.
    ///
    /// This is only a performance hint. It does not establish a hardware
    /// capacity and does not permit invalid mappings.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            logical_to_physical: HashMap::with_capacity(capacity),
            physical_to_logical: HashMap::with_capacity(capacity),
        }
    }

    /// Creates a mapping from an iterator of assignments.
    ///
    /// The construction is transactional: if any assignment is invalid, no
    /// partially constructed mapping is returned.
    pub fn from_assignments<I>(
        assignments: I,
    ) -> Result<Self, MappingError>
    where
        I: IntoIterator<Item = (LogicalQubitId, PhysicalQubitId)>,
    {
        let mut mapping = Self::new();

        for (logical, physical) in assignments {
            mapping.assign(logical, physical)?;
        }

        Ok(mapping)
    }

    /// Creates a mapping from an iterator with a caller-supplied operation
    /// bound.
    ///
    /// This is useful at compiler/security boundaries where input may be
    /// externally generated.
    pub fn from_assignments_with_limit<I>(
        assignments: I,
        maximum: usize,
    ) -> Result<Self, MappingError>
    where
        I: IntoIterator<Item = (LogicalQubitId, PhysicalQubitId)>,
    {
        let mut mapping = Self::new();

        for (count, (logical, physical)) in assignments.into_iter().enumerate()
        {
            let requested = count + 1;

            if requested > maximum {
                return Err(MappingError::OperationLimitExceeded {
                    requested,
                    maximum,
                });
            }

            mapping.assign(logical, physical)?;
        }

        Ok(mapping)
    }

    // =========================================================================
    // Basic state
    // =========================================================================

    /// Returns the number of active logical-to-physical assignments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.logical_to_physical.len()
    }

    /// Returns whether the mapping contains no assignments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.logical_to_physical.is_empty()
    }

    /// Returns whether a logical qubit currently has a physical location.
    #[must_use]
    pub fn contains_logical(&self, logical: LogicalQubitId) -> bool {
        self.logical_to_physical.contains_key(&logical)
    }

    /// Returns whether a physical qubit currently contains a logical qubit.
    #[must_use]
    pub fn contains_physical(&self, physical: PhysicalQubitId) -> bool {
        self.physical_to_logical.contains_key(&physical)
    }

    /// Returns the physical location of a logical qubit.
    #[must_use]
    pub fn physical_of(
        &self,
        logical: LogicalQubitId,
    ) -> Option<PhysicalQubitId> {
        self.logical_to_physical.get(&logical).copied()
    }

    /// Returns the logical qubit currently occupying a physical location.
    #[must_use]
    pub fn logical_at(
        &self,
        physical: PhysicalQubitId,
    ) -> Option<LogicalQubitId> {
        self.physical_to_logical.get(&physical).copied()
    }

    /// Returns the physical location of a logical qubit or an explicit error.
    pub fn require_physical(
        &self,
        logical: LogicalQubitId,
    ) -> Result<PhysicalQubitId, MappingError> {
        self.physical_of(logical)
            .ok_or(MappingError::LogicalNotMapped { logical })
    }

    /// Returns the logical qubit at a physical location or an explicit error.
    pub fn require_logical(
        &self,
        physical: PhysicalQubitId,
    ) -> Result<LogicalQubitId, MappingError> {
        self.logical_at(physical)
            .ok_or(MappingError::PhysicalNotMapped { physical })
    }

    // =========================================================================
    // Deterministic iteration
    // =========================================================================

    /// Returns deterministic logical-to-physical assignments.
    ///
    /// `HashMap` iteration order is intentionally not exposed as a semantic
    /// ordering.
    #[must_use]
    pub fn logical_to_physical(
        &self,
    ) -> Vec<(LogicalQubitId, PhysicalQubitId)> {
        let mut entries: Vec<_> = self
            .logical_to_physical
            .iter()
            .map(|(&logical, &physical)| (logical, physical))
            .collect();

        entries.sort_unstable_by_key(|(logical, _)| *logical);
        entries
    }

    /// Returns deterministic physical-to-logical assignments.
    #[must_use]
    pub fn physical_to_logical(
        &self,
    ) -> Vec<(PhysicalQubitId, LogicalQubitId)> {
        let mut entries: Vec<_> = self
            .physical_to_logical
            .iter()
            .map(|(&physical, &logical)| (physical, logical))
            .collect();

        entries.sort_unstable_by_key(|(physical, _)| *physical);
        entries
    }

    /// Returns all mapped logical qubits in deterministic order.
    #[must_use]
    pub fn logical_qubits(&self) -> Vec<LogicalQubitId> {
        let mut qubits: Vec<_> =
            self.logical_to_physical.keys().copied().collect();

        qubits.sort_unstable();
        qubits
    }

    /// Returns all occupied physical qubits in deterministic order.
    #[must_use]
    pub fn physical_qubits(&self) -> Vec<PhysicalQubitId> {
        let mut qubits: Vec<_> =
            self.physical_to_logical.keys().copied().collect();

        qubits.sort_unstable();
        qubits
    }

    // =========================================================================
    // Assignment
    // =========================================================================

    /// Assigns one logical qubit to one physical location.
    ///
    /// The operation succeeds only when:
    ///
    /// - the logical qubit is not already mapped;
    /// - the physical qubit is not already occupied.
    ///
    /// No topology validation occurs here.
    ///
    /// This is intentional. `PhysicalQubitId(123)` may be a perfectly valid
    /// mapping primitive even if a particular hardware topology does not have
    /// qubit 123. The topology layer owns that distinction.
    pub fn assign(
        &mut self,
        logical: LogicalQubitId,
        physical: PhysicalQubitId,
    ) -> Result<(), MappingError> {
        if let Some(&existing_physical) =
            self.logical_to_physical.get(&logical)
        {
            return Err(MappingError::LogicalAlreadyMapped {
                logical,
                physical: existing_physical,
            });
        }

        if let Some(&existing_logical) =
            self.physical_to_logical.get(&physical)
        {
            return Err(MappingError::PhysicalAlreadyMapped {
                physical,
                logical: existing_logical,
            });
        }

        self.logical_to_physical.insert(logical, physical);
        self.physical_to_logical.insert(physical, logical);

        Ok(())
    }

    /// Assigns a logical qubit to a physical location after checking the
    /// caller-provided physical-resource predicate.
    ///
    /// This is the topology integration hook.
    ///
    /// Example future usage from `topology.rs` / `router.rs`:
    ///
    /// ```text
    /// mapping.assign_checked(logical, physical, |p| topology.contains(p))
    /// ```
    ///
    /// The mapping module therefore does not need to import `Topology`.
    pub fn assign_checked<F>(
        &mut self,
        logical: LogicalQubitId,
        physical: PhysicalQubitId,
        physical_exists: F,
    ) -> Result<(), MappingError>
    where
        F: FnOnce(PhysicalQubitId) -> bool,
    {
        if !physical_exists(physical) {
            return Err(MappingError::InvariantViolation {
                message: format!(
                    "physical qubit {physical} is not present in the supplied \
                     physical-resource domain"
                ),
            });
        }

        self.assign(logical, physical)
    }

    /// Assigns a sequence of logical qubits to physical qubits atomically.
    ///
    /// If any assignment fails, the original mapping remains unchanged.
    pub fn assign_many<I>(
        &mut self,
        assignments: I,
    ) -> Result<(), MappingError>
    where
        I: IntoIterator<Item = (LogicalQubitId, PhysicalQubitId)>,
    {
        let snapshot = self.snapshot();

        for (logical, physical) in assignments {
            if let Err(error) = self.assign(logical, physical) {
                self.restore(snapshot);
                return Err(error);
            }
        }

        Ok(())
    }

    /// Assigns a sequence of logical qubits to physical qubits atomically and
    /// validates every physical identifier through a caller-supplied predicate.
    pub fn assign_many_checked<I, F>(
        &mut self,
        assignments: I,
        physical_exists: F,
    ) -> Result<(), MappingError>
    where
        I: IntoIterator<Item = (LogicalQubitId, PhysicalQubitId)>,
        F: Fn(PhysicalQubitId) -> bool,
    {
        let snapshot = self.snapshot();

        for (logical, physical) in assignments {
            if !physical_exists(physical) {
                self.restore(snapshot);

                return Err(MappingError::InvariantViolation {
                    message: format!(
                        "physical qubit {physical} is not present in the \
                         supplied physical-resource domain"
                    ),
                });
            }

            if let Err(error) = self.assign(logical, physical) {
                self.restore(snapshot);
                return Err(error);
            }
        }

        Ok(())
    }

    // =========================================================================
    // Unassignment
    // =========================================================================

    /// Removes the mapping for a logical qubit.
    ///
    /// Returns the physical location that was released.
    pub fn unassign_logical(
        &mut self,
        logical: LogicalQubitId,
    ) -> Result<PhysicalQubitId, MappingError> {
        let physical = self
            .logical_to_physical
            .remove(&logical)
            .ok_or(MappingError::LogicalNotMapped { logical })?;

        let removed = self.physical_to_logical.remove(&physical);

        if removed != Some(logical) {
            // Restore the state before returning the invariant failure.
            self.logical_to_physical.insert(logical, physical);

            if let Some(previous) = removed {
                self.physical_to_logical.insert(physical, previous);
            }

            return Err(MappingError::InvariantViolation {
                message: format!(
                    "logical {logical} mapped to {physical}, but reverse \
                     mapping contained {removed:?}"
                ),
            });
        }

        Ok(physical)
    }

    /// Removes the mapping occupying a physical qubit.
    ///
    /// Returns the logical qubit that was released.
    pub fn unassign_physical(
        &mut self,
        physical: PhysicalQubitId,
    ) -> Result<LogicalQubitId, MappingError> {
        let logical = self
            .physical_to_logical
            .remove(&physical)
            .ok_or(MappingError::PhysicalNotMapped { physical })?;

        let removed = self.logical_to_physical.remove(&logical);

        if removed != Some(physical) {
            // Restore the state before returning the invariant failure.
            self.physical_to_logical.insert(physical, logical);

            if let Some(previous) = removed {
                self.logical_to_physical.insert(logical, previous);
            }

            return Err(MappingError::InvariantViolation {
                message: format!(
                    "physical {physical} mapped to {logical}, but forward \
                     mapping contained {removed:?}"
                ),
            });
        }

        Ok(logical)
    }

    /// Removes every mapping.
    pub fn clear(&mut self) {
        self.logical_to_physical.clear();
        self.physical_to_logical.clear();
    }

    // =========================================================================
    // Physical movement
    // =========================================================================

    /// Moves a logical qubit from its current physical location to an
    /// unoccupied physical location.
    ///
    /// This operation is intentionally different from `swap_physical()`.
    ///
    /// It represents:
    ///
    /// ```text
    /// source: occupied
    /// target: empty
    ///
    /// source -> target
    /// ```
    ///
    /// It fails if the target is already occupied.
    pub fn move_logical(
        &mut self,
        logical: LogicalQubitId,
        target: PhysicalQubitId,
    ) -> Result<PhysicalQubitId, MappingError> {
        let source = self.require_physical(logical)?;

        if source == target {
            return Ok(source);
        }

        if let Some(existing) = self.physical_to_logical.get(&target) {
            return Err(MappingError::PhysicalCollision {
                physical: target,
                existing: *existing,
                requested: logical,
            });
        }

        let removed_forward =
            self.logical_to_physical.remove(&logical);

        if removed_forward != Some(source) {
            if let Some(previous) = removed_forward {
                self.logical_to_physical.insert(logical, previous);
            }

            return Err(MappingError::InvariantViolation {
                message: format!(
                    "logical {logical} expected at physical {source}, \
                     but forward mapping changed unexpectedly"
                ),
            });
        }

        let removed_reverse = self.physical_to_logical.remove(&source);

        if removed_reverse != Some(logical) {
            self.logical_to_physical.insert(logical, source);

            if let Some(previous) = removed_reverse {
                self.physical_to_logical.insert(source, previous);
            }

            return Err(MappingError::InvariantViolation {
                message: format!(
                    "physical {source} expected to contain logical {logical}, \
                     but reverse mapping changed unexpectedly"
                ),
            });
        }

        self.logical_to_physical.insert(logical, target);
        self.physical_to_logical.insert(target, logical);

        Ok(source)
    }

    /// Exchanges the logical occupants of two physical locations.
    ///
    /// This is the fundamental mapping operation used by routing SWAP
    /// insertion.
    ///
    /// Both physical locations may be occupied or one may be empty.
    ///
    /// Cases:
    ///
    /// ```text
    /// A = q0, B = q1  -> A = q1, B = q0
    ///
    /// A = q0, B = empty -> A = empty, B = q0
    ///
    /// A = empty, B = q1 -> A = q1, B = empty
    ///
    /// A = empty, B = empty -> unchanged
    /// ```
    ///
    /// No gate is generated here.
    pub fn swap_physical(
        &mut self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> Result<(), MappingError> {
        if a == b {
            return Ok(());
        }

        let logical_a = self.physical_to_logical.get(&a).copied();
        let logical_b = self.physical_to_logical.get(&b).copied();

        match (logical_a, logical_b) {
            (None, None) => Ok(()),

            (Some(logical), None) => {
                self.move_occupied_physical(a, b, logical)
            }

            (None, Some(logical)) => {
                self.move_occupied_physical(b, a, logical)
            }

            (Some(logical_a), Some(logical_b)) => {
                self.logical_to_physical.insert(logical_a, b);
                self.logical_to_physical.insert(logical_b, a);

                self.physical_to_logical.insert(a, logical_b);
                self.physical_to_logical.insert(b, logical_a);

                Ok(())
            }
        }
    }

    /// Internal helper for moving an occupant between physical locations when
    /// the destination is known to be empty.
    fn move_occupied_physical(
        &mut self,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
        logical: LogicalQubitId,
    ) -> Result<(), MappingError> {
        let reverse_source =
            self.physical_to_logical.get(&source).copied();

        if reverse_source != Some(logical) {
            return Err(MappingError::InvariantViolation {
                message: format!(
                    "physical {source} expected to contain logical {logical}, \
                     found {reverse_source:?}"
                ),
            });
        }

        let forward_source =
            self.logical_to_physical.get(&logical).copied();

        if forward_source != Some(source) {
            return Err(MappingError::InvariantViolation {
                message: format!(
                    "logical {logical} expected at physical {source}, \
                     found {forward_source:?}"
                ),
            });
        }

        if self.physical_to_logical.contains_key(&target) {
            return Err(MappingError::InvariantViolation {
                message: format!(
                    "target physical qubit {target} is unexpectedly occupied"
                ),
            });
        }

        self.physical_to_logical.remove(&source);
        self.logical_to_physical.insert(logical, target);
        self.physical_to_logical.insert(target, logical);

        Ok(())
    }

    /// Exchanges the physical locations of two logical qubits.
    pub fn swap_logical(
        &mut self,
        logical_a: LogicalQubitId,
        logical_b: LogicalQubitId,
    ) -> Result<(), MappingError> {
        if logical_a == logical_b {
            return Ok(());
        }

        let physical_a = self.require_physical(logical_a)?;
        let physical_b = self.require_physical(logical_b)?;

        self.swap_physical(physical_a, physical_b)
    }

    // =========================================================================
    // Atomic permutation / SWAP application
    // =========================================================================

    /// Applies a sequence of physical SWAP/permutation operations atomically.
    ///
    /// If any operation fails, the mapping is restored to its state before the
    /// call.
    ///
    /// This is the primitive intended for:
    ///
    /// - shortest-path routing;
    /// - lookahead;
    /// - SABRE;
    /// - speculative routing;
    /// - dynamic rerouting;
    /// - route replay.
    pub fn apply_swaps<I>(
        &mut self,
        swaps: I,
    ) -> Result<(), MappingError>
    where
        I: IntoIterator<Item = (PhysicalQubitId, PhysicalQubitId)>,
    {
        let snapshot = self.snapshot();

        for (a, b) in swaps {
            if let Err(error) = self.swap_physical(a, b) {
                self.restore(snapshot);
                return Err(error);
            }
        }

        Ok(())
    }

    /// Applies a sequence of physical SWAP operations with an explicit
    /// operation-count limit.
    ///
    /// This is useful when routing configuration provides `max_swaps`.
    pub fn apply_swaps_with_limit<I>(
        &mut self,
        swaps: I,
        maximum: usize,
    ) -> Result<usize, MappingError>
    where
        I: IntoIterator<Item = (PhysicalQubitId, PhysicalQubitId)>,
    {
        let snapshot = self.snapshot();
        let mut count = 0usize;

        for (a, b) in swaps {
            count += 1;

            if count > maximum {
                self.restore(snapshot);

                return Err(MappingError::OperationLimitExceeded {
                    requested: count,
                    maximum,
                });
            }

            if let Err(error) = self.swap_physical(a, b) {
                self.restore(snapshot);
                return Err(error);
            }
        }

        Ok(count)
    }

    /// Applies a complete physical permutation atomically.
    ///
    /// The permutation is represented as a sequence of transpositions:
    ///
    /// ```text
    /// (p0, p1)
    /// (p1, p2)
    /// ...
    /// ```
    ///
    /// Repeated endpoints are legal here because sequential SWAP semantics are
    /// meaningful. The method therefore differs from an unordered edge-set
    /// validator.
    pub fn apply_permutation<I>(
        &mut self,
        swaps: I,
    ) -> Result<(), MappingError>
    where
        I: IntoIterator<Item = (PhysicalQubitId, PhysicalQubitId)>,
    {
        self.apply_swaps(swaps)
    }

    // =========================================================================
    // Snapshots and transactions
    // =========================================================================

    /// Creates an immutable snapshot of the current mapping.
    #[must_use]
    pub fn snapshot(&self) -> QubitMappingSnapshot {
        QubitMappingSnapshot {
            logical_to_physical: self.logical_to_physical.clone(),
            physical_to_logical: self.physical_to_logical.clone(),
        }
    }

    /// Restores the mapping from a previously captured snapshot.
    ///
    /// The snapshot type is constructed only by this mapping implementation,
    /// so restoration cannot receive an externally fabricated inconsistent
    /// state.
    pub fn restore(&mut self, snapshot: QubitMappingSnapshot) {
        self.logical_to_physical = snapshot.logical_to_physical;
        self.physical_to_logical = snapshot.physical_to_logical;
    }

    /// Executes a speculative mutation and commits it only when the callback
    /// succeeds.
    ///
    /// This is the main transaction primitive for routing algorithms.
    ///
    /// The callback receives a mutable mapping. If it returns `Ok(T)`, all
    /// changes are retained. If it returns `Err(E)`, the mapping is restored
    /// automatically.
    pub fn transaction<T, E, F>(
        &mut self,
        operation: F,
    ) -> Result<T, E>
    where
        F: FnOnce(&mut Self) -> Result<T, E>,
    {
        let snapshot = self.snapshot();

        match operation(self) {
            Ok(value) => Ok(value),
            Err(error) => {
                self.restore(snapshot);
                Err(error)
            }
        }
    }

    /// Executes a mapping mutation and verifies its internal invariants before
    /// committing.
    ///
    /// This is useful for development, strict verification, and algorithm
    /// implementations where a mutation may be complex.
    pub fn transaction_checked<T, E, F>(
        &mut self,
        operation: F,
    ) -> Result<T, MappingTransactionError<E>>
    where
        F: FnOnce(&mut Self) -> Result<T, E>,
    {
        let snapshot = self.snapshot();

        match operation(self) {
            Ok(value) => {
                if let Err(error) = self.validate() {
                    self.restore(snapshot);

                    return Err(MappingTransactionError::Invariant(
                        error,
                    ));
                }

                Ok(value)
            }

            Err(error) => {
                self.restore(snapshot);
                Err(MappingTransactionError::Operation(error))
            }
        }
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Validates the complete bidirectional mapping invariant.
    ///
    /// This operation does not validate hardware topology. It only verifies
    /// the mathematical consistency of the mapping itself.
    pub fn validate(&self) -> Result<(), MappingError> {
        if self.logical_to_physical.len()
            != self.physical_to_logical.len()
        {
            return Err(MappingError::InvariantViolation {
                message: format!(
                    "forward mapping contains {} entries while reverse mapping \
                     contains {} entries",
                    self.logical_to_physical.len(),
                    self.physical_to_logical.len()
                ),
            });
        }

        for (&logical, &physical) in &self.logical_to_physical {
            match self.physical_to_logical.get(&physical) {
                Some(&reverse_logical) if reverse_logical == logical => {}
                Some(&reverse_logical) => {
                    return Err(MappingError::InvariantViolation {
                        message: format!(
                            "logical {logical} maps to {physical}, but reverse \
                             mapping points to {reverse_logical}"
                        ),
                    });
                }
                None => {
                    return Err(MappingError::InvariantViolation {
                        message: format!(
                            "logical {logical} maps to {physical}, but physical \
                             reverse entry is missing"
                        ),
                    });
                }
            }
        }

        for (&physical, &logical) in &self.physical_to_logical {
            match self.logical_to_physical.get(&logical) {
                Some(&reverse_physical) if reverse_physical == physical => {}
                Some(&reverse_physical) => {
                    return Err(MappingError::InvariantViolation {
                        message: format!(
                            "physical {physical} maps to {logical}, but forward \
                             mapping points to {reverse_physical}"
                        ),
                    });
                }
                None => {
                    return Err(MappingError::InvariantViolation {
                        message: format!(
                            "physical {physical} maps to {logical}, but logical \
                             forward entry is missing"
                        ),
                    });
                }
            }
        }

        Ok(())
    }

    /// Validates the mapping and additionally validates every physical
    /// identifier against a caller-supplied physical-resource predicate.
    ///
    /// This is the stable integration point for `topology.rs` and
    /// `hardware/`.
    pub fn validate_with<F>(
        &self,
        physical_exists: F,
    ) -> Result<(), MappingError>
    where
        F: Fn(PhysicalQubitId) -> bool,
    {
        self.validate()?;

        for physical in self.physical_to_logical.keys().copied() {
            if !physical_exists(physical) {
                return Err(MappingError::InvariantViolation {
                    message: format!(
                        "mapping references physical qubit {physical}, which \
                         is absent from the supplied physical-resource domain"
                    ),
                });
            }
        }

        Ok(())
    }

    /// Validates that a logical qubit collection is completely mapped.
    pub fn validate_logical_qubits<I>(
        &self,
        logical_qubits: I,
    ) -> Result<(), MappingError>
    where
        I: IntoIterator<Item = LogicalQubitId>,
    {
        for logical in logical_qubits {
            if !self.contains_logical(logical) {
                return Err(MappingError::LogicalNotMapped { logical });
            }
        }

        Ok(())
    }

    /// Validates that a physical-qubit collection is available in the mapping
    /// domain.
    ///
    /// This means every supplied physical identifier is present in the
    /// mapping's physical namespace, not that the hardware itself contains the
    /// qubit.
    pub fn validate_physical_qubits<I>(
        &self,
        physical_qubits: I,
    ) -> Result<(), MappingError>
    where
        I: IntoIterator<Item = PhysicalQubitId>,
    {
        for physical in physical_qubits {
            if !self.contains_physical(physical) {
                return Err(MappingError::PhysicalNotMapped { physical });
            }
        }

        Ok(())
    }

    // =========================================================================
    // Equality / state comparison
    // =========================================================================

    /// Returns whether two mappings contain exactly the same assignments.
    #[must_use]
    pub fn equivalent(&self, other: &Self) -> bool {
        self.logical_to_physical == other.logical_to_physical
    }

    /// Returns whether this mapping differs from another mapping.
    #[must_use]
    pub fn differs_from(&self, other: &Self) -> bool {
        !self.equivalent(other)
    }

    /// Returns the assignments that differ between two mappings.
    ///
    /// Each returned tuple contains:
    ///
    /// ```text
    /// (logical, self_physical, other_physical)
    /// ```
    ///
    /// `None` means that the logical qubit is unmapped in that mapping.
    #[must_use]
    pub fn differences(
        &self,
        other: &Self,
    ) -> Vec<(
        LogicalQubitId,
        Option<PhysicalQubitId>,
        Option<PhysicalQubitId>,
    )> {
        let mut logicals = self.logical_qubits();

        for logical in other.logical_to_physical.keys().copied() {
            if !logicals.contains(&logical) {
                logicals.push(logical);
            }
        }

        logicals.sort_unstable();
        logicals.dedup();

        logicals
            .into_iter()
            .filter_map(|logical| {
                let current = self.physical_of(logical);
                let target = other.physical_of(logical);

                if current == target {
                    None
                } else {
                    Some((logical, current, target))
                }
            })
            .collect()
    }
}

// =============================================================================
// Transaction error
// =============================================================================

/// Error returned by [`QubitMapping::transaction_checked`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MappingTransactionError<E> {
    /// The callback operation failed.
    Operation(E),

    /// The callback returned success but produced an invalid mapping.
    Invariant(MappingError),
}

impl<E: fmt::Display> fmt::Display for MappingTransactionError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Operation(error) => {
                write!(formatter, "mapping transaction failed: {error}")
            }

            Self::Invariant(error) => {
                write!(
                    formatter,
                    "mapping transaction violated invariant: {error}"
                )
            }
        }
    }
}

impl<E> std::error::Error for MappingTransactionError<E>
where
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Operation(error) => Some(error),
            Self::Invariant(error) => Some(error),
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn logical(index: usize) -> LogicalQubitId {
        LogicalQubitId::new(index)
    }

    fn physical(index: usize) -> PhysicalQubitId {
        PhysicalQubitId::new(index)
    }

    fn mapping_with_three() -> QubitMapping {
        QubitMapping::from_assignments([
            (logical(0), physical(0)),
            (logical(1), physical(1)),
            (logical(2), physical(2)),
        ])
        .expect("test mapping should be valid")
    }

    #[test]
    fn empty_mapping_is_valid() {
        let mapping = QubitMapping::new();

        assert!(mapping.is_empty());
        assert_eq!(mapping.len(), 0);
        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn assign_creates_both_directions() {
        let mut mapping = QubitMapping::new();

        mapping
            .assign(logical(0), physical(4))
            .expect("assignment should succeed");

        assert_eq!(
            mapping.physical_of(logical(0)),
            Some(physical(4))
        );

        assert_eq!(
            mapping.logical_at(physical(4)),
            Some(logical(0))
        );

        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn duplicate_logical_is_rejected() {
        let mut mapping = QubitMapping::new();

        mapping
            .assign(logical(0), physical(0))
            .expect("first assignment should succeed");

        let error = mapping
            .assign(logical(0), physical(1))
            .expect_err("duplicate logical must fail");

        assert_eq!(
            error,
            MappingError::LogicalAlreadyMapped {
                logical: logical(0),
                physical: physical(0),
            }
        );

        assert_eq!(
            mapping.physical_of(logical(0)),
            Some(physical(0))
        );

        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn duplicate_physical_is_rejected() {
        let mut mapping = QubitMapping::new();

        mapping
            .assign(logical(0), physical(0))
            .expect("first assignment should succeed");

        let error = mapping
            .assign(logical(1), physical(0))
            .expect_err("physical collision must fail");

        assert_eq!(
            error,
            MappingError::PhysicalAlreadyMapped {
                physical: physical(0),
                logical: logical(0),
            }
        );

        assert_eq!(
            mapping.physical_of(logical(0)),
            Some(physical(0))
        );

        assert_eq!(mapping.physical_of(logical(1)), None);

        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn unassign_logical_removes_both_directions() {
        let mut mapping = mapping_with_three();

        let released = mapping
            .unassign_logical(logical(1))
            .expect("logical should be mapped");

        assert_eq!(released, physical(1));
        assert_eq!(mapping.physical_of(logical(1)), None);
        assert_eq!(mapping.logical_at(physical(1)), None);
        assert_eq!(mapping.len(), 2);
        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn unassign_physical_removes_both_directions() {
        let mut mapping = mapping_with_three();

        let released = mapping
            .unassign_physical(physical(1))
            .expect("physical should be occupied");

        assert_eq!(released, logical(1));
        assert_eq!(mapping.physical_of(logical(1)), None);
        assert_eq!(mapping.logical_at(physical(1)), None);
        assert_eq!(mapping.len(), 2);
        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn move_logical_requires_empty_destination() {
        let mut mapping = mapping_with_three();

        mapping
            .move_logical(logical(0), physical(5))
            .expect("destination should be empty");

        assert_eq!(
            mapping.physical_of(logical(0)),
            Some(physical(5))
        );

        assert_eq!(mapping.logical_at(physical(0)), None);
        assert_eq!(
            mapping.logical_at(physical(5)),
            Some(logical(0))
        );

        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn move_logical_rejects_occupied_destination() {
        let mut mapping = mapping_with_three();

        let error = mapping
            .move_logical(logical(0), physical(1))
            .expect_err("occupied destination must fail");

        assert_eq!(
            error,
            MappingError::PhysicalCollision {
                physical: physical(1),
                existing: logical(1),
                requested: logical(0),
            }
        );

        assert_eq!(
            mapping.physical_of(logical(0)),
            Some(physical(0))
        );

        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn swap_physical_exchanges_two_occupied_locations() {
        let mut mapping = mapping_with_three();

        mapping
            .swap_physical(physical(0), physical(2))
            .expect("swap should succeed");

        assert_eq!(
            mapping.physical_of(logical(0)),
            Some(physical(2))
        );

        assert_eq!(
            mapping.physical_of(logical(2)),
            Some(physical(0))
        );

        assert_eq!(
            mapping.logical_at(physical(0)),
            Some(logical(2))
        );

        assert_eq!(
            mapping.logical_at(physical(2)),
            Some(logical(0))
        );

        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn swap_physical_can_move_into_empty_location() {
        let mut mapping = QubitMapping::new();

        mapping
            .assign(logical(0), physical(0))
            .expect("assignment should succeed");

        mapping
            .swap_physical(physical(0), physical(5))
            .expect("swap with empty location should succeed");

        assert_eq!(
            mapping.physical_of(logical(0)),
            Some(physical(5))
        );

        assert_eq!(mapping.logical_at(physical(0)), None);
        assert_eq!(
            mapping.logical_at(physical(5)),
            Some(logical(0))
        );

        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn swap_empty_locations_is_noop() {
        let mut mapping = QubitMapping::new();

        mapping
            .swap_physical(physical(0), physical(1))
            .expect("empty swap should succeed");

        assert!(mapping.is_empty());
        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn swapping_same_physical_location_is_noop() {
        let mut mapping = mapping_with_three();

        let before = mapping.clone();

        mapping
            .swap_physical(physical(1), physical(1))
            .expect("self swap should be harmless");

        assert_eq!(mapping, before);
    }

    #[test]
    fn swap_logical_exchanges_locations() {
        let mut mapping = mapping_with_three();

        mapping
            .swap_logical(logical(0), logical(2))
            .expect("logical swap should succeed");

        assert_eq!(
            mapping.physical_of(logical(0)),
            Some(physical(2))
        );

        assert_eq!(
            mapping.physical_of(logical(2)),
            Some(physical(0))
        );

        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn apply_swaps_is_atomic() {
        let mut mapping = mapping_with_three();
        let before = mapping.clone();

        let result = mapping.apply_swaps([
            (physical(0), physical(1)),
            (physical(2), physical(99)),
        ]);

        assert!(result.is_ok());

        // The mapping primitive does not consider p99 invalid; this is
        // intentional because topology ownership belongs elsewhere.
        assert!(mapping.validate().is_ok());

        assert_ne!(mapping, before);
    }

    #[test]
    fn apply_swaps_with_limit_rolls_back() {
        let mut mapping = mapping_with_three();
        let before = mapping.clone();

        let error = mapping
            .apply_swaps_with_limit(
                [
                    (physical(0), physical(1)),
                    (physical(1), physical(2)),
                ],
                1,
            )
            .expect_err("second operation exceeds limit");

        assert_eq!(
            error,
            MappingError::OperationLimitExceeded {
                requested: 2,
                maximum: 1,
            }
        );

        assert_eq!(mapping, before);
        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn snapshot_and_restore_are_exact() {
        let mut mapping = mapping_with_three();

        let snapshot = mapping.snapshot();

        mapping
            .swap_physical(physical(0), physical(2))
            .expect("swap should succeed");

        assert_ne!(mapping, {
            let mut restored = QubitMapping::new();
            restored.restore(snapshot.clone());
            restored
        });

        mapping.restore(snapshot);

        assert_eq!(
            mapping.physical_of(logical(0)),
            Some(physical(0))
        );

        assert_eq!(
            mapping.physical_of(logical(2)),
            Some(physical(2))
        );

        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn transaction_rolls_back_on_error() {
        let mut mapping = mapping_with_three();
        let before = mapping.clone();

        let result: Result<(), &'static str> =
            mapping.transaction(|mapping| {
                mapping
                    .swap_physical(physical(0), physical(2))
                    .expect("swap should succeed");

                Err("speculative route rejected")
            });

        assert_eq!(result, Err("speculative route rejected"));
        assert_eq!(mapping, before);
    }

    #[test]
    fn transaction_commits_on_success() {
        let mut mapping = mapping_with_three();

        let result: Result<(), &'static str> =
            mapping.transaction(|mapping| {
                mapping
                    .swap_physical(physical(0), physical(2))
                    .expect("swap should succeed");

                Ok(())
            });

        assert_eq!(result, Ok(()));

        assert_eq!(
            mapping.physical_of(logical(0)),
            Some(physical(2))
        );

        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn checked_transaction_commits_valid_state() {
        let mut mapping = mapping_with_three();

        let result: Result<(), MappingTransactionError<&'static str>> =
            mapping.transaction_checked(|mapping| {
                mapping
                    .swap_physical(physical(0), physical(1))
                    .map_err(|_| "swap failed")?;

                Ok(())
            });

        assert!(result.is_ok());
        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn deterministic_logical_iteration() {
        let mapping = QubitMapping::from_assignments([
            (logical(9), physical(2)),
            (logical(1), physical(7)),
            (logical(4), physical(3)),
        ])
        .expect("mapping should be valid");

        assert_eq!(
            mapping.logical_to_physical(),
            vec![
                (logical(1), physical(7)),
                (logical(4), physical(3)),
                (logical(9), physical(2)),
            ]
        );
    }

    #[test]
    fn deterministic_physical_iteration() {
        let mapping = QubitMapping::from_assignments([
            (logical(9), physical(2)),
            (logical(1), physical(7)),
            (logical(4), physical(3)),
        ])
        .expect("mapping should be valid");

        assert_eq!(
            mapping.physical_to_logical(),
            vec![
                (physical(2), logical(9)),
                (physical(3), logical(4)),
                (physical(7), logical(1)),
            ]
        );
    }

    #[test]
    fn differences_are_deterministic() {
        let first = QubitMapping::from_assignments([
            (logical(0), physical(0)),
            (logical(1), physical(1)),
        ])
        .expect("mapping should be valid");

        let second = QubitMapping::from_assignments([
            (logical(0), physical(2)),
            (logical(1), physical(1)),
            (logical(2), physical(3)),
        ])
        .expect("mapping should be valid");

        assert_eq!(
            first.differences(&second),
            vec![
                (logical(0), Some(physical(0)), Some(physical(2))),
                (logical(2), None, Some(physical(3))),
            ]
        );
    }

    #[test]
    fn assign_many_is_atomic() {
        let mut mapping = mapping_with_three();
        let before = mapping.clone();

        let result = mapping.assign_many([
            (logical(3), physical(3)),
            (logical(4), physical(1)),
        ]);

        assert!(result.is_err());
        assert_eq!(mapping, before);
        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn checked_assignment_uses_external_resource_predicate() {
        let mut mapping = QubitMapping::new();

        mapping
            .assign_checked(
                logical(0),
                physical(4),
                |physical| physical.index() < 8,
            )
            .expect("p4 belongs to the supplied domain");

        let error = mapping
            .assign_checked(
                logical(1),
                physical(99),
                |physical| physical.index() < 8,
            )
            .expect_err("p99 is outside the supplied domain");

        assert!(matches!(
            error,
            MappingError::InvariantViolation { .. }
        ));

        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn validate_with_external_resource_predicate() {
        let mapping = mapping_with_three();

        assert!(mapping
            .validate_with(|physical| physical.index() < 8)
            .is_ok());

        assert!(mapping
            .validate_with(|physical| physical.index() < 2)
            .is_err());
    }

    #[test]
    fn require_methods_return_explicit_errors() {
        let mapping = QubitMapping::new();

        assert_eq!(
            mapping.require_physical(logical(0)),
            Err(MappingError::LogicalNotMapped {
                logical: logical(0)
            })
        );

        assert_eq!(
            mapping.require_logical(physical(0)),
            Err(MappingError::PhysicalNotMapped {
                physical: physical(0)
            })
        );
    }

    #[test]
    fn clear_removes_every_assignment() {
        let mut mapping = mapping_with_three();

        mapping.clear();

        assert!(mapping.is_empty());
        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn equivalent_mappings_are_equal_semantically() {
        let first = QubitMapping::from_assignments([
            (logical(0), physical(4)),
            (logical(1), physical(2)),
        ])
        .expect("mapping should be valid");

        let second = QubitMapping::from_assignments([
            (logical(1), physical(2)),
            (logical(0), physical(4)),
        ])
        .expect("mapping should be valid");

        assert!(first.equivalent(&second));
        assert!(!first.differs_from(&second));
    }

    #[test]
    fn physical_namespace_is_independent_of_topology() {
        let mut mapping = QubitMapping::new();

        // Mapping itself must not reject this identifier because topology
        // ownership belongs to topology.rs.
        mapping
            .assign(logical(0), physical(10_000))
            .expect("mapping primitive must remain topology-independent");

        assert!(mapping.validate().is_ok());
    }
}