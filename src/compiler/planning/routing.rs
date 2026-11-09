
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Heterogeneous Routing
//!
//! Path: src/compiler/planning/routing.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Coordinates routing across compiler partition boundaries by consuming:
//!
//! - A complete compiler partitioning report.
//! - Authoritative placement assignments.
//! - A validated distributed routing graph snapshot.
//! - An explicit routing policy.
//! - An optional provider-owned edge evaluator.
//!
//! It produces deterministic, validated, abstract route plans.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Compiler partition-boundary validation.
//! - Placement-to-routing integration.
//! - Compiler-level route result association.
//! - Route aggregation and accounting.
//! - Compiler routing diagnostics.
//!
//! It does not own:
//!
//! - Source grammar or semantic analysis.
//! - AST or canonical IR.
//! - Workload partitioning.
//! - Physical placement.
//! - Hardware or topology discovery.
//! - Network transport.
//! - Physical resource allocation.
//! - Execution scheduling.
//! - Quantum physical routing or QEC.
//! - Backend code generation.
//! - Runtime execution.
//!
//! # Authoritative integrations
//!
//! Partitioning:
//!     crate::compiler::planning::partitioning
//!
//! Target identity:
//!     crate::compiler::configuration::target
//!
//! Distributed route calculation:
//!     crate::distributed::planning::routing
//!
//! Data movement:
//!     crate::compiler::planning::data_movement
//!
//! Scheduling:
//!     crate::compiler::planning::scheduling
//!
//! Semantic intent:
//!     crate::frontend::semantic
//!
//! Networking syntax and semantics:
//!     grammar/networking/routing.g4
//!     grammar/spec/networking.md
//!
//! # Cost semantics
//!
//! Route costs and abstract communication-work estimates are separate
//! quantities. They must never be added together unless a higher-level
//! policy explicitly defines compatible units.
//!
//! # Scalability
//!
//! No arbitrary universal limits are imposed on targets, routes,
//! partitions, dependencies, graph nodes, links, or hops.
//!
//! Optional budgets are caller-configured operational policies.
//! Actual memory, address-space, numeric and physical resource limits
//! remain finite.
//!
//! # Determinism
//!
//! Boundary processing is ordered by stable logical identities.
//! Route-search determinism additionally depends on a stable graph
//! snapshot and a deterministic edge evaluator.
//!
//! # Runtime boundary
//!
//! A route is a proposal over a supplied topology snapshot.
//! It does not reserve bandwidth, establish a connection, authenticate
//! peers, or guarantee that the route remains available.
//!
//! Runtime and networking providers must revalidate before execution.

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
    BaseCostEvaluator,
    RouteEdgeEvaluator,
    RoutePlan,
    RoutingError,
    RoutingGraph,
    RoutingPolicy,
};

use crate::zuir::ids::OperationId;

/// Stable architectural identity of this module.
pub const COMPILER_ROUTING_MODULE_NAME: &str =
    "zamani.compiler.planning.routing";

/// Public API version of this module.
pub const COMPILER_ROUTING_API_VERSION: u16 = 2;

/// Cost type supplied by the distributed routing authority.
///
/// The unit is caller-defined and must be consistent within a report.
pub type CompilerRoutingCost = u128;

// ============================================================================
// BUDGETS AND OPTIONS
// ============================================================================

/// Optional operational limits for compiler-level routing.
///
/// These are compilation policies, not language-level or hardware limits.
/// A missing limit means this module imposes no additional restriction.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CompilerRoutingBudget {
    max_boundary_routes: Option<usize>,
}

impl CompilerRoutingBudget {
    /// Creates a budget without additional boundary-count restrictions.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_boundary_routes: None,
        }
    }

    /// Sets an optional maximum number of boundary routes.
    #[must_use]
    pub const fn with_max_boundary_routes(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_boundary_routes = maximum;
        self
    }

    /// Returns the optional maximum boundary-route count.
    #[must_use]
    pub const fn max_boundary_routes(self) -> Option<usize> {
        self.max_boundary_routes
    }
}

/// Configuration for one compiler routing invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompilerRoutingOptions {
    path_policy: RoutingPolicy,
    budget: CompilerRoutingBudget,
}

impl CompilerRoutingOptions {
    /// Creates compiler routing options.
    #[must_use]
    pub const fn new(
        path_policy: RoutingPolicy,
        budget: CompilerRoutingBudget,
    ) -> Self {
        Self {
            path_policy,
            budget,
        }
    }

    /// Creates options without caller-imposed compiler routing limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            path_policy: RoutingPolicy::new(),
            budget: CompilerRoutingBudget::unlimited(),
        }
    }

    /// Replaces the distributed path-search policy.
    #[must_use]
    pub const fn with_path_policy(
        mut self,
        policy: RoutingPolicy,
    ) -> Self {
        self.path_policy = policy;
        self
    }

    /// Replaces the compiler routing budget.
    #[must_use]
    pub const fn with_budget(
        mut self,
        budget: CompilerRoutingBudget,
    ) -> Self {
        self.budget = budget;
        self
    }

    /// Returns the distributed routing policy.
    #[must_use]
    pub const fn path_policy(self) -> RoutingPolicy {
        self.path_policy
    }

    /// Returns the compiler routing budget.
    #[must_use]
    pub const fn budget(self) -> CompilerRoutingBudget {
        self.budget
    }
}

impl Default for CompilerRoutingOptions {
    fn default() -> Self {
        Self::unlimited()
    }
}

// ============================================================================
// PLACEMENT EVIDENCE
// ============================================================================

/// Mapping from logical partition segments to selected target identities.
///
/// The mapping must originate from the authoritative placement subsystem.
/// This module does not choose targets.
pub type PlacementAssignments =
    BTreeMap<PartitionSegmentId, TargetId>;

// ============================================================================
// ROUTED BOUNDARY
// ============================================================================

/// A successfully routed compiler partition boundary.
///
/// This is an abstract route proposal, not a physical reservation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundaryRoute {
    source_operation: OperationId,
    destination_operation: OperationId,

    source_segment: PartitionSegmentId,
    destination_segment: PartitionSegmentId,

    source_target: TargetId,
    destination_target: TargetId,

    route: RoutePlan<TargetId>,

    communication_work_estimate: u128,
}

impl BoundaryRoute {
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

    /// Returns the selected source target.
    #[must_use]
    pub fn source_target(&self) -> &TargetId {
        &self.source_target
    }

    /// Returns the selected destination target.
    #[must_use]
    pub fn destination_target(&self) -> &TargetId {
        &self.destination_target
    }

    /// Returns the underlying distributed route plan.
    #[must_use]
    pub fn route(&self) -> &RoutePlan<TargetId> {
        &self.route
    }

    /// Returns the original abstract communication-work estimate.
    ///
    /// This estimate is not automatically added to route cost.
    #[must_use]
    pub const fn communication_work_estimate(&self) -> u128 {
        self.communication_work_estimate
    }

    /// Returns the route's total edge cost.
    #[must_use]
    pub const fn route_cost(&self) -> CompilerRoutingCost {
        self.route.total_cost()
    }

    /// Returns the number of communication hops.
    #[must_use]
    pub fn hop_count(&self) -> usize {
        self.route.hops()
    }

    /// Returns the ordered target path.
    #[must_use]
    pub fn path(&self) -> &[TargetId] {
        self.route.nodes()
    }
}

// ============================================================================
// REPORT
// ============================================================================

/// Complete compiler-level routing report.
///
/// A report is returned only when every supplied boundary has a
/// validated route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilerRoutingReport {
    routes: Vec<BoundaryRoute>,

    /// Expected count from the original partitioning report.
    partition_boundary_count: usize,

    total_route_cost: u128,
    total_hops: usize,
    total_communication_work_estimate: u128,
}

impl CompilerRoutingReport {
    fn new(
        routes: Vec<BoundaryRoute>,
        partition_boundary_count: usize,
    ) -> Result<Self, CompilerRoutingError> {
        let mut total_route_cost = 0_u128;
        let mut total_hops = 0_usize;
        let mut total_communication_work_estimate = 0_u128;

        for route in &routes {
            total_route_cost = total_route_cost
                .checked_add(route.route_cost())
                .ok_or(CompilerRoutingError::ArithmeticOverflow)?;

            total_hops = total_hops
                .checked_add(route.hop_count())
                .ok_or(CompilerRoutingError::ArithmeticOverflow)?;

            total_communication_work_estimate =
                total_communication_work_estimate
                    .checked_add(route.communication_work_estimate())
                    .ok_or(CompilerRoutingError::ArithmeticOverflow)?;
        }

        Ok(Self {
            routes,
            partition_boundary_count,
            total_route_cost,
            total_hops,
            total_communication_work_estimate,
        })
    }

    /// Returns all successfully routed partition boundaries.
    #[must_use]
    pub fn routes(&self) -> &[BoundaryRoute] {
        &self.routes
    }

    /// Returns the original partition-boundary count.
    #[must_use]
    pub const fn boundary_count(&self) -> usize {
        self.partition_boundary_count
    }

    /// Returns the number of successfully routed boundaries.
    #[must_use]
    pub fn routed_boundary_count(&self) -> usize {
        self.routes.len()
    }

    /// Returns the sum of route costs.
    ///
    /// All route costs must use the same unit.
    #[must_use]
    pub const fn total_route_cost(&self) -> u128 {
        self.total_route_cost
    }

    /// Returns the total number of communication hops.
    #[must_use]
    pub const fn total_hops(&self) -> usize {
        self.total_hops
    }

    /// Returns the sum of abstract communication-work estimates.
    ///
    /// This value is not a payload size or route cost.
    #[must_use]
    pub const fn total_communication_work_estimate(&self) -> u128 {
        self.total_communication_work_estimate
    }

    /// Returns whether the report contains no boundary routes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.routes.is_empty()
    }

    /// Returns whether every original boundary was routed.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.routes.len() == self.partition_boundary_count
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by compiler-level routing.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CompilerRoutingError {
    /// Partitioning did not produce a complete result.
    IncompletePartitioning,

    /// Duplicate partition segment identity was encountered.
    DuplicatePartitionSegment {
        /// Duplicate segment identity.
        segment: PartitionSegmentId,
    },

    /// An operation appears in more than one partition segment.
    DuplicateOperationAssignment {
        /// Operation with duplicate membership.
        operation: OperationId,
    },

    /// A boundary references a segment that does not exist.
    MissingPartitionSegment {
        /// Missing segment identity.
        segment: PartitionSegmentId,
    },

    /// A boundary operation is not a member of its referenced segment.
    BoundaryOperationNotInSegment {
        /// Operation identity.
        operation: OperationId,
        /// Referenced segment.
        segment: PartitionSegmentId,
    },

    /// A boundary refers to a partition without a placement assignment.
    MissingPlacement {
        /// Missing partition segment.
        segment: PartitionSegmentId,
    },

    /// A selected target is not a candidate for its partition segment.
    IncompatiblePlacement {
        /// Partition segment.
        segment: PartitionSegmentId,
        /// Selected target identity.
        target: TargetId,
    },

    /// The configured boundary-route budget was exceeded.
    BoundaryBudgetExceeded {
        /// Number of required boundary routes.
        required: usize,
        /// Configured maximum.
        maximum: usize,
    },

    /// A distributed route search failed.
    PathRoutingFailed {
        /// Source target.
        source: TargetId,
        /// Destination target.
        destination: TargetId,
        /// Underlying routing failure.
        cause: RoutingError,
    },

    /// A required collection could not reserve memory.
    AllocationFailed {
        /// Collection that could not be reserved.
        collection: &'static str,
    },

    /// Checked report arithmetic overflowed.
    ArithmeticOverflow,
}

impl fmt::Display for CompilerRoutingError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::IncompletePartitioning => {
                formatter.write_str(
                    "compiler routing requires complete partitioning evidence",
                )
            }

            Self::DuplicatePartitionSegment { segment } => {
                write!(
                    formatter,
                    "duplicate partition segment identity: {segment}"
                )
            }

            Self::DuplicateOperationAssignment { operation } => {
                write!(
                    formatter,
                    "operation {operation} belongs to more than one partition segment"
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
            } => {
                write!(
                    formatter,
                    "operation {operation} does not belong to partition segment {segment}"
                )
            }

            Self::MissingPlacement { segment } => {
                write!(
                    formatter,
                    "partition segment {segment} has no placement assignment"
                )
            }

            Self::IncompatiblePlacement { segment, target } => {
                write!(
                    formatter,
                    "target {} is not a candidate for partition segment {}",
                    target.as_str(),
                    segment
                )
            }

            Self::BoundaryBudgetExceeded {
                required,
                maximum,
            } => {
                write!(
                    formatter,
                    "routing requires {required} boundary routes, exceeding the configured maximum of {maximum}"
                )
            }

            Self::PathRoutingFailed {
                source,
                destination,
                cause,
            } => {
                write!(
                    formatter,
                    "routing from {} to {} failed: {}",
                    source.as_str(),
                    destination.as_str(),
                    cause
                )
            }

            Self::AllocationFailed { collection } => {
                write!(
                    formatter,
                    "unable to reserve memory for {collection}"
                )
            }

            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "compiler routing report arithmetic overflowed",
                )
            }
        }
    }
}

impl Error for CompilerRoutingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::PathRoutingFailed { cause, .. } => Some(cause),
            _ => None,
        }
    }
}

// ============================================================================
// INTERNAL VALIDATED INPUT
// ============================================================================

struct ValidatedBoundary {
    boundary: PartitionBoundary,
    source_target: TargetId,
    destination_target: TargetId,
}

fn resolve_placement(
    segment_id: PartitionSegmentId,
    placements: &PlacementAssignments,
    segments: &BTreeMap<PartitionSegmentId, &PartitionSegment>,
) -> Result<TargetId, CompilerRoutingError> {
    let target = placements
        .get(&segment_id)
        .ok_or(CompilerRoutingError::MissingPlacement {
            segment: segment_id,
        })?;

    let segment = segments
        .get(&segment_id)
        .copied()
        .ok_or(CompilerRoutingError::MissingPartitionSegment {
            segment: segment_id,
        })?;

    if !segment.candidate_targets().contains(target) {
        return Err(CompilerRoutingError::IncompatiblePlacement {
            segment: segment_id,
            target: target.clone(),
        });
    }

    Ok(target.clone())
}

fn validate_partitioning(
    partitioning: &PartitioningReport,
) -> Result<
    (
        BTreeMap<PartitionSegmentId, &PartitionSegment>,
        BTreeMap<OperationId, PartitionSegmentId>,
    ),
    CompilerRoutingError,
> {
    if !partitioning.is_complete() {
        return Err(CompilerRoutingError::IncompletePartitioning);
    }

    let mut segments = BTreeMap::new();
    let mut operation_segments = BTreeMap::new();

    for segment in partitioning.segments() {
        if segments.insert(segment.id(), segment).is_some() {
            return Err(
                CompilerRoutingError::DuplicatePartitionSegment {
                    segment: segment.id(),
                },
            );
        }

        for operation in segment.work_items() {
            if operation_segments
                .insert(*operation, segment.id())
                .is_some()
            {
                return Err(
                    CompilerRoutingError::DuplicateOperationAssignment {
                        operation: *operation,
                    },
                );
            }
        }
    }

    Ok((segments, operation_segments))
}

fn validate_boundaries(
    partitioning: &PartitioningReport,
    placements: &PlacementAssignments,
    segments: &BTreeMap<PartitionSegmentId, &PartitionSegment>,
    operation_segments: &BTreeMap<OperationId, PartitionSegmentId>,
) -> Result<Vec<ValidatedBoundary>, CompilerRoutingError> {
    let boundaries = partitioning.boundaries();

    let mut validated = Vec::new();

    validated
        .try_reserve(boundaries.len())
        .map_err(|_| CompilerRoutingError::AllocationFailed {
            collection: "validated partition boundaries",
        })?;

    for boundary in boundaries {
        let source_segment = boundary.source_segment();
        let destination_segment = boundary.destination_segment();

        if !segments.contains_key(&source_segment) {
            return Err(
                CompilerRoutingError::MissingPartitionSegment {
                    segment: source_segment,
                },
            );
        }

        if !segments.contains_key(&destination_segment) {
            return Err(
                CompilerRoutingError::MissingPartitionSegment {
                    segment: destination_segment,
                },
            );
        }

        if operation_segments.get(&boundary.source())
            != Some(&source_segment)
        {
            return Err(
                CompilerRoutingError::BoundaryOperationNotInSegment {
                    operation: boundary.source(),
                    segment: source_segment,
                },
            );
        }

        if operation_segments.get(&boundary.destination())
            != Some(&destination_segment)
        {
            return Err(
                CompilerRoutingError::BoundaryOperationNotInSegment {
                    operation: boundary.destination(),
                    segment: destination_segment,
                },
            );
        }

        let source_target =
            resolve_placement(source_segment, placements, segments)?;

        let destination_target =
            resolve_placement(destination_segment, placements, segments)?;

        validated.push(ValidatedBoundary {
            boundary: boundary.clone(),
            source_target,
            destination_target,
        });
    }

    // A stable order independent of dependency insertion order.
    validated.sort_by_key(|item| {
        (
            item.boundary.source(),
            item.boundary.destination(),
            item.boundary.source_segment(),
            item.boundary.destination_segment(),
            item.boundary.communication_cost(),
        )
    });

    Ok(validated)
}

// ============================================================================
// ROUTING ENGINE
// ============================================================================

/// Stateless compiler-level routing coordinator.
#[derive(Clone, Copy, Debug, Default)]
pub struct CompilerRouter;

impl CompilerRouter {
    /// Creates a stateless compiler router.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Returns the stable module identity.
    #[must_use]
    pub const fn module_name() -> &'static str {
        COMPILER_ROUTING_MODULE_NAME
    }

    /// Returns the public API version.
    #[must_use]
    pub const fn api_version() -> u16 {
        COMPILER_ROUTING_API_VERSION
    }

    /// Routes every cross-partition dependency using supplied placement
    /// assignments and an authoritative routing graph snapshot.
    ///
    /// The placement map must contain the selected target for every
    /// partition segment referenced by a boundary.
    ///
    /// Every selected target must belong to its segment's candidate set.
    ///
    /// The evaluator determines edge eligibility and effective cost.
    ///
    /// Repeated source/destination target pairs are routed once per
    /// invocation and their immutable route result is reused.
    ///
    /// # Errors
    ///
    /// Returns errors for incomplete or inconsistent partitioning,
    /// missing or incompatible placement, unavailable routes,
    /// evaluator failures, exhausted budgets, allocation failures,
    /// or arithmetic overflow.
    pub fn route_partition_boundaries<Evaluator>(
        &self,
        partitioning: &PartitioningReport,
        placements: &PlacementAssignments,
        graph: &RoutingGraph<TargetId>,
        options: CompilerRoutingOptions,
        evaluator: &Evaluator,
    ) -> Result<CompilerRoutingReport, CompilerRoutingError>
    where
        Evaluator: RouteEdgeEvaluator<TargetId>,
    {
        let (segments, operation_segments) =
            validate_partitioning(partitioning)?;

        let boundaries = partitioning.boundaries();

        if let Some(maximum) =
            options.budget().max_boundary_routes()
        {
            if boundaries.len() > maximum {
                return Err(
                    CompilerRoutingError::BoundaryBudgetExceeded {
                        required: boundaries.len(),
                        maximum,
                    },
                );
            }
        }

        // Validate every boundary and placement before performing any
        // potentially expensive route search.
        let validated = validate_boundaries(
            partitioning,
            placements,
            &segments,
            &operation_segments,
        )?;

        let mut routes = Vec::new();

        routes
            .try_reserve(validated.len())
            .map_err(|_| CompilerRoutingError::AllocationFailed {
                collection: "compiler boundary routes",
            })?;

        // Cache by ordered directed endpoints. Reverse paths are not
        // interchangeable because routing graphs are directed.
        let mut route_cache =
            BTreeMap::<(TargetId, TargetId), RoutePlan<TargetId>>::new();

        for item in validated {
            let key = (
                item.source_target.clone(),
                item.destination_target.clone(),
            );

            let route = if let Some(cached) = route_cache.get(&key) {
                cached.clone()
            } else {
                let computed = find_route(
                    graph,
                    &item.source_target,
                    &item.destination_target,
                    options.path_policy(),
                    evaluator,
                )
                .map_err(|cause| {
                    CompilerRoutingError::PathRoutingFailed {
                        source: item.source_target.clone(),
                        destination: item.destination_target.clone(),
                        cause,
                    }
                })?;

                computed
                    .validate_structure()
                    .map_err(|cause| {
                        CompilerRoutingError::PathRoutingFailed {
                            source: item.source_target.clone(),
                            destination: item.destination_target.clone(),
                            cause,
                        }
                    })?;

                route_cache.insert(key, computed.clone());
                computed
            };

            routes.push(BoundaryRoute {
                source_operation: item.boundary.source(),
                destination_operation: item.boundary.destination(),

                source_segment: item.boundary.source_segment(),
                destination_segment: item.boundary.destination_segment(),

                source_target: item.source_target,
                destination_target: item.destination_target,

                route,

                communication_work_estimate:
                    item.boundary.communication_cost(),
            });
        }

        CompilerRoutingReport::new(
            routes,
            boundaries.len(),
        )
    }

    /// Routes one selected source target to one selected destination target.
    ///
    /// This entry point is useful when routing is requested independently
    /// of compiler partition-boundary orchestration.
    ///
    /// # Errors
    ///
    /// Returns the distributed routing engine's typed routing error.
    pub fn route_targets<Evaluator>(
        &self,
        graph: &RoutingGraph<TargetId>,
        source: &TargetId,
        destination: &TargetId,
        policy: RoutingPolicy,
        evaluator: &Evaluator,
    ) -> Result<RoutePlan<TargetId>, RoutingError>
    where
        Evaluator: RouteEdgeEvaluator<TargetId>,
    {
        let route = find_route(
            graph,
            source,
            destination,
            policy,
            evaluator,
        )?;

        route.validate_structure()?;

        Ok(route)
    }
}

// ============================================================================
// CONVENIENCE API
// ============================================================================

/// Routes partition boundaries using the distributed router's base costs.
///
/// This convenience function does not apply additional provider-specific
/// edge evaluation.
///
/// # Errors
///
/// Returns a compiler routing error when routing cannot be completed.
pub fn route_partition_boundaries(
    partitioning: &PartitioningReport,
    placements: &PlacementAssignments,
    graph: &RoutingGraph<TargetId>,
    options: CompilerRoutingOptions,
) -> Result<CompilerRoutingReport, CompilerRoutingError> {
    CompilerRouter::new().route_partition_boundaries(
        partitioning,
        placements,
        graph,
        options,
        &BaseCostEvaluator,
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::distributed::planning::routing::{
        BaseCostEvaluator,
        RoutingGraph,
        RoutingPolicy,
    };

    fn target(value: &str) -> TargetId {
        TargetId::new(value).expect("valid target identifier")
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            CompilerRouter::module_name(),
            COMPILER_ROUTING_MODULE_NAME,
        );

        assert_eq!(
            CompilerRouter::api_version(),
            COMPILER_ROUTING_API_VERSION,
        );
    }

    #[test]
    fn unlimited_options_have_no_boundary_limit() {
        assert_eq!(
            CompilerRoutingOptions::unlimited()
                .budget()
                .max_boundary_routes(),
            None,
        );
    }

    #[test]
    fn boundary_budget_accepts_optional_limits() {
        let budget = CompilerRoutingBudget::unlimited()
            .with_max_boundary_routes(Some(12));

        assert_eq!(budget.max_boundary_routes(), Some(12));
    }

    #[test]
    fn distributed_graph_routes_between_targets() {
        let source = target("cpu:worker-a");
        let relay = target("network:relay-a");
        let destination = target("accelerator:worker-b");

        let mut graph = RoutingGraph::new();

        graph.add_node(source.clone());
        graph.add_node(relay.clone());
        graph.add_node(destination.clone());

        graph.add_edge(&source, relay.clone(), 4).unwrap();
        graph.add_edge(&relay, destination.clone(), 6).unwrap();

        let result = CompilerRouter::new()
            .route_targets(
                &graph,
                &source,
                &destination,
                RoutingPolicy::new(),
                &BaseCostEvaluator,
            )
            .expect("route should exist");

        assert_eq!(result.total_cost(), 10);
        assert_eq!(result.hops(), 2);

        assert_eq!(
            result.nodes(),
            &[source, relay, destination],
        );
    }

    #[test]
    fn lowest_cost_path_is_selected() {
        let source = target("cpu:source");
        let slow = target("network:slow");
        let fast = target("network:fast");
        let destination = target("accelerator:destination");

        let mut graph = RoutingGraph::new();

        for node in [
            source.clone(),
            slow.clone(),
            fast.clone(),
            destination.clone(),
        ] {
            graph.add_node(node);
        }

        graph.add_edge(&source, slow.clone(), 50).unwrap();
        graph.add_edge(&slow, destination.clone(), 50).unwrap();
        graph.add_edge(&source, fast.clone(), 2).unwrap();
        graph.add_edge(&fast, destination.clone(), 3).unwrap();

        let result = CompilerRouter::new()
            .route_targets(
                &graph,
                &source,
                &destination,
                RoutingPolicy::new(),
                &BaseCostEvaluator,
            )
            .expect("route should exist");

        assert_eq!(result.total_cost(), 5);
        assert_eq!(
            result.nodes(),
            &[source, fast, destination],
        );
    }

    #[test]
    fn same_target_requires_no_network_hop() {
        let source = target("cpu:local");

        let mut graph = RoutingGraph::new();
        graph.add_node(source.clone());

        let result = CompilerRouter::new()
            .route_targets(
                &graph,
                &source,
                &source,
                RoutingPolicy::new(),
                &BaseCostEvaluator,
            )
            .expect("local route");

        assert_eq!(result.hops(), 0);
        assert_eq!(result.total_cost(), 0);
    }

    #[test]
    fn unknown_target_returns_typed_error() {
        let source = target("cpu:source");
        let destination = target("accelerator:destination");

        let mut graph = RoutingGraph::new();
        graph.add_node(source.clone());

        let result = CompilerRouter::new()
            .route_targets(
                &graph,
                &source,
                &destination,
                RoutingPolicy::new(),
                &BaseCostEvaluator,
            );

        assert_eq!(
            result.unwrap_err(),
            RoutingError::UnknownDestination,
        );
    }

    #[test]
    fn zero_cost_edges_are_supported() {
        let source = target("cpu:source");
        let destination = target("accelerator:destination");

        let mut graph = RoutingGraph::new();

        graph.add_node(source.clone());
        graph.add_node(destination.clone());

        graph.add_edge(&source, destination.clone(), 0).unwrap();

        let result = CompilerRouter::new()
            .route_targets(
                &graph,
                &source,
                &destination,
                RoutingPolicy::new(),
                &BaseCostEvaluator,
            )
            .expect("zero-cost route");

        assert_eq!(result.total_cost(), 0);
    }

    #[test]
    fn route_plan_preserves_endpoints() {
        let source = target("cpu:source");
        let destination = target("accelerator:destination");

        let mut graph = RoutingGraph::new();

        graph.add_node(source.clone());
        graph.add_node(destination.clone());

        graph.add_edge(&source, destination.clone(), 1).unwrap();

        let route = CompilerRouter::new()
            .route_targets(
                &graph,
                &source,
                &destination,
                RoutingPolicy::new(),
                &BaseCostEvaluator,
            )
            .expect("route");

        assert_eq!(route.nodes().first(), Some(&source));
        assert_eq!(route.nodes().last(), Some(&destination));
        assert!(route.validate_structure().is_ok());
    }

    #[test]
    fn boundary_budget_is_an_operational_policy() {
        let budget = CompilerRoutingBudget::unlimited()
            .with_max_boundary_routes(None);

        assert_eq!(budget.max_boundary_routes(), None);
    }
}
