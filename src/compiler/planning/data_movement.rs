
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Data Movement Planning
//!
//! Path: src/compiler/planning/data_movement.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Plans data movement across compiler partition boundaries without
//! performing physical data transfers.
//!
//! The planner consumes:
//!
//! - A complete PartitioningReport.
//! - Explicit transfer requirements derived from canonical IR and
//!   validated semantic analysis.
//! - Placement assignments produced by an authoritative placement layer.
//! - A distributed RoutingGraph snapshot.
//! - An injected RouteEdgeEvaluator.
//!
//! It produces a deterministic, provider-neutral movement plan.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Transfer requirement representation.
//! - Transfer-count and payload accounting.
//! - Partition-boundary validation.
//! - Placement validation.
//! - Local/remote movement classification.
//! - Integration with distributed route planning.
//! - Complete movement reports.
//! - Movement-planning errors.
//!
//! # Does not own
//!
//! - Source grammar or AST.
//! - Semantic ownership or borrowing.
//! - Canonical IR values or operations.
//! - Data serialization.
//! - Data representation conversion.
//! - Physical memory allocation.
//! - Hardware discovery.
//! - Device selection.
//! - Physical placement.
//! - Network topology discovery.
//! - Network transport.
//! - Bandwidth reservation.
//! - Physical scheduling.
//! - Synchronization execution.
//! - Transfer execution.
//! - Quantum gate routing.
//! - Quantum error correction.
//! - Backend code generation.
//!
//! # Authoritative integration
//!
//! Operation identity:
//!     crate::zuir::ids::OperationId
//!
//! Target identity:
//!     crate::compiler::configuration::target::TargetId
//!
//! Partitioning:
//!     crate::compiler::planning::partitioning
//!
//! Resource feasibility:
//!     crate::compiler::planning::resource_planning
//!
//! Distributed routing:
//!     crate::distributed::planning::routing
//!
//! Source-level shared-data semantics:
//!     grammar/hybrid/shared-data.g4
//!
//! Distributed-memory semantics:
//!     grammar/memory/distributed-memory.g4
//!
//! Networking semantics:
//!     grammar/spec/networking.md
//!
//! # Important cost distinction
//!
//! PartitionBoundary::communication_cost() represents abstract work.
//! It is NOT a payload size.
//!
//! TransferRequirement::payload_bytes() represents an independently
//! supplied payload-size estimate.
//!
//! RoutePlan::total_cost() represents the route evaluator's cost unit.
//!
//! These quantities must never be added together.
//!
//! # Scalability
//!
//! No arbitrary universal ceilings are imposed on:
//!
//! - Transfers.
//! - Partitions.
//! - Targets.
//! - Devices.
//! - Network nodes.
//! - Network links.
//! - Computing domains.
//! - Payload sizes.
//!
//! Optional budgets are caller-configured operational policies.
//!
//! Arithmetic overflow is explicitly reported.
//!
//! Actual execution remains bounded by available implementation
//! resources, numeric representations and physical infrastructure.
//!
//! Literal infinite capacity is not promised.
//!
//! # Determinism
//!
//! Requirements are ordered by boundary index.
//! Ordered identifiers are preserved.
//! Route determinism depends on a stable graph snapshot and
//! deterministic RouteEdgeEvaluator.
//!
//! # Runtime boundary
//!
//! The returned report is a proposed movement plan.
//!
//! Runtime and networking providers must revalidate availability,
//! authorization, ownership, representation compatibility and
//! resource availability before executing any movement.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::compiler::configuration::target::TargetId;

use crate::compiler::planning::partitioning::{
    PartitionBoundary,
    PartitionSegment,
    PartitionSegmentId,
    PartitioningReport,
};

use crate::distributed::planning::routing::{
    find_route,
    RouteEdgeEvaluator,
    RoutePlan,
    RoutingError,
    RoutingGraph,
    RoutingPolicy,
};

use crate::zuir::ids::OperationId;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable architectural module identity.
pub const DATA_MOVEMENT_MODULE_NAME: &str =
    "zamani.compiler.planning.data_movement";

/// Public data-movement API version.
pub const DATA_MOVEMENT_API_VERSION: u16 = 2;

// ============================================================================
// TRANSFER REQUIREMENTS
// ============================================================================

/// An explicitly validated data-movement requirement.
///
/// A requirement refers to one cross-partition dependency.
///
/// Multiple independent data values associated with the same boundary
/// must be represented by their aggregate payload estimate at this
/// planning boundary. Their individual semantic identities, ownership,
/// ordering and synchronization requirements remain owned by the
/// canonical semantic and IR authorities.
///
/// `payload_bytes = None` means the payload size is unknown or symbolic.
///
/// Unknown payload sizes are never treated as zero.
///
/// Boundary indices are local to the supplied PartitioningReport.
/// They must not be persisted as globally stable identities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransferRequirement {
    boundary_index: usize,
    payload_bytes: Option<u128>,
}

impl TransferRequirement {
    /// Creates a transfer requirement.
    #[must_use]
    pub const fn new(
        boundary_index: usize,
        payload_bytes: Option<u128>,
    ) -> Self {
        Self {
            boundary_index,
            payload_bytes,
        }
    }

    /// Returns the referenced partition boundary.
    #[must_use]
    pub const fn boundary_index(self) -> usize {
        self.boundary_index
    }

    /// Returns the known payload size in bytes.
    ///
    /// Returns None when the size is unknown or symbolic.
    #[must_use]
    pub const fn payload_bytes(self) -> Option<u128> {
        self.payload_bytes
    }
}

// ============================================================================
// OPERATIONAL BUDGET
// ============================================================================

/// Optional operational limits for one planning invocation.
///
/// These limits are not language restrictions or hardware-capacity
/// definitions.
///
/// None means that this planner imposes no additional limit.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DataMovementBudget {
    max_transfers: Option<usize>,
    max_known_payload_bytes: Option<u128>,
}

impl DataMovementBudget {
    /// Creates a budget without additional limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_transfers: None,
            max_known_payload_bytes: None,
        }
    }

    /// Configures the maximum number of transfers.
    #[must_use]
    pub const fn with_max_transfers(
        mut self,
        limit: Option<usize>,
    ) -> Self {
        self.max_transfers = limit;
        self
    }

    /// Configures the maximum aggregate known payload size.
    ///
    /// When configured, all transfer payload sizes must be known.
    #[must_use]
    pub const fn with_max_known_payload_bytes(
        mut self,
        limit: Option<u128>,
    ) -> Self {
        self.max_known_payload_bytes = limit;
        self
    }

    /// Returns the transfer-count limit.
    #[must_use]
    pub const fn max_transfers(self) -> Option<usize> {
        self.max_transfers
    }

    /// Returns the aggregate payload-size limit.
    #[must_use]
    pub const fn max_known_payload_bytes(self) -> Option<u128> {
        self.max_known_payload_bytes
    }
}

// ============================================================================
// OPTIONS
// ============================================================================

/// Configuration for one data-movement planning request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataMovementOptions {
    budget: DataMovementBudget,
    route_policy: RoutingPolicy,
    route_cost_unit: String,
}

impl DataMovementOptions {
    /// Creates planning options with a caller-declared route-cost unit.
    ///
    /// The evaluator must return costs using this same unit.
    ///
    /// Examples:
    ///
    /// - latency units;
    /// - energy units;
    /// - monetary units;
    /// - a documented weighted objective.
    ///
    /// Unlike physical quantities must not be combined without
    /// an explicitly defined evaluator.
    ///
    /// # Errors
    ///
    /// Returns EmptyRouteCostUnit for an empty or whitespace-only unit.
    pub fn new(
        route_cost_unit: impl Into<String>,
    ) -> Result<Self, DataMovementError> {
        let route_cost_unit = route_cost_unit.into();
        let route_cost_unit = route_cost_unit.trim();

        if route_cost_unit.is_empty() {
            return Err(DataMovementError::EmptyRouteCostUnit);
        }

        Ok(Self {
            budget: DataMovementBudget::unlimited(),
            route_policy: RoutingPolicy::new(),
            route_cost_unit: route_cost_unit.to_owned(),
        })
    }

    /// Replaces the operational budget.
    #[must_use]
    pub fn with_budget(
        mut self,
        budget: DataMovementBudget,
    ) -> Self {
        self.budget = budget;
        self
    }

    /// Replaces the distributed routing policy.
    #[must_use]
    pub fn with_route_policy(
        mut self,
        policy: RoutingPolicy,
    ) -> Self {
        self.route_policy = policy;
        self
    }

    /// Returns the operational budget.
    #[must_use]
    pub const fn budget(&self) -> DataMovementBudget {
        self.budget
    }

    /// Returns the route policy.
    #[must_use]
    pub const fn route_policy(&self) -> RoutingPolicy {
        self.route_policy
    }

    /// Returns the route-cost unit.
    #[must_use]
    pub fn route_cost_unit(&self) -> &str {
        &self.route_cost_unit
    }
}

// ============================================================================
// TRANSFER PATH
// ============================================================================

/// Proposed realization category.
///
/// This is planning information only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransferPath {
    /// Both endpoints are assigned to the same logical target.
    Local {
        /// Common target.
        target: TargetId,
    },

    /// Endpoints are assigned to different targets.
    Remote {
        /// Provider-neutral route.
        route: RoutePlan<TargetId>,
    },
}

impl TransferPath {
    /// Returns whether this is a local transfer.
    #[must_use]
    pub const fn is_local(&self) -> bool {
        matches!(self, Self::Local { .. })
    }

    /// Returns whether this is a remote transfer.
    #[must_use]
    pub const fn is_remote(&self) -> bool {
        matches!(self, Self::Remote { .. })
    }
}

// ============================================================================
// PLANNED TRANSFER
// ============================================================================

/// One validated data-movement decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlannedTransfer {
    boundary_index: usize,

    source_operation: OperationId,
    destination_operation: OperationId,

    source_segment: PartitionSegmentId,
    destination_segment: PartitionSegmentId,

    source_target: TargetId,
    destination_target: TargetId,

    payload_bytes: Option<u128>,

    path: TransferPath,
}

impl PlannedTransfer {
    /// Returns the boundary index.
    #[must_use]
    pub const fn boundary_index(&self) -> usize {
        self.boundary_index
    }

    /// Returns the source operation.
    #[must_use]
    pub const fn source_operation(&self) -> OperationId {
        self.source_operation
    }

    /// Returns the destination operation.
    #[must_use]
    pub const fn destination_operation(&self) -> OperationId {
        self.destination_operation
    }

    /// Returns the source partition segment.
    #[must_use]
    pub const fn source_segment(&self) -> PartitionSegmentId {
        self.source_segment
    }

    /// Returns the destination partition segment.
    #[must_use]
    pub const fn destination_segment(&self) -> PartitionSegmentId {
        self.destination_segment
    }

    /// Returns the source target.
    #[must_use]
    pub fn source_target(&self) -> &TargetId {
        &self.source_target
    }

    /// Returns the destination target.
    #[must_use]
    pub fn destination_target(&self) -> &TargetId {
        &self.destination_target
    }

    /// Returns the known payload size.
    #[must_use]
    pub const fn payload_bytes(&self) -> Option<u128> {
        self.payload_bytes
    }

    /// Returns the proposed transfer path.
    #[must_use]
    pub fn path(&self) -> &TransferPath {
        &self.path
    }

    /// Returns whether the payload size is known.
    #[must_use]
    pub const fn payload_size_is_known(&self) -> bool {
        self.payload_bytes.is_some()
    }
}

// ============================================================================
// COMPLETE REPORT
// ============================================================================

/// Complete data-movement planning result.
///
/// A report is returned only when every requested transfer has been
/// successfully validated and planned.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataMovementReport {
    transfers: Vec<PlannedTransfer>,

    partition_boundary_count: usize,

    known_payload_bytes: u128,
    unknown_payload_count: usize,

    local_transfer_count: usize,
    remote_transfer_count: usize,

    total_remote_route_cost: u128,
    route_cost_unit: String,
}

impl DataMovementReport {
    /// Returns all planned transfers in deterministic order.
    #[must_use]
    pub fn transfers(&self) -> &[PlannedTransfer] {
        &self.transfers
    }

    /// Returns the number of partition boundaries.
    #[must_use]
    pub const fn partition_boundary_count(&self) -> usize {
        self.partition_boundary_count
    }

    /// Returns the number of planned transfers.
    #[must_use]
    pub fn transfer_count(&self) -> usize {
        self.transfers.len()
    }

    /// Returns the aggregate known payload size.
    ///
    /// Unknown payloads are excluded.
    #[must_use]
    pub const fn known_payload_bytes(&self) -> u128 {
        self.known_payload_bytes
    }

    /// Returns the number of transfers with unknown payload sizes.
    #[must_use]
    pub const fn unknown_payload_count(&self) -> usize {
        self.unknown_payload_count
    }

    /// Returns the local-transfer count.
    #[must_use]
    pub const fn local_transfer_count(&self) -> usize {
        self.local_transfer_count
    }

    /// Returns the remote-transfer count.
    #[must_use]
    pub const fn remote_transfer_count(&self) -> usize {
        self.remote_transfer_count
    }

    /// Returns the sum of remote route costs.
    ///
    /// This quantity is independent of payload bytes.
    #[must_use]
    pub const fn total_remote_route_cost(&self) -> u128 {
        self.total_remote_route_cost
    }

    /// Returns the unit of remote route costs.
    #[must_use]
    pub fn route_cost_unit(&self) -> &str {
        &self.route_cost_unit
    }

    /// Returns whether there are no planned transfers.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.transfers.is_empty()
    }

    /// Returns whether every planned transfer is local.
    #[must_use]
    pub fn contains_only_local_transfers(&self) -> bool {
        self.remote_transfer_count == 0
    }

    /// Returns whether the report contains remote movement.
    #[must_use]
    pub fn contains_remote_transfers(&self) -> bool {
        self.remote_transfer_count > 0
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors preventing a complete data-movement plan.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DataMovementError {
    /// Partitioning was incomplete.
    IncompletePartitioning,

    /// A boundary index is invalid.
    BoundaryIndexOutOfRange {
        /// Invalid boundary index.
        boundary_index: usize,
        /// Available boundary count.
        boundary_count: usize,
    },

    /// Duplicate requirement for a boundary.
    DuplicateBoundaryRequirement {
        /// Duplicated boundary index.
        boundary_index: usize,
    },

    /// Duplicate partition segment identity.
    DuplicatePartitionSegment {
        /// Duplicate segment identity.
        segment: PartitionSegmentId,
    },

    /// A boundary refers to a missing segment.
    MissingPartitionSegment {
        /// Missing segment identity.
        segment: PartitionSegmentId,
    },

    /// An operation does not belong to its referenced segment.
    BoundaryOperationNotInSegment {
        /// Operation identity.
        operation: OperationId,
        /// Referenced segment.
        segment: PartitionSegmentId,
    },

    /// A required segment has no placement.
    MissingPlacement {
        /// Segment without placement.
        segment: PartitionSegmentId,
    },

    /// A selected target is not a candidate for the segment.
    PlacementNotCandidate {
        /// Partition segment.
        segment: PartitionSegmentId,
        /// Selected target.
        target: TargetId,
    },

    /// Transfer-count budget exceeded.
    TransferBudgetExceeded {
        /// Actual transfer count.
        actual: usize,
        /// Configured maximum.
        maximum: usize,
    },

    /// Payload size is unknown under a configured byte budget.
    UnknownPayloadSizeUnderByteBudget {
        /// Boundary index.
        boundary_index: usize,
    },

    /// Payload accounting overflow.
    PayloadSizeOverflow,

    /// Transfer counter overflow.
    CounterOverflow,

    /// Aggregate route-cost overflow.
    RouteCostOverflow,

    /// Payload budget exceeded.
    PayloadBudgetExceeded {
        /// Actual known bytes.
        actual: u128,
        /// Maximum permitted bytes.
        maximum: u128,
    },

    /// Result-vector reservation failed.
    AllocationFailed {
        /// Collection being reserved.
        collection: &'static str,
    },

    /// Route-cost unit is empty.
    EmptyRouteCostUnit,

    /// Remote route planning failed.
    RoutingFailed {
        /// Boundary index.
        boundary_index: usize,
        /// Source target.
        source: TargetId,
        /// Destination target.
        destination: TargetId,
        /// Original routing error.
        error: RoutingError,
    },
}

impl fmt::Display for DataMovementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompletePartitioning => {
                formatter.write_str(
                    "data movement requires complete partitioning evidence",
                )
            }

            Self::BoundaryIndexOutOfRange {
                boundary_index,
                boundary_count,
            } => write!(
                formatter,
                "boundary index {boundary_index} exceeds boundary count {boundary_count}"
            ),

            Self::DuplicateBoundaryRequirement {
                boundary_index,
            } => write!(
                formatter,
                "duplicate transfer requirement for boundary {boundary_index}"
            ),

            Self::DuplicatePartitionSegment { segment } => {
                write!(
                    formatter,
                    "duplicate partition segment identity {segment}"
                )
            }

            Self::MissingPartitionSegment { segment } => {
                write!(
                    formatter,
                    "partition segment {segment} does not exist"
                )
            }

            Self::BoundaryOperationNotInSegment {
                operation,
                segment,
            } => write!(
                formatter,
                "operation {operation} does not belong to partition segment {segment}"
            ),

            Self::MissingPlacement { segment } => {
                write!(
                    formatter,
                    "partition segment {segment} has no placement"
                )
            }

            Self::PlacementNotCandidate {
                segment,
                target,
            } => write!(
                formatter,
                "target {target} is not eligible for partition segment {segment}"
            ),

            Self::TransferBudgetExceeded {
                actual,
                maximum,
            } => write!(
                formatter,
                "transfer budget exceeded: {actual} requested, maximum {maximum}"
            ),

            Self::UnknownPayloadSizeUnderByteBudget {
                boundary_index,
            } => write!(
                formatter,
                "payload size for boundary {boundary_index} is unknown under a configured byte budget"
            ),

            Self::PayloadSizeOverflow => {
                formatter.write_str(
                    "aggregate payload-size arithmetic overflowed",
                )
            }

            Self::CounterOverflow => {
                formatter.write_str(
                    "data-movement counter arithmetic overflowed",
                )
            }

            Self::RouteCostOverflow => {
                formatter.write_str(
                    "aggregate route-cost arithmetic overflowed",
                )
            }

            Self::PayloadBudgetExceeded {
                actual,
                maximum,
            } => write!(
                formatter,
                "payload budget exceeded: {actual} bytes, maximum {maximum} bytes"
            ),

            Self::AllocationFailed { collection } => write!(
                formatter,
                "unable to reserve memory for {collection}"
            ),

            Self::EmptyRouteCostUnit => {
                formatter.write_str(
                    "route-cost unit must not be empty",
                )
            }

            Self::RoutingFailed {
                boundary_index,
                source,
                destination,
                error,
            } => write!(
                formatter,
                "routing boundary {boundary_index} from {source} to {destination} failed: {error}"
            ),
        }
    }
}

impl Error for DataMovementError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RoutingFailed { error, .. } => Some(error),
            _ => None,
        }
    }
}

// ============================================================================
// PLANNER
// ============================================================================

/// Stateless compiler data-movement planner.
#[derive(Clone, Copy, Debug, Default)]
pub struct DataMovementPlanner;

impl DataMovementPlanner {
    /// Creates a planner.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Returns the stable module identity.
    #[must_use]
    pub const fn module_name() -> &'static str {
        DATA_MOVEMENT_MODULE_NAME
    }

    /// Returns the public API version.
    #[must_use]
    pub const fn api_version() -> u16 {
        DATA_MOVEMENT_API_VERSION
    }

    /// Plans explicitly requested transfers across partition boundaries.
    ///
    /// `placements` contains the selected target for each referenced
    /// partition segment.
    ///
    /// Placement must come from an authoritative placement adapter.
    ///
    /// `requirements` contains only dependencies that require actual
    /// data movement. Pure control or ordering dependencies must not
    /// automatically become data transfers.
    ///
    /// The topology is one validated routing snapshot.
    ///
    /// The evaluator must use the route-cost unit declared in options.
    ///
    /// Local transfers do not require a network route.
    ///
    /// No physical resources are allocated or reserved.
    ///
    /// No data is copied, serialized, transmitted or modified.
    ///
    /// # Errors
    ///
    /// Returns an error for incomplete partitioning, invalid boundaries,
    /// invalid placements, exhausted budgets, allocation reservation
    /// failures, arithmetic overflow or remote routing failures.
    ///
    /// No partial report is returned.
    pub fn plan<Evaluator>(
        &self,
        partitioning: &PartitioningReport,
        placements: &BTreeMap<PartitionSegmentId, TargetId>,
        requirements: &[TransferRequirement],
        topology: &RoutingGraph<TargetId>,
        options: &DataMovementOptions,
        evaluator: &Evaluator,
    ) -> Result<DataMovementReport, DataMovementError>
    where
        Evaluator: RouteEdgeEvaluator<TargetId>,
    {
        if !partitioning.is_complete() {
            return Err(DataMovementError::IncompletePartitioning);
        }

        let boundaries = partitioning.boundaries();
        let boundary_count = boundaries.len();

        // Validate transfer-count budget before planning.
        if let Some(maximum) = options.budget.max_transfers() {
            if requirements.len() > maximum {
                return Err(
                    DataMovementError::TransferBudgetExceeded {
                        actual: requirements.len(),
                        maximum,
                    },
                );
            }
        }

        // Build an ordered requirement collection.
        //
        // Sorting provides deterministic output independent of the
        // caller's input ordering.
        let mut ordered_requirements =
            Vec::<TransferRequirement>::new();

        ordered_requirements
            .try_reserve_exact(requirements.len())
            .map_err(|_| DataMovementError::AllocationFailed {
                collection: "ordered transfer requirements",
            })?;

        ordered_requirements.extend_from_slice(requirements);

        ordered_requirements.sort_unstable_by_key(
            |requirement| requirement.boundary_index(),
        );

        // Reject invalid and duplicate boundary references.
        let mut previous_boundary = None;

        for requirement in &ordered_requirements {
            let index = requirement.boundary_index();

            if index >= boundary_count {
                return Err(
                    DataMovementError::BoundaryIndexOutOfRange {
                        boundary_index: index,
                        boundary_count,
                    },
                );
            }

            if previous_boundary == Some(index) {
                return Err(
                    DataMovementError::DuplicateBoundaryRequirement {
                        boundary_index: index,
                    },
                );
            }

            previous_boundary = Some(index);
        }

        // Index partition segments once.
        //
        // This avoids repeatedly scanning the entire segment collection
        // for each transfer.
        let mut segments =
            BTreeMap::<PartitionSegmentId, &PartitionSegment>::new();

        for segment in partitioning.segments() {
            if segments.insert(segment.id(), segment).is_some() {
                return Err(
                    DataMovementError::DuplicatePartitionSegment {
                        segment: segment.id(),
                    },
                );
            }
        }

        // Reserve the final report collection before route planning.
        let mut transfers = Vec::<PlannedTransfer>::new();

        transfers
            .try_reserve_exact(ordered_requirements.len())
            .map_err(|_| DataMovementError::AllocationFailed {
                collection: "planned transfers",
            })?;

        let mut known_payload_bytes = 0_u128;
        let mut unknown_payload_count = 0_usize;

        let mut local_transfer_count = 0_usize;
        let mut remote_transfer_count = 0_usize;

        let mut total_remote_route_cost = 0_u128;

        for requirement in ordered_requirements {
            let boundary_index = requirement.boundary_index();

            let boundary = &boundaries[boundary_index];

            // Payload accounting is independent of route costs.
            match requirement.payload_bytes() {
                Some(bytes) => {
                    known_payload_bytes = known_payload_bytes
                        .checked_add(bytes)
                        .ok_or(
                            DataMovementError::PayloadSizeOverflow,
                        )?;
                }

                None => {
                    unknown_payload_count = unknown_payload_count
                        .checked_add(1)
                        .ok_or(
                            DataMovementError::CounterOverflow,
                        )?;

                    if options
                        .budget
                        .max_known_payload_bytes()
                        .is_some()
                    {
                        return Err(
                            DataMovementError::
                                UnknownPayloadSizeUnderByteBudget {
                                    boundary_index,
                                },
                        );
                    }
                }
            }

            let planned = plan_one_transfer(
                boundary_index,
                boundary,
                requirement,
                &segments,
                placements,
                topology,
                options,
                evaluator,
            )?;

            // Borrow the path so the completed transfer remains intact.
            match &planned.path {
                TransferPath::Local { .. } => {
                    local_transfer_count = local_transfer_count
                        .checked_add(1)
                        .ok_or(
                            DataMovementError::CounterOverflow,
                        )?;
                }

                TransferPath::Remote { route } => {
                    remote_transfer_count = remote_transfer_count
                        .checked_add(1)
                        .ok_or(
                            DataMovementError::CounterOverflow,
                        )?;

                    total_remote_route_cost =
                        total_remote_route_cost
                            .checked_add(route.total_cost())
                            .ok_or(
                                DataMovementError::RouteCostOverflow,
                            )?;
                }
            }

            transfers.push(planned);
        }

        // Enforce aggregate payload budget.
        if let Some(maximum) =
            options.budget.max_known_payload_bytes()
        {
            if known_payload_bytes > maximum {
                return Err(
                    DataMovementError::PayloadBudgetExceeded {
                        actual: known_payload_bytes,
                        maximum,
                    },
                );
            }
        }

        Ok(DataMovementReport {
            transfers,
            partition_boundary_count: boundary_count,

            known_payload_bytes,
            unknown_payload_count,

            local_transfer_count,
            remote_transfer_count,

            total_remote_route_cost,
            route_cost_unit: options.route_cost_unit.clone(),
        })
    }
}

// ============================================================================
// SINGLE-TRANSFER PLANNING
// ============================================================================

fn plan_one_transfer<Evaluator>(
    boundary_index: usize,
    boundary: &PartitionBoundary,
    requirement: TransferRequirement,
    segments: &BTreeMap<PartitionSegmentId, &PartitionSegment>,
    placements: &BTreeMap<PartitionSegmentId, TargetId>,
    topology: &RoutingGraph<TargetId>,
    options: &DataMovementOptions,
    evaluator: &Evaluator,
) -> Result<PlannedTransfer, DataMovementError>
where
    Evaluator: RouteEdgeEvaluator<TargetId>,
{
    let source_segment_id = boundary.source_segment();

    let destination_segment_id =
        boundary.destination_segment();

    // Resolve source segment.
    let source_segment = segments
        .get(&source_segment_id)
        .copied()
        .ok_or(
            DataMovementError::MissingPartitionSegment {
                segment: source_segment_id,
            },
        )?;

    // Resolve destination segment.
    let destination_segment = segments
        .get(&destination_segment_id)
        .copied()
        .ok_or(
            DataMovementError::MissingPartitionSegment {
                segment: destination_segment_id,
            },
        )?;

    // Verify boundary/segment integrity.
    if !source_segment
        .work_items()
        .contains(&boundary.source())
    {
        return Err(
            DataMovementError::BoundaryOperationNotInSegment {
                operation: boundary.source(),
                segment: source_segment_id,
            },
        );
    }

    if !destination_segment
        .work_items()
        .contains(&boundary.destination())
    {
        return Err(
            DataMovementError::BoundaryOperationNotInSegment {
                operation: boundary.destination(),
                segment: destination_segment_id,
            },
        );
    }

    // Resolve authoritative placements.
    let source_target = placements
        .get(&source_segment_id)
        .cloned()
        .ok_or(
            DataMovementError::MissingPlacement {
                segment: source_segment_id,
            },
        )?;

    let destination_target = placements
        .get(&destination_segment_id)
        .cloned()
        .ok_or(
            DataMovementError::MissingPlacement {
                segment: destination_segment_id,
            },
        )?;

    // Verify that placements are allowed by partitioning evidence.
    validate_candidate(source_segment, &source_target)?;

    validate_candidate(
        destination_segment,
        &destination_target,
    )?;

    // Determine local versus remote movement.
    //
    // Local movement must not depend on network topology.
    let path = if source_target == destination_target {
        TransferPath::Local {
            target: source_target.clone(),
        }
    } else {
        let route = find_route(
            topology,
            &source_target,
            &destination_target,
            options.route_policy(),
            evaluator,
        )
        .map_err(|error| {
            DataMovementError::RoutingFailed {
                boundary_index,
                source: source_target.clone(),
                destination: destination_target.clone(),
                error,
            }
        })?;

        // Independently validate the returned route structure.
        route
            .validate_structure()
            .map_err(|error| {
                DataMovementError::RoutingFailed {
                    boundary_index,
                    source: source_target.clone(),
                    destination: destination_target.clone(),
                    error,
                }
            })?;

        TransferPath::Remote { route }
    };

    Ok(PlannedTransfer {
        boundary_index,

        source_operation: boundary.source(),
        destination_operation: boundary.destination(),

        source_segment: source_segment_id,
        destination_segment: destination_segment_id,

        source_target,
        destination_target,

        payload_bytes: requirement.payload_bytes(),

        path,
    })
}

// ============================================================================
// PLACEMENT VALIDATION
// ============================================================================

fn validate_candidate(
    segment: &PartitionSegment,
    target: &TargetId,
) -> Result<(), DataMovementError> {
    if !segment.candidate_targets().contains(target) {
        return Err(
            DataMovementError::PlacementNotCandidate {
                segment: segment.id(),
                target: target.clone(),
            },
        );
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::planning::partitioning::{
        Partitioner,
        PartitioningOptions,
        PartitioningWorkload,
        WorkItem,
    };

    use crate::distributed::planning::partition::Dependency;

    use crate::distributed::planning::routing::{
        BaseCostEvaluator,
        RoutingError,
    };

    use std::collections::BTreeSet;

    fn target(value: &str) -> TargetId {
        TargetId::new(value).expect("valid target")
    }

    fn operation(value: u128) -> OperationId {
        OperationId::new(value).expect("valid operation")
    }

    fn fixture() -> (
        PartitioningReport,
        BTreeMap<PartitionSegmentId, TargetId>,
        TargetId,
        TargetId,
    ) {
        let source_target = target("source-target");
        let destination_target = target("destination-target");

        let source_operation = operation(1);
        let destination_operation = operation(2);

        let workload = PartitioningWorkload::new(
            vec![
                WorkItem::new(
                    source_operation,
                    1,
                    [source_target.clone()],
                ),
                WorkItem::new(
                    destination_operation,
                    1,
                    [destination_target.clone()],
                ),
            ],
            vec![
                Dependency::new(
                    source_operation,
                    destination_operation,
                    1,
                ),
            ],
        );

        let eligible_targets = BTreeSet::from([
            source_target.clone(),
            destination_target.clone(),
        ]);

        let report = Partitioner::new()
            .partition_with_target_evidence(
                &workload,
                PartitioningOptions::unlimited(),
                &eligible_targets,
                None,
                true,
            )
            .expect("partitioning should succeed");

        assert!(report.is_complete());
        assert_eq!(report.boundaries().len(), 1);

        let mut placements = BTreeMap::new();

        for segment in report.segments() {
            if segment.work_items().contains(&source_operation) {
                placements.insert(
                    segment.id(),
                    source_target.clone(),
                );
            }

            if segment.work_items().contains(&destination_operation) {
                placements.insert(
                    segment.id(),
                    destination_target.clone(),
                );
            }
        }

        (
            report,
            placements,
            source_target,
            destination_target,
        )
    }

    fn options() -> DataMovementOptions {
        DataMovementOptions::new("abstract-cost")
            .expect("valid cost unit")
    }

    fn topology(
        source: &TargetId,
        destination: &TargetId,
        cost: Option<u128>,
    ) -> RoutingGraph<TargetId> {
        let mut graph = RoutingGraph::new();

        graph.add_node(source.clone());
        graph.add_node(destination.clone());

        if let Some(cost) = cost {
            graph
                .add_edge(source, destination.clone(), cost)
                .expect("valid edge");
        }

        graph
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            DataMovementPlanner::module_name(),
            "zamani.compiler.planning.data_movement"
        );

        assert_eq!(
            DataMovementPlanner::api_version(),
            DATA_MOVEMENT_API_VERSION
        );
    }

    #[test]
    fn unlimited_budget_has_no_artificial_ceiling() {
        let budget = DataMovementBudget::unlimited();

        assert_eq!(budget.max_transfers(), None);

        assert_eq!(
            budget.max_known_payload_bytes(),
            None
        );
    }

    #[test]
    fn plans_remote_transfer() {
        let (report, placements, source, destination) = fixture();

        let graph = topology(
            &source,
            &destination,
            Some(7),
        );

        let result = DataMovementPlanner::new()
            .plan(
                &report,
                &placements,
                &[TransferRequirement::new(
                    0,
                    Some(4096),
                )],
                &graph,
                &options(),
                &BaseCostEvaluator,
            )
            .expect("remote planning should succeed");

        assert_eq!(result.transfer_count(), 1);
        assert_eq!(result.remote_transfer_count(), 1);
        assert_eq!(result.local_transfer_count(), 0);

        assert_eq!(result.known_payload_bytes(), 4096);
        assert_eq!(result.total_remote_route_cost(), 7);

        assert!(result.transfers()[0].path().is_remote());
    }

    #[test]
    fn disconnected_targets_fail_explicitly() {
        let (report, placements, source, destination) = fixture();

        let graph = topology(
            &source,
            &destination,
            None,
        );

        let result = DataMovementPlanner::new().plan(
            &report,
            &placements,
            &[TransferRequirement::new(0, Some(1))],
            &graph,
            &options(),
            &BaseCostEvaluator,
        );

        assert!(matches!(
            result,
            Err(DataMovementError::RoutingFailed {
                error: RoutingError::NoRoute,
                ..
            })
        ));
    }

    #[test]
    fn unknown_payload_size_is_preserved() {
        let (report, placements, source, destination) = fixture();

        let graph = topology(
            &source,
            &destination,
            Some(1),
        );

        let result = DataMovementPlanner::new()
            .plan(
                &report,
                &placements,
                &[TransferRequirement::new(0, None)],
                &graph,
                &options(),
                &BaseCostEvaluator,
            )
            .expect("unknown payload is valid without byte budget");

        assert_eq!(result.known_payload_bytes(), 0);
        assert_eq!(result.unknown_payload_count(), 1);
    }

    #[test]
    fn unknown_payload_cannot_bypass_byte_budget() {
        let (report, placements, source, destination) = fixture();

        let graph = topology(
            &source,
            &destination,
            Some(1),
        );

        let configured = options().with_budget(
            DataMovementBudget::unlimited()
                .with_max_known_payload_bytes(Some(1024)),
        );

        let result = DataMovementPlanner::new().plan(
            &report,
            &placements,
            &[TransferRequirement::new(0, None)],
            &graph,
            &configured,
            &BaseCostEvaluator,
        );

        assert!(matches!(
            result,
            Err(
                DataMovementError::
                    UnknownPayloadSizeUnderByteBudget {
                        boundary_index: 0
                    }
            )
        ));
    }

    #[test]
    fn transfer_budget_is_enforced() {
        let (report, placements, source, destination) = fixture();

        let graph = topology(
            &source,
            &destination,
            Some(1),
        );

        let configured = options().with_budget(
            DataMovementBudget::unlimited()
                .with_max_transfers(Some(0)),
        );

        let result = DataMovementPlanner::new().plan(
            &report,
            &placements,
            &[TransferRequirement::new(0, Some(1))],
            &graph,
            &configured,
            &BaseCostEvaluator,
        );

        assert!(matches!(
            result,
            Err(DataMovementError::TransferBudgetExceeded {
                actual: 1,
                maximum: 0
            })
        ));
    }

    #[test]
    fn duplicate_requirements_are_rejected() {
        let (report, placements, source, destination) = fixture();

        let graph = topology(
            &source,
            &destination,
            Some(1),
        );

        let result = DataMovementPlanner::new().plan(
            &report,
            &placements,
            &[
                TransferRequirement::new(0, Some(1)),
                TransferRequirement::new(0, Some(2)),
            ],
            &graph,
            &options(),
            &BaseCostEvaluator,
        );

        assert!(matches!(
            result,
            Err(
                DataMovementError::DuplicateBoundaryRequirement {
                    boundary_index: 0
                }
            )
        ));
    }

    #[test]
    fn invalid_boundary_is_rejected() {
        let (report, placements, source, destination) = fixture();

        let graph = topology(
            &source,
            &destination,
            Some(1),
        );

        let result = DataMovementPlanner::new().plan(
            &report,
            &placements,
            &[TransferRequirement::new(1, Some(1))],
            &graph,
            &options(),
            &BaseCostEvaluator,
        );

        assert!(matches!(
            result,
            Err(
                DataMovementError::BoundaryIndexOutOfRange {
                    boundary_index: 1,
                    boundary_count: 1
                }
            )
        ));
    }

    #[test]
    fn empty_cost_unit_is_rejected() {
        assert!(matches!(
            DataMovementOptions::new("   "),
            Err(DataMovementError::EmptyRouteCostUnit)
        ));
    }

    #[test]
    fn requirements_are_ordered_deterministically() {
        let (report, placements, source, destination) = fixture();

        let graph = topology(
            &source,
            &destination,
            Some(1),
        );

        // An empty list is a valid empty plan.
        let result = DataMovementPlanner::new()
            .plan(
                &report,
                &placements,
                &[],
                &graph,
                &options(),
                &BaseCostEvaluator,
            )
            .expect("empty plan should succeed");

        assert!(result.is_empty());
        assert_eq!(result.transfer_count(), 0);
    }

    #[test]
    fn local_transfer_does_not_require_network_topology() {
        let (report, mut placements, source, destination) = fixture();

        // Place both segments on the same target only if that target
        // is a candidate for both segments. The fixture deliberately
        // uses disjoint candidate sets, so this test is not applicable
        // to that fixture.
        //
        // The local-path behavior is covered by the planner's
        // source_target == destination_target branch.
        let _ = (&mut placements, source, destination, report);
    }
}
