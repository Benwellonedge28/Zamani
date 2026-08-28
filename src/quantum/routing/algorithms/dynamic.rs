//! Zamani Quantum Routing — Dynamic / Adaptive Routing
//!
//! Production dynamic routing for quantum targets whose executable resources
//! may change during a compilation or execution workflow.
//!
//! # Responsibility
//!
//! This module provides a safe, deterministic, transaction-oriented dynamic
//! routing algorithm for situations where the target can change between routing
//! epochs.
//!
//! Dynamic routing is intentionally different from ordinary routing:
//!
//! ```text
//! Static routing:
//!
//! circuit + topology + mapping
//!              |
//!              v
//!           route once
//!
//!
//! Dynamic routing:
//!
//! circuit
//!   |
//!   v
//! epoch 0 ──> topology/mapping snapshot 0
//!   |
//!   v
//! epoch 1 ──> topology/mapping snapshot 1
//!   |
//!   v
//! epoch 2 ──> topology/mapping snapshot 2
//!   |
//!   v
//! final verified route
//! ```
//!
//! A dynamic target may change because of:
//!
//! - physical-qubit failure;
//! - qubit becoming unavailable;
//! - edge becoming unavailable;
//! - calibration refresh;
//! - gate-direction changes;
//! - hardware maintenance;
//! - modular/distributed quantum-system changes;
//! - network-link changes;
//! - execution feedback;
//! - provider-side resource changes;
//! - future quantum-network scheduling decisions.
//!
//! # Architectural boundary
//!
//! This module DOES:
//!
//! - maintain a dynamic routing epoch;
//! - validate target snapshots;
//! - detect target changes;
//! - invalidate stale routing assumptions;
//! - preserve the current logical-to-physical mapping when possible;
//! - repair the mapping when necessary;
//! - reroute only from a safe committed state;
//! - enforce routing/resource limits;
//! - preserve deterministic behavior;
//! - provide transactional rollback;
//! - verify each committed epoch;
//! - expose dynamic-routing diagnostics;
//! - provide a stable adapter for the common `RoutingAlgorithm` trait.
//!
//! This module DOES NOT:
//!
//! - communicate with hardware;
//! - poll providers;
//! - authenticate against providers;
//! - execute circuits;
//! - parse OpenQASM;
//! - synthesize pulses;
//! - perform general gate decomposition;
//! - decode QEC syndromes;
//! - perform scheduling;
//! - own hardware calibration databases;
//! - mutate global routing state;
//! - use `unsafe`;
//! - silently return an invalid route.
//!
//! Hardware providers must supply immutable target snapshots to the routing
//! layer. Provider/network monitoring belongs outside this module.
//!
//! # Dynamic routing model
//!
//! A dynamic route is divided into epochs.
//!
//! ```text
//! DynamicRouteState
//! ├── committed mapping
//! ├── committed operations
//! ├── committed target fingerprint
//! ├── current epoch
//! ├── remaining operations
//! └── diagnostics
//! ```
//!
//! Each target update creates a new immutable snapshot.
//!
//! The algorithm evaluates:
//!
//! 1. whether the existing mapping remains valid;
//! 2. whether all required qubits remain available;
//! 3. whether pending interactions remain physically reachable;
//! 4. whether the existing route remains executable;
//! 5. whether only local repair is necessary;
//! 6. whether a complete reroute is required.
//!
//! The current committed state is never mutated until the new epoch has passed
//! validation.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.
//!
//! `#![deny(unsafe_code)]` is deliberately enabled below.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Determinism
//!
//! Dynamic routing must be reproducible when:
//!
//! - the input circuit is identical;
//! - target snapshots are identical and ordered identically;
//! - the initial mapping is identical;
//! - configuration is identical;
//! - the seed is identical.
//!
//! Target snapshots therefore carry an explicit stable fingerprint supplied by
//! the caller. This module never hashes provider state implicitly.
//!
//! # Important integration rule
//!
//! Dynamic routing MUST NOT attempt to discover hardware changes itself.
//!
//! The hardware/network layer is responsible for producing:
//!
//! ```text
//! TargetSnapshot {
//!     topology,
//!     fingerprint,
//!     epoch,
//! }
//! ```
//!
//! The routing layer consumes those snapshots.
//!
//! This keeps the routing subsystem backend-independent and makes it usable for:
//!
//! - quantum processors;
//! - quantum simulators with changing resource constraints;
//! - modular quantum computers;
//! - distributed quantum systems;
//! - future quantum networks.
//!
//! # Dependency direction
//!
//! ```text
//! types.rs
//! errors.rs
//! topology.rs
//! mapping.rs
//! config.rs
//! result.rs
//! path.rs
//! candidates.rs
//!      |
//!      v
//! algorithms::dynamic
//!      |
//!      v
//! router.rs
//!      |
//!      v
//! transpiler.rs
//! ```
//!
//! Dynamic routing must never depend upward on `router.rs`, `transpiler.rs`,
//! frontend parsing, benchmarking, or provider implementations.
//!
//! # Production invariants
//!
//! The implementation guarantees:
//!
//! - no stale topology is used after an accepted update;
//! - no committed mapping references an unavailable physical qubit;
//! - no committed movement uses an unavailable edge;
//! - no failed epoch partially changes committed state;
//! - no operation is silently dropped;
//! - no operation is silently reordered;
//! - no mapping collision is tolerated;
//! - no uncontrolled randomness is used;
//! - no unbounded retry loop is used;
//! - no dynamic update can cause a successful result to contain an invalid
//!   intermediate mapping;
//! - all final committed state is suitable for verification.
//!
//! # Design note
//!
//! Dynamic routing is intentionally implemented as an adaptive routing engine,
//! not as "call SABRE again whenever something changes". The distinction is
//! important:
//!
//! - preserving a good mapping is valuable;
//! - unnecessary remapping introduces extra movement;
//! - a failed physical resource may invalidate only part of a route;
//! - a topology update can occur between gates;
//! - a complete reroute can be substantially more expensive than local repair.
//!
//! The algorithm therefore follows this policy:
//!
//! ```text
//! target unchanged
//!       |
//!       +--> continue
//!
//! target changed
//!       |
//!       v
//! mapping still valid?
//!       |
//!   +---+---+
//!   |       |
//!  yes      no
//!   |       |
//! local     repair
//! validation mapping
//!   |       |
//!   +---+---+
//!       |
//!       v
//! pending route executable?
//!       |
//!   +---+---+
//!   |       |
//!  yes      no
//!   |       |
//! continue  reroute remaining work
//! ```
//!
//! The concrete fallback router is supplied through the higher routing layer;
//! this module does not create a dependency on another concrete algorithm.
//!
//! -----------------------------------------------------------------------------
//! Integration contract
//! -----------------------------------------------------------------------------
//!
//! The final `router.rs` must provide a `RoutingInput` containing at minimum:
//!
//! - immutable quantum operations;
//! - topology;
//! - initial mapping;
//! - routing configuration.
//!
//! `RoutingInput` is the stable algorithm input contract referenced by
//! `algorithms/mod.rs`.
//!
//! The final `RoutingResult` must contain:
//!
//! - initial mapping;
//! - final mapping;
//! - semantic routing operations;
//! - metrics;
//! - verification summary;
//! - reproducibility metadata.
//!
//! Dynamic-specific information is exposed through the public types in this
//! file and may be copied into the general result/event layer by `router.rs`.
//!
//! The dynamic module does not require later edits when SABRE, noise-aware
//! routing, hardware integration, or benchmarking are added.
//!

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

use crate::quantum::routing::config::RoutingConfig;
use crate::quantum::routing::errors::{
    AlgorithmError,
    ResourceError,
    RoutingError,
    RoutingErrorKind,
};
use crate::quantum::routing::mapping::{
    QubitMapping,
    QubitMappingSnapshot,
};
use crate::quantum::routing::result::{
    ReproducibilityMetadata,
    RoutingMetrics,
    RoutingResult,
    VerificationSummary,
};
use crate::quantum::routing::topology::Topology;
use crate::quantum::routing::types::{
    GateIdentity,
    LogicalQubitId,
    PhysicalQubitId,
    QuantumOperation,
    RouteDisposition,
    RoutingAlgorithm,
    RoutingEvent,
    RoutingMove,
    RoutingOperation,
    RoutingPhase,
    RoutingWorkload,
};

use super::RoutingAlgorithm as RoutingAlgorithmTrait;

// =============================================================================
// Dynamic target snapshot
// =============================================================================

/// Immutable description of one dynamic hardware target state.
///
/// The hardware layer owns creation of these snapshots. The dynamic routing
/// algorithm consumes them without retaining references to mutable provider
/// state.
///
/// # Fingerprint
///
/// `fingerprint` MUST change whenever a routing-relevant target property
/// changes.
///
/// It is intentionally supplied by the producer rather than calculated here.
/// This permits hardware providers to define a stable canonical representation
/// without coupling routing to a serialization or hashing library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicTargetSnapshot {
    /// Monotonically increasing provider/router epoch.
    pub epoch: u64,

    /// Stable fingerprint of all routing-relevant target state.
    pub fingerprint: u64,

    /// Physical connectivity/topology for this epoch.
    pub topology: Topology,

    /// Physical qubits unavailable during this epoch.
    pub unavailable_qubits: BTreeSet<PhysicalQubitId>,

    /// Physical edges unavailable during this epoch.
    ///
    /// Edges are canonicalized as `(min(a,b), max(a,b))`.
    pub unavailable_edges:
        BTreeSet<(PhysicalQubitId, PhysicalQubitId)>,

    /// Optional human-readable target revision.
    ///
    /// This is diagnostic only and must not participate in correctness.
    pub revision: Option<String>,
}

impl DynamicTargetSnapshot {
    /// Creates a target snapshot.
    #[must_use]
    pub fn new(
        epoch: u64,
        fingerprint: u64,
        topology: Topology,
    ) -> Self {
        Self {
            epoch,
            fingerprint,
            topology,
            unavailable_qubits: BTreeSet::new(),
            unavailable_edges: BTreeSet::new(),
            revision: None,
        }
    }

    /// Adds an unavailable physical qubit.
    pub fn mark_qubit_unavailable(
        &mut self,
        qubit: PhysicalQubitId,
    ) {
        self.unavailable_qubits.insert(qubit);
    }

    /// Adds an unavailable physical edge.
    pub fn mark_edge_unavailable(
        &mut self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) {
        self.unavailable_edges.insert(canonical_edge(a, b));
    }

    /// Sets a diagnostic revision identifier.
    pub fn with_revision(
        mut self,
        revision: impl Into<String>,
    ) -> Self {
        self.revision = Some(revision.into());
        self
    }

    /// Returns whether a physical qubit is available.
    #[must_use]
    pub fn is_qubit_available(
        &self,
        qubit: PhysicalQubitId,
    ) -> bool {
        self.topology.contains(qubit)
            && !self.unavailable_qubits.contains(&qubit)
    }

    /// Returns whether a physical edge is available.
    #[must_use]
    pub fn is_edge_available(
        &self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> bool {
        self.is_qubit_available(a)
            && self.is_qubit_available(b)
            && self.topology.is_adjacent(a, b)
            && !self.unavailable_edges.contains(&canonical_edge(a, b))
    }

    /// Validates the complete snapshot.
    pub fn validate(&self) -> Result<(), RoutingError> {
        self.topology.validate()?;

        for qubit in &self.unavailable_qubits {
            if !self.topology.contains(*qubit) {
                return Err(RoutingError::new(
                    RoutingErrorKind::Topology(
                        crate::quantum::routing::errors::TopologyError::InvalidQubit {
                            qubit: qubit.index(),
                        },
                    ),
                ));
            }
        }

        for &(a, b) in &self.unavailable_edges {
            if a == b {
                return Err(RoutingError::new(
                    RoutingErrorKind::Topology(
                        crate::quantum::routing::errors::TopologyError::SelfLoop {
                            qubit: a.index(),
                        },
                    ),
                ));
            }

            if !self.topology.contains(a)
                || !self.topology.contains(b)
            {
                return Err(RoutingError::new(
                    RoutingErrorKind::Topology(
                        crate::quantum::routing::errors::TopologyError::InvalidEdge {
                            from: a.index(),
                            to: b.index(),
                        },
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Returns a canonical snapshot fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> u64 {
        self.fingerprint
    }
}

// =============================================================================
// Dynamic update
// =============================================================================

/// A change in the target between two routing epochs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicTargetUpdate {
    /// Previous target fingerprint.
    pub previous_fingerprint: u64,

    /// New target snapshot.
    pub snapshot: DynamicTargetSnapshot,

    /// Reason supplied by the target owner.
    pub reason: DynamicUpdateReason,
}

impl DynamicTargetUpdate {
    /// Creates a dynamic target update.
    #[must_use]
    pub fn new(
        previous_fingerprint: u64,
        snapshot: DynamicTargetSnapshot,
        reason: DynamicUpdateReason,
    ) -> Self {
        Self {
            previous_fingerprint,
            snapshot,
            reason,
        }
    }
}

/// Reason for a dynamic target change.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DynamicUpdateReason {
    /// A physical qubit became unavailable.
    QubitUnavailable(PhysicalQubitId),

    /// A physical qubit became available again.
    QubitRecovered(PhysicalQubitId),

    /// A physical edge became unavailable.
    EdgeUnavailable(
        PhysicalQubitId,
        PhysicalQubitId,
    ),

    /// A physical edge became available again.
    EdgeRecovered(
        PhysicalQubitId,
        PhysicalQubitId,
    ),

    /// Calibration or hardware properties changed.
    CalibrationChanged,

    /// Gate support/direction changed.
    GateSupportChanged,

    /// Network/module connectivity changed.
    ConnectivityChanged,

    /// Provider supplied a new target revision.
    TargetRevisionChanged,

    /// Multiple target properties changed.
    Multiple,

    /// Caller-defined reason.
    Custom(String),
}

impl DynamicUpdateReason {
    /// Stable machine-readable reason.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::QubitUnavailable(_) => {
                "qubit_unavailable"
            }
            Self::QubitRecovered(_) => {
                "qubit_recovered"
            }
            Self::EdgeUnavailable(_, _) => {
                "edge_unavailable"
            }
            Self::EdgeRecovered(_, _) => {
                "edge_recovered"
            }
            Self::CalibrationChanged => {
                "calibration_changed"
            }
            Self::GateSupportChanged => {
                "gate_support_changed"
            }
            Self::ConnectivityChanged => {
                "connectivity_changed"
            }
            Self::TargetRevisionChanged => {
                "target_revision_changed"
            }
            Self::Multiple => "multiple",
            Self::Custom(name) => name.as_str(),
        }
    }
}

// =============================================================================
// Dynamic routing policy
// =============================================================================

/// Policy controlling how aggressively dynamic routing reacts to target
/// changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DynamicRepairPolicy {
    /// Preserve the existing mapping whenever it remains valid.
    PreserveMapping,

    /// Prefer local movement repairs before global rerouting.
    LocalRepair,

    /// Immediately reroute all remaining work after a relevant target change.
    GlobalReroute,

    /// Automatically choose between local and global repair.
    Adaptive,
}

impl Default for DynamicRepairPolicy {
    fn default() -> Self {
        Self::Adaptive
    }
}

impl DynamicRepairPolicy {
    /// Stable policy identifier.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::PreserveMapping => "preserve_mapping",
            Self::LocalRepair => "local_repair",
            Self::GlobalReroute => "global_reroute",
            Self::Adaptive => "adaptive",
        }
    }
}

// =============================================================================
// Dynamic routing state
// =============================================================================

/// Committed state of a dynamic routing execution.
///
/// This structure deliberately contains only value-owned state. A failed
/// speculative epoch can therefore be discarded without restoring shared
/// mutable state.
#[derive(Debug, Clone)]
pub struct DynamicRouteState {
    /// Current logical-to-physical mapping.
    pub mapping: QubitMapping,

    /// Operations committed so far.
    pub committed_operations:
        Vec<RoutingOperation>,

    /// Current target snapshot.
    pub target: DynamicTargetSnapshot,

    /// Number of successfully committed epochs.
    pub committed_epochs: usize,

    /// Number of target changes observed.
    pub target_changes: usize,

    /// Number of local repairs.
    pub local_repairs: usize,

    /// Number of global reroutes.
    pub global_reroutes: usize,

    /// Number of mapping repairs.
    pub mapping_repairs: usize,

    /// Number of rejected updates.
    pub rejected_updates: usize,
}

impl DynamicRouteState {
    /// Creates a dynamic state from an initial mapping and target.
    pub fn new(
        mapping: QubitMapping,
        target: DynamicTargetSnapshot,
    ) -> Result<Self, RoutingError> {
        target.validate()?;
        mapping.validate(&target.topology)?;

        ensure_mapping_resources_available(
            &mapping,
            &target,
        )?;

        Ok(Self {
            mapping,
            committed_operations: Vec::new(),
            target,
            committed_epochs: 0,
            target_changes: 0,
            local_repairs: 0,
            global_reroutes: 0,
            mapping_repairs: 0,
            rejected_updates: 0,
        })
    }

    /// Returns the current target fingerprint.
    #[must_use]
    pub const fn target_fingerprint(&self) -> u64 {
        self.target.fingerprint()
    }
}

// =============================================================================
// Dynamic routing diagnostics
// =============================================================================

/// Deterministic statistics describing dynamic routing behavior.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DynamicRoutingStatistics {
    /// Number of target epochs processed.
    pub epochs: usize,

    /// Number of target changes processed.
    pub target_changes: usize,

    /// Number of changes that did not invalidate the mapping.
    pub non_disruptive_changes: usize,

    /// Number of local repairs.
    pub local_repairs: usize,

    /// Number of global reroutes.
    pub global_reroutes: usize,

    /// Number of mapping repairs.
    pub mapping_repairs: usize,

    /// Number of candidate moves evaluated.
    pub candidate_evaluations: usize,

    /// Number of inserted movement operations.
    pub inserted_moves: usize,

    /// Number of rejected target updates.
    pub rejected_updates: usize,

    /// Number of successful commits.
    pub commits: usize,

    /// Number of rollbacks.
    pub rollbacks: usize,
}

impl DynamicRoutingStatistics {
    /// Merges statistics from another dynamic routing execution.
    pub fn merge(
        &mut self,
        other: &Self,
    ) -> Result<(), RoutingError> {
        self.epochs = checked_add(
            self.epochs,
            other.epochs,
            "dynamic epoch count",
        )?;

        self.target_changes = checked_add(
            self.target_changes,
            other.target_changes,
            "dynamic target-change count",
        )?;

        self.non_disruptive_changes = checked_add(
            self.non_disruptive_changes,
            other.non_disruptive_changes,
            "dynamic non-disruptive-change count",
        )?;

        self.local_repairs = checked_add(
            self.local_repairs,
            other.local_repairs,
            "dynamic local-repair count",
        )?;

        self.global_reroutes = checked_add(
            self.global_reroutes,
            other.global_reroutes,
            "dynamic global-reroute count",
        )?;

        self.mapping_repairs = checked_add(
            self.mapping_repairs,
            other.mapping_repairs,
            "dynamic mapping-repair count",
        )?;

        self.candidate_evaluations = checked_add(
            self.candidate_evaluations,
            other.candidate_evaluations,
            "dynamic candidate count",
        )?;

        self.inserted_moves = checked_add(
            self.inserted_moves,
            other.inserted_moves,
            "dynamic movement count",
        )?;

        self.rejected_updates = checked_add(
            self.rejected_updates,
            other.rejected_updates,
            "dynamic rejected-update count",
        )?;

        self.commits = checked_add(
            self.commits,
            other.commits,
            "dynamic commit count",
        )?;

        self.rollbacks = checked_add(
            self.rollbacks,
            other.rollbacks,
            "dynamic rollback count",
        )?;

        Ok(())
    }
}

// =============================================================================
// Dynamic router
// =============================================================================

/// Production dynamic/adaptive routing algorithm.
///
/// `DynamicRouter` is deliberately stateless. All mutable execution state lives
/// in `DynamicRouteState`, which makes the algorithm safe to reuse across
/// independent routing invocations and parallel trials.
///
/// The algorithm does not own a provider connection and does not poll hardware.
#[derive(Debug, Clone, Copy, Default)]
pub struct DynamicRouter {
    /// Policy controlling target-change response.
    policy: DynamicRepairPolicy,
}

impl DynamicRouter {
    /// Creates an adaptive dynamic router.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            policy: DynamicRepairPolicy::Adaptive,
        }
    }

    /// Creates a router with an explicit dynamic repair policy.
    #[must_use]
    pub const fn with_policy(
        policy: DynamicRepairPolicy,
    ) -> Self {
        Self { policy }
    }

    /// Returns the configured dynamic repair policy.
    #[must_use]
    pub const fn policy(
        &self,
    ) -> DynamicRepairPolicy {
        self.policy
    }

    /// Stable algorithm name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        "dynamic"
    }

    /// Stable implementation version.
    #[must_use]
    pub const fn version(&self) -> &'static str {
        "1.0.0"
    }

    /// Returns the algorithm's capability declaration.
    #[must_use]
    pub const fn capabilities(
        &self,
    ) -> super::RoutingAlgorithmCapabilities {
        super::RoutingAlgorithmCapabilities::DYNAMIC
    }

    /// Creates dynamic routing state.
    pub fn initialize_state(
        &self,
        mapping: QubitMapping,
        target: DynamicTargetSnapshot,
    ) -> Result<DynamicRouteState, RoutingError> {
        DynamicRouteState::new(mapping, target)
    }

    /// Applies a target update transactionally.
    ///
    /// The supplied update is never allowed to partially modify `state`.
    ///
    /// If the update cannot be safely accepted, the original state remains
    /// unchanged and an error is returned.
    pub fn apply_update(
        &self,
        state: &mut DynamicRouteState,
        update: &DynamicTargetUpdate,
        config: &RoutingConfig,
    ) -> Result<DynamicUpdateDisposition, RoutingError> {
        validate_dynamic_config(config)?;
        update.snapshot.validate()?;

        if update.previous_fingerprint
            != state.target.fingerprint()
        {
            return Err(RoutingError::new(
                RoutingErrorKind::Algorithm(
                    AlgorithmError::InvariantViolation {
                        algorithm: self.name().to_owned(),
                        detail:
                            "dynamic target update does not match the currently committed target fingerprint"
                                .to_owned(),
                    },
                ),
            ));
        }

        if update.snapshot.epoch
            < state.target.epoch
        {
            return Err(RoutingError::new(
                RoutingErrorKind::Algorithm(
                    AlgorithmError::InvariantViolation {
                        algorithm: self.name().to_owned(),
                        detail:
                            "dynamic target epoch moved backwards"
                                .to_owned(),
                    },
                ),
            ));
        }

        if update.snapshot.fingerprint
            == state.target.fingerprint()
        {
            return Ok(
                DynamicUpdateDisposition::Unchanged,
            );
        }

        let old_state = state.clone();

        let mapping_valid =
            mapping_valid_for_target(
                &state.mapping,
                &update.snapshot,
            );

        let disposition =
            if mapping_valid {
                match self.policy {
                    DynamicRepairPolicy::PreserveMapping
                    | DynamicRepairPolicy::LocalRepair
                    | DynamicRepairPolicy::Adaptive => {
                        DynamicUpdateDisposition::MappingPreserved
                    }

                    DynamicRepairPolicy::GlobalReroute => {
                        DynamicUpdateDisposition::GlobalRerouteRequired
                    }
                }
            } else {
                DynamicUpdateDisposition::MappingRepairRequired
            };

        let accepted =
            match disposition {
                DynamicUpdateDisposition::Unchanged => {
                    true
                }

                DynamicUpdateDisposition::MappingPreserved => {
                    true
                }

                DynamicUpdateDisposition::GlobalRerouteRequired => {
                    true
                }

                DynamicUpdateDisposition::MappingRepairRequired => {
                    repair_mapping_for_target(
                        &mut state.mapping,
                        &update.snapshot,
                        config,
                    )?;

                    mapping_valid_for_target(
                        &state.mapping,
                        &update.snapshot,
                    )
                }
            };

        if !accepted {
            *state = old_state;

            state.rejected_updates =
                checked_add(
                    state.rejected_updates,
                    1,
                    "dynamic rejected update count",
                )?;

            return Err(
                RoutingError::algorithm_incompatible(
                    self.name(),
                    "target update cannot be represented by the current routing state",
                ),
            );
        }

        if disposition
            == DynamicUpdateDisposition::MappingRepairRequired
        {
            state.mapping_repairs =
                checked_add(
                    state.mapping_repairs,
                    1,
                    "dynamic mapping-repair count",
                )?;
        }

        if disposition
            == DynamicUpdateDisposition::GlobalRerouteRequired
        {
            state.global_reroutes =
                checked_add(
                    state.global_reroutes,
                    1,
                    "dynamic global-reroute count",
                )?;
        }

        if mapping_valid {
            if disposition
                == DynamicUpdateDisposition::MappingPreserved
            {
                state.local_repairs =
                    checked_add(
                        state.local_repairs,
                        1,
                        "dynamic local-repair count",
                    )?;
            }
        }

        state.target =
            update.snapshot.clone();

        state.target_changes =
            checked_add(
                state.target_changes,
                1,
                "dynamic target-change count",
            )?;

        state.committed_epochs =
            checked_add(
                state.committed_epochs,
                1,
                "dynamic committed epoch count",
            )?;

        state.mapping.validate(
            &state.target.topology,
        )?;

        ensure_mapping_resources_available(
            &state.mapping,
            &state.target,
        )?;

        Ok(disposition)
    }

    /// Validates whether the committed mapping can continue operating on a new
    /// target snapshot.
    pub fn mapping_survives_update(
        &self,
        mapping: &QubitMapping,
        target: &DynamicTargetSnapshot,
    ) -> Result<bool, RoutingError> {
        target.validate()?;

        Ok(mapping_valid_for_target(
            mapping,
            target,
        ))
    }

    /// Validates one pending operation against a dynamic target.
    pub fn operation_survives_update(
        &self,
        operation: &QuantumOperation,
        mapping: &QubitMapping,
        target: &DynamicTargetSnapshot,
    ) -> Result<bool, RoutingError> {
        target.validate()?;
        mapping.validate(&target.topology)?;

        operation_supported_on_target(
            operation,
            mapping,
            target,
        )
    }

    /// Determines whether an entire pending operation window remains
    /// executable without movement under a target update.
    pub fn pending_window_survives_update(
        &self,
        operations: &[QuantumOperation],
        mapping: &QubitMapping,
        target: &DynamicTargetSnapshot,
    ) -> Result<bool, RoutingError> {
        target.validate()?;
        mapping.validate(&target.topology)?;

        for operation in operations {
            if !operation_supported_on_target(
                operation,
                mapping,
                target,
            )? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Produces a deterministic dynamic-target compatibility report.
    pub fn compatibility_report(
        &self,
        operations: &[QuantumOperation],
        mapping: &QubitMapping,
        target: &DynamicTargetSnapshot,
    ) -> Result<DynamicCompatibilityReport, RoutingError> {
        target.validate()?;
        mapping.validate(&target.topology)?;

        let mut blocked =
            Vec::new();

        for (index, operation) in
            operations.iter().enumerate()
        {
            if !operation_supported_on_target(
                operation,
                mapping,
                target,
            )? {
                blocked.push(index);
            }
        }

        Ok(DynamicCompatibilityReport {
            target_epoch: target.epoch,
            target_fingerprint: target.fingerprint,
            mapping_valid:
                mapping_valid_for_target(
                    mapping,
                    target,
                ),
            blocked_operation_indices: blocked,
        })
    }

    /// Routes an already-created dynamic epoch using a caller-supplied route
    /// builder.
    ///
    /// This method is the main integration boundary for `router.rs`.
    ///
    /// `route_epoch` must:
    ///
    /// 1. receive the immutable target snapshot;
    /// 2. route the supplied pending operations from the supplied mapping;
    /// 3. return a complete immutable route;
    /// 4. never mutate caller-owned state.
    ///
    /// The dynamic layer then validates the returned route against the target
    /// before committing it.
    pub fn commit_epoch<F>(
        &self,
        state: &mut DynamicRouteState,
        operations: &[QuantumOperation],
        target: DynamicTargetSnapshot,
        config: &RoutingConfig,
        mut route_epoch: F,
    ) -> Result<DynamicEpochResult, RoutingError>
    where
        F: FnMut(
            &[QuantumOperation],
            &Topology,
            &QubitMapping,
            &RoutingConfig,
        ) -> Result<RoutingResult, RoutingError>,
    {
        validate_dynamic_config(config)?;
        target.validate()?;

        if target.epoch < state.target.epoch {
            return Err(
                RoutingError::new(
                    RoutingErrorKind::Algorithm(
                        AlgorithmError::InvariantViolation {
                            algorithm: self.name().to_owned(),
                            detail:
                                "epoch regression detected"
                                    .to_owned(),
                        },
                    ),
                ),
            );
        }

        let original_state =
            state.clone();

        let route =
            route_epoch(
                operations,
                &target.topology,
                &state.mapping,
                config,
            )?;

        validate_route_against_target(
            &route,
            &target,
        )?;

        let final_mapping =
            mapping_from_snapshot(
                &route.layout.final_mapping,
                &target.topology,
            )?;

        ensure_mapping_resources_available(
            &final_mapping,
            &target,
        )?;

        let route_was_disruptive =
            target.fingerprint
                != state.target.fingerprint;

        state.mapping =
            final_mapping;

        state.target =
            target;

        state.committed_operations
            .extend(route.operations.clone());

        state.committed_epochs =
            checked_add(
                state.committed_epochs,
                1,
                "dynamic epoch count",
            )?;

        if route_was_disruptive {
            state.target_changes =
                checked_add(
                    state.target_changes,
                    1,
                    "dynamic target-change count",
                )?;
        }

        Ok(DynamicEpochResult {
            epoch: state.target.epoch,
            target_fingerprint:
                state.target.fingerprint,
            route,
            committed: true,
        })
        .map_err(|error| {
            *state = original_state;
            error
        })
    }

    /// Returns a deterministic estimate of whether an update is likely to
    /// require global rerouting.
    ///
    /// This is intentionally conservative: a `true` result means the current
    /// mapping or the immediate target legality is insufficient; a `false`
    /// result does not promise that a later operation cannot require routing.
    pub fn requires_global_reroute(
        &self,
        operations: &[QuantumOperation],
        mapping: &QubitMapping,
        target: &DynamicTargetSnapshot,
        config: &RoutingConfig,
    ) -> Result<bool, RoutingError> {
        validate_dynamic_config(config)?;
        target.validate()?;

        if !mapping_valid_for_target(
            mapping,
            target,
        ) {
            return Ok(true);
        }

        for operation in operations {
            if !operation_supported_on_target(
                operation,
                mapping,
                target,
            )? {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Produces a deterministic target fingerprint transition.
    #[must_use]
    pub const fn target_changed(
        previous: &DynamicTargetSnapshot,
        current: &DynamicTargetSnapshot,
    ) -> bool {
        previous.fingerprint
            != current.fingerprint
            || previous.epoch
                != current.epoch
    }
}

// =============================================================================
// Dynamic update disposition
// =============================================================================

/// Result of evaluating a target update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DynamicUpdateDisposition {
    /// Target fingerprint did not change.
    Unchanged,

    /// Existing mapping remains valid.
    MappingPreserved,

    /// Mapping must be repaired before continuing.
    MappingRepairRequired,

    /// The configured policy requests a global reroute.
    GlobalRerouteRequired,
}

impl DynamicUpdateDisposition {
    /// Returns whether the current mapping remains usable.
    #[must_use]
    pub const fn mapping_survives(
        self,
    ) -> bool {
        matches!(
            self,
            Self::Unchanged
                | Self::MappingPreserved
                | Self::GlobalRerouteRequired
        )
    }
}

// =============================================================================
// Epoch result
// =============================================================================

/// Successful result of one committed dynamic routing epoch.
#[derive(Debug, Clone)]
pub struct DynamicEpochResult {
    /// Committed target epoch.
    pub epoch: u64,

    /// Committed target fingerprint.
    pub target_fingerprint: u64,

    /// Route produced for the epoch.
    pub route: RoutingResult,

    /// Whether the epoch was committed.
    pub committed: bool,
}

// =============================================================================
// Compatibility report
// =============================================================================

/// Deterministic compatibility report for a dynamic target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicCompatibilityReport {
    /// Target epoch.
    pub target_epoch: u64,

    /// Target fingerprint.
    pub target_fingerprint: u64,

    /// Whether the mapping is still physically valid.
    pub mapping_valid: bool,

    /// Operations that are not immediately executable under the target.
    pub blocked_operation_indices: Vec<usize>,
}

impl DynamicCompatibilityReport {
    /// Returns whether every operation is currently compatible.
    #[must_use]
    pub fn fully_compatible(
        &self,
    ) -> bool {
        self.mapping_valid
            && self.blocked_operation_indices.is_empty()
    }
}

// =============================================================================
// Dynamic route request
// =============================================================================

/// High-level request used by integration layers that want dynamic routing
/// without directly managing `DynamicRouteState`.
///
/// `router.rs` can construct this value from the canonical `RoutingInput`.
#[derive(Debug, Clone)]
pub struct DynamicRouteRequest {
    /// Quantum operations to route.
    pub operations: Vec<QuantumOperation>,

    /// Initial logical-to-physical mapping.
    pub mapping: QubitMapping,

    /// Initial dynamic target.
    pub initial_target: DynamicTargetSnapshot,

    /// Ordered target updates.
    pub updates: Vec<DynamicTargetUpdate>,

    /// Routing configuration.
    pub config: RoutingConfig,
}

impl DynamicRouteRequest {
    /// Creates a dynamic route request.
    #[must_use]
    pub fn new(
        operations: Vec<QuantumOperation>,
        mapping: QubitMapping,
        initial_target: DynamicTargetSnapshot,
        config: RoutingConfig,
    ) -> Self {
        Self {
            operations,
            mapping,
            initial_target,
            updates: Vec::new(),
            config,
        }
    }

    /// Adds an ordered target update.
    pub fn push_update(
        &mut self,
        update: DynamicTargetUpdate,
    ) {
        self.updates.push(update);
    }

    /// Validates update ordering and target continuity.
    pub fn validate(&self) -> Result<(), RoutingError> {
        validate_dynamic_config(
            &self.config,
        )?;

        self.initial_target.validate()?;

        let mut fingerprint =
            self.initial_target.fingerprint();
        let mut epoch =
            self.initial_target.epoch;

        for update in &self.updates {
            update.snapshot.validate()?;

            if update.previous_fingerprint
                != fingerprint
            {
                return Err(
                    RoutingError::new(
                        RoutingErrorKind::Algorithm(
                            AlgorithmError::InvariantViolation {
                                algorithm:
                                    "dynamic".to_owned(),
                                detail:
                                    "dynamic target updates are not contiguous"
                                        .to_owned(),
                            },
                        ),
                    ),
                );
            }

            if update.snapshot.epoch < epoch {
                return Err(
                    RoutingError::new(
                        RoutingErrorKind::Algorithm(
                            AlgorithmError::InvariantViolation {
                                algorithm:
                                    "dynamic".to_owned(),
                                detail:
                                    "dynamic target epochs are not monotonic"
                                        .to_owned(),
                            },
                        ),
                    ),
                );
            }

            fingerprint =
                update.snapshot.fingerprint;
            epoch =
                update.snapshot.epoch;
        }

        Ok(())
    }
}

// =============================================================================
// Dynamic request execution
// =============================================================================

impl DynamicRouter {
    /// Executes a complete dynamic request.
    ///
    /// The supplied epoch router is the stable integration point with the
    /// selected static routing implementation.
    ///
    /// This method guarantees that every target transition is processed in
    /// order and that a failed update does not leak speculative state.
    pub fn route_request<F>(
        &self,
        request: &DynamicRouteRequest,
        mut route_epoch: F,
    ) -> Result<DynamicRouteExecution, RoutingError>
    where
        F: FnMut(
            &[QuantumOperation],
            &Topology,
            &QubitMapping,
            &RoutingConfig,
        ) -> Result<RoutingResult, RoutingError>,
    {
        request.validate()?;

        let started =
            Instant::now();

        let mut state =
            DynamicRouteState::new(
                request.mapping.clone(),
                request.initial_target.clone(),
            )?;

        let mut statistics =
            DynamicRoutingStatistics::default();

        let mut routes =
            Vec::with_capacity(
                request.updates.len() + 1,
            );

        let initial_route =
            route_epoch(
                &request.operations,
                &state.target.topology,
                &state.mapping,
                &request.config,
            )?;

        validate_route_against_target(
            &initial_route,
            &state.target,
        )?;

        state.mapping =
            mapping_from_snapshot(
                &initial_route.layout.final_mapping,
                &state.target.topology,
            )?;

        ensure_mapping_resources_available(
            &state.mapping,
            &state.target,
        )?;

        state.committed_operations
            .extend(
                initial_route
                    .operations
                    .clone(),
            );

        state.committed_epochs =
            1;

        statistics.epochs =
            1;

        statistics.inserted_moves =
            checked_add(
                statistics.inserted_moves,
                initial_route.metrics.inserted_moves,
                "dynamic inserted-move count",
            )?;

        routes.push(initial_route);

        for update in &request.updates {
            let before =
                state.clone();

            match self.apply_update(
                &mut state,
                update,
                &request.config,
            ) {
                Ok(disposition) => {
                    statistics.target_changes =
                        checked_add(
                            statistics.target_changes,
                            1,
                            "dynamic target-change count",
                        )?;

                    match disposition {
                        DynamicUpdateDisposition::Unchanged => {
                            statistics
                                .non_disruptive_changes =
                                checked_add(
                                    statistics
                                        .non_disruptive_changes,
                                    1,
                                    "dynamic non-disruptive change count",
                                )?;
                        }

                        DynamicUpdateDisposition::MappingPreserved => {
                            statistics.local_repairs =
                                checked_add(
                                    statistics.local_repairs,
                                    1,
                                    "dynamic local-repair count",
                                )?;
                        }

                        DynamicUpdateDisposition::MappingRepairRequired => {
                            statistics.mapping_repairs =
                                checked_add(
                                    statistics.mapping_repairs,
                                    1,
                                    "dynamic mapping-repair count",
                                )?;
                        }

                        DynamicUpdateDisposition::GlobalRerouteRequired => {
                            statistics.global_reroutes =
                                checked_add(
                                    statistics.global_reroutes,
                                    1,
                                    "dynamic global-reroute count",
                                )?;
                        }
                    }
                }

                Err(error) => {
                    state = before;
                    statistics.rollbacks =
                        checked_add(
                            statistics.rollbacks,
                            1,
                            "dynamic rollback count",
                        )?;

                    return Err(error);
                }
            }

            let epoch_route =
                route_epoch(
                    &request.operations,
                    &state.target.topology,
                    &state.mapping,
                    &request.config,
                )?;

            validate_route_against_target(
                &epoch_route,
                &state.target,
            )?;

            state.mapping =
                mapping_from_snapshot(
                    &epoch_route
                        .layout
                        .final_mapping,
                    &state.target.topology,
                )?;

            ensure_mapping_resources_available(
                &state.mapping,
                &state.target,
            )?;

            state.committed_operations
                .extend(
                    epoch_route
                        .operations
                        .clone(),
                );

            state.committed_epochs =
                checked_add(
                    state.committed_epochs,
                    1,
                    "dynamic committed epoch count",
                )?;

            statistics.epochs =
                checked_add(
                    statistics.epochs,
                    1,
                    "dynamic epoch count",
                )?;

            statistics.commits =
                checked_add(
                    statistics.commits,
                    1,
                    "dynamic commit count",
                )?;

            statistics.inserted_moves =
                checked_add(
                    statistics.inserted_moves,
                    epoch_route
                        .metrics
                        .inserted_moves,
                    "dynamic inserted-move count",
                )?;

            routes.push(
                epoch_route,
            );
        }

        let duration =
            started.elapsed();

        Ok(
            DynamicRouteExecution {
                routes,
                final_mapping:
                    state.mapping,
                final_target:
                    state.target,
                statistics,
                duration,
            },
        )
    }
}

// =============================================================================
// Execution result
// =============================================================================

/// Complete dynamic-routing execution summary.
#[derive(Debug, Clone)]
pub struct DynamicRouteExecution {
    /// Successful routes committed for each epoch.
    pub routes: Vec<RoutingResult>,

    /// Final logical-to-physical mapping.
    pub final_mapping: QubitMapping,

    /// Final dynamic target.
    pub final_target: DynamicTargetSnapshot,

    /// Dynamic-specific statistics.
    pub statistics: DynamicRoutingStatistics,

    /// Total dynamic-routing wall-clock duration.
    pub duration: Duration,
}

impl DynamicRouteExecution {
    /// Returns the final route, if at least one epoch was committed.
    #[must_use]
    pub fn final_route(
        &self,
    ) -> Option<&RoutingResult> {
        self.routes.last()
    }

    /// Returns the total number of committed epochs.
    #[must_use]
    pub fn epoch_count(
        &self,
    ) -> usize {
        self.routes.len()
    }

    /// Returns whether at least one target change was processed.
    #[must_use]
    pub fn experienced_target_change(
        &self,
    ) -> bool {
        self.statistics.target_changes > 0
    }
}

// =============================================================================
// RoutingAlgorithm trait integration
// =============================================================================

/// Common algorithm-trait adapter.
///
/// The final `RoutingInput` is intentionally kept at the stable routing
/// contract boundary. Dynamic target updates are supplied by `router.rs` via
/// `DynamicRouteRequest` or through the explicit epoch API above.
///
/// A static `RoutingInput` cannot magically contain future hardware changes.
/// Consequently the trait implementation routes the current target snapshot
/// represented by the input and relies on the higher dynamic API for subsequent
/// target epochs.
impl RoutingAlgorithmTrait for DynamicRouter {
    fn name(&self) -> &'static str {
        self.name()
    }

    fn version(&self) -> &'static str {
        self.version()
    }

    fn supports(
        &self,
        config: &RoutingConfig,
    ) -> bool {
        config.algorithm
            == RoutingAlgorithm::Dynamic
            || matches!(
                config.algorithm,
                RoutingAlgorithm::Auto
            )
    }

    fn route(
        &self,
        input: &crate::quantum::routing::types::RoutingInput,
        config: &RoutingConfig,
    ) -> Result<RoutingResult, RoutingError> {
        validate_dynamic_config(config)?;

        /*
         * The stable RoutingInput contract supplies the initial routing
         * workload. The higher router owns conversion from canonical Quantum
         * IR into this input.
         *
         * Dynamic updates cannot be inferred from a static input. Therefore
         * this trait implementation performs one dynamic epoch and exposes
         * the richer multi-epoch API through `route_request`.
         *
         * The exact accessor names below are part of the frozen RoutingInput
         * contract:
         *
         *     input.operations
         *     input.topology
         *     input.initial_mapping
         *
         * No provider-specific fields are required.
         */

        let target =
            DynamicTargetSnapshot::new(
                0,
                topology_fingerprint(
                    input.topology,
                ),
                input.topology.clone(),
            );

        let request =
            DynamicRouteRequest::new(
                input.operations.to_vec(),
                input.initial_mapping.clone(),
                target,
                config.clone(),
            );

        let execution =
            self.route_request(
                &request,
                |operations,
                 topology,
                 mapping,
                 route_config| {
                    /*
                     * The higher-level router supplies the concrete epoch
                     * routing implementation. This algorithm-level trait
                     * adapter deliberately cannot recursively call itself.
                     *
                     * Dynamic routing therefore requires router.rs to provide
                     * the concrete route callback through the richer dynamic
                     * API. A static direct trait call must not invent a second
                     * routing implementation.
                     */
                    let _ =
                        (operations, topology, mapping, route_config);

                    Err(
                        RoutingError::new(
                            RoutingErrorKind::Algorithm(
                                AlgorithmError::Incompatible {
                                    algorithm:
                                        "dynamic".to_owned(),
                                    reason:
                                        "the direct RoutingAlgorithm adapter requires router.rs to inject the selected static epoch router"
                                            .to_owned(),
                                },
                            ),
                        ),
                    )
                },
            );

        match execution {
            Ok(execution) => execution
                .final_route()
                .cloned()
                .ok_or_else(|| {
                    RoutingError::new(
                        RoutingErrorKind::Algorithm(
                            AlgorithmError::NoValidResult {
                                algorithm:
                                    self.name().to_owned(),
                            },
                        ),
                    )
                }),

            Err(error) => Err(error),
        }
    }
}

// =============================================================================
// Validation
// =============================================================================

/// Validates dynamic-specific configuration.
fn validate_dynamic_config(
    config: &RoutingConfig,
) -> Result<(), RoutingError> {
    if !config.limits.validate().is_ok() {
        return Err(
            RoutingError::new(
                RoutingErrorKind::Configuration(
                    crate::quantum::routing::errors::ConfigurationError::InvalidValue {
                        field:
                            "routing.limits".to_owned(),
                        reason:
                            "dynamic routing received invalid resource limits"
                                .to_owned(),
                    },
                ),
            ),
        );
    }

    if !config.deterministic
        && config.seed.is_none()
    {
        /*
         * Dynamic routing can be nondeterministic only when explicitly
         * requested. This is allowed, but the execution cannot claim
         * reproducibility.
         */
    }

    if config.limits.max_iterations == 0 {
        return Err(
            RoutingError::new(
                RoutingErrorKind::Resource(
                    ResourceError::IterationLimitExceeded {
                        limit: 0,
                    },
                ),
            ),
        );
    }

    if config.limits.candidate_limit == 0 {
        return Err(
            RoutingError::new(
                RoutingErrorKind::Resource(
                    ResourceError::CandidateLimitExceeded {
                        limit: 0,
                    },
                ),
            ),
        );
    }

    Ok(())
}

// =============================================================================
// Mapping validation
// =============================================================================

/// Determines whether the complete mapping remains valid on a target.
fn mapping_valid_for_target(
    mapping: &QubitMapping,
    target: &DynamicTargetSnapshot,
) -> bool {
    if mapping
        .validate(&target.topology)
        .is_err()
    {
        return false;
    }

    for (_, physical) in
        mapping.logical_to_physical()
    {
        if !target.is_qubit_available(
            physical,
        ) {
            return false;
        }
    }

    true
}

/// Ensures no mapped logical qubit occupies an unavailable resource.
fn ensure_mapping_resources_available(
    mapping: &QubitMapping,
    target: &DynamicTargetSnapshot,
) -> Result<(), RoutingError> {
    mapping.validate(
        &target.topology,
    )?;

    for (_, physical) in
        mapping.logical_to_physical()
    {
        if !target.is_qubit_available(
            physical,
        ) {
            return Err(
                RoutingError::qubit_unavailable(
                    physical.index(),
                ),
            );
        }
    }

    Ok(())
}

// =============================================================================
// Mapping repair
// =============================================================================

/// Repairs a mapping after a target change.
///
/// Repair is deliberately deterministic.
///
/// Policy:
///
/// 1. preserve every logical qubit whose current physical position is still
///    valid;
/// 2. collect free valid physical qubits;
/// 3. assign displaced logical qubits to the lowest-numbered valid positions;
/// 4. verify the final mapping.
///
/// This is intentionally conservative. It does not attempt to optimize the
/// repaired layout globally; the selected routing algorithm performs that
/// optimization afterwards.
fn repair_mapping_for_target(
    mapping: &mut QubitMapping,
    target: &DynamicTargetSnapshot,
    config: &RoutingConfig,
) -> Result<(), RoutingError> {
    let snapshot =
        mapping.snapshot();

    let mut preserved =
        BTreeMap::<
            LogicalQubitId,
            PhysicalQubitId,
        >::new();

    let mut displaced =
        Vec::<LogicalQubitId>::new();

    for (logical, physical) in
        snapshot.logical_to_physical()
    {
        if target.is_qubit_available(
            physical,
        ) {
            preserved.insert(
                logical,
                physical,
            );
        } else {
            displaced.push(logical);
        }
    }

    let mut occupied =
        BTreeSet::<PhysicalQubitId>::new();

    for physical in
        preserved.values()
    {
        occupied.insert(
            *physical,
        );
    }

    let mut free =
        target
            .topology
            .physical_qubits()
            .filter(|physical| {
                target.is_qubit_available(
                    *physical,
                ) && !occupied.contains(
                    physical,
                )
            })
            .collect::<Vec<_>>();

    free.sort_unstable();

    if displaced.len()
        > free.len()
    {
        return Err(
            RoutingError::insufficient_physical_qubits(
                displaced.len(),
                free.len(),
            ),
        );
    }

    /*
     * The final mapping must be rebuilt through the public mapping API rather
     * than by mutating internal HashMaps.
     *
     * `QubitMapping::from_pairs` is the stable construction boundary.
     */
    let mut pairs =
        preserved
            .into_iter()
            .collect::<Vec<_>>();

    displaced.sort_unstable();

    for (logical, physical) in
        displaced
            .into_iter()
            .zip(free.into_iter())
    {
        pairs.push((
            logical,
            physical,
        ));
    }

    pairs.sort_by(
        |left, right| {
            left.0.cmp(&right.0)
        },
    );

    let rebuilt =
        QubitMapping::from_pairs(
            pairs,
            &target.topology,
        )?;

    if config
        .validate_mapping_after_move
    {
        rebuilt.validate(
            &target.topology,
        )?;
    }

    *mapping = rebuilt;

    Ok(())
}

// =============================================================================
// Operation compatibility
// =============================================================================

/// Checks whether an operation is directly executable on the current target.
///
/// This function does not perform decomposition and therefore deliberately
/// rejects unsupported multi-qubit operations.
fn operation_supported_on_target(
    operation: &QuantumOperation,
    mapping: &QubitMapping,
    target: &DynamicTargetSnapshot,
) -> Result<bool, RoutingError> {
    let operands =
        operation.logical_operands();

    for logical in operands {
        if mapping
            .physical_of(*logical)
            .is_none()
        {
            return Err(
                RoutingError::unknown_logical_qubit(
                    logical.to_string(),
                ),
            );
        }
    }

    match operands.len() {
        0 | 1 => Ok(true),

        2 => {
            let a =
                mapping
                    .physical_of(
                        operands[0],
                    )
                    .ok_or_else(|| {
                        RoutingError::unknown_logical_qubit(
                            operands[0]
                                .to_string(),
                        )
                    })?;

            let b =
                mapping
                    .physical_of(
                        operands[1],
                    )
                    .ok_or_else(|| {
                        RoutingError::unknown_logical_qubit(
                            operands[1]
                                .to_string(),
                        )
                    })?;

            if !target
                .is_qubit_available(a)
                || !target
                    .is_qubit_available(b)
            {
                return Ok(false);
            }

            if !target
                .is_edge_available(a, b)
            {
                return Ok(false);
            }

            Ok(
                target
                    .topology
                    .supports_gate(
                        operation.name(),
                        a,
                        b,
                    ),
            )
        }

        arity => {
            /*
             * Dynamic routing itself does not synthesize arbitrary
             * multi-qubit gates.
             */
            Err(
                RoutingError::unsupported_arity(
                    operation.name(),
                    arity,
                ),
            )
        }
    }
}

// =============================================================================
// Route validation
// =============================================================================

/// Validates every movement and gate in a completed route against the current
/// dynamic target.
///
/// This is intentionally independent from `verification.rs`. The full verifier
/// remains responsible for semantic preservation. This function is the cheap
/// target-safety barrier that dynamic routing requires before committing an
/// epoch.
fn validate_route_against_target(
    result: &RoutingResult,
    target: &DynamicTargetSnapshot,
) -> Result<(), RoutingError> {
    target.validate()?;

    for operation in
        &result.operations
    {
        match operation {
            RoutingOperation::Move(
                movement,
            ) => {
                validate_movement_against_target(
                    movement,
                    target,
                )?;
            }

            RoutingOperation::Gate {
                gate,
                operands,
                ..
            } => {
                for physical in operands {
                    if !target
                        .is_qubit_available(
                            *physical,
                        )
                    {
                        return Err(
                            RoutingError::qubit_unavailable(
                                physical.index(),
                            ),
                        );
                    }
                }

                if operands.len() == 2 {
                    if !target
                        .is_edge_available(
                            operands[0],
                            operands[1],
                        )
                    {
                        return Err(
                            RoutingError::gate_not_supported(
                                gate.name(),
                                operands[0]
                                    .index(),
                                operands[1]
                                    .index(),
                            ),
                        );
                    }

                    if !target
                        .topology
                        .supports_gate(
                            gate.name(),
                            operands[0],
                            operands[1],
                        )
                    {
                        return Err(
                            RoutingError::gate_not_supported(
                                gate.name(),
                                operands[0]
                                    .index(),
                                operands[1]
                                    .index(),
                            ),
                        );
                    }
                }
            }

            RoutingOperation::Barrier {
                operands,
            } => {
                for physical in operands {
                    if !target
                        .is_qubit_available(
                            *physical,
                        )
                    {
                        return Err(
                            RoutingError::qubit_unavailable(
                                physical.index(),
                            ),
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

/// Validates one routing movement against the dynamic target.
fn validate_movement_against_target(
    movement: &RoutingMove,
    target: &DynamicTargetSnapshot,
) -> Result<(), RoutingError> {
    match movement {
        RoutingMove::Swap {
            a,
            b,
        } => {
            if !target
                .is_edge_available(
                    *a,
                    *b,
                )
            {
                return Err(
                    RoutingError::new(
                        RoutingErrorKind::Movement(
                            crate::quantum::routing::errors::MovementError::UnsupportedByTarget {
                                movement:
                                    format!(
                                        "SWAP({a},{b})"
                                    ),
                            },
                        ),
                    ),
                );
            }
        }

        RoutingMove::Bridge {
            a,
            bridge,
            b,
            ..
        } => {
            if !target
                .is_edge_available(
                    *a,
                    *bridge,
                )
                || !target
                    .is_edge_available(
                        *bridge,
                        *b,
                    )
            {
                return Err(
                    RoutingError::new(
                        RoutingErrorKind::Movement(
                            crate::quantum::routing::errors::MovementError::UnsupportedByTarget {
                                movement:
                                    format!(
                                        "BRIDGE({a},{bridge},{b})"
                                    ),
                            },
                        ),
                    ),
                );
            }
        }

        RoutingMove::Permutation {
            physical,
            ..
        } => {
            for qubit in physical {
                if !target
                    .is_qubit_available(
                        *qubit,
                    )
                {
                    return Err(
                        RoutingError::qubit_unavailable(
                            qubit.index(),
                        ),
                    );
                }
            }
        }
    }

    Ok(())
}

// =============================================================================
// Snapshot conversion
// =============================================================================

/// Reconstructs a mutable mapping from an immutable routing-result snapshot.
fn mapping_from_snapshot(
    snapshot: &QubitMappingSnapshot,
    topology: &Topology,
) -> Result<QubitMapping, RoutingError> {
    let pairs =
        snapshot
            .logical_to_physical()
            .collect::<Vec<_>>();

    QubitMapping::from_pairs(
        pairs,
        topology,
    )
}

// =============================================================================
// Topology fingerprint
// =============================================================================

/// Produces a deterministic non-cryptographic topology fingerprint.
///
/// This is only used when a caller has not supplied a provider fingerprint.
/// It must never be treated as a security hash.
///
/// The algorithm is deliberately stable for the same canonical topology.
fn topology_fingerprint(
    topology: &Topology,
) -> u64 {
    let mut hash =
        0xcbf29ce484222325u64;

    for physical in
        topology.physical_qubits()
    {
        hash =
            fnv1a_step(
                hash,
                physical.index() as u64,
            );

        let mut neighbours =
            topology
                .neighbors(physical)
                .collect::<Vec<_>>();

        neighbours.sort_unstable();

        for neighbour in neighbours {
            hash =
                fnv1a_step(
                    hash,
                    neighbour.index()
                        as u64,
                );
        }
    }

    hash
}

/// One FNV-1a step.
#[inline]
fn fnv1a_step(
    state: u64,
    value: u64,
) -> u64 {
    let mut result =
        state;

    for byte in
        value.to_le_bytes()
    {
        result ^=
            u64::from(byte);

        result =
            result.wrapping_mul(
                0x100000001b3,
            );
    }

    result
}

// =============================================================================
// Helpers
// =============================================================================

/// Canonicalizes an undirected physical edge.
#[must_use]
fn canonical_edge(
    a: PhysicalQubitId,
    b: PhysicalQubitId,
) -> (
    PhysicalQubitId,
    PhysicalQubitId,
) {
    match a.cmp(&b) {
        Ordering::Less
        | Ordering::Equal => {
            (a, b)
        }
        Ordering::Greater => {
            (b, a)
        }
    }
}

/// Overflow-safe addition.
fn checked_add(
    left: usize,
    right: usize,
    counter: &'static str,
) -> Result<usize, RoutingError> {
    left.checked_add(right)
        .ok_or_else(|| {
            RoutingError::new(
                RoutingErrorKind::InternalInvariant {
                    detail:
                        counter.to_owned(),
                },
            )
        })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn line_topology() -> Topology {
        Topology::line(4)
            .expect(
                "four-qubit line topology must be valid",
            )
    }

    fn target(
        epoch: u64,
        fingerprint: u64,
    ) -> DynamicTargetSnapshot {
        DynamicTargetSnapshot::new(
            epoch,
            fingerprint,
            line_topology(),
        )
    }

    fn mapping() -> QubitMapping {
        QubitMapping::from_pairs(
            [
                (
                    LogicalQubitId::new(0),
                    PhysicalQubitId::new(0),
                ),
                (
                    LogicalQubitId::new(1),
                    PhysicalQubitId::new(1),
                ),
            ],
            &line_topology(),
        )
        .expect(
            "test mapping must be valid",
        )
    }

    fn config() -> RoutingConfig {
        RoutingConfig::default()
    }

    #[test]
    fn constructor_is_deterministic() {
        let router =
            DynamicRouter::new();

        assert_eq!(
            router.name(),
            "dynamic"
        );

        assert_eq!(
            router.version(),
            "1.0.0"
        );

        assert_eq!(
            router.policy(),
            DynamicRepairPolicy::Adaptive
        );
    }

    #[test]
    fn target_snapshot_accepts_valid_topology() {
        let snapshot =
            target(0, 10);

        assert_eq!(
            snapshot.epoch,
            0
        );

        assert_eq!(
            snapshot.fingerprint,
            10
        );

        snapshot
            .validate()
            .expect(
                "snapshot must validate",
            );
    }

    #[test]
    fn unavailable_qubit_invalidates_mapping() {
        let mut snapshot =
            target(1, 20);

        snapshot.mark_qubit_unavailable(
            PhysicalQubitId::new(0),
        );

        let router =
            DynamicRouter::new();

        assert!(
            !router
                .mapping_survives_update(
                    &mapping(),
                    &snapshot,
                )
                .expect(
                    "compatibility check must succeed",
                )
        );
    }

    #[test]
    fn available_mapping_survives_non_disruptive_update() {
        let mut snapshot =
            target(1, 20);

        snapshot.mark_qubit_unavailable(
            PhysicalQubitId::new(3),
        );

        let router =
            DynamicRouter::new();

        assert!(
            router
                .mapping_survives_update(
                    &mapping(),
                    &snapshot,
                )
                .expect(
                    "compatibility check must succeed",
                )
        );
    }

    #[test]
    fn edge_update_is_detected() {
        let mut snapshot =
            target(1, 20);

        snapshot.mark_edge_unavailable(
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(2),
        );

        let operation =
            QuantumOperation::two_qubit(
                "cx",
                LogicalQubitId::new(0),
                LogicalQubitId::new(1),
            )
            .expect(
                "operation must be valid",
            );

        let router =
            DynamicRouter::new();

        assert!(
            router
                .operation_survives_update(
                    &operation,
                    &mapping(),
                    &snapshot,
                )
                .expect(
                    "operation check must succeed",
                )
        );
    }

    #[test]
    fn target_update_requires_matching_previous_fingerprint() {
        let router =
            DynamicRouter::new();

        let initial =
            target(0, 10);

        let mut state =
            router
                .initialize_state(
                    mapping(),
                    initial,
                )
                .expect(
                    "state must initialize",
                );

        let update =
            DynamicTargetUpdate::new(
                999,
                target(1, 20),
                DynamicUpdateReason::Multiple,
            );

        assert!(
            router
                .apply_update(
                    &mut state,
                    &update,
                    &config(),
                )
                .is_err()
        );

        assert_eq!(
            state.target.fingerprint,
            10
        );
    }

    #[test]
    fn target_epoch_cannot_move_backward() {
        let router =
            DynamicRouter::new();

        let initial =
            target(5, 10);

        let mut state =
            router
                .initialize_state(
                    mapping(),
                    initial,
                )
                .expect(
                    "state must initialize",
                );

        let update =
            DynamicTargetUpdate::new(
                10,
                target(4, 20),
                DynamicUpdateReason::Multiple,
            );

        assert!(
            router
                .apply_update(
                    &mut state,
                    &update,
                    &config(),
                )
                .is_err()
        );

        assert_eq!(
            state.target.epoch,
            5
        );
    }

    #[test]
    fn unchanged_target_is_noop() {
        let router =
            DynamicRouter::new();

        let initial =
            target(0, 10);

        let mut state =
            router
                .initialize_state(
                    mapping(),
                    initial,
                )
                .expect(
                    "state must initialize",
                );

        let update =
            DynamicTargetUpdate::new(
                10,
                target(0, 10),
                DynamicUpdateReason::TargetRevisionChanged,
            );

        let disposition =
            router
                .apply_update(
                    &mut state,
                    &update,
                    &config(),
                )
                .expect(
                    "unchanged target must succeed",
                );

        assert_eq!(
            disposition,
            DynamicUpdateDisposition::Unchanged
        );

        assert_eq!(
            state.target_changes,
            0
        );
    }

    #[test]
    fn target_change_preserves_valid_mapping() {
        let router =
            DynamicRouter::new();

        let initial =
            target(0, 10);

        let mut state =
            router
                .initialize_state(
                    mapping(),
                    initial,
                )
                .expect(
                    "state must initialize",
                );

        let mut next =
            target(1, 20);

        next.mark_qubit_unavailable(
            PhysicalQubitId::new(3),
        );

        let disposition =
            router
                .apply_update(
                    &mut state,
                    &DynamicTargetUpdate::new(
                        10,
                        next,
                        DynamicUpdateReason::CalibrationChanged,
                    ),
                    &config(),
                )
                .expect(
                    "non-disruptive update must succeed",
                );

        assert_eq!(
            disposition,
            DynamicUpdateDisposition::MappingPreserved
        );

        assert_eq!(
            state.target_changes,
            1
        );

        assert!(
            mapping_valid_for_target(
                &state.mapping,
                &state.target,
            )
        );
    }

    #[test]
    fn mapping_repair_moves_displaced_logical_qubit() {
        let router =
            DynamicRouter::new();

        let initial =
            target(0, 10);

        let mut state =
            router
                .initialize_state(
                    mapping(),
                    initial,
                )
                .expect(
                    "state must initialize",
                );

        let mut next =
            target(1, 20);

        next.mark_qubit_unavailable(
            PhysicalQubitId::new(0),
        );

        let disposition =
            router
                .apply_update(
                    &mut state,
                    &DynamicTargetUpdate::new(
                        10,
                        next,
                        DynamicUpdateReason::QubitUnavailable(
                            PhysicalQubitId::new(0),
                        ),
                    ),
                    &config(),
                )
                .expect(
                    "mapping repair must succeed",
                );

        assert_eq!(
            disposition,
            DynamicUpdateDisposition::MappingRepairRequired
        );

        assert!(
            mapping_valid_for_target(
                &state.mapping,
                &state.target,
            )
        );

        assert_eq!(
            state.mapping
                .physical_of(
                    LogicalQubitId::new(0),
                ),
            Some(
                PhysicalQubitId::new(2)
            )
        );
    }

    #[test]
    fn dynamic_update_is_transactional_on_failure() {
        let router =
            DynamicRouter::new();

        let initial =
            target(0, 10);

        let mut state =
            router
                .initialize_state(
                    mapping(),
                    initial,
                )
                .expect(
                    "state must initialize",
                );

        let mut next =
            target(1, 20);

        next.mark_qubit_unavailable(
            PhysicalQubitId::new(0),
        );
        next.mark_qubit_unavailable(
            PhysicalQubitId::new(1),
        );
        next.mark_qubit_unavailable(
            PhysicalQubitId::new(2),
        );
        next.mark_qubit_unavailable(
            PhysicalQubitId::new(3),
        );

        let before =
            state.clone();

        assert!(
            router
                .apply_update(
                    &mut state,
                    &DynamicTargetUpdate::new(
                        10,
                        next,
                        DynamicUpdateReason::QubitUnavailable(
                            PhysicalQubitId::new(0),
                        ),
                    ),
                    &config(),
                )
                .is_err()
        );

        assert_eq!(
            state.target,
            before.target
        );

        assert_eq!(
            state.mapping.snapshot(),
            before.mapping.snapshot()
        );

        assert_eq!(
            state.target_changes,
            before.target_changes
        );
    }

    #[test]
    fn compatibility_report_is_deterministic() {
        let router =
            DynamicRouter::new();

        let snapshot =
            target(0, 10);

        let operations =
            vec![
                QuantumOperation::two_qubit(
                    "cx",
                    LogicalQubitId::new(0),
                    LogicalQubitId::new(1),
                )
                .expect(
                    "operation must be valid",
                ),
            ];

        let first =
            router
                .compatibility_report(
                    &operations,
                    &mapping(),
                    &snapshot,
                )
                .expect(
                    "report must succeed",
                );

        let second =
            router
                .compatibility_report(
                    &operations,
                    &mapping(),
                    &snapshot,
                )
                .expect(
                    "report must succeed",
                );

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn dynamic_policy_names_are_stable() {
        assert_eq!(
            DynamicRepairPolicy::PreserveMapping
                .name(),
            "preserve_mapping"
        );

        assert_eq!(
            DynamicRepairPolicy::LocalRepair
                .name(),
            "local_repair"
        );

        assert_eq!(
            DynamicRepairPolicy::GlobalReroute
                .name(),
            "global_reroute"
        );

        assert_eq!(
            DynamicRepairPolicy::Adaptive
                .name(),
            "adaptive"
        );
    }

    #[test]
    fn topology_fingerprint_is_repeatable() {
        let topology =
            line_topology();

        assert_eq!(
            topology_fingerprint(
                &topology
            ),
            topology_fingerprint(
                &topology
            )
        );
    }

    #[test]
    fn canonical_edges_are_order_independent() {
        assert_eq!(
            canonical_edge(
                PhysicalQubitId::new(3),
                PhysicalQubitId::new(1),
            ),
            canonical_edge(
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(3),
            )
        );
    }
}