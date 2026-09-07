//! Zamani Quantum Resilience — Physical Resource State
//!
//! Path:
//!     src/quantum/resilience/state/physical.rs
//!
//! Responsibility
//! ============
//!
//! This module maintains resilience-owned lifecycle state for physical quantum
//! resources, principally `PhysicalQubitId` resources.
//!
//! It answers:
//!
//!     "What operational lifecycle state does resilience currently believe
//!      this physical resource is in?"
//!
//! It does NOT answer:
//!
//! - what hardware the resource belongs to;
//! - whether the hardware is calibrated;
//! - the resource's topology;
//! - the resource's native gates;
//! - its physical noise model;
//! - its measured fidelity;
//! - its temperature/frequency/pulse parameters;
//! - which logical qubit is routed onto it;
//! - how a route is calculated;
//! - how a schedule is calculated;
//! - how the backend is contacted;
//! - how quantum state is represented.
//!
//! Those responsibilities belong respectively to the hardware HAL, ZQN,
//! routing, scheduling, execution, simulator and other canonical subsystems.
//!
//! Canonical identity
//! ==================
//!
//! Physical resource identity MUST use:
//!
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This module intentionally does not define another `PhysicalQubitId`.
//!
//! Logical identity is a different type:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! This module does not use integer equality between those domains and does
//! not infer a logical-to-physical mapping from numeric identifiers.
//!
//! Architectural separation
//! =========================
//!
//! ```text
//! quantum::ir::qubit
//!        │
//!        │ PhysicalQubitId
//!        ▼
//! hardware HAL ───────► capabilities / topology / calibration / telemetry
//!        │
//!        ▼
//! resilience::state::physical
//!        │
//!        ├── lifecycle state
//!        ├── revision
//!        ├── transactional transitions
//!        └── deterministic snapshots
//!
//! routing ─────────────► logical ↔ physical placement
//! scheduling ───────────► execution timing
//! ZQN ─────────────────► physical fault/noise semantics
//! model::health ───────► health assessment
//! coordination ────────► ownership / leases
//!
//! resilience::state::physical MUST NOT replace any of those subsystems.
//! ```
//!
//! Scaling
//! =======
//!
//! There is no architectural maximum number of physical resources.
//!
//! The store uses an ordered sparse map rather than a fixed-size array or a
//! vector indexed by physical-qubit identity. This is intentional:
//!
//! - physical IDs are identities, not array positions;
//! - sparse hardware/resource inventories are valid;
//! - identifiers need not be contiguous;
//! - a backend can expose arbitrary identifier values;
//! - distributed systems can merge independently discovered resource IDs;
//! - memory consumption remains proportional to resources actually tracked.
//!
//! "Infinity" means no artificial finite machine-size ceiling is encoded here.
//! Every concrete execution remains bounded by actual memory, CPU, target
//! capabilities and explicit resource policies.
//!
//! Transactional semantics
//! =======================
//!
//! Individual transitions are validated against the lifecycle graph.
//!
//! Batch transitions are transactional:
//!
//!     all requested transitions succeed
//!         OR
//!     the complete previous state is restored.
//!
//! No partial successful batch is exposed to callers.
//!
//! Determinism
//! ===========
//!
//! `BTreeMap` is used deliberately instead of `HashMap` so iteration order is
//! deterministic for identical state.
//!
//! This module does not:
//!
//! - read the clock;
//! - generate randomness;
//! - access global mutable state;
//! - contact a backend;
//! - perform I/O;
//! - perform asynchronous work.
//!
//! Persistence/serialization belongs to the resilience checkpoint and
//! serialization subsystems.
//!
//! Rust contract
//! ==============
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! Integration contract
//! ====================
//!
//! `state/mod.rs` should expose:
//!
//!     pub mod physical;
//!
//! Consumers should import the types from this module rather than duplicating
//! physical-resource state definitions.
//!
//! Hardware HAL integration:
//!
//!     hardware -> observations/capabilities -> resilience controller
//!     resilience state -> lifecycle decisions
//!
//! Routing integration:
//!
//!     routing owns logical -> physical mapping.
//!
//! Coordination integration:
//!
//!     coordination owns leases/ownership.
//!
//! Health integration:
//!
//!     model::health owns health semantics.
//!
//! Recovery integration:
//!
//!     recovery/* requests transitions through this state store.
//!
//! Verification integration:
//!
//!     verification/* validates that a resource state used by an execution is
//!     still acceptable before accepting the result.
//!
//! Persistence integration:
//!
//!     state/persistence.rs and checkpoint/* own persistence.
//!
//! This module intentionally exposes deterministic snapshots so those layers
//! can persist the state without requiring this module to know the storage
//! format.
//!
//! Safety invariant
//! ================
//!
//! A physical resource marked `Quarantined` or `Retired` must never transition
//! directly to `Active` or `Reserved`.
//!
//! A recovery path must pass through `Recovering` and normally `Verifying`
//! before becoming `Available` again.
//!
//! This prevents a stale recovery decision from silently reintroducing a
//! previously failed physical resource.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::quantum::ir::qubit::PhysicalQubitId;

// =============================================================================
// Physical resource status
// =============================================================================

/// Resilience lifecycle state of a physical quantum resource.
///
/// This is deliberately different from `model::health::HealthState`.
///
/// `HealthState` answers a health/condition question.
///
/// `PhysicalQubitStatus` answers a lifecycle/operational-state question.
///
/// For example:
///
/// ```text
/// Health = Degraded
/// Lifecycle = Available
/// ```
///
/// can be valid when the policy still permits use of a degraded resource.
///
/// Conversely:
///
/// ```text
/// Health = Unknown
/// Lifecycle = Quarantined
/// ```
///
/// is valid when resilience refuses to use a resource until its condition is
/// established.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PhysicalQubitStatus {
    /// Resource is known to resilience and may be considered for allocation.
    Available,

    /// Resource has been reserved by a higher-level execution/coordination
    /// operation.
    ///
    /// This module records the lifecycle state but does not own the lease or
    /// ownership semantics.
    Reserved,

    /// Resource is currently participating in an execution.
    Active,

    /// Resource is temporarily unavailable while an execution/recovery
    /// boundary is preserved.
    Suspended,

    /// Resource requires recovery before normal operation may resume.
    RecoveryRequired,

    /// A recovery operation is actively repairing/reinitializing the
    /// resource.
    Recovering,

    /// Recovery has completed and the resource is awaiting verification.
    Verifying,

    /// Resource has been deliberately isolated and MUST NOT be selected for
    /// ordinary execution.
    Quarantined,

    /// Resource has permanently left the resilience-managed resource set.
    ///
    /// `Retired` is terminal.
    Retired,
}

impl PhysicalQubitStatus {
    /// Returns whether ordinary allocation may consider this resource.
    #[must_use]
    pub const fn is_allocatable(self) -> bool {
        matches!(self, Self::Available)
    }

    /// Returns whether the resource is currently executing.
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether the resource is in a recovery lifecycle.
    #[must_use]
    pub const fn is_recovering(self) -> bool {
        matches!(
            self,
            Self::RecoveryRequired | Self::Recovering | Self::Verifying
        )
    }

    /// Returns whether the resource is isolated from ordinary execution.
    #[must_use]
    pub const fn is_isolated(self) -> bool {
        matches!(self, Self::Quarantined | Self::Retired)
    }

    /// Returns whether this status is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Retired)
    }

    /// Returns whether a transition from `self` to `target` is permitted.
    ///
    /// The transition graph is intentionally explicit and provider-independent.
    ///
    /// ```text
    /// Available
    ///    │
    ///    ├──► Reserved ──► Active
    ///    │                   │
    ///    │                   ├──► Available
    ///    │                   ├──► Suspended
    ///    │                   ├──► RecoveryRequired
    ///    │                   ├──► Verifying
    ///    │                   ├──► Quarantined
    ///    │                   └──► Retired
    ///    │
    ///    ├──► Active
    ///    ├──► RecoveryRequired
    ///    ├──► Quarantined
    ///    └──► Retired
    ///
    /// RecoveryRequired
    ///       │
    ///       ├──► Recovering
    ///       ├──► Quarantined
    ///       └──► Retired
    ///
    /// Recovering
    ///       │
    ///       ├──► Verifying
    ///       ├──► RecoveryRequired
    ///       ├──► Quarantined
    ///       └──► Retired
    ///
    /// Verifying
    ///       │
    ///       ├──► Available
    ///       ├──► RecoveryRequired
    ///       ├──► Quarantined
    ///       └──► Retired
    ///
    /// Quarantined
    ///       │
    ///       ├──► Recovering
    ///       └──► Retired
    ///
    /// Retired
    ///       └──► terminal
    /// ```
    #[must_use]
    pub const fn can_transition_to(self, target: Self) -> bool {
        match self {
            Self::Available => matches!(
                target,
                Self::Reserved
                    | Self::Active
                    | Self::Suspended
                    | Self::RecoveryRequired
                    | Self::Quarantined
                    | Self::Retired
            ),

            Self::Reserved => matches!(
                target,
                Self::Available
                    | Self::Active
                    | Self::Suspended
                    | Self::RecoveryRequired
                    | Self::Quarantined
                    | Self::Retired
            ),

            Self::Active => matches!(
                target,
                Self::Available
                    | Self::Suspended
                    | Self::RecoveryRequired
                    | Self::Verifying
                    | Self::Quarantined
                    | Self::Retired
            ),

            Self::Suspended => matches!(
                target,
                Self::Available
                    | Self::Active
                    | Self::RecoveryRequired
                    | Self::Quarantined
                    | Self::Retired
            ),

            Self::RecoveryRequired => matches!(
                target,
                Self::Recovering | Self::Quarantined | Self::Retired
            ),

            Self::Recovering => matches!(
                target,
                Self::Verifying
                    | Self::RecoveryRequired
                    | Self::Quarantined
                    | Self::Retired
            ),

            Self::Verifying => matches!(
                target,
                Self::Available
                    | Self::RecoveryRequired
                    | Self::Quarantined
                    | Self::Retired
            ),

            Self::Quarantined => {
                matches!(target, Self::Recovering | Self::Retired)
            }

            Self::Retired => false,
        }
    }
}

impl fmt::Display for PhysicalQubitStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Available => "available",
            Self::Reserved => "reserved",
            Self::Active => "active",
            Self::Suspended => "suspended",
            Self::RecoveryRequired => "recovery-required",
            Self::Recovering => "recovering",
            Self::Verifying => "verifying",
            Self::Quarantined => "quarantined",
            Self::Retired => "retired",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Individual physical-resource state
// =============================================================================

/// Resilience state associated with one physical qubit.
///
/// This is intentionally small and identity-oriented.
///
/// It does not contain a logical-qubit owner because logical→physical mapping
/// belongs to routing/adaptation. It does not contain health metrics because
/// those belong to the resilience model/health layer and ultimately hardware
/// observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalQubitState {
    status: PhysicalQubitStatus,

    /// Monotonic per-resource state generation.
    ///
    /// This allows callers to detect stale observations without coupling this
    /// module to wall-clock timestamps.
    generation: u64,
}

impl PhysicalQubitState {
    /// Creates the initial state of a newly registered physical resource.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            status: PhysicalQubitStatus::Available,
            generation: 0,
        }
    }

    /// Returns the lifecycle status.
    #[must_use]
    pub const fn status(self) -> PhysicalQubitStatus {
        self.status
    }

    /// Returns the resource-local generation.
    #[must_use]
    pub const fn generation(self) -> u64 {
        self.generation
    }

    /// Returns whether the resource can currently be allocated.
    #[must_use]
    pub const fn is_allocatable(self) -> bool {
        self.status.is_allocatable()
    }

    /// Returns whether the resource is currently isolated.
    #[must_use]
    pub const fn is_isolated(self) -> bool {
        self.status.is_isolated()
    }

    fn transition_to(
        &mut self,
        target: PhysicalQubitStatus,
    ) -> Result<(), PhysicalStateError> {
        if self.status == target {
            return Ok(());
        }

        if !self.status.can_transition_to(target) {
            return Err(PhysicalStateError::InvalidTransition {
                from: self.status,
                to: target,
            });
        }

        let next_generation = self.generation.checked_add(1).ok_or(
            PhysicalStateError::GenerationOverflow,
        )?;

        self.status = target;
        self.generation = next_generation;

        Ok(())
    }
}

impl Default for PhysicalQubitState {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Transition record
// =============================================================================

/// Immutable record describing one successful physical-resource transition.
///
/// The record is suitable for telemetry, audit trails and deterministic
/// recovery history. It contains no timestamp because this module deliberately
/// does not own time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalStateTransition {
    resource: PhysicalQubitId,
    from: PhysicalQubitStatus,
    to: PhysicalQubitStatus,
    generation_before: u64,
    generation_after: u64,
    revision: u64,
}

impl PhysicalStateTransition {
    /// Returns the affected physical resource.
    #[must_use]
    pub const fn resource(self) -> PhysicalQubitId {
        self.resource
    }

    /// Returns the previous status.
    #[must_use]
    pub const fn from(self) -> PhysicalQubitStatus {
        self.from
    }

    /// Returns the resulting status.
    #[must_use]
    pub const fn to(self) -> PhysicalQubitStatus {
        self.to
    }

    /// Returns the resource generation before the transition.
    #[must_use]
    pub const fn generation_before(self) -> u64 {
        self.generation_before
    }

    /// Returns the resource generation after the transition.
    #[must_use]
    pub const fn generation_after(self) -> u64 {
        self.generation_after
    }

    /// Returns the store revision after the transition.
    #[must_use]
    pub const fn revision(self) -> u64 {
        self.revision
    }
}

// =============================================================================
// Snapshot
// =============================================================================

/// Deterministic snapshot of the complete physical-resource resilience state.
///
/// This type contains no storage-format assumptions. Checkpoint/serialization
/// layers may encode it using their own versioned schema.
///
/// Cloning a snapshot is intentionally an explicit O(number of tracked
/// resources) operation. Callers requiring very large-scale checkpointing can
/// stream state through the public iterator instead of cloning the map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalStateSnapshot {
    revision: u64,
    resources: BTreeMap<PhysicalQubitId, PhysicalQubitState>,
}

impl PhysicalStateSnapshot {
    /// Returns the global state revision represented by this snapshot.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// Returns the number of resources represented in the snapshot.
    #[must_use]
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    /// Returns whether the snapshot contains no resources.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Looks up one resource.
    #[must_use]
    pub fn get(
        &self,
        resource: PhysicalQubitId,
    ) -> Option<PhysicalQubitState> {
        self.resources.get(&resource).copied()
    }

    /// Iterates deterministically by `PhysicalQubitId`.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (PhysicalQubitId, PhysicalQubitState)> + '_ {
        self.resources.iter().map(|(id, state)| (*id, *state))
    }
}

// =============================================================================
// Store
// =============================================================================

/// Deterministic resilience state store for physical quantum resources.
///
/// The store:
///
/// - uses canonical `PhysicalQubitId`;
/// - supports sparse/non-contiguous identifiers;
/// - has no fixed resource limit;
/// - validates lifecycle transitions;
/// - provides transactional batch transitions;
/// - provides optimistic revision checking;
/// - provides deterministic snapshots.
///
/// It deliberately does NOT:
///
/// - allocate hardware;
/// - discover hardware;
/// - own leases;
/// - calculate routes;
/// - calculate schedules;
/// - assess physical noise;
/// - execute quantum operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalStateStore {
    revision: u64,
    resources: BTreeMap<PhysicalQubitId, PhysicalQubitState>,
}

impl PhysicalStateStore {
    /// Creates an empty physical-resource state store.
    #[must_use]
    pub fn new() -> Self {
        Self {
            revision: 0,
            resources: BTreeMap::new(),
        }
    }

    /// Creates an empty store.
    ///
    /// The capacity argument is intentionally advisory and does not become a
    /// machine-size limit. `BTreeMap` does not expose a stable capacity API,
    /// so the store starts empty and grows according to actual demand.
    #[must_use]
    pub fn with_capacity(_capacity: usize) -> Self {
        Self::new()
    }

    /// Returns the current global state revision.
    ///
    /// Revision zero represents an empty/new store before any mutation.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// Returns the number of tracked physical resources.
    #[must_use]
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    /// Returns whether no physical resources are tracked.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Returns whether a resource is registered.
    #[must_use]
    pub fn contains(&self, resource: PhysicalQubitId) -> bool {
        self.resources.contains_key(&resource)
    }

    /// Returns the current state of a resource.
    #[must_use]
    pub fn get(
        &self,
        resource: PhysicalQubitId,
    ) -> Option<PhysicalQubitState> {
        self.resources.get(&resource).copied()
    }

    /// Returns an immutable deterministic iterator over all tracked resources.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (PhysicalQubitId, PhysicalQubitState)> + '_ {
        self.resources.iter().map(|(id, state)| (*id, *state))
    }

    /// Registers a new physical resource.
    ///
    /// A newly registered resource begins in `Available`.
    ///
    /// Registration is separate from hardware discovery. The caller should
    /// only register resources that have already been established as valid by
    /// the hardware/discovery layer.
    pub fn register(
        &mut self,
        resource: PhysicalQubitId,
    ) -> Result<(), PhysicalStateError> {
        if self.resources.contains_key(&resource) {
            return Err(PhysicalStateError::AlreadyRegistered { resource });
        }

        self.resources.insert(resource, PhysicalQubitState::new());

        self.bump_revision()?;

        Ok(())
    }

    /// Registers many resources transactionally.
    ///
    /// Duplicate IDs within the input are rejected and the store remains
    /// unchanged if any registration fails.
    pub fn register_many<I>(
        &mut self,
        resources: I,
    ) -> Result<(), PhysicalStateError>
    where
        I: IntoIterator<Item = PhysicalQubitId>,
    {
        let mut added = Vec::new();

        for resource in resources {
            if self.resources.contains_key(&resource)
                || added.iter().any(|existing| *existing == resource)
            {
                for id in added.into_iter().rev() {
                    self.resources.remove(&id);
                }

                return Err(PhysicalStateError::AlreadyRegistered { resource });
            }

            self.resources.insert(resource, PhysicalQubitState::new());
            added.push(resource);
        }

        if !added.is_empty() {
            let count = u64::try_from(added.len()).map_err(|_| {
                PhysicalStateError::RevisionOverflow
            })?;

            self.revision = self
                .revision
                .checked_add(count)
                .ok_or(PhysicalStateError::RevisionOverflow)?;
        }

        Ok(())
    }

    /// Transitions one resource to a new lifecycle state.
    ///
    /// Returns a transition record suitable for telemetry/history.
    pub fn transition(
        &mut self,
        resource: PhysicalQubitId,
        target: PhysicalQubitStatus,
    ) -> Result<PhysicalStateTransition, PhysicalStateError> {
        let state = self
            .resources
            .get_mut(&resource)
            .ok_or(PhysicalStateError::NotRegistered { resource })?;

        let before = *state;

        state.transition_to(target)?;

        // A no-op transition is not a state mutation and therefore does not
        // consume a global revision.
        if before == *state {
            return Ok(PhysicalStateTransition {
                resource,
                from: before.status,
                to: before.status,
                generation_before: before.generation,
                generation_after: before.generation,
                revision: self.revision,
            });
        }

        let next_revision = self
            .revision
            .checked_add(1)
            .ok_or(PhysicalStateError::RevisionOverflow)?;

        self.revision = next_revision;

        Ok(PhysicalStateTransition {
            resource,
            from: before.status,
            to: state.status,
            generation_before: before.generation,
            generation_after: state.generation,
            revision: self.revision,
        })
    }

    /// Transitions many resources atomically.
    ///
    /// If any transition fails, every state and the global revision are
    /// restored exactly to their pre-call values.
    ///
    /// The returned transition records are ordered exactly as the caller's
    /// input sequence.
    pub fn transition_many<I>(
        &mut self,
        transitions: I,
    ) -> Result<Vec<PhysicalStateTransition>, PhysicalStateError>
    where
        I: IntoIterator<Item = (PhysicalQubitId, PhysicalQubitStatus)>,
    {
        let original_revision = self.revision;
        let mut journal: Vec<(PhysicalQubitId, PhysicalQubitState)> = Vec::new();
        let mut records = Vec::new();

        for (resource, target) in transitions {
            let previous = match self.resources.get(&resource).copied() {
                Some(state) => state,
                None => {
                    self.rollback(&journal, original_revision);
                    return Err(PhysicalStateError::NotRegistered { resource });
                }
            };

            match self.transition(resource, target) {
                Ok(record) => {
                    if previous != self.resources[&resource] {
                        journal.push((resource, previous));
                    }

                    records.push(record);
                }
                Err(error) => {
                    self.rollback(&journal, original_revision);
                    return Err(error);
                }
            }
        }

        Ok(records)
    }

    /// Marks a resource as available after successful verification.
    ///
    /// This is a semantic convenience wrapper. It does not bypass transition
    /// validation.
    pub fn mark_available(
        &mut self,
        resource: PhysicalQubitId,
    ) -> Result<PhysicalStateTransition, PhysicalStateError> {
        self.transition(resource, PhysicalQubitStatus::Available)
    }

    /// Marks a resource as reserved.
    pub fn reserve(
        &mut self,
        resource: PhysicalQubitId,
    ) -> Result<PhysicalStateTransition, PhysicalStateError> {
        self.transition(resource, PhysicalQubitStatus::Reserved)
    }

    /// Marks a resource as actively executing.
    pub fn activate(
        &mut self,
        resource: PhysicalQubitId,
    ) -> Result<PhysicalStateTransition, PhysicalStateError> {
        self.transition(resource, PhysicalQubitStatus::Active)
    }

    /// Marks a resource as requiring recovery.
    pub fn require_recovery(
        &mut self,
        resource: PhysicalQubitId,
    ) -> Result<PhysicalStateTransition, PhysicalStateError> {
        self.transition(resource, PhysicalQubitStatus::RecoveryRequired)
    }

    /// Starts resource recovery.
    pub fn start_recovery(
        &mut self,
        resource: PhysicalQubitId,
    ) -> Result<PhysicalStateTransition, PhysicalStateError> {
        self.transition(resource, PhysicalQubitStatus::Recovering)
    }

    /// Moves a recovered resource into verification.
    pub fn begin_verification(
        &mut self,
        resource: PhysicalQubitId,
    ) -> Result<PhysicalStateTransition, PhysicalStateError> {
        self.transition(resource, PhysicalQubitStatus::Verifying)
    }

    /// Quarantines a resource.
    pub fn quarantine(
        &mut self,
        resource: PhysicalQubitId,
    ) -> Result<PhysicalStateTransition, PhysicalStateError> {
        self.transition(resource, PhysicalQubitStatus::Quarantined)
    }

    /// Retires a resource permanently.
    pub fn retire(
        &mut self,
        resource: PhysicalQubitId,
    ) -> Result<PhysicalStateTransition, PhysicalStateError> {
        self.transition(resource, PhysicalQubitStatus::Retired)
    }

    /// Removes a resource from the state store.
    ///
    /// Removal is deliberately restricted to already-retired resources.
    ///
    /// This prevents callers from using removal as a shortcut for recovery,
    /// which would otherwise destroy lifecycle history.
    pub fn remove_retired(
        &mut self,
        resource: PhysicalQubitId,
    ) -> Result<PhysicalQubitState, PhysicalStateError> {
        let state = self
            .resources
            .get(&resource)
            .copied()
            .ok_or(PhysicalStateError::NotRegistered { resource })?;

        if state.status != PhysicalQubitStatus::Retired {
            return Err(PhysicalStateError::RemovalRequiresRetired {
                resource,
                status: state.status,
            });
        }

        self.resources.remove(&resource);

        self.bump_revision()?;

        Ok(state)
    }

    /// Returns the number of currently allocatable resources.
    ///
    /// This is an O(n) query over tracked resources. The state store does not
    /// maintain a second mutable index, avoiding index consistency hazards.
    pub fn allocatable_count(&self) -> usize {
        self.resources
            .values()
            .filter(|state| state.is_allocatable())
            .count()
    }

    /// Returns whether at least one resource is currently allocatable.
    #[must_use]
    pub fn has_allocatable_resource(&self) -> bool {
        self.resources
            .values()
            .any(|state| state.is_allocatable())
    }

    /// Creates a deterministic snapshot.
    #[must_use]
    pub fn snapshot(&self) -> PhysicalStateSnapshot {
        PhysicalStateSnapshot {
            revision: self.revision,
            resources: self.resources.clone(),
        }
    }

    /// Restores a complete snapshot.
    ///
    /// The snapshot is assumed to have already passed through the type-safe
    /// construction performed by this module. The operation is still
    /// transactional from the caller's perspective because replacement occurs
    /// only after the snapshot has been moved into the store.
    pub fn restore(
        &mut self,
        snapshot: PhysicalStateSnapshot,
    ) -> Result<(), PhysicalStateError> {
        self.resources = snapshot.resources;
        self.revision = snapshot.revision;

        self.validate()
    }

    /// Restores a snapshot only if the current revision matches the expected
    /// revision.
    ///
    /// This provides optimistic concurrency control without introducing locks
    /// or global synchronization into this low-level state component.
    pub fn restore_if_revision(
        &mut self,
        expected_revision: u64,
        snapshot: PhysicalStateSnapshot,
    ) -> Result<(), PhysicalStateError> {
        if self.revision != expected_revision {
            return Err(PhysicalStateError::RevisionMismatch {
                expected: expected_revision,
                actual: self.revision,
            });
        }

        self.restore(snapshot)
    }

    /// Validates internal state invariants.
    ///
    /// This is intentionally cheap relative to external hardware validation.
    /// Hardware correctness remains the responsibility of the HAL and
    /// verification layers.
    pub fn validate(&self) -> Result<(), PhysicalStateError> {
        // BTreeMap itself guarantees unique keys. The explicit loop provides a
        // stable place for future invariants without changing the public API.
        for (resource, state) in &self.resources {
            let _ = resource;
            let _ = state;

            // All currently representable lifecycle values are valid.
            // Resource-specific hardware validity is intentionally not checked
            // here because this module has no authority over hardware.
        }

        Ok(())
    }

    fn bump_revision(&mut self) -> Result<(), PhysicalStateError> {
        self.revision = self
            .revision
            .checked_add(1)
            .ok_or(PhysicalStateError::RevisionOverflow)?;

        Ok(())
    }

    fn rollback(
        &mut self,
        journal: &[(PhysicalQubitId, PhysicalQubitState)],
        original_revision: u64,
    ) {
        for (resource, state) in journal.iter().rev() {
            self.resources.insert(*resource, *state);
        }

        self.revision = original_revision;
    }
}

impl Default for PhysicalStateStore {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the physical-resource resilience state store.
///
/// These errors are deliberately local to the state component. The higher
/// resilience error layer can classify/wrap them without forcing this low-level
/// file to depend on the complete resilience error hierarchy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalStateError {
    /// Resource was registered more than once.
    AlreadyRegistered {
        resource: PhysicalQubitId,
    },

    /// Operation referenced an unknown resource.
    NotRegistered {
        resource: PhysicalQubitId,
    },

    /// Requested lifecycle transition is not allowed.
    InvalidTransition {
        from: PhysicalQubitStatus,
        to: PhysicalQubitStatus,
    },

    /// A per-resource generation would overflow.
    GenerationOverflow,

    /// The store-wide revision would overflow.
    RevisionOverflow,

    /// Optimistic concurrency check failed.
    RevisionMismatch {
        expected: u64,
        actual: u64,
    },

    /// A resource cannot be removed until it is retired.
    RemovalRequiresRetired {
        resource: PhysicalQubitId,
        status: PhysicalQubitStatus,
    },
}

impl fmt::Display for PhysicalStateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyRegistered { resource } => {
                write!(formatter, "physical resource {resource} is already registered")
            }

            Self::NotRegistered { resource } => {
                write!(formatter, "physical resource {resource} is not registered")
            }

            Self::InvalidTransition { from, to } => {
                write!(
                    formatter,
                    "invalid physical-resource lifecycle transition: {from} -> {to}"
                )
            }

            Self::GenerationOverflow => {
                formatter.write_str("physical-resource generation overflow")
            }

            Self::RevisionOverflow => {
                formatter.write_str("physical-resource state revision overflow")
            }

            Self::RevisionMismatch { expected, actual } => {
                write!(
                    formatter,
                    "physical-resource state revision mismatch: expected {expected}, actual {actual}"
                )
            }

            Self::RemovalRequiresRetired { resource, status } => {
                write!(
                    formatter,
                    "physical resource {resource} cannot be removed while in status {status}; retirement is required"
                )
            }
        }
    }
}

impl Error for PhysicalStateError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn physical(index: usize) -> PhysicalQubitId {
        PhysicalQubitId::new(index)
    }

    #[test]
    fn new_store_is_empty_and_deterministic() {
        let store = PhysicalStateStore::new();

        assert_eq!(store.len(), 0);
        assert!(store.is_empty());
        assert_eq!(store.revision(), 0);
        assert!(!store.has_allocatable_resource());
    }

    #[test]
    fn registration_uses_canonical_physical_identity() {
        let mut store = PhysicalStateStore::new();
        let id = physical(37);

        store.register(id).expect("registration must succeed");

        assert!(store.contains(id));
        assert_eq!(
            store.get(id).expect("resource must exist").status(),
            PhysicalQubitStatus::Available
        );
    }

    #[test]
    fn identifiers_do_not_need_to_be_contiguous() {
        let mut store = PhysicalStateStore::new();

        store
            .register_many([physical(1), physical(10_000), physical(9_999_999)])
            .expect("registration must succeed");

        assert_eq!(store.len(), 3);
        assert!(store.contains(physical(1)));
        assert!(store.contains(physical(10_000)));
        assert!(store.contains(physical(9_999_999)));
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut store = PhysicalStateStore::new();
        let id = physical(4);

        store.register(id).expect("first registration must succeed");

        let error = store.register(id).expect_err("duplicate must fail");

        assert_eq!(
            error,
            PhysicalStateError::AlreadyRegistered { resource: id }
        );
    }

    #[test]
    fn transition_increments_generation_and_revision() {
        let mut store = PhysicalStateStore::new();
        let id = physical(2);

        store.register(id).expect("registration must succeed");

        let before_revision = store.revision();

        let transition = store
            .transition(id, PhysicalQubitStatus::Reserved)
            .expect("transition must succeed");

        assert_eq!(transition.resource(), id);
        assert_eq!(transition.from(), PhysicalQubitStatus::Available);
        assert_eq!(transition.to(), PhysicalQubitStatus::Reserved);
        assert_eq!(transition.generation_before(), 0);
        assert_eq!(transition.generation_after(), 1);
        assert_eq!(transition.revision(), before_revision + 1);
    }

    #[test]
    fn identical_transition_is_a_noop() {
        let mut store = PhysicalStateStore::new();
        let id = physical(2);

        store.register(id).expect("registration must succeed");

        let before_revision = store.revision();

        let transition = store
            .transition(id, PhysicalQubitStatus::Available)
            .expect("no-op transition must succeed");

        assert_eq!(transition.from(), PhysicalQubitStatus::Available);
        assert_eq!(transition.to(), PhysicalQubitStatus::Available);
        assert_eq!(transition.generation_before(), 0);
        assert_eq!(transition.generation_after(), 0);
        assert_eq!(transition.revision(), before_revision);
    }

    #[test]
    fn invalid_transition_is_rejected() {
        let mut store = PhysicalStateStore::new();
        let id = physical(3);

        store.register(id).expect("registration must succeed");

        let error = store
            .transition(id, PhysicalQubitStatus::Verifying)
            .expect_err("available -> verifying must be rejected");

        assert_eq!(
            error,
            PhysicalStateError::InvalidTransition {
                from: PhysicalQubitStatus::Available,
                to: PhysicalQubitStatus::Verifying,
            }
        );
    }

    #[test]
    fn recovery_requires_verification_before_availability() {
        let mut store = PhysicalStateStore::new();
        let id = physical(5);

        store.register(id).expect("registration must succeed");

        store
            .require_recovery(id)
            .expect("recovery requirement must succeed");

        store
            .start_recovery(id)
            .expect("recovery start must succeed");

        let error = store
            .mark_available(id)
            .expect_err("recovering -> available must be rejected");

        assert_eq!(
            error,
            PhysicalStateError::InvalidTransition {
                from: PhysicalQubitStatus::Recovering,
                to: PhysicalQubitStatus::Available,
            }
        );

        store
            .begin_verification(id)
            .expect("verification must succeed");

        store
            .mark_available(id)
            .expect("verified resource must become available");
    }

    #[test]
    fn quarantined_resource_cannot_return_directly_to_available() {
        let mut store = PhysicalStateStore::new();
        let id = physical(7);

        store.register(id).expect("registration must succeed");

        store
            .quarantine(id)
            .expect("quarantine must succeed");

        let error = store
            .mark_available(id)
            .expect_err("quarantine -> available must be rejected");

        assert_eq!(
            error,
            PhysicalStateError::InvalidTransition {
                from: PhysicalQubitStatus::Quarantined,
                to: PhysicalQubitStatus::Available,
            }
        );

        store
            .start_recovery(id)
            .expect("quarantined resource may enter recovery");

        store
            .begin_verification(id)
            .expect("recovery must be verified");

        store
            .mark_available(id)
            .expect("verified recovery may restore availability");
    }

    #[test]
    fn retired_is_terminal() {
        let mut store = PhysicalStateStore::new();
        let id = physical(8);

        store.register(id).expect("registration must succeed");
        store.retire(id).expect("retirement must succeed");

        assert!(store.get(id).expect("resource must exist").status().is_terminal());

        let error = store
            .transition(id, PhysicalQubitStatus::Available)
            .expect_err("retired resource must remain terminal");

        assert_eq!(
            error,
            PhysicalStateError::InvalidTransition {
                from: PhysicalQubitStatus::Retired,
                to: PhysicalQubitStatus::Available,
            }
        );
    }

    #[test]
    fn non_retired_resources_cannot_be_removed() {
        let mut store = PhysicalStateStore::new();
        let id = physical(9);

        store.register(id).expect("registration must succeed");

        let error = store
            .remove_retired(id)
            .expect_err("available resource must not be removed");

        assert_eq!(
            error,
            PhysicalStateError::RemovalRequiresRetired {
                resource: id,
                status: PhysicalQubitStatus::Available,
            }
        );

        assert!(store.contains(id));
    }

    #[test]
    fn retired_resource_can_be_removed() {
        let mut store = PhysicalStateStore::new();
        let id = physical(11);

        store.register(id).expect("registration must succeed");
        store.retire(id).expect("retirement must succeed");

        let state = store
            .remove_retired(id)
            .expect("retired resource may be removed");

        assert_eq!(state.status(), PhysicalQubitStatus::Retired);
        assert!(!store.contains(id));
    }

    #[test]
    fn batch_transition_is_atomic() {
        let mut store = PhysicalStateStore::new();

        store
            .register_many([physical(1), physical(2)])
            .expect("registration must succeed");

        let original = store.snapshot();

        let error = store
            .transition_many([
                (physical(1), PhysicalQubitStatus::Reserved),
                (physical(2), PhysicalQubitStatus::Verifying),
            ])
            .expect_err("second transition must fail");

        assert_eq!(
            error,
            PhysicalStateError::InvalidTransition {
                from: PhysicalQubitStatus::Available,
                to: PhysicalQubitStatus::Verifying,
            }
        );

        assert_eq!(store.snapshot(), original);
    }

    #[test]
    fn successful_batch_preserves_input_order() {
        let mut store = PhysicalStateStore::new();

        store
            .register_many([physical(1), physical(2)])
            .expect("registration must succeed");

        let records = store
            .transition_many([
                (physical(1), PhysicalQubitStatus::Reserved),
                (physical(2), PhysicalQubitStatus::Reserved),
            ])
            .expect("batch must succeed");

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].resource(), physical(1));
        assert_eq!(records[1].resource(), physical(2));
    }

    #[test]
    fn snapshots_are_deterministic() {
        let mut first = PhysicalStateStore::new();
        let mut second = PhysicalStateStore::new();

        first
            .register_many([physical(9), physical(1), physical(5)])
            .expect("registration must succeed");

        second
            .register_many([physical(9), physical(1), physical(5)])
            .expect("registration must succeed");

        assert_eq!(first.snapshot(), second.snapshot());
        assert_eq!(
            first.iter().collect::<Vec<_>>(),
            second.iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn snapshot_restore_round_trip() {
        let mut original = PhysicalStateStore::new();

        original
            .register_many([physical(1), physical(2)])
            .expect("registration must succeed");

        original
            .reserve(physical(1))
            .expect("reservation must succeed");

        let snapshot = original.snapshot();

        let mut restored = PhysicalStateStore::new();

        restored
            .restore(snapshot.clone())
            .expect("restore must succeed");

        assert_eq!(restored.snapshot(), snapshot);
    }

    #[test]
    fn optimistic_restore_rejects_stale_revision() {
        let mut store = PhysicalStateStore::new();

        store.register(physical(1)).expect("registration must succeed");

        let snapshot = store.snapshot();

        store
            .reserve(physical(1))
            .expect("reservation must succeed");

        let error = store
            .restore_if_revision(snapshot.revision(), snapshot)
            .expect_err("stale revision must be rejected");

        assert_eq!(
            error,
            PhysicalStateError::RevisionMismatch {
                expected: 1,
                actual: 2,
            }
        );
    }

    #[test]
    fn allocatable_count_tracks_lifecycle() {
        let mut store = PhysicalStateStore::new();

        store
            .register_many([physical(1), physical(2), physical(3)])
            .expect("registration must succeed");

        assert_eq!(store.allocatable_count(), 3);

        store
            .reserve(physical(1))
            .expect("reservation must succeed");

        store
            .quarantine(physical(2))
            .expect("quarantine must succeed");

        assert_eq!(store.allocatable_count(), 1);
        assert!(store.has_allocatable_resource());
    }

    #[test]
    fn unknown_resource_is_rejected() {
        let mut store = PhysicalStateStore::new();

        let error = store
            .activate(physical(99))
            .expect_err("unknown resource must fail");

        assert_eq!(
            error,
            PhysicalStateError::NotRegistered {
                resource: physical(99),
            }
        );
    }

    #[test]
    fn recovery_path_can_be_completed() {
        let mut store = PhysicalStateStore::new();
        let id = physical(12);

        store.register(id).expect("registration must succeed");
        store.activate(id).expect("activation must succeed");
        store
            .require_recovery(id)
            .expect("recovery requirement must succeed");
        store
            .start_recovery(id)
            .expect("recovery must start");
        store
            .begin_verification(id)
            .expect("verification must start");
        store
            .mark_available(id)
            .expect("verified resource must become available");

        assert_eq!(
            store.get(id).expect("resource must exist").status(),
            PhysicalQubitStatus::Available
        );
    }
}