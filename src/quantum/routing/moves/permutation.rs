//! Zamani Quantum Routing — Physical Qubit Permutations
//!
//! Production-grade representation and application of physical-qubit
//! permutations used by the routing subsystem.
//!
//! # Responsibility
//!
//! This module represents a permutation of physical qubit locations:
//!
//! ```text
//! physical source ─────────► physical destination
//! ```
//!
//! A permutation describes how quantum states/logical-qubit occupants move
//! between physical locations. It does NOT describe how those movements are
//! implemented as hardware gates.
//!
//! For example:
//!
//! ```text
//! q0 -> p0
//! q1 -> p1
//! q2 -> p2
//!
//! permutation:
//! p0 -> p1
//! p1 -> p2
//! p2 -> p0
//!
//! result:
//! q0 -> p1
//! q1 -> p2
//! q2 -> p0
//! ```
//!
//! # Architectural boundary
//!
//! This file deliberately does NOT:
//!
//! - parse Zamani source;
//! - parse OpenQASM;
//! - inspect compiler IR;
//! - choose a routing algorithm;
//! - choose a layout;
//! - validate hardware connectivity;
//! - generate native hardware gates;
//! - decompose SWAP;
//! - schedule operations;
//! - generate pulses;
//! - access a QPU;
//! - perform calibration;
//! - perform simulation;
//! - perform QEC decoding.
//!
//! Those responsibilities belong to other subsystems.
//!
//! # Relationship with `swap.rs`
//!
//! `PhysicalPermutation` is the higher-level mathematical representation.
//!
//! ```text
//! PhysicalPermutation
//!         │
//!         ▼
//! canonical sequence of SwapMove-equivalent exchanges
//!         │
//!         ▼
//! QubitMapping::apply_swaps()
//!         │
//!         ▼
//! updated logical ↔ physical mapping
//! ```
//!
//! The permutation itself does not require the physical edges to be adjacent.
//! This is intentional.
//!
//! A permutation such as:
//!
//! ```text
//! p0 -> p5
//! ```
//!
//! can be mathematically valid even when p0 and p5 are not directly connected.
//! A routing algorithm may later realize the same permutation using a sequence
//! of legal adjacent SWAPs.
//!
//! Therefore topology validation belongs to the movement/routing layer, not
//! this mathematical permutation primitive.
//!
//! # Permutation convention
//!
//! Every entry has the form:
//!
//! ```text
//! source -> destination
//! ```
//!
//! and means:
//!
//! > The quantum state/logical occupant currently at `source` must end up at
//! > `destination`.
//!
//! The representation is a *partial explicit permutation*: positions not
//! mentioned are implicitly mapped to themselves.
//!
//! However, all explicitly mentioned sources and destinations must form the
//! same set. This guarantees that the explicit mapping is bijective.
//!
//! Example:
//!
//! ```text
//! [(p0, p1), (p1, p0)]
//! ```
//!
//! is valid.
//!
//! ```text
//! [(p0, p1), (p2, p1)]
//! ```
//!
//! is invalid because two sources target p1.
//!
//! ```text
//! [(p0, p1)]
//! ```
//!
//! is invalid because p0 is explicitly moved but p1 is not represented as a
//! source. A valid permutation must conserve the complete set of explicitly
//! affected locations.
//!
//! # Empty positions
//!
//! `QubitMapping` permits physical locations to be unoccupied. This module
//! therefore does not require every physical location to contain a logical
//! qubit.
//!
//! Applying a valid permutation to a partially occupied mapping is still
//! correct: occupants move according to the permutation and empty locations
//! remain empty.
//!
//! # Atomicity
//!
//! Applying a permutation to a mapping is transactional.
//!
//! If any internal operation fails:
//!
//! ```text
//! original mapping
//!       │
//!       ▼
//! permutation application
//!       │
//!       ├── success ──► committed mapping
//!       │
//!       └── failure ──► original mapping restored
//! ```
//!
//! `QubitMapping::apply_swaps()` already provides atomic rollback. This module
//! additionally validates the complete permutation and constructs its swap
//! decomposition before mutating the mapping.
//!
//! # Determinism
//!
//! All externally observable ordering is deterministic.
//!
//! In particular:
//!
//! - entries are normalized into deterministic order;
//! - cycles are discovered in sorted physical-qubit order;
//! - swap decomposition is deterministic;
//! - composition is deterministic;
//! - inverse is deterministic;
//! - serialization/debug-oriented accessors do not depend on `HashMap` order.
//!
//! This is required for reproducible routing, SABRE trials, benchmarks,
//! compiler CI, debugging, and route replay.
//!
//! # Complexity
//!
//! Let `n` be the number of explicitly affected physical locations.
//!
//! - construction: O(n log n);
//! - validation: O(n log n);
//! - lookup: O(log n) using sorted entries;
//! - inverse: O(n log n);
//! - composition: O(n log n);
//! - decomposition: O(n);
//! - mapping application: O(n);
//! - identity: O(1);
//! - number of moved locations: O(n).
//!
//! The implementation intentionally uses a sorted `Vec` rather than a
//! `HashMap`. Routing permutations are generally small relative to full
//! circuits, deterministic iteration is important, and this representation
//! avoids exposing hash-order semantics.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features.
//! No external dependencies.
//! No `unsafe`.
//!
//! # Integration contract
//!
//! This file depends only on:
//!
//! - `routing::types::PhysicalQubitId`;
//! - `routing::mapping::QubitMapping`.
//!
//! It does not depend on:
//!
//! - `router.rs`;
//! - `layout.rs`;
//! - routing algorithms;
//! - hardware providers;
//! - Quantum IR.
//!
//! Later components consume this type as follows:
//!
//! ```text
//! SABRE/lookahead
//!       │
//!       ▼
//! PhysicalPermutation
//!       │
//!       ├── inverse()
//!       ├── compose()
//!       ├── to_swaps()
//!       │
//!       ▼
//! QubitMapping
//! ```
//!
//! `swap.rs` remains responsible for checking whether individual exchanges
//! represented by the decomposition are legal on a particular topology.
//!
//! -----------------------------------------------------------------------------
//! Imports
//! -----------------------------------------------------------------------------

use crate::quantum::routing::mapping::{MappingError, QubitMapping};
use crate::quantum::routing::types::PhysicalQubitId;

use std::fmt;

// =============================================================================
// Permutation entry
// =============================================================================

/// One directed entry of a physical-qubit permutation.
///
/// The semantic meaning is:
///
/// ```text
/// state currently at `source`
///             │
///             ▼
///      destination
/// ```
///
/// The source and destination are physical locations, not logical qubits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PermutationEntry {
    source: PhysicalQubitId,
    destination: PhysicalQubitId,
}

impl PermutationEntry {
    /// Creates a permutation entry.
    ///
    /// `source == destination` is allowed at this low level because an identity
    /// entry is mathematically valid. `PhysicalPermutation::new()` removes
    /// identity entries from its canonical representation.
    #[must_use]
    pub const fn new(
        source: PhysicalQubitId,
        destination: PhysicalQubitId,
    ) -> Self {
        Self {
            source,
            destination,
        }
    }

    /// Returns the source physical location.
    #[must_use]
    pub const fn source(self) -> PhysicalQubitId {
        self.source
    }

    /// Returns the destination physical location.
    #[must_use]
    pub const fn destination(self) -> PhysicalQubitId {
        self.destination
    }

    /// Returns whether this entry is an identity mapping.
    #[must_use]
    pub const fn is_identity(self) -> bool {
        self.source == self.destination
    }

    /// Returns the inverse entry.
    #[must_use]
    pub const fn inverse(self) -> Self {
        Self {
            source: self.destination,
            destination: self.source,
        }
    }
}

impl fmt::Display for PermutationEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} -> {}",
            self.source,
            self.destination
        )
    }
}

// =============================================================================
// Permutation errors
// =============================================================================

/// Errors specific to physical permutation construction/manipulation.
///
/// These errors remain local to this file because a permutation can be used
/// independently of a routing invocation. Higher-level routing code can map
/// these failures into the canonical `RoutingError` model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermutationError {
    /// The same physical source appears more than once.
    DuplicateSource {
        source: PhysicalQubitId,
    },

    /// Multiple sources target the same physical destination.
    DuplicateDestination {
        destination: PhysicalQubitId,
    },

    /// The explicit source and destination domains are different.
    ///
    /// A valid explicit partial permutation must conserve the affected
    /// physical-location set.
    DomainMismatch {
        sources: Vec<PhysicalQubitId>,
        destinations: Vec<PhysicalQubitId>,
    },

    /// The permutation contains an invalid entry.
    InvalidEntry {
        source: PhysicalQubitId,
        destination: PhysicalQubitId,
    },

    /// Composition failed because an entry could not be resolved.
    CompositionFailed {
        physical: PhysicalQubitId,
    },

    /// A requested operation exceeded a caller-supplied bound.
    OperationLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// The permutation cannot be applied to the supplied mapping.
    MappingApplicationFailed {
        detail: String,
    },
}

impl fmt::Display for PermutationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateSource { source } => write!(
                formatter,
                "physical permutation contains duplicate source {source}"
            ),

            Self::DuplicateDestination { destination } => write!(
                formatter,
                "physical permutation contains duplicate destination {destination}"
            ),

            Self::DomainMismatch {
                sources,
                destinations,
            } => write!(
                formatter,
                "physical permutation source/destination domains differ: \
                 sources={sources:?}, destinations={destinations:?}"
            ),

            Self::InvalidEntry {
                source,
                destination,
            } => write!(
                formatter,
                "invalid physical permutation entry {source} -> {destination}"
            ),

            Self::CompositionFailed { physical } => write!(
                formatter,
                "failed to compose physical permutation at {physical}"
            ),

            Self::OperationLimitExceeded {
                requested,
                maximum,
            } => write!(
                formatter,
                "physical permutation contains {requested} operations, \
                 exceeding maximum {maximum}"
            ),

            Self::MappingApplicationFailed { detail } => write!(
                formatter,
                "failed to apply physical permutation to mapping: {detail}"
            ),
        }
    }
}

impl std::error::Error for PermutationError {}

impl From<MappingError> for PermutationError {
    fn from(error: MappingError) -> Self {
        Self::MappingApplicationFailed {
            detail: error.to_string(),
        }
    }
}

// =============================================================================
// Physical permutation
// =============================================================================

/// A deterministic partial permutation of physical qubit locations.
///
/// An entry:
///
/// ```text
/// p0 -> p1
/// ```
///
/// means that the quantum state currently occupying p0 must finish at p1.
///
/// Physical locations not explicitly present are fixed points.
///
/// # Canonical representation
///
/// The internal representation:
///
/// - contains no identity entries;
/// - contains no duplicate sources;
/// - contains no duplicate destinations;
/// - has identical source and destination domains;
/// - is sorted by source;
/// - contains only moved locations.
///
/// Therefore two semantically equivalent permutations have the same canonical
/// representation.
///
/// # Example
///
/// ```text
/// p0 -> p1
/// p1 -> p2
/// p2 -> p0
/// ```
///
/// represents a three-location cycle.
///
/// Its inverse is:
///
/// ```text
/// p0 -> p2
/// p2 -> p1
/// p1 -> p0
/// ```
///
/// # Important semantic rule
///
/// This type represents *state movement*, not logical identity mutation.
///
/// If:
///
/// ```text
/// q0 -> p0
/// q1 -> p1
/// ```
///
/// and the permutation contains:
///
/// ```text
/// p0 -> p1
/// p1 -> p0
/// ```
///
/// the result is:
///
/// ```text
/// q0 -> p1
/// q1 -> p0
/// ```
///
/// The logical identities q0 and q1 remain unchanged.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicalPermutation {
    entries: Vec<PermutationEntry>,
}

impl PhysicalPermutation {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates the identity permutation.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Creates a canonical physical permutation.
    ///
    /// Identity entries are discarded.
    ///
    /// All non-identity sources must be unique.
    ///
    /// All non-identity destinations must be unique.
    ///
    /// The set of explicit sources must equal the set of explicit
    /// destinations.
    pub fn new<I>(entries: I) -> Result<Self, PermutationError>
    where
        I: IntoIterator<Item = PermutationEntry>,
    {
        let mut normalized: Vec<PermutationEntry> = entries
            .into_iter()
            .filter(|entry| !entry.is_identity())
            .collect();

        normalized.sort_unstable_by_key(|entry| {
            (entry.source(), entry.destination())
        });

        for pair in normalized.windows(2) {
            if let [previous, current] = pair {
                if previous.source() == current.source() {
                    return Err(PermutationError::DuplicateSource {
                        source: current.source(),
                    });
                }
            }
        }

        let mut destinations = normalized
            .iter()
            .map(|entry| entry.destination())
            .collect::<Vec<_>>();

        destinations.sort_unstable();

        for pair in destinations.windows(2) {
            if let [previous, current] = pair {
                if previous == current {
                    return Err(PermutationError::DuplicateDestination {
                        destination: *current,
                    });
                }
            }
        }

        let mut sources = normalized
            .iter()
            .map(|entry| entry.source())
            .collect::<Vec<_>>();

        sources.sort_unstable();

        if sources != destinations {
            return Err(PermutationError::DomainMismatch {
                sources,
                destinations,
            });
        }

        Ok(Self {
            entries: normalized,
        })
    }

    /// Creates a permutation from `(source, destination)` pairs.
    pub fn from_pairs<I>(
        pairs: I,
    ) -> Result<Self, PermutationError>
    where
        I: IntoIterator<Item = (PhysicalQubitId, PhysicalQubitId)>,
    {
        Self::new(
            pairs
                .into_iter()
                .map(|(source, destination)| {
                    PermutationEntry::new(source, destination)
                }),
        )
    }

    /// Creates a permutation from physical-qubit indices.
    ///
    /// Each tuple is interpreted as:
    ///
    /// ```text
    /// source_index -> destination_index
    /// ```
    pub fn from_indices<I>(
        pairs: I,
    ) -> Result<Self, PermutationError>
    where
        I: IntoIterator<Item = (usize, usize)>,
    {
        Self::from_pairs(pairs.into_iter().map(|(source, destination)| {
            (
                PhysicalQubitId::new(source),
                PhysicalQubitId::new(destination),
            )
        }))
    }

    /// Creates a permutation from a complete positional image.
    ///
    /// `image[i] = j` means:
    ///
    /// ```text
    /// physical i -> physical j
    /// ```
    ///
    /// The vector represents all positions from `0..image.len()`.
    ///
    /// The image must itself be a bijection over that range.
    pub fn from_image(
        image: &[usize],
    ) -> Result<Self, PermutationError> {
        let mut entries = Vec::with_capacity(image.len());

        for (source, &destination) in image.iter().enumerate() {
            if destination >= image.len() {
                return Err(PermutationError::InvalidEntry {
                    source: PhysicalQubitId::new(source),
                    destination: PhysicalQubitId::new(destination),
                });
            }

            entries.push(PermutationEntry::new(
                PhysicalQubitId::new(source),
                PhysicalQubitId::new(destination),
            ));
        }

        Self::new(entries)
    }

    /// Creates the transposition exchanging two physical locations.
    ///
    /// This is the mathematical permutation represented by one physical SWAP.
    pub fn transposition(
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> Result<Self, PermutationError> {
        if a == b {
            return Ok(Self::identity());
        }

        Self::new([
            PermutationEntry::new(a, b),
            PermutationEntry::new(b, a),
        ])
    }

    /// Creates a permutation from a cycle.
    ///
    /// For:
    ///
    /// ```text
    /// [p0, p1, p2]
    /// ```
    ///
    /// the resulting permutation is:
    ///
    /// ```text
    /// p0 -> p1
    /// p1 -> p2
    /// p2 -> p0
    /// ```
    ///
    /// A cycle with fewer than two elements is the identity.
    pub fn from_cycle(
        cycle: &[PhysicalQubitId],
    ) -> Result<Self, PermutationError> {
        if cycle.len() < 2 {
            return Ok(Self::identity());
        }

        let mut entries = Vec::with_capacity(cycle.len());

        for index in 0..cycle.len() {
            let source = cycle[index];

            let destination = cycle[(index + 1) % cycle.len()];

            entries.push(PermutationEntry::new(
                source,
                destination,
            ));
        }

        Self::new(entries)
    }

    // =========================================================================
    // Accessors
    // =========================================================================

    /// Returns the number of explicitly moved physical locations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether this permutation is the identity.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns all canonical entries.
    #[must_use]
    pub fn entries(&self) -> &[PermutationEntry] {
        &self.entries
    }

    /// Returns all affected physical locations in deterministic order.
    #[must_use]
    pub fn affected_qubits(&self) -> Vec<PhysicalQubitId> {
        self.entries
            .iter()
            .map(|entry| entry.source())
            .collect()
    }

    /// Returns whether a physical location is affected by this permutation.
    #[must_use]
    pub fn contains(&self, physical: PhysicalQubitId) -> bool {
        self.entries
            .binary_search_by_key(&physical, |entry| entry.source())
            .is_ok()
    }

    /// Applies the permutation to one physical location.
    ///
    /// Locations not explicitly affected are fixed points.
    #[must_use]
    pub fn apply_to_physical(
        &self,
        physical: PhysicalQubitId,
    ) -> PhysicalQubitId {
        match self
            .entries
            .binary_search_by_key(&physical, |entry| entry.source())
        {
            Ok(index) => self.entries[index].destination(),
            Err(_) => physical,
        }
    }

    /// Applies the permutation to a physical index.
    #[must_use]
    pub fn apply_to_index(&self, physical: usize) -> usize {
        self.apply_to_physical(PhysicalQubitId::new(physical))
            .index()
    }

    /// Returns the source corresponding to a destination.
    ///
    /// Because the permutation is bijective, this is the inverse mapping.
    #[must_use]
    pub fn source_for_destination(
        &self,
        destination: PhysicalQubitId,
    ) -> PhysicalQubitId {
        for entry in &self.entries {
            if entry.destination() == destination {
                return entry.source();
            }
        }

        destination
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Validates the canonical invariants.
    ///
    /// This method is intentionally independent of topology.
    pub fn validate(&self) -> Result<(), PermutationError> {
        let mut sources = Vec::with_capacity(self.entries.len());
        let mut destinations = Vec::with_capacity(self.entries.len());

        for entry in &self.entries {
            if entry.is_identity() {
                return Err(PermutationError::InvalidEntry {
                    source: entry.source(),
                    destination: entry.destination(),
                });
            }

            sources.push(entry.source());
            destinations.push(entry.destination());
        }

        for pair in self.entries.windows(2) {
            if let [previous, current] = pair {
                if previous.source() >= current.source() {
                    return Err(PermutationError::InvalidEntry {
                        source: current.source(),
                        destination: current.destination(),
                    });
                }
            }
        }

        destinations.sort_unstable();

        for pair in destinations.windows(2) {
            if let [previous, current] = pair {
                if previous == current {
                    return Err(PermutationError::DuplicateDestination {
                        destination: *current,
                    });
                }
            }
        }

        sources.sort_unstable();

        if sources != destinations {
            return Err(PermutationError::DomainMismatch {
                sources,
                destinations,
            });
        }

        Ok(())
    }

    // =========================================================================
    // Inverse
    // =========================================================================

    /// Returns the inverse permutation.
    ///
    /// For:
    ///
    /// ```text
    /// p0 -> p1
    /// p1 -> p2
    /// p2 -> p0
    /// ```
    ///
    /// returns:
    ///
    /// ```text
    /// p0 -> p2
    /// p2 -> p1
    /// p1 -> p0
    /// ```
    #[must_use]
    pub fn inverse(&self) -> Self {
        let mut entries: Vec<_> = self
            .entries
            .iter()
            .map(|entry| entry.inverse())
            .collect();

        entries.sort_unstable_by_key(|entry| {
            (entry.source(), entry.destination())
        });

        Self { entries }
    }

    // =========================================================================
    // Composition
    // =========================================================================

    /// Composes this permutation with another permutation.
    ///
    /// Composition follows execution order:
    ///
    /// ```text
    /// self
    ///   │
    ///   ▼
    /// other
    /// ```
    ///
    /// Therefore:
    ///
    /// ```text
    /// compose(other)
    /// ```
    ///
    /// represents:
    ///
    /// ```text
    /// result(x) = other(self(x))
    /// ```
    ///
    /// Example:
    ///
    /// ```text
    /// self:
    /// p0 -> p1
    /// p1 -> p0
    ///
    /// other:
    /// p0 -> p2
    /// p2 -> p0
    ///
    /// result:
    /// p0 -> p2
    /// p2 -> p1
    /// p1 -> p0
    /// ```
    pub fn compose(
        &self,
        other: &Self,
    ) -> Result<Self, PermutationError> {
        self.validate()?;
        other.validate()?;

        let mut domain = self.affected_qubits();
        domain.extend(other.affected_qubits());

        domain.sort_unstable();
        domain.dedup();

        let entries = domain
            .iter()
            .copied()
            .filter_map(|physical| {
                let after_self = self.apply_to_physical(physical);
                let after_other =
                    other.apply_to_physical(after_self);

                if after_other == physical {
                    None
                } else {
                    Some(PermutationEntry::new(
                        physical,
                        after_other,
                    ))
                }
            })
            .collect::<Vec<_>>();

        Self::new(entries)
    }

    /// Returns whether this permutation commutes with another permutation.
    ///
    /// This is useful for independent routing-region transformations.
    pub fn commutes_with(
        &self,
        other: &Self,
    ) -> Result<bool, PermutationError> {
        let lhs = self.compose(other)?;
        let rhs = other.compose(self)?;

        Ok(lhs == rhs)
    }

    // =========================================================================
    // Cycle analysis
    // =========================================================================

    /// Returns the non-trivial cycles of the permutation.
    ///
    /// Each cycle contains physical locations in execution direction.
    ///
    /// Example:
    ///
    /// ```text
    /// p0 -> p1
    /// p1 -> p2
    /// p2 -> p0
    /// ```
    ///
    /// produces:
    ///
    /// ```text
    /// [p0, p1, p2]
    /// ```
    #[must_use]
    pub fn cycles(&self) -> Vec<Vec<PhysicalQubitId>> {
        let mut cycles = Vec::new();
        let mut visited = Vec::with_capacity(self.entries.len());

        for entry in &self.entries {
            let start = entry.source();

            if visited.contains(&start) {
                continue;
            }

            let mut cycle = Vec::new();
            let mut current = start;

            loop {
                if visited.contains(&current) {
                    break;
                }

                visited.push(current);
                cycle.push(current);

                let next = self.apply_to_physical(current);

                if next == start {
                    break;
                }

                current = next;
            }

            if cycle.len() >= 2 {
                cycles.push(cycle);
            }
        }

        cycles
    }

    /// Returns the number of non-trivial cycles.
    #[must_use]
    pub fn cycle_count(&self) -> usize {
        self.cycles().len()
    }

    // =========================================================================
    // SWAP decomposition
    // =========================================================================

    /// Decomposes the permutation into mathematical transpositions.
    ///
    /// The returned pairs are physical-location exchanges:
    ///
    /// ```text
    /// (a, b)
    /// ```
    ///
    /// means exchange the occupants of a and b.
    ///
    /// The decomposition is deterministic.
    ///
    /// For a cycle:
    ///
    /// ```text
    /// [p0, p1, p2, p3]
    /// ```
    ///
    /// the returned exchanges are:
    ///
    /// ```text
    /// (p0, p3)
    /// (p0, p2)
    /// (p0, p1)
    /// ```
    ///
    /// Applying them in that order produces:
    ///
    /// ```text
    /// p0 -> p1
    /// p1 -> p2
    /// p2 -> p3
    /// p3 -> p0
    /// ```
    ///
    /// This is a mathematical decomposition. It does not establish that each
    /// pair is physically adjacent. `swap.rs`/the routing algorithm must
    /// validate or realize those exchanges on the target topology.
    #[must_use]
    pub fn to_swaps(
        &self,
    ) -> Vec<(PhysicalQubitId, PhysicalQubitId)> {
        let mut swaps = Vec::new();

        for cycle in self.cycles() {
            if cycle.len() < 2 {
                continue;
            }

            let pivot = cycle[0];

            for index in (1..cycle.len()).rev() {
                swaps.push((pivot, cycle[index]));
            }
        }

        swaps
    }

    /// Returns the minimum number of arbitrary transpositions required by the
    /// cycle structure.
    ///
    /// For a cycle of length `k`, exactly `k - 1` transpositions are required.
    #[must_use]
    pub fn minimum_transposition_count(&self) -> usize {
        self.cycles()
            .iter()
            .map(|cycle| cycle.len().saturating_sub(1))
            .sum()
    }

    /// Returns the parity of the permutation.
    ///
    /// `true` means odd permutation.
    ///
    /// `false` means even permutation.
    #[must_use]
    pub fn is_odd(&self) -> bool {
        self.minimum_transposition_count() % 2 == 1
    }

    // =========================================================================
    // Mapping integration
    // =========================================================================

    /// Applies this permutation to a logical/physical mapping.
    ///
    /// The operation is atomic.
    ///
    /// The permutation is first validated completely. It is then converted to
    /// a deterministic sequence of physical exchanges and delegated to
    /// `QubitMapping::apply_swaps()`.
    ///
    /// No topology validation occurs here.
    ///
    /// This is intentional: the permutation is a mathematical state movement
    /// and may later be lowered into topology-aware movement operations.
    pub fn apply_to_mapping(
        &self,
        mapping: &mut QubitMapping,
    ) -> Result<(), PermutationError> {
        self.validate()?;

        let swaps = self.to_swaps();

        mapping
            .apply_swaps(swaps)
            .map_err(PermutationError::from)?;

        debug_assert!(
            mapping.validate().is_ok(),
            "QubitMapping invariant violated after PhysicalPermutation::apply_to_mapping"
        );

        Ok(())
    }

    /// Applies the permutation to a mapping with an explicit transposition
    /// limit.
    ///
    /// The mapping is unchanged if the limit is exceeded or application fails.
    pub fn apply_to_mapping_with_limit(
        &self,
        mapping: &mut QubitMapping,
        maximum_transpositions: usize,
    ) -> Result<usize, PermutationError> {
        self.validate()?;

        let swaps = self.to_swaps();

        if swaps.len() > maximum_transpositions {
            return Err(PermutationError::OperationLimitExceeded {
                requested: swaps.len(),
                maximum: maximum_transpositions,
            });
        }

        mapping
            .apply_swaps(swaps.clone())
            .map_err(PermutationError::from)?;

        debug_assert!(
            mapping.validate().is_ok(),
            "QubitMapping invariant violated after bounded permutation application"
        );

        Ok(swaps.len())
    }

    /// Returns the mapping that results from applying this permutation without
    /// mutating the supplied mapping.
    ///
    /// This is useful for SABRE/lookahead candidate evaluation.
    pub fn preview_mapping(
        &self,
        mapping: &QubitMapping,
    ) -> Result<QubitMapping, PermutationError> {
        let mut preview = mapping.clone();

        self.apply_to_mapping(&mut preview)?;

        Ok(preview)
    }

    // =========================================================================
    // Mapping comparison
    // =========================================================================

    /// Constructs the permutation required to transform `from` into `to`.
    ///
    /// Both mappings must contain exactly the same logical qubits.
    ///
    /// Example:
    ///
    /// ```text
    /// from:
    /// q0 -> p0
    /// q1 -> p1
    ///
    /// to:
    /// q0 -> p1
    /// q1 -> p0
    ///
    /// result:
    /// p0 -> p1
    /// p1 -> p0
    /// ```
    pub fn between(
        from: &QubitMapping,
        to: &QubitMapping,
    ) -> Result<Self, PermutationError> {
        let from_entries = from.logical_to_physical();
        let to_entries = to.logical_to_physical();

        if from_entries.len() != to_entries.len() {
            return Err(PermutationError::MappingApplicationFailed {
                detail: format!(
                    "mapping sizes differ: from={}, to={}",
                    from_entries.len(),
                    to_entries.len()
                ),
            });
        }

        let mut entries = Vec::new();

        for (logical, source) in from_entries {
            let destination = to
                .physical_of(logical)
                .ok_or_else(|| {
                    PermutationError::MappingApplicationFailed {
                        detail: format!(
                            "logical qubit {logical} exists in the source \
                             mapping but not in the destination mapping"
                        ),
                    }
                })?;

            if source != destination {
                entries.push(PermutationEntry::new(
                    source,
                    destination,
                ));
            }
        }

        Self::new(entries)
    }

    /// Returns whether this permutation transforms one mapping into another.
    pub fn transforms(
        &self,
        from: &QubitMapping,
        to: &QubitMapping,
    ) -> Result<bool, PermutationError> {
        let mut candidate = from.clone();

        self.apply_to_mapping(&mut candidate)?;

        Ok(candidate == *to)
    }

    // =========================================================================
    // Formatting
    // =========================================================================

    /// Returns a deterministic human-readable representation.
    #[must_use]
    pub fn display_string(&self) -> String {
        if self.is_identity() {
            return "identity".to_string();
        }

        self.entries
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl Default for PhysicalPermutation {
    fn default() -> Self {
        Self::identity()
    }
}

impl fmt::Display for PhysicalPermutation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.display_string())
    }
}

// =============================================================================
// Iterator support
// =============================================================================

impl<'a> IntoIterator for &'a PhysicalPermutation {
    type Item = &'a PermutationEntry;
    type IntoIter = std::slice::Iter<'a, PermutationEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

impl IntoIterator for PhysicalPermutation {
    type Item = PermutationEntry;
    type IntoIter = std::vec::IntoIter<PermutationEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

// =============================================================================
// From conversions
// =============================================================================

impl TryFrom<Vec<(PhysicalQubitId, PhysicalQubitId)>>
    for PhysicalPermutation
{
    type Error = PermutationError;

    fn try_from(
        pairs: Vec<(PhysicalQubitId, PhysicalQubitId)>,
    ) -> Result<Self, Self::Error> {
        Self::from_pairs(pairs)
    }
}

impl TryFrom<Vec<(usize, usize)>> for PhysicalPermutation {
    type Error = PermutationError;

    fn try_from(
        pairs: Vec<(usize, usize)>,
    ) -> Result<Self, Self::Error> {
        Self::from_indices(pairs)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn p(index: usize) -> PhysicalQubitId {
        PhysicalQubitId::new(index)
    }

    fn mapping_3() -> QubitMapping {
        QubitMapping::from_assignments([
            (crate::quantum::routing::types::LogicalQubitId::new(0), p(0)),
            (crate::quantum::routing::types::LogicalQubitId::new(1), p(1)),
            (crate::quantum::routing::types::LogicalQubitId::new(2), p(2)),
        ])
        .expect("test mapping must be valid")
    }

    #[test]
    fn identity_is_empty_and_valid() {
        let permutation = PhysicalPermutation::identity();

        assert!(permutation.is_identity());
        assert_eq!(permutation.len(), 0);
        assert!(permutation.validate().is_ok());
    }

    #[test]
    fn identity_leaves_physical_qubit_unchanged() {
        let permutation = PhysicalPermutation::identity();

        assert_eq!(permutation.apply_to_physical(p(42)), p(42));
    }

    #[test]
    fn removes_identity_entries() {
        let permutation = PhysicalPermutation::new([
            PermutationEntry::new(p(0), p(0)),
            PermutationEntry::new(p(1), p(2)),
            PermutationEntry::new(p(2), p(1)),
        ])
        .expect("permutation must be valid");

        assert_eq!(permutation.len(), 2);
        assert!(!permutation.contains(p(0)));
    }

    #[test]
    fn rejects_duplicate_sources() {
        let result = PhysicalPermutation::new([
            PermutationEntry::new(p(0), p(1)),
            PermutationEntry::new(p(0), p(2)),
        ]);

        assert!(matches!(
            result,
            Err(PermutationError::DuplicateSource {
                source
            }) if source == p(0)
        ));
    }

    #[test]
    fn rejects_duplicate_destinations() {
        let result = PhysicalPermutation::new([
            PermutationEntry::new(p(0), p(1)),
            PermutationEntry::new(p(2), p(1)),
        ]);

        assert!(matches!(
            result,
            Err(PermutationError::DuplicateDestination {
                destination
            }) if destination == p(1)
        ));
    }

    #[test]
    fn rejects_domain_mismatch() {
        let result = PhysicalPermutation::new([
            PermutationEntry::new(p(0), p(1)),
        ]);

        assert!(matches!(
            result,
            Err(PermutationError::DomainMismatch { .. })
        ));
    }

    #[test]
    fn transposition_is_self_inverse() {
        let permutation =
            PhysicalPermutation::transposition(p(0), p(1))
                .expect("transposition must be valid");

        assert_eq!(
            permutation.inverse(),
            permutation
        );
    }

    #[test]
    fn cycle_maps_in_forward_direction() {
        let permutation =
            PhysicalPermutation::from_cycle(&[p(0), p(1), p(2)])
                .expect("cycle must be valid");

        assert_eq!(permutation.apply_to_physical(p(0)), p(1));
        assert_eq!(permutation.apply_to_physical(p(1)), p(2));
        assert_eq!(permutation.apply_to_physical(p(2)), p(0));
    }

    #[test]
    fn inverse_reverses_cycle() {
        let permutation =
            PhysicalPermutation::from_cycle(&[p(0), p(1), p(2)])
                .expect("cycle must be valid");

        let inverse = permutation.inverse();

        assert_eq!(inverse.apply_to_physical(p(0)), p(2));
        assert_eq!(inverse.apply_to_physical(p(2)), p(1));
        assert_eq!(inverse.apply_to_physical(p(1)), p(0));
    }

    #[test]
    fn composition_follows_execution_order() {
        let first =
            PhysicalPermutation::transposition(p(0), p(1))
                .expect("first permutation must be valid");

        let second =
            PhysicalPermutation::transposition(p(1), p(2))
                .expect("second permutation must be valid");

        let composed = first
            .compose(&second)
            .expect("composition must succeed");

        assert_eq!(
            composed.apply_to_physical(p(0)),
            p(2)
        );

        assert_eq!(
            composed.apply_to_physical(p(2)),
            p(1)
        );

        assert_eq!(
            composed.apply_to_physical(p(1)),
            p(0)
        );
    }

    #[test]
    fn composition_with_inverse_is_identity() {
        let permutation =
            PhysicalPermutation::from_cycle(&[p(0), p(1), p(2)])
                .expect("cycle must be valid");

        let inverse = permutation.inverse();

        let composed = permutation
            .compose(&inverse)
            .expect("composition must succeed");

        assert!(composed.is_identity());
    }

    #[test]
    fn inverse_composed_with_permutation_is_identity() {
        let permutation =
            PhysicalPermutation::from_cycle(&[p(0), p(1), p(2)])
                .expect("cycle must be valid");

        let inverse = permutation.inverse();

        let composed = inverse
            .compose(&permutation)
            .expect("composition must succeed");

        assert!(composed.is_identity());
    }

    #[test]
    fn cycles_are_deterministic() {
        let permutation =
            PhysicalPermutation::from_cycle(&[p(0), p(1), p(2)])
                .expect("cycle must be valid");

        assert_eq!(
            permutation.cycles(),
            vec![vec![p(0), p(1), p(2)]]
        );
    }

    #[test]
    fn decomposes_three_cycle_into_two_swaps() {
        let permutation =
            PhysicalPermutation::from_cycle(&[p(0), p(1), p(2)])
                .expect("cycle must be valid");

        let swaps = permutation.to_swaps();

        assert_eq!(
            swaps,
            vec![(p(0), p(2)), (p(0), p(1))]
        );
    }

    #[test]
    fn decomposed_swaps_have_minimum_count() {
        let permutation =
            PhysicalPermutation::from_cycle(&[
                p(0),
                p(1),
                p(2),
                p(3),
            ])
            .expect("cycle must be valid");

        assert_eq!(
            permutation.to_swaps().len(),
            3
        );

        assert_eq!(
            permutation.minimum_transposition_count(),
            3
        );
    }

    #[test]
    fn applies_transposition_to_mapping() {
        let mut mapping = mapping_3();

        let permutation =
            PhysicalPermutation::transposition(p(0), p(1))
                .expect("transposition must be valid");

        permutation
            .apply_to_mapping(&mut mapping)
            .expect("mapping application must succeed");

        assert_eq!(
            mapping.physical_of(
                crate::quantum::routing::types::LogicalQubitId::new(0)
            ),
            Some(p(1))
        );

        assert_eq!(
            mapping.physical_of(
                crate::quantum::routing::types::LogicalQubitId::new(1)
            ),
            Some(p(0))
        );

        assert_eq!(
            mapping.physical_of(
                crate::quantum::routing::types::LogicalQubitId::new(2)
            ),
            Some(p(2))
        );
    }

    #[test]
    fn applies_three_cycle_to_mapping() {
        let mut mapping = mapping_3();

        let permutation =
            PhysicalPermutation::from_cycle(&[p(0), p(1), p(2)])
                .expect("cycle must be valid");

        permutation
            .apply_to_mapping(&mut mapping)
            .expect("mapping application must succeed");

        assert_eq!(
            mapping.physical_of(
                crate::quantum::routing::types::LogicalQubitId::new(0)
            ),
            Some(p(1))
        );

        assert_eq!(
            mapping.physical_of(
                crate::quantum::routing::types::LogicalQubitId::new(1)
            ),
            Some(p(2))
        );

        assert_eq!(
            mapping.physical_of(
                crate::quantum::routing::types::LogicalQubitId::new(2)
            ),
            Some(p(0))
        );
    }

    #[test]
    fn preview_does_not_mutate_original() {
        let mapping = mapping_3();

        let permutation =
            PhysicalPermutation::transposition(p(0), p(1))
                .expect("transposition must be valid");

        let preview = permutation
            .preview_mapping(&mapping)
            .expect("preview must succeed");

        assert_ne!(preview, mapping);

        assert_eq!(
            mapping.physical_of(
                crate::quantum::routing::types::LogicalQubitId::new(0)
            ),
            Some(p(0))
        );
    }

    #[test]
    fn between_two_mappings_produces_correct_permutation() {
        let from = mapping_3();

        let mut to = mapping_3();

        to.swap_physical(p(0), p(1))
            .expect("mapping swap must succeed");

        let permutation =
            PhysicalPermutation::between(&from, &to)
                .expect("mapping difference must be representable");

        assert_eq!(
            permutation.apply_to_physical(p(0)),
            p(1)
        );

        assert_eq!(
            permutation.apply_to_physical(p(1)),
            p(0)
        );
    }

    #[test]
    fn transforms_detects_correct_target() {
        let from = mapping_3();

        let mut to = mapping_3();

        to.swap_physical(p(0), p(1))
            .expect("mapping swap must succeed");

        let permutation =
            PhysicalPermutation::between(&from, &to)
                .expect("permutation construction must succeed");

        assert!(
            permutation
                .transforms(&from, &to)
                .expect("transformation check must succeed")
        );
    }

    #[test]
    fn parity_of_transposition_is_odd() {
        let permutation =
            PhysicalPermutation::transposition(p(0), p(1))
                .expect("transposition must be valid");

        assert!(permutation.is_odd());
    }

    #[test]
    fn parity_of_three_cycle_is_even() {
        let permutation =
            PhysicalPermutation::from_cycle(&[p(0), p(1), p(2)])
                .expect("cycle must be valid");

        assert!(!permutation.is_odd());
    }

    #[test]
    fn unrelated_physical_location_is_fixed() {
        let permutation =
            PhysicalPermutation::transposition(p(0), p(1))
                .expect("transposition must be valid");

        assert_eq!(
            permutation.apply_to_physical(p(99)),
            p(99)
        );
    }

    #[test]
    fn conversion_from_indices_is_supported() {
        let permutation =
            PhysicalPermutation::from_indices([
                (0, 1),
                (1, 0),
            ])
            .expect("index permutation must be valid");

        assert_eq!(
            permutation.apply_to_index(0),
            1
        );

        assert_eq!(
            permutation.apply_to_index(1),
            0
        );
    }

    #[test]
    fn full_image_identity_is_identity() {
        let permutation =
            PhysicalPermutation::from_image(&[0, 1, 2])
                .expect("identity image must be valid");

        assert!(permutation.is_identity());
    }

    #[test]
    fn rejects_out_of_range_image() {
        let result =
            PhysicalPermutation::from_image(&[1, 0, 4]);

        assert!(matches!(
            result,
            Err(PermutationError::InvalidEntry { .. })
        ));
    }

    #[test]
    fn empty_cycle_is_identity() {
        let permutation =
            PhysicalPermutation::from_cycle(&[]).expect("empty cycle is valid");

        assert!(permutation.is_identity());
    }

    #[test]
    fn one_element_cycle_is_identity() {
        let permutation =
            PhysicalPermutation::from_cycle(&[p(0)])
                .expect("single-element cycle is valid");

        assert!(permutation.is_identity());
    }

    #[test]
    fn display_is_deterministic() {
        let permutation =
            PhysicalPermutation::transposition(p(0), p(1))
                .expect("transposition must be valid");

        assert_eq!(
            permutation.to_string(),
            "p0 -> p1, p1 -> p0"
        );
    }

    #[test]
    fn canonical_order_is_stable() {
        let permutation =
            PhysicalPermutation::from_pairs([
                (p(2), p(0)),
                (p(0), p(2)),
            ])
            .expect("permutation must be valid");

        assert_eq!(
            permutation.entries()[0].source(),
            p(0)
        );

        assert_eq!(
            permutation.entries()[1].source(),
            p(2)
        );
    }

    #[test]
    fn commutation_detects_disjoint_swaps() {
        let first =
            PhysicalPermutation::transposition(p(0), p(1))
                .expect("first swap must be valid");

        let second =
            PhysicalPermutation::transposition(p(2), p(3))
                .expect("second swap must be valid");

        assert!(
            first
                .commutes_with(&second)
                .expect("commutation check must succeed")
        );
    }
}