//! Zamani Quantum Resilience — Logical Resource State
//!
//! This module owns the resilience-layer execution state associated with
//! canonical logical qubits.
//!
//! # Architectural responsibility
//!
//! This module answers:
//!
//! > "What resilience/execution state is currently associated with each
//! > logical qubit in this resilience context?"
//!
//! The canonical logical-qubit identity is owned by:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! This module therefore uses that type directly and MUST NOT introduce
//! another logical-qubit identifier type.
//!
//! # Ownership boundaries
//!
//! This module owns:
//!
//! - logical-qubit resilience lifecycle state;
//! - deterministic logical-qubit state storage;
//! - state transition validation;
//! - per-logical-qubit state generations;
//! - atomic multi-qubit state transitions;
//! - immutable snapshots of logical resilience state;
//! - bounded/unbounded-by-policy collection semantics.
//!
//! This module does NOT own:
//!
//! - physical-qubit identity;
//! - logical-to-physical placement;
//! - hardware calibration;
//! - hardware health;
//! - device topology;
//! - routing;
//! - scheduling;
//! - quantum amplitudes;
//! - wavefunctions;
//! - density matrices;
//! - simulation state;
//! - QEC decoding;
//! - syndrome processing;
//! - error-mitigation algorithms;
//! - recovery planning;
//! - recovery execution;
//! - resilience lifecycle orchestration.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Relationship to other resilience modules
//!
//! ```text
//!                 canonical Zamani IR
//!                         │
//!                         │ QubitId
//!                         ▼
//!             ┌───────────────────────┐
//!             │ resilience::state::   │
//!             │       logical         │
//!             └───────────┬───────────┘
//!                         │
//!             logical execution state
//!                         │
//!          ┌──────────────┼──────────────┐
//!          ▼              ▼              ▼
//!       planning       recovery       verification
//!          │              │              │
//!          └──────────────┼──────────────┘
//!                         ▼
//!                  routing / hardware
//! ```
//!
//! The logical state is therefore a resilience view, not a replacement for
//! canonical IR or hardware state.
//!
//! # Logical versus physical identity
//!
//! A logical qubit is deliberately represented by:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! Physical placement is intentionally absent from this file.
//!
//! For example, this module may know that:
//!
//!     QubitId(17) -> Recovering
//!
//! but it must NOT know that:
//!
//!     QubitId(17) -> PhysicalQubitId(42)
//!
//! That mapping belongs to routing/placement and may change during recovery.
//!
//! This separation is what allows a Zamani program to remain unchanged while
//! its physical realization changes.
//!
//! # State semantics
//!
//! The state represented here is resilience/execution bookkeeping.
//!
//! It is NOT a quantum-mechanical state.
//!
//! In particular:
//!
//! ```text
//! Ready       != |0>
//! Active      != a particular wavefunction
//! Suspended   != decoherence
//! Recovering  != QEC decoding
//! Verified    != mathematical proof of amplitudes
//! ```
//!
//! Simulation state belongs to the simulator.
//!
//! QEC state belongs to the QEC subsystem.
//!
//! Hardware health belongs to the hardware/resilience health models.
//!
//! # Write once, scale everywhere
//!
//! No fixed number of logical qubits is encoded here.
//!
//! There is no:
//!
//! - MAX_QUBITS;
//! - fixed-size array;
//! - provider-specific qubit number;
//! - topology assumption;
//! - architecture-specific identifier;
//! - retry count;
//! - hard-coded failure threshold.
//!
//! The collection grows according to the logical workload and available host
//! resources.
//!
//! "Infinity" is interpreted architecturally as the absence of an artificial
//! machine-size ceiling. Every concrete process remains bounded by available
//! memory, address space, execution policy, and target resources.
//!
//! # Determinism
//!
//! `BTreeMap` is used rather than `HashMap` so that iteration order is stable.
//!
//! Deterministic iteration is important for:
//!
//! - reproducible recovery planning;
//! - deterministic serialization;
//! - audit trails;
//! - testing;
//! - distributed replay;
//! - provenance.
//!
//! # Atomicity
//!
//! Single-qubit transitions are atomic with respect to this in-memory object.
//!
//! Multi-qubit transitions are transactional:
//!
//! - either every requested transition succeeds;
//! - or the state is restored exactly to its previous values.
//!
//! The implementation uses a compact rollback journal rather than cloning the
//! entire state map. This avoids making transaction cost proportional to the
//! total number of logical qubits.
//!
//! # Concurrency
//!
//! This type intentionally contains no internal mutex or runtime-specific
//! synchronization primitive.
//!
//! Ownership and synchronization belong to the resilience controller/runtime.
//!
//! The type is composed only of standard Rust containers and values and is
//! therefore suitable for use behind an external synchronization boundary.
//!
//! # Persistence and serialization
//!
//! This module owns semantic state only.
//!
//! Canonical wire encoding belongs to:
//!
//!     crate::quantum::resilience::serialization
//!
//! A serialized representation MUST preserve:
//!
//! - logical-qubit identity;
//! - lifecycle state;
//! - generation;
//! - schema/version information supplied by the serialization layer.
//!
//! # Security
//!
//! This module does not accept credentials, backend tokens, authentication
//! material, or authorization decisions.
//!
//! A caller that changes a logical-qubit state must already have passed through
//! the appropriate policy/authorization boundary.
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
//! - no unsafe code.
//!
//! # Integration contract
//!
//! Consumers:
//!
//! - `state::machine`
//! - `state::execution`
//! - `state::recovery`
//! - `planning`
//! - `adaptation`
//! - `recovery`
//! - `verification`
//! - `checkpoint`
//! - `telemetry`
//!
//! Providers/authorities:
//!
//! - `quantum::ir::qubit::QubitId` for identity;
//! - routing for physical placement;
//! - hardware for physical resource state;
//! - QEC for logical-error information;
//! - verification for acceptance;
//! - policy for permitted transitions.
//!
//! This file does not directly depend on those higher-level implementations.
//! That keeps the state model independently testable and prevents circular
//! dependencies.

// -----------------------------------------------------------------------------
// Safety contract
// -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::quantum::ir::qubit::QubitId;

// ============================================================================
// Logical resource state
// ============================================================================

/// Resilience/execution state of one logical qubit.
///
/// This state describes how resilience currently treats the logical resource.
/// It is not a quantum-mechanical state.
///
/// The transitions are intentionally conservative:
///
/// ```text
/// Uninitialized
///      │
///      ▼
///    Ready
///      │
///      ▼
///    Active
///   /     \
///  ▼       ▼
///Suspended Recovering
///  │          │
///  └────┬─────┘
///       ▼
///     Active
///
/// Active ─────────────► Verified
/// Active ─────────────► Quarantined
/// Suspended ──────────► Quarantined
/// Recovering ─────────► Quarantined
/// Verified ───────────► Active
///
/// Any non-retired state ──► Retired
/// ```
///
/// `Retired` is terminal.
///
/// `Quarantined` is intentionally not automatically returned to `Active`;
/// an explicit recovery/verification decision is required.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LogicalResourceState {
    /// The logical qubit is known to the state store but has not yet been
    /// admitted into active execution.
    Uninitialized,

    /// The logical qubit is available for execution.
    Ready,

    /// The logical qubit participates in active execution.
    Active,

    /// Execution involving the logical qubit is temporarily suspended.
    Suspended,

    /// The logical qubit requires an ongoing resilience/recovery operation.
    Recovering,

    /// The logical qubit has passed the relevant execution verification
    /// boundary.
    Verified,

    /// The logical qubit is isolated from ordinary execution pending an
    /// explicit policy decision.
    Quarantined,

    /// The logical qubit is permanently retired from this resilience context.
    Retired,
}

impl LogicalResourceState {
    /// Returns a stable machine-readable state name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Uninitialized => "uninitialized",
            Self::Ready => "ready",
            Self::Active => "active",
            Self::Suspended => "suspended",
            Self::Recovering => "recovering",
            Self::Verified => "verified",
            Self::Quarantined => "quarantined",
            Self::Retired => "retired",
        }
    }

    /// Returns a stable numeric representation suitable for deterministic
    /// serialization.
    ///
    /// These values are part of the semantic representation and therefore
    /// must not be changed casually.
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        match self {
            Self::Uninitialized => 0,
            Self::Ready => 1,
            Self::Active => 2,
            Self::Suspended => 3,
            Self::Recovering => 4,
            Self::Verified => 5,
            Self::Quarantined => 6,
            Self::Retired => 7,
        }
    }

    /// Converts the stable numeric representation back to a state.
    #[must_use]
    pub const fn from_ordinal(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Uninitialized),
            1 => Some(Self::Ready),
            2 => Some(Self::Active),
            3 => Some(Self::Suspended),
            4 => Some(Self::Recovering),
            5 => Some(Self::Verified),
            6 => Some(Self::Quarantined),
            7 => Some(Self::Retired),
            _ => None,
        }
    }

    /// Returns whether the state permits ordinary execution.
    #[must_use]
    pub const fn is_execution_ready(self) -> bool {
        matches!(self, Self::Ready | Self::Active | Self::Verified)
    }

    /// Returns whether the state permits active quantum execution.
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether the resource is undergoing recovery.
    #[must_use]
    pub const fn is_recovering(self) -> bool {
        matches!(self, Self::Recovering)
    }

    /// Returns whether the resource is isolated.
    #[must_use]
    pub const fn is_quarantined(self) -> bool {
        matches!(self, Self::Quarantined)
    }

    /// Returns whether the state is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Retired)
    }

    /// Returns whether the resource may be referenced by a new execution
    /// operation.
    ///
    /// `Verified` is considered reusable because verification of one execution
    /// does not permanently consume the logical resource.
    #[must_use]
    pub const fn accepts_execution(self) -> bool {
        matches!(self, Self::Ready | Self::Active | Self::Verified)
    }

    /// Returns whether this state requires an explicit resilience decision
    /// before ordinary execution may continue.
    #[must_use]
    pub const fn requires_intervention(self) -> bool {
        matches!(
            self,
            Self::Uninitialized
                | Self::Suspended
                | Self::Recovering
                | Self::Quarantined
        )
    }
}

impl Default for LogicalResourceState {
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl fmt::Display for LogicalResourceState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Transition
// ============================================================================

/// A legal logical-resource state transition.
///
/// The transition does not contain timestamps or telemetry. Those belong to
/// the resilience event/telemetry layers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LogicalStateTransition {
    from: LogicalResourceState,
    to: LogicalResourceState,
}

impl LogicalStateTransition {
    /// Creates and validates a state transition.
    pub const fn new(
        from: LogicalResourceState,
        to: LogicalResourceState,
    ) -> Result<Self, LogicalStateError> {
        if !is_valid_transition(from, to) {
            return Err(LogicalStateError::InvalidTransition { from, to });
        }

        Ok(Self { from, to })
    }

    /// Returns the source state.
    #[must_use]
    pub const fn from(self) -> LogicalResourceState {
        self.from
    }

    /// Returns the destination state.
    #[must_use]
    pub const fn to(self) -> LogicalResourceState {
        self.to
    }
}

// ============================================================================
// Logical resource entry
// ============================================================================

/// State record for one canonical logical qubit.
///
/// `generation` increases after every successful state transition.
///
/// It provides a lightweight optimistic-concurrency/replay mechanism without
/// introducing locks into the state model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LogicalResourceEntry {
    id: QubitId,
    state: LogicalResourceState,
    generation: u64,
}

impl LogicalResourceEntry {
    /// Creates a new logical-resource state entry.
    #[must_use]
    pub const fn new(id: QubitId) -> Self {
        Self {
            id,
            state: LogicalResourceState::Uninitialized,
            generation: 0,
        }
    }

    /// Creates an entry with an explicit state and generation.
    ///
    /// This is primarily intended for checkpoint restoration and deterministic
    /// deserialization.
    pub const fn from_parts(
        id: QubitId,
        state: LogicalResourceState,
        generation: u64,
    ) -> Self {
        Self {
            id,
            state,
            generation,
        }
    }

    /// Returns the canonical logical-qubit identity.
    #[must_use]
    pub const fn id(self) -> QubitId {
        self.id
    }

    /// Returns the current resilience state.
    #[must_use]
    pub const fn state(self) -> LogicalResourceState {
        self.state
    }

    /// Returns the state generation.
    ///
    /// Generation zero is the initial state.
    #[must_use]
    pub const fn generation(self) -> u64 {
        self.generation
    }

    /// Returns whether ordinary execution is currently permitted.
    #[must_use]
    pub const fn accepts_execution(self) -> bool {
        self.state.accepts_execution()
    }

    /// Returns whether the resource requires intervention.
    #[must_use]
    pub const fn requires_intervention(self) -> bool {
        self.state.requires_intervention()
    }

    /// Returns whether this entry is retired.
    #[must_use]
    pub const fn is_retired(self) -> bool {
        self.state.is_terminal()
    }
}

impl fmt::Display for LogicalResourceEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}@{}",
            self.id, self.state, self.generation
        )
    }
}

// ============================================================================
// Snapshot
// ============================================================================

/// Immutable logical-state snapshot.
///
/// The snapshot contains the complete logical state represented by the
/// associated `LogicalStateStore`.
///
/// It intentionally uses an ordered map so that snapshots have deterministic
/// iteration order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogicalStateSnapshot {
    entries: BTreeMap<QubitId, LogicalResourceEntry>,
}

impl LogicalStateSnapshot {
    /// Creates a snapshot from an ordered map.
    #[must_use]
    pub fn from_entries(entries: BTreeMap<QubitId, LogicalResourceEntry>) -> Self {
        Self { entries }
    }

    /// Returns the number of logical resources represented.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the snapshot contains no logical resources.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Looks up one logical qubit.
    #[must_use]
    pub fn get(&self, id: QubitId) -> Option<&LogicalResourceEntry> {
        self.entries.get(&id)
    }

    /// Returns deterministic snapshot iteration.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&QubitId, &LogicalResourceEntry)> + '_ {
        self.entries.iter()
    }

    /// Returns the underlying immutable map.
    ///
    /// The returned reference cannot mutate the snapshot.
    #[must_use]
    pub fn entries(&self) -> &BTreeMap<QubitId, LogicalResourceEntry> {
        &self.entries
    }

    /// Consumes the snapshot and returns its entries.
    #[must_use]
    pub fn into_entries(self) -> BTreeMap<QubitId, LogicalResourceEntry> {
        self.entries
    }
}

// ============================================================================
// Store
// ============================================================================

/// Deterministic resilience state store for logical qubits.
///
/// This is the main type consumed by higher-level resilience state modules.
///
/// It deliberately does not contain physical placement.
///
/// A single store may represent:
///
/// - one logical qubit;
/// - a small circuit;
/// - a very large logical register;
/// - a partition of a distributed workload.
///
/// No machine-size constant is encoded.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LogicalStateStore {
    entries: BTreeMap<QubitId, LogicalResourceEntry>,
}

impl LogicalStateStore {
    /// Creates an empty logical state store.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Creates a store with the supplied initial entries.
    ///
    /// The input must not contain an entry whose key differs from its embedded
    /// logical identity.
    pub fn from_entries(
        entries: BTreeMap<QubitId, LogicalResourceEntry>,
    ) -> Result<Self, LogicalStateError> {
        for (key, entry) in &entries {
            if *key != entry.id() {
                return Err(LogicalStateError::IdentityMismatch {
                    key: *key,
                    entry: entry.id(),
                });
            }
        }

        Ok(Self { entries })
    }

    /// Returns the number of tracked logical qubits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether no logical qubits are tracked.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns whether the store contains the specified logical qubit.
    #[must_use]
    pub fn contains(&self, id: QubitId) -> bool {
        self.entries.contains_key(&id)
    }

    /// Returns one logical-resource entry.
    #[must_use]
    pub fn get(&self, id: QubitId) -> Option<&LogicalResourceEntry> {
        self.entries.get(&id)
    }

    /// Returns one logical-resource entry by value.
    #[must_use]
    pub fn entry(&self, id: QubitId) -> Option<LogicalResourceEntry> {
        self.entries.get(&id).copied()
    }

    /// Returns deterministic iteration over all logical resources.
    pub fn iter(&self) -> impl Iterator<Item = (&QubitId, &LogicalResourceEntry)> + '_ {
        self.entries.iter()
    }

    /// Returns the canonical ordered map.
    ///
    /// This is useful to checkpoint/serialization layers.
    #[must_use]
    pub fn entries(&self) -> &BTreeMap<QubitId, LogicalResourceEntry> {
        &self.entries
    }

    /// Registers a logical qubit if it does not already exist.
    ///
    /// Returns `true` when a new entry was created and `false` when the
    /// resource already existed.
    pub fn ensure(&mut self, id: QubitId) -> bool {
        match self.entries.entry(id) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(LogicalResourceEntry::new(id));
                true
            }
            std::collections::btree_map::Entry::Occupied(_) => false,
        }
    }

    /// Registers a logical qubit in a specified initial state.
    ///
    /// Initial state must be `Uninitialized`, `Ready`, or `Verified`.
    /// Creating a resource directly in a recovery/isolation state would bypass
    /// the resilience lifecycle.
    pub fn insert(
        &mut self,
        id: QubitId,
        state: LogicalResourceState,
    ) -> Result<bool, LogicalStateError> {
        if !matches!(
            state,
            LogicalResourceState::Uninitialized
                | LogicalResourceState::Ready
                | LogicalResourceState::Verified
        ) {
            return Err(LogicalStateError::InvalidInitialState { state });
        }

        match self.entries.entry(id) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(LogicalResourceEntry::from_parts(id, state, 0));
                Ok(true)
            }
            std::collections::btree_map::Entry::Occupied(_) => Ok(false),
        }
    }

    /// Removes a logical qubit from this state context.
    ///
    /// Removal is a state-store operation, not a physical-resource
    /// deallocation. Higher-level lifecycle code must decide whether removal is
    /// semantically permitted.
    pub fn remove(
        &mut self,
        id: QubitId,
    ) -> Result<LogicalResourceEntry, LogicalStateError> {
        self.entries
            .remove(&id)
            .ok_or(LogicalStateError::UnknownLogicalQubit { id })
    }

    /// Clears all state records.
    ///
    /// This is intended for lifecycle reset of the state context. It does not
    /// alter the canonical IR or physical hardware.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns the current state of one logical qubit.
    pub fn state(
        &self,
        id: QubitId,
    ) -> Result<LogicalResourceState, LogicalStateError> {
        self.entries
            .get(&id)
            .map(|entry| entry.state())
            .ok_or(LogicalStateError::UnknownLogicalQubit { id })
    }

    /// Returns the current generation of one logical qubit.
    pub fn generation(&self, id: QubitId) -> Result<u64, LogicalStateError> {
        self.entries
            .get(&id)
            .map(|entry| entry.generation())
            .ok_or(LogicalStateError::UnknownLogicalQubit { id })
    }

    /// Determines whether a transition is currently legal without mutating
    /// state.
    pub fn can_transition(
        &self,
        id: QubitId,
        to: LogicalResourceState,
    ) -> Result<bool, LogicalStateError> {
        let entry = self
            .entries
            .get(&id)
            .ok_or(LogicalStateError::UnknownLogicalQubit { id })?;

        Ok(is_valid_transition(entry.state(), to))
    }

    /// Computes the next state without mutating this store.
    pub fn next_state(
        &self,
        id: QubitId,
        to: LogicalResourceState,
    ) -> Result<LogicalResourceState, LogicalStateError> {
        let current = self.state(id)?;

        if !is_valid_transition(current, to) {
            return Err(LogicalStateError::InvalidTransition {
                from: current,
                to,
            });
        }

        Ok(to)
    }

    /// Transitions one logical qubit.
    ///
    /// The generation is incremented exactly once on success.
    pub fn transition(
        &mut self,
        id: QubitId,
        to: LogicalResourceState,
    ) -> Result<LogicalStateTransition, LogicalStateError> {
        let entry = self
            .entries
            .get_mut(&id)
            .ok_or(LogicalStateError::UnknownLogicalQubit { id })?;

        let from = entry.state();

        if !is_valid_transition(from, to) {
            return Err(LogicalStateError::InvalidTransition { from, to });
        }

        let generation = entry
            .generation()
            .checked_add(1)
            .ok_or(LogicalStateError::GenerationOverflow { id })?;

        entry.state = to;
        entry.generation = generation;

        Ok(LogicalStateTransition { from, to })
    }

    /// Transitions one logical qubit using an optimistic generation check.
    ///
    /// This allows higher layers to detect stale decisions without requiring
    /// locks.
    pub fn transition_if_generation(
        &mut self,
        id: QubitId,
        expected_generation: u64,
        to: LogicalResourceState,
    ) -> Result<LogicalStateTransition, LogicalStateError> {
        let entry = self
            .entries
            .get_mut(&id)
            .ok_or(LogicalStateError::UnknownLogicalQubit { id })?;

        if entry.generation() != expected_generation {
            return Err(LogicalStateError::GenerationConflict {
                id,
                expected: expected_generation,
                actual: entry.generation(),
            });
        }

        let from = entry.state();

        if !is_valid_transition(from, to) {
            return Err(LogicalStateError::InvalidTransition { from, to });
        }

        let generation = entry
            .generation()
            .checked_add(1)
            .ok_or(LogicalStateError::GenerationOverflow { id })?;

        entry.state = to;
        entry.generation = generation;

        Ok(LogicalStateTransition { from, to })
    }

    /// Applies multiple logical-qubit transitions transactionally.
    ///
    /// If any transition fails, every state modified by this call is restored
    /// to its exact previous value.
    ///
    /// The rollback journal is proportional to the number of requested
    /// transitions, not to the total number of tracked logical qubits.
    ///
    /// Duplicate logical-qubit IDs are allowed. They are processed in input
    /// order and therefore permit explicit transition sequences such as:
    ///
    /// ```text
    /// Active -> Recovering -> Active
    /// ```
    ///
    /// within one transaction.
    pub fn transition_many<I>(
        &mut self,
        transitions: I,
    ) -> Result<Vec<LogicalStateTransition>, LogicalStateError>
    where
        I: IntoIterator<Item = (QubitId, LogicalResourceState)>,
    {
        let mut journal: Vec<(QubitId, LogicalResourceEntry)> = Vec::new();
        let mut result: Vec<LogicalStateTransition> = Vec::new();

        for (id, to) in transitions {
            let current = match self.entries.get(&id).copied() {
                Some(entry) => entry,
                None => {
                    rollback_entries(&mut self.entries, &journal);
                    return Err(LogicalStateError::UnknownLogicalQubit { id });
                }
            };

            if !is_valid_transition(current.state(), to) {
                rollback_entries(&mut self.entries, &journal);
                return Err(LogicalStateError::InvalidTransition {
                    from: current.state(),
                    to,
                });
            }

            let next_generation = match current.generation().checked_add(1) {
                Some(generation) => generation,
                None => {
                    rollback_entries(&mut self.entries, &journal);
                    return Err(LogicalStateError::GenerationOverflow { id });
                }
            };

            journal.push((id, current));

            let updated = LogicalResourceEntry::from_parts(
                id,
                to,
                next_generation,
            );

            self.entries.insert(id, updated);
            result.push(LogicalStateTransition {
                from: current.state(),
                to,
            });
        }

        Ok(result)
    }

    /// Transitions every currently tracked logical qubit from one state to
    /// another.
    ///
    /// This operation is useful when a system-wide event affects every
    /// logical resource represented by this state context.
    ///
    /// It remains transactional.
    pub fn transition_all(
        &mut self,
        from: LogicalResourceState,
        to: LogicalResourceState,
    ) -> Result<usize, LogicalStateError> {
        if !is_valid_transition(from, to) {
            return Err(LogicalStateError::InvalidTransition { from, to });
        }

        if self.entries.is_empty() {
            return Ok(0);
        }

        let ids: Vec<QubitId> = self
            .entries
            .iter()
            .filter_map(|(id, entry)| {
                if entry.state() == from {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect();

        let count = ids.len();

        let requests = ids.into_iter().map(|id| (id, to));

        self.transition_many(requests)?;

        Ok(count)
    }

    /// Creates an immutable deterministic snapshot.
    #[must_use]
    pub fn snapshot(&self) -> LogicalStateSnapshot {
        LogicalStateSnapshot {
            entries: self.entries.clone(),
        }
    }

    /// Replaces the current state with a previously validated snapshot.
    ///
    /// The snapshot must contain self-consistent key/identity pairs.
    pub fn restore(
        &mut self,
        snapshot: LogicalStateSnapshot,
    ) -> Result<(), LogicalStateError> {
        for (key, entry) in snapshot.entries() {
            if *key != entry.id() {
                return Err(LogicalStateError::IdentityMismatch {
                    key: *key,
                    entry: entry.id(),
                });
            }
        }

        self.entries = snapshot.into_entries();
        Ok(())
    }

    /// Counts logical resources in a particular state.
    #[must_use]
    pub fn count_state(&self, state: LogicalResourceState) -> usize {
        self.entries
            .values()
            .filter(|entry| entry.state() == state)
            .count()
    }

    /// Returns whether every tracked logical resource is in the specified
    /// state.
    ///
    /// An empty store returns `true`, following the standard universal
    /// quantification rule.
    #[must_use]
    pub fn all_in_state(&self, state: LogicalResourceState) -> bool {
        self.entries
            .values()
            .all(|entry| entry.state() == state)
    }

    /// Returns whether at least one tracked logical resource is in the
    /// specified state.
    #[must_use]
    pub fn any_in_state(&self, state: LogicalResourceState) -> bool {
        self.entries
            .values()
            .any(|entry| entry.state() == state)
    }

    /// Returns whether every tracked logical resource currently accepts
    /// execution.
    #[must_use]
    pub fn all_execution_ready(&self) -> bool {
        self.entries
            .values()
            .all(|entry| entry.accepts_execution())
    }

    /// Returns whether any logical resource requires resilience intervention.
    #[must_use]
    pub fn requires_intervention(&self) -> bool {
        self.entries
            .values()
            .any(|entry| entry.requires_intervention())
    }
}

// ============================================================================
// State transition relation
// ============================================================================

/// Determines whether a logical-resource state transition is legal.
///
/// This relation is deliberately pure and independent of:
///
/// - hardware;
/// - QEC;
/// - routing;
/// - scheduling;
/// - policy;
/// - provider.
///
/// Higher-level policy may impose *additional* restrictions, but it must not
/// silently bypass this lifecycle invariant.
#[must_use]
pub const fn is_valid_transition(
    from: LogicalResourceState,
    to: LogicalResourceState,
) -> bool {
    match from {
        LogicalResourceState::Uninitialized => matches!(
            to,
            LogicalResourceState::Ready
                | LogicalResourceState::Retired
        ),

        LogicalResourceState::Ready => matches!(
            to,
            LogicalResourceState::Active
                | LogicalResourceState::Suspended
                | LogicalResourceState::Recovering
                | LogicalResourceState::Quarantined
                | LogicalResourceState::Retired
        ),

        LogicalResourceState::Active => matches!(
            to,
            LogicalResourceState::Suspended
                | LogicalResourceState::Recovering
                | LogicalResourceState::Verified
                | LogicalResourceState::Quarantined
                | LogicalResourceState::Retired
        ),

        LogicalResourceState::Suspended => matches!(
            to,
            LogicalResourceState::Active
                | LogicalResourceState::Recovering
                | LogicalResourceState::Quarantined
                | LogicalResourceState::Retired
        ),

        LogicalResourceState::Recovering => matches!(
            to,
            LogicalResourceState::Active
                | LogicalResourceState::Verified
                | LogicalResourceState::Suspended
                | LogicalResourceState::Quarantined
                | LogicalResourceState::Retired
        ),

        LogicalResourceState::Verified => matches!(
            to,
            LogicalResourceState::Active
                | LogicalResourceState::Suspended
                | LogicalResourceState::Recovering
                | LogicalResourceState::Quarantined
                | LogicalResourceState::Retired
        ),

        LogicalResourceState::Quarantined => matches!(
            to,
            LogicalResourceState::Recovering
                | LogicalResourceState::Retired
        ),

        LogicalResourceState::Retired => false,
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by logical resilience state management.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogicalStateError {
    /// The requested logical qubit is not tracked.
    UnknownLogicalQubit {
        /// Requested logical-qubit identity.
        id: QubitId,
    },

    /// A requested lifecycle transition is not legal.
    InvalidTransition {
        /// Current state.
        from: LogicalResourceState,

        /// Requested destination state.
        to: LogicalResourceState,
    },

    /// The supplied initial state would bypass the lifecycle.
    InvalidInitialState {
        /// Invalid initial state.
        state: LogicalResourceState,
    },

    /// A map key and embedded logical identity differ.
    IdentityMismatch {
        /// Map key.
        key: QubitId,

        /// Identity stored inside the entry.
        entry: QubitId,
    },

    /// A generation counter cannot be incremented further.
    GenerationOverflow {
        /// Logical resource whose generation overflowed.
        id: QubitId,
    },

    /// An optimistic transition used stale state.
    GenerationConflict {
        /// Logical resource.
        id: QubitId,

        /// Generation expected by the caller.
        expected: u64,

        /// Generation actually present.
        actual: u64,
    },
}

impl fmt::Display for LogicalStateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownLogicalQubit { id } => {
                write!(formatter, "unknown logical qubit {id}")
            }

            Self::InvalidTransition { from, to } => {
                write!(
                    formatter,
                    "invalid logical resilience transition: {from} -> {to}"
                )
            }

            Self::InvalidInitialState { state } => {
                write!(
                    formatter,
                    "invalid initial logical resilience state: {state}"
                )
            }

            Self::IdentityMismatch { key, entry } => {
                write!(
                    formatter,
                    "logical state identity mismatch: map key {key} != entry identity {entry}"
                )
            }

            Self::GenerationOverflow { id } => {
                write!(
                    formatter,
                    "logical state generation overflow for {id}"
                )
            }

            Self::GenerationConflict {
                id,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "logical state generation conflict for {id}: expected {expected}, actual {actual}"
                )
            }
        }
    }
}

impl Error for LogicalStateError {}

// ============================================================================
// Rollback
// ============================================================================

/// Restores entries recorded by a transaction journal.
///
/// The journal contains the exact previous value for every mutation. Because
/// `BTreeMap::insert` cannot fail under normal Rust semantics, rollback does
/// not require unsafe code or fallible allocation.
///
/// If allocation had failed while constructing the journal/result before this
/// function was reached, Rust's allocation failure behavior is process-level;
/// it is not representable as an ordinary recoverable Rust `Result`.
fn rollback_entries(
    entries: &mut BTreeMap<QubitId, LogicalResourceEntry>,
    journal: &[(QubitId, LogicalResourceEntry)],
) {
    for (id, previous) in journal.iter().rev() {
        entries.insert(*id, *previous);
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    #[test]
    fn state_ordinals_are_stable() {
        assert_eq!(
            LogicalResourceState::Uninitialized.ordinal(),
            0
        );
        assert_eq!(LogicalResourceState::Ready.ordinal(), 1);
        assert_eq!(LogicalResourceState::Active.ordinal(), 2);
        assert_eq!(LogicalResourceState::Suspended.ordinal(), 3);
        assert_eq!(LogicalResourceState::Recovering.ordinal(), 4);
        assert_eq!(LogicalResourceState::Verified.ordinal(), 5);
        assert_eq!(LogicalResourceState::Quarantined.ordinal(), 6);
        assert_eq!(LogicalResourceState::Retired.ordinal(), 7);

        for value in 0..=7 {
            assert!(LogicalResourceState::from_ordinal(value).is_some());
        }

        assert_eq!(LogicalResourceState::from_ordinal(8), None);
    }

    #[test]
    fn canonical_logical_identity_is_used() {
        let id = q(42);
        let entry = LogicalResourceEntry::new(id);

        assert_eq!(entry.id(), id);
        assert_eq!(entry.state(), LogicalResourceState::Uninitialized);
        assert_eq!(entry.generation(), 0);
    }

    #[test]
    fn default_store_is_empty() {
        let store = LogicalStateStore::new();

        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
        assert!(!store.contains(q(0)));
    }

    #[test]
    fn ensure_is_idempotent() {
        let mut store = LogicalStateStore::new();

        assert!(store.ensure(q(1)));
        assert!(!store.ensure(q(1)));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn valid_lifecycle_works() {
        let mut store = LogicalStateStore::new();

        store.ensure(q(0));

        store
            .transition(q(0), LogicalResourceState::Ready)
            .expect("uninitialized -> ready");

        store
            .transition(q(0), LogicalResourceState::Active)
            .expect("ready -> active");

        store
            .transition(q(0), LogicalResourceState::Recovering)
            .expect("active -> recovering");

        store
            .transition(q(0), LogicalResourceState::Verified)
            .expect("recovering -> verified");

        assert_eq!(
            store.state(q(0)).expect("state exists"),
            LogicalResourceState::Verified
        );

        assert_eq!(
            store.generation(q(0)).expect("generation exists"),
            4
        );
    }

    #[test]
    fn invalid_transition_does_not_mutate() {
        let mut store = LogicalStateStore::new();

        store.ensure(q(0));

        let before = store.entry(q(0)).expect("entry exists");

        let error = store
            .transition(q(0), LogicalResourceState::Active)
            .expect_err("uninitialized -> active must fail");

        assert_eq!(
            error,
            LogicalStateError::InvalidTransition {
                from: LogicalResourceState::Uninitialized,
                to: LogicalResourceState::Active,
            }
        );

        assert_eq!(
            store.entry(q(0)).expect("entry exists"),
            before
        );
    }

    #[test]
    fn retired_is_terminal() {
        assert!(!is_valid_transition(
            LogicalResourceState::Retired,
            LogicalResourceState::Ready
        ));

        assert!(!is_valid_transition(
            LogicalResourceState::Retired,
            LogicalResourceState::Active
        ));
    }

    #[test]
    fn quarantine_requires_recovery() {
        assert!(is_valid_transition(
            LogicalResourceState::Quarantined,
            LogicalResourceState::Recovering
        ));

        assert!(!is_valid_transition(
            LogicalResourceState::Quarantined,
            LogicalResourceState::Active
        ));
    }

    #[test]
    fn optimistic_generation_prevents_stale_update() {
        let mut store = LogicalStateStore::new();

        store.ensure(q(1));

        store
            .transition(q(1), LogicalResourceState::Ready)
            .expect("initialization");

        let generation = store.generation(q(1)).expect("generation");

        store
            .transition_if_generation(
                q(1),
                generation,
                LogicalResourceState::Active,
            )
            .expect("current generation must work");

        let error = store
            .transition_if_generation(
                q(1),
                generation,
                LogicalResourceState::Suspended,
            )
            .expect_err("stale generation must fail");

        assert_eq!(
            error,
            LogicalStateError::GenerationConflict {
                id: q(1),
                expected: generation,
                actual: generation + 1,
            }
        );
    }

    #[test]
    fn transaction_commits_all_transitions() {
        let mut store = LogicalStateStore::new();

        store.ensure(q(0));
        store.ensure(q(1));

        let result = store
            .transition_many([
                (q(0), LogicalResourceState::Ready),
                (q(1), LogicalResourceState::Ready),
            ])
            .expect("transaction must succeed");

        assert_eq!(result.len(), 2);
        assert_eq!(
            store.state(q(0)).expect("q0"),
            LogicalResourceState::Ready
        );
        assert_eq!(
            store.state(q(1)).expect("q1"),
            LogicalResourceState::Ready
        );
    }

    #[test]
    fn transaction_rolls_back_on_failure() {
        let mut store = LogicalStateStore::new();

        store.ensure(q(0));
        store.ensure(q(1));

        let before_q0 = store.entry(q(0)).expect("q0");
        let before_q1 = store.entry(q(1)).expect("q1");

        let error = store
            .transition_many([
                (q(0), LogicalResourceState::Ready),
                (q(1), LogicalResourceState::Active),
            ])
            .expect_err("second transition must fail");

        assert_eq!(
            error,
            LogicalStateError::InvalidTransition {
                from: LogicalResourceState::Uninitialized,
                to: LogicalResourceState::Active,
            }
        );

        assert_eq!(
            store.entry(q(0)).expect("q0"),
            before_q0
        );
        assert_eq!(
            store.entry(q(1)).expect("q1"),
            before_q1
        );
    }

    #[test]
    fn duplicate_ids_are_transactional_and_ordered() {
        let mut store = LogicalStateStore::new();

        store.ensure(q(7));

        let result = store
            .transition_many([
                (q(7), LogicalResourceState::Ready),
                (q(7), LogicalResourceState::Active),
                (q(7), LogicalResourceState::Recovering),
                (q(7), LogicalResourceState::Verified),
            ])
            .expect("ordered transitions must succeed");

        assert_eq!(result.len(), 4);
        assert_eq!(
            store.state(q(7)).expect("q7"),
            LogicalResourceState::Verified
        );
        assert_eq!(
            store.generation(q(7)).expect("generation"),
            4
        );
    }

    #[test]
    fn transition_all_is_transactional() {
        let mut store = LogicalStateStore::new();

        for id in 0..8 {
            store.ensure(q(id));
            store
                .transition(q(id), LogicalResourceState::Ready)
                .expect("ready");
        }

        let count = store
            .transition_all(
                LogicalResourceState::Ready,
                LogicalResourceState::Active,
            )
            .expect("transition all");

        assert_eq!(count, 8);
        assert!(store.all_in_state(LogicalResourceState::Active));
    }

    #[test]
    fn empty_transition_all_is_valid() {
        let mut store = LogicalStateStore::new();

        let count = store
            .transition_all(
                LogicalResourceState::Ready,
                LogicalResourceState::Active,
            )
            .expect("empty store");

        assert_eq!(count, 0);
    }

    #[test]
    fn snapshot_is_deterministic() {
        let mut store = LogicalStateStore::new();

        store.ensure(q(100));
        store.ensure(q(2));
        store.ensure(q(50));

        let snapshot = store.snapshot();

        let ids: Vec<QubitId> =
            snapshot.iter().map(|(id, _)| *id).collect();

        assert_eq!(ids, vec![q(2), q(50), q(100)]);
    }

    #[test]
    fn snapshot_restore_round_trip() {
        let mut store = LogicalStateStore::new();

        store.ensure(q(1));
        store
            .transition(q(1), LogicalResourceState::Ready)
            .expect("ready");
        store
            .transition(q(1), LogicalResourceState::Active)
            .expect("active");

        let snapshot = store.snapshot();

        let mut restored = LogicalStateStore::new();

        restored
            .restore(snapshot.clone())
            .expect("restore");

        assert_eq!(restored.snapshot(), snapshot);
    }

    #[test]
    fn count_helpers_work() {
        let mut store = LogicalStateStore::new();

        for id in 0..5 {
            store.ensure(q(id));
        }

        store
            .transition_all(
                LogicalResourceState::Uninitialized,
                LogicalResourceState::Ready,
            )
            .expect("ready");

        store
            .transition(q(0), LogicalResourceState::Active)
            .expect("active");

        store
            .transition(q(1), LogicalResourceState::Active)
            .expect("active");

        assert_eq!(
            store.count_state(LogicalResourceState::Active),
            2
        );

        assert_eq!(
            store.count_state(LogicalResourceState::Ready),
            3
        );

        assert!(store.any_in_state(LogicalResourceState::Active));
        assert!(!store.all_in_state(LogicalResourceState::Active));
        assert!(store.all_execution_ready());
    }

    #[test]
    fn unknown_qubit_is_rejected() {
        let mut store = LogicalStateStore::new();

        let error = store
            .transition(q(999), LogicalResourceState::Ready)
            .expect_err("unknown qubit must fail");

        assert_eq!(
            error,
            LogicalStateError::UnknownLogicalQubit { id: q(999) }
        );
    }

    #[test]
    fn identity_mismatch_is_rejected() {
        let mut entries = BTreeMap::new();

        entries.insert(
            q(1),
            LogicalResourceEntry::new(q(2)),
        );

        let error = LogicalStateStore::from_entries(entries)
            .expect_err("identity mismatch must fail");

        assert_eq!(
            error,
            LogicalStateError::IdentityMismatch {
                key: q(1),
                entry: q(2),
            }
        );
    }

    #[test]
    fn retired_resource_can_be_created_only_explicitly_then_is_terminal() {
        let mut store = LogicalStateStore::new();

        let error = store
            .insert(q(1), LogicalResourceState::Retired)
            .expect_err("retired cannot bypass lifecycle");

        assert_eq!(
            error,
            LogicalStateError::InvalidInitialState {
                state: LogicalResourceState::Retired,
            }
        );

        store.ensure(q(1));

        store
            .transition(q(1), LogicalResourceState::Ready)
            .expect("ready");

        store
            .transition(q(1), LogicalResourceState::Retired)
            .expect("retire");

        assert!(store.entry(q(1)).expect("entry").is_retired());
    }

    #[test]
    fn state_properties_are_semantically_consistent() {
        assert!(
            LogicalResourceState::Ready.is_execution_ready()
        );
        assert!(
            LogicalResourceState::Active.is_execution_ready()
        );
        assert!(
            LogicalResourceState::Verified.is_execution_ready()
        );

        assert!(
            !LogicalResourceState::Recovering.is_execution_ready()
        );
        assert!(
            !LogicalResourceState::Quarantined.is_execution_ready()
        );

        assert!(
            LogicalResourceState::Recovering.requires_intervention()
        );
        assert!(
            LogicalResourceState::Quarantined.requires_intervention()
        );

        assert!(
            LogicalResourceState::Retired.is_terminal()
        );
    }
}