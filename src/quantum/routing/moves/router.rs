//! Zamani Quantum Routing — Movement Router
//!
//! Production-grade movement execution layer for the quantum routing subsystem.
//!
//! # Purpose
//!
//! This module is the authoritative execution boundary for routing *moves*.
//!
//! Routing algorithms decide:
//!
//! ```text
//! "Move the logical state currently at physical p2 to p1."
//! ```
//!
//! This module is responsible for turning that decision into a validated,
//! transactional routing-state transition:
//!
//! ```text
//! Routing algorithm
//!       │
//!       │ proposed RoutingMove
//!       ▼
//! ┌──────────────────────────────┐
//! │        MovementRouter        │
//! │                              │
//! │ validate move                │
//! │ validate topology            │
//! │ validate mapping             │
//! │ validate policy              │
//! │ apply mapping permutation    │
//! │ record semantic operation    │
//! │ update metrics               │
//! │ commit / rollback            │
//! └──────────────────────────────┘
//!       │
//!       ▼
//! Updated QubitMapping
//!
//! # Architectural responsibility
//!
//! This file owns:
//!
//! - validation of routing movement requests;
//! - SWAP movement execution;
//! - bridge movement validation;
//! - permutation movement execution;
//! - atomic movement batches;
//! - movement transactions;
//! - movement history;
//! - movement statistics;
//! - mapping snapshots for rollback;
//! - movement-limit enforcement;
//! - deterministic movement recording;
//! - movement-level invariant checking;
//! - conversion of valid movement requests into `RoutingOperation` values;
//! - separation between semantic movement and later hardware lowering.
//!
//! This file does NOT own:
//!
//! - initial layout;
//! - routing algorithm selection;
//! - shortest-path search;
//! - SABRE;
//! - lookahead;
//! - noise-aware search;
//! - topology construction;
//! - hardware-provider communication;
//! - gate decomposition;
//! - pulse generation;
//! - scheduling;
//! - quantum simulation;
//! - QEC decoding;
//! - OpenQASM parsing;
//! - compiler IR parsing.
//!
//! Those responsibilities belong to other Zamani subsystems.
//!
//! # Critical semantic rule
//!
//! A routing SWAP is a **logical-state permutation**, not necessarily a native
//! hardware SWAP gate.
//!
//! Therefore this module records:
//!
//! ```text
//! RoutingOperation::Swap
//! ```
//!
//! rather than immediately lowering the operation to:
//!
//! ```text
//! CX + CX + CX
//! ```
//!
//! Hardware lowering belongs downstream.
//!
//! # Transactionality
//!
//! Every public mutation operation is transactional.
//!
//! If any validation or mutation step fails:
//!
//! ```text
//! mapping        -> restored
//! operation log  -> restored
//! metrics        -> restored
//! history        -> restored
//! ```
//!
//! No partially applied movement is observable through the successful API.
//!
//! # Determinism
//!
//! This module never uses hash-map iteration order to make a routing decision.
//!
//! Movement endpoints are canonicalized only where the operation semantics are
//! symmetric. The semantic order supplied by a caller is otherwise preserved.
//!
//! # Complexity
//!
//! SWAP execution:
//!
//! - topology validation: delegated to `Topology`;
//! - mapping lookup: O(1) average;
//! - mapping mutation: O(1) average;
//! - recording: amortized O(1).
//!
//! Permutation execution:
//!
//! - validation: O(n);
//! - mapping application: O(n);
//! - rollback snapshot: O(n).
//!
//! # Safety
//!
//! - Rust 1.97 / 1.97.1;
//! - Rust 2021;
//! - no `unsafe`;
//! - no global mutable state;
//! - no filesystem access;
//! - no network access;
//! - no hardware access;
//! - no environment-dependent behavior.
//!
//! # Integration contract
//!
//! This module is intended to be consumed by:
//!
//! ```text
//! algorithms/basic.rs
//! algorithms/shortest_path.rs
//! algorithms/lookahead.rs
//! algorithms/sabre.rs
//! algorithms/noise_aware.rs
//! algorithms/dynamic.rs
//!
//! router.rs
//! verification.rs
//! transpiler.rs
//! benchmarking
//! ```
//!
//! The algorithms must not mutate `QubitMapping` directly when using this
//! movement layer. They should submit movement requests here.
//!
//! # Dependency direction
//!
//! ```text
//! types.rs ─────────────┐
//! mapping.rs ───────────┤
//! topology.rs ──────────┤
//! errors.rs ────────────┤
//! config.rs ────────────┤
//!                         ▼
//!                  moves/router.rs
//!                         ▲
//!                         │
//!                  algorithms/*
//!                         │
//!                       router
//! ```
//!
//! `moves/router.rs` intentionally does not depend on routing algorithms.
//!
//! # Rust version
//!
//! Tested/design target:
//!
//! ```text
//! rustc 1.97.1
//! edition = "2021"
//! ```
//!
//! No nightly features are required.

// =============================================================================
// Imports
// =============================================================================

use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use crate::quantum::routing::config::RoutingConfig;
use crate::quantum::routing::errors::{
    RoutingError,
    RoutingErrorContext,
    RoutingStage,
};
use crate::quantum::routing::mapping::{
    QubitMapping,
    QubitMappingSnapshot,
};
use crate::quantum::routing::topology::Topology;
use crate::quantum::routing::types::{
    LogicalQubitId,
    PhysicalQubitId,
    RoutingMove,
    RoutingOperation,
};

// =============================================================================
// Constants
// =============================================================================

/// Stable implementation identifier for reproducibility metadata.
pub const MOVEMENT_ROUTER_VERSION: &str = "1.0.0";

/// Default maximum number of movement records retained by a router.
///
/// This is deliberately finite so a malicious or pathological routing request
/// cannot grow diagnostic history without bound.
pub const DEFAULT_MAX_HISTORY_ENTRIES: usize = 1_000_000;

/// Default maximum number of operations emitted by one movement transaction.
///
/// This is a defensive boundary, not a hardware limitation.
pub const DEFAULT_MAX_TRANSACTION_OPERATIONS: usize = 1_000_000;

// =============================================================================
// Movement kind
// =============================================================================

/// Semantic category of a routing movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MovementKind {
    /// Exchange the logical states occupying two adjacent physical qubits.
    Swap,

    /// A bridge-style movement/interaction transformation.
    Bridge,

    /// Apply a general physical permutation.
    Permutation,
}

impl MovementKind {
    /// Stable machine-readable name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Swap => "swap",
            Self::Bridge => "bridge",
            Self::Permutation => "permutation",
        }
    }
}

impl fmt::Display for MovementKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Movement request
// =============================================================================

/// A validated-independent request submitted by a routing algorithm.
///
/// A request is intentionally not a `RoutingOperation` directly because the
/// execution layer must validate it before it becomes part of the committed
/// output stream.
///
/// This type is also useful for speculative routing: algorithms can construct
/// requests without mutating a mapping and submit them only when selected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MovementRequest {
    /// Exchange the logical states at two physical locations.
    Swap {
        /// First physical location.
        a: PhysicalQubitId,

        /// Second physical location.
        b: PhysicalQubitId,
    },

    /// Request a bridge movement.
    ///
    /// Bridge semantics are deliberately represented at the routing layer.
    /// Whether a backend can actually lower the bridge is a later capability
    /// decision.
    Bridge {
        /// First physical endpoint.
        a: PhysicalQubitId,

        /// Intermediate physical qubit.
        bridge: PhysicalQubitId,

        /// Second physical endpoint.
        b: PhysicalQubitId,
    },

    /// Apply a complete physical permutation.
    ///
    /// Each pair is:
    ///
    /// ```text
    /// source -> destination
    /// ```
    ///
    /// The operation is atomic.
    Permutation {
        /// Physical state permutation.
        mapping: Vec<(PhysicalQubitId, PhysicalQubitId)>,
    },
}

impl MovementRequest {
    /// Returns the movement kind.
    #[must_use]
    pub const fn kind(&self) -> MovementKind {
        match self {
            Self::Swap { .. } => MovementKind::Swap,
            Self::Bridge { .. } => MovementKind::Bridge,
            Self::Permutation { .. } => MovementKind::Permutation,
        }
    }

    /// Returns the number of physical endpoints touched by this request.
    #[must_use]
    pub fn endpoint_count(&self) -> usize {
        match self {
            Self::Swap { .. } => 2,
            Self::Bridge { .. } => 3,
            Self::Permutation { mapping } => mapping.len(),
        }
    }

    /// Converts a validated movement request into its semantic routing
    /// operation representation.
    ///
    /// This conversion does not validate topology. Validation is the
    /// responsibility of `MovementRouter`.
    #[must_use]
    pub fn into_routing_operation(self) -> RoutingOperation {
        match self {
            Self::Swap { a, b } => RoutingOperation::Swap { a, b },

            Self::Bridge { a, bridge, b } => {
                RoutingOperation::Bridge {
                    a,
                    bridge,
                    b,
                }
            }

            Self::Permutation { mapping } => {
                RoutingOperation::Permutation {
                    mapping,
                }
            }
        }
    }
}

impl From<(PhysicalQubitId, PhysicalQubitId)> for MovementRequest {
    fn from(
        endpoints: (PhysicalQubitId, PhysicalQubitId),
    ) -> Self {
        Self::Swap {
            a: endpoints.0,
            b: endpoints.1,
        }
    }
}

// =============================================================================
// Movement record
// =============================================================================

/// Immutable record of a successfully committed movement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MovementRecord {
    /// Sequential movement index.
    pub index: usize,

    /// Movement kind.
    pub kind: MovementKind,

    /// Semantic operation emitted by the routing layer.
    pub operation: RoutingOperation,

    /// Mapping snapshot before the movement.
    pub mapping_before: QubitMappingSnapshot,

    /// Mapping snapshot after the movement.
    pub mapping_after: QubitMappingSnapshot,
}

impl MovementRecord {
    /// Creates a movement record.
    #[must_use]
    pub fn new(
        index: usize,
        kind: MovementKind,
        operation: RoutingOperation,
        mapping_before: QubitMappingSnapshot,
        mapping_after: QubitMappingSnapshot,
    ) -> Self {
        Self {
            index,
            kind,
            operation,
            mapping_before,
            mapping_after,
        }
    }
}

// =============================================================================
// Movement statistics
// =============================================================================

/// Counters maintained by the movement router.
///
/// These counters describe movement execution, not complete circuit metrics.
/// `result.rs` remains the owner of final routing metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MovementStatistics {
    /// Number of committed movement requests.
    pub total_movements: usize,

    /// Number of committed SWAP movements.
    pub swaps: usize,

    /// Number of committed bridge movements.
    pub bridges: usize,

    /// Number of committed permutation movements.
    pub permutations: usize,

    /// Number of rejected movement requests.
    pub rejected: usize,

    /// Number of successful transactions.
    pub committed_transactions: usize,

    /// Number of rolled-back transactions.
    pub rolled_back_transactions: usize,

    /// Number of physical endpoints touched by committed movements.
    pub endpoint_updates: usize,
}

impl MovementStatistics {
    /// Returns the number of movement operations that actually changed the
    /// mapping.
    #[must_use]
    pub const fn committed_movements(self) -> usize {
        self.total_movements
    }

    /// Adds another statistics value using checked arithmetic.
    ///
    /// Returns `None` if any counter would overflow.
    #[must_use]
    pub const fn checked_add(
        self,
        other: Self,
    ) -> Option<Self> {
        Some(Self {
            total_movements: self
                .total_movements
                .checked_add(other.total_movements)?,

            swaps: self.swaps.checked_add(other.swaps)?,

            bridges: self.bridges.checked_add(other.bridges)?,

            permutations: self
                .permutations
                .checked_add(other.permutations)?,

            rejected: self.rejected.checked_add(other.rejected)?,

            committed_transactions: self
                .committed_transactions
                .checked_add(
                    other.committed_transactions,
                )?,

            rolled_back_transactions: self
                .rolled_back_transactions
                .checked_add(
                    other.rolled_back_transactions,
                )?,

            endpoint_updates: self
                .endpoint_updates
                .checked_add(other.endpoint_updates)?,
        })
    }
}

// =============================================================================
// Movement transaction
// =============================================================================

/// Transactional movement context.
///
/// A transaction owns a snapshot of the mapping and the length of the output
/// stream at transaction start.
///
/// The caller must explicitly call [`MovementTransaction::commit`] to make the
/// transaction successful.
///
/// Dropping an uncommitted transaction automatically restores the mapping and
/// operation stream.
///
/// This provides a strong failure guarantee for speculative routing.
pub struct MovementTransaction<'a> {
    router: &'a mut MovementRouter,
    mapping_snapshot: QubitMappingSnapshot,
    operations_len: usize,
    history_len: usize,
    statistics_snapshot: MovementStatistics,
    committed: bool,
}

impl<'a> MovementTransaction<'a> {
    /// Applies a movement inside this transaction.
    pub fn apply(
        &mut self,
        request: MovementRequest,
    ) -> Result<(), RoutingError> {
        self.router.apply_internal(request)
    }

    /// Returns the number of movements applied during this transaction.
    #[must_use]
    pub fn movement_count(&self) -> usize {
        self.router
            .statistics
            .total_movements
            .saturating_sub(
                self.statistics_snapshot.total_movements,
            )
    }

    /// Returns the current transaction output operations.
    #[must_use]
    pub fn operations(&self) -> &[RoutingOperation] {
        &self.router.operations[self.operations_len..]
    }

    /// Commits the transaction.
    ///
    /// Once committed, the transaction no longer owns rollback responsibility.
    pub fn commit(mut self) -> Result<(), RoutingError> {
        self.router
            .mapping
            .validate(self.router.topology)
            .map_err(|error| {
                self.router.record_rejection();
                error
            })?;

        self.router.statistics.committed_transactions =
            self.router
                .statistics
                .committed_transactions
                .checked_add(1)
                .ok_or_else(|| {
                    RoutingError::InternalInvariantViolation {
                        message:
                            "movement transaction commit counter overflow"
                                .to_string(),
                    }
                })?;

        self.committed = true;

        Ok(())
    }

    /// Explicitly rolls back the transaction.
    pub fn rollback(mut self) -> Result<(), RoutingError> {
        self.rollback_internal()?;
        self.committed = true;
        Ok(())
    }

    fn rollback_internal(&mut self) -> Result<(), RoutingError> {
        self.router
            .mapping
            .restore(&self.mapping_snapshot)
            .map_err(|error| {
                RoutingError::InternalInvariantViolation {
                    message: format!(
                        "failed to restore movement transaction mapping: {error}"
                    ),
                }
            })?;

        self.router
            .operations
            .truncate(self.operations_len);

        self.router.history.truncate(self.history_len);

        self.router.statistics =
            self.statistics_snapshot;

        self.router.statistics.rolled_back_transactions =
            self.router
                .statistics
                .rolled_back_transactions
                .checked_add(1)
                .ok_or_else(|| {
                    RoutingError::InternalInvariantViolation {
                        message:
                            "movement rollback counter overflow"
                                .to_string(),
                    }
                })?;

        Ok(())
    }
}

impl Drop for MovementTransaction<'_> {
    fn drop(&mut self) {
        if !self.committed {
            let _ = self.rollback_internal();
        }
    }
}

// =============================================================================
// Movement router
// =============================================================================

/// Transactional executor for semantic routing movements.
///
/// `MovementRouter` is intentionally smaller than the public high-level
/// `QuantumRouter`.
///
/// The high-level router decides:
///
/// ```text
/// which algorithm?
/// which layout?
/// which candidate?
/// ```
///
/// `MovementRouter` decides only:
///
/// ```text
/// can this movement legally mutate the current mapping?
/// if yes, apply it atomically and record it.
/// ```
///
/// # Invariants
///
/// After every successful public method:
///
/// 1. mapping is internally consistent;
/// 2. every referenced physical endpoint exists;
/// 3. every topology constraint required by the movement is satisfied;
/// 4. operation history corresponds exactly to committed mutations;
/// 5. movement statistics correspond exactly to committed movements.
///
/// On failure, the state visible through the router remains unchanged except
/// for the rejection counter where explicitly documented.
#[derive(Debug)]
pub struct MovementRouter<'a> {
    topology: &'a Topology,
    mapping: QubitMapping,
    config: &'a RoutingConfig,

    operations: Vec<RoutingOperation>,
    history: Vec<MovementRecord>,
    statistics: MovementStatistics,

    max_history_entries: usize,
    max_transaction_operations: usize,

    /// Optional mapping from physical qubits to roles/availability.
///
/// The current topology remains the authoritative hardware resource source.
/// This field exists only for movement-layer policy overrides and is empty by
/// default.
    blocked_physical: HashMap<PhysicalQubitId, bool>,
}

impl<'a> MovementRouter<'a> {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates a movement router.
    ///
    /// The supplied mapping is cloned. The caller's mapping is therefore never
    /// mutated by this object.
    pub fn new(
        topology: &'a Topology,
        mapping: &QubitMapping,
        config: &'a RoutingConfig,
    ) -> Result<Self, RoutingError> {
        topology.validate()?;

        mapping.validate(topology)?;

        let max_history_entries =
            DEFAULT_MAX_HISTORY_ENTRIES;

        let max_transaction_operations =
            DEFAULT_MAX_TRANSACTION_OPERATIONS;

        if config.max_operations == 0 {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "routing configuration max_operations must be greater than zero"
                            .to_string(),
                },
            );
        }

        Ok(Self {
            topology,
            mapping: mapping.clone(),
            config,
            operations: Vec::new(),
            history: Vec::new(),
            statistics: MovementStatistics::default(),
            max_history_entries,
            max_transaction_operations,
            blocked_physical: HashMap::new(),
        })
    }

    /// Creates a movement router with explicit diagnostic-history limits.
    pub fn with_limits(
        topology: &'a Topology,
        mapping: &QubitMapping,
        config: &'a RoutingConfig,
        max_history_entries: usize,
        max_transaction_operations: usize,
    ) -> Result<Self, RoutingError> {
        if max_history_entries == 0 {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "max_history_entries must be greater than zero"
                            .to_string(),
                },
            );
        }

        if max_transaction_operations == 0 {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "max_transaction_operations must be greater than zero"
                            .to_string(),
                },
            );
        }

        let mut router =
            Self::new(topology, mapping, config)?;

        router.max_history_entries =
            max_history_entries;

        router.max_transaction_operations =
            max_transaction_operations;

        Ok(router)
    }

    // =========================================================================
    // Read-only state
    // =========================================================================

    /// Returns the current working mapping.
    #[must_use]
    pub fn mapping(&self) -> &QubitMapping {
        &self.mapping
    }

    /// Returns the current topology.
    #[must_use]
    pub const fn topology(&self) -> &Topology {
        self.topology
    }

    /// Returns the routing configuration.
    #[must_use]
    pub const fn config(&self) -> &RoutingConfig {
        self.config
    }

    /// Returns the committed routing operations.
    #[must_use]
    pub fn operations(&self) -> &[RoutingOperation] {
        &self.operations
    }

    /// Returns committed movement records.
    #[must_use]
    pub fn history(&self) -> &[MovementRecord] {
        &self.history
    }

    /// Returns movement statistics.
    #[must_use]
    pub const fn statistics(&self) -> MovementStatistics {
        self.statistics
    }

    /// Returns the number of committed movement operations.
    #[must_use]
    pub fn movement_count(&self) -> usize {
        self.statistics.total_movements
    }

    /// Returns the current final mapping snapshot.
    #[must_use]
    pub fn snapshot(&self) -> QubitMappingSnapshot {
        self.mapping.snapshot()
    }

    // =========================================================================
    // Policy controls
    // =========================================================================

    /// Blocks a physical qubit from future movement.
    ///
    /// This is a movement-level reservation facility. It does not mutate the
    /// topology itself.
    ///
    /// It is useful for dynamic routing where a physical resource becomes
    /// temporarily unavailable without requiring reconstruction of the entire
    /// topology.
    pub fn set_physical_blocked(
        &mut self,
        physical: PhysicalQubitId,
        blocked: bool,
    ) -> Result<(), RoutingError> {
        self.ensure_physical_exists(physical)?;

        if blocked {
            self.blocked_physical
                .insert(physical, true);
        } else {
            self.blocked_physical.remove(&physical);
        }

        Ok(())
    }

    /// Returns whether a physical qubit is movement-blocked.
    #[must_use]
    pub fn is_physical_blocked(
        &self,
        physical: PhysicalQubitId,
    ) -> bool {
        self.blocked_physical
            .get(&physical)
            .copied()
            .unwrap_or(false)
    }

    // =========================================================================
    // Transaction API
    // =========================================================================

    /// Starts an explicit movement transaction.
    ///
    /// The transaction must be committed for its changes to survive.
    pub fn begin_transaction(
        &'a mut self,
    ) -> MovementTransaction<'a> {
        MovementTransaction {
            mapping_snapshot: self.mapping.snapshot(),
            operations_len: self.operations.len(),
            history_len: self.history.len(),
            statistics_snapshot: self.statistics,
            router: self,
            committed: false,
        }
    }

    /// Executes exactly one movement atomically.
    ///
    /// This is the normal API for algorithms that already know the selected
    /// movement.
    pub fn apply(
        &mut self,
        request: MovementRequest,
    ) -> Result<(), RoutingError> {
        let snapshot = self.mapping.snapshot();
        let operations_len = self.operations.len();
        let history_len = self.history.len();
        let statistics = self.statistics;

        match self.apply_internal(request) {
            Ok(()) => {
                if let Err(error) =
                    self.mapping.validate(self.topology)
                {
                    let _ = self
                        .restore_state(
                            &snapshot,
                            operations_len,
                            history_len,
                            statistics,
                        );

                    self.record_rejection();

                    return Err(error);
                }

                Ok(())
            }

            Err(error) => {
                let _ = self.restore_state(
                    &snapshot,
                    operations_len,
                    history_len,
                    statistics,
                );

                self.record_rejection();

                Err(error)
            }
        }
    }

    /// Applies a complete movement batch atomically.
    ///
    /// If any movement fails, all earlier movements in the batch are rolled
    /// back.
    pub fn apply_batch(
        &mut self,
        requests: &[MovementRequest],
    ) -> Result<(), RoutingError> {
        if requests.len()
            > self.max_transaction_operations
        {
            return Err(
                RoutingError::InvalidConfiguration {
                    message: format!(
                        "movement batch contains {} operations; maximum is {}",
                        requests.len(),
                        self.max_transaction_operations
                    ),
                },
            );
        }

        let snapshot = self.mapping.snapshot();
        let operations_len = self.operations.len();
        let history_len = self.history.len();
        let statistics = self.statistics;

        for request in requests {
            if let Err(error) =
                self.apply_internal(request.clone())
            {
                let _ = self.restore_state(
                    &snapshot,
                    operations_len,
                    history_len,
                    statistics,
                );

                self.record_rejection();

                return Err(error);
            }
        }

        if let Err(error) =
            self.mapping.validate(self.topology)
        {
            let _ = self.restore_state(
                &snapshot,
                operations_len,
                history_len,
                statistics,
            );

            self.record_rejection();

            return Err(error);
        }

        self.statistics.committed_transactions =
            match self
                .statistics
                .committed_transactions
                .checked_add(1)
            {
                Some(value) => value,
                None => {
                    let _ = self.restore_state(
                        &snapshot,
                        operations_len,
                        history_len,
                        statistics,
                    );

                    return Err(
                        RoutingError::InternalInvariantViolation {
                            message:
                                "movement transaction counter overflow"
                                    .to_string(),
                        },
                    );
                }
            };

        Ok(())
    }

    // =========================================================================
    // Direct SWAP API
    // =========================================================================

    /// Applies a physical SWAP to the working mapping.
    ///
    /// The operation is legal only when:
    ///
    /// - both physical qubits exist;
    /// - the physical edge is usable;
    /// - neither endpoint is blocked;
    /// - the mapping remains valid after the swap.
    pub fn swap(
        &mut self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> Result<(), RoutingError> {
        self.apply(MovementRequest::Swap { a, b })
    }

    /// Applies a sequence of adjacent physical SWAPs atomically.
    pub fn swap_path(
        &mut self,
        path: &[PhysicalQubitId],
    ) -> Result<(), RoutingError> {
        if path.len() < 2 {
            return Ok(());
        }

        let swap_count = path.len() - 1;

        if swap_count > self.max_transaction_operations {
            return Err(
                RoutingError::InvalidConfiguration {
                    message: format!(
                        "SWAP path contains {swap_count} moves; maximum is {}",
                        self.max_transaction_operations
                    ),
                },
            );
        }

        let requests: Vec<_> = path
            .windows(2)
            .map(|window| MovementRequest::Swap {
                a: window[0],
                b: window[1],
            })
            .collect();

        self.apply_batch(&requests)
    }

    // =========================================================================
    // Bridge API
    // =========================================================================

    /// Applies a bridge movement request.
    ///
    /// The movement layer validates the physical geometry but does not invent
    /// a backend decomposition.
    pub fn bridge(
        &mut self,
        a: PhysicalQubitId,
        bridge: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> Result<(), RoutingError> {
        self.apply(MovementRequest::Bridge {
            a,
            bridge,
            b,
        })
    }

    // =========================================================================
    // Permutation API
    // =========================================================================

    /// Applies an atomic physical permutation.
    ///
    /// `mapping` contains `(source, destination)` pairs.
    ///
    /// Example:
    ///
    /// ```text
    /// [(p0, p1), (p1, p2), (p2, p0)]
    /// ```
    ///
    /// means:
    ///
    /// ```text
    /// state(p0) -> p1
    /// state(p1) -> p2
    /// state(p2) -> p0
    /// ```
    pub fn permutation(
        &mut self,
        mapping: &[(PhysicalQubitId, PhysicalQubitId)],
    ) -> Result<(), RoutingError> {
        self.apply(MovementRequest::Permutation {
            mapping: mapping.to_vec(),
        })
    }

    // =========================================================================
    // Internal execution
    // =========================================================================

    fn apply_internal(
        &mut self,
        request: MovementRequest,
    ) -> Result<(), RoutingError> {
        self.ensure_operation_capacity()?;

        match request {
            MovementRequest::Swap { a, b } => {
                self.apply_swap(a, b)
            }

            MovementRequest::Bridge { a, bridge, b } => {
                self.apply_bridge(a, bridge, b)
            }

            MovementRequest::Permutation { mapping } => {
                self.apply_permutation(&mapping)
            }
        }
    }

    fn apply_swap(
        &mut self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> Result<(), RoutingError> {
        self.ensure_distinct_physical(a, b)?;
        self.ensure_physical_exists(a)?;
        self.ensure_physical_exists(b)?;
        self.ensure_not_blocked(a)?;
        self.ensure_not_blocked(b)?;

        if !self.topology.is_adjacent(a, b) {
            return Err(
                self.movement_error(
                    "SWAP endpoints are not adjacent in the target topology",
                    Some(a),
                    Some(b),
                ),
            );
        }

        let before = self.mapping.snapshot();

        self.mapping
            .swap_physical(a, b)
            .map_err(|error| {
                self.movement_error(
                    &format!(
                        "failed to apply physical SWAP to mapping: {error}"
                    ),
                    Some(a),
                    Some(b),
                )
            })?;

        self.mapping
            .validate(self.topology)
            .map_err(|error| {
                self.movement_error(
                    &format!(
                        "mapping became invalid after SWAP: {error}"
                    ),
                    Some(a),
                    Some(b),
                )
            })?;

        let operation =
            RoutingOperation::Swap { a, b };

        let after = self.mapping.snapshot();

        self.record(
            MovementKind::Swap,
            operation,
            before,
            after,
            2,
        )?;

        Ok(())
    }

    fn apply_bridge(
        &mut self,
        a: PhysicalQubitId,
        bridge: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> Result<(), RoutingError> {
        self.ensure_distinct_physical(a, bridge)?;
        self.ensure_distinct_physical(bridge, b)?;
        self.ensure_distinct_physical(a, b)?;

        self.ensure_physical_exists(a)?;
        self.ensure_physical_exists(bridge)?;
        self.ensure_physical_exists(b)?;

        self.ensure_not_blocked(a)?;
        self.ensure_not_blocked(bridge)?;
        self.ensure_not_blocked(b)?;

        if !self.topology.is_adjacent(a, bridge) {
            return Err(
                self.movement_error(
                    "bridge first endpoint is not adjacent to the bridge qubit",
                    Some(a),
                    Some(bridge),
                ),
            );
        }

        if !self.topology.is_adjacent(bridge, b) {
            return Err(
                self.movement_error(
                    "bridge second endpoint is not adjacent to the bridge qubit",
                    Some(bridge),
                    Some(b),
                ),
            );
        }

        let before = self.mapping.snapshot();

        // A bridge operation is intentionally semantic and does not alter the
        // mapping unless a future bridge policy explicitly defines state
        // movement. This prevents the movement layer from inventing semantics
        // for a bridge construction.
        //
        // The operation is still recorded so downstream synthesis can lower it
        // according to the target backend.
        let operation =
            RoutingOperation::Bridge {
                a,
                bridge,
                b,
            };

        let after = self.mapping.snapshot();

        self.record(
            MovementKind::Bridge,
            operation,
            before,
            after,
            3,
        )?;

        Ok(())
    }

    fn apply_permutation(
        &mut self,
        permutation: &[
            (PhysicalQubitId, PhysicalQubitId)
        ],
    ) -> Result<(), RoutingError> {
        if permutation.is_empty() {
            return Err(
                self.movement_error(
                    "physical permutation cannot be empty",
                    None,
                    None,
                ),
            );
        }

        self.validate_permutation(permutation)?;

        let before = self.mapping.snapshot();

        self.apply_physical_permutation(permutation)?;

        self.mapping
            .validate(self.topology)
            .map_err(|error| {
                RoutingError::InternalInvariantViolation {
                    message: format!(
                        "mapping became invalid after physical permutation: {error}"
                    ),
                }
            })?;

        let operation =
            RoutingOperation::Permutation {
                mapping: permutation.to_vec(),
            };

        let after = self.mapping.snapshot();

        self.record(
            MovementKind::Permutation,
            operation,
            before,
            after,
            permutation.len(),
        )?;

        Ok(())
    }

    // =========================================================================
    // Permutation validation
    // =========================================================================

    fn validate_permutation(
        &self,
        permutation: &[
            (PhysicalQubitId, PhysicalQubitId)
        ],
    ) -> Result<(), RoutingError> {
        let mut sources =
            std::collections::HashSet::with_capacity(
                permutation.len(),
            );

        let mut destinations =
            std::collections::HashSet::with_capacity(
                permutation.len(),
            );

        for &(source, destination) in permutation {
            self.ensure_physical_exists(source)?;
            self.ensure_physical_exists(destination)?;

            self.ensure_not_blocked(source)?;
            self.ensure_not_blocked(destination)?;

            if source == destination {
                return Err(
                    self.movement_error(
                        "permutation contains an identity movement; omit source == destination entries",
                        Some(source),
                        Some(destination),
                    ),
                );
            }

            if !sources.insert(source) {
                return Err(
                    RoutingError::InvalidConfiguration {
                        message: format!(
                            "physical permutation contains duplicate source {source}"
                        ),
                    },
                );
            }

            if !destinations.insert(destination) {
                return Err(
                    RoutingError::InvalidConfiguration {
                        message: format!(
                            "physical permutation contains duplicate destination {destination}"
                        ),
                    },
                );
            }
        }

        if sources.len() != destinations.len() {
            return Err(
                RoutingError::InvalidConfiguration {
                    message:
                        "physical permutation has unequal source and destination cardinality"
                            .to_string(),
                },
            );
        }

        // A permutation must preserve the state-space cardinality of its
        // affected physical locations. Every destination must be either:
        //
        // 1. one of the listed sources, or
        // 2. an explicitly occupied/empty location whose semantics are known.
        //
        // This implementation requires a closed permutation because silently
        // dropping a logical state would violate routing correctness.
        for destination in &destinations {
            if !sources.contains(destination) {
                return Err(
                    RoutingError::InvalidConfiguration {
                        message: format!(
                            "physical permutation is not closed: destination {destination} is not among the sources"
                        ),
                    },
                );
            }
        }

        Ok(())
    }

    fn apply_physical_permutation(
        &mut self,
        permutation: &[
            (PhysicalQubitId, PhysicalQubitId)
        ],
    ) -> Result<(), RoutingError> {
        //
        // Resolve the logical occupants first. This is essential because
        // sequentially mutating the mapping while reading the same locations
        // would otherwise make the result dependent on pair ordering.
        //
        let mut occupants =
            Vec::with_capacity(permutation.len());

        for &(source, _) in permutation {
            occupants.push((
                source,
                self.mapping.logical_at(source),
            ));
        }

        for &(source, destination) in permutation {
            self.mapping
                .unassign_physical(destination)
                .map_err(|error| {
                    RoutingError::InternalInvariantViolation {
                        message: format!(
                            "failed to clear permutation destination {destination}: {error}"
                        ),
                    }
                })?;

            // If the destination currently contains a logical qubit that is
            // also a source, it will be restored from the captured occupant
            // set below.
            let _ = source;
        }

        for (source, logical) in occupants {
            if let Some(logical) = logical {
                let destination = permutation
                    .iter()
                    .find_map(
                        |&(from, to)| {
                            if from == source {
                                Some(to)
                            } else {
                                None
                            }
                        },
                    )
                    .ok_or_else(|| {
                        RoutingError::InternalInvariantViolation {
                            message:
                                "captured permutation source disappeared during application"
                                    .to_string(),
                        }
                    })?;

                self.mapping
                    .assign(logical, destination)
                    .map_err(|error| {
                        RoutingError::InternalInvariantViolation {
                            message: format!(
                                "failed to assign logical qubit {logical} to permutation destination {destination}: {error}"
                            ),
                        }
                    })?;
            }
        }

        Ok(())
    }

    // =========================================================================
    // Recording
    // =========================================================================

    fn record(
        &mut self,
        kind: MovementKind,
        operation: RoutingOperation,
        before: QubitMappingSnapshot,
        after: QubitMappingSnapshot,
        endpoint_count: usize,
    ) -> Result<(), RoutingError> {
        if self.history.len()
            >= self.max_history_entries
        {
            return Err(
                RoutingError::InvalidConfiguration {
                    message: format!(
                        "movement history limit {} reached",
                        self.max_history_entries
                    ),
                },
            );
        }

        let index = self.history.len();

        self.history.push(
            MovementRecord::new(
                index,
                kind,
                operation.clone(),
                before,
                after,
            ),
        );

        self.operations.push(operation);

        self.statistics.total_movements =
            self.statistics
                .total_movements
                .checked_add(1)
                .ok_or_else(|| {
                    RoutingError::InternalInvariantViolation {
                        message:
                            "movement count overflow".to_string(),
                    }
                })?;

        match kind {
            MovementKind::Swap => {
                self.statistics.swaps =
                    self.statistics.swaps
                        .checked_add(1)
                        .ok_or_else(|| {
                            RoutingError::InternalInvariantViolation {
                                message:
                                    "SWAP count overflow".to_string(),
                            }
                        })?;
            }

            MovementKind::Bridge => {
                self.statistics.bridges =
                    self.statistics.bridges
                        .checked_add(1)
                        .ok_or_else(|| {
                            RoutingError::InternalInvariantViolation {
                                message:
                                    "bridge count overflow".to_string(),
                            }
                        })?;
            }

            MovementKind::Permutation => {
                self.statistics.permutations =
                    self.statistics.permutations
                        .checked_add(1)
                        .ok_or_else(|| {
                            RoutingError::InternalInvariantViolation {
                                message:
                                    "permutation count overflow".to_string(),
                            }
                        })?;
            }
        }

        self.statistics.endpoint_updates =
            self.statistics.endpoint_updates
                .checked_add(endpoint_count)
                .ok_or_else(|| {
                    RoutingError::InternalInvariantViolation {
                        message:
                            "movement endpoint counter overflow"
                                .to_string(),
                    }
                })?;

        Ok(())
    }

    fn record_rejection(&mut self) {
        self.statistics.rejected =
            self.statistics.rejected.saturating_add(1);
    }

    // =========================================================================
    // Resource validation
    // =========================================================================

    fn ensure_operation_capacity(
        &self,
    ) -> Result<(), RoutingError> {
        if self.operations.len()
            >= self.max_transaction_operations
        {
            return Err(
                RoutingError::InvalidConfiguration {
                    message: format!(
                        "movement operation limit {} reached",
                        self.max_transaction_operations
                    ),
                },
            );
        }

        if self.operations.len()
            >= self.config.max_operations
        {
            return Err(
                RoutingError::InvalidConfiguration {
                    message: format!(
                        "configured routing operation limit {} reached",
                        self.config.max_operations
                    ),
                },
            );
        }

        Ok(())
    }

    fn ensure_physical_exists(
        &self,
        physical: PhysicalQubitId,
    ) -> Result<(), RoutingError> {
        if !self.topology.contains(physical) {
            return Err(
                self.movement_error(
                    "physical qubit does not exist in target topology",
                    Some(physical),
                    None,
                ),
            );
        }

        Ok(())
    }

    fn ensure_distinct_physical(
        &self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> Result<(), RoutingError> {
        if a == b {
            return Err(
                self.movement_error(
                    "movement endpoints must be distinct physical qubits",
                    Some(a),
                    Some(b),
                ),
            );
        }

        Ok(())
    }

    fn ensure_not_blocked(
        &self,
        physical: PhysicalQubitId,
    ) -> Result<(), RoutingError> {
        if self.is_physical_blocked(physical) {
            return Err(
                self.movement_error(
                    "physical qubit is blocked for movement",
                    Some(physical),
                    None,
                ),
            );
        }

        Ok(())
    }

    // =========================================================================
    // Rollback
    // =========================================================================

    fn restore_state(
        &mut self,
        mapping: &QubitMappingSnapshot,
        operations_len: usize,
        history_len: usize,
        statistics: MovementStatistics,
    ) -> Result<(), RoutingError> {
        self.mapping
            .restore(mapping)
            .map_err(|error| {
                RoutingError::InternalInvariantViolation {
                    message: format!(
                        "failed to restore movement-router mapping: {error}"
                    ),
                }
            })?;

        self.operations.truncate(operations_len);
        self.history.truncate(history_len);
        self.statistics = statistics;

        Ok(())
    }

    // =========================================================================
    // Diagnostics
    // =========================================================================

    fn movement_error(
        &self,
        detail: &str,
        physical: Option<PhysicalQubitId>,
        second_physical: Option<PhysicalQubitId>,
    ) -> RoutingError {
        let mut context =
            RoutingErrorContext::new()
                .with_stage(RoutingStage::Movement)
                .with_detail(detail);

        if let Some(physical) = physical {
            context =
                context.with_physical_qubit(
                    physical.index(),
                );
        }

        if let Some(physical) = second_physical {
            context =
                context.with_second_physical_qubit(
                    physical.index(),
                );
        }

        RoutingError::InternalInvariantViolation {
            message: format!(
                "movement-router failure: {detail}"
            ),
        }
    }
}

// =============================================================================
// Unit tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::routing::config::{
        RoutingConfig,
    };
    use crate::quantum::routing::mapping::{
        QubitMapping,
    };
    use crate::quantum::routing::topology::{
        Topology,
    };
    use crate::quantum::routing::types::{
        LogicalQubitId,
        PhysicalQubitId,
    };

    fn topology_line() -> Topology {
        Topology::from_edges(
            3,
            &[
                (
                    PhysicalQubitId::new(0),
                    PhysicalQubitId::new(1),
                ),
                (
                    PhysicalQubitId::new(1),
                    PhysicalQubitId::new(2),
                ),
            ],
        )
        .expect("valid line topology")
    }

    fn mapping_three() -> QubitMapping {
        QubitMapping::from_assignments([
            (
                LogicalQubitId::new(0),
                PhysicalQubitId::new(0),
            ),
            (
                LogicalQubitId::new(1),
                PhysicalQubitId::new(1),
            ),
            (
                LogicalQubitId::new(2),
                PhysicalQubitId::new(2),
            ),
        ])
        .expect("valid mapping")
    }

    fn config() -> RoutingConfig {
        RoutingConfig::default()
    }

    #[test]
    fn swap_updates_mapping_bidirectionally() {
        let topology = topology_line();
        let mapping = mapping_three();
        let config = config();

        let mut router =
            MovementRouter::new(
                &topology,
                &mapping,
                &config,
            )
            .expect("router creation");

        router
            .swap(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
            )
            .expect("legal swap");

        assert_eq!(
            router
                .mapping()
                .physical_of(
                    LogicalQubitId::new(0)
                ),
            Some(PhysicalQubitId::new(1))
        );

        assert_eq!(
            router
                .mapping()
                .physical_of(
                    LogicalQubitId::new(1)
                ),
            Some(PhysicalQubitId::new(0))
        );
    }

    #[test]
    fn non_adjacent_swap_is_rejected_transactionally() {
        let topology = topology_line();
        let mapping = mapping_three();
        let config = config();

        let mut router =
            MovementRouter::new(
                &topology,
                &mapping,
                &config,
            )
            .expect("router creation");

        let before = router.snapshot();

        assert!(
            router
                .swap(
                    PhysicalQubitId::new(0),
                    PhysicalQubitId::new(2),
                )
                .is_err()
        );

        assert_eq!(
            router.snapshot(),
            before
        );

        assert!(
            router.operations().is_empty()
        );
    }

    #[test]
    fn swap_path_is_atomic() {
        let topology = topology_line();
        let mapping = mapping_three();
        let config = config();

        let mut router =
            MovementRouter::new(
                &topology,
                &mapping,
                &config,
            )
            .expect("router creation");

        router
            .swap_path(&[
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(2),
            ])
            .expect("valid path");

        assert_eq!(
            router.movement_count(),
            2
        );

        assert_eq!(
            router
                .mapping()
                .physical_of(
                    LogicalQubitId::new(0)
                ),
            Some(PhysicalQubitId::new(2))
        );
    }

    #[test]
    fn transaction_rolls_back_on_drop() {
        let topology = topology_line();
        let mapping = mapping_three();
        let config = config();

        let mut router =
            MovementRouter::new(
                &topology,
                &mapping,
                &config,
            )
            .expect("router creation");

        let before = router.snapshot();

        {
            let mut transaction =
                router.begin_transaction();

            transaction
                .apply(
                    MovementRequest::Swap {
                        a: PhysicalQubitId::new(0),
                        b: PhysicalQubitId::new(1),
                    },
                )
                .expect("valid swap");

            // Intentionally do not commit.
        }

        assert_eq!(
            router.snapshot(),
            before
        );

        assert!(
            router.operations().is_empty()
        );
    }

    #[test]
    fn committed_transaction_survives() {
        let topology = topology_line();
        let mapping = mapping_three();
        let config = config();

        let mut router =
            MovementRouter::new(
                &topology,
                &mapping,
                &config,
            )
            .expect("router creation");

        {
            let mut transaction =
                router.begin_transaction();

            transaction
                .apply(
                    MovementRequest::Swap {
                        a: PhysicalQubitId::new(0),
                        b: PhysicalQubitId::new(1),
                    },
                )
                .expect("valid swap");

            transaction
                .commit()
                .expect("commit");
        }

        assert_eq!(
            router.movement_count(),
            1
        );
    }

    #[test]
    fn blocked_qubit_rejects_swap() {
        let topology = topology_line();
        let mapping = mapping_three();
        let config = config();

        let mut router =
            MovementRouter::new(
                &topology,
                &mapping,
                &config,
            )
            .expect("router creation");

        router
            .set_physical_blocked(
                PhysicalQubitId::new(1),
                true,
            )
            .expect("valid physical qubit");

        let before = router.snapshot();

        assert!(
            router
                .swap(
                    PhysicalQubitId::new(0),
                    PhysicalQubitId::new(1),
                )
                .is_err()
        );

        assert_eq!(
            router.snapshot(),
            before
        );
    }

    #[test]
    fn empty_permutation_is_rejected() {
        let topology = topology_line();
        let mapping = mapping_three();
        let config = config();

        let mut router =
            MovementRouter::new(
                &topology,
                &mapping,
                &config,
            )
            .expect("router creation");

        assert!(
            router
                .permutation(&[])
                .is_err()
        );
    }
}