//! Zamani Quantum IR — Logical-to-Physical Qubit Mapping
//!
//! Canonical, deterministic, hardware-independent representation of a
//! logical-to-physical quantum-resource mapping.
//!
//! # Architectural role
//!
//! This module owns the semantic mapping:
//!
//! ```text
//! logical qubit -> physical qubit
//! ```
//!
//! It records an already-selected placement. It does not decide that
//! placement.
//!
//! # Responsibilities
//!
//! This module:
//!
//! - represents logical-to-physical assignments;
//! - supports partial mappings;
//! - supports complete mappings;
//! - supports sparse identifiers;
//! - supports very large finite mappings;
//! - preserves deterministic ordering;
//! - maintains bidirectional lookup indexes;
//! - prevents two logical qubits from occupying one physical qubit;
//! - supports atomic insertion/replacement;
//! - supports removal;
//! - supports swapping two assignments;
//! - supports lookup in either direction;
//! - validates its internal invariants;
//! - checks completeness against an explicitly supplied logical domain;
//! - provides deterministic entry iteration;
//! - remains independent of hardware topology;
//! - remains independent of routing algorithms;
//! - remains independent of scheduling;
//! - remains independent of calibration;
//! - remains independent of backend execution;
//! - remains independent of serialization format;
//! - remains independent of cryptographic hashing.
//!
//! # Non-responsibilities
//!
//! This module does NOT:
//!
//! - discover physical devices;
//! - allocate physical hardware;
//! - inspect hardware topology;
//! - decide whether a physical qubit exists;
//! - decide whether a physical qubit is available;
//! - decide whether a physical qubit is calibrated;
//! - decide whether an operation is supported;
//! - route gates;
//! - insert SWAP operations;
//! - optimize circuits;
//! - schedule operations;
//! - generate pulses;
//! - execute quantum programs;
//! - communicate with a QPU;
//! - perform simulation;
//! - perform QEC decoding;
//! - impose a fixed maximum number of qubits.
//!
//! Those responsibilities belong to the corresponding downstream layers.
//!
//! # Universal-program principle
//!
//! Zamani programs are intended to be written once and lowered to quantum
//! machines of different sizes and architectures.
//!
//! Therefore this module contains no architectural constants such as:
//!
//! ```text
//! 32
//! 64
//! 127
//! 4096
//! 1_000_000
//! ```
//!
//! A mapping is limited only by:
//!
//! 1. the identifier representation provided by `quantum::ir::qubit`;
//! 2. memory available to the current process;
//! 3. explicit resource/security policies enforced by higher layers.
//!
//! "Infinity" is therefore not represented as a machine-size constant.
//! The same finite mapping model works for one qubit or for the largest
//! mapping that the selected execution environment can actually represent.
//!
//! # Identity boundary
//!
//! The canonical identity types are imported directly from:
//!
//! ```text
//! quantum::ir::qubit::{QubitId, PhysicalQubitId}
//! ```
//!
//! This is intentional. This module must not define a second logical-qubit
//! or physical-qubit identity type.
//!
//! `QubitId` identifies a logical namespace element.
//!
//! `PhysicalQubitId` identifies a physical-resource namespace element.
//!
//! Constructing a `PhysicalQubitId` does NOT prove that the corresponding
//! physical resource exists. Hardware validation happens outside this module.
//!
//! # Mapping invariant
//!
//! A `QubitMapping` is injective:
//!
//! ```text
//! q0 -> p7
//! q1 -> p12
//! ```
//!
//! is valid.
//!
//! ```text
//! q0 -> p7
//! q1 -> p7
//! ```
//!
//! is invalid.
//!
//! A physical resource can therefore have at most one logical owner in a
//! single mapping state.
//!
//! # Partial mappings
//!
//! A mapping may be incomplete:
//!
//! ```text
//! q0 -> p7
//! q1 -> p12
//! q2 -> unmapped
//! ```
//!
//! This is required during routing and intermediate compilation stages.
//!
//! Completeness is not stored as a mutable boolean because that would become
//! stale whenever entries are inserted or removed. Instead, completeness is
//! evaluated against an explicitly supplied logical domain.
//!
//! # Determinism
//!
//! `BTreeMap` is deliberately used for both directions:
//!
//! ```text
//! logical_to_physical
//! physical_to_logical
//! ```
//!
//! This provides deterministic iteration without depending on hash-map
//! iteration order.
//!
//! # Atomicity
//!
//! Mutating operations perform all conflict checks before changing either
//! internal index. A failed operation therefore leaves the mapping unchanged.
//!
//! # Scalability
//!
//! The representation is sparse. A mapping containing only:
//!
//! ```text
//! q0   -> p7
//! q1000000000 -> p42
//! ```
//!
//! does not allocate entries for the identifiers between them.
//!
//! Domain completeness is also represented as a start identifier plus a
//! count rather than by materializing every identifier.
//!
//! # Serialization boundary
//!
//! This file does not define a serialization format. The canonical
//! serialization layer should consume `iter()` and encode entries in the
//! deterministic order provided here.
//!
//! # Hashing boundary
//!
//! This file does not define a cryptographic fingerprint. The canonical
//! hashing layer should hash the semantic mapping entries in `iter()` order.
//!
//! This avoids coupling the IR mapping representation to a particular hash
//! algorithm.
//!
//! # Error boundary
//!
//! `MappingError` is intentionally local to this module. The global IR error
//! layer may wrap it later. This prevents `mapping.rs` from depending on
//! higher-level error aggregation and therefore keeps the dependency graph
//! acyclic.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! Requirements:
//!
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! Downstream consumers:
//!
//! ```text
//! quantum::ir::resource
//! quantum::ir::validation
//! quantum::ir::analysis
//! quantum::ir::program
//! quantum::routing
//! quantum::hardware
//! quantum::scheduling
//! quantum::optimization
//! quantum::ir::serialization
//! quantum::ir::hash
//! backend adapters
//! ```
//!
//! None of those modules are required to be imported here.
//!
//! Consequently, changes in routing, hardware, scheduling, optimization,
//! serialization, or hashing do not require this file to be edited merely
//! because their implementations changed.
//!
//! # Example
//!
//! ```ignore
//! use crate::quantum::ir::resources::mapping::QubitMapping;
//! use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
//!
//! let mut mapping = QubitMapping::new();
//!
//! mapping.insert(
//!     QubitId::new(0),
//!     PhysicalQubitId::new(17),
//! )?;
//!
//! mapping.insert(
//!     QubitId::new(1),
//!     PhysicalQubitId::new(42),
//! )?;
//!
//! assert_eq!(
//!     mapping.physical_for(QubitId::new(0)),
//!     Some(PhysicalQubitId::new(17)),
//! );
//!
//! # Ok::<(), crate::quantum::ir::resources::mapping::MappingError>(())
//! ```

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt;

use super::super::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Mapping entry
// =============================================================================

/// One immutable logical-to-physical mapping entry.
///
/// `MappingEntry` is a semantic value object. It contains no information
/// about topology, calibration, availability, or execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MappingEntry {
    logical: QubitId,
    physical: PhysicalQubitId,
}

impl MappingEntry {
    /// Creates a mapping entry.
    #[must_use]
    pub const fn new(logical: QubitId, physical: PhysicalQubitId) -> Self {
        Self { logical, physical }
    }

    /// Returns the logical qubit.
    #[must_use]
    pub const fn logical(self) -> QubitId {
        self.logical
    }

    /// Returns the physical qubit.
    #[must_use]
    pub const fn physical(self) -> PhysicalQubitId {
        self.physical
    }
}

impl fmt::Display for MappingEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} -> {}",
            self.logical,
            self.physical
        )
    }
}

// =============================================================================
// Mapping errors
// =============================================================================

/// Errors produced by [`QubitMapping`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MappingError {
    /// The logical qubit is already mapped to another physical qubit.
    LogicalAlreadyMapped {
        logical: QubitId,
        existing: PhysicalQubitId,
        requested: PhysicalQubitId,
    },

    /// The physical qubit is already owned by another logical qubit.
    PhysicalAlreadyMapped {
        physical: PhysicalQubitId,
        existing: QubitId,
        requested: QubitId,
    },

    /// The logical qubit has no mapping.
    LogicalNotMapped {
        logical: QubitId,
    },

    /// The physical qubit has no logical owner.
    PhysicalNotMapped {
        physical: PhysicalQubitId,
    },

    /// A requested mapping is incomplete for the supplied domain.
    IncompleteMapping {
        expected: usize,
        mapped: usize,
    },

    /// A supplied domain is invalid.
    InvalidDomain {
        start: QubitId,
        length: usize,
    },

    /// An internal mapping invariant has been violated.
    ///
    /// This should never be produced by safe public mutation methods.
    /// It exists so validation can report corruption or an incorrectly
    /// constructed value if future code is extended.
    InvariantViolation {
        logical: QubitId,
        physical: PhysicalQubitId,
    },
}

impl fmt::Display for MappingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LogicalAlreadyMapped {
                logical,
                existing,
                requested,
            } => write!(
                formatter,
                "logical qubit {logical} is already mapped to {existing}; \
                 cannot assign it to {requested}"
            ),

            Self::PhysicalAlreadyMapped {
                physical,
                existing,
                requested,
            } => write!(
                formatter,
                "physical qubit {physical} is already mapped from {existing}; \
                 cannot assign it to {requested}"
            ),

            Self::LogicalNotMapped { logical } => {
                write!(formatter, "logical qubit {logical} is not mapped")
            }

            Self::PhysicalNotMapped { physical } => {
                write!(formatter, "physical qubit {physical} is not mapped")
            }

            Self::IncompleteMapping { expected, mapped } => write!(
                formatter,
                "mapping is incomplete: expected {expected} logical qubits, \
                 but {mapped} are mapped"
            ),

            Self::InvalidDomain { start, length } => write!(
                formatter,
                "logical mapping domain starting at {start} with length \
                 {length} cannot be represented without overflowing the \
                 logical identifier namespace"
            ),

            Self::InvariantViolation { logical, physical } => write!(
                formatter,
                "mapping invariant violation for {logical} -> {physical}"
            ),
        }
    }
}

impl std::error::Error for MappingError {}

// =============================================================================
// Mapping domain
// =============================================================================

/// A logical-qubit domain used to evaluate mapping completeness.
///
/// The domain is represented as:
///
/// ```text
/// start + length
/// ```
///
/// rather than `[start, end)`.
///
/// This is important because `QubitId` uses `usize`, and `usize::MAX + 1`
/// cannot be represented. A start-plus-length representation can therefore
/// represent a domain containing the largest possible identifier.
///
/// The domain does not allocate or materialize its qubits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MappingDomain {
    start: QubitId,
    length: usize,
}

impl MappingDomain {
    /// Creates a domain beginning at `start` and containing `length` logical
    /// identifiers.
    ///
    /// The final identifier must fit in the canonical `QubitId` namespace.
    pub const fn new(
        start: QubitId,
        length: usize,
    ) -> Result<Self, MappingError> {
        if length == 0 {
            return Ok(Self { start, length });
        }

        match start.index().checked_add(length - 1) {
            Some(_) => Ok(Self { start, length }),
            None => Err(MappingError::InvalidDomain { start, length }),
        }
    }

    /// Creates an empty domain.
    #[must_use]
    pub const fn empty(start: QubitId) -> Self {
        Self {
            start,
            length: 0,
        }
    }

    /// Creates a domain containing exactly one logical qubit.
    #[must_use]
    pub const fn single(logical: QubitId) -> Self {
        Self {
            start: logical,
            length: 1,
        }
    }

    /// Returns the first logical identifier.
    #[must_use]
    pub const fn start(self) -> QubitId {
        self.start
    }

    /// Returns the number of logical identifiers in the domain.
    #[must_use]
    pub const fn len(self) -> usize {
        self.length
    }

    /// Returns whether this domain is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.length == 0
    }

    /// Returns whether a logical qubit belongs to this domain.
    #[must_use]
    pub const fn contains(self, logical: QubitId) -> bool {
        if logical.index() < self.start.index() {
            return false;
        }

        match logical.index().checked_sub(self.start.index()) {
            Some(offset) => offset < self.length,
            None => false,
        }
    }

    /// Returns the logical identifier at an offset.
    #[must_use]
    pub const fn get(self, offset: usize) -> Option<QubitId> {
        if offset >= self.length {
            return None;
        }

        match self.start.index().checked_add(offset) {
            Some(index) => Some(QubitId::new(index)),
            None => None,
        }
    }

    /// Returns a lazy iterator over the domain.
    pub fn iter(self) -> MappingDomainIter {
        MappingDomainIter {
            next_index: self.start.index(),
            remaining: self.length,
        }
    }
}

impl fmt::Display for MappingDomain {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} + {} qubits",
            self.start,
            self.length
        )
    }
}

/// Lazy iterator over a [`MappingDomain`].
#[derive(Debug, Clone)]
pub struct MappingDomainIter {
    next_index: usize,
    remaining: usize,
}

impl Iterator for MappingDomainIter {
    type Item = QubitId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let result = QubitId::new(self.next_index);

        self.remaining -= 1;

        if self.remaining != 0 {
            self.next_index = self
                .next_index
                .checked_add(1)
                .expect("MappingDomain invariant violated");
        }

        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for MappingDomainIter {}

impl std::iter::FusedIterator for MappingDomainIter {}

// =============================================================================
// Mapping completeness
// =============================================================================

/// Result of checking a mapping against a logical domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MappingCompleteness {
    /// Every logical qubit in the supplied domain is mapped.
    Complete,

    /// At least one logical qubit in the supplied domain is not mapped.
    Partial {
        /// Number of logical qubits expected by the domain.
        expected: usize,

        /// Number of logical qubits currently mapped.
        mapped: usize,
    },
}

impl MappingCompleteness {
    /// Returns `true` when the mapping is complete.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Returns `true` when the mapping is partial.
    #[must_use]
    pub const fn is_partial(self) -> bool {
        matches!(self, Self::Partial { .. })
    }
}

// =============================================================================
// Qubit mapping
// =============================================================================

/// Canonical logical-to-physical qubit mapping.
///
/// The structure maintains two synchronized indexes:
///
/// ```text
/// logical_to_physical:
///
///     QubitId -> PhysicalQubitId
///
/// physical_to_logical:
///
///     PhysicalQubitId -> QubitId
/// ```
///
/// Both indexes are required because routing, scheduling, hardware mapping,
/// validation, and analysis frequently need to query either direction.
///
/// All public mutation methods preserve the invariant that both indexes
/// contain exactly the same assignments.
///
/// # Complexity
///
/// For `n` mappings:
///
/// - insertion: `O(log n)`
/// - replacement: `O(log n)`
/// - removal: `O(log n)`
/// - logical lookup: `O(log n)`
/// - physical lookup: `O(log n)`
/// - iteration: `O(n)`
/// - invariant validation: `O(n log n)`
///
/// No fixed qubit count is embedded in the structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QubitMapping {
    logical_to_physical: BTreeMap<QubitId, PhysicalQubitId>,
    physical_to_logical: BTreeMap<PhysicalQubitId, QubitId>,
}

impl Default for QubitMapping {
    fn default() -> Self {
        Self::new()
    }
}

impl QubitMapping {
    /// Creates an empty mapping.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            logical_to_physical: BTreeMap::new(),
            physical_to_logical: BTreeMap::new(),
        }
    }

    /// Returns the number of logical-to-physical assignments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.logical_to_physical.len()
    }

    /// Returns `true` when no assignments exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.logical_to_physical.is_empty()
    }

    /// Returns the physical qubit assigned to a logical qubit.
    #[must_use]
    pub fn physical_for(
        &self,
        logical: QubitId,
    ) -> Option<PhysicalQubitId> {
        self.logical_to_physical.get(&logical).copied()
    }

    /// Returns the logical qubit assigned to a physical qubit.
    #[must_use]
    pub fn logical_for(
        &self,
        physical: PhysicalQubitId,
    ) -> Option<QubitId> {
        self.physical_to_logical.get(&physical).copied()
    }

    /// Returns whether a logical qubit is mapped.
    #[must_use]
    pub fn contains_logical(&self, logical: QubitId) -> bool {
        self.logical_to_physical.contains_key(&logical)
    }

    /// Returns whether a physical qubit is occupied by this mapping.
    #[must_use]
    pub fn contains_physical(
        &self,
        physical: PhysicalQubitId,
    ) -> bool {
        self.physical_to_logical.contains_key(&physical)
    }

    /// Returns an iterator over mappings in deterministic logical-ID order.
    pub fn iter(&self) -> impl Iterator<Item = MappingEntry> + '_ {
        self.logical_to_physical
            .iter()
            .map(|(logical, physical)| MappingEntry::new(*logical, *physical))
    }

    /// Returns an iterator over logical identifiers in deterministic order.
    pub fn logicals(&self) -> impl Iterator<Item = QubitId> + '_ {
        self.logical_to_physical.keys().copied()
    }

    /// Returns an iterator over physical identifiers in deterministic order.
    pub fn physicals(
        &self,
    ) -> impl Iterator<Item = PhysicalQubitId> + '_ {
        self.physical_to_logical.keys().copied()
    }

    /// Inserts a new mapping.
    ///
    /// The operation fails if either identity is already occupied by a
    /// different assignment.
    ///
    /// If the exact same assignment already exists, the operation succeeds
    /// without changing the mapping.
    ///
    /// The mapping is unchanged when an error is returned.
    pub fn insert(
        &mut self,
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> Result<(), MappingError> {
        match self.logical_to_physical.get(&logical).copied() {
            Some(existing) if existing != physical => {
                return Err(MappingError::LogicalAlreadyMapped {
                    logical,
                    existing,
                    requested: physical,
                });
            }
            Some(_) => {
                return Ok(());
            }
            None => {}
        }

        match self.physical_to_logical.get(&physical).copied() {
            Some(existing) if existing != logical => {
                return Err(MappingError::PhysicalAlreadyMapped {
                    physical,
                    existing,
                    requested: logical,
                });
            }
            Some(_) => {
                return Ok(());
            }
            None => {}
        }

        self.logical_to_physical.insert(logical, physical);
        self.physical_to_logical.insert(physical, logical);

        Ok(())
    }

    /// Replaces the physical assignment of a logical qubit.
    ///
    /// This operation is atomic:
    ///
    /// 1. all conflicts are checked;
    /// 2. the old assignment is identified;
    /// 3. the old reverse entry is removed;
    /// 4. the new assignment is inserted in both indexes.
    ///
    /// If the requested physical qubit belongs to another logical qubit,
    /// the mapping remains unchanged and an error is returned.
    pub fn remap(
        &mut self,
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> Result<Option<PhysicalQubitId>, MappingError> {
        let previous = self.logical_to_physical.get(&logical).copied();

        if previous == Some(physical) {
            return Ok(previous);
        }

        if let Some(existing) =
            self.physical_to_logical.get(&physical).copied()
        {
            if existing != logical {
                return Err(MappingError::PhysicalAlreadyMapped {
                    physical,
                    existing,
                    requested: logical,
                });
            }
        }

        if let Some(old_physical) = previous {
            self.logical_to_physical.remove(&logical);
            self.physical_to_logical.remove(&old_physical);
        }

        self.logical_to_physical.insert(logical, physical);
        self.physical_to_logical.insert(physical, logical);

        Ok(previous)
    }

    /// Inserts or replaces a mapping.
    ///
    /// This is the operation normally useful to routing passes that are
    /// constructing a new placement incrementally.
    pub fn insert_or_replace(
        &mut self,
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> Result<Option<PhysicalQubitId>, MappingError> {
        self.remap(logical, physical)
    }

    /// Removes the mapping for a logical qubit.
    ///
    /// Returns the previously assigned physical qubit when present.
    pub fn remove_logical(
        &mut self,
        logical: QubitId,
    ) -> Option<PhysicalQubitId> {
        let physical = self.logical_to_physical.remove(&logical)?;

        self.physical_to_logical.remove(&physical);

        Some(physical)
    }

    /// Removes the mapping for a physical qubit.
    ///
    /// Returns the previously assigned logical qubit when present.
    pub fn remove_physical(
        &mut self,
        physical: PhysicalQubitId,
    ) -> Option<QubitId> {
        let logical = self.physical_to_logical.remove(&physical)?;

        self.logical_to_physical.remove(&logical);

        Some(logical)
    }

    /// Removes all assignments.
    pub fn clear(&mut self) {
        self.logical_to_physical.clear();
        self.physical_to_logical.clear();
    }

    /// Swaps the physical assignments of two logical qubits.
    ///
    /// This is a mapping-state operation only. It does NOT insert a quantum
    /// SWAP gate and does NOT claim that the hardware can realize the swap.
    ///
    /// This method is therefore useful to routing algorithms without coupling
    /// this IR layer to routing implementation details.
    pub fn swap_logical_assignments(
        &mut self,
        first: QubitId,
        second: QubitId,
    ) -> Result<(), MappingError> {
        if first == second {
            return Ok(());
        }

        let first_physical = self.physical_for(first);
        let second_physical = self.physical_for(second);

        match (first_physical, second_physical) {
            (None, _) => {
                Err(MappingError::LogicalNotMapped { logical: first })
            }

            (_, None) => {
                Err(MappingError::LogicalNotMapped { logical: second })
            }

            (Some(first_physical), Some(second_physical)) => {
                self.logical_to_physical
                    .insert(first, second_physical);
                self.logical_to_physical
                    .insert(second, first_physical);

                self.physical_to_logical
                    .insert(first_physical, second);
                self.physical_to_logical
                    .insert(second_physical, first);

                Ok(())
            }
        }
    }

    /// Returns the mapping for a logical qubit or an explicit error.
    pub fn require_physical(
        &self,
        logical: QubitId,
    ) -> Result<PhysicalQubitId, MappingError> {
        self.physical_for(logical)
            .ok_or(MappingError::LogicalNotMapped { logical })
    }

    /// Returns the logical owner of a physical qubit or an explicit error.
    pub fn require_logical(
        &self,
        physical: PhysicalQubitId,
    ) -> Result<QubitId, MappingError> {
        self.logical_for(physical)
            .ok_or(MappingError::PhysicalNotMapped { physical })
    }

    /// Evaluates completeness against an explicitly supplied logical domain.
    ///
    /// A mapping may contain additional logical qubits outside the supplied
    /// domain. Those additional entries do not make the supplied domain
    /// incomplete.
    #[must_use]
    pub fn completeness(
        &self,
        domain: MappingDomain,
    ) -> MappingCompleteness {
        let expected = domain.len();

        if expected == 0 {
            return MappingCompleteness::Complete;
        }

        let mut mapped = 0usize;

        for logical in domain.iter() {
            if self.contains_logical(logical) {
                mapped += 1;
            }
        }

        if mapped == expected {
            MappingCompleteness::Complete
        } else {
            MappingCompleteness::Partial { expected, mapped }
        }
    }

    /// Verifies that the mapping is complete for a domain.
    pub fn require_complete(
        &self,
        domain: MappingDomain,
    ) -> Result<(), MappingError> {
        match self.completeness(domain) {
            MappingCompleteness::Complete => Ok(()),

            MappingCompleteness::Partial { expected, mapped } => {
                Err(MappingError::IncompleteMapping { expected, mapped })
            }
        }
    }

    /// Returns an inverse mapping.
    ///
    /// Because `QubitMapping` is injective, the inverse contains exactly one
    /// logical owner for every mapped physical resource.
    ///
    /// The returned mapping uses the same canonical identity domains in the
    /// opposite semantic direction:
    ///
    /// ```text
    /// original:
    ///     q0 -> p7
    ///
    /// inverse:
    ///     p7 -> q0
    /// ```
    ///
    /// The physical IDs are represented as the logical side of the returned
    /// value by converting through their numeric identity value.
    ///
    /// This method is intentionally provided as a separate operation instead
    /// of changing the meaning of `QubitMapping`, whose canonical direction
    /// remains logical -> physical.
    pub fn inverse(
        &self,
    ) -> Result<BTreeMap<PhysicalQubitId, QubitId>, MappingError> {
        self.validate()?;

        Ok(self.physical_to_logical.clone())
    }

    /// Validates all internal mapping invariants.
    ///
    /// A successfully constructed `QubitMapping` should always validate.
    /// This method exists as a defensive boundary for:
    ///
    /// - deserialization;
    /// - compiler diagnostics;
    /// - debugging;
    /// - fuzzing;
    /// - integration testing;
    /// - future representation changes.
    pub fn validate(&self) -> Result<(), MappingError> {
        if self.logical_to_physical.len()
            != self.physical_to_logical.len()
        {
            return Err(MappingError::InvariantViolation {
                logical: QubitId::new(0),
                physical: PhysicalQubitId::new(0),
            });
        }

        for (logical, physical) in &self.logical_to_physical {
            match self.physical_to_logical.get(physical) {
                Some(owner) if owner == logical => {}
                _ => {
                    return Err(MappingError::InvariantViolation {
                        logical: *logical,
                        physical: *physical,
                    });
                }
            }
        }

        for (physical, logical) in &self.physical_to_logical {
            match self.logical_to_physical.get(logical) {
                Some(mapped) if mapped == physical => {}
                _ => {
                    return Err(MappingError::InvariantViolation {
                        logical: *logical,
                        physical: *physical,
                    });
                }
            }
        }

        Ok(())
    }

    /// Creates a mapping from an iterator of mapping entries.
    ///
    /// The operation is fail-fast and never returns a partially constructed
    /// mapping.
    pub fn from_entries<I>(
        entries: I,
    ) -> Result<Self, MappingError>
    where
        I: IntoIterator<Item = MappingEntry>,
    {
        let mut mapping = Self::new();

        for entry in entries {
            mapping.insert(entry.logical(), entry.physical())?;
        }

        mapping.validate()?;

        Ok(mapping)
    }

    /// Creates a mapping from logical/physical pairs.
    ///
    /// This is convenient for routing and lowering code while preserving all
    /// insertion invariants.
    pub fn from_pairs<I>(
        pairs: I,
    ) -> Result<Self, MappingError>
    where
        I: IntoIterator<Item = (QubitId, PhysicalQubitId)>,
    {
        let mut mapping = Self::new();

        for (logical, physical) in pairs {
            mapping.insert(logical, physical)?;
        }

        mapping.validate()?;

        Ok(mapping)
    }

    /// Returns a detached, deterministic vector of mapping entries.
    ///
    /// This is useful for serialization and hashing layers that need an
    /// owned sequence without exposing the internal map representation.
    #[must_use]
    pub fn to_entries(&self) -> Vec<MappingEntry> {
        self.iter().collect()
    }
}

// =============================================================================
// Trait integrations
// =============================================================================

impl<'a> IntoIterator for &'a QubitMapping {
    type Item = MappingEntry;
    type IntoIter =
        std::iter::Map<
            std::collections::btree_map::Iter<
                'a,
                QubitId,
                PhysicalQubitId,
            >,
            fn(
                (&'a QubitId, &'a PhysicalQubitId),
            ) -> MappingEntry,
        >;

    fn into_iter(self) -> Self::IntoIter {
        fn make_entry(
            pair: (&QubitId, &PhysicalQubitId),
        ) -> MappingEntry {
            MappingEntry::new(*pair.0, *pair.1)
        }

        self.logical_to_physical.iter().map(make_entry)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn p(index: usize) -> PhysicalQubitId {
        PhysicalQubitId::new(index)
    }

    #[test]
    fn empty_mapping_is_empty() {
        let mapping = QubitMapping::new();

        assert!(mapping.is_empty());
        assert_eq!(mapping.len(), 0);
    }

    #[test]
    fn insert_creates_both_directions() {
        let mut mapping = QubitMapping::new();

        mapping.insert(q(0), p(7)).unwrap();

        assert_eq!(mapping.physical_for(q(0)), Some(p(7)));
        assert_eq!(mapping.logical_for(p(7)), Some(q(0)));
        assert_eq!(mapping.len(), 1);
    }

    #[test]
    fn duplicate_identical_insert_is_idempotent() {
        let mut mapping = QubitMapping::new();

        mapping.insert(q(0), p(7)).unwrap();
        mapping.insert(q(0), p(7)).unwrap();

        assert_eq!(mapping.len(), 1);
        assert_eq!(mapping.physical_for(q(0)), Some(p(7)));
    }

    #[test]
    fn logical_conflict_does_not_mutate_mapping() {
        let mut mapping = QubitMapping::new();

        mapping.insert(q(0), p(7)).unwrap();

        let result = mapping.insert(q(0), p(8));

        assert!(matches!(
            result,
            Err(MappingError::LogicalAlreadyMapped {
                logical,
                existing,
                requested
            }) if logical == q(0)
                && existing == p(7)
                && requested == p(8)
        ));

        assert_eq!(mapping.physical_for(q(0)), Some(p(7)));
        assert_eq!(mapping.logical_for(p(7)), Some(q(0)));
        assert_eq!(mapping.logical_for(p(8)), None);
    }

    #[test]
    fn physical_conflict_does_not_mutate_mapping() {
        let mut mapping = QubitMapping::new();

        mapping.insert(q(0), p(7)).unwrap();

        let result = mapping.insert(q(1), p(7));

        assert!(matches!(
            result,
            Err(MappingError::PhysicalAlreadyMapped {
                physical,
                existing,
                requested
            }) if physical == p(7)
                && existing == q(0)
                && requested == q(1)
        ));

        assert_eq!(mapping.physical_for(q(0)), Some(p(7)));
        assert_eq!(mapping.physical_for(q(1)), None);
        assert_eq!(mapping.logical_for(p(7)), Some(q(0)));
    }

    #[test]
    fn remap_replaces_assignment_atomically() {
        let mut mapping = QubitMapping::new();

        mapping.insert(q(0), p(7)).unwrap();

        let previous = mapping.remap(q(0), p(9)).unwrap();

        assert_eq!(previous, Some(p(7)));
        assert_eq!(mapping.physical_for(q(0)), Some(p(9)));
        assert_eq!(mapping.logical_for(p(7)), None);
        assert_eq!(mapping.logical_for(p(9)), Some(q(0)));
        assert_eq!(mapping.len(), 1);
    }

    #[test]
    fn remap_rejects_occupied_physical_without_mutation() {
        let mut mapping = QubitMapping::new();

        mapping.insert(q(0), p(7)).unwrap();
        mapping.insert(q(1), p(8)).unwrap();

        let result = mapping.remap(q(0), p(8));

        assert!(matches!(
            result,
            Err(MappingError::PhysicalAlreadyMapped {
                physical,
                existing,
                requested
            }) if physical == p(8)
                && existing == q(1)
                && requested == q(0)
        ));

        assert_eq!(mapping.physical_for(q(0)), Some(p(7)));
        assert_eq!(mapping.physical_for(q(1)), Some(p(8)));
    }

    #[test]
    fn remove_logical_removes_reverse_entry() {
        let mut mapping = QubitMapping::new();

        mapping.insert(q(0), p(7)).unwrap();

        assert_eq!(
            mapping.remove_logical(q(0)),
            Some(p(7))
        );

        assert!(mapping.is_empty());
        assert_eq!(mapping.logical_for(p(7)), None);
    }

    #[test]
    fn remove_physical_removes_reverse_entry() {
        let mut mapping = QubitMapping::new();

        mapping.insert(q(0), p(7)).unwrap();

        assert_eq!(
            mapping.remove_physical(p(7)),
            Some(q(0))
        );

        assert!(mapping.is_empty());
        assert_eq!(mapping.physical_for(q(0)), None);
    }

    #[test]
    fn logical_assignment_swap_only_changes_mapping_state() {
        let mut mapping = QubitMapping::new();

        mapping.insert(q(0), p(7)).unwrap();
        mapping.insert(q(1), p(8)).unwrap();

        mapping.swap_logical_assignments(q(0), q(1)).unwrap();

        assert_eq!(mapping.physical_for(q(0)), Some(p(8)));
        assert_eq!(mapping.physical_for(q(1)), Some(p(7)));
        assert_eq!(mapping.logical_for(p(7)), Some(q(1)));
        assert_eq!(mapping.logical_for(p(8)), Some(q(0)));
    }

    #[test]
    fn mapping_domain_does_not_overflow_at_max_identifier() {
        let start = QubitId::new(usize::MAX);

        let domain = MappingDomain::single(start);

        assert_eq!(domain.len(), 1);
        assert!(domain.contains(start));
        assert_eq!(domain.get(0), Some(start));

        let values: Vec<_> = domain.iter().collect();

        assert_eq!(values, vec![start]);
    }

    #[test]
    fn mapping_domain_supports_large_sparse_ranges_without_materialization() {
        let start = QubitId::new(1_000_000_000);
        let domain = MappingDomain::new(start, 3).unwrap();

        assert_eq!(domain.get(0), Some(q(1_000_000_000)));
        assert_eq!(domain.get(1), Some(q(1_000_000_001)));
        assert_eq!(domain.get(2), Some(q(1_000_000_002)));
        assert_eq!(domain.get(3), None);
    }

    #[test]
    fn completeness_detects_partial_mapping() {
        let mut mapping = QubitMapping::new();

        mapping.insert(q(0), p(7)).unwrap();
        mapping.insert(q(1), p(8)).unwrap();

        let domain = MappingDomain::new(q(0), 3).unwrap();

        assert_eq!(
            mapping.completeness(domain),
            MappingCompleteness::Partial {
                expected: 3,
                mapped: 2
            }
        );
    }

    #[test]
    fn completeness_detects_complete_mapping() {
        let mut mapping = QubitMapping::new();

        mapping.insert(q(0), p(7)).unwrap();
        mapping.insert(q(1), p(8)).unwrap();
        mapping.insert(q(2), p(9)).unwrap();

        let domain = MappingDomain::new(q(0), 3).unwrap();

        assert_eq!(
            mapping.completeness(domain),
            MappingCompleteness::Complete
        );

        mapping.require_complete(domain).unwrap();
    }

    #[test]
    fn empty_domain_is_complete() {
        let mapping = QubitMapping::new();
        let domain = MappingDomain::empty(q(0));

        assert_eq!(
            mapping.completeness(domain),
            MappingCompleteness::Complete
        );
    }

    #[test]
    fn inverse_is_deterministic() {
        let mut mapping = QubitMapping::new();

        mapping.insert(q(2), p(9)).unwrap();
        mapping.insert(q(0), p(7)).unwrap();
        mapping.insert(q(1), p(8)).unwrap();

        let inverse = mapping.inverse().unwrap();

        assert_eq!(inverse.get(&p(7)), Some(&q(0)));
        assert_eq!(inverse.get(&p(8)), Some(&q(1)));
        assert_eq!(inverse.get(&p(9)), Some(&q(2)));
    }

    #[test]
    fn iteration_is_deterministic() {
        let mut mapping = QubitMapping::new();

        mapping.insert(q(2), p(9)).unwrap();
        mapping.insert(q(0), p(7)).unwrap();
        mapping.insert(q(1), p(8)).unwrap();

        let entries = mapping.to_entries();

        assert_eq!(
            entries,
            vec![
                MappingEntry::new(q(0), p(7)),
                MappingEntry::new(q(1), p(8)),
                MappingEntry::new(q(2), p(9)),
            ]
        );
    }

    #[test]
    fn from_entries_rejects_duplicate_physical_owner() {
        let result = QubitMapping::from_entries([
            MappingEntry::new(q(0), p(7)),
            MappingEntry::new(q(1), p(7)),
        ]);

        assert!(matches!(
            result,
            Err(MappingError::PhysicalAlreadyMapped { .. })
        ));
    }

    #[test]
    fn validation_succeeds_for_valid_mapping() {
        let mut mapping = QubitMapping::new();

        mapping.insert(q(0), p(7)).unwrap();
        mapping.insert(q(1), p(8)).unwrap();

        mapping.validate().unwrap();
    }

    #[test]
    fn clear_removes_both_indexes() {
        let mut mapping = QubitMapping::new();

        mapping.insert(q(0), p(7)).unwrap();
        mapping.insert(q(1), p(8)).unwrap();

        mapping.clear();

        assert!(mapping.is_empty());
        assert_eq!(mapping.logical_for(p(7)), None);
        assert_eq!(mapping.logical_for(p(8)), None);
    }

    #[test]
    fn missing_lookup_has_explicit_error() {
        let mapping = QubitMapping::new();

        assert!(matches!(
            mapping.require_physical(q(0)),
            Err(MappingError::LogicalNotMapped { logical }) if logical == q(0)
        ));

        assert!(matches!(
            mapping.require_logical(p(0)),
            Err(MappingError::PhysicalNotMapped { physical }) if physical == p(0)
        ));
    }
}