//! Zamani Quantum IR — Logical-to-Physical Qubit Mapping
//!
//! Canonical, hardware-independent representation of logical-to-physical
//! qubit identity mappings.
//!
//! # Architectural role
//!
//! `quantum::ir::mapping` owns the semantic representation of a mapping
//! between:
//!
//! ```text
//! logical qubit  ->  physical qubit
//! ```
//!
//! The canonical qubit identity types are imported from:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module DOES:
//!
//! - represent logical-to-physical assignments;
//! - represent partial mappings;
//! - represent complete mappings;
//! - represent mapping entries;
//! - provide deterministic iteration;
//! - validate uniqueness and consistency;
//! - provide checked insertion and replacement;
//! - support removal and lookup;
//! - support inversion when the mapping is bijective;
//! - support immutable mapping views;
//! - provide mapping fingerprints suitable for deterministic local use;
//! - distinguish complete mappings from partial mappings;
//! - remain independent of hardware topology;
//! - remain independent of routing algorithms;
//! - remain independent of scheduling;
//! - remain independent of calibration;
//! - remain independent of backend execution.
//!
//! This module DOES NOT:
//!
//! - choose where a logical qubit should be placed;
//! - calculate a routing solution;
//! - inspect hardware topology;
//! - inspect hardware calibration;
//! - inspect gate connectivity;
//! - perform SWAP insertion;
//! - schedule operations;
//! - generate pulses;
//! - execute a QPU job;
//! - communicate with a backend;
//! - claim that a physical qubit actually exists;
//! - claim that a physical qubit is available;
//! - claim that a physical qubit supports an operation;
//! - impose a fixed maximum number of qubits.
//!
//! Those responsibilities belong to routing, hardware, scheduling,
//! optimization, pulse compilation, and backend subsystems.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once and may target different quantum machines.
//!
//! Consequently, this module contains NO architectural maximum such as:
//!
//! ```text
//! 63
//! 64
//! 4096
//! 1_000_000
//! ```
//!
//! The size of a mapping is limited only by:
//!
//! 1. the representable identifier space;
//! 2. available memory/resources;
//! 3. explicit resource/security policies enforced by higher layers.
//!
//! A mapping containing one qubit and a mapping containing an arbitrarily
//! large finite number of qubits use exactly the same representation model.
//!
//! # Important distinction
//!
//! ```text
//! mapping.rs
//!     = records WHERE logical qubits are mapped.
//!
//! routing/
//!     = decides HOW to obtain a valid mapping.
//!
//! hardware/
//!     = describes WHICH physical resources exist and what they support.
//!
//! scheduling/
//!     = decides WHEN mapped operations execute.
//! ```
//!
//! # Physical identity semantics
//!
//! `PhysicalQubitId` is an IR identity vocabulary. Creating or storing one
//! does not prove that a hardware device contains that qubit.
//!
//! Hardware validation must happen at the hardware compatibility boundary.
//!
//! # Partial mappings
//!
//! A mapping may intentionally be partial.
//!
//! Example:
//!
//! ```text
//! q0 -> p7
//! q1 -> p12
//! q2 -> <unmapped>
//! ```
//!
//! This is useful during compilation and routing.
//!
//! A complete mapping means that every logical qubit in the declared domain
//! has exactly one physical target.
//!
//! # Bijectivity
//!
//! A valid mapping is injective:
//!
//! ```text
//! q0 -> p7
//! q1 -> p12
//! q2 -> p7   // invalid
//! ```
//!
//! Two distinct logical qubits must never occupy the same physical qubit in
//! the same mapping state.
//!
//! The inverse mapping is therefore only defined when requested for the
//! current mapping and is guaranteed to contain one logical owner per mapped
//! physical qubit.
//!
//! # Stability and determinism
//!
//! Entries are stored in `BTreeMap`, giving deterministic ordering without
//! relying on hash-map iteration order.
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
//! - no `unsafe`;
//! - no external dependencies.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! This file intentionally depends only on the canonical qubit module and
//! the Rust standard library.
//!
//! Therefore later IR modules may consume this API without requiring this
//! file to be modified.
//!
//! Expected consumers:
//!
//! - `quantum::ir::program`
//! - `quantum::ir::resource`
//! - `quantum::ir::capability`
//! - `quantum::ir::validation`
//! - `quantum::ir::analysis`
//! - `quantum::routing`
//! - `quantum::hardware`
//! - `quantum::scheduling`
//! - backend adapters
//!
//! Routing and hardware code may construct a `QubitMapping`, while this
//! module remains unaware of those implementations.
//!
//! # Example
//!
//! ```ignore
//! use crate::quantum::ir::mapping::QubitMapping;
//! use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
//!
//! let mut mapping = QubitMapping::new();
//! mapping.insert(QubitId::new(0), PhysicalQubitId::new(17))?;
//! mapping.insert(QubitId::new(1), PhysicalQubitId::new(42))?;
//!
//! assert_eq!(
//!     mapping.physical_for(QubitId::new(0)),
//!     Some(PhysicalQubitId::new(17))
//! );
//! # Ok::<(), crate::quantum::ir::mapping::MappingError>(())
//! ```
//!
//! The `ignore` marker is used because this file intentionally does not
//! assume the surrounding crate's final module path or error integration.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt;

use super::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Mapping entry
// =============================================================================

/// A single logical-to-physical qubit mapping entry.
///
/// This is a value object and contains no hardware information beyond the
/// physical identity reference itself.
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
        write!(formatter, "{} -> {}", self.logical, self.physical)
    }
}

// =============================================================================
// Mapping errors
// =============================================================================

/// Errors produced by mapping operations.
///
/// These errors are local to the mapping abstraction and deliberately do not
/// depend on the global IR error taxonomy. A future `errors.rs` can wrap or
/// translate these errors without requiring changes here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MappingError {
    /// A logical qubit already has a different physical assignment.
    LogicalAlreadyMapped {
        logical: QubitId,
        existing: PhysicalQubitId,
        requested: PhysicalQubitId,
    },

    /// A physical qubit is already occupied by another logical qubit.
    PhysicalAlreadyMapped {
        physical: PhysicalQubitId,
        existing: QubitId,
        requested: QubitId,
    },

    /// The requested logical qubit is not mapped.
    LogicalNotMapped {
        logical: QubitId,
    },

    /// The requested physical qubit is not mapped.
    PhysicalNotMapped {
        physical: PhysicalQubitId,
    },

    /// A complete mapping was requested for a logical domain but one or more
    /// logical qubits have no physical assignment.
    IncompleteMapping {
        expected: usize,
        mapped: usize,
    },

    /// The requested operation would cause a checked arithmetic overflow.
    SizeOverflow,

    /// A caller supplied an invalid domain range.
    InvalidDomain {
        start: usize,
        end: usize,
    },

    /// The mapping contains an invariant violation.
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
                 but only {mapped} are mapped"
            ),

            Self::SizeOverflow => {
                write!(formatter, "mapping size calculation overflowed")
            }

            Self::InvalidDomain { start, end } => {
                write!(
                    formatter,
                    "invalid logical domain [{start}, {end}): start exceeds end"
                )
            }

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

/// Logical-qubit domain used when checking mapping completeness.
///
/// A domain is semantic information supplied by the caller. It does not
/// allocate qubits.
///
/// The half-open representation `[start, end)` allows large domains to be
/// represented without materializing every logical identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MappingDomain {
    start: usize,
    end: usize,
}

impl MappingDomain {
    /// Creates a logical domain `[start, end)`.
    pub const fn new(start: usize, end: usize) -> Result<Self, MappingError> {
        if start > end {
            return Err(MappingError::InvalidDomain { start, end });
        }

        Ok(Self { start, end })
    }

    /// Creates a domain containing exactly one logical qubit.
    #[must_use]
    pub const fn single(logical: QubitId) -> Self {
        let index = logical.index();

        Self {
            start: index,
            end: match index.checked_add(1) {
                Some(value) => value,
                None => index,
            },
        }
    }

    /// Creates an empty domain.
    #[must_use]
    pub const fn empty(index: usize) -> Self {
        Self {
            start: index,
            end: index,
        }
    }

    /// Returns the first logical index.
    #[must_use]
    pub const fn start(self) -> usize {
        self.start
    }

    /// Returns the exclusive end.
    #[must_use]
    pub const fn end(self) -> usize {
        self.end
    }

    /// Returns the number of logical identifiers in this domain.
    #[must_use]
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    /// Returns whether the domain contains no logical identifiers.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// Returns whether a logical qubit belongs to this domain.
    #[must_use]
    pub const fn contains(self, logical: QubitId) -> bool {
        let index = logical.index();

        index >= self.start && index < self.end
    }

    /// Returns an iterator over the logical identifiers in the domain.
    pub fn iter(self) -> MappingDomainIter {
        MappingDomainIter {
            current: self.start,
            end: self.end,
        }
    }
}

impl fmt::Display for MappingDomain {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[{}, {})", self.start, self.end)
    }
}

/// Iterator over a [`MappingDomain`].
#[derive(Debug, Clone)]
pub struct MappingDomainIter {
    current: usize,
    end: usize,
}

impl Iterator for MappingDomainIter {
    type Item = QubitId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            return None;
        }

        let current = self.current;

        self.current = match self.current.checked_add(1) {
            Some(next) => next,
            None => self.end,
        };

        Some(QubitId::new(current))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end.saturating_sub(self.current);
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for MappingDomainIter {}

impl std::iter::FusedIterator for MappingDomainIter {}

// =============================================================================
// Mapping
// =============================================================================

/// Canonical logical-to-physical qubit mapping.
///
/// The mapping is injective:
///
/// ```text
/// logical -> physical
/// ```
///
/// A physical qubit can belong to at most one logical qubit in a given
/// mapping.
///
/// The structure is intentionally independent from hardware and routing.
///
/// # Scalability
///
/// `QubitMapping` has no fixed architectural qubit limit. Its practical
/// capacity is determined by the memory available to the process and by
/// explicit resource policies imposed by higher layers.
///
/// The implementation uses `BTreeMap` rather than a fixed-size array, so
/// sparse identifiers and very large identifier values do not require
/// allocating all identifiers between zero and the largest identifier.
///
/// # Determinism
///
/// Both logical and physical indexes are maintained in deterministic ordered
/// maps. Iteration order is stable for the same mapping.
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

    /// Creates an empty mapping with a requested capacity.
    ///
    /// `BTreeMap` does not expose a stable capacity-reservation API on the
    /// supported Rust versions, so this method intentionally behaves exactly
    /// like `new`.
    ///
    /// It exists as an explicit API boundary so callers that work with
    /// capacity-aware containers do not need to know the internal collection
    /// type.
    #[must_use]
    pub const fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }

    /// Returns the number of mapped logical qubits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.logical_to_physical.len()
    }

    /// Returns whether the mapping contains no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.logical_to_physical.is_empty()
    }

    /// Removes all mappings.
    pub fn clear(&mut self) {
        self.logical_to_physical.clear();
        self.physical_to_logical.clear();
    }

    /// Inserts a new logical-to-physical mapping.
    ///
    /// Both identity domains are unique:
    ///
    /// - the logical qubit must not already map somewhere else;
    /// - the physical qubit must not already belong to another logical qubit.
    ///
    /// If the requested mapping already exists exactly, this method succeeds
    /// without changing the mapping.
    pub fn insert(
        &mut self,
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> Result<(), MappingError> {
        if let Some(existing) = self.logical_to_physical.get(&logical).copied() {
            if existing == physical {
                return Ok(());
            }

            return Err(MappingError::LogicalAlreadyMapped {
                logical,
                existing,
                requested: physical,
            });
        }

        if let Some(existing) = self.physical_to_logical.get(&physical).copied() {
            if existing == logical {
                return Ok(());
            }

            return Err(MappingError::PhysicalAlreadyMapped {
                physical,
                existing,
                requested: logical,
            });
        }

        self.logical_to_physical.insert(logical, physical);
        self.physical_to_logical.insert(physical, logical);

        Ok(())
    }

    /// Inserts or replaces a logical-to-physical mapping.
    ///
    /// Replacement is safe and preserves injectivity.
    ///
    /// If the requested physical qubit is owned by another logical qubit,
    /// the operation fails and the existing mapping remains unchanged.
    pub fn replace(
        &mut self,
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> Result<Option<PhysicalQubitId>, MappingError> {
        if let Some(owner) = self.physical_to_logical.get(&physical).copied() {
            if owner != logical {
                return Err(MappingError::PhysicalAlreadyMapped {
                    physical,
                    existing: owner,
                    requested: logical,
                });
            }
        }

        let previous = self.logical_to_physical.get(&logical).copied();

        if previous == Some(physical) {
            return Ok(previous);
        }

        if let Some(old_physical) = previous {
            self.logical_to_physical.remove(&logical);
            self.physical_to_logical.remove(&old_physical);
        }

        self.logical_to_physical.insert(logical, physical);
        self.physical_to_logical.insert(physical, logical);

        Ok(previous)
    }

    /// Removes a logical mapping and returns its physical target.
    pub fn remove_logical(
        &mut self,
        logical: QubitId,
    ) -> Result<PhysicalQubitId, MappingError> {
        let physical = self
            .logical_to_physical
            .remove(&logical)
            .ok_or(MappingError::LogicalNotMapped { logical })?;

        self.physical_to_logical.remove(&physical);

        Ok(physical)
    }

    /// Removes the mapping occupying a physical qubit and returns its logical
    /// owner.
    pub fn remove_physical(
        &mut self,
        physical: PhysicalQubitId,
    ) -> Result<QubitId, MappingError> {
        let logical = self
            .physical_to_logical
            .remove(&physical)
            .ok_or(MappingError::PhysicalNotMapped { physical })?;

        self.logical_to_physical.remove(&logical);

        Ok(logical)
    }

    /// Looks up the physical qubit assigned to a logical qubit.
    #[must_use]
    pub fn physical_for(&self, logical: QubitId) -> Option<PhysicalQubitId> {
        self.logical_to_physical.get(&logical).copied()
    }

    /// Looks up the logical qubit occupying a physical qubit.
    #[must_use]
    pub fn logical_for(&self, physical: PhysicalQubitId) -> Option<QubitId> {
        self.physical_to_logical.get(&physical).copied()
    }

    /// Returns whether a logical qubit has a mapping.
    #[must_use]
    pub fn contains_logical(&self, logical: QubitId) -> bool {
        self.logical_to_physical.contains_key(&logical)
    }

    /// Returns whether a physical qubit is occupied by this mapping.
    #[must_use]
    pub fn contains_physical(&self, physical: PhysicalQubitId) -> bool {
        self.physical_to_logical.contains_key(&physical)
    }

    /// Returns whether the mapping contains the exact assignment.
    #[must_use]
    pub fn contains_pair(
        &self,
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> bool {
        self.logical_to_physical.get(&logical).copied() == Some(physical)
    }

    /// Returns the mapping entry for a logical qubit.
    #[must_use]
    pub fn entry(&self, logical: QubitId) -> Option<MappingEntry> {
        self.physical_for(logical)
            .map(|physical| MappingEntry::new(logical, physical))
    }

    /// Returns all mapping entries in deterministic logical-ID order.
    pub fn entries(
        &self,
    ) -> impl Iterator<Item = MappingEntry> + '_ {
        self.logical_to_physical
            .iter()
            .map(|(&logical, &physical)| MappingEntry::new(logical, physical))
    }

    /// Returns all logical identifiers in deterministic order.
    pub fn logicals(&self) -> impl Iterator<Item = QubitId> + '_ {
        self.logical_to_physical.keys().copied()
    }

    /// Returns all physical identifiers in deterministic order.
    pub fn physicals(&self) -> impl Iterator<Item = PhysicalQubitId> + '_ {
        self.physical_to_logical.keys().copied()
    }

    /// Returns the number of distinct physical qubits occupied.
    #[must_use]
    pub fn physical_count(&self) -> usize {
        self.physical_to_logical.len()
    }

    /// Returns whether every logical qubit in the supplied domain is mapped.
    ///
    /// This does not require the domain to have been materialized.
    #[must_use]
    pub fn is_complete_for(&self, domain: MappingDomain) -> bool {
        if domain.len() != self.len() {
            return false;
        }

        domain.iter().all(|logical| self.contains_logical(logical))
    }

    /// Validates that the mapping is complete for the supplied logical domain.
    pub fn require_complete_for(
        &self,
        domain: MappingDomain,
    ) -> Result<(), MappingError> {
        let expected = domain.len();
        let mapped = self.len();

        if mapped != expected || !domain.iter().all(|logical| self.contains_logical(logical)) {
            return Err(MappingError::IncompleteMapping { expected, mapped });
        }

        Ok(())
    }

    /// Validates all internal mapping invariants.
    ///
    /// This is intentionally cheap enough to be used at subsystem boundaries.
    pub fn validate(&self) -> Result<(), MappingError> {
        if self.logical_to_physical.len() != self.physical_to_logical.len() {
            return Err(MappingError::InvariantViolation {
                logical: self
                    .logical_to_physical
                    .keys()
                    .next()
                    .copied()
                    .unwrap_or_else(|| QubitId::new(0)),
                physical: self
                    .physical_to_logical
                    .keys()
                    .next()
                    .copied()
                    .unwrap_or_else(|| PhysicalQubitId::new(0)),
            });
        }

        for (&logical, &physical) in &self.logical_to_physical {
            match self.physical_to_logical.get(&physical) {
                Some(owner) if *owner == logical => {}
                _ => {
                    return Err(MappingError::InvariantViolation { logical, physical });
                }
            }
        }

        for (&physical, &logical) in &self.physical_to_logical {
            match self.logical_to_physical.get(&logical) {
                Some(mapped) if *mapped == physical => {}
                _ => {
                    return Err(MappingError::InvariantViolation { logical, physical });
                }
            }
        }

        Ok(())
    }

    /// Creates the inverse mapping.
    ///
    /// Because `QubitMapping` enforces physical uniqueness, inversion is
    /// always well-defined for a valid mapping.
    #[must_use]
    pub fn inverse(&self) -> QubitMapping {
        let mut inverse = QubitMapping::new();

        for (&physical, &logical) in &self.physical_to_logical {
            // The source mapping already guarantees injectivity, therefore
            // insertion cannot fail unless the internal invariant was broken.
            //
            // Do not panic in production code. The result is built directly
            // from the two already-consistent indexes.
            inverse
                .logical_to_physical
                .insert(logical, physical);
            inverse
                .physical_to_logical
                .insert(physical, logical);
        }

        inverse
    }

    /// Returns a deterministic mapping fingerprint.
    ///
    /// This is NOT a cryptographic hash and must not be used for security,
    /// authentication, signatures, or cryptographic commitments.
    ///
    /// It is intended for deterministic local cache keys, equality shortcuts,
    /// diagnostics, and reproducibility support.
    #[must_use]
    pub fn fingerprint(&self) -> u64 {
        let mut hash = FNV_OFFSET_BASIS;

        for (&logical, &physical) in &self.logical_to_physical {
            hash = fnv_mix_usize(hash, logical.index());
            hash = fnv_mix_usize(hash, physical.index());
        }

        hash
    }

    /// Returns a snapshot of the mapping entries.
    ///
    /// This allocates a `Vec` and therefore should be used only when an owned
    /// collection is required. Use `entries()` for streaming iteration.
    pub fn to_entries(&self) -> Vec<MappingEntry> {
        self.entries().collect()
    }

    /// Constructs a mapping from an iterator of entries.
    ///
    /// Insertion is transactional with respect to each entry: if an entry is
    /// invalid, the partially constructed mapping is not returned.
    pub fn from_entries<I>(entries: I) -> Result<Self, MappingError>
    where
        I: IntoIterator<Item = MappingEntry>,
    {
        let mut mapping = Self::new();

        for entry in entries {
            mapping.insert(entry.logical(), entry.physical())?;
        }

        Ok(mapping)
    }

    /// Constructs a mapping from an iterator of `(QubitId, PhysicalQubitId)`
    /// pairs.
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

        Ok(mapping)
    }

    /// Extends the mapping using additional entries.
    ///
    /// Existing mappings are not replaced.
    pub fn extend<I>(&mut self, entries: I) -> Result<(), MappingError>
    where
        I: IntoIterator<Item = MappingEntry>,
    {
        for entry in entries {
            self.insert(entry.logical(), entry.physical())?;
        }

        Ok(())
    }

    /// Returns a mapping view suitable for read-only consumers.
    #[must_use]
    pub const fn as_view(&self) -> QubitMappingView<'_> {
        QubitMappingView { mapping: self }
    }
}

// =============================================================================
// Mapping view
// =============================================================================

/// Read-only view of a [`QubitMapping`].
///
/// This type allows routing, scheduling, validation and hardware integration
/// code to consume mappings without taking ownership or gaining mutation
/// access.
#[derive(Debug, Clone, Copy)]
pub struct QubitMappingView<'a> {
    mapping: &'a QubitMapping,
}

impl<'a> QubitMappingView<'a> {
    /// Returns the number of mapped logical qubits.
    #[must_use]
    pub fn len(self) -> usize {
        self.mapping.len()
    }

    /// Returns whether the mapping is empty.
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.mapping.is_empty()
    }

    /// Looks up a physical qubit.
    #[must_use]
    pub fn physical_for(self, logical: QubitId) -> Option<PhysicalQubitId> {
        self.mapping.physical_for(logical)
    }

    /// Looks up a logical qubit.
    #[must_use]
    pub fn logical_for(self, physical: PhysicalQubitId) -> Option<QubitId> {
        self.mapping.logical_for(physical)
    }

    /// Returns whether the logical qubit is mapped.
    #[must_use]
    pub fn contains_logical(self, logical: QubitId) -> bool {
        self.mapping.contains_logical(logical)
    }

    /// Returns whether the physical qubit is occupied.
    #[must_use]
    pub fn contains_physical(self, physical: PhysicalQubitId) -> bool {
        self.mapping.contains_physical(physical)
    }

    /// Iterates over mapping entries.
    pub fn entries(
        self,
    ) -> impl Iterator<Item = MappingEntry> + 'a {
        self.mapping.entries()
    }

    /// Validates the mapping.
    pub fn validate(self) -> Result<(), MappingError> {
        self.mapping.validate()
    }

    /// Checks completeness for a logical domain.
    #[must_use]
    pub fn is_complete_for(self, domain: MappingDomain) -> bool {
        self.mapping.is_complete_for(domain)
    }
}

// =============================================================================
// Mapping builder
// =============================================================================

/// Builder for constructing a mapping incrementally.
///
/// The builder provides an explicit construction boundary for compiler and
/// routing passes while keeping the final mapping immutable by convention
/// after construction.
#[derive(Debug, Clone, Default)]
pub struct QubitMappingBuilder {
    mapping: QubitMapping,
}

impl QubitMappingBuilder {
    /// Creates an empty builder.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            mapping: QubitMapping::new(),
        }
    }

    /// Adds a logical-to-physical assignment.
    pub fn map(
        &mut self,
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> Result<&mut Self, MappingError> {
        self.mapping.insert(logical, physical)?;
        Ok(self)
    }

    /// Adds an existing mapping entry.
    pub fn entry(
        &mut self,
        entry: MappingEntry,
    ) -> Result<&mut Self, MappingError> {
        self.map(entry.logical(), entry.physical())
    }

    /// Returns the number of entries currently built.
    #[must_use]
    pub fn len(&self) -> usize {
        self.mapping.len()
    }

    /// Returns whether no entries have been added.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.mapping.is_empty()
    }

    /// Validates completeness against a logical domain and returns the final
    /// mapping.
    pub fn build_complete(
        self,
        domain: MappingDomain,
    ) -> Result<QubitMapping, MappingError> {
        self.mapping.require_complete_for(domain)?;
        self.mapping.validate()?;
        Ok(self.mapping)
    }

    /// Builds the current mapping without requiring completeness.
    pub fn build(self) -> Result<QubitMapping, MappingError> {
        self.mapping.validate()?;
        Ok(self.mapping)
    }

    /// Returns a read-only view of the mapping being constructed.
    #[must_use]
    pub const fn as_view(&self) -> QubitMappingView<'_> {
        self.mapping.as_view()
    }
}

// =============================================================================
// Mapping transformations
// =============================================================================

/// A single mapping movement.
///
/// This type describes a change from one physical location to another for a
/// logical qubit. It does NOT imply that the move is physically executable.
///
/// Routing/scheduling/hardware layers must determine how the transition is
/// realized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MappingMove {
    logical: QubitId,
    from: Option<PhysicalQubitId>,
    to: PhysicalQubitId,
}

impl MappingMove {
    /// Creates a move from an optional previous physical location to a new
    /// physical location.
    #[must_use]
    pub const fn new(
        logical: QubitId,
        from: Option<PhysicalQubitId>,
        to: PhysicalQubitId,
    ) -> Self {
        Self { logical, from, to }
    }

    /// Returns the logical qubit being moved.
    #[must_use]
    pub const fn logical(self) -> QubitId {
        self.logical
    }

    /// Returns the previous physical location.
    #[must_use]
    pub const fn from(self) -> Option<PhysicalQubitId> {
        self.from
    }

    /// Returns the destination physical location.
    #[must_use]
    pub const fn to(self) -> PhysicalQubitId {
        self.to
    }
}

impl QubitMapping {
    /// Computes the changes required to transform this mapping into `target`.
    ///
    /// The method does not perform routing and does not mutate either mapping.
    ///
    /// The returned moves describe semantic mapping changes only.
    pub fn diff(&self, target: &QubitMapping) -> Vec<MappingMove> {
        let mut moves = Vec::new();

        for (&logical, &target_physical) in &target.logical_to_physical {
            let current_physical = self.logical_to_physical.get(&logical).copied();

            if current_physical != Some(target_physical) {
                moves.push(MappingMove::new(
                    logical,
                    current_physical,
                    target_physical,
                ));
            }
        }

        for (&logical, &current_physical) in &self.logical_to_physical {
            if !target.logical_to_physical.contains_key(&logical) {
                // There is no destination in the target mapping. We do not
                // manufacture a destination and therefore do not emit a move.
                let _ = current_physical;
            }
        }

        moves
    }

    /// Applies a sequence of semantic mapping moves.
    ///
    /// The sequence is checked incrementally. If a move is invalid, the
    /// operation stops with an error.
    pub fn apply_moves<I>(
        &mut self,
        moves: I,
    ) -> Result<(), MappingError>
    where
        I: IntoIterator<Item = MappingMove>,
    {
        for movement in moves {
            let current = self.physical_for(movement.logical());

            if current != movement.from() {
                return Err(match movement.from() {
                    Some(expected) => {
                        if let Some(actual) = current {
                            MappingError::LogicalAlreadyMapped {
                                logical: movement.logical(),
                                existing: actual,
                                requested: movement.to(),
                            }
                        } else {
                            MappingError::LogicalNotMapped {
                                logical: movement.logical(),
                            }
                        }
                    }

                    None => MappingError::LogicalAlreadyMapped {
                        logical: movement.logical(),
                        existing: current.unwrap_or(movement.to()),
                        requested: movement.to(),
                    },
                });
            }

            if movement.from().is_some() {
                self.remove_logical(movement.logical())?;
            }

            self.insert(movement.logical(), movement.to())?;
        }

        self.validate()
    }
}

// =============================================================================
// Deterministic lightweight fingerprint
// =============================================================================

const FNV_OFFSET_BASIS: u64 = 14_695_981_039_346_656_037;
const FNV_PRIME: u64 = 1_099_511_628_211;

fn fnv_mix_byte(mut hash: u64, byte: u8) -> u64 {
    hash ^= u64::from(byte);
    hash.wrapping_mul(FNV_PRIME)
}

fn fnv_mix_usize(mut hash: u64, value: usize) -> u64 {
    let bytes = value.to_le_bytes();

    for byte in bytes {
        hash = fnv_mix_byte(hash, byte);
    }

    hash
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
    fn empty_mapping_is_empty() {
        let mapping = QubitMapping::new();

        assert!(mapping.is_empty());
        assert_eq!(mapping.len(), 0);
        assert_eq!(mapping.physical_count(), 0);
    }

    #[test]
    fn insert_and_lookup_both_directions() {
        let mut mapping = QubitMapping::new();

        mapping.insert(logical(0), physical(17)).unwrap();

        assert_eq!(
            mapping.physical_for(logical(0)),
            Some(physical(17))
        );

        assert_eq!(
            mapping.logical_for(physical(17)),
            Some(logical(0))
        );
    }

    #[test]
    fn duplicate_same_pair_is_idempotent() {
        let mut mapping = QubitMapping::new();

        mapping.insert(logical(0), physical(17)).unwrap();
        mapping.insert(logical(0), physical(17)).unwrap();

        assert_eq!(mapping.len(), 1);
    }

    #[test]
    fn logical_cannot_be_mapped_twice() {
        let mut mapping = QubitMapping::new();

        mapping.insert(logical(0), physical(17)).unwrap();

        let error = mapping
            .insert(logical(0), physical(42))
            .unwrap_err();

        assert_eq!(
            error,
            MappingError::LogicalAlreadyMapped {
                logical: logical(0),
                existing: physical(17),
                requested: physical(42),
            }
        );
    }

    #[test]
    fn physical_cannot_be_occupied_twice() {
        let mut mapping = QubitMapping::new();

        mapping.insert(logical(0), physical(17)).unwrap();

        let error = mapping
            .insert(logical(1), physical(17))
            .unwrap_err();

        assert_eq!(
            error,
            MappingError::PhysicalAlreadyMapped {
                physical: physical(17),
                existing: logical(0),
                requested: logical(1),
            }
        );
    }

    #[test]
    fn replace_moves_logical_qubit() {
        let mut mapping = QubitMapping::new();

        mapping.insert(logical(0), physical(17)).unwrap();

        let previous = mapping
            .replace(logical(0), physical(42))
            .unwrap();

        assert_eq!(previous, Some(physical(17)));
        assert_eq!(
            mapping.physical_for(logical(0)),
            Some(physical(42))
        );
        assert_eq!(
            mapping.logical_for(physical(17)),
            None
        );
    }

    #[test]
    fn replacement_cannot_take_another_logicals_physical_qubit() {
        let mut mapping = QubitMapping::new();

        mapping.insert(logical(0), physical(17)).unwrap();
        mapping.insert(logical(1), physical(42)).unwrap();

        let error = mapping
            .replace(logical(0), physical(42))
            .unwrap_err();

        assert_eq!(
            error,
            MappingError::PhysicalAlreadyMapped {
                physical: physical(42),
                existing: logical(1),
                requested: logical(0),
            }
        );

        assert_eq!(
            mapping.physical_for(logical(0)),
            Some(physical(17))
        );
    }

    #[test]
    fn remove_logical_removes_both_indexes() {
        let mut mapping = QubitMapping::new();

        mapping.insert(logical(0), physical(17)).unwrap();

        assert_eq!(
            mapping.remove_logical(logical(0)).unwrap(),
            physical(17)
        );

        assert_eq!(mapping.physical_for(logical(0)), None);
        assert_eq!(mapping.logical_for(physical(17)), None);
        assert!(mapping.is_empty());
    }

    #[test]
    fn remove_physical_removes_both_indexes() {
        let mut mapping = QubitMapping::new();

        mapping.insert(logical(0), physical(17)).unwrap();

        assert_eq!(
            mapping.remove_physical(physical(17)).unwrap(),
            logical(0)
        );

        assert_eq!(mapping.physical_for(logical(0)), None);
        assert_eq!(mapping.logical_for(physical(17)), None);
        assert!(mapping.is_empty());
    }

    #[test]
    fn domain_supports_large_sparse_identifier_space() {
        let domain = MappingDomain::new(1_000_000, 1_000_003).unwrap();

        assert_eq!(domain.len(), 3);
        assert!(domain.contains(logical(1_000_000)));
        assert!(domain.contains(logical(1_000_002)));
        assert!(!domain.contains(logical(1_000_003)));
    }

    #[test]
    fn domain_is_half_open() {
        let domain = MappingDomain::new(2, 5).unwrap();

        let values: Vec<_> = domain.iter().map(QubitId::index).collect();

        assert_eq!(values, vec![2, 3, 4]);
    }

    #[test]
    fn complete_mapping_is_detected() {
        let mut mapping = QubitMapping::new();

        mapping.insert(logical(0), physical(7)).unwrap();
        mapping.insert(logical(1), physical(8)).unwrap();
        mapping.insert(logical(2), physical(9)).unwrap();

        let domain = MappingDomain::new(0, 3).unwrap();

        assert!(mapping.is_complete_for(domain));
        assert!(mapping.require_complete_for(domain).is_ok());
    }

    #[test]
    fn incomplete_mapping_is_rejected() {
        let mut mapping = QubitMapping::new();

        mapping.insert(logical(0), physical(7)).unwrap();
        mapping.insert(logical(2), physical(9)).unwrap();

        let domain = MappingDomain::new(0, 3).unwrap();

        assert!(!mapping.is_complete_for(domain));

        assert_eq!(
            mapping.require_complete_for(domain).unwrap_err(),
            MappingError::IncompleteMapping {
                expected: 3,
                mapped: 2,
            }
        );
    }

    #[test]
    fn inverse_mapping_is_correct() {
        let mut mapping = QubitMapping::new();

        mapping.insert(logical(0), physical(7)).unwrap();
        mapping.insert(logical(1), physical(42)).unwrap();

        let inverse = mapping.inverse();

        assert_eq!(
            inverse.physical_for(logical(7)),
            Some(physical(0))
        );

        assert_eq!(
            inverse.physical_for(logical(42)),
            Some(physical(1))
        );
    }

    #[test]
    fn entries_are_deterministic() {
        let mut mapping = QubitMapping::new();

        mapping.insert(logical(9), physical(90)).unwrap();
        mapping.insert(logical(1), physical(10)).unwrap();
        mapping.insert(logical(5), physical(50)).unwrap();

        let values: Vec<_> = mapping
            .entries()
            .map(|entry| entry.logical().index())
            .collect();

        assert_eq!(values, vec![1, 5, 9]);
    }

    #[test]
    fn from_pairs_builds_mapping() {
        let mapping = QubitMapping::from_pairs([
            (logical(0), physical(7)),
            (logical(1), physical(8)),
        ])
        .unwrap();

        assert_eq!(mapping.len(), 2);
        assert_eq!(
            mapping.physical_for(logical(1)),
            Some(physical(8))
        );
    }

    #[test]
    fn invalid_from_pairs_is_rejected() {
        let result = QubitMapping::from_pairs([
            (logical(0), physical(7)),
            (logical(1), physical(7)),
        ]);

        assert!(matches!(
            result,
            Err(MappingError::PhysicalAlreadyMapped { .. })
        ));
    }

    #[test]
    fn builder_creates_complete_mapping() {
        let mut builder = QubitMappingBuilder::new();

        builder.map(logical(0), physical(7)).unwrap();
        builder.map(logical(1), physical(8)).unwrap();

        let domain = MappingDomain::new(0, 2).unwrap();

        let mapping = builder.build_complete(domain).unwrap();

        assert_eq!(mapping.len(), 2);
    }

    #[test]
    fn builder_rejects_incomplete_mapping() {
        let mut builder = QubitMappingBuilder::new();

        builder.map(logical(0), physical(7)).unwrap();

        let domain = MappingDomain::new(0, 2).unwrap();

        assert!(matches!(
            builder.build_complete(domain),
            Err(MappingError::IncompleteMapping { .. })
        ));
    }

    #[test]
    fn mapping_validation_succeeds_for_normal_mapping() {
        let mut mapping = QubitMapping::new();

        mapping.insert(logical(0), physical(17)).unwrap();
        mapping.insert(logical(1), physical(42)).unwrap();
        mapping.insert(logical(2), physical(99)).unwrap();

        assert!(mapping.validate().is_ok());
    }

    #[test]
    fn view_is_read_only() {
        let mut mapping = QubitMapping::new();

        mapping.insert(logical(0), physical(17)).unwrap();

        let view = mapping.as_view();

        assert_eq!(
            view.physical_for(logical(0)),
            Some(physical(17))
        );
        assert_eq!(view.len(), 1);
    }

    #[test]
    fn fingerprint_is_deterministic() {
        let mut first = QubitMapping::new();
        let mut second = QubitMapping::new();

        first.insert(logical(0), physical(17)).unwrap();
        first.insert(logical(1), physical(42)).unwrap();

        second.insert(logical(1), physical(42)).unwrap();
        second.insert(logical(0), physical(17)).unwrap();

        assert_eq!(first.fingerprint(), second.fingerprint());
    }

    #[test]
    fn different_mappings_have_different_normal_fingerprints() {
        let mut first = QubitMapping::new();
        let mut second = QubitMapping::new();

        first.insert(logical(0), physical(17)).unwrap();
        second.insert(logical(0), physical(18)).unwrap();

        assert_ne!(first.fingerprint(), second.fingerprint());
    }

    #[test]
    fn diff_detects_mapping_change() {
        let mut current = QubitMapping::new();
        let mut target = QubitMapping::new();

        current.insert(logical(0), physical(7)).unwrap();
        target.insert(logical(0), physical(42)).unwrap();

        let changes = current.diff(&target);

        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].logical(), logical(0));
        assert_eq!(changes[0].from(), Some(physical(7)));
        assert_eq!(changes[0].to(), physical(42));
    }

    #[test]
    fn apply_moves_changes_mapping() {
        let mut mapping = QubitMapping::new();

        mapping.insert(logical(0), physical(7)).unwrap();

        mapping
            .apply_moves([
                MappingMove::new(
                    logical(0),
                    Some(physical(7)),
                    physical(42),
                ),
            ])
            .unwrap();

        assert_eq!(
            mapping.physical_for(logical(0)),
            Some(physical(42))
        );

        assert_eq!(
            mapping.logical_for(physical(7)),
            None
        );
    }

    #[test]
    fn mapping_does_not_have_a_fixed_qubit_limit() {
        let mut mapping = QubitMapping::new();

        let large_logical = usize::MAX - 1;
        let large_physical = usize::MAX - 2;

        mapping
            .insert(
                logical(large_logical),
                physical(large_physical),
            )
            .unwrap();

        assert_eq!(
            mapping.physical_for(logical(large_logical)),
            Some(physical(large_physical))
        );
    }

    #[test]
    fn qids_are_distinct_types() {
        let logical = QubitId::new(7);
        let physical = PhysicalQubitId::new(7);

        assert_eq!(logical.index(), physical.index());
        assert_ne!(
            QubitRef::Logical(logical),
            QubitRef::Physical(physical)
        );
    }

    #[test]
    fn mapping_entry_display_is_stable() {
        let entry = MappingEntry::new(logical(3), physical(17));

        assert_eq!(entry.to_string(), "q3 -> p17");
    }
}