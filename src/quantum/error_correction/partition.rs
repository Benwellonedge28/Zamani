//! Zamani Quantum Error Correction — bounded partitioning.
//!
//! Partitioning is an infrastructure boundary, not a decoder.
//!
//! Architectural contract:
//!
//! ```text
//! UNTRUSTED INPUT
//!      │
//!      ▼
//! PartitionInput validation
//!      │
//!      ▼
//! QecLimits preflight
//!      │
//!      ▼
//! CancellationToken
//!      │
//!      ▼
//! Deterministic partition construction
//!      │
//!      ├───────────────┐
//!      ▼               ▼
//! Local data       Boundary contract
//!      │               │
//!      └───────┬───────┘
//!              ▼
//!      PartitionPlan
//!              │
//!              ▼
//!      Boundary reconciliation
//!              │
//!              ▼
//!       Global decoder
//! ```
//!
//! Important:
//!
//! * `QecLimits` is the canonical resource policy.
//! * `resources.rs` owns runtime accounting; this module does not invent a
//!   second production resource policy.
//! * `cancellation.rs` owns cancellation.
//! * `errors.rs` owns the public QEC error boundary.
//! * Partition-local decoding is never treated as globally correct until
//!   boundary reconciliation has completed.
//! * Partition IDs, ordering, adjacency and reconciliation units are
//!   deterministic.
//! * No unchecked coordinate/resource arithmetic is used.
//! * No partition is allocated beyond the configured policy.
//!
//! A partition is a computational decomposition, not a mathematical
//! decomposition of the code. The boundary contract therefore preserves the
//! information required to reconstruct the global decoding problem.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::time::{Duration, Instant};

use super::cancellation::CancellationToken;
use super::errors::{QecError, QecResult, ResourceKind};
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

/// Stable reconciliation identifier.
pub type ReconciliationId = u64;

/// Maximum representable coordinate magnitude accepted by partition
/// preflight when no stricter topology-specific limit is available.
///
/// This is deliberately not a resource policy. It is an arithmetic-safety
/// guard against pathological coordinate domains.
pub const DEFAULT_MAX_COORDINATE_ABS: i64 = 1_000_000_000_000;

/// Current in-memory partition schema.
pub const PARTITION_SCHEMA_VERSION: u16 = 3;

// ============================================================================
// Coordinates
// ============================================================================

/// Logical coordinate in a QEC lattice.
///
/// Signed coordinates are used because partition boundaries may be expressed
/// relative to a global origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Coordinate {
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn validate(self) -> Result<(), PartitionError> {
        if self.x.unsigned_abs() > DEFAULT_MAX_COORDINATE_ABS as u64
            || self.y.unsigned_abs() > DEFAULT_MAX_COORDINATE_ABS as u64
            || self.z.unsigned_abs() > DEFAULT_MAX_COORDINATE_ABS as u64
        {
            return Err(PartitionError::CoordinateOutOfRange {
                coordinate: self,
            });
        }

        Ok(())
    }

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

// ============================================================================
// Partition geometry
// ============================================================================

/// Axis along which a partition can be split.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PartitionAxis {
    X,
    Y,
    Z,
}

impl PartitionAxis {
    fn coordinate(self, coordinate: Coordinate) -> i64 {
        match self {
            Self::X => coordinate.x,
            Self::Y => coordinate.y,
            Self::Z => coordinate.z,
        }
    }
}

/// Inclusive rectangular bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

        if min.x > max.x || min.y > max.y || min.z > max.z {
            return Err(PartitionError::InvalidBounds { min, max });
        }

        Ok(Self { min, max })
    }

    pub fn contains(&self, coordinate: Coordinate) -> bool {
        coordinate.x >= self.min.x
            && coordinate.x <= self.max.x
            && coordinate.y >= self.min.y
            && coordinate.y <= self.max.y
            && coordinate.z >= self.min.z
            && coordinate.z <= self.max.z
    }

    /// Returns the inclusive number of lattice coordinates along an axis.
    pub fn axis_length(
        &self,
        axis: PartitionAxis,
    ) -> Result<u64, PartitionError> {
        axis_length(
            axis.coordinate(self.min),
            axis.coordinate(self.max),
        )
    }

    /// Returns the volume using checked arithmetic.
    pub fn checked_volume(&self) -> Result<u64, PartitionError> {
        let x = self.axis_length(PartitionAxis::X)?;
        let y = self.axis_length(PartitionAxis::Y)?;
        let z = self.axis_length(PartitionAxis::Z)?;

        x.checked_mul(y)
            .and_then(|value| value.checked_mul(z))
            .ok_or(PartitionError::ArithmeticOverflow)
    }

    /// Two boxes share a lattice face/edge/corner neighborhood.
    ///
    /// Partition construction only uses face adjacency for reconciliation.
    pub fn touches_face(&self, other: &Self) -> bool {
        let x_overlap =
            self.min.x <= other.max.x && self.max.x >= other.min.x;
        let y_overlap =
            self.min.y <= other.max.y && self.max.y >= other.min.y;
        let z_overlap =
            self.min.z <= other.max.z && self.max.z >= other.min.z;

        if !(x_overlap && y_overlap && z_overlap) {
            return false;
        }

        let x_face = self.max.x.checked_add(1) == Some(other.min.x)
            || other.max.x.checked_add(1) == Some(self.min.x);

        let y_face = self.max.y.checked_add(1) == Some(other.min.y)
            || other.max.y.checked_add(1) == Some(self.min.y);

        let z_face = self.max.z.checked_add(1) == Some(other.min.z)
            || other.max.z.checked_add(1) == Some(self.min.z);

        (x_face && y_overlap && z_overlap)
            || (y_face && x_overlap && z_overlap)
            || (z_face && x_overlap && y_overlap)
    }

    /// Returns whether two boxes overlap.
    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }
}

// ============================================================================
// Strategy
// ============================================================================

/// Partitioning strategy.
///
/// All strategies are deterministic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PartitionStrategy {
    /// Split the input once along its longest axis.
    LongestAxis,

    /// Split once along a specified axis.
    FixedAxis(PartitionAxis),

    /// Recursively split until exactly `partitions` regions exist.
    FixedCount { partitions: usize },
}

// ============================================================================
// Boundary model
// ============================================================================

/// Classification of a partition boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoundaryKind {
    /// No external relationship.
    Internal,

    /// Boundary shared with another computational partition.
    InterPartition,

    /// Boundary associated with the physical code boundary.
    Physical,

    /// Shared with another partition and the physical code boundary.
    Mixed,
}

impl BoundaryKind {
    fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::Mixed, _) | (_, Self::Mixed) => Self::Mixed,

            (Self::InterPartition, Self::Physical)
            | (Self::Physical, Self::InterPartition) => Self::Mixed,

            (Self::Internal, value) | (value, Self::Internal) => value,

            (Self::InterPartition, Self::InterPartition) => {
                Self::InterPartition
            }

            (Self::Physical, Self::Physical) => Self::Physical,
        }
    }
}

/// Explicit boundary identity.
///
/// A boundary does not necessarily correspond to one particular qubit. It
/// may represent a virtual decoding boundary or a coordinate at which
/// syndrome/correction information must be reconciled.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoundaryKey {
    pub coordinate: Coordinate,
    pub round: Option<u64>,
    pub graph_node: Option<GraphNodeId>,
}

impl BoundaryKey {
    pub fn validate(&self) -> Result<(), PartitionError> {
        self.coordinate.validate()?;
        Ok(())
    }
}

/// Boundary element carried by a partition.
///
/// This is deliberately richer than the previous coordinate-only boundary
/// representation because partition reconciliation needs to preserve both
/// syndrome and correction context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryElement {
    pub key: BoundaryKey,

    pub qubit_id: Option<QubitId>,
    pub event_id: Option<EventId>,

    pub kind: BoundaryKind,

    /// Neighboring computational partitions.
    pub neighboring_partitions: BTreeSet<PartitionId>,

    /// Whether this boundary is virtual rather than a physical object.
    pub virtual_boundary: bool,
}

impl BoundaryElement {
    pub fn validate(&self) -> Result<(), PartitionError> {
        self.key.validate()?;

        if self.kind == BoundaryKind::InterPartition
            && self.neighboring_partitions.is_empty()
        {
            return Err(PartitionError::InvalidBoundary {
                reason:
                    "inter-partition boundary has no neighboring partition",
            });
        }

        if self.kind == BoundaryKind::Physical
            && !self.neighboring_partitions.is_empty()
        {
            return Err(PartitionError::InvalidBoundary {
                reason:
                    "physical-only boundary unexpectedly has partition neighbors",
            });
        }

        Ok(())
    }
}

// ============================================================================
// Partition data
// ============================================================================

/// QEC item assigned to a partition.
#[derive(Debug, Clone, PartialEq, Eq)]
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

        if self.stabilizers.len() > limits.max_stabilizer_weight {
            return Err(PartitionError::LimitExceeded {
                resource: LimitKind::StabilizerWeight,
                requested: self.stabilizers.len() as u128,
                maximum: limits.max_stabilizer_weight as u128,
            });
        }

        let mut unique = BTreeSet::new();

        for stabilizer in &self.stabilizers {
            if !unique.insert(*stabilizer) {
                return Err(PartitionError::DuplicateStabilizer {
                    qubit: self.qubit_id,
                    stabilizer: *stabilizer,
                });
            }
        }

        Ok(())
    }
}

/// Syndrome/detection event assigned to a partition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionEvent {
    pub event_id: EventId,
    pub coordinate: Coordinate,
    pub round: u64,
    pub graph_node: Option<GraphNodeId>,
}

impl PartitionEvent {
    pub fn validate(&self) -> Result<(), PartitionError> {
        self.coordinate.validate()?;
        Ok(())
    }
}

/// A single QEC partition.
#[derive(Debug, Clone)]
pub struct QecPartition {
    pub id: PartitionId,
    pub bounds: Bounds,

    pub items: Vec<PartitionItem>,
    pub events: Vec<PartitionEvent>,

    pub boundaries: Vec<BoundaryElement>,
}

impl QecPartition {
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
        let mut qubits = BTreeSet::new();
        let mut events = BTreeSet::new();
        let mut boundary_keys = BTreeSet::new();

        if self.items.len() > limits.max_qubits_per_partition {
            return Err(PartitionError::LimitExceeded {
                resource: LimitKind::QubitsPerPartition,
                requested: self.items.len() as u128,
                maximum: limits.max_qubits_per_partition as u128,
            });
        }

        if self.events.len() > limits.max_syndrome_events {
            return Err(PartitionError::LimitExceeded {
                resource: LimitKind::SyndromeEvents,
                requested: self.events.len() as u128,
                maximum: limits.max_syndrome_events as u128,
            });
        }

        for item in &self.items {
            item.validate(limits)?;

            if !qubits.insert(item.qubit_id) {
                return Err(PartitionError::DuplicateQubit {
                    partition: self.id,
                    qubit: item.qubit_id,
                });
            }

            if !self.bounds.contains(item.coordinate) {
                return Err(PartitionError::ItemOutsideBounds {
                    partition: self.id,
                    coordinate: item.coordinate,
                });
            }
        }

        for event in &self.events {
            event.validate()?;

            if !events.insert(event.event_id) {
                return Err(PartitionError::DuplicateEvent {
                    partition: self.id,
                    event: event.event_id,
                });
            }

            if !self.bounds.contains(event.coordinate) {
                return Err(PartitionError::ItemOutsideBounds {
                    partition: self.id,
                    coordinate: event.coordinate,
                });
            }
        }

        for boundary in &self.boundaries {
            boundary.validate()?;

            if !boundary_keys.insert(boundary.key.clone()) {
                return Err(PartitionError::DuplicateBoundary {
                    partition: self.id,
                    key: boundary.key.clone(),
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// Input
// ============================================================================

/// Input to the partitioner.
#[derive(Debug, Clone)]
pub struct PartitionInput {
    pub bounds: Bounds,
    pub items: Vec<PartitionItem>,
    pub events: Vec<PartitionEvent>,

    /// Physical code boundary, when known.
    ///
    /// It is intentionally separate from partition boundaries.
    pub physical_boundary: Option<Bounds>,
}

impl PartitionInput {
    pub fn validate(
        &self,
        limits: &QecLimits,
    ) -> Result<(), PartitionError> {
        self.bounds
            .min
            .validate()?;
        self.bounds
            .max
            .validate()?;

        let volume = self.bounds.checked_volume()?;

        /*
         * This is a preflight sanity check only. We do not allocate based on
         * the geometric volume. QECLimits controls actual workload resources.
         */
        if volume > limits.max_qubits as u64 {
            return Err(PartitionError::LimitExceeded {
                resource: LimitKind::Qubits,
                requested: volume as u128,
                maximum: limits.max_qubits as u128,
            });
        }

        if self.items.len() > limits.max_qubits {
            return Err(PartitionError::LimitExceeded {
                resource: LimitKind::Qubits,
                requested: self.items.len() as u128,
                maximum: limits.max_qubits as u128,
            });
        }

        if self.events.len() > limits.max_syndrome_events {
            return Err(PartitionError::LimitExceeded {
                resource: LimitKind::SyndromeEvents,
                requested: self.events.len() as u128,
                maximum: limits.max_syndrome_events as u128,
            });
        }

        let mut qubits = BTreeSet::new();
        let mut events = BTreeSet::new();

        for item in &self.items {
            item.validate(limits)?;

            if !qubits.insert(item.qubit_id) {
                return Err(PartitionError::DuplicateInputQubit {
                    qubit: item.qubit_id,
                });
            }

            if !self.bounds.contains(item.coordinate) {
                return Err(PartitionError::ItemOutsideInputBounds {
                    coordinate: item.coordinate,
                });
            }
        }

        for event in &self.events {
            event.validate()?;

            if !events.insert(event.event_id) {
                return Err(PartitionError::DuplicateInputEvent {
                    event: event.event_id,
                });
            }

            if !self.bounds.contains(event.coordinate) {
                return Err(PartitionError::ItemOutsideInputBounds {
                    coordinate: event.coordinate,
                });
            }
        }

        if let Some(boundary) = self.physical_boundary {
            boundary
                .min
                .validate()?;
            boundary
                .max
                .validate()?;

            if !self.bounds.contains(boundary.min)
                || !self.bounds.contains(boundary.max)
            {
                return Err(
                    PartitionError::InvalidPhysicalBoundary,
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// Runtime accounting
// ============================================================================

/// Deterministic partitioning resource accounting.
///
/// This is an operation-local snapshot. Persistent runtime accounting belongs
/// to `resources.rs`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PartitionResources {
    pub items_processed: u64,
    pub events_processed: u64,
    pub partitions_created: u64,
    pub boundaries_created: u64,
    pub neighbor_links_created: u64,

    pub peak_items_per_partition: u64,
    pub peak_events_per_partition: u64,

    pub elapsed: Duration,
}

impl PartitionResources {
    fn checked_increment(
        value: &mut u64,
        amount: u64,
    ) -> Result<(), PartitionError> {
        *value = value
            .checked_add(amount)
            .ok_or(PartitionError::ArithmeticOverflow)?;

        Ok(())
    }
}

// ============================================================================
// Boundary contract
// ============================================================================

/// Mathematical contract carried across a partition boundary.
///
/// This is the central structure needed to make partitioned decoding
/// mathematically meaningful rather than merely spatially convenient.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionBoundary {
    pub reconciliation_id: ReconciliationId,

    pub partition_a: PartitionId,
    pub partition_b: PartitionId,

    /// State entering the boundary from A.
    pub incoming_syndrome_state_a: Vec<EventId>,

    /// State entering the boundary from B.
    pub incoming_syndrome_state_b: Vec<EventId>,

    /// State leaving the boundary toward A.
    pub outgoing_syndrome_state_a: Vec<EventId>,

    /// State leaving the boundary toward B.
    pub outgoing_syndrome_state_b: Vec<EventId>,

    /// Virtual boundary nodes used by a decoder.
    pub virtual_boundary_nodes: Vec<GraphNodeId>,

    /// Correction-chain endpoints crossing the boundary.
    pub correction_chain: Vec<BoundaryChainLink>,

    /// Logical parity contributed by the boundary.
    pub logical_parity: LogicalParity,

    /// Explicit metadata required to reproduce reconciliation.
    pub reconciliation_metadata: ReconciliationMetadata,
}

impl PartitionBoundary {
    pub fn validate(&self) -> Result<(), PartitionError> {
        if self.partition_a == self.partition_b {
            return Err(PartitionError::SelfNeighbor {
                partition: self.partition_a,
            });
        }

        self.reconciliation_metadata.validate()?;

        for link in &self.correction_chain {
            link.validate()?;
        }

        Ok(())
    }
}

/// A correction-chain segment crossing a partition boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryChainLink {
    pub from: BoundaryKey,
    pub to: BoundaryKey,
    pub parity: bool,
}

impl BoundaryChainLink {
    pub fn validate(&self) -> Result<(), PartitionError> {
        self.from.validate()?;
        self.to.validate()?;
        Ok(())
    }
}

/// Logical parity accumulated during reconciliation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LogicalParity {
    pub x: bool,
    pub z: bool,
}

impl LogicalParity {
    pub fn xor(self, other: Self) -> Self {
        Self {
            x: self.x ^ other.x,
            z: self.z ^ other.z,
        }
    }
}

/// Deterministic reconciliation metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReconciliationMetadata {
    pub schema_version: u16,
    pub partition_a: PartitionId,
    pub partition_b: PartitionId,
    pub boundary_count_a: usize,
    pub boundary_count_b: usize,
}

impl ReconciliationMetadata {
    fn validate(&self) -> Result<(), PartitionError> {
        if self.schema_version != PARTITION_SCHEMA_VERSION {
            return Err(
                PartitionError::UnsupportedSchemaVersion {
                    version: self.schema_version,
                },
            );
        }

        if self.partition_a == self.partition_b {
            return Err(PartitionError::SelfNeighbor {
                partition: self.partition_a,
            });
        }

        Ok(())
    }
}

// ============================================================================
// Partition plan
// ============================================================================

/// Result of deterministic partition planning.
#[derive(Debug, Clone)]
pub struct PartitionPlan {
    pub schema_version: u16,

    pub partitions: Vec<QecPartition>,

    /// Symmetric deterministic adjacency.
    pub adjacency:
        BTreeMap<PartitionId, BTreeSet<PartitionId>>,

    /// Boundary metadata grouped by partition.
    pub boundaries:
        BTreeMap<PartitionId, Vec<BoundaryElement>>,

    /// Explicit mathematical boundary contracts.
    pub boundary_contracts:
        Vec<PartitionBoundary>,

    pub resources: PartitionResources,

    /// Local results must not be interpreted globally without reconciliation.
    pub requires_reconciliation: bool,
}

impl PartitionPlan {
    pub fn validate(
        &self,
        limits: &QecLimits,
    ) -> Result<(), PartitionError> {
        if self.schema_version != PARTITION_SCHEMA_VERSION {
            return Err(
                PartitionError::UnsupportedSchemaVersion {
                    version: self.schema_version,
                },
            );
        }

        if self.partitions.is_empty() {
            return Err(PartitionError::NoPartitions);
        }

        if self.partitions.len() > limits.max_partitions {
            return Err(PartitionError::LimitExceeded {
                resource: LimitKind::Partitions,
                requested: self.partitions.len() as u128,
                maximum: limits.max_partitions as u128,
            });
        }

        let mut ids = BTreeSet::new();

        for partition in &self.partitions {
            if !ids.insert(partition.id) {
                return Err(PartitionError::DuplicatePartition {
                    partition: partition.id,
                });
            }

            partition.validate(limits)?;
        }

        for (partition, neighbors) in &self.adjacency {
            if !ids.contains(partition) {
                return Err(PartitionError::UnknownPartition {
                    partition: *partition,
                });
            }

            for neighbor in neighbors {
                if *partition == *neighbor {
                    return Err(PartitionError::SelfNeighbor {
                        partition: *partition,
                    });
                }

                if !ids.contains(neighbor) {
                    return Err(PartitionError::UnknownPartition {
                        partition: *neighbor,
                    });
                }

                let reverse = self
                    .adjacency
                    .get(neighbor)
                    .is_some_and(|set| set.contains(partition));

                if !reverse {
                    return Err(
                        PartitionError::AsymmetricAdjacency {
                            a: *partition,
                            b: *neighbor,
                        },
                    );
                }
            }
        }

        for contract in &self.boundary_contracts {
            contract.validate()?;

            if !self
                .adjacency
                .get(&contract.partition_a)
                .is_some_and(|set| {
                    set.contains(&contract.partition_b)
                })
            {
                return Err(
                    PartitionError::BoundaryWithoutAdjacency {
                        a: contract.partition_a,
                        b: contract.partition_b,
                    },
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// Partitioner
// ============================================================================

/// Deterministic, bounded partition planner.
///
/// `QecLimits` is copied into the planner. This is intentional: limits are a
/// small immutable policy object and therefore cannot change during a single
/// partitioning operation.
#[derive(Debug, Clone, Copy)]
pub struct Partitioner {
    strategy: PartitionStrategy,
    limits: QecLimits,
}

impl Partitioner {
    pub fn new(
        strategy: PartitionStrategy,
        limits: QecLimits,
    ) -> QecResult<Self> {
        limits
            .validate()
            .map_err(map_limit_error)?;

        if let PartitionStrategy::FixedCount { partitions } = strategy {
            if partitions == 0 {
                return Err(QecError::invalid_input(
                    "partition count must be greater than zero",
                ));
            }

            if partitions > limits.max_partitions {
                return Err(QecError::resource_limit(
                    ResourceKind::Partitions,
                    partitions as u128,
                    limits.max_partitions as u128,
                    "requested partition count exceeds QEC limits",
                ));
            }
        }

        Ok(Self { strategy, limits })
    }

    #[must_use]
    pub const fn strategy(&self) -> PartitionStrategy {
        self.strategy
    }

    #[must_use]
    pub const fn limits(&self) -> QecLimits {
        self.limits
    }

    /// Deterministically partitions a validated workload.
    pub fn partition(
        &self,
        input: PartitionInput,
    ) -> QecResult<PartitionPlan> {
        let cancellation = CancellationToken::new();

        self.partition_with_cancellation(
            input,
            &cancellation,
        )
    }

    /// Partitions using the canonical QEC cancellation infrastructure.
    pub fn partition_with_cancellation(
        &self,
        input: PartitionInput,
        cancellation: &CancellationToken,
    ) -> QecResult<PartitionPlan> {
        let started = Instant::now();

        self.preflight(&input, cancellation)?;

        let bounds =
            self.calculate_partition_bounds(&input.bounds)?;

        cancellation.check()?;

        if bounds.len() > self.limits.max_partitions {
            return Err(QecError::resource_limit(
                ResourceKind::Partitions,
                bounds.len() as u128,
                self.limits.max_partitions as u128,
                "partition planner produced too many partitions",
            ));
        }

        let mut partitions = Vec::with_capacity(bounds.len());

        for (index, bounds) in bounds.into_iter().enumerate() {
            cancellation.check()?;

            let id = PartitionId::try_from(index)
                .map_err(|_| {
                    QecError::numerical_failure(
                        "partition ID conversion overflow",
                    )
                })?;

            partitions.push(QecPartition::new(id, bounds));
        }

        let mut resources = PartitionResources {
            partitions_created: partitions.len() as u64,
            ..PartitionResources::default()
        };

        self.assign_items(
            &input,
            &mut partitions,
            &mut resources,
            cancellation,
            started,
        )?;

        self.assign_events(
            &input,
            &mut partitions,
            &mut resources,
            cancellation,
            started,
        )?;

        self.construct_boundaries(
            &input,
            &mut partitions,
            &mut resources,
            cancellation,
            started,
        )?;

        cancellation.check()?;

        let adjacency = build_adjacency(&partitions)?;

        let boundaries = partitions
            .iter()
            .map(|partition| {
                (partition.id, partition.boundaries.clone())
            })
            .collect::<BTreeMap<_, _>>();

        let boundary_contracts =
            build_boundary_contracts(
                &partitions,
                &adjacency,
                cancellation,
            )?;

        resources.elapsed = started.elapsed();

        let plan = PartitionPlan {
            schema_version: PARTITION_SCHEMA_VERSION,
            partitions,
            adjacency,
            boundaries,
            boundary_contracts,
            resources,
            requires_reconciliation: true,
        };

        plan.validate(&self.limits)
            .map_err(PartitionError::into_qec_error)?;

        Ok(plan)
    }

    fn preflight(
        &self,
        input: &PartitionInput,
        cancellation: &CancellationToken,
    ) -> QecResult<()> {
        cancellation.check()?;

        input
            .validate(&self.limits)
            .map_err(PartitionError::into_qec_error)?;

        /*
         * Geometric preflight.
         *
         * We do not allocate a dense lattice. The volume check only prevents
         * a caller from asking the partitioner to reason about a geometry
         * larger than the configured qubit policy.
         */
        let volume = input
            .bounds
            .checked_volume()
            .map_err(PartitionError::into_qec_error)?;

        if volume > self.limits.max_qubits as u64 {
            return Err(QecError::resource_limit(
                ResourceKind::Qubits,
                volume as u128,
                self.limits.max_qubits as u128,
                "partition geometry exceeds the configured qubit policy",
            ));
        }

        Ok(())
    }

    fn calculate_partition_bounds(
        &self,
        bounds: &Bounds,
    ) -> Result<Vec<Bounds>, PartitionError> {
        match self.strategy {
            PartitionStrategy::LongestAxis => {
                self.split_axis(bounds, longest_axis(bounds)?)
            }

            PartitionStrategy::FixedAxis(axis) => {
                self.split_axis(bounds, axis)
            }

            PartitionStrategy::FixedCount { partitions } => {
                self.split_fixed_count(bounds, partitions)
            }
        }
    }

    fn split_axis(
        &self,
        bounds: &Bounds,
        axis: PartitionAxis,
    ) -> Result<Vec<Bounds>, PartitionError> {
        let length = bounds.axis_length(axis)?;

        if length > self.limits.max_code_distance as u64
            && axis != PartitionAxis::Z
        {
            /*
             * `max_code_distance` is the closest canonical topology
             * dimension available in QecLimits. We use it only as a
             * conservative geometric preflight; actual code topology remains
             * responsible for its own distance validation.
             */
        }

        if length <= 1 {
            return Ok(vec![*bounds]);
        }

        let min = axis.coordinate(bounds.min);
        let max = axis.coordinate(bounds.max);

        let delta = max
            .checked_sub(min)
            .ok_or(PartitionError::ArithmeticOverflow)?;

        let midpoint = min
            .checked_add(delta / 2)
            .ok_or(PartitionError::ArithmeticOverflow)?;

        let right_min = midpoint
            .checked_add(1)
            .ok_or(PartitionError::ArithmeticOverflow)?;

        let left =
            replace_axis(*bounds, axis, min, midpoint)?;

        let right =
            replace_axis(*bounds, axis, right_min, max)?;

        Ok(vec![left, right])
    }

    fn split_fixed_count(
        &self,
        bounds: &Bounds,
        count: usize,
    ) -> Result<Vec<Bounds>, PartitionError> {
        if count == 0 {
            return Err(PartitionError::InvalidPartitionCount);
        }

        if count > self.limits.max_partitions {
            return Err(PartitionError::LimitExceeded {
                resource: LimitKind::Partitions,
                requested: count as u128,
                maximum: self.limits.max_partitions as u128,
            });
        }

        if count == 1 {
            return Ok(vec![*bounds]);
        }

        let mut result = vec![*bounds];

        while result.len() < count {
            let index = result
                .iter()
                .enumerate()
                .filter_map(|(index, candidate)| {
                    candidate
                        .axis_length(longest_axis(candidate).ok()?)
                        .ok()
                        .map(|length| (length, index))
                })
                .max_by_key(|(length, index)| (*length, *index))
                .map(|(_, index)| index)
                .ok_or(PartitionError::NoPartitions)?;

            let candidate = result.remove(index);

            let axis = longest_axis(&candidate)?;
            let pieces = self.split_axis(&candidate, axis)?;

            if pieces.len() == 1 {
                result.push(candidate);

                return Err(
                    PartitionError::UnableToSplit {
                        requested: count,
                        achieved: result.len(),
                    },
                );
            }

            result.extend(pieces);
        }

        Ok(result)
    }

    fn assign_items(
        &self,
        input: &PartitionInput,
        partitions: &mut [QecPartition],
        resources: &mut PartitionResources,
        cancellation: &CancellationToken,
        started: Instant,
    ) -> Result<(), PartitionError> {
        for item in &input.items {
            cancellation
                .check()
                .map_err(PartitionError::from_qec_error)?;

            check_elapsed(started, &self.limits)?;

            let partition =
                find_partition_mut(partitions, item.coordinate)
                    .ok_or(PartitionError::UnassignedItem {
                        qubit: item.qubit_id,
                    })?;

            if partition.items.len()
                >= self.limits.max_qubits_per_partition
            {
                return Err(PartitionError::LimitExceeded {
                    resource: LimitKind::QubitsPerPartition,
                    requested: (partition.items.len() + 1)
                        as u128,
                    maximum: self
                        .limits
                        .max_qubits_per_partition
                        as u128,
                });
            }

            partition.items.push(item.clone());

            PartitionResources::checked_increment(
                &mut resources.items_processed,
                1,
            )?;

            resources.peak_items_per_partition =
                resources.peak_items_per_partition.max(
                    partition.items.len() as u64,
                );
        }

        Ok(())
    }

    fn assign_events(
        &self,
        input: &PartitionInput,
        partitions: &mut [QecPartition],
        resources: &mut PartitionResources,
        cancellation: &CancellationToken,
        started: Instant,
    ) -> Result<(), PartitionError> {
        for event in &input.events {
            cancellation
                .check()
                .map_err(PartitionError::from_qec_error)?;

            check_elapsed(started, &self.limits)?;

            let partition =
                find_partition_mut(partitions, event.coordinate)
                    .ok_or(PartitionError::UnassignedEvent {
                        event: event.event_id,
                    })?;

            if partition.events.len()
                >= self.limits.max_syndrome_events
            {
                return Err(PartitionError::LimitExceeded {
                    resource: LimitKind::SyndromeEvents,
                    requested: (partition.events.len() + 1)
                        as u128,
                    maximum: self
                        .limits
                        .max_syndrome_events
                        as u128,
                });
            }

            partition.events.push(event.clone());

            PartitionResources::checked_increment(
                &mut resources.events_processed,
                1,
            )?;

            resources.peak_events_per_partition =
                resources.peak_events_per_partition.max(
                    partition.events.len() as u64,
                );
        }

        Ok(())
    }

    fn construct_boundaries(
        &self,
        input: &PartitionInput,
        partitions: &mut [QecPartition],
        resources: &mut PartitionResources,
        cancellation: &CancellationToken,
        started: Instant,
    ) -> Result<(), PartitionError> {
        /*
         * Determine adjacency from geometry first. This avoids scanning all
         * item/event pairs merely to discover neighboring partitions.
         */
        for i in 0..partitions.len() {
            cancellation
                .check()
                .map_err(PartitionError::from_qec_error)?;

            check_elapsed(started, &self.limits)?;

            for j in (i + 1)..partitions.len() {
                cancellation
                    .check()
                    .map_err(PartitionError::from_qec_error)?;

                if !partitions[i]
                    .bounds
                    .touches_face(&partitions[j].bounds)
                {
                    continue;
                }

                let a_id = partitions[i].id;
                let b_id = partitions[j].id;

                let coordinates_a =
                    shared_boundary_coordinates(
                        &partitions[i],
                        &partitions[j],
                    );

                let coordinates_b =
                    shared_boundary_coordinates(
                        &partitions[j],
                        &partitions[i],
                    );

                /*
                 * Even when there is no local item/event exactly on a
                 * coordinate, the boundary remains meaningful as a virtual
                 * reconciliation surface.
                 */
                if coordinates_a.is_empty()
                    && coordinates_b.is_empty()
                {
                    let coordinate =
                        shared_face_anchor(
                            &partitions[i].bounds,
                            &partitions[j].bounds,
                        )?;

                    add_boundary(
                        &mut partitions[i],
                        BoundaryElement {
                            key: BoundaryKey {
                                coordinate,
                                round: None,
                                graph_node: None,
                            },
                            qubit_id: None,
                            event_id: None,
                            kind: BoundaryKind::InterPartition,
                            neighboring_partitions:
                                BTreeSet::from([b_id]),
                            virtual_boundary: true,
                        },
                        self.limits.max_syndrome_events,
                        resources,
                    )?;

                    add_boundary(
                        &mut partitions[j],
                        BoundaryElement {
                            key: BoundaryKey {
                                coordinate,
                                round: None,
                                graph_node: None,
                            },
                            qubit_id: None,
                            event_id: None,
                            kind: BoundaryKind::InterPartition,
                            neighboring_partitions:
                                BTreeSet::from([a_id]),
                            virtual_boundary: true,
                        },
                        self.limits.max_syndrome_events,
                        resources,
                    )?;

                    continue;
                }

                for coordinate in coordinates_a {
                    let boundary = boundary_for_coordinate(
                        &partitions[i],
                        coordinate,
                        b_id,
                    );

                    add_boundary(
                        &mut partitions[i],
                        boundary,
                        self.limits.max_syndrome_events,
                        resources,
                    )?;
                }

                for coordinate in coordinates_b {
                    let boundary = boundary_for_coordinate(
                        &partitions[j],
                        coordinate,
                        a_id,
                    );

                    add_boundary(
                        &mut partitions[j],
                        boundary,
                        self.limits.max_syndrome_events,
                        resources,
                    )?;
                }
            }
        }

        /*
         * Physical boundaries are deliberately processed separately so that
         * a physical boundary cannot be confused with an inter-partition
         * virtual boundary.
         */
        if let Some(physical) = input.physical_boundary {
            for partition in partitions {
                cancellation
                    .check()
                    .map_err(PartitionError::from_qec_error)?;

                check_elapsed(started, &self.limits)?;

                if !partition.bounds.intersects(&physical) {
                    continue;
                }

                for coordinate in
                    physical_boundary_coordinates(partition, &physical)
                {
                    let key = BoundaryKey {
                        coordinate,
                        round: None,
                        graph_node: None,
                    };

                    if let Some(existing) =
                        partition.boundaries.iter_mut().find(
                            |boundary| boundary.key == key,
                        )
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
                                qubit_id: find_qubit_at(
                                    partition,
                                    coordinate,
                                ),
                                event_id: find_event_at(
                                    partition,
                                    coordinate,
                                ),
                                kind: BoundaryKind::Physical,
                                neighboring_partitions:
                                    BTreeSet::new(),
                                virtual_boundary: false,
                            },
                            self.limits.max_syndrome_events,
                            resources,
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// Boundary reconciliation
// ============================================================================

/// Creates deterministic boundary contracts for every neighboring pair.
pub fn build_boundary_contracts(
    partitions: &[QecPartition],
    adjacency: &BTreeMap<
        PartitionId,
        BTreeSet<PartitionId>,
    >,
    cancellation: &CancellationToken,
) -> Result<Vec<PartitionBoundary>, PartitionError> {
    let mut contracts = Vec::new();
    let mut next_id: ReconciliationId = 0;

    for (a, neighbors) in adjacency {
        cancellation
            .check()
            .map_err(PartitionError::from_qec_error)?;

        for b in neighbors {
            if a >= b {
                continue;
            }

            cancellation
                .check()
                .map_err(PartitionError::from_qec_error)?;

            let partition_a = partitions
                .iter()
                .find(|partition| partition.id == *a)
                .ok_or(PartitionError::UnknownPartition {
                    partition: *a,
                })?;

            let partition_b = partitions
                .iter()
                .find(|partition| partition.id == *b)
                .ok_or(PartitionError::UnknownPartition {
                    partition: *b,
                })?;

            let boundaries_a = partition_a
                .boundaries
                .iter()
                .filter(|boundary| {
                    boundary
                        .neighboring_partitions
                        .contains(b)
                })
                .collect::<Vec<_>>();

            let boundaries_b = partition_b
                .boundaries
                .iter()
                .filter(|boundary| {
                    boundary
                        .neighboring_partitions
                        .contains(a)
                })
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
                boundary_event_ids(&boundaries_a);
            let incoming_b =
                boundary_event_ids(&boundaries_b);

            let virtual_nodes = boundaries_a
                .iter()
                .chain(boundaries_b.iter())
                .filter_map(|boundary| {
                    boundary.key.graph_node
                })
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            let chain = build_boundary_chain(
                &boundaries_a,
                &boundaries_b,
            )?;

            let contract =
                PartitionBoundary {
                    reconciliation_id: next_id,
                    partition_a: *a,
                    partition_b: *b,

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

                    correction_chain: chain,

                    /*
                     * The partitioner itself does not decide the final
                     * logical class. It therefore carries the boundary
                     * parity initialized to identity.
                     */
                    logical_parity:
                        LogicalParity::default(),

                    reconciliation_metadata:
                        ReconciliationMetadata {
                            schema_version:
                                PARTITION_SCHEMA_VERSION,
                            partition_a: *a,
                            partition_b: *b,
                            boundary_count_a:
                                boundaries_a.len(),
                            boundary_count_b:
                                boundaries_b.len(),
                        },
                };

            contract.validate()?;
            contracts.push(contract);

            next_id = next_id
                .checked_add(1)
                .ok_or(PartitionError::ArithmeticOverflow)?;
        }
    }

    Ok(contracts)
}

/// Compatibility-oriented reconciliation-unit view.
///
/// This is intentionally a view over the stronger `PartitionBoundary`
/// contract rather than a second boundary representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryReconciliation {
    pub partition_a: PartitionId,
    pub partition_b: PartitionId,
    pub boundaries_a: Vec<BoundaryElement>,
    pub boundaries_b: Vec<BoundaryElement>,
    pub contract: PartitionBoundary,
}

impl BoundaryReconciliation {
    pub fn validate(&self) -> Result<(), PartitionError> {
        self.contract.validate()?;

        if self.partition_a != self.contract.partition_a
            || self.partition_b != self.contract.partition_b
        {
            return Err(
                PartitionError::ReconciliationMismatch,
            );
        }

        if self.boundaries_a.is_empty()
            || self.boundaries_b.is_empty()
        {
            return Err(PartitionError::MissingBoundaryData {
                a: self.partition_a,
                b: self.partition_b,
            });
        }

        Ok(())
    }
}

/// Generates deterministic reconciliation units.
pub fn reconciliation_units(
    plan: &PartitionPlan,
    limits: &QecLimits,
    cancellation: &CancellationToken,
) -> QecResult<Vec<BoundaryReconciliation>> {
    plan.validate(limits)
        .map_err(PartitionError::into_qec_error)?;

    let mut units = Vec::new();

    for contract in &plan.boundary_contracts {
        cancellation.check()?;

        let boundaries_a = plan
            .boundaries
            .get(&contract.partition_a)
            .ok_or_else(|| {
                QecError::invalid_topology(
                    "boundary contract references unknown partition",
                )
            })?
            .iter()
            .filter(|boundary| {
                boundary
                    .neighboring_partitions
                    .contains(&contract.partition_b)
            })
            .cloned()
            .collect::<Vec<_>>();

        let boundaries_b = plan
            .boundaries
            .get(&contract.partition_b)
            .ok_or_else(|| {
                QecError::invalid_topology(
                    "boundary contract references unknown partition",
                )
            })?
            .iter()
            .filter(|boundary| {
                boundary
                    .neighboring_partitions
                    .contains(&contract.partition_a)
            })
            .cloned()
            .collect::<Vec<_>>();

        let unit = BoundaryReconciliation {
            partition_a: contract.partition_a,
            partition_b: contract.partition_b,
            boundaries_a,
            boundaries_b,
            contract: contract.clone(),
        };

        unit.validate()
            .map_err(PartitionError::into_qec_error)?;

        units.push(unit);
    }

    Ok(units)
}

// ============================================================================
// Partition execution
// ============================================================================

/// Backend-independent partition executor.
///
/// The executor must not mutate global partition topology.
pub trait PartitionExecutor: Send + Sync {
    type Output;

    fn execute(
        &self,
        partition: &QecPartition,
        cancellation: &CancellationToken,
    ) -> QecResult<Self::Output>;
}

/// Results of deterministic partition execution.
#[derive(Debug, Clone)]
pub struct PartitionExecution<O> {
    pub results: BTreeMap<PartitionId, O>,
    pub adjacency:
        BTreeMap<PartitionId, BTreeSet<PartitionId>>,
    pub boundary_contracts:
        Vec<PartitionBoundary>,

    /// Always true until an explicit reconciliation stage completes.
    pub requires_boundary_reconciliation: bool,
}

impl<O> PartitionExecution<O> {
    /// Execute in stable partition-ID order.
    ///
    /// Parallel/distributed scheduling may be added above this interface, but
    /// deterministic collection remains keyed by partition ID.
    pub fn execute(
        plan: &PartitionPlan,
        limits: &QecLimits,
        executor: &impl PartitionExecutor<Output = O>,
        cancellation: &CancellationToken,
    ) -> QecResult<Self> {
        plan.validate(limits)
            .map_err(PartitionError::into_qec_error)?;

        let mut results = BTreeMap::new();

        for partition in &plan.partitions {
            cancellation.check()?;

            let result =
                executor.execute(partition, cancellation)?;

            if results.insert(partition.id, result).is_some() {
                return Err(QecError::internal_invariant(
                    "duplicate partition result",
                    "partition execution produced duplicate partition ID",
                ));
            }
        }

        Ok(Self {
            results,
            adjacency: plan.adjacency.clone(),
            boundary_contracts:
                plan.boundary_contracts.clone(),
            requires_boundary_reconciliation: true,
        })
    }
}

// ============================================================================
// Helpers
// ============================================================================

fn axis_length(
    min: i64,
    max: i64,
) -> Result<u64, PartitionError> {
    if min > max {
        return Err(PartitionError::InvalidBounds {
            min: Coordinate::new(min, 0, 0),
            max: Coordinate::new(max, 0, 0),
        });
    }

    let difference = max
        .checked_sub(min)
        .ok_or(PartitionError::ArithmeticOverflow)?;

    let length = difference
        .checked_add(1)
        .ok_or(PartitionError::ArithmeticOverflow)?;

    u64::try_from(length)
        .map_err(|_| PartitionError::ArithmeticOverflow)
}

fn longest_axis(
    bounds: &Bounds,
) -> Result<PartitionAxis, PartitionError> {
    let x = bounds.axis_length(PartitionAxis::X)?;
    let y = bounds.axis_length(PartitionAxis::Y)?;
    let z = bounds.axis_length(PartitionAxis::Z)?;

    Ok(if x >= y && x >= z {
        PartitionAxis::X
    } else if y >= x && y >= z {
        PartitionAxis::Y
    } else {
        PartitionAxis::Z
    })
}

fn replace_axis(
    bounds: Bounds,
    axis: PartitionAxis,
    min: i64,
    max: i64,
) -> Result<Bounds, PartitionError> {
    let mut lower = bounds.min;
    let mut upper = bounds.max;

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

    Bounds::new(lower, upper)
}

fn find_partition_mut(
    partitions: &mut [QecPartition],
    coordinate: Coordinate,
) -> Option<&mut QecPartition> {
    partitions
        .iter_mut()
        .find(|partition| {
            partition.bounds.contains(coordinate)
        })
}

fn find_qubit_at(
    partition: &QecPartition,
    coordinate: Coordinate,
) -> Option<QubitId> {
    partition
        .items
        .iter()
        .find(|item| item.coordinate == coordinate)
        .map(|item| item.qubit_id)
}

fn find_event_at(
    partition: &QecPartition,
    coordinate: Coordinate,
) -> Option<EventId> {
    partition
        .events
        .iter()
        .find(|event| event.coordinate == coordinate)
        .map(|event| event.event_id)
}

fn shared_boundary_coordinates(
    a: &QecPartition,
    b: &QecPartition,
) -> Vec<Coordinate> {
    let mut coordinates = BTreeSet::new();

    for item in &a.items {
        if is_adjacent_to_bounds(
            item.coordinate,
            &b.bounds,
        ) {
            coordinates.insert(item.coordinate);
        }
    }

    for event in &a.events {
        if is_adjacent_to_bounds(
            event.coordinate,
            &b.bounds,
        ) {
            coordinates.insert(event.coordinate);
        }
    }

    coordinates.into_iter().collect()
}

fn is_adjacent_to_bounds(
    coordinate: Coordinate,
    other: &Bounds,
) -> bool {
    let same_y =
        coordinate.y >= other.min.y
            && coordinate.y <= other.max.y;
    let same_z =
        coordinate.z >= other.min.z
            && coordinate.z <= other.max.z;

    let same_x =
        coordinate.x >= other.min.x
            && coordinate.x <= other.max.x;

    let x_adjacent =
        coordinate.x.checked_add(1) == Some(other.min.x)
            || coordinate.x.checked_sub(1) == Some(other.max.x);

    let y_adjacent =
        coordinate.y.checked_add(1) == Some(other.min.y)
            || coordinate.y.checked_sub(1) == Some(other.max.y);

    let z_adjacent =
        coordinate.z.checked_add(1) == Some(other.min.z)
            || coordinate.z.checked_sub(1) == Some(other.max.z);

    (x_adjacent && same_y && same_z)
        || (y_adjacent && same_x && same_z)
        || (z_adjacent && same_x && same_y)
}

fn shared_face_anchor(
    a: &Bounds,
    b: &Bounds,
) -> Result<Coordinate, PartitionError> {
    let x = if a.max.x.checked_add(1) == Some(b.min.x) {
        b.min.x
    } else if b.max.x.checked_add(1) == Some(a.min.x) {
        a.min.x
    } else {
        a.min.x.max(b.min.x)
    };

    let y = a.min.y.max(b.min.y);
    let z = a.min.z.max(b.min.z);

    Coordinate::new(x, y, z).validate()?;

    Ok(Coordinate::new(x, y, z))
}

fn boundary_for_coordinate(
    partition: &QecPartition,
    coordinate: Coordinate,
    neighbor: PartitionId,
) -> BoundaryElement {
    BoundaryElement {
        key: BoundaryKey {
            coordinate,
            round: None,
            graph_node: find_event_at(
                partition,
                coordinate,
            ),
        },
        qubit_id: find_qubit_at(
            partition,
            coordinate,
        ),
        event_id: find_event_at(
            partition,
            coordinate,
        ),
        kind: BoundaryKind::InterPartition,
        neighboring_partitions:
            BTreeSet::from([neighbor]),
        virtual_boundary: false,
    }
}

fn add_boundary(
    partition: &mut QecPartition,
    boundary: BoundaryElement,
    max_boundaries: usize,
    resources: &mut PartitionResources,
) -> Result<(), PartitionError> {
    boundary.validate()?;

    if let Some(existing) =
        partition.boundaries.iter_mut().find(
            |existing| existing.key == boundary.key,
        )
    {
        existing.kind =
            existing.kind.merge(boundary.kind);

        existing
            .neighboring_partitions
            .extend(boundary.neighboring_partitions);

        existing.qubit_id =
            existing.qubit_id.or(boundary.qubit_id);

        existing.event_id =
            existing.event_id.or(boundary.event_id);

        existing.virtual_boundary &=
            boundary.virtual_boundary;

        return Ok(());
    }

    if partition.boundaries.len() >= max_boundaries {
        return Err(PartitionError::LimitExceeded {
            resource: LimitKind::SyndromeEvents,
            requested: (partition.boundaries.len() + 1)
                as u128,
            maximum: max_boundaries as u128,
        });
    }

    partition.boundaries.push(boundary);

    PartitionResources::checked_increment(
        &mut resources.boundaries_created,
        1,
    )?;

    Ok(())
}

fn physical_boundary_coordinates(
    partition: &QecPartition,
    physical: &Bounds,
) -> Vec<Coordinate> {
    let mut coordinates = BTreeSet::new();

    for item in &partition.items {
        if physical.contains(item.coordinate) {
            coordinates.insert(item.coordinate);
        }
    }

    for event in &partition.events {
        if physical.contains(event.coordinate) {
            coordinates.insert(event.coordinate);
        }
    }

    coordinates.into_iter().collect()
}

fn build_adjacency(
    partitions: &[QecPartition],
) -> Result<
    BTreeMap<PartitionId, BTreeSet<PartitionId>>,
    PartitionError,
> {
    let mut adjacency = BTreeMap::new();

    for partition in partitions {
        adjacency.entry(partition.id).or_insert_with(
            BTreeSet::new,
        );
    }

    for partition in partitions {
        for boundary in &partition.boundaries {
            for neighbor in
                &boundary.neighboring_partitions
            {
                if *neighbor == partition.id {
                    return Err(
                        PartitionError::SelfNeighbor {
                            partition: partition.id,
                        },
                    );
                }

                adjacency
                    .entry(partition.id)
                    .or_insert_with(BTreeSet::new)
                    .insert(*neighbor);

                adjacency
                    .entry(*neighbor)
                    .or_insert_with(BTreeSet::new)
                    .insert(partition.id);
            }
        }
    }

    Ok(adjacency)
}

fn boundary_event_ids(
    boundaries: &[&BoundaryElement],
) -> Vec<EventId> {
    boundaries
        .iter()
        .filter_map(|boundary| boundary.event_id)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn build_boundary_chain(
    boundaries_a: &[&BoundaryElement],
    boundaries_b: &[&BoundaryElement],
) -> Result<Vec<BoundaryChainLink>, PartitionError> {
    let mut links = Vec::new();

    let mut a =
        boundaries_a.iter().map(|b| &b.key).collect::<Vec<_>>();

    let mut b =
        boundaries_b.iter().map(|b| &b.key).collect::<Vec<_>>();

    a.sort();
    b.sort();

    let count = a.len().min(b.len());

    for index in 0..count {
        links.push(BoundaryChainLink {
            from: a[index].clone(),
            to: b[index].clone(),
            parity: false,
        });
    }

    Ok(links)
}

fn check_elapsed(
    started: Instant,
    limits: &QecLimits,
) -> Result<(), PartitionError> {
    let elapsed = started.elapsed();

    let maximum =
        Duration::from_nanos(limits.max_decoder_time_ns);

    if elapsed > maximum {
        return Err(PartitionError::TimeLimitExceeded);
    }

    Ok(())
}

// ============================================================================
// Errors
// ============================================================================

/// Partition-local diagnostic error.
///
/// Public/high-level partition APIs convert this into `QecError`.
#[derive(Debug, Clone, PartialEq, Eq)]
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

    TimeLimitExceeded,

    ArithmeticOverflow,

    CancellationRequested,
}

impl PartitionError {
    fn from_qec_error(error: QecError) -> Self {
        match error {
            QecError::CancellationRequested { .. } => {
                Self::CancellationRequested
            }

            QecError::TimeLimitExceeded { .. } => {
                Self::TimeLimitExceeded
            }

            QecError::ResourceLimitExceeded {
                resource,
                requested,
                limit,
                ..
            } => {
                let resource = match resource {
                    ResourceKind::CodeDistance => {
                        LimitKind::CodeDistance
                    }
                    ResourceKind::Qubits => LimitKind::Qubits,
                    ResourceKind::Stabilizers => {
                        LimitKind::Stabilizers
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
                    ResourceKind::Parallelism => {
                        LimitKind::Parallelism
                    }
                    ResourceKind::CheckpointSize => {
                        LimitKind::CheckpointSizeBytes
                    }
                    ResourceKind::MemoryBytes => {
                        LimitKind::MemoryBytes
                    }
                    ResourceKind::QpuShots => {
                        LimitKind::QpuShots
                    }
                    ResourceKind::QpuCircuits => {
                        LimitKind::QpuCircuits
                    }
                    ResourceKind::Partitions => {
                        LimitKind::Partitions
                    }
                    ResourceKind::StreamBuffer => {
                        LimitKind::StreamBufferEvents
                    }
                    ResourceKind::AllocationCount
                    | ResourceKind::Custom => {
                        LimitKind::Partitions
                    }
                };

                Self::LimitExceeded {
                    resource,
                    requested,
                    maximum: limit,
                }
            }

            other => Self::InvalidBoundary {
                reason: match other {
                    QecError::InvalidInput { .. } => {
                        "invalid partition input"
                    }
                    QecError::InvalidTopology { .. } => {
                        "invalid partition topology"
                    }
                    _ => "QEC operation failed during partitioning",
                },
            },
        }
    }

    fn into_qec_error(self) -> QecError {
        match self {
            Self::LimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                QecError::resource_limit(
                    resource_to_qec_kind(resource),
                    requested,
                    maximum,
                    "partition resource policy exceeded",
                )
            }

            Self::TimeLimitExceeded => {
                QecError::time_limit(
                    0,
                    0,
                    "partition operation exceeded the configured QEC time limit",
                )
            }

            Self::CancellationRequested => {
                QecError::cancelled(
                    "partition operation was cancelled",
                )
            }

            Self::InvalidBounds { .. }
            | Self::CoordinateOutOfRange { .. }
            | Self::InvalidPartitionCount
            | Self::InvalidPhysicalBoundary
            | Self::InvalidBoundary { .. }
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
            | Self::NoPartitions
            | Self::UnableToSplit { .. }
            | Self::UnsupportedSchemaVersion { .. } => {
                QecError::invalid_topology(
                    self.to_string(),
                )
            }

            Self::ArithmeticOverflow => {
                QecError::numerical_failure(
                    "partition arithmetic overflow",
                )
            }
        }
    }
}

impl fmt::Display for PartitionError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidBounds { .. } => {
                write!(f, "invalid partition bounds")
            }

            Self::CoordinateOutOfRange { coordinate } => {
                write!(
                    f,
                    "partition coordinate out of range: {coordinate:?}"
                )
            }

            Self::InvalidPartitionCount => {
                write!(
                    f,
                    "partition count must be greater than zero"
                )
            }

            Self::InvalidPhysicalBoundary => {
                write!(
                    f,
                    "physical boundary lies outside input bounds"
                )
            }

            Self::InvalidBoundary { reason } => {
                write!(f, "invalid partition boundary: {reason}")
            }

            Self::DuplicatePartition { partition } => {
                write!(f, "duplicate partition: {partition}")
            }

            Self::DuplicateQubit { partition, qubit } => {
                write!(
                    f,
                    "duplicate qubit {qubit} in partition {partition}"
                )
            }

            Self::DuplicateEvent { partition, event } => {
                write!(
                    f,
                    "duplicate event {event} in partition {partition}"
                )
            }

            Self::DuplicateInputQubit { qubit } => {
                write!(f, "duplicate input qubit: {qubit}")
            }

            Self::DuplicateInputEvent { event } => {
                write!(f, "duplicate input event: {event}")
            }

            Self::DuplicateStabilizer {
                qubit,
                stabilizer,
            } => {
                write!(
                    f,
                    "duplicate stabilizer {stabilizer} on qubit {qubit}"
                )
            }

            Self::DuplicateBoundary {
                partition,
                ..
            } => {
                write!(
                    f,
                    "duplicate boundary in partition {partition}"
                )
            }

            Self::ItemOutsideBounds {
                partition,
                coordinate,
            } => {
                write!(
                    f,
                    "coordinate {coordinate:?} is outside partition {partition}"
                )
            }

            Self::ItemOutsideInputBounds { coordinate } => {
                write!(
                    f,
                    "coordinate {coordinate:?} is outside input bounds"
                )
            }

            Self::UnassignedItem { qubit } => {
                write!(
                    f,
                    "qubit {qubit} could not be assigned to a partition"
                )
            }

            Self::UnassignedEvent { event } => {
                write!(
                    f,
                    "event {event} could not be assigned to a partition"
                )
            }

            Self::UnknownPartition { partition } => {
                write!(f, "unknown partition: {partition}")
            }

            Self::SelfNeighbor { partition } => {
                write!(
                    f,
                    "partition {partition} cannot neighbor itself"
                )
            }

            Self::AsymmetricAdjacency { a, b } => {
                write!(
                    f,
                    "asymmetric partition adjacency between {a} and {b}"
                )
            }

            Self::BoundaryWithoutAdjacency { a, b } => {
                write!(
                    f,
                    "boundary contract {a}<->{b} has no adjacency"
                )
            }

            Self::MissingBoundaryData { a, b } => {
                write!(
                    f,
                    "missing boundary data for partitions {a}<->{b}"
                )
            }

            Self::ReconciliationMismatch => {
                write!(
                    f,
                    "partition reconciliation metadata mismatch"
                )
            }

            Self::NoPartitions => {
                write!(f, "partition plan contains no partitions")
            }

            Self::UnableToSplit {
                requested,
                achieved,
            } => {
                write!(
                    f,
                    "unable to create {requested} partitions; achieved {achieved}"
                )
            }

            Self::UnsupportedSchemaVersion { version } => {
                write!(
                    f,
                    "unsupported partition schema version {version}"
                )
            }

            Self::LimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "partition limit {resource} exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::TimeLimitExceeded => {
                write!(
                    f,
                    "partition operation exceeded its time limit"
                )
            }

            Self::ArithmeticOverflow => {
                write!(f, "partition arithmetic overflow")
            }

            Self::CancellationRequested => {
                write!(f, "partition operation cancelled")
            }
        }
    }
}

impl std::error::Error for PartitionError {}

fn map_limit_error(error: LimitError) -> QecError {
    match error {
        LimitError::InvalidLimit {
            resource,
            value,
        } => QecError::invalid_input(format!(
            "invalid QEC limit {resource}: {value}"
        )),

        LimitError::Exceeded {
            resource,
            requested,
            maximum,
        } => QecError::resource_limit(
            resource_to_qec_kind(resource),
            requested,
            maximum,
            "QEC limit exceeded",
        ),

        LimitError::ArithmeticOverflow {
            resource,
        } => QecError::numerical_failure(format!(
            "overflow while validating QEC limit {resource}"
        )),

        LimitError::InconsistentLimits {
            resource,
            related_resource,
            reason,
        } => QecError::invalid_input(format!(
            "inconsistent QEC limits {resource}/{related_resource}: {reason}"
        )),
    }
}

fn resource_to_qec_kind(
    resource: LimitKind,
) -> ResourceKind {
    match resource {
        LimitKind::CodeDistance => ResourceKind::CodeDistance,
        LimitKind::Qubits => ResourceKind::Qubits,
        LimitKind::Stabilizers => ResourceKind::Stabilizers,
        LimitKind::SyndromeEvents => {
            ResourceKind::SyndromeEvents
        }
        LimitKind::MeasurementRounds => {
            ResourceKind::MeasurementRounds
        }
        LimitKind::GraphNodes => ResourceKind::GraphNodes,
        LimitKind::GraphEdges => ResourceKind::GraphEdges,
        LimitKind::MemoryBytes => ResourceKind::MemoryBytes,
        LimitKind::DecoderTimeNs => {
            ResourceKind::DecoderIterations
        }
        LimitKind::Parallelism => ResourceKind::Parallelism,
        LimitKind::CheckpointSizeBytes => {
            ResourceKind::CheckpointSize
        }
        LimitKind::Partitions => ResourceKind::Partitions,
        LimitKind::StreamBufferEvents => {
            ResourceKind::StreamBuffer
        }
        LimitKind::DecoderIterations => {
            ResourceKind::DecoderIterations
        }
        LimitKind::StabilizerWeight => {
            ResourceKind::Stabilizers
        }
        LimitKind::LogicalOperatorWeight => {
            ResourceKind::Qubits
        }
        LimitKind::QubitsPerPartition => {
            ResourceKind::Qubits
        }
        LimitKind::QpuShots => ResourceKind::QpuShots,
        LimitKind::QpuCircuits => ResourceKind::QpuCircuits,
        LimitKind::VerificationOperations => {
            ResourceKind::AllocationCount
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_limits() -> QecLimits {
        let mut limits = QecLimits::default();

        limits.max_qubits = 1_000;
        limits.max_syndrome_events = 1_000;
        limits.max_partitions = 64;
        limits.max_qubits_per_partition = 1_000;
        limits.max_decoder_time_ns = 10_000_000_000;

        limits
    }

    fn test_bounds() -> Bounds {
        Bounds::new(
            Coordinate::new(0, 0, 0),
            Coordinate::new(9, 9, 0),
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
            coordinate: Coordinate::new(x, y, 0),
            stabilizers: vec![id],
        }
    }

    #[test]
    fn bounds_use_checked_geometry() {
        let bounds = test_bounds();

        assert_eq!(
            bounds
                .axis_length(PartitionAxis::X)
                .unwrap(),
            10
        );

        assert_eq!(
            bounds.checked_volume().unwrap(),
            100
        );
    }

    #[test]
    fn longest_axis_partition_is_deterministic() {
        let limits = test_limits();

        let partitioner = Partitioner::new(
            PartitionStrategy::LongestAxis,
            limits,
        )
        .unwrap();

        let input = PartitionInput {
            bounds: test_bounds(),
            items: vec![
                item(1, 1, 1),
                item(2, 8, 8),
            ],
            events: Vec::new(),
            physical_boundary: None,
        };

        let a = partitioner.partition(input.clone()).unwrap();
        let b = partitioner.partition(input).unwrap();

        assert_eq!(
            a.partitions
                .iter()
                .map(|p| p.bounds)
                .collect::<Vec<_>>(),
            b.partitions
                .iter()
                .map(|p| p.bounds)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn fixed_count_creates_exact_count() {
        let limits = test_limits();

        let partitioner = Partitioner::new(
            PartitionStrategy::FixedCount { partitions: 8 },
            limits,
        )
        .unwrap();

        let input = PartitionInput {
            bounds: test_bounds(),
            items: Vec::new(),
            events: Vec::new(),
            physical_boundary: None,
        };

        let plan = partitioner.partition(input).unwrap();

        assert_eq!(plan.partitions.len(), 8);
    }

    #[test]
    fn duplicate_input_qubits_are_rejected() {
        let limits = test_limits();

        let partitioner = Partitioner::new(
            PartitionStrategy::FixedCount { partitions: 2 },
            limits,
        )
        .unwrap();

        let input = PartitionInput {
            bounds: test_bounds(),
            items: vec![
                item(1, 1, 1),
                item(1, 2, 2),
            ],
            events: Vec::new(),
            physical_boundary: None,
        };

        assert!(partitioner.partition(input).is_err());
    }

    #[test]
    fn cancellation_is_honoured() {
        let limits = test_limits();

        let partitioner = Partitioner::new(
            PartitionStrategy::FixedCount { partitions: 2 },
            limits,
        )
        .unwrap();

        let source =
            super::super::cancellation::CancellationSource::new();

        source.cancel();

        let input = PartitionInput {
            bounds: test_bounds(),
            items: Vec::new(),
            events: Vec::new(),
            physical_boundary: None,
        };

        let result =
            partitioner.partition_with_cancellation(
                input,
                &source.token(),
            );

        assert!(matches!(
            result,
            Err(QecError::CancellationRequested { .. })
        ));
    }

    #[test]
    fn adjacency_is_symmetric() {
        let limits = test_limits();

        let partitioner = Partitioner::new(
            PartitionStrategy::FixedCount { partitions: 2 },
            limits,
        )
        .unwrap();

        let input = PartitionInput {
            bounds: test_bounds(),
            items: vec![
                item(1, 1, 1),
                item(2, 8, 8),
            ],
            events: Vec::new(),
            physical_boundary: None,
        };

        let plan = partitioner.partition(input).unwrap();

        plan.validate(&limits).unwrap();

        for (a, neighbors) in &plan.adjacency {
            for b in neighbors {
                assert!(
                    plan.adjacency
                        .get(b)
                        .is_some_and(|set| set.contains(a))
                );
            }
        }
    }

    #[test]
    fn reconciliation_contracts_are_explicit() {
        let limits = test_limits();

        let partitioner = Partitioner::new(
            PartitionStrategy::FixedCount { partitions: 2 },
            limits,
        )
        .unwrap();

        let input = PartitionInput {
            bounds: test_bounds(),
            items: vec![
                item(1, 1, 1),
                item(2, 8, 8),
            ],
            events: vec![
                PartitionEvent {
                    event_id: 1,
                    coordinate: Coordinate::new(4, 4, 0),
                    round: 1,
                    graph_node: Some(10),
                },
            ],
            physical_boundary: None,
        };

        let plan = partitioner.partition(input).unwrap();

        assert!(plan.requires_reconciliation);

        for contract in &plan.boundary_contracts {
            contract.validate().unwrap();
        }
    }

    #[test]
    fn physical_boundary_is_not_confused_with_partition_boundary() {
        let limits = test_limits();

        let partitioner = Partitioner::new(
            PartitionStrategy::FixedCount { partitions: 2 },
            limits,
        )
        .unwrap();

        let input = PartitionInput {
            bounds: test_bounds(),
            items: vec![
                item(1, 0, 0),
                item(2, 9, 9),
            ],
            events: Vec::new(),
            physical_boundary: Some(
                Bounds::new(
                    Coordinate::new(0, 0, 0),
                    Coordinate::new(0, 9, 0),
                )
                .unwrap(),
            ),
        };

        let plan = partitioner.partition(input).unwrap();

        assert!(plan.partitions.iter().any(|partition| {
            partition
                .boundaries
                .iter()
                .any(|boundary| {
                    matches!(
                        boundary.kind,
                        BoundaryKind::Physical
                            | BoundaryKind::Mixed
                    )
                })
        }));
    }
}