
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Data Movement Planning
//!
//! Path: `src/compiler/planning/data_movement.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Produce deterministic, target-independent data-movement plans for
//! explicitly identified dependencies crossing compiler partition
//! boundaries.
//!
//! The planner consumes:
//!
//! - A complete compiler PartitioningReport.
//! - A placement mapping from compiler partition segments to TargetIds.
//! - Explicit data-transfer requirements derived from canonical IR and
//!   semantic analysis.
//! - A validated distributed RoutingGraph.
//! - An injected RouteEdgeEvaluator.
//!
//! It produces:
//!
//! - Local movement decisions.
//! - Remote movement decisions with provider-neutral route plans.
//! - Payload-size accounting where sizes are known.
//! - Explicit failures for incomplete or infeasible plans.
//!
//! ## AUTHORITATIVE OWNERS
//!
//! Canonical operation identity:
//!     crate::zuir::ids::OperationId
//!
//! Canonical target identity:
//!     crate::compiler::configuration::target::TargetId
//!
//! Logical workload partitioning:
//!     crate::compiler::planning::partitioning
//!
//! Physical placement:
//!     compiler placement adapter / distributed placement
//!
//! Communication route search:
//!     crate::distributed::planning::routing
//!
//! Networking semantics:
//!     grammar/spec/networking.md
//!
//! Shared-data semantics:
//!     grammar/hybrid/shared-data.g4
//!
//! Distributed-memory semantics:
//!     grammar/memory/distributed-memory.g4
//!
//! Canonical computation representations:
//!     crate::zuir
//!     crate::classical
//!     crate::quantum::ir
//!     crate::hdl::hdlir
//!     crate::ai
//!     crate::hybrid
//!     crate::distributed
//!
//! ## OWNS
//!
//! - Compiler-level data-transfer planning.
//! - Mapping explicit transfer requirements to partition boundaries.
//! - Placement validation for transfer endpoints.
//! - Candidate-target validation.
//! - Local-versus-remote movement classification.
//! - Route planning through the existing distributed routing engine.
//! - Transfer-count and payload-size budgets.
//! - Deterministic transfer reports.
//!
//! ## DOES NOT OWN
//!
//! - Source grammar or AST.
//! - Semantic data ownership.
//! - Canonical IR values or operations.
//! - Data serialization formats.
//! - Data representation conversion.
//! - Physical memory allocation.
//! - Device selection.
//! - Physical placement algorithms.
//! - Network topology discovery.
//! - Network transport.
//! - Bandwidth reservation.
//! - Transfer execution.
//! - Scheduling or synchronization.
//! - Quantum gate routing.
//! - Quantum error correction.
//! - Noise modeling.
//! - Backend code generation.
//!
//! ## IMPORTANT COST RULE
//!
//! PartitionBoundary::communication_cost() is an abstract WorkEstimate.
//! This module never interprets it as bytes.
//!
//! Payload sizes are supplied separately through TransferRequirement.
//!
//! Route costs are supplied by RouteEdgeEvaluator and are measured in the
//! caller-declared route-cost unit. Route costs are not added to payload
//! sizes or partition communication estimates.
//!
//! ## SCALABILITY
//!
//! There are no universal maximums for:
//!
//! - Transfer count.
//! - Payload size.
//! - Partition count.
//! - Targets.
//! - Devices.
//! - Network nodes.
//! - Network links.
//! - Computing domains.
//! - Hardware types.
//!
//! Optional budgets are caller-controlled operational policies.
//! Arithmetic overflow is reported explicitly.
//!
//! Literal infinite execution capacity is not promised. Actual scale is
//! bounded by available memory, address space, numeric representation,
//! processing time and caller-supplied budgets.
//!
//! ## DETERMINISM
//!
//! Transfers are processed in ascending boundary-index order.
//! Partition and target identities use their canonical ordered types.
//! Reproducible route selection additionally requires a deterministic
//! RouteEdgeEvaluator and a fixed routing graph snapshot.
//!
//! ## INTEGRATION
//!
//! The caller must derive TransferRequirement values from canonical IR
//! data dependencies and semantic analysis.
//!
//! Boundary indices refer to the exact PartitioningReport supplied to
//! this invocation. They must not be persisted as cross-compilation IDs.
//!
//! Placement assignments must come from the compiler's placement adapter.
//! This module does not choose targets.
//!
//! The result is an abstract plan. The runtime and networking layers must
//! revalidate resource availability and perform the actual transfer.
//!
//! ## FREEZE CRITERIA
//!
//! - Incomplete partitioning is rejected.
//! - Invalid boundary references are rejected.
//! - Duplicate transfer requirements are rejected.
//! - Missing placement is rejected.
//! - Incompatible target placement is rejected.
//! - Local transfers do not require a network route.
//! - Remote transfers use the existing routing engine.
//! - No partial report is returned after a failure.
//! - Configured budgets are enforced.
//! - Unknown payload sizes are not fabricated.
//! - No physical transfer is executed.
//! - No unsafe Rust is used.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
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

/// Stable architectural identity of this module.
pub const DATA_MOVEMENT_MODULE_NAME: &str =
    "zamani.compiler.planning.data_movement";

/// Public API version of this module.
pub const DATA_MOVEMENT_API_VERSION: u16 = 1;

// ============================================================================
// TRANSFER REQUIREMENTS
// ============================================================================

/// One explicitly identified data-transfer requirement.
///
/// `boundary_index` identifies an entry in the supplied
/// PartitioningReport::boundaries() collection.
///
/// A requirement represents the aggregate payload for that boundary.
/// If the semantic analysis identifies independent transfers with different
/// correctness or synchronization requirements, those must remain separate
/// in the canonical semantic/IR model and must not be merged merely to
/// simplify this planning interface.
///
/// `payload_bytes` is optional because the compiler may not know the
/// concrete size at planning time. Unknown size is never treated as zero.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TransferRequirement {
    boundary_index: usize,
    payload_bytes: Option<u128>,
}

impl TransferRequirement {
    /// Creates a transfer requirement.
    ///
    /// `payload_bytes = None` means that the payload size is unknown or
    /// symbolic at this planning stage.
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

    /// Returns the index of the corresponding partition boundary.
    #[must_use]
    pub const fn boundary_index(self) -> usize {
        self.boundary_index
    }

    /// Returns the known payload size in bytes, if available.
    #[must_use]
    pub const fn payload_bytes(self) -> Option<u128> {
        self.payload_bytes
    }
}

// ============================================================================
// OPERATIONAL BUDGET
// ============================================================================

/// Caller-configurable operational limits for one data-movement plan.
///
/// These budgets are not language limits or hardware-capacity definitions.
///
/// `None` means that this planner adds no limit for that dimension.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DataMovementBudget {
    max_transfers: Option<usize>,
    max_known_payload_bytes: Option<u128>,
}

impl DataMovementBudget {
    /// Creates a budget without additional planner-level limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_transfers: None,
            max_known_payload_bytes: None,
        }
    }

    /// Sets the optional maximum number of planned transfers.
    #[must_use]
    pub const fn with_max_transfers(
        mut self,
        limit: Option<usize>,
    ) -> Self {
        self.max_transfers = limit;
        self
    }

    /// Sets the optional maximum aggregate known payload size.
    ///
    /// When this budget is configured, every transfer must have a known
    /// payload size. An unknown size cannot be proven to fit the budget.
    #[must_use]
    pub const fn with_max_known_payload_bytes(
        mut self,
        limit: Option<u128>,
    ) -> Self {
        self.max_known_payload_bytes = limit;
        self
    }

    /// Returns the configured transfer-count limit.
    #[must_use]
    pub const fn max_transfers(self) -> Option<usize> {
        self.max_transfers
    }

    /// Returns the configured known-payload limit.
    #[must_use]
    pub const fn max_known_payload_bytes(self) -> Option<u128> {
        self.max_known_payload_bytes
    }
}

impl Default for DataMovementBudget {
    fn default() -> Self {
        Self::unlimited()
    }
}

// ============================================================================
// PLANNING OPTIONS
// ============================================================================

/// Options for one data-movement planning invocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataMovementOptions {
    budget: DataMovementBudget,
    route_policy: RoutingPolicy,
    route_cost_unit: String,
}

impl DataMovementOptions {
    /// Creates options using a caller-declared route-cost unit.
    ///
    /// Examples of possible units include latency units, energy units,
    /// monetary units or a documented weighted objective.
    ///
    /// The evaluator must return costs consistently using this unit.
    ///
    /// # Errors
    ///
    /// Returns DataMovementError::EmptyRouteCostUnit when the unit is empty
    /// or consists only of whitespace.
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

    /// Replaces the data-movement operational budget.
    #[must_use]
    pub fn with_budget(
        mut self,
        budget: DataMovementBudget,
    ) -> Self {
        self.budget = budget;
        self
    }

    /// Replaces the policy used by the existing route planner.
    #[must_use]
    pub fn with_route_policy(
        mut self,
        policy: RoutingPolicy,
    ) -> Self {
        self.route_policy = policy;
        self
    }

    /// Returns the data-movement budget.
    #[must_use]
    pub const fn budget(&self) -> DataMovementBudget {
        self.budget
    }

    /// Returns the route policy.
    #[must_use]
    pub const fn route_policy(&self) -> RoutingPolicy {
        self.route_policy
    }

    /// Returns the declared unit of route costs.
    #[must_use]
    pub fn route_cost_unit(&self) -> &str {
        &self.route_cost_unit
    }
}

// ============================================================================
// TRANSFER PATH
// ============================================================================

/// Physical-realization category proposed by data-movement planning.
///
/// This is a planning classification only. Neither variant performs
/// a transfer or reserves physical resources.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransferPath {
    /// Both partition segments are placed on the same logical target.
    ///
    /// The runtime may realize this using local memory, a local reference,
    /// a shared-memory mechanism or another valid implementation.
    Local {
        /// Target hosting both partition segments.
        target: TargetId,
    },

    /// The partition segments are placed on different targets and a
    /// route has been found through the supplied routing graph.
    Remote {
        /// Provider-neutral route through the supplied graph snapshot.
        route: RoutePlan<TargetId>,
    },
}

impl TransferPath {
    /// Returns whether the planned transfer is local.
    #[must_use]
    pub const fn is_local(&self) -> bool {
        matches!(self, Self::Local { .. })
    }

    /// Returns whether the planned transfer is remote.
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
    /// Returns the original partition-boundary index.
    #[must_use]
    pub const fn boundary_index(&self) -> usize {
        self.boundary_index
    }

    /// Returns the source operation identity.
    #[must_use]
    pub const fn source_operation(&self) -> OperationId {
        self.source_operation
    }

    /// Returns the destination operation identity.
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

    /// Returns the known payload size, if supplied.
    #[must_use]
    pub const fn payload_bytes(&self) -> Option<u128> {
        self.payload_bytes
    }

    /// Returns the proposed local or remote transfer path.
    #[must_use]
    pub fn path(&self) -> &TransferPath {
        &self.path
    }
}

// ============================================================================
// COMPLETE REPORT
// ============================================================================

/// Complete data-movement planning report.
///
/// The report contains no partial success state. The planner returns this
/// structure only after every requested transfer has been validated and
/// planned successfully.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataMovementReport {
    transfers: Vec<PlannedTransfer>,
    partition_boundary_count: usize,
    known_payload_bytes: u128,
    unknown_payload_count: usize,
    local_transfer_count: usize,
    remote_transfer_count: usize,
    route_cost_unit: String,
}

impl DataMovementReport {
    /// Returns all planned transfers in deterministic boundary order.
    #[must_use]
    pub fn transfers(&self) -> &[PlannedTransfer] {
        &self.transfers
    }

    /// Returns the number of partition boundaries in the input report.
    #[must_use]
    pub const fn partition_boundary_count(&self) -> usize {
        self.partition_boundary_count
    }

    /// Returns the number of planned transfers.
    #[must_use]
    pub fn transfer_count(&self) -> usize {
        self.transfers.len()
    }

    /// Returns the sum of known payload sizes.
    ///
    /// Transfers with unknown sizes are not included in this sum.
    #[must_use]
    pub const fn known_payload_bytes(&self) -> u128 {
        self.known_payload_bytes
    }

    /// Returns the number of transfers whose payload size is unknown.
    #[must_use]
    pub const fn unknown_payload_count(&self) -> usize {
        self.unknown_payload_count
    }

    /// Returns the number of local transfers.
    #[must_use]
    pub const fn local_transfer_count(&self) -> usize {
        self.local_transfer_count
    }

    /// Returns the number of remote transfers.
    #[must_use]
    pub const fn remote_transfer_count(&self) -> usize {
        self.remote_transfer_count
    }

    /// Returns the declared unit used by remote route costs.
    #[must_use]
    pub fn route_cost_unit(&self) -> &str {
        &self.route_cost_unit
    }

    /// Returns whether the report contains no requested transfers.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.transfers.is_empty()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Failures that prevent a complete data-movement plan.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DataMovementError {
    /// Upstream partitioning was incomplete or infeasible.
    IncompletePartitioning,

    /// A transfer refers to a boundary index outside the report.
    BoundaryIndexOutOfRange {
        /// Invalid boundary index.
        boundary_index: usize,
        /// Number of available boundaries.
        boundary_count: usize,
    },

    /// More than one transfer requirement refers to the same boundary.
    DuplicateBoundaryRequirement {
        /// Duplicated boundary index.
        boundary_index: usize,
    },

    /// The boundary refers to a segment missing from the partition report.
    MissingPartitionSegment {
        /// Missing segment.
        segment: PartitionSegmentId,
    },

    /// No target assignment exists for a required partition segment.
    MissingPlacement {
        /// Segment without a placement.
        segment: PartitionSegmentId,
    },

    /// A selected target is not among the segment's eligible candidates.
    PlacementNotCandidate {
        /// Partition segment.
        segment: PartitionSegmentId,
        /// Selected target.
        target: TargetId,
    },

    /// The configured transfer-count budget was exceeded.
    TransferBudgetExceeded {
        /// Requested transfer count.
        actual: usize,
        /// Configured maximum.
        maximum: usize,
    },

    /// The configured payload budget cannot be checked because a transfer
    /// has an unknown payload size.
    UnknownPayloadSizeUnderByteBudget {
        /// Boundary whose payload size is unknown.
        boundary_index: usize,
    },

    /// Checked payload-size accumulation overflowed u128.
    PayloadSizeOverflow,

    /// The known payload total exceeded the configured budget.
    PayloadBudgetExceeded {
        /// Aggregate known payload bytes.
        actual: u128,
        /// Configured maximum.
        maximum: u128,
    },

    /// The caller supplied an empty route-cost unit.
    EmptyRouteCostUnit,

    /// The existing route planner could not find or validate a route.
    RoutingFailed {
        /// Boundary for which route planning failed.
        boundary_index: usize,
        /// Source target.
        source: TargetId,
        /// Destination target.
        destination: TargetId,
        /// Original route-planning error.
        error: RoutingError,
    },
}

impl fmt::Display for DataMovementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompletePartitioning => {
                formatter.write_str(
                    "data movement requires a complete partitioning report",
                )
            }
            Self::BoundaryIndexOutOfRange {
                boundary_index,
                boundary_count,
            } => write!(
                formatter,
                "transfer boundary index {boundary_index} is outside \
                 the available boundary count {boundary_count}"
            ),
            Self::DuplicateBoundaryRequirement { boundary_index } => {
                write!(
                    formatter,
                    "duplicate transfer requirement for boundary {boundary_index}"
                )
            }
            Self::MissingPartitionSegment { segment } => {
                write!(formatter, "partition segment {segment} was not found")
            }
            Self::MissingPlacement { segment } => {
                write!(
                    formatter,
                    "partition segment {segment} has no target placement"
                )
            }
            Self::PlacementNotCandidate { segment, target } => {
                write!(
                    formatter,
                    "target {target} is not an eligible candidate for segment {segment}"
                )
            }
            Self::TransferBudgetExceeded { actual, maximum } => {
                write!(
                    formatter,
                    "transfer budget exceeded: requested {actual}, maximum {maximum}"
                )
            }
            Self::UnknownPayloadSizeUnderByteBudget {
                boundary_index,
            } => write!(
                formatter,
                "payload size for boundary {boundary_index} is unknown \
                 while a byte budget is configured"
            ),
            Self::PayloadSizeOverflow => {
                formatter.write_str("aggregate payload-size arithmetic overflowed")
            }
            Self::PayloadBudgetExceeded { actual, maximum } => {
                write!(
                    formatter,
                    "payload budget exceeded: {actual} bytes, maximum {maximum} bytes"
                )
            }
            Self::EmptyRouteCostUnit => {
                formatter.write_str("route-cost unit must not be empty")
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
    /// Creates a data-movement planner.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Plans explicitly requested transfers across partition boundaries.
    ///
    /// `placements` maps compiler partition-segment IDs to the targets
    /// selected by the upstream placement authority.
    ///
    /// `requirements` contains only boundaries that represent actual
    /// data movement. A dependency that expresses ordering or control
    /// dependence without moving data must not be included.
    ///
    /// The supplied routing graph must represent one validated topology
    /// snapshot. The evaluator must use the route-cost unit declared in
    /// `options`.
    ///
    /// Local transfers do not require the routing graph to contain the
    /// local target.
    ///
    /// # Errors
    ///
    /// Returns an error if partitioning is incomplete, a boundary or
    /// placement is invalid, a budget is exceeded, or remote route planning
    /// fails. No partial report is returned.
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

        let boundary_count = partitioning.boundaries().len();

        if let Some(maximum) = options.budget.max_transfers {
            if requirements.len() > maximum {
                return Err(DataMovementError::TransferBudgetExceeded {
                    actual: requirements.len(),
                    maximum,
                });
            }
        }

        // Validate references and establish deterministic input order.
        let mut ordered_requirements =
            BTreeMap::<usize, TransferRequirement>::new();

        for requirement in requirements {
            let index = requirement.boundary_index;

            if index >= boundary_count {
                return Err(DataMovementError::BoundaryIndexOutOfRange {
                    boundary_index: index,
                    boundary_count,
                });
            }

            if ordered_requirements
                .insert(index, *requirement)
                .is_some()
            {
                return Err(
                    DataMovementError::DuplicateBoundaryRequirement {
                        boundary_index: index,
                    },
                );
            }
        }

        // Build a segment index once rather than searching the segment
        // collection separately for every transfer.
        let mut segments =
            BTreeMap::<PartitionSegmentId, &PartitionSegment>::new();

        for segment in partitioning.segments() {
            if segments.insert(segment.id(), segment).is_some() {
                return Err(DataMovementError::MissingPartitionSegment {
                    segment: segment.id(),
                });
            }
        }

        let mut transfers = Vec::with_capacity(ordered_requirements.len());

        let mut known_payload_bytes = 0_u128;
        let mut unknown_payload_count = 0_usize;
        let mut local_transfer_count = 0_usize;
        let mut remote_transfer_count = 0_usize;

        for (boundary_index, requirement) in ordered_requirements {
            let boundary = &partitioning.boundaries()[boundary_index];

            if let Some(payload_bytes) = requirement.payload_bytes {
                known_payload_bytes = known_payload_bytes
                    .checked_add(payload_bytes)
                    .ok_or(DataMovementError::PayloadSizeOverflow)?;
            } else {
                unknown_payload_count = unknown_payload_count
                    .checked_add(1)
                    .ok_or(DataMovementError::PayloadSizeOverflow)?;

                if options.budget.max_known_payload_bytes.is_some() {
                    return Err(
                        DataMovementError::UnknownPayloadSizeUnderByteBudget {
                            boundary_index,
                        },
                    );
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

            match planned.path {
                TransferPath::Local { .. } => {
                    local_transfer_count = local_transfer_count
                        .checked_add(1)
                        .ok_or(DataMovementError::PayloadSizeOverflow)?;
                }
                TransferPath::Remote { .. } => {
                    remote_transfer_count = remote_transfer_count
                        .checked_add(1)
                        .ok_or(DataMovementError::PayloadSizeOverflow)?;
                }
            }

            transfers.push(planned);
        }

        if let Some(maximum) = options.budget.max_known_payload_bytes {
            if known_payload_bytes > maximum {
                return Err(DataMovementError::PayloadBudgetExceeded {
                    actual: known_payload_bytes,
                    maximum,
                });
            }
        }

        Ok(DataMovementReport {
            transfers,
            partition_boundary_count: boundary_count,
            known_payload_bytes,
            unknown_payload_count,
            local_transfer_count,
            remote_transfer_count,
            route_cost_unit: options.route_cost_unit.clone(),
        })
    }
}

// ============================================================================
// SINGLE-TRANSFER VALIDATION AND PLANNING
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
    let destination_segment_id = boundary.destination_segment();

    let source_segment = segments
        .get(&source_segment_id)
        .copied()
        .ok_or(DataMovementError::MissingPartitionSegment {
            segment: source_segment_id,
        })?;

    let destination_segment = segments
        .get(&destination_segment_id)
        .copied()
        .ok_or(DataMovementError::MissingPartitionSegment {
            segment: destination_segment_id,
        })?;

    let source_target = placements
        .get(&source_segment_id)
        .cloned()
        .ok_or(DataMovementError::MissingPlacement {
            segment: source_segment_id,
        })?;

    let destination_target = placements
        .get(&destination_segment_id)
        .cloned()
        .ok_or(DataMovementError::MissingPlacement {
            segment: destination_segment_id,
        })?;

    validate_candidate(source_segment, &source_target)?;
    validate_candidate(destination_segment, &destination_target)?;

    let path = if source_target == destination_target {
        TransferPath::Local {
            target: source_target.clone(),
        }
    } else {
        let route = find_route(
            topology,
            &source_target,
            &destination_target,
            options.route_policy,
            evaluator,
        )
        .map_err(|error| DataMovementError::RoutingFailed {
            boundary_index,
            source: source_target.clone(),
            destination: destination_target.clone(),
            error,
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
        payload_bytes: requirement.payload_bytes,
        path,
    })
}

fn validate_candidate(
    segment: &PartitionSegment,
    target: &TargetId,
) -> Result<(), DataMovementError> {
    if !segment.candidate_targets().contains(target) {
        return Err(DataMovementError::PlacementNotCandidate {
            segment: segment.id(),
            target: target.clone(),
        });
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

    use crate::distributed::planning::routing::BaseCostEvaluator;

    use std::collections::BTreeSet;

    fn target(value: &str) -> TargetId {
        TargetId::new(value).expect("valid target identifier")
    }

    fn operation(value: u128) -> OperationId {
        OperationId::new(value).expect("nonzero operation identifier")
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
            .expect("valid partitioning input");

        assert!(report.is_complete());
        assert_eq!(report.boundaries().len(), 1);

        let mut placements = BTreeMap::new();

        for segment in report.segments() {
            if segment.work_items().contains(&source_operation) {
                placements.insert(
                    segment.id(),
                    source_target.clone(),
                );
            } else if segment.work_items().contains(&destination_operation) {
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
            .expect("non-empty route cost unit")
    }

    #[test]
    fn plans_a_remote_transfer_using_existing_route_engine() {
        let (report, placements, source, destination) = fixture();

        let mut topology = RoutingGraph::new();
        topology.add_node(source.clone());
        topology.add_node(destination.clone());
        topology
            .add_edge(&source, destination.clone(), 7)
            .expect("valid route edge");

        let result = DataMovementPlanner::new()
            .plan(
                &report,
                &placements,
                &[TransferRequirement::new(0, Some(4096))],
                &topology,
                &options(),
                &BaseCostEvaluator,
            )
            .expect("remote route should be available");

        assert_eq!(result.transfer_count(), 1);
        assert_eq!(result.remote_transfer_count(), 1);
        assert_eq!(result.local_transfer_count(), 0);
        assert_eq!(result.known_payload_bytes(), 4096);

        let transfer = &result.transfers()[0];

        assert!(transfer.path().is_remote());

        match transfer.path() {
            TransferPath::Remote { route } => {
                assert_eq!(route.total_cost(), 7);
                assert_eq!(
                    route.nodes(),
                    &[source, destination],
                );
            }
            TransferPath::Local { .. } => {
                panic!("expected a remote route");
            }
        }
    }

    #[test]
    fn disconnected_targets_produce_explicit_failure() {
        let (report, placements, source, destination) = fixture();

        let mut topology = RoutingGraph::new();
        topology.add_node(source.clone());
        topology.add_node(destination.clone());

        let result = DataMovementPlanner::new().plan(
            &report,
            &placements,
            &[TransferRequirement::new(0, Some(1))],
            &topology,
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

        let mut topology = RoutingGraph::new();
        topology.add_node(source.clone());
        topology.add_node(destination.clone());
        topology
            .add_edge(&source, destination, 1)
            .expect("valid edge");

        let result = DataMovementPlanner::new()
            .plan(
                &report,
                &placements,
                &[TransferRequirement::new(0, None)],
                &topology,
                &options(),
                &BaseCostEvaluator,
            )
            .expect("unknown size is allowed without byte budget");

        assert_eq!(result.known_payload_bytes(), 0);
        assert_eq!(result.unknown_payload_count(), 1);
    }

    #[test]
    fn unknown_payload_size_cannot_bypass_a_byte_budget() {
        let (report, placements, source, destination) = fixture();

        let mut topology = RoutingGraph::new();
        topology.add_node(source.clone());
        topology.add_node(destination.clone());
        topology
            .add_edge(&source, destination, 1)
            .expect("valid edge");

        let configured = options().with_budget(
            DataMovementBudget::unlimited()
                .with_max_known_payload_bytes(Some(1024)),
        );

        let result = DataMovementPlanner::new().plan(
            &report,
            &placements,
            &[TransferRequirement::new(0, None)],
            &topology,
            &configured,
            &BaseCostEvaluator,
        );

        assert!(matches!(
            result,
            Err(
                DataMovementError::UnknownPayloadSizeUnderByteBudget {
                    boundary_index: 0
                }
            )
        ));
    }

    #[test]
    fn transfer_count_budget_is_enforced() {
        let (report, placements, source, destination) = fixture();

        let mut topology = RoutingGraph::new();
        topology.add_node(source.clone());
        topology.add_node(destination.clone());
        topology
            .add_edge(&source, destination, 1)
            .expect("valid edge");

        let configured = options().with_budget(
            DataMovementBudget::unlimited()
                .with_max_transfers(Some(0)),
        );

        let result = DataMovementPlanner::new().plan(
            &report,
            &placements,
            &[TransferRequirement::new(0, Some(1))],
            &topology,
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
    fn duplicate_boundary_requirements_are_rejected() {
        let (report, placements, source, destination) = fixture();

        let mut topology = RoutingGraph::new();
        topology.add_node(source.clone());
        topology.add_node(destination.clone());
        topology
            .add_edge(&source, destination, 1)
            .expect("valid edge");

        let result = DataMovementPlanner::new().plan(
            &report,
            &placements,
            &[
                TransferRequirement::new(0, Some(1)),
                TransferRequirement::new(0, Some(2)),
            ],
            &topology,
            &options(),
            &BaseCostEvaluator,
        );

        assert!(matches!(
            result,
            Err(DataMovementError::DuplicateBoundaryRequirement {
                boundary_index: 0
            })
        ));
    }

    #[test]
    fn invalid_boundary_index_is_rejected() {
        let (report, placements, source, destination) = fixture();

        let mut topology = RoutingGraph::new();
        topology.add_node(source);
        topology.add_node(destination);

        let result = DataMovementPlanner::new().plan(
            &report,
            &placements,
            &[TransferRequirement::new(1, Some(1))],
            &topology,
            &options(),
            &BaseCostEvaluator,
        );

        assert!(matches!(
            result,
            Err(DataMovementError::BoundaryIndexOutOfRange {
                boundary_index: 1,
                boundary_count: 1
            })
        ));
    }

    #[test]
    fn empty_route_cost_unit_is_rejected() {
        assert!(matches!(
            DataMovementOptions::new("   "),
            Err(DataMovementError::EmptyRouteCostUnit)
        ));
    }
}
