//! Zamani Quantum Error Correction — deterministic bounded partitioning.
//!
//! # Responsibility
//!
//! `partition.rs` owns the decomposition of a QEC workload into deterministic
//! computational partitions and the explicit boundary contracts required to
//! reconcile those partitions later.
//!
//! It does NOT own:
//!
//! - stabilizer algebra;
//! - decoder algorithms;
//! - decoding-graph construction;
//! - distributed transport;
//! - stream buffering;
//! - scheduling;
//! - durable checkpoint serialization;
//! - QPU transport;
//! - global resource accounting.
//!
//! # Architectural contract
//!
//! ```text
//! PartitionInput
//!      │
//!      ▼
//! structural validation
//!      │
//!      ▼
//! QecLimits preflight
//!      │
//!      ▼
//! cancellation check
//!      │
//!      ▼
//! deterministic geometry split
//!      │
//!      ├───────────────┐
//!      ▼               ▼
//! local workload    boundary surfaces
//!      │               │
//!      └───────┬───────┘
//!              ▼
//!        PartitionPlan
//!              │
//!              ▼
//! BoundaryReconciliation
//!              │
//!              ▼
//! global decoder / distributed executor
//! ```
//!
//! # Important invariants
//!
//! 1. `QecLimits` remains the canonical declarative resource policy.
//! 2. `resources.rs` owns global runtime accounting.
//! 3. `cancellation.rs` owns cancellation.
//! 4. `errors.rs` owns the public QEC error boundary.
//! 5. Partition-local decoding is never declared globally correct until
//!    boundary reconciliation has completed.
//! 6. Partition IDs are deterministic and contiguous.
//! 7. Partition bounds are disjoint and collectively cover the input geometry.
//! 8. No workload item is silently dropped.
//! 9. No syndrome event is silently dropped.
//! 10. Boundary information is never silently discarded.
//! 11. No unchecked coordinate arithmetic is used.
//! 12. Partitioning does not invent a second resource-policy structure.
//! 13. Geometry volume is NOT interpreted as physical-qubit count.
//! 14. Partitioning itself does not consume the decoder-time budget.
//! 15. Expensive callers must supply a `CancellationToken`.
//! 16. Boundary contracts preserve enough information for deterministic
//!     reconciliation.
//!
//! # Integration
//!
//! `surface_code.rs`
//!     -> supplies validated code/workload geometry.
//!
//! `syndrome.rs`
//!     -> supplies detection events.
//!
//! `decoding_graph.rs`
//!     -> may associate graph-node identifiers with boundary events.
//!
//! `decoder.rs`
//!     -> consumes partition-local work through `PartitionExecutor`.
//!
//! `streaming.rs`
//!     -> may use partition boundaries as incremental reconciliation units.
//!
//! `distributed.rs`
//!     -> may execute partitions independently and reconcile them later.
//!
//! `scheduler.rs`
//!     -> may schedule `PartitionExecutor` jobs.
//!
//! `checkpoint.rs`
//!     -> may persist `PartitionPlan`/reconciliation state using its own
//!        durable schema.
//!
//! `resources.rs`
//!     -> consumes `PartitionResources` as an operation-local accounting
//!        snapshot; it remains the owner of global runtime accounting.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! No unsafe code is required.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::cancellation::CancellationToken;
use super::errors::{
    QecError,
    QecResult,
    NumericalOperation,
    ResourceKind,
};
use super::limits::{LimitError, LimitKind, QecLimits};

/// Stable partition identifier.
pub type PartitionId = u64;

/// Stable physical-qubit identifier.
pub type QubitId = u64;

/// Stable stabilizer identifier.
pub type StabilizerId = u64;

/// Stable syndrome/detection-event identifier.
pub type EventId = u64;

/// Stable decoding-graph node identifier.
pub type GraphNodeId = u64;

/// Stable boundary reconciliation identifier.
pub type ReconciliationId = u64;

/// Current in-memory partition schema.
pub const PARTITION_SCHEMA_VERSION: u16 = 4;

/// Maximum coordinate magnitude accepted by this infrastructure layer.
///
/// This is an arithmetic-safety guard, not a QEC resource policy.
pub const DEFAULT_MAX_COORDINATE_ABS: i64 = 1_000_000_000_000;

/// ============================================================================
/// Coordinates
/// ============================================================================

/// Signed lattice coordinate.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Coordinate {
    #[must_use]
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    /// Validates coordinate-domain safety.
    pub fn validate(self) -> Result<(), PartitionError> {
        let maximum = DEFAULT_MAX_COORDINATE_ABS as u64;

        if self.x.unsigned_abs() > maximum
            || self.y.unsigned_abs() > maximum
            || self.z.unsigned_abs() > maximum
        {
            return Err(
                PartitionError::CoordinateOutOfRange {
                    coordinate: self,
                },
            );
        }

        Ok(())
    }

    /// Performs checked coordinate translation.
    pub fn checked_offset(
        self,
        dx: i64,
        dy: i64,
        dz: i64,
    ) -> Result<Self, PartitionError> {
        let coordinate = Self {
            x: self
                .x
                .checked_add(dx)
                .ok_or(PartitionError::ArithmeticOverflow)?,
            y: self
                .y
                .checked_add(dy)
                .ok_or(PartitionError::ArithmeticOverflow)?,
            z: self
                .z
                .checked_add(dz)
                .ok_or(PartitionError::ArithmeticOverflow)?,
        };

        coordinate.validate()?;

        Ok(coordinate)
    }
}

/// ============================================================================
/// Geometry
/// ============================================================================

/// Axis used for deterministic partition splitting.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum PartitionAxis {
    X,
    Y,
    Z,
}

impl PartitionAxis {
    #[must_use]
    pub const fn coordinate(
        self,
        coordinate: Coordinate,
    ) -> i64 {
        match self {
            Self::X => coordinate.x,
            Self::Y => coordinate.y,
            Self::Z => coordinate.z,
        }
    }
}

/// Inclusive rectangular/cuboid bounds.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct Bounds {
    pub min: Coordinate,
    pub max: Coordinate,
}

impl Bounds {
    pub fn new(
        min: Coordinate,
        max: Coordinate,
    ) -> Result<Self, PartitionError> {
        min.validate()?;
        max.validate()?;

        if min.x > max.x
            || min.y > max.y
            || min.z > max.z
        {
            return Err(PartitionError::InvalidBounds {
                min,
                max,
            });
        }

        Ok(Self { min, max })
    }

    #[must_use]
    pub fn contains(
        &self,
        coordinate: Coordinate,
    ) -> bool {
        coordinate.x >= self.min.x
            && coordinate.x <= self.max.x
            && coordinate.y >= self.min.y
            && coordinate.y <= self.max.y
            && coordinate.z >= self.min.z
            && coordinate.z <= self.max.z
    }

    /// Inclusive lattice length along an axis.
    pub fn axis_length(
        &self,
        axis: PartitionAxis,
    ) -> Result<u64, PartitionError> {
        checked_axis_length(
            axis.coordinate(self.min),
            axis.coordinate(self.max),
        )
    }

    /// Geometric lattice volume.
    ///
    /// This is deliberately NOT interpreted as the number of physical qubits.
    pub fn checked_volume(&self) -> Result<u64, PartitionError> {
        let x = self.axis_length(PartitionAxis::X)?;
        let y = self.axis_length(PartitionAxis::Y)?;
        let z = self.axis_length(PartitionAxis::Z)?;

        x.checked_mul(y)
            .and_then(|value| value.checked_mul(z))
            .ok_or(PartitionError::ArithmeticOverflow)
    }

    /// Returns true when two disjoint cuboids share a complete lattice face.
    ///
    /// This is the corrected adjacency test. Adjacent partitions do NOT
    /// overlap, therefore a conventional volume-overlap test is insufficient.
    #[must_use]
    pub fn shares_face(
        &self,
        other: &Self,
    ) -> bool {
        let x_overlap = intervals_overlap(
            self.min.x,
            self.max.x,
            other.min.x,
            other.max.x,
        );

        let y_overlap = intervals_overlap(
            self.min.y,
            self.max.y,
            other.min.y,
            other.max.y,
        );

        let z_overlap = intervals_overlap(
            self.min.z,
            self.max.z,
            other.min.z,
            other.max.z,
        );

        let x_adjacent =
            self.max.x.checked_add(1) == Some(other.min.x)
                || other.max.x.checked_add(1) == Some(self.min.x);

        let y_adjacent =
            self.max.y.checked_add(1) == Some(other.min.y)
                || other.max.y.checked_add(1) == Some(self.min.y);

        let z_adjacent =
            self.max.z.checked_add(1) == Some(other.min.z)
                || other.max.z.checked_add(1) == Some(self.min.z);

        (x_adjacent && y_overlap && z_overlap)
            || (y_adjacent && x_overlap && z_overlap)
            || (z_adjacent && x_overlap && y_overlap)
    }

    #[must_use]
    pub fn intersects(
        &self,
        other: &Self,
    ) -> bool {
        intervals_overlap(
            self.min.x,
            self.max.x,
            other.min.x,
            other.max.x,
        ) && intervals_overlap(
            self.min.y,
            self.max.y,
            other.min.y,
            other.max.y,
        ) && intervals_overlap(
            self.min.z,
            self.max.z,
            other.min.z,
            other.max.z,
        )
    }
}

/// ============================================================================
/// Strategy
/// ============================================================================

/// Deterministic partition strategy.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum PartitionStrategy {
    /// Split once along the longest axis.
    LongestAxis,

    /// Split once along the requested axis.
    FixedAxis(PartitionAxis),

    /// Recursively split until exactly this many partitions exist.
    FixedCount {
        partitions: usize,
    },
}

/// ============================================================================
/// Boundary identity
/// ============================================================================

/// Boundary classification.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum BoundaryKind {
    /// Internal computational boundary.
    Internal,

    /// Shared by computational partitions.
    InterPartition,

    /// Physical code boundary.
    Physical,

    /// Both physical and inter-partition.
    Mixed,
}

impl BoundaryKind {
    #[must_use]
    fn merge(
        self,
        other: Self,
    ) -> Self {
        match (self, other) {
            (Self::Mixed, _)
            | (_, Self::Mixed) => Self::Mixed,

            (Self::InterPartition, Self::Physical)
            | (Self::Physical, Self::InterPartition) => {
                Self::Mixed
            }

            (Self::Internal, value)
            | (value, Self::Internal) => value,

            (
                Self::InterPartition,
                Self::InterPartition,
            ) => Self::InterPartition,

            (Self::Physical, Self::Physical) => {
                Self::Physical
            }
        }
    }
}

/// Stable boundary identity.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct BoundaryKey {
    pub coordinate: Coordinate,
    pub round: Option<u64>,
    pub graph_node: Option<GraphNodeId>,
}

impl BoundaryKey {
    pub fn validate(&self) -> Result<(), PartitionError> {
        self.coordinate.validate()
    }
}

/// One boundary element.
///
/// Boundary elements may represent actual qubits/events or virtual
/// reconciliation surfaces.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct BoundaryElement {
    pub key: BoundaryKey,

    pub qubit_id: Option<QubitId>,

    pub event_id: Option<EventId>,

    pub kind: BoundaryKind,

    pub neighboring_partitions:
        BTreeSet<PartitionId>,

    pub virtual_boundary: bool,
}

impl BoundaryElement {
    pub fn validate(&self) -> Result<(), PartitionError> {
        self.key.validate()?;

        if self.kind == BoundaryKind::InterPartition
            && self.neighboring_partitions.is_empty()
        {
            return Err(
                PartitionError::InvalidBoundary {
                    reason:
                        "inter-partition boundary has no neighboring partition",
                },
            );
        }

        if self.kind == BoundaryKind::Physical
            && !self.neighboring_partitions.is_empty()
        {
            return Err(
                PartitionError::InvalidBoundary {
                    reason:
                        "physical-only boundary has partition neighbors",
                },
            );
        }

        Ok(())
    }
}

/// ============================================================================
/// Workload
/// ============================================================================

/// Physical/stabilizer item assigned to a partition.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct PartitionItem {
    pub qubit_id: QubitId,

    pub coordinate: Coordinate,

    pub stabilizers: Vec<StabilizerId>,
}

impl PartitionItem {
    pub fn validate(
        &self,
        limits: &QecLimits,
    ) -> Result<(), PartitionError> {
        self.coordinate.validate()?;

        limits
            .validate_stabilizer(
                self.stabilizers.len(),
            )
            .map_err(map_limit_error)?;

        let mut unique =
            BTreeSet::new();

        for stabilizer in &self.stabilizers {
            if !unique.insert(*stabilizer) {
                return Err(
                    PartitionError::DuplicateStabilizer {
                        qubit: self.qubit_id,
                        stabilizer: *stabilizer,
                    },
                );
            }
        }

        Ok(())
    }
}

/// Detection event assigned to a partition.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct PartitionEvent {
    pub event_id: EventId,

    pub coordinate: Coordinate,

    pub round: u64,

    pub graph_node: Option<GraphNodeId>,
}

impl PartitionEvent {
    pub fn validate(&self) -> Result<(), PartitionError> {
        self.coordinate.validate()
    }
}

/// A single computational partition.
#[derive(
    Debug,
    Clone,
)]
pub struct QecPartition {
    pub id: PartitionId,

    pub bounds: Bounds,

    pub items: Vec<PartitionItem>,

    pub events: Vec<PartitionEvent>,

    pub boundaries:
        Vec<BoundaryElement>,
}

impl QecPartition {
    #[must_use]
    pub fn new(
        id: PartitionId,
        bounds: Bounds,
    ) -> Self {
        Self {
            id,
            bounds,
            items: Vec::new(),
            events: Vec::new(),
            boundaries: Vec::new(),
        }
    }

    pub fn validate(
        &self,
        limits: &QecLimits,
    ) -> Result<(), PartitionError> {
        limits
            .validate_partition(
                1,
                self.items.len(),
            )
            .map_err(map_limit_error)?;

        limits
            .validate_syndrome(
                self.events.len(),
                1,
            )
            .map_err(map_limit_error)?;

        let mut qubits =
            BTreeSet::new();

        let mut events =
            BTreeSet::new();

        let mut boundary_keys =
            BTreeSet::new();

        for item in &self.items {
            item.validate(limits)?;

            if !qubits.insert(item.qubit_id) {
                return Err(
                    PartitionError::DuplicateQubit {
                        partition: self.id,
                        qubit: item.qubit_id,
                    },
                );
            }

            if !self.bounds.contains(
                item.coordinate,
            ) {
                return Err(
                    PartitionError::ItemOutsideBounds {
                        partition: self.id,
                        coordinate: item.coordinate,
                    },
                );
            }
        }

        for event in &self.events {
            event.validate()?;

            if !events.insert(event.event_id) {
                return Err(
                    PartitionError::DuplicateEvent {
                        partition: self.id,
                        event: event.event_id,
                    },
                );
            }

            if !self.bounds.contains(
                event.coordinate,
            ) {
                return Err(
                    PartitionError::ItemOutsideBounds {
                        partition: self.id,
                        coordinate: event.coordinate,
                    },
                );
            }
        }

        for boundary in &self.boundaries {
            boundary.validate()?;

            if !boundary_keys
                .insert(boundary.key.clone())
            {
                return Err(
                    PartitionError::DuplicateBoundary {
                        partition: self.id,
                        key: boundary.key.clone(),
                    },
                );
            }
        }

        Ok(())
    }
}

/// ============================================================================
/// Input
/// ============================================================================

/// Input to deterministic partition planning.
#[derive(
    Debug,
    Clone,
)]
pub struct PartitionInput {
    pub bounds: Bounds,

    pub items: Vec<PartitionItem>,

    pub events: Vec<PartitionEvent>,

    /// Physical code boundary, when known.
    pub physical_boundary:
        Option<Bounds>,
}

impl PartitionInput {
    pub fn validate(
        &self,
        limits: &QecLimits,
    ) -> Result<(), PartitionError> {
        self.bounds.min.validate()?;
        self.bounds.max.validate()?;

        if self.items.len()
            > limits.max_qubits
        {
            return Err(
                PartitionError::LimitExceeded {
                    resource:
                        LimitKind::Qubits,
                    requested:
                        self.items.len() as u128,
                    maximum:
                        limits.max_qubits as u128,
                },
            );
        }

        if self.events.len()
            > limits.max_syndrome_events
        {
            return Err(
                PartitionError::LimitExceeded {
                    resource:
                        LimitKind::SyndromeEvents,
                    requested:
                        self.events.len() as u128,
                    maximum:
                        limits.max_syndrome_events
                            as u128,
                },
            );
        }

        let mut qubits =
            BTreeSet::new();

        let mut events =
            BTreeSet::new();

        for item in &self.items {
            item.validate(limits)?;

            if !qubits.insert(item.qubit_id) {
                return Err(
                    PartitionError::DuplicateInputQubit {
                        qubit: item.qubit_id,
                    },
                );
            }

            if !self.bounds.contains(
                item.coordinate,
            ) {
                return Err(
                    PartitionError::ItemOutsideInputBounds {
                        coordinate:
                            item.coordinate,
                    },
                );
            }
        }

        for event in &self.events {
            event.validate()?;

            if !events.insert(event.event_id) {
                return Err(
                    PartitionError::DuplicateInputEvent {
                        event: event.event_id,
                    },
                );
            }

            if !self.bounds.contains(
                event.coordinate,
            ) {
                return Err(
                    PartitionError::ItemOutsideInputBounds {
                        coordinate:
                            event.coordinate,
                    },
                );
            }
        }

        if let Some(physical) =
            self.physical_boundary
        {
            physical.min.validate()?;
            physical.max.validate()?;

            if !self.bounds.contains(
                physical.min,
            ) || !self.bounds.contains(
                physical.max,
            ) {
                return Err(
                    PartitionError::InvalidPhysicalBoundary,
                );
            }
        }

        Ok(())
    }
}

/// ============================================================================
/// Resource snapshot
/// ============================================================================

/// Operation-local partitioning accounting.
///
/// This is NOT a replacement for `resources.rs`.
///
/// `resources.rs` may ingest this snapshot into the global execution
/// accounting system.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
)]
pub struct PartitionResources {
    pub items_processed: u64,

    pub events_processed: u64,

    pub partitions_created: u64,

    pub boundaries_created: u64,

    pub neighbor_links_created: u64,

    pub peak_items_per_partition: u64,

    pub peak_events_per_partition: u64,
}

impl PartitionResources {
    fn increment(
        value: &mut u64,
    ) -> Result<(), PartitionError> {
        *value = value
            .checked_add(1)
            .ok_or(
                PartitionError::ArithmeticOverflow,
            )?;

        Ok(())
    }
}

/// ============================================================================
/// Boundary reconciliation
/// ============================================================================

/// Logical parity crossing a boundary.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
)]
pub struct LogicalParity {
    pub x: bool,
    pub z: bool,
}

impl LogicalParity {
    #[must_use]
    pub const fn xor(
        self,
        other: Self,
    ) -> Self {
        Self {
            x: self.x ^ other.x,
            z: self.z ^ other.z,
        }
    }
}

/// A correction-chain relation across a boundary.
///
/// `None` permits the contract to preserve unmatched boundary elements rather
/// than silently dropping them.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct BoundaryChainLink {
    pub from: Option<BoundaryKey>,

    pub to: Option<BoundaryKey>,

    pub parity: bool,
}

impl BoundaryChainLink {
    pub fn validate(&self) -> Result<(), PartitionError> {
        if self.from.is_none()
            && self.to.is_none()
        {
            return Err(
                PartitionError::InvalidBoundaryChain,
            );
        }

        if let Some(from) = &self.from {
            from.validate()?;
        }

        if let Some(to) = &self.to {
            to.validate()?;
        }

        Ok(())
    }
}

/// Metadata required to reproduce reconciliation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct ReconciliationMetadata {
    pub schema_version: u16,

    pub partition_a: PartitionId,

    pub partition_b: PartitionId,

    pub boundary_count_a: usize,

    pub boundary_count_b: usize,
}

impl ReconciliationMetadata {
    pub fn validate(
        &self,
    ) -> Result<(), PartitionError> {
        if self.schema_version
            != PARTITION_SCHEMA_VERSION
        {
            return Err(
                PartitionError::UnsupportedSchemaVersion {
                    version:
                        self.schema_version,
                },
            );
        }

        if self.partition_a
            == self.partition_b
        {
            return Err(
                PartitionError::SelfNeighbor {
                    partition:
                        self.partition_a,
                },
            );
        }

        Ok(())
    }
}

/// Complete mathematical reconciliation contract between two partitions.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct PartitionBoundary {
    pub reconciliation_id:
        ReconciliationId,

    pub partition_a:
        PartitionId,

    pub partition_b:
        PartitionId,

    pub incoming_syndrome_state_a:
        Vec<EventId>,

    pub incoming_syndrome_state_b:
        Vec<EventId>,

    pub outgoing_syndrome_state_a:
        Vec<EventId>,

    pub outgoing_syndrome_state_b:
        Vec<EventId>,

    pub virtual_boundary_nodes:
        Vec<GraphNodeId>,

    pub correction_chain:
        Vec<BoundaryChainLink>,

    pub logical_parity:
        LogicalParity,

    pub reconciliation_metadata:
        ReconciliationMetadata,
}

impl PartitionBoundary {
    pub fn validate(
        &self,
    ) -> Result<(), PartitionError> {
        if self.partition_a
            == self.partition_b
        {
            return Err(
                PartitionError::SelfNeighbor {
                    partition:
                        self.partition_a,
                },
            );
        }

        self.reconciliation_metadata
            .validate()?;

        if self.reconciliation_metadata
            .partition_a
            != self.partition_a
            || self.reconciliation_metadata
                .partition_b
                != self.partition_b
        {
            return Err(
                PartitionError::ReconciliationMismatch,
            );
        }

        for link in
            &self.correction_chain
        {
            link.validate()?;
        }

        Ok(())
    }
}

/// Compatibility view for downstream distributed/streaming code.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct BoundaryReconciliation {
    pub partition_a:
        PartitionId,

    pub partition_b:
        PartitionId,

    pub boundaries_a:
        Vec<BoundaryElement>,

    pub boundaries_b:
        Vec<BoundaryElement>,

    pub contract:
        PartitionBoundary,
}

impl BoundaryReconciliation {
    pub fn validate(
        &self,
    ) -> Result<(), PartitionError> {
        self.contract.validate()?;

        if self.partition_a
            != self.contract.partition_a
            || self.partition_b
                != self.contract.partition_b
        {
            return Err(
                PartitionError::ReconciliationMismatch,
            );
        }

        if self.boundaries_a.is_empty()
            || self.boundaries_b.is_empty()
        {
            return Err(
                PartitionError::MissingBoundaryData {
                    a: self.partition_a,
                    b: self.partition_b,
                },
            );
        }

        Ok(())
    }
}

/// ============================================================================
/// Partition plan
/// ============================================================================

/// Deterministic partitioning result.
#[derive(
    Debug,
    Clone,
)]
pub struct PartitionPlan {
    pub schema_version: u16,

    pub partitions:
        Vec<QecPartition>,

    pub adjacency:
        BTreeMap<
            PartitionId,
            BTreeSet<PartitionId>,
        >,

    pub boundaries:
        BTreeMap<
            PartitionId,
            Vec<BoundaryElement>,
        >,

    pub boundary_contracts:
        Vec<PartitionBoundary>,

    pub resources:
        PartitionResources,

    /// Always true for a partitioned decoding workload.
    pub requires_reconciliation:
        bool,
}

impl PartitionPlan {
    pub fn validate(
        &self,
        limits: &QecLimits,
    ) -> Result<(), PartitionError> {
        if self.schema_version
            != PARTITION_SCHEMA_VERSION
        {
            return Err(
                PartitionError::UnsupportedSchemaVersion {
                    version:
                        self.schema_version,
                },
            );
        }

        if self.partitions.is_empty() {
            return Err(
                PartitionError::NoPartitions,
            );
        }

        limits
            .validate_partition(
                self.partitions.len(),
                maximum_partition_size(
                    &self.partitions,
                ),
            )
            .map_err(map_limit_error)?;

        let mut ids =
            BTreeSet::new();

        for partition in
            &self.partitions
        {
            if !ids.insert(
                partition.id,
            ) {
                return Err(
                    PartitionError::DuplicatePartition {
                        partition:
                            partition.id,
                    },
                );
            }

            partition.validate(
                limits,
            )?;
        }

        /*
         * Bounds must be pairwise disjoint.
         *
         * A valid partition plan must never assign one coordinate to two
         * partitions.
         */
        for i in 0..self.partitions.len() {
            for j in
                (i + 1)..self.partitions.len()
            {
                if self.partitions[i]
                    .bounds
                    .intersects(
                        &self.partitions[j]
                            .bounds,
                    )
                {
                    return Err(
                        PartitionError::OverlappingPartitions {
                            a: self.partitions[i]
                                .id,
                            b: self.partitions[j]
                                .id,
                        },
                    );
                }
            }
        }

        for (
            partition,
            neighbors,
        ) in &self.adjacency
        {
            if !ids.contains(
                partition,
            ) {
                return Err(
                    PartitionError::UnknownPartition {
                        partition:
                            *partition,
                    },
                );
            }

            for neighbor in
                neighbors
            {
                if *partition
                    == *neighbor
                {
                    return Err(
                        PartitionError::SelfNeighbor {
                            partition:
                                *partition,
                        },
                    );
                }

                if !ids.contains(
                    neighbor,
                ) {
                    return Err(
                        PartitionError::UnknownPartition {
                            partition:
                                *neighbor,
                        },
                    );
                }

                let symmetric =
                    self.adjacency
                        .get(neighbor)
                        .is_some_and(
                            |set| {
                                set.contains(
                                    partition,
                                )
                            },
                        );

                if !symmetric {
                    return Err(
                        PartitionError::AsymmetricAdjacency {
                            a:
                                *partition,
                            b:
                                *neighbor,
                        },
                    );
                }
            }
        }

        for contract in
            &self.boundary_contracts
        {
            contract.validate()?;

            let adjacent =
                self.adjacency
                    .get(
                        &contract
                            .partition_a,
                    )
                    .is_some_and(
                        |neighbors| {
                            neighbors
                                .contains(
                                    &contract
                                        .partition_b,
                                )
                        },
                    );

            if !adjacent {
                return Err(
                    PartitionError::BoundaryWithoutAdjacency {
                        a:
                            contract
                                .partition_a,
                        b:
                            contract
                                .partition_b,
                    },
                );
            }
        }

        Ok(())
    }
}

/// ============================================================================
/// Partitioner
/// ============================================================================

/// Deterministic bounded partition planner.
#[derive(
    Debug,
    Clone,
    Copy,
)]
pub struct Partitioner {
    strategy:
        PartitionStrategy,

    limits:
        QecLimits,
}

impl Partitioner {
    pub fn new(
        strategy: PartitionStrategy,
        limits: QecLimits,
    ) -> QecResult<Self> {
        limits
            .validate()
            .map_err(map_limit_error)?;

        if let PartitionStrategy::FixedCount {
            partitions,
        } = strategy
        {
            if partitions == 0 {
                return Err(
                    QecError::invalid_input(
                        "partition count must be greater than zero",
                    ),
                );
            }

            limits
                .validate_partition(
                    partitions,
                    1,
                )
                .map_err(map_limit_error)?;
        }

        Ok(Self {
            strategy,
            limits,
        })
    }

    #[must_use]
    pub const fn strategy(
        &self,
    ) -> PartitionStrategy {
        self.strategy
    }

    #[must_use]
    pub const fn limits(
        &self,
    ) -> QecLimits {
        self.limits
    }

    /// Creates a partition plan using a fresh active cancellation token.
    pub fn partition(
        &self,
        input: PartitionInput,
    ) -> QecResult<PartitionPlan> {
        let cancellation =
            CancellationToken::new();

        self.partition_with_cancellation(
            input,
            &cancellation,
        )
    }

    /// Creates a partition plan using caller-owned cancellation.
    pub fn partition_with_cancellation(
        &self,
        input: PartitionInput,
        cancellation:
            &CancellationToken,
    ) -> QecResult<PartitionPlan> {
        cancellation.check()?;

        input
            .validate(&self.limits)
            .map_err(PartitionError::into_qec_error)?;

        let bounds =
            self.calculate_bounds(
                &input.bounds,
            )?;

        cancellation.check()?;

        self.limits
            .validate_partition(
                bounds.len(),
                maximum_partition_size_by_bounds(
                    &input,
                    &bounds,
                ),
            )
            .map_err(map_limit_error)?;

        let mut partitions =
            Vec::with_capacity(
                bounds.len(),
            );

        for (
            index,
            bounds,
        ) in bounds.into_iter().enumerate()
        {
            cancellation.check()?;

            let id =
                u64::try_from(index)
                    .map_err(
                        |_| {
                            QecError::numerical_failure(
                                NumericalOperation::IntegerConversion,
                                "partition ID conversion overflow",
                            )
                        },
                    )?;

            partitions.push(
                QecPartition::new(
                    id,
                    bounds,
                ),
            );
        }

        let mut resources =
            PartitionResources {
                partitions_created:
                    partitions.len()
                        as u64,
                ..Default::default()
            };

        self.assign_items(
            &input,
            &mut partitions,
            &mut resources,
            cancellation,
        )?;

        self.assign_events(
            &input,
            &mut partitions,
            &mut resources,
            cancellation,
        )?;

        self.construct_boundaries(
            &input,
            &mut partitions,
            &mut resources,
            cancellation,
        )?;

        cancellation.check()?;

        let adjacency =
            build_adjacency(
                &partitions,
            )?;

        let boundaries =
            partitions
                .iter()
                .map(
                    |partition| {
                        (
                            partition.id,
                            partition
                                .boundaries
                                .clone(),
                        )
                    },
                )
                .collect();

        let boundary_contracts =
            build_boundary_contracts(
                &partitions,
                &adjacency,
                cancellation,
            )?;

        let plan =
            PartitionPlan {
                schema_version:
                    PARTITION_SCHEMA_VERSION,
                partitions,
                adjacency,
                boundaries,
                boundary_contracts,
                resources,
                requires_reconciliation:
                    true,
            };

        plan.validate(&self.limits)
            .map_err(
                PartitionError::into_qec_error,
            )?;

        Ok(plan)
    }

    fn calculate_bounds(
        &self,
        bounds: &Bounds,
    ) -> Result<
        Vec<Bounds>,
        PartitionError,
    > {
        match self.strategy {
            PartitionStrategy::LongestAxis => {
                let axis =
                    longest_axis(bounds)?;

                self.split_axis(
                    bounds,
                    axis,
                )
            }

            PartitionStrategy::FixedAxis(
                axis,
            ) => {
                self.split_axis(
                    bounds,
                    axis,
                )
            }

            PartitionStrategy::FixedCount {
                partitions,
            } => {
                self.split_fixed_count(
                    bounds,
                    partitions,
                )
            }
        }
    }

    fn split_axis(
        &self,
        bounds: &Bounds,
        axis: PartitionAxis,
    ) -> Result<
        Vec<Bounds>,
        PartitionError,
    > {
        let length =
            bounds.axis_length(axis)?;

        if length <= 1 {
            return Ok(vec![*bounds]);
        }

        let minimum =
            axis.coordinate(
                bounds.min,
            );

        let maximum =
            axis.coordinate(
                bounds.max,
            );

        let difference =
            maximum
                .checked_sub(minimum)
                .ok_or(
                    PartitionError::ArithmeticOverflow,
                )?;

        let midpoint =
            minimum
                .checked_add(
                    difference / 2,
                )
                .ok_or(
                    PartitionError::ArithmeticOverflow,
                )?;

        let right_min =
            midpoint
                .checked_add(1)
                .ok_or(
                    PartitionError::ArithmeticOverflow,
                )?;

        let left =
            replace_axis(
                *bounds,
                axis,
                minimum,
                midpoint,
            )?;

        let right =
            replace_axis(
                *bounds,
                axis,
                right_min,
                maximum,
            )?;

        Ok(vec![
            left,
            right,
        ])
    }

    fn split_fixed_count(
        &self,
        bounds: &Bounds,
        count: usize,
    ) -> Result<
        Vec<Bounds>,
        PartitionError,
    > {
        if count == 0 {
            return Err(
                PartitionError::InvalidPartitionCount,
            );
        }

        if count == 1 {
            return Ok(vec![*bounds]);
        }

        if count
            > self.limits.max_partitions
        {
            return Err(
                PartitionError::LimitExceeded {
                    resource:
                        LimitKind::Partitions,
                    requested:
                        count as u128,
                    maximum:
                        self.limits.max_partitions
                            as u128,
                },
            );
        }

        let mut result =
            vec![*bounds];

        while result.len() < count {
            let candidate_index =
                result
                    .iter()
                    .enumerate()
                    .filter_map(
                        |(index, candidate)| {
                            let axis =
                                longest_axis(
                                    candidate,
                                )
                                .ok()?;

                            let length =
                                candidate
                                    .axis_length(
                                        axis,
                                    )
                                    .ok()?;

                            if length <= 1 {
                                None
                            } else {
                                Some(
                                    (
                                        length,
                                        index,
                                    ),
                                )
                            }
                        },
                    )
                    .max_by(
                        |left, right| {
                            left.0
                                .cmp(
                                    &right.0,
                                )
                                .then_with(
                                    || {
                                        /*
                                         * Stable tie-break:
                                         * lower index wins.
                                         */
                                        right.1
                                            .cmp(
                                                &left.1,
                                            )
                                    },
                                )
                        },
                    )
                    .map(
                        |(_, index)| index,
                    )
                    .ok_or(
                        PartitionError::UnableToSplit {
                            requested:
                                count,
                            achieved:
                                result.len(),
                        },
                    )?;

            let candidate =
                result.remove(
                    candidate_index,
                );

            let axis =
                longest_axis(
                    &candidate,
                )?;

            let pieces =
                self.split_axis(
                    &candidate,
                    axis,
                )?;

            if pieces.len()
                != 2
            {
                return Err(
                    PartitionError::UnableToSplit {
                        requested:
                            count,
                        achieved:
                            result.len(),
                    },
                );
            }

            result.extend(
                pieces,
            );

            result.sort_by_key(
                |bounds| {
                    (
                        bounds.min,
                        bounds.max,
                    )
                },
            );
        }

        Ok(result)
    }

    fn assign_items(
        &self,
        input: &PartitionInput,
        partitions:
            &mut [QecPartition],
        resources:
            &mut PartitionResources,
        cancellation:
            &CancellationToken,
    ) -> Result<(), PartitionError> {
        for item in
            &input.items
        {
            cancellation
                .check()
                .map_err(
                    PartitionError::from_qec_error,
                )?;

            let index =
                find_partition_index(
                    partitions,
                    item.coordinate,
                )
                .ok_or(
                    PartitionError::UnassignedItem {
                        qubit:
                            item.qubit_id,
                    },
                )?;

            let partition =
                &mut partitions[index];

            if partition.items.len()
                >= self
                    .limits
                    .max_qubits_per_partition
            {
                return Err(
                    PartitionError::LimitExceeded {
                        resource:
                            LimitKind::QubitsPerPartition,
                        requested:
                            (partition.items.len()
                                + 1)
                                as u128,
                        maximum:
                            self.limits
                                .max_qubits_per_partition
                                as u128,
                    },
                );
            }

            partition
                .items
                .push(item.clone());

            PartitionResources::increment(
                &mut resources.items_processed,
            )?;

            resources
                .peak_items_per_partition =
                resources
                    .peak_items_per_partition
                    .max(
                        partition.items.len()
                            as u64,
                    );
        }

        Ok(())
    }

    fn assign_events(
        &self,
        input: &PartitionInput,
        partitions:
            &mut [QecPartition],
        resources:
            &mut PartitionResources,
        cancellation:
            &CancellationToken,
    ) -> Result<(), PartitionError> {
        for event in
            &input.events
        {
            cancellation
                .check()
                .map_err(
                    PartitionError::from_qec_error,
                )?;

            let index =
                find_partition_index(
                    partitions,
                    event.coordinate,
                )
                .ok_or(
                    PartitionError::UnassignedEvent {
                        event:
                            event.event_id,
                    },
                )?;

            let partition =
                &mut partitions[index];

            if partition.events.len()
                >= self
                    .limits
                    .max_syndrome_events
            {
                return Err(
                    PartitionError::LimitExceeded {
                        resource:
                            LimitKind::SyndromeEvents,
                        requested:
                            (partition.events.len()
                                + 1)
                                as u128,
                        maximum:
                            self.limits
                                .max_syndrome_events
                                as u128,
                    },
                );
            }

            partition
                .events
                .push(event.clone());

            PartitionResources::increment(
                &mut resources.events_processed,
            )?;

            resources
                .peak_events_per_partition =
                resources
                    .peak_events_per_partition
                    .max(
                        partition.events.len()
                            as u64,
                    );
        }

        Ok(())
    }

    fn construct_boundaries(
        &self,
        input: &PartitionInput,
        partitions:
            &mut [QecPartition],
        resources:
            &mut PartitionResources,
        cancellation:
            &CancellationToken,
    ) -> Result<(), PartitionError> {
        /*
         * Inter-partition boundaries.
         *
         * Because `shares_face()` correctly handles disjoint adjacent
         * cuboids, this loop establishes deterministic adjacency.
         */
        for i in 0..partitions.len() {
            cancellation
                .check()
                .map_err(
                    PartitionError::from_qec_error,
                )?;

            for j in
                (i + 1)..partitions.len()
            {
                cancellation
                    .check()
                    .map_err(
                        PartitionError::from_qec_error,
                    )?;

                if !partitions[i]
                    .bounds
                    .shares_face(
                        &partitions[j]
                            .bounds,
                    )
                {
                    continue;
                }

                let a_id =
                    partitions[i].id;

                let b_id =
                    partitions[j].id;

                let coords_a =
                    shared_boundary_coordinates(
                        &partitions[i],
                        &partitions[j],
                    );

                let coords_b =
                    shared_boundary_coordinates(
                        &partitions[j],
                        &partitions[i],
                    );

                if coords_a.is_empty()
                    && coords_b.is_empty()
                {
                    let anchor =
                        shared_face_anchor(
                            &partitions[i]
                                .bounds,
                            &partitions[j]
                                .bounds,
                        )?;

                    add_boundary(
                        &mut partitions[i],
                        BoundaryElement {
                            key:
                                BoundaryKey {
                                    coordinate:
                                        anchor,
                                    round:
                                        None,
                                    graph_node:
                                        None,
                                },
                            qubit_id:
                                None,
                            event_id:
                                None,
                            kind:
                                BoundaryKind::InterPartition,
                            neighboring_partitions:
                                BTreeSet::from(
                                    [b_id],
                                ),
                            virtual_boundary:
                                true,
                        },
                        self.limits
                            .max_graph_nodes,
                        resources,
                    )?;

                    add_boundary(
                        &mut partitions[j],
                        BoundaryElement {
                            key:
                                BoundaryKey {
                                    coordinate:
                                        anchor,
                                    round:
                                        None,
                                    graph_node:
                                        None,
                                },
                            qubit_id:
                                None,
                            event_id:
                                None,
                            kind:
                                BoundaryKind::InterPartition,
                            neighboring_partitions:
                                BTreeSet::from(
                                    [a_id],
                                ),
                            virtual_boundary:
                                true,
                        },
                        self.limits
                            .max_graph_nodes,
                        resources,
                    )?;

                    continue;
                }

                for coordinate in
                    coords_a
                {
                    add_boundary(
                        &mut partitions[i],
                        boundary_for_coordinate(
                            &partitions[i],
                            coordinate,
                            b_id,
                        ),
                        self.limits
                            .max_graph_nodes,
                        resources,
                    )?;
                }

                for coordinate in
                    coords_b
                {
                    add_boundary(
                        &mut partitions[j],
                        boundary_for_coordinate(
                            &partitions[j],
                            coordinate,
                            a_id,
                        ),
                        self.limits
                            .max_graph_nodes,
                        resources,
                    )?;
                }
            }
        }

        /*
         * Physical boundaries are handled independently from inter-partition
         * boundaries so a mixed boundary remains explicitly identifiable.
         */
        if let Some(physical) =
            input.physical_boundary
        {
            for partition in
                partitions
            {
                cancellation
                    .check()
                    .map_err(
                        PartitionError::from_qec_error,
                    )?;

                if !partition
                    .bounds
                    .intersects(
                        &physical,
                    )
                {
                    continue;
                }

                for coordinate in
                    physical_boundary_coordinates(
                        partition,
                        &physical,
                    )
                {
                    let key =
                        BoundaryKey {
                            coordinate,
                            round: None,
                            graph_node:
                                find_event_at(
                                    partition,
                                    coordinate,
                                )
                                .and_then(
                                    |event_id| {
                                        partition
                                            .events
                                            .iter()
                                            .find(
                                                |event| {
                                                    event.event_id
                                                        == event_id
                                                },
                                            )
                                            .and_then(
                                                |event| {
                                                    event.graph_node
                                                },
                                            )
                                    },
                                ),
                        };

                    let existing =
                        partition
                            .boundaries
                            .iter_mut()
                            .find(
                                |boundary| {
                                    boundary
                                        .key
                                        == key
                                },
                            );

                    if let Some(existing) =
                        existing
                    {
                        existing.kind =
                            existing.kind.merge(
                                BoundaryKind::Physical,
                            );
                    } else {
                        add_boundary(
                            partition,
                            BoundaryElement {
                                key,
                                qubit_id:
                                    find_qubit_at(
                                        partition,
                                        coordinate,
                                    ),
                                event_id:
                                    find_event_at(
                                        partition,
                                        coordinate,
                                    ),
                                kind:
                                    BoundaryKind::Physical,
                                neighboring_partitions:
                                    BTreeSet::new(),
                                virtual_boundary:
                                    false,
                            },
                            self.limits
                                .max_graph_nodes,
                            resources,
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}

/// ============================================================================
/// Boundary construction
/// ============================================================================

/// Builds one deterministic reconciliation contract per adjacent partition
/// pair.
pub fn build_boundary_contracts(
    partitions:
        &[QecPartition],
    adjacency:
        &BTreeMap<
            PartitionId,
            BTreeSet<PartitionId>,
        >,
    cancellation:
        &CancellationToken,
) -> Result<
    Vec<PartitionBoundary>,
    PartitionError,
> {
    let mut contracts =
        Vec::new();

    let mut next_id =
        0_u64;

    for (
        a,
        neighbors,
    ) in adjacency
    {
        cancellation
            .check()
            .map_err(
                PartitionError::from_qec_error,
            )?;

        for b in
            neighbors
        {
            /*
             * Each unordered pair is represented once.
             */
            if a >= b {
                continue;
            }

            let partition_a =
                partitions
                    .iter()
                    .find(
                        |partition| {
                            partition.id
                                == *a
                        },
                    )
                    .ok_or(
                        PartitionError::UnknownPartition {
                            partition:
                                *a,
                        },
                    )?;

            let partition_b =
                partitions
                    .iter()
                    .find(
                        |partition| {
                            partition.id
                                == *b
                        },
                    )
                    .ok_or(
                        PartitionError::UnknownPartition {
                            partition:
                                *b,
                        },
                    )?;

            let boundaries_a =
                partition_a
                    .boundaries
                    .iter()
                    .filter(
                        |boundary| {
                            boundary
                                .neighboring_partitions
                                .contains(
                                    b,
                                )
                        },
                    )
                    .collect::<Vec<_>>();

            let boundaries_b =
                partition_b
                    .boundaries
                    .iter()
                    .filter(
                        |boundary| {
                            boundary
                                .neighboring_partitions
                                .contains(
                                    a,
                                )
                        },
                    )
                    .collect::<Vec<_>>();

            if boundaries_a.is_empty()
                || boundaries_b.is_empty()
            {
                return Err(
                    PartitionError::MissingBoundaryData {
                        a: *a,
                        b: *b,
                    },
                );
            }

            let incoming_a =
                boundary_event_ids(
                    &boundaries_a,
                );

            let incoming_b =
                boundary_event_ids(
                    &boundaries_b,
                );

            let virtual_nodes =
                boundaries_a
                    .iter()
                    .chain(
                        boundaries_b
                            .iter(),
                    )
                    .filter_map(
                        |boundary| {
                            boundary
                                .key
                                .graph_node
                        },
                    )
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>();

            let correction_chain =
                build_boundary_chain(
                    &boundaries_a,
                    &boundaries_b,
                )?;

            let contract =
                PartitionBoundary {
                    reconciliation_id:
                        next_id,

                    partition_a:
                        *a,

                    partition_b:
                        *b,

                    incoming_syndrome_state_a:
                        incoming_a.clone(),

                    incoming_syndrome_state_b:
                        incoming_b.clone(),

                    outgoing_syndrome_state_a:
                        incoming_a,

                    outgoing_syndrome_state_b:
                        incoming_b,

                    virtual_boundary_nodes:
                        virtual_nodes,

                    correction_chain,

                    logical_parity:
                        LogicalParity::default(),

                    reconciliation_metadata:
                        ReconciliationMetadata {
                            schema_version:
                                PARTITION_SCHEMA_VERSION,

                            partition_a:
                                *a,

                            partition_b:
                                *b,

                            boundary_count_a:
                                boundaries_a
                                    .len(),

                            boundary_count_b:
                                boundaries_b
                                    .len(),
                        },
                };

            contract.validate()?;

            contracts.push(
                contract,
            );

            next_id =
                next_id
                    .checked_add(1)
                    .ok_or(
                        PartitionError::ArithmeticOverflow,
                    )?;
        }
    }

    Ok(contracts)
}

/// Creates reconciliation views for a complete plan.
pub fn reconciliation_units(
    plan:
        &PartitionPlan,
    limits:
        &QecLimits,
    cancellation:
        &CancellationToken,
) -> QecResult<
    Vec<BoundaryReconciliation>,
> {
    plan.validate(limits)
        .map_err(
            PartitionError::into_qec_error,
        )?;

    let mut units =
        Vec::with_capacity(
            plan.boundary_contracts.len(),
        );

    for contract in
        &plan.boundary_contracts
    {
        cancellation.check()?;

        let boundaries_a =
            plan.boundaries
                .get(
                    &contract
                        .partition_a,
                )
                .ok_or_else(
                    || {
                        QecError::invalid_topology(
                            "boundary contract references unknown partition",
                        )
                    },
                )?
                .iter()
                .filter(
                    |boundary| {
                        boundary
                            .neighboring_partitions
                            .contains(
                                &contract
                                    .partition_b,
                            )
                    },
                )
                .cloned()
                .collect::<Vec<_>>();

        let boundaries_b =
            plan.boundaries
                .get(
                    &contract
                        .partition_b,
                )
                .ok_or_else(
                    || {
                        QecError::invalid_topology(
                            "boundary contract references unknown partition",
                        )
                    },
                )?
                .iter()
                .filter(
                    |boundary| {
                        boundary
                            .neighboring_partitions
                            .contains(
                                &contract
                                    .partition_a,
                            )
                    },
                )
                .cloned()
                .collect::<Vec<_>>();

        let unit =
            BoundaryReconciliation {
                partition_a:
                    contract.partition_a,

                partition_b:
                    contract.partition_b,

                boundaries_a,

                boundaries_b,

                contract:
                    contract.clone(),
            };

        unit.validate()
            .map_err(
                PartitionError::into_qec_error,
            )?;

        units.push(
            unit,
        );
    }

    Ok(units)
}

/// ============================================================================
/// Partition execution integration
/// ============================================================================

/// Backend-independent local partition executor.
///
/// `distributed.rs` and `scheduler.rs` can implement orchestration around
/// this trait without making `partition.rs` aware of transport or scheduling.
pub trait PartitionExecutor:
    Send + Sync
{
    type Output;

    fn execute(
        &self,
        partition:
            &QecPartition,
        cancellation:
            &CancellationToken,
    ) -> QecResult<Self::Output>;
}

/// Deterministic collection of partition-local results.
#[derive(
    Debug,
    Clone,
)]
pub struct PartitionExecution<O> {
    pub results:
        BTreeMap<
            PartitionId,
            O,
        >,

    pub adjacency:
        BTreeMap<
            PartitionId,
            BTreeSet<PartitionId>,
        >,

    pub boundary_contracts:
        Vec<PartitionBoundary>,

    /// Always true until an explicit reconciliation phase completes.
    pub requires_boundary_reconciliation:
        bool,
}

impl<O> PartitionExecution<O> {
    /// Executes partitions in deterministic ID order.
    pub fn execute(
        plan:
            &PartitionPlan,
        limits:
            &QecLimits,
        executor:
            &impl PartitionExecutor<
                Output = O,
            >,
        cancellation:
            &CancellationToken,
    ) -> QecResult<Self> {
        plan.validate(limits)
            .map_err(
                PartitionError::into_qec_error,
            )?;

        let mut results =
            BTreeMap::new();

        for partition in
            &plan.partitions
        {
            cancellation.check()?;

            let result =
                executor.execute(
                    partition,
                    cancellation,
                )?;

            if results
                .insert(
                    partition.id,
                    result,
                )
                .is_some()
            {
                return Err(
                    QecError::internal_invariant(
                        "partition execution produced duplicate partition ID",
                        "partition IDs must be unique",
                    ),
                );
            }
        }

        Ok(Self {
            results,
            adjacency:
                plan.adjacency.clone(),
            boundary_contracts:
                plan.boundary_contracts
                    .clone(),
            requires_boundary_reconciliation:
                true,
        })
    }
}

/// ============================================================================
/// Geometry helpers
/// ============================================================================

fn checked_axis_length(
    min: i64,
    max: i64,
) -> Result<u64, PartitionError> {
    if min > max {
        return Err(
            PartitionError::InvalidBounds {
                min:
                    Coordinate::new(
                        min,
                        0,
                        0,
                    ),
                max:
                    Coordinate::new(
                        max,
                        0,
                        0,
                    ),
            },
        );
    }

    let difference =
        max.checked_sub(min)
            .ok_or(
                PartitionError::ArithmeticOverflow,
            )?;

    let length =
        difference.checked_add(1)
            .ok_or(
                PartitionError::ArithmeticOverflow,
            )?;

    u64::try_from(length)
        .map_err(
            |_| {
                PartitionError::ArithmeticOverflow
            },
        )
}

fn intervals_overlap(
    a_min: i64,
    a_max: i64,
    b_min: i64,
    b_max: i64,
) -> bool {
    a_min <= b_max
        && b_min <= a_max
}

fn longest_axis(
    bounds:
        &Bounds,
) -> Result<
    PartitionAxis,
    PartitionError,
> {
    let x =
        bounds.axis_length(
            PartitionAxis::X,
        )?;

    let y =
        bounds.axis_length(
            PartitionAxis::Y,
        )?;

    let z =
        bounds.axis_length(
            PartitionAxis::Z,
        )?;

    /*
     * Ties are deliberately resolved X > Y > Z.
     */
    if x >= y && x >= z {
        Ok(PartitionAxis::X)
    } else if y >= x && y >= z {
        Ok(PartitionAxis::Y)
    } else {
        Ok(PartitionAxis::Z)
    }
}

fn replace_axis(
    bounds:
        Bounds,
    axis:
        PartitionAxis,
    min:
        i64,
    max:
        i64,
) -> Result<
    Bounds,
    PartitionError,
> {
    let mut lower =
        bounds.min;

    let mut upper =
        bounds.max;

    match axis {
        PartitionAxis::X => {
            lower.x = min;
            upper.x = max;
        }

        PartitionAxis::Y => {
            lower.y = min;
            upper.y = max;
        }

        PartitionAxis::Z => {
            lower.z = min;
            upper.z = max;
        }
    }

    Bounds::new(
        lower,
        upper,
    )
}

fn find_partition_index(
    partitions:
        &[QecPartition],
    coordinate:
        Coordinate,
) -> Option<usize> {
    partitions
        .iter()
        .position(
            |partition| {
                partition
                    .bounds
                    .contains(
                        coordinate,
                    )
            },
        )
}

fn maximum_partition_size(
    partitions:
        &[QecPartition],
) -> usize {
    partitions
        .iter()
        .map(
            |partition| {
                partition
                    .items
                    .len()
            },
        )
        .max()
        .unwrap_or(1)
}

fn maximum_partition_size_by_bounds(
    input:
        &PartitionInput,
    bounds:
        &[Bounds],
) -> usize {
    bounds
        .iter()
        .map(
            |partition_bounds| {
                input
                    .items
                    .iter()
                    .filter(
                        |item| {
                            partition_bounds
                                .contains(
                                    item.coordinate,
                                )
                        },
                    )
                    .count()
            },
        )
        .max()
        .unwrap_or(1)
}

/// ============================================================================
/// Boundary helpers
/// ============================================================================

fn find_qubit_at(
    partition:
        &QecPartition,
    coordinate:
        Coordinate,
) -> Option<QubitId> {
    partition
        .items
        .iter()
        .find(
            |item| {
                item.coordinate
                    == coordinate
            },
        )
        .map(
            |item| {
                item.qubit_id
            },
        )
}

fn find_event_at(
    partition:
        &QecPartition,
    coordinate:
        Coordinate,
) -> Option<EventId> {
    partition
        .events
        .iter()
        .find(
            |event| {
                event.coordinate
                    == coordinate
            },
        )
        .map(
            |event| {
                event.event_id
            },
        )
}

fn find_graph_node_at(
    partition:
        &QecPartition,
    coordinate:
        Coordinate,
) -> Option<GraphNodeId> {
    partition
        .events
        .iter()
        .find(
            |event| {
                event.coordinate
                    == coordinate
            },
        )
        .and_then(
            |event| {
                event.graph_node
            },
        )
}

fn shared_boundary_coordinates(
    a:
        &QecPartition,
    b:
        &QecPartition,
) -> Vec<Coordinate> {
    let mut coordinates =
        BTreeSet::new();

    for item in
        &a.items
    {
        if coordinate_is_on_face_with(
            item.coordinate,
            &a.bounds,
            &b.bounds,
        ) {
            coordinates.insert(
                item.coordinate,
            );
        }
    }

    for event in
        &a.events
    {
        if coordinate_is_on_face_with(
            event.coordinate,
            &a.bounds,
            &b.bounds,
        ) {
            coordinates.insert(
                event.coordinate,
            );
        }
    }

    coordinates
        .into_iter()
        .collect()
}

fn coordinate_is_on_face_with(
    coordinate:
        Coordinate,
    own:
        &Bounds,
    other:
        &Bounds,
) -> bool {
    if !own.contains(
        coordinate,
    ) {
        return false;
    }

    let x_overlap =
        intervals_overlap(
            own.min.x,
            own.max.x,
            other.min.x,
            other.max.x,
        );

    let y_overlap =
        intervals_overlap(
            own.min.y,
            own.max.y,
            other.min.y,
            other.max.y,
        );

    let z_overlap =
        intervals_overlap(
            own.min.z,
            own.max.z,
            other.min.z,
            other.max.z,
        );

    let x_face =
        (own.max.x.checked_add(1)
            == Some(other.min.x)
            && coordinate.x
                == own.max.x)
            || (other.max.x.checked_add(1)
                == Some(own.min.x)
                && coordinate.x
                    == own.min.x);

    let y_face =
        (own.max.y.checked_add(1)
            == Some(other.min.y)
            && coordinate.y
                == own.max.y)
            || (other.max.y.checked_add(1)
                == Some(own.min.y)
                && coordinate.y
                    == own.min.y);

    let z_face =
        (own.max.z.checked_add(1)
            == Some(other.min.z)
            && coordinate.z
                == own.max.z)
            || (other.max.z.checked_add(1)
                == Some(own.min.z)
                && coordinate.z
                    == own.min.z);

    (x_face && y_overlap && z_overlap)
        || (y_face && x_overlap && z_overlap)
        || (z_face && x_overlap && y_overlap)
}

fn shared_face_anchor(
    a:
        &Bounds,
    b:
        &Bounds,
) -> Result<
    Coordinate,
    PartitionError,
> {
    let x =
        if a.max.x.checked_add(1)
            == Some(b.min.x)
        {
            a.max.x
        } else if b.max.x.checked_add(1)
            == Some(a.min.x)
        {
            b.max.x
        } else {
            a.min.x.max(
                b.min.x,
            )
        };

    let y =
        a.min.y.max(
            b.min.y,
        );

    let z =
        a.min.z.max(
            b.min.z,
        );

    let coordinate =
        Coordinate::new(
            x,
            y,
            z,
        );

    coordinate.validate()?;

    Ok(coordinate)
}

fn boundary_for_coordinate(
    partition:
        &QecPartition,
    coordinate:
        Coordinate,
    neighbor:
        PartitionId,
) -> BoundaryElement {
    BoundaryElement {
        key:
            BoundaryKey {
                coordinate,
                round: None,
                graph_node:
                    find_graph_node_at(
                        partition,
                        coordinate,
                    ),
            },

        qubit_id:
            find_qubit_at(
                partition,
                coordinate,
            ),

        event_id:
            find_event_at(
                partition,
                coordinate,
            ),

        kind:
            BoundaryKind::InterPartition,

        neighboring_partitions:
            BTreeSet::from(
                [neighbor],
            ),

        virtual_boundary:
            false,
    }
}

fn add_boundary(
    partition:
        &mut QecPartition,
    boundary:
        BoundaryElement,
    maximum:
        usize,
    resources:
        &mut PartitionResources,
) -> Result<(), PartitionError> {
    boundary.validate()?;

    if let Some(existing) =
        partition
            .boundaries
            .iter_mut()
            .find(
                |existing| {
                    existing.key
                        == boundary.key
                },
            )
    {
        existing.kind =
            existing.kind.merge(
                boundary.kind,
            );

        existing
            .neighboring_partitions
            .extend(
                boundary
                    .neighboring_partitions,
            );

        existing.qubit_id =
            existing
                .qubit_id
                .or(
                    boundary.qubit_id,
                );

        existing.event_id =
            existing
                .event_id
                .or(
                    boundary.event_id,
                );

        existing.virtual_boundary &=
            boundary.virtual_boundary;

        return Ok(());
    }

    /*
     * `max_graph_nodes` is the closest canonical limit for explicit
     * reconciliation surfaces because these boundaries become graph-facing
     * interface nodes. No new production limit is invented here.
     */
    if partition
        .boundaries
        .len()
        >= maximum
    {
        return Err(
            PartitionError::LimitExceeded {
                resource:
                    LimitKind::GraphNodes,
                requested:
                    (partition
                        .boundaries
                        .len()
                        + 1)
                        as u128,
                maximum:
                    maximum as u128,
            },
        );
    }

    partition
        .boundaries
        .push(
            boundary,
        );

    PartitionResources::increment(
        &mut resources
            .boundaries_created,
    )?;

    Ok(())
}

fn physical_boundary_coordinates(
    partition:
        &QecPartition,
    physical:
        &Bounds,
) -> Vec<Coordinate> {
    let mut coordinates =
        BTreeSet::new();

    for item in
        &partition.items
    {
        if physical.contains(
            item.coordinate,
        ) {
            coordinates.insert(
                item.coordinate,
            );
        }
    }

    for event in
        &partition.events
    {
        if physical.contains(
            event.coordinate,
        ) {
            coordinates.insert(
                event.coordinate,
            );
        }
    }

    coordinates
        .into_iter()
        .collect()
}

fn build_adjacency(
    partitions:
        &[QecPartition],
) -> Result<
    BTreeMap<
        PartitionId,
        BTreeSet<PartitionId>,
    >,
    PartitionError,
> {
    let mut adjacency =
        BTreeMap::new();

    for partition in
        partitions
    {
        adjacency.insert(
            partition.id,
            BTreeSet::new(),
        );
    }

    for partition in
        partitions
    {
        for boundary in
            &partition.boundaries
        {
            for neighbor in
                &boundary
                    .neighboring_partitions
            {
                if *neighbor
                    == partition.id
                {
                    return Err(
                        PartitionError::SelfNeighbor {
                            partition:
                                partition.id,
                        },
                    );
                }

                adjacency
                    .entry(
                        partition.id,
                    )
                    .or_default()
                    .insert(
                        *neighbor,
                    );

                adjacency
                    .entry(
                        *neighbor,
                    )
                    .or_default()
                    .insert(
                        partition.id,
                    );
            }
        }
    }

    Ok(adjacency)
}

fn boundary_event_ids(
    boundaries:
        &[&BoundaryElement],
) -> Vec<EventId> {
    boundaries
        .iter()
        .filter_map(
            |boundary| {
                boundary.event_id
            },
        )
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Creates a lossless deterministic chain representation.
///
/// Unlike the previous implementation, unmatched boundary elements are
/// retained as one-sided links instead of being silently discarded.
fn build_boundary_chain(
    boundaries_a:
        &[&BoundaryElement],
    boundaries_b:
        &[&BoundaryElement],
) -> Result<
    Vec<BoundaryChainLink>,
    PartitionError,
> {
    let mut a =
        boundaries_a
            .iter()
            .map(
                |boundary| {
                    boundary.key.clone()
                },
            )
            .collect::<Vec<_>>();

    let mut b =
        boundaries_b
            .iter()
            .map(
                |boundary| {
                    boundary.key.clone()
                },
            )
            .collect::<Vec<_>>();

    a.sort();
    b.sort();

    let count =
        a.len().max(
            b.len(),
        );

    let mut links =
        Vec::with_capacity(
            count,
        );

    for index in 0..count {
        let link =
            BoundaryChainLink {
                from:
                    a.get(
                        index,
                    )
                    .cloned(),

                to:
                    b.get(
                        index,
                    )
                    .cloned(),

                parity:
                    false,
            };

        link.validate()?;

        links.push(
            link,
        );
    }

    Ok(links)
}

/// ============================================================================
/// Errors
/// ============================================================================

/// Local partitioning diagnostic error.
///
/// Public APIs convert this to `QecError`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum PartitionError {
    InvalidBounds {
        min: Coordinate,
        max: Coordinate,
    },

    CoordinateOutOfRange {
        coordinate: Coordinate,
    },

    InvalidPartitionCount,

    InvalidPhysicalBoundary,

    InvalidBoundary {
        reason: &'static str,
    },

    InvalidBoundaryChain,

    DuplicatePartition {
        partition: PartitionId,
    },

    DuplicateQubit {
        partition: PartitionId,
        qubit: QubitId,
    },

    DuplicateEvent {
        partition: PartitionId,
        event: EventId,
    },

    DuplicateInputQubit {
        qubit: QubitId,
    },

    DuplicateInputEvent {
        event: EventId,
    },

    DuplicateStabilizer {
        qubit: QubitId,
        stabilizer: StabilizerId,
    },

    DuplicateBoundary {
        partition: PartitionId,
        key: BoundaryKey,
    },

    ItemOutsideBounds {
        partition: PartitionId,
        coordinate: Coordinate,
    },

    ItemOutsideInputBounds {
        coordinate: Coordinate,
    },

    UnassignedItem {
        qubit: QubitId,
    },

    UnassignedEvent {
        event: EventId,
    },

    UnknownPartition {
        partition: PartitionId,
    },

    SelfNeighbor {
        partition: PartitionId,
    },

    AsymmetricAdjacency {
        a: PartitionId,
        b: PartitionId,
    },

    BoundaryWithoutAdjacency {
        a: PartitionId,
        b: PartitionId,
    },

    MissingBoundaryData {
        a: PartitionId,
        b: PartitionId,
    },

    ReconciliationMismatch,

    OverlappingPartitions {
        a: PartitionId,
        b: PartitionId,
    },

    NoPartitions,

    UnableToSplit {
        requested: usize,
        achieved: usize,
    },

    UnsupportedSchemaVersion {
        version: u16,
    },

    LimitExceeded {
        resource: LimitKind,
        requested: u128,
        maximum: u128,
    },

    ArithmeticOverflow,

    CancellationRequested,
}

impl PartitionError {
    fn from_qec_error(
        error: QecError,
    ) -> Self {
        match error {
            QecError::CancellationRequested {
                ..
            } => {
                Self::CancellationRequested
            }

            QecError::ResourceLimitExceeded {
                resource,
                requested,
                limit,
                ..
            } => {
                Self::LimitExceeded {
                    resource:
                        qec_resource_to_limit(
                            resource,
                        ),
                    requested,
                    maximum:
                        limit,
                }
            }

            QecError::TimeLimitExceeded {
                ..
            } => {
                /*
                 * Partitioning deliberately does not own a time policy.
                 * Preserve the public error as a cancellation-like local
                 * failure rather than inventing a partition timeout enum.
                 */
                Self::CancellationRequested
            }

            QecError::NumericalFailure {
                ..
            } => {
                Self::ArithmeticOverflow
            }

            QecError::InvalidInput {
                ..
            }
            | QecError::InvalidTopology {
                ..
            }
            | QecError::InvalidSyndrome {
                ..
            }
            | QecError::InvalidGraph {
                ..
            } => {
                Self::InvalidBoundary {
                    reason:
                        "invalid QEC input or topology",
                }
            }

            _ => {
                Self::InvalidBoundary {
                    reason:
                        "QEC operation failed during partitioning",
                }
            }
        }
    }

    fn into_qec_error(
        self,
    ) -> QecError {
        match self {
            Self::LimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                QecError::resource_limit(
                    resource_to_qec_kind(
                        resource,
                    ),
                    requested,
                    0,
                    maximum,
                    "partition resource policy exceeded",
                )
            }

            Self::CancellationRequested => {
                QecError::cancelled(
                    "partition operation was cancelled",
                )
            }

            Self::ArithmeticOverflow => {
                QecError::numerical_failure(
                    NumericalOperation::CoordinateCalculation,
                    "partition arithmetic overflow",
                )
            }

            Self::InvalidBounds { .. }
            | Self::CoordinateOutOfRange { .. }
            | Self::InvalidPartitionCount
            | Self::InvalidPhysicalBoundary
            | Self::InvalidBoundary { .. }
            | Self::InvalidBoundaryChain
            | Self::DuplicatePartition { .. }
            | Self::DuplicateQubit { .. }
            | Self::DuplicateEvent { .. }
            | Self::DuplicateInputQubit { .. }
            | Self::DuplicateInputEvent { .. }
            | Self::DuplicateStabilizer { .. }
            | Self::DuplicateBoundary { .. }
            | Self::ItemOutsideBounds { .. }
            | Self::ItemOutsideInputBounds { .. }
            | Self::UnassignedItem { .. }
            | Self::UnassignedEvent { .. }
            | Self::UnknownPartition { .. }
            | Self::SelfNeighbor { .. }
            | Self::AsymmetricAdjacency { .. }
            | Self::BoundaryWithoutAdjacency { .. }
            | Self::MissingBoundaryData { .. }
            | Self::ReconciliationMismatch
            | Self::OverlappingPartitions { .. }
            | Self::NoPartitions
            | Self::UnableToSplit { .. }
            | Self::UnsupportedSchemaVersion { .. } => {
                QecError::invalid_topology(
                    self.to_string(),
                )
            }
        }
    }
}

impl fmt::Display
    for PartitionError
{
    fn fmt(
        &self,
        formatter:
            &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidBounds {
                min,
                max,
            } => {
                write!(
                    formatter,
                    "invalid partition bounds: min={min:?}, max={max:?}"
                )
            }

            Self::CoordinateOutOfRange {
                coordinate,
            } => {
                write!(
                    formatter,
                    "partition coordinate out of range: {coordinate:?}"
                )
            }

            Self::InvalidPartitionCount => {
                formatter.write_str(
                    "partition count must be greater than zero",
                )
            }

            Self::InvalidPhysicalBoundary => {
                formatter.write_str(
                    "physical boundary lies outside input bounds",
                )
            }

            Self::InvalidBoundary {
                reason,
            } => {
                write!(
                    formatter,
                    "invalid partition boundary: {reason}"
                )
            }

            Self::InvalidBoundaryChain => {
                formatter.write_str(
                    "boundary chain contains no endpoint",
                )
            }

            Self::DuplicatePartition {
                partition,
            } => {
                write!(
                    formatter,
                    "duplicate partition: {partition}"
                )
            }

            Self::DuplicateQubit {
                partition,
                qubit,
            } => {
                write!(
                    formatter,
                    "duplicate qubit {qubit} in partition {partition}"
                )
            }

            Self::DuplicateEvent {
                partition,
                event,
            } => {
                write!(
                    formatter,
                    "duplicate event {event} in partition {partition}"
                )
            }

            Self::DuplicateInputQubit {
                qubit,
            } => {
                write!(
                    formatter,
                    "duplicate input qubit: {qubit}"
                )
            }

            Self::DuplicateInputEvent {
                event,
            } => {
                write!(
                    formatter,
                    "duplicate input event: {event}"
                )
            }

            Self::DuplicateStabilizer {
                qubit,
                stabilizer,
            } => {
                write!(
                    formatter,
                    "duplicate stabilizer {stabilizer} on qubit {qubit}"
                )
            }

            Self::DuplicateBoundary {
                partition,
                ..
            } => {
                write!(
                    formatter,
                    "duplicate boundary in partition {partition}"
                )
            }

            Self::ItemOutsideBounds {
                partition,
                coordinate,
            } => {
                write!(
                    formatter,
                    "coordinate {coordinate:?} is outside partition {partition}"
                )
            }

            Self::ItemOutsideInputBounds {
                coordinate,
            } => {
                write!(
                    formatter,
                    "coordinate {coordinate:?} is outside input bounds"
                )
            }

            Self::UnassignedItem {
                qubit,
            } => {
                write!(
                    formatter,
                    "qubit {qubit} could not be assigned to a partition"
                )
            }

            Self::UnassignedEvent {
                event,
            } => {
                write!(
                    formatter,
                    "event {event} could not be assigned to a partition"
                )
            }

            Self::UnknownPartition {
                partition,
            } => {
                write!(
                    formatter,
                    "unknown partition: {partition}"
                )
            }

            Self::SelfNeighbor {
                partition,
            } => {
                write!(
                    formatter,
                    "partition {partition} cannot neighbor itself"
                )
            }

            Self::AsymmetricAdjacency {
                a,
                b,
            } => {
                write!(
                    formatter,
                    "asymmetric partition adjacency between {a} and {b}"
                )
            }

            Self::BoundaryWithoutAdjacency {
                a,
                b,
            } => {
                write!(
                    formatter,
                    "boundary contract {a}<->{b} has no adjacency"
                )
            }

            Self::MissingBoundaryData {
                a,
                b,
            } => {
                write!(
                    formatter,
                    "missing boundary data for partitions {a}<->{b}"
                )
            }

            Self::ReconciliationMismatch => {
                formatter.write_str(
                    "partition reconciliation metadata mismatch",
                )
            }

            Self::OverlappingPartitions {
                a,
                b,
            } => {
                write!(
                    formatter,
                    "partitions {a} and {b} overlap"
                )
            }

            Self::NoPartitions => {
                formatter.write_str(
                    "partition plan contains no partitions",
                )
            }

            Self::UnableToSplit {
                requested,
                achieved,
            } => {
                write!(
                    formatter,
                    "unable to create {requested} partitions; achieved {achieved}"
                )
            }

            Self::UnsupportedSchemaVersion {
                version,
            } => {
                write!(
                    formatter,
                    "unsupported partition schema version {version}"
                )
            }

            Self::LimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "partition limit {resource} exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "partition arithmetic overflow",
                )
            }

            Self::CancellationRequested => {
                formatter.write_str(
                    "partition operation cancelled",
                )
            }
        }
    }
}

impl std::error::Error
    for PartitionError
{}

/// ============================================================================
/// Error mapping
/// ============================================================================

fn map_limit_error(
    error: LimitError,
) -> QecError {
    match error {
        LimitError::InvalidLimit {
            resource,
            value,
        } => {
            QecError::invalid_input(
                format!(
                    "invalid QEC limit {resource}: {value}"
                ),
            )
        }

        LimitError::Exceeded {
            resource,
            requested,
            maximum,
        } => {
            QecError::resource_limit(
                resource_to_qec_kind(
                    resource,
                ),
                requested,
                0,
                maximum,
                "QEC limit exceeded",
            )
        }

        LimitError::ArithmeticOverflow {
            resource,
        } => {
            QecError::numerical_failure(
                NumericalOperation::MemorySizeCalculation,
                format!(
                    "overflow while validating QEC limit {resource}"
                ),
            )
        }

        LimitError::InconsistentLimits {
            resource,
            related_resource,
            reason,
        } => {
            QecError::invalid_input(
                format!(
                    "inconsistent QEC limits {resource}/{related_resource}: {reason}"
                ),
            )
        }

        LimitError::UnsupportedSchema {
            found,
            expected,
        } => {
            QecError::version_mismatch(
                "QecLimits",
                expected.to_string(),
                found.to_string(),
                "unsupported QEC limits schema",
            )
        }
    }
}

fn qec_resource_to_limit(
    resource:
        ResourceKind,
) -> LimitKind {
    match resource {
        ResourceKind::CodeDistance => {
            LimitKind::CodeDistance
        }

        ResourceKind::Qubits => {
            LimitKind::Qubits
        }

        ResourceKind::Stabilizers => {
            LimitKind::Stabilizers
        }

        ResourceKind::StabilizerWeight => {
            LimitKind::StabilizerWeight
        }

        ResourceKind::SyndromeEvents => {
            LimitKind::SyndromeEvents
        }

        ResourceKind::MeasurementRounds => {
            LimitKind::MeasurementRounds
        }

        ResourceKind::GraphNodes => {
            LimitKind::GraphNodes
        }

        ResourceKind::GraphEdges => {
            LimitKind::GraphEdges
        }

        ResourceKind::DecoderIterations => {
            LimitKind::DecoderIterations
        }

        ResourceKind::Parallelism
        | ResourceKind::Workers => {
            LimitKind::Parallelism
        }

        ResourceKind::MemoryBytes => {
            LimitKind::MemoryBytes
        }

        ResourceKind::CheckpointSize => {
            LimitKind::CheckpointSizeBytes
        }

        ResourceKind::Partitions => {
            LimitKind::Partitions
        }

        ResourceKind::StreamBuffer => {
            LimitKind::StreamBufferEvents
        }

        ResourceKind::QpuShots => {
            LimitKind::QpuShots
        }

        ResourceKind::QpuCircuits => {
            LimitKind::QpuCircuits
        }

        ResourceKind::LogicalWeight => {
            LimitKind::LogicalOperatorWeight
        }

        ResourceKind::Operations
        | ResourceKind::DecoderOperations
        | ResourceKind::Allocations
        | ResourceKind::Checkpoints
        | ResourceKind::Time
        | ResourceKind::Custom => {
            LimitKind::Partitions
        }
    }
}

fn resource_to_qec_kind(
    resource:
        LimitKind,
) -> ResourceKind {
    match resource {
        LimitKind::CodeDistance => {
            ResourceKind::CodeDistance
        }

        LimitKind::Qubits
        | LimitKind::QubitsPerPartition => {
            ResourceKind::Qubits
        }

        LimitKind::Stabilizers => {
            ResourceKind::Stabilizers
        }

        LimitKind::StabilizerWeight => {
            ResourceKind::StabilizerWeight
        }

        LimitKind::SyndromeEvents => {
            ResourceKind::SyndromeEvents
        }

        LimitKind::MeasurementRounds => {
            ResourceKind::MeasurementRounds
        }

        LimitKind::GraphNodes => {
            ResourceKind::GraphNodes
        }

        LimitKind::GraphEdges => {
            ResourceKind::GraphEdges
        }

        LimitKind::MemoryBytes => {
            ResourceKind::MemoryBytes
        }

        LimitKind::DecoderTimeNs => {
            ResourceKind::Time
        }

        LimitKind::Parallelism => {
            ResourceKind::Parallelism
        }

        LimitKind::CheckpointSizeBytes => {
            ResourceKind::CheckpointSize
        }

        LimitKind::Partitions => {
            ResourceKind::Partitions
        }

        LimitKind::StreamBufferEvents => {
            ResourceKind::StreamBuffer
        }

        LimitKind::DecoderIterations => {
            ResourceKind::DecoderIterations
        }

        LimitKind::LogicalOperatorWeight => {
            ResourceKind::LogicalWeight
        }

        LimitKind::QpuShots => {
            ResourceKind::QpuShots
        }

        LimitKind::QpuCircuits => {
            ResourceKind::QpuCircuits
        }

        LimitKind::VerificationOperations => {
            ResourceKind::Operations
        }
    }
}

/// ============================================================================
/// Tests
/// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> QecLimits {
        let mut limits =
            QecLimits::default();

        limits.max_qubits =
            1_000;

        limits.max_stabilizers =
            1_000;

        limits.max_syndrome_events =
            1_000;

        limits.max_partitions =
            64;

        limits.max_qubits_per_partition =
            1_000;

        limits.max_graph_nodes =
            10_000;

        limits
    }

    fn bounds() -> Bounds {
        Bounds::new(
            Coordinate::new(
                0,
                0,
                0,
            ),
            Coordinate::new(
                9,
                9,
                0,
            ),
        )
        .unwrap()
    }

    fn item(
        id: u64,
        x: i64,
        y: i64,
    ) -> PartitionItem {
        PartitionItem {
            qubit_id: id,
            coordinate:
                Coordinate::new(
                    x,
                    y,
                    0,
                ),
            stabilizers:
                vec![id],
        }
    }

    #[test]
    fn checked_geometry_is_correct() {
        let bounds =
            bounds();

        assert_eq!(
            bounds
                .axis_length(
                    PartitionAxis::X,
                )
                .unwrap(),
            10
        );

        assert_eq!(
            bounds
                .checked_volume()
                .unwrap(),
            100
        );
    }

    #[test]
    fn adjacent_boxes_share_a_face() {
        let left =
            Bounds::new(
                Coordinate::new(
                    0,
                    0,
                    0,
                ),
                Coordinate::new(
                    4,
                    4,
                    0,
                ),
            )
            .unwrap();

        let right =
            Bounds::new(
                Coordinate::new(
                    5,
                    0,
                    0,
                ),
                Coordinate::new(
                    9,
                    4,
                    0,
                ),
            )
            .unwrap();

        assert!(
            left.shares_face(
                &right,
            )
        );
    }

    #[test]
    fn non_adjacent_boxes_do_not_share_a_face() {
        let left =
            Bounds::new(
                Coordinate::new(
                    0,
                    0,
                    0,
                ),
                Coordinate::new(
                    4,
                    4,
                    0,
                ),
            )
            .unwrap();

        let right =
            Bounds::new(
                Coordinate::new(
                    6,
                    0,
                    0,
                ),
                Coordinate::new(
                    9,
                    4,
                    0,
                ),
            )
            .unwrap();

        assert!(
            !left.shares_face(
                &right,
            )
        );
    }

    #[test]
    fn fixed_count_is_exact_when_geometry_allows_it() {
        let partitioner =
            Partitioner::new(
                PartitionStrategy::FixedCount {
                    partitions: 8,
                },
                limits(),
            )
            .unwrap();

        let input =
            PartitionInput {
                bounds:
                    bounds(),
                items:
                    Vec::new(),
                events:
                    Vec::new(),
                physical_boundary:
                    None,
            };

        let plan =
            partitioner
                .partition(
                    input,
                )
                .unwrap();

        assert_eq!(
            plan.partitions.len(),
            8
        );
    }

    #[test]
    fn partition_adjacency_is_created() {
        let partitioner =
            Partitioner::new(
                PartitionStrategy::FixedCount {
                    partitions: 2,
                },
                limits(),
            )
            .unwrap();

        let input =
            PartitionInput {
                bounds:
                    bounds(),
                items:
                    vec![
                        item(
                            1,
                            2,
                            2,
                        ),
                        item(
                            2,
                            7,
                            7,
                        ),
                    ],
                events:
                    Vec::new(),
                physical_boundary:
                    None,
            };

        let plan =
            partitioner
                .partition(
                    input,
                )
                .unwrap();

        assert_eq!(
            plan.partitions.len(),
            2
        );

        assert!(
            plan.adjacency
                .get(&0)
                .is_some_and(
                    |neighbors| {
                        neighbors
                            .contains(&1)
                    },
                )
        );

        assert!(
            plan.adjacency
                .get(&1)
                .is_some_and(
                    |neighbors| {
                        neighbors
                            .contains(&0)
                    },
                )
        );

        assert_eq!(
            plan.boundary_contracts.len(),
            1
        );
    }

    #[test]
    fn duplicate_input_qubits_are_rejected() {
        let partitioner =
            Partitioner::new(
                PartitionStrategy::FixedCount {
                    partitions: 2,
                },
                limits(),
            )
            .unwrap();

        let input =
            PartitionInput {
                bounds:
                    bounds(),
                items:
                    vec![
                        item(
                            1,
                            1,
                            1,
                        ),
                        item(
                            1,
                            2,
                            2,
                        ),
                    ],
                events:
                    Vec::new(),
                physical_boundary:
                    None,
            };

        assert!(
            partitioner
                .partition(
                    input,
                )
                .is_err()
        );
    }

    #[test]
    fn events_are_never_silently_dropped() {
        let partitioner =
            Partitioner::new(
                PartitionStrategy::FixedCount {
                    partitions: 2,
                },
                limits(),
            )
            .unwrap();

        let input =
            PartitionInput {
                bounds:
                    bounds(),
                items:
                    Vec::new(),
                events:
                    vec![
                        PartitionEvent {
                            event_id: 1,
                            coordinate:
                                Coordinate::new(
                                    1,
                                    1,
                                    0,
                                ),
                            round: 0,
                            graph_node:
                                Some(1),
                        },
                        PartitionEvent {
                            event_id: 2,
                            coordinate:
                                Coordinate::new(
                                    8,
                                    8,
                                    0,
                                ),
                            round: 0,
                            graph_node:
                                Some(2),
                        },
                    ],
                physical_boundary:
                    None,
            };

        let plan =
            partitioner
                .partition(
                    input,
                )
                .unwrap();

        let event_count =
            plan.partitions
                .iter()
                .map(
                    |partition| {
                        partition
                            .events
                            .len()
                    },
                )
                .sum::<usize>();

        assert_eq!(
            event_count,
            2
        );
    }

    #[test]
    fn unmatched_boundary_elements_are_preserved() {
        let a =
            BoundaryElement {
                key:
                    BoundaryKey {
                        coordinate:
                            Coordinate::new(
                                1,
                                1,
                                0,
                            ),
                        round: None,
                        graph_node:
                            None,
                    },
                qubit_id:
                    Some(1),
                event_id:
                    Some(1),
                kind:
                    BoundaryKind::InterPartition,
                neighboring_partitions:
                    BTreeSet::from(
                        [1],
                    ),
                virtual_boundary:
                    false,
            };

        let b =
            BoundaryElement {
                key:
                    BoundaryKey {
                        coordinate:
                            Coordinate::new(
                                1,
                                2,
                                0,
                            ),
                        round: None,
                        graph_node:
                            None,
                    },
                qubit_id:
                    Some(2),
                event_id:
                    Some(2),
                kind:
                    BoundaryKind::InterPartition,
                neighboring_partitions:
                    BTreeSet::from(
                        [0],
                    ),
                virtual_boundary:
                    false,
            };

        let c =
            BoundaryElement {
                key:
                    BoundaryKey {
                        coordinate:
                            Coordinate::new(
                                1,
                                3,
                                0,
                            ),
                        round: None,
                        graph_node:
                            None,
                    },
                qubit_id:
                    Some(3),
                event_id:
                    Some(3),
                kind:
                    BoundaryKind::InterPartition,
                neighboring_partitions:
                    BTreeSet::from(
                        [1],
                    ),
                virtual_boundary:
                    false,
            };

        let links =
            build_boundary_chain(
                &[&a, &c],
                &[&b],
            )
            .unwrap();

        assert_eq!(
            links.len(),
            2
        );

        assert!(
            links[1]
                .from
                .is_some()
                || links[1]
                    .to
                    .is_some()
        );
    }

    #[test]
    fn cancellation_is_honoured() {
        let partitioner =
            Partitioner::new(
                PartitionStrategy::FixedCount {
                    partitions: 2,
                },
                limits(),
            )
            .unwrap();

        let source =
            super::super::cancellation::CancellationSource::new();

        source.cancel();

        let input =
            PartitionInput {
                bounds:
                    bounds(),
                items:
                    Vec::new(),
                events:
                    Vec::new(),
                physical_boundary:
                    None,
            };

        assert!(
            partitioner
                .partition_with_cancellation(
                    input,
                    &source.token(),
                )
                .is_err()
        );
    }

    #[test]
    fn deterministic_partition_order_is_stable() {
        let partitioner =
            Partitioner::new(
                PartitionStrategy::FixedCount {
                    partitions: 8,
                },
                limits(),
            )
            .unwrap();

        let input =
            PartitionInput {
                bounds:
                    bounds(),
                items:
                    vec![
                        item(
                            1,
                            1,
                            1,
                        ),
                        item(
                            2,
                            8,
                            8,
                        ),
                    ],
                events:
                    Vec::new(),
                physical_boundary:
                    None,
            };

        let first =
            partitioner
                .partition(
                    input.clone(),
                )
                .unwrap();

        let second =
            partitioner
                .partition(
                    input,
                )
                .unwrap();

        let first_bounds =
            first
                .partitions
                .iter()
                .map(
                    |partition| {
                        (
                            partition
                                .id,
                            partition
                                .bounds,
                        )
                    },
                )
                .collect::<Vec<_>>();

        let second_bounds =
            second
                .partitions
                .iter()
                .map(
                    |partition| {
                        (
                            partition
                                .id,
                            partition
                                .bounds,
                        )
                    },
                )
                .collect::<Vec<_>>();

        assert_eq!(
            first_bounds,
            second_bounds
        );
    }
}