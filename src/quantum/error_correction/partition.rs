//! Zamani Quantum Error Correction — Partitioning
//!
//! Provides bounded, deterministic partitioning of large QEC workloads.
//!
//! Design goals:
//! - Never promise infinite resources.
//! - Never allocate directly from untrusted sizes without checking limits.
//! - Preserve boundary information required for global decoding.
//! - Support streaming/incremental partition construction.
//! - Support deterministic single-threaded and parallel/distributed execution.
//! - Support cancellation and resource accounting.
//! - Avoid panics for malformed external input.
//! - Keep partitioning independent from a particular decoder.
//!
//! The partitioner deliberately does NOT perform global correction.
//! It creates independently addressable regions plus the boundary state
//! required by a later reconciliation stage.
//!
//! Typical flow:
//!
//!   validated QEC object
//!          |
//!          v
//!      Partitioner
//!          |
//!          +---- Partition A
//!          +---- Partition B
//!          +---- Partition C
//!          |
//!          v
//!   Boundary reconciliation
//!          |
//!          v
//!     Global decoder
//!
//! Security model:
//!   External input -> validate -> bounded planning -> allocation -> execution.
//!
//! Correctness model:
//!   Local partition results MUST NOT be considered globally correct until
//!   boundary reconciliation has completed.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::time::{Duration, Instant};

/// Stable identifier for a QEC partition.
pub type PartitionId = u64;

/// Stable identifier for a qubit.
pub type QubitId = u64;

/// Stable identifier for a stabilizer.
pub type StabilizerId = u64;

/// Stable identifier for a syndrome/detection event.
pub type EventId = u64;

/// Stable identifier for a graph node.
pub type GraphNodeId = u64;

/// Logical coordinate in a QEC lattice.
///
/// Coordinates are signed because partition boundaries may be represented
/// relative to a global origin or transformed coordinate system.
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

    pub fn checked_offset(
        self,
        dx: i64,
        dy: i64,
        dz: i64,
    ) -> Option<Self> {
        Some(Self {
            x: self.x.checked_add(dx)?,
            y: self.y.checked_add(dy)?,
            z: self.z.checked_add(dz)?,
        })
    }
}

/// Axis along which a partition is split.
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

/// Partitioning strategy.
///
/// Deterministic geometric partitioning is preferred for QEC because
/// partition identities should not depend on hash-map iteration order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionStrategy {
    /// Split along the longest coordinate extent.
    LongestAxis,

    /// Split along a specified axis.
    FixedAxis(PartitionAxis),

    /// Divide into a fixed number of rectangular regions.
    FixedCount {
        partitions: usize,
    },
}

/// Boundary classification.
///
/// A boundary event/qubit/node can belong to one or more categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoundaryKind {
    /// Internal to a partition.
    Internal,

    /// Touches another partition.
    InterPartition,

    /// Touches the physical boundary of the QEC code.
    Physical,

    /// Touches both another partition and a physical boundary.
    Mixed,
}

/// A geometric bounding box.
///
/// Bounds are inclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bounds {
    pub min: Coordinate,
    pub max: Coordinate,
}

impl Bounds {
    pub fn new(min: Coordinate, max: Coordinate) -> Result<Self, PartitionError> {
        if min.x > max.x || min.y > max.y || min.z > max.z {
            return Err(PartitionError::InvalidBounds {
                min,
                max,
            });
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

    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    pub fn touches(&self, other: &Self) -> bool {
        if !self.intersects_expanded(other) {
            return false;
        }

        let touching_x =
            self.max.x.checked_add(1) == Some(other.min.x)
                || other.max.x.checked_add(1) == Some(self.min.x);

        let touching_y =
            self.max.y.checked_add(1) == Some(other.min.y)
                || other.max.y.checked_add(1) == Some(self.min.y);

        let touching_z =
            self.max.z.checked_add(1) == Some(other.min.z)
                || other.max.z.checked_add(1) == Some(self.min.z);

        touching_x || touching_y || touching_z
    }

    fn intersects_expanded(&self, other: &Self) -> bool {
        fn axis_intersects(
            a_min: i64,
            a_max: i64,
            b_min: i64,
            b_max: i64,
        ) -> bool {
            let a_max = a_max.checked_add(1).unwrap_or(i64::MAX);
            let b_max = b_max.checked_add(1).unwrap_or(i64::MAX);

            a_min <= b_max && b_min <= a_max
        }

        axis_intersects(self.min.x, self.max.x, other.min.x, other.max.x)
            && axis_intersects(self.min.y, self.max.y, other.min.y, other.max.y)
            && axis_intersects(self.min.z, self.max.z, other.min.z, other.max.z)
    }
}

/// QEC item assigned to a partition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionItem {
    pub qubit_id: QubitId,
    pub coordinate: Coordinate,
    pub stabilizers: Vec<StabilizerId>,
}

impl PartitionItem {
    pub fn validate(&self) -> Result<(), PartitionError> {
        if self.stabilizers.windows(2).any(|w| w[0] == w[1]) {
            return Err(PartitionError::DuplicateStabilizer {
                partition_item: self.qubit_id,
            });
        }

        Ok(())
    }
}

/// A syndrome/detection event associated with a partition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionEvent {
    pub event_id: EventId,
    pub coordinate: Coordinate,
    pub round: u64,
    pub graph_node: Option<GraphNodeId>,
}

impl PartitionEvent {
    pub fn validate(&self) -> Result<(), PartitionError> {
        // Round itself is unsigned and therefore cannot be negative.
        // This method exists so future schema validation can be added
        // without changing the public API.
        let _ = self.round;
        Ok(())
    }
}

/// Boundary element.
///
/// Boundary information is intentionally explicit instead of being inferred
/// later from local partition data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryElement {
    pub qubit_id: Option<QubitId>,
    pub event_id: Option<EventId>,
    pub coordinate: Coordinate,
    pub kind: BoundaryKind,
    pub neighboring_partitions: BTreeSet<PartitionId>,
}

impl BoundaryElement {
    fn validate(&self) -> Result<(), PartitionError> {
        if self.qubit_id.is_none() && self.event_id.is_none() {
            return Err(PartitionError::InvalidBoundary {
                reason: "boundary element has neither qubit nor event identity",
            });
        }

        if self.kind == BoundaryKind::InterPartition
            && self.neighboring_partitions.is_empty()
        {
            return Err(PartitionError::InvalidBoundary {
                reason: "inter-partition boundary has no neighboring partitions",
            });
        }

        Ok(())
    }
}

/// A partition of a QEC workload.
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

    pub fn validate(&self) -> Result<(), PartitionError> {
        let mut qubits = BTreeSet::new();
        let mut events = BTreeSet::new();

        for item in &self.items {
            item.validate()?;

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
        }

        Ok(())
    }
}

/// Input to the partitioner.
///
/// This structure intentionally uses iterables owned by the caller rather
/// than requiring a dense global matrix.
#[derive(Debug, Clone)]
pub struct PartitionInput {
    pub bounds: Bounds,
    pub items: Vec<PartitionItem>,
    pub events: Vec<PartitionEvent>,
    pub physical_boundary: Option<Bounds>,
}

impl PartitionInput {
    pub fn validate(&self) -> Result<(), PartitionError> {
        let mut qubits = BTreeSet::new();
        let mut events = BTreeSet::new();

        for item in &self.items {
            item.validate()?;

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

        if let Some(physical_boundary) = self.physical_boundary {
            if !self.bounds.contains(physical_boundary.min)
                || !self.bounds.contains(physical_boundary.max)
            {
                return Err(PartitionError::InvalidPhysicalBoundary);
            }
        }

        Ok(())
    }
}

/// Explicit partitioning limits.
///
/// These are intentionally local to this module so that partitioning cannot
/// silently bypass global QEC resource policies.
#[derive(Debug, Clone, Copy)]
pub struct PartitionLimits {
    pub max_partitions: usize,
    pub max_items_total: usize,
    pub max_events_total: usize,
    pub max_items_per_partition: usize,
    pub max_events_per_partition: usize,
    pub max_boundaries_total: usize,
    pub max_neighbors_per_boundary: usize,
    pub max_partition_dimension: u64,
    pub max_execution_time: Option<Duration>,
}

impl Default for PartitionLimits {
    fn default() -> Self {
        Self {
            max_partitions: 4096,
            max_items_total: 10_000_000,
            max_events_total: 10_000_000,
            max_items_per_partition: 5_000_000,
            max_events_per_partition: 5_000_000,
            max_boundaries_total: 20_000_000,
            max_neighbors_per_boundary: 64,
            max_partition_dimension: 1_000_000,
            max_execution_time: None,
        }
    }
}

impl PartitionLimits {
    pub fn validate(&self) -> Result<(), PartitionError> {
        if self.max_partitions == 0 {
            return Err(PartitionError::InvalidLimit {
                name: "max_partitions",
            });
        }

        if self.max_items_per_partition == 0 {
            return Err(PartitionError::InvalidLimit {
                name: "max_items_per_partition",
            });
        }

        if self.max_events_per_partition == 0 {
            return Err(PartitionError::InvalidLimit {
                name: "max_events_per_partition",
            });
        }

        if self.max_boundaries_total == 0 {
            return Err(PartitionError::InvalidLimit {
                name: "max_boundaries_total",
            });
        }

        Ok(())
    }
}

/// Cancellation interface.
///
/// Implementations can bridge to Zamani's dedicated cancellation module.
pub trait CancellationToken: Send + Sync {
    fn is_cancelled(&self) -> bool;
}

/// A cancellation token that never cancels.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoCancellation;

impl CancellationToken for NoCancellation {
    fn is_cancelled(&self) -> bool {
        false
    }
}

/// Resource accounting for a partitioning operation.
#[derive(Debug, Clone, Default)]
pub struct PartitionResources {
    pub items_processed: u64,
    pub events_processed: u64,
    pub partitions_created: u64,
    pub boundaries_created: u64,
    pub neighbor_links_created: u64,
    pub peak_partition_items: u64,
    pub peak_partition_events: u64,
    pub elapsed: Duration,
}

impl PartitionResources {
    fn check_limits(
        &self,
        limits: &PartitionLimits,
    ) -> Result<(), PartitionError> {
        if self.items_processed
            > limits.max_items_total as u64
        {
            return Err(PartitionError::ResourceLimitExceeded {
                resource: "items",
            });
        }

        if self.events_processed
            > limits.max_events_total as u64
        {
            return Err(PartitionError::ResourceLimitExceeded {
                resource: "events",
            });
        }

        if self.partitions_created
            > limits.max_partitions as u64
        {
            return Err(PartitionError::ResourceLimitExceeded {
                resource: "partitions",
            });
        }

        if self.boundaries_created
            > limits.max_boundaries_total as u64
        {
            return Err(PartitionError::ResourceLimitExceeded {
                resource: "boundaries",
            });
        }

        Ok(())
    }
}

/// Result of a partitioning operation.
#[derive(Debug, Clone)]
pub struct PartitionPlan {
    pub partitions: Vec<QecPartition>,

    /// Deterministically ordered adjacency map.
    ///
    /// `A -> {B, C}` means partition A has a boundary relationship with B/C.
    pub adjacency: BTreeMap<PartitionId, BTreeSet<PartitionId>>,

    /// Boundary elements grouped by partition.
    pub boundaries: BTreeMap<PartitionId, Vec<BoundaryElement>>,

    pub resources: PartitionResources,

    /// True when local partitioning completed but global boundary
    /// reconciliation has not yet been performed.
    pub requires_reconciliation: bool,
}

impl PartitionPlan {
    pub fn validate(&self) -> Result<(), PartitionError> {
        if self.partitions.is_empty() {
            return Err(PartitionError::NoPartitions);
        }

        let mut ids = BTreeSet::new();

        for partition in &self.partitions {
            if !ids.insert(partition.id) {
                return Err(PartitionError::DuplicatePartition {
                    partition: partition.id,
                });
            }

            partition.validate()?;
        }

        for (partition, neighbors) in &self.adjacency {
            if !ids.contains(partition) {
                return Err(PartitionError::UnknownPartition {
                    partition: *partition,
                });
            }

            for neighbor in neighbors {
                if !ids.contains(neighbor) {
                    return Err(PartitionError::UnknownPartition {
                        partition: *neighbor,
                    });
                }

                if partition == neighbor {
                    return Err(PartitionError::SelfNeighbor {
                        partition: *partition,
                    });
                }

                let reverse = self
                    .adjacency
                    .get(neighbor)
                    .map(|set| set.contains(partition))
                    .unwrap_or(false);

                if !reverse {
                    return Err(PartitionError::AsymmetricAdjacency {
                        a: *partition,
                        b: *neighbor,
                    });
                }
            }
        }

        Ok(())
    }
}

/// Main partitioning engine.
#[derive(Debug, Clone)]
pub struct Partitioner {
    strategy: PartitionStrategy,
    limits: PartitionLimits,
}

impl Partitioner {
    pub fn new(
        strategy: PartitionStrategy,
        limits: PartitionLimits,
    ) -> Result<Self, PartitionError> {
        limits.validate()?;

        if let PartitionStrategy::FixedCount { partitions } = strategy {
            if partitions == 0 {
                return Err(PartitionError::InvalidPartitionCount);
            }

            if partitions > limits.max_partitions {
                return Err(PartitionError::ResourceLimitExceeded {
                    resource: "partitions",
                });
            }
        }

        Ok(Self {
            strategy,
            limits,
        })
    }

    pub fn strategy(&self) -> PartitionStrategy {
        self.strategy
    }

    pub fn limits(&self) -> PartitionLimits {
        self.limits
    }

    /// Partition a validated workload.
    ///
    /// The input is validated before allocation or partition construction.
    pub fn partition(
        &self,
        input: PartitionInput,
    ) -> Result<PartitionPlan, PartitionError> {
        self.partition_with_cancellation(input, &NoCancellation)
    }

    /// Partition with cooperative cancellation.
    pub fn partition_with_cancellation<C>(
        &self,
        input: PartitionInput,
        cancellation: &C,
    ) -> Result<PartitionPlan, PartitionError>
    where
        C: CancellationToken,
    {
        let started = Instant::now();

        input.validate()?;

        let mut resources = PartitionResources::default();

        self.check_cancelled(cancellation)?;
        self.check_time(started)?;

        let bounds = self.calculate_partition_bounds(&input.bounds)?;

        self.check_cancelled(cancellation)?;

        let mut partitions = bounds
            .into_iter()
            .enumerate()
            .map(|(index, bounds)| {
                QecPartition::new(index as PartitionId, bounds)
            })
            .collect::<Vec<_>>();

        resources.partitions_created = partitions.len() as u64;
        resources.check_limits(&self.limits)?;

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

        let adjacency = self.build_adjacency(&partitions)?;

        let boundaries = partitions
            .iter()
            .map(|partition| {
                (
                    partition.id,
                    partition.boundaries.clone(),
                )
            })
            .collect();

        resources.elapsed = started.elapsed();

        let plan = PartitionPlan {
            partitions,
            adjacency,
            boundaries,
            resources,
            requires_reconciliation: true,
        };

        plan.validate()?;

        Ok(plan)
    }

    fn calculate_partition_bounds(
        &self,
        bounds: &Bounds,
    ) -> Result<Vec<Bounds>, PartitionError> {
        match self.strategy {
            PartitionStrategy::LongestAxis => {
                self.split_longest_axis(bounds)
            }

            PartitionStrategy::FixedAxis(axis) => {
                self.split_axis(bounds, axis)
            }

            PartitionStrategy::FixedCount { partitions } => {
                self.split_fixed_count(bounds, partitions)
            }
        }
    }

    fn split_longest_axis(
        &self,
        bounds: &Bounds,
    ) -> Result<Vec<Bounds>, PartitionError> {
        let x = axis_length(bounds.min.x, bounds.max.x)?;
        let y = axis_length(bounds.min.y, bounds.max.y)?;
        let z = axis_length(bounds.min.z, bounds.max.z)?;

        let axis = if x >= y && x >= z {
            PartitionAxis::X
        } else if y >= x && y >= z {
            PartitionAxis::Y
        } else {
            PartitionAxis::Z
        };

        self.split_axis(bounds, axis)
    }

    fn split_axis(
        &self,
        bounds: &Bounds,
        axis: PartitionAxis,
    ) -> Result<Vec<Bounds>, PartitionError> {
        let min = axis.coordinate(bounds.min);
        let max = axis.coordinate(bounds.max);

        let length = axis_length(min, max)?;

        if length > self.limits.max_partition_dimension {
            return Err(PartitionError::DimensionLimitExceeded {
                axis,
                length,
            });
        }

        if length <= 1 {
            return Ok(vec![*bounds]);
        }

        let midpoint = min
            .checked_add((max - min) / 2)
            .ok_or(PartitionError::ArithmeticOverflow)?;

        let (left_min, left_max) = (min, midpoint);
        let right_min = midpoint
            .checked_add(1)
            .ok_or(PartitionError::ArithmeticOverflow)?;

        let left = replace_axis(
            *bounds,
            axis,
            left_min,
            left_max,
        )?;

        let right = replace_axis(
            *bounds,
            axis,
            right_min,
            max,
        )?;

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
            return Err(PartitionError::ResourceLimitExceeded {
                resource: "partitions",
            });
        }

        if count == 1 {
            return Ok(vec![*bounds]);
        }

        // Build a deterministic recursive binary subdivision.
        //
        // This avoids a potentially enormous dense 3-D grid and preserves
        // rectangular regions.
        let mut result = vec![*bounds];

        while result.len() < count {
            let index = result
                .iter()
                .enumerate()
                .max_by_key(|(_, bound)| {
                    (
                        axis_length(bound.min.x, bound.max.x).unwrap_or(0),
                        axis_length(bound.min.y, bound.max.y).unwrap_or(0),
                        axis_length(bound.min.z, bound.max.z).unwrap_or(0),
                    )
                })
                .map(|(index, _)| index)
                .ok_or(PartitionError::NoPartitions)?;

            let candidate = result.remove(index);

            let axis = longest_axis(&candidate)?;
            let pieces = self.split_axis(&candidate, axis)?;

            if pieces.len() == 1 {
                result.push(candidate);

                return Err(PartitionError::UnableToSplit {
                    requested: count,
                    achieved: result.len(),
                });
            }

            result.extend(pieces);
        }

        Ok(result)
    }

    fn assign_items<C>(
        &self,
        input: &PartitionInput,
        partitions: &mut [QecPartition],
        resources: &mut PartitionResources,
        cancellation: &C,
        started: Instant,
    ) -> Result<(), PartitionError>
    where
        C: CancellationToken,
    {
        for item in &input.items {
            self.check_cancelled(cancellation)?;
            self.check_time(started)?;

            let partition = find_partition_mut(
                partitions,
                item.coordinate,
            )
            .ok_or(PartitionError::UnassignedItem {
                qubit: item.qubit_id,
            })?;

            if partition.items.len()
                >= self.limits.max_items_per_partition
            {
                return Err(PartitionError::ResourceLimitExceeded {
                    resource: "items_per_partition",
                });
            }

            partition.items.push(item.clone());

            resources.items_processed =
                resources.items_processed.saturating_add(1);

            resources.peak_partition_items =
                resources.peak_partition_items.max(
                    partition.items.len() as u64,
                );

            resources.check_limits(&self.limits)?;
        }

        Ok(())
    }

    fn assign_events<C>(
        &self,
        input: &PartitionInput,
        partitions: &mut [QecPartition],
        resources: &mut PartitionResources,
        cancellation: &C,
        started: Instant,
    ) -> Result<(), PartitionError>
    where
        C: CancellationToken,
    {
        for event in &input.events {
            self.check_cancelled(cancellation)?;
            self.check_time(started)?;

            let partition = find_partition_mut(
                partitions,
                event.coordinate,
            )
            .ok_or(PartitionError::UnassignedEvent {
                event: event.event_id,
            })?;

            if partition.events.len()
                >= self.limits.max_events_per_partition
            {
                return Err(PartitionError::ResourceLimitExceeded {
                    resource: "events_per_partition",
                });
            }

            partition.events.push(event.clone());

            resources.events_processed =
                resources.events_processed.saturating_add(1);

            resources.peak_partition_events =
                resources.peak_partition_events.max(
                    partition.events.len() as u64,
                );

            resources.check_limits(&self.limits)?;
        }

        Ok(())
    }

    fn construct_boundaries<C>(
        &self,
        input: &PartitionInput,
        partitions: &mut [QecPartition],
        resources: &mut PartitionResources,
        cancellation: &C,
        started: Instant,
    ) -> Result<(), PartitionError>
    where
        C: CancellationToken,
    {
        // First construct inter-partition boundary metadata.
        for i in 0..partitions.len() {
            self.check_cancelled(cancellation)?;
            self.check_time(started)?;

            for j in (i + 1)..partitions.len() {
                if !partitions[i]
                    .bounds
                    .touches(&partitions[j].bounds)
                {
                    continue;
                }

                let left_id = partitions[i].id;
                let right_id = partitions[j].id;

                let left_boundary_coordinates =
                    boundary_coordinates(
                        &partitions[i],
                        &partitions[j],
                    );

                let right_boundary_coordinates =
                    left_boundary_coordinates.clone();

                for coordinate in left_boundary_coordinates {
                    self.add_boundary(
                        &mut partitions[i],
                        BoundaryElement {
                            qubit_id: find_qubit_at(
                                &partitions[i],
                                coordinate,
                            ),
                            event_id: find_event_at(
                                &partitions[i],
                                coordinate,
                            ),
                            coordinate,
                            kind: BoundaryKind::InterPartition,
                            neighboring_partitions:
                                BTreeSet::from([right_id]),
                        },
                        resources,
                    )?;
                }

                for coordinate in right_boundary_coordinates {
                    self.add_boundary(
                        &mut partitions[j],
                        BoundaryElement {
                            qubit_id: find_qubit_at(
                                &partitions[j],
                                coordinate,
                            ),
                            event_id: find_event_at(
                                &partitions[j],
                                coordinate,
                            ),
                            coordinate,
                            kind: BoundaryKind::InterPartition,
                            neighboring_partitions:
                                BTreeSet::from([left_id]),
                        },
                        resources,
                    )?;
                }
            }
        }

        // Physical boundaries are tracked independently. This is essential:
        // a partition touching the physical code boundary must not confuse
        // physical boundaries with partition boundaries.
        if let Some(physical) = input.physical_boundary {
            for partition in partitions {
                if !partition.bounds.intersects(&physical) {
                    continue;
                }

                let coordinates =
                    physical_boundary_coordinates(
                        partition,
                        &physical,
                    );

                for coordinate in coordinates {
                    let existing = partition
                        .boundaries
                        .iter_mut()
                        .find(|boundary| {
                            boundary.coordinate == coordinate
                        });

                    if let Some(boundary) = existing {
                        boundary.kind = BoundaryKind::Mixed;
                    } else {
                        self.add_boundary(
                            partition,
                            BoundaryElement {
                                qubit_id: find_qubit_at(
                                    partition,
                                    coordinate,
                                ),
                                event_id: find_event_at(
                                    partition,
                                    coordinate,
                                ),
                                coordinate,
                                kind: BoundaryKind::Physical,
                                neighboring_partitions:
                                    BTreeSet::new(),
                            },
                            resources,
                        )?;
                    }
                }
            }
        }

        Ok(())
    }

    fn add_boundary(
        &self,
        partition: &mut QecPartition,
        boundary: BoundaryElement,
        resources: &mut PartitionResources,
    ) -> Result<(), PartitionError> {
        boundary.validate()?;

        if partition.boundaries.len()
            >= self.limits.max_boundaries_total
        {
            return Err(PartitionError::ResourceLimitExceeded {
                resource: "boundaries_per_partition",
            });
        }

        if boundary.neighboring_partitions.len()
            > self.limits.max_neighbors_per_boundary
        {
            return Err(PartitionError::ResourceLimitExceeded {
                resource: "boundary_neighbors",
            });
        }

        // Deduplicate boundary coordinates within a partition.
        if let Some(existing) = partition
            .boundaries
            .iter_mut()
            .find(|existing| {
                existing.coordinate == boundary.coordinate
            })
        {
            existing.kind = merge_boundary_kind(
                existing.kind,
                boundary.kind,
            );

            existing
                .neighboring_partitions
                .extend(boundary.neighboring_partitions);

            if existing.neighboring_partitions.len()
                > self.limits.max_neighbors_per_boundary
            {
                return Err(PartitionError::ResourceLimitExceeded {
                    resource: "boundary_neighbors",
                });
            }

            return Ok(());
        }

        resources.boundaries_created =
            resources.boundaries_created.saturating_add(1);

        resources.check_limits(&self.limits)?;

        resources.neighbor_links_created =
            resources
                .neighbor_links_created
                .saturating_add(
                    boundary.neighboring_partitions.len() as u64,
                );

        partition.boundaries.push(boundary);

        Ok(())
    }

    fn build_adjacency(
        &self,
        partitions: &[QecPartition],
    ) -> Result<BTreeMap<PartitionId, BTreeSet<PartitionId>>, PartitionError>
    {
        let mut adjacency: BTreeMap<
            PartitionId,
            BTreeSet<PartitionId>,
        > = BTreeMap::new();

        for partition in partitions {
            adjacency.entry(partition.id).or_default();

            for boundary in &partition.boundaries {
                for neighbor in &boundary.neighboring_partitions {
                    adjacency
                        .entry(partition.id)
                        .or_default()
                        .insert(*neighbor);

                    adjacency
                        .entry(*neighbor)
                        .or_default()
                        .insert(partition.id);
                }
            }
        }

        Ok(adjacency)
    }

    fn check_cancelled<C>(
        &self,
        cancellation: &C,
    ) -> Result<(), PartitionError>
    where
        C: CancellationToken,
    {
        if cancellation.is_cancelled() {
            return Err(PartitionError::CancellationRequested);
        }

        Ok(())
    }

    fn check_time(
        &self,
        started: Instant,
    ) -> Result<(), PartitionError> {
        if let Some(limit) = self.limits.max_execution_time {
            if started.elapsed() > limit {
                return Err(PartitionError::TimeLimitExceeded);
            }
        }

        Ok(())
    }
}

/// Distributed partition execution abstraction.
///
/// The partitioner itself remains backend-independent. A distributed
/// implementation can use this trait to execute already validated partitions.
pub trait PartitionExecutor: Send + Sync {
    type Output;

    fn execute(
        &self,
        partition: &QecPartition,
    ) -> Result<Self::Output, PartitionError>;
}

/// Result of executing all partitions.
#[derive(Debug, Clone)]
pub struct PartitionExecution<O> {
    pub results: BTreeMap<PartitionId, O>,
    pub adjacency: BTreeMap<PartitionId, BTreeSet<PartitionId>>,
    pub requires_boundary_reconciliation: bool,
}

impl<O> PartitionExecution<O> {
    /// Execute deterministically in partition-ID order.
    pub fn execute<E>(
        plan: &PartitionPlan,
        executor: &E,
    ) -> Result<Self, PartitionError>
    where
        E: PartitionExecutor<Output = O>,
    {
        plan.validate()?;

        let mut results = BTreeMap::new();

        for partition in &plan.partitions {
            let result = executor.execute(partition)?;
            results.insert(partition.id, result);
        }

        Ok(Self {
            results,
            adjacency: plan.adjacency.clone(),
            requires_boundary_reconciliation:
                plan.requires_reconciliation,
        })
    }
}

/// Explicit boundary reconciliation input.
///
/// A later decoder/distributed coordinator can consume this structure
/// without needing to reconstruct partition topology.
#[derive(Debug, Clone)]
pub struct BoundaryReconciliation {
    pub partition_a: PartitionId,
    pub partition_b: PartitionId,
    pub boundaries_a: Vec<BoundaryElement>,
    pub boundaries_b: Vec<BoundaryElement>,
}

impl BoundaryReconciliation {
    pub fn validate(&self) -> Result<(), PartitionError> {
        if self.partition_a == self.partition_b {
            return Err(PartitionError::SelfNeighbor {
                partition: self.partition_a,
            });
        }

        if self.boundaries_a.is_empty()
            || self.boundaries_b.is_empty()
        {
            return Err(PartitionError::MissingBoundaryData);
        }

        for boundary in
            self.boundaries_a.iter().chain(self.boundaries_b.iter())
        {
            boundary.validate()?;
        }

        Ok(())
    }
}

/// Construct deterministic reconciliation units.
pub fn reconciliation_units(
    plan: &PartitionPlan,
) -> Result<Vec<BoundaryReconciliation>, PartitionError> {
    plan.validate()?;

    let mut units = Vec::new();

    for (partition_a, neighbors) in &plan.adjacency {
        for partition_b in neighbors {
            if partition_a >= partition_b {
                continue;
            }

            let boundaries_a = plan
                .boundaries
                .get(partition_a)
                .ok_or(PartitionError::UnknownPartition {
                    partition: *partition_a,
                })?
                .iter()
                .filter(|boundary| {
                    boundary
                        .neighboring_partitions
                        .contains(partition_b)
                })
                .cloned()
                .collect::<Vec<_>>();

            let boundaries_b = plan
                .boundaries
                .get(partition_b)
                .ok_or(PartitionError::UnknownPartition {
                    partition: *partition_b,
                })?
                .iter()
                .filter(|boundary| {
                    boundary
                        .neighboring_partitions
                        .contains(partition_a)
                })
                .cloned()
                .collect::<Vec<_>>();

            let unit = BoundaryReconciliation {
                partition_a: *partition_a,
                partition_b: *partition_b,
                boundaries_a,
                boundaries_b,
            };

            unit.validate()?;
            units.push(unit);
        }
    }

    Ok(units)
}

/// Compute an integer axis length safely.
///
/// Inclusive bounds mean min=0,max=0 has length 1.
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
    let x = axis_length(bounds.min.x, bounds.max.x)?;
    let y = axis_length(bounds.min.y, bounds.max.y)?;
    let z = axis_length(bounds.min.z, bounds.max.z)?;

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
        .find(|partition| partition.bounds.contains(coordinate))
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

fn boundary_coordinates(
    a: &QecPartition,
    b: &QecPartition,
) -> Vec<Coordinate> {
    let mut coordinates = BTreeSet::new();

    for item in &a.items {
        if is_boundary_coordinate(
            item.coordinate,
            &b.bounds,
        ) {
            coordinates.insert(item.coordinate);
        }
    }

    for event in &a.events {
        if is_boundary_coordinate(
            event.coordinate,
            &b.bounds,
        ) {
            coordinates.insert(event.coordinate);
        }
    }

    coordinates.into_iter().collect()
}

fn is_boundary_coordinate(
    coordinate: Coordinate,
    other: &Bounds,
) -> bool {
    let adjacent_x =
        coordinate.x.checked_add(1) == Some(other.min.x)
            || coordinate.x.checked_sub(1) == Some(other.max.x);

    let adjacent_y =
        coordinate.y.checked_add(1) == Some(other.min.y)
            || coordinate.y.checked_sub(1) == Some(other.max.y);

    let adjacent_z =
        coordinate.z.checked_add(1) == Some(other.min.z)
            || coordinate.z.checked_sub(1) == Some(other.max.z);

    other.contains(coordinate)
        || adjacent_x && coordinate.y >= other.min.y
            && coordinate.y <= other.max.y
            && coordinate.z >= other.min.z
            && coordinate.z <= other.max.z
        || adjacent_y && coordinate.x >= other.min.x
            && coordinate.x <= other.max.x
            && coordinate.z >= other.min.z
            && coordinate.z <= other.max.z
        || adjacent_z && coordinate.x >= other.min.x
            && coordinate.x <= other.max.x
            && coordinate.y >= other.min.y
            && coordinate.y <= other.max.y
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

fn merge_boundary_kind(
    a: BoundaryKind,
    b: BoundaryKind,
) -> BoundaryKind {
    match (a, b) {
        (BoundaryKind::Mixed, _)
        | (_, BoundaryKind::Mixed) => BoundaryKind::Mixed,

        (BoundaryKind::InterPartition, BoundaryKind::Physical)
        | (BoundaryKind::Physical, BoundaryKind::InterPartition) => {
            BoundaryKind::Mixed
        }

        (BoundaryKind::Internal, other)
        | (other, BoundaryKind::Internal) => other,

        (BoundaryKind::InterPartition, BoundaryKind::InterPartition) => {
            BoundaryKind::InterPartition
        }

        (BoundaryKind::Physical, BoundaryKind::Physical) => {
            BoundaryKind::Physical
        }
    }
}

/// Errors produced by partition planning/execution.
///
/// The error model intentionally remains independent of the concrete
/// `errors.rs` implementation so this file can be introduced without
/// creating cyclic dependencies. `mod.rs` can later map these variants into
/// the global `QecError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartitionError {
    InvalidBounds {
        min: Coordinate,
        max: Coordinate,
    },

    InvalidLimit {
        name: &'static str,
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
        partition_item: QubitId,
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

    MissingBoundaryData,

    NoPartitions,

    UnableToSplit {
        requested: usize,
        achieved: usize,
    },

    DimensionLimitExceeded {
        axis: PartitionAxis,
        length: u64,
    },

    ResourceLimitExceeded {
        resource: &'static str,
    },

    CancellationRequested,

    TimeLimitExceeded,

    ArithmeticOverflow,
}

impl fmt::Display for PartitionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidBounds { .. } => {
                write!(formatter, "invalid partition bounds")
            }

            Self::InvalidLimit { name } => {
                write!(formatter, "invalid partition limit: {name}")
            }

            Self::InvalidPartitionCount => {
                write!(formatter, "partition count must be greater than zero")
            }

            Self::InvalidPhysicalBoundary => {
                write!(formatter, "physical boundary lies outside input bounds")
            }

            Self::InvalidBoundary { reason } => {
                write!(formatter, "invalid boundary: {reason}")
            }

            Self::DuplicatePartition { partition } => {
                write!(formatter, "duplicate partition: {partition}")
            }

            Self::DuplicateQubit { partition, qubit } => {
                write!(
                    formatter,
                    "duplicate qubit {qubit} in partition {partition}"
                )
            }

            Self::DuplicateEvent { partition, event } => {
                write!(
                    formatter,
                    "duplicate event {event} in partition {partition}"
                )
            }

            Self::DuplicateInputQubit { qubit } => {
                write!(formatter, "duplicate input qubit: {qubit}")
            }

            Self::DuplicateInputEvent { event } => {
                write!(formatter, "duplicate input event: {event}")
            }

            Self::DuplicateStabilizer { partition_item } => {
                write!(
                    formatter,
                    "duplicate stabilizer in partition item {partition_item}"
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

            Self::ItemOutsideInputBounds { coordinate } => {
                write!(
                    formatter,
                    "coordinate {coordinate:?} is outside input bounds"
                )
            }

            Self::UnassignedItem { qubit } => {
                write!(formatter, "qubit {qubit} could not be assigned")
            }

            Self::UnassignedEvent { event } => {
                write!(formatter, "event {event} could not be assigned")
            }

            Self::UnknownPartition { partition } => {
                write!(formatter, "unknown partition: {partition}")
            }

            Self::SelfNeighbor { partition } => {
                write!(formatter, "partition {partition} cannot neighbor itself")
            }

            Self::AsymmetricAdjacency { a, b } => {
                write!(
                    formatter,
                    "asymmetric partition adjacency between {a} and {b}"
                )
            }

            Self::MissingBoundaryData => {
                write!(formatter, "missing boundary reconciliation data")
            }

            Self::NoPartitions => {
                write!(formatter, "partition plan contains no partitions")
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

            Self::DimensionLimitExceeded {
                axis,
                length,
            } => {
                write!(
                    formatter,
                    "partition dimension limit exceeded on {axis:?}: {length}"
                )
            }

            Self::ResourceLimitExceeded { resource } => {
                write!(
                    formatter,
                    "partition resource limit exceeded: {resource}"
                )
            }

            Self::CancellationRequested => {
                write!(formatter, "partitioning cancelled")
            }

            Self::TimeLimitExceeded => {
                write!(formatter, "partitioning time limit exceeded")
            }

            Self::ArithmeticOverflow => {
                write!(formatter, "partition arithmetic overflow")
            }
        }
    }
}

impl std::error::Error for PartitionError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn bounds() -> Bounds {
        Bounds::new(
            Coordinate::new(0, 0, 0),
            Coordinate::new(9, 9, 0),
        )
        .unwrap()
    }

    fn item(id: u64, x: i64, y: i64) -> PartitionItem {
        PartitionItem {
            qubit_id: id,
            coordinate: Coordinate::new(x, y, 0),
            stabilizers: vec![id],
        }
    }

    #[test]
    fn splits_longest_axis_deterministically() {
        let partitioner = Partitioner::new(
            PartitionStrategy::LongestAxis,
            PartitionLimits::default(),
        )
        .unwrap();

        let result = partitioner
            .partition(PartitionInput {
                bounds: bounds(),
                items: vec![
                    item(1, 1, 1),
                    item(2, 8, 8),
                ],
                events: Vec::new(),
                physical_boundary: None,
            })
            .unwrap();

        assert_eq!(result.partitions.len(), 2);
        assert!(result.requires_reconciliation);
    }

    #[test]
    fn rejects_duplicate_qubits() {
        let partitioner = Partitioner::new(
            PartitionStrategy::LongestAxis,
            PartitionLimits::default(),
        )
        .unwrap();

        let result = partitioner.partition(PartitionInput {
            bounds: bounds(),
            items: vec![
                item(1, 1, 1),
                item(1, 2, 2),
            ],
            events: Vec::new(),
            physical_boundary: None,
        });

        assert!(matches!(
            result,
            Err(PartitionError::DuplicateInputQubit { .. })
        ));
    }

    #[test]
    fn fixed_count_is_deterministic() {
        let partitioner = Partitioner::new(
            PartitionStrategy::FixedCount { partitions: 4 },
            PartitionLimits::default(),
        )
        .unwrap();

        let input = PartitionInput {
            bounds: Bounds::new(
                Coordinate::new(0, 0, 0),
                Coordinate::new(15, 15, 0),
            )
            .unwrap(),
            items: Vec::new(),
            events: Vec::new(),
            physical_boundary: None,
        };

        let first = partitioner.partition(input.clone()).unwrap();
        let second = partitioner.partition(input).unwrap();

        for (a, b) in first
            .partitions
            .iter()
            .zip(second.partitions.iter())
        {
            assert_eq!(a.bounds, b.bounds);
        }
    }

    #[test]
    fn cancellation_is_respected() {
        struct Cancelled;

        impl CancellationToken for Cancelled {
            fn is_cancelled(&self) -> bool {
                true
            }
        }

        let partitioner = Partitioner::new(
            PartitionStrategy::LongestAxis,
            PartitionLimits::default(),
        )
        .unwrap();

        let result = partitioner.partition_with_cancellation(
            PartitionInput {
                bounds: bounds(),
                items: Vec::new(),
                events: Vec::new(),
                physical_boundary: None,
            },
            &Cancelled,
        );

        assert_eq!(
            result,
            Err(PartitionError::CancellationRequested)
        );
    }

    #[test]
    fn resource_limits_are_enforced() {
        let limits = PartitionLimits {
            max_partitions: 1,
            ..PartitionLimits::default()
        };

        let partitioner = Partitioner::new(
            PartitionStrategy::FixedCount { partitions: 2 },
            limits,
        );

        assert!(partitioner.is_err());
    }

    #[test]
    fn reconciliation_units_are_symmetric() {
        let partitioner = Partitioner::new(
            PartitionStrategy::FixedCount { partitions: 2 },
            PartitionLimits::default(),
        )
        .unwrap();

        let input = PartitionInput {
            bounds: Bounds::new(
                Coordinate::new(0, 0, 0),
                Coordinate::new(9, 0, 0),
            )
            .unwrap(),
            items: vec![
                item(1, 4, 0),
                item(2, 5, 0),
            ],
            events: Vec::new(),
            physical_boundary: None,
        };

        let plan = partitioner.partition(input).unwrap();

        let units = reconciliation_units(&plan).unwrap();

        assert_eq!(units.len(), 1);
        assert_ne!(
            units[0].partition_a,
            units[0].partition_b
        );
    }

    #[test]
    fn malformed_bounds_are_rejected() {
        let result = Bounds::new(
            Coordinate::new(10, 0, 0),
            Coordinate::new(0, 0, 0),
        );

        assert!(matches!(
            result,
            Err(PartitionError::InvalidBounds { .. })
        ));
    }

    #[test]
    fn plan_validation_rejects_asymmetric_adjacency() {
        let bounds = Bounds::new(
            Coordinate::new(0, 0, 0),
            Coordinate::new(0, 0, 0),
        )
        .unwrap();

        let mut adjacency = BTreeMap::new();
        adjacency.insert(0, BTreeSet::from([1]));
        adjacency.insert(1, BTreeSet::new());

        let plan = PartitionPlan {
            partitions: vec![
                QecPartition::new(0, bounds),
                QecPartition::new(1, bounds),
            ],
            adjacency,
            boundaries: BTreeMap::new(),
            resources: PartitionResources::default(),
            requires_reconciliation: true,
        };

        assert!(matches!(
            plan.validate(),
            Err(PartitionError::AsymmetricAdjacency { .. })
        ));
    }
}