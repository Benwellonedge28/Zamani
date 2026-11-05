
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — Heterogeneous Routing
//!
//! Path: src/compiler/planning/routing.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Coordinate routing between compiler-planned heterogeneous workload
//! partitions using authoritative placement evidence and the existing
//! provider-neutral distributed routing implementation.
//!
//! This module consumes:
//!
//! - Compiler PartitioningReport.
//! - PartitionBoundary.
//! - Placement assignments.
//! - Canonical TargetId.
//! - A validated distributed RoutingGraph.
//! - An injectable RouteEdgeEvaluator.
//!
//! It produces:
//!
//! - Deterministic inter-target routes.
//! - Partition-boundary route associations.
//! - Routing cost information.
//! - Explicit routing failures.
//! - Structural validation.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Compiler partition-boundary routing.
//! - Placement-to-routing integration.
//! - Placement compatibility validation.
//! - Routing report construction.
//! - Routing budget enforcement at the compiler boundary.
//! - Routing diagnostics.
//!
//! ## DOES NOT OWN
//!
//! - Source syntax.
//! - AST.
//! - Semantic analysis.
//! - ZUIR.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - AI IR.
//! - Workload partitioning.
//! - Physical placement.
//! - Hardware discovery.
//! - Network topology authority.
//! - Network transport.
//! - Physical resource allocation.
//! - Execution scheduling.
//! - Quantum routing.
//! - Quantum error correction.
//! - Noise or calibration models.
//! - Runtime execution.
//!
//! ## AUTHORITATIVE INTEGRATIONS
//!
//! Partitioning:
//!     crate::compiler::planning::partitioning
//!
//! Target identity:
//!     crate::compiler::configuration::target
//!
//! Distributed routing:
//!     crate::distributed::planning::routing
//!
//! Placement:
//!     crate::compiler::planning::placement
//!     (consumed through a placement-assignment map)
//!
//! Scheduling:
//!     crate::compiler::planning::scheduling
//!
//! Runtime:
//!     crate::runtime
//!
//! Networking:
//!     crate::networking
//!
//! ## ROUTING SEMANTICS
//!
//! A partition boundary identifies a dependency crossing two logical
//! partition segments.
//!
//! Placement evidence maps each partition segment to its selected
//! target.
//!
//! A directed routing graph describes available logical communication
//! paths between target identities.
//!
//! The distributed routing subsystem computes the actual proposed path.
//!
//! Graph edge costs must use one consistent caller-defined unit.
//!
//! Communication work estimates are retained separately because they
//! must not be added to route costs unless their units are compatible.
//!
//! ## SCALABILITY
//!
//! No arbitrary universal limits are imposed on:
//!
//! - Targets.
//! - Nodes.
//! - Links.
//! - Routes.
//! - Partition segments.
//! - Dependencies.
//! - Network size.
//! - Routing hops.
//! - Computing domains.
//! - Distributed participants.
//!
//! Optional limits are operational policies.
//!
//! Physical capacity and implementation resources remain finite.
//!
//! Literal infinite execution capacity is not promised.
//!
//! ## DETERMINISM
//!
//! Determinism is inherited from the canonical distributed routing engine.
//!
//! TargetId ordering provides deterministic target identity ordering.
//!
//! The supplied edge evaluator must be deterministic for reproducible
//! routing results.
//!
//! ## SAFETY
//!
//! Safe Rust only.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::compiler::configuration::target::TargetId;

use crate::compiler::planning::partitioning::{
    PartitionBoundary,
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

/// Stable architectural identity of this module.
pub const COMPILER_ROUTING_MODULE_NAME: &str =
    "zamani.compiler.planning.routing";

/// Public API version of this module.
pub const COMPILER_ROUTING_API_VERSION: u16 = 1;

/// Routing cost supplied by the distributed routing authority.
///
/// Units are caller-defined and must remain consistent within a
/// routing graph and evaluation policy.
pub type CompilerRoutingCost = u128;

// ============================================================================
// ROUTING OPTIONS
// ============================================================================

/// Operational limits for compiler-level routing.
///
/// These are compilation policies, not language-level hardware limits.
///
/// A missing limit means that this module does not impose an
/// additional restriction for that dimension.
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

    /// Creates unrestricted compiler routing options.
    ///
    /// The underlying distributed routing policy also has no
    /// caller-imposed search limits.
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
/// This is supplied by the authoritative placement subsystem.
///
/// The routing module never chooses a target merely because it appears
/// in a segment's candidate-target set.
///
/// The selected target must be present in that segment's candidate set.
pub type PlacementAssignments =
    BTreeMap<PartitionSegmentId, TargetId>;

// ============================================================================
// ROUTED BOUNDARY
// ============================================================================

/// A successfully routed compiler partition boundary.
///
/// The route is a proposed path over a supplied routing snapshot.
///
/// It does not reserve network bandwidth or establish a connection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundaryRoute {
    source_operation: crate::zuir::ids::OperationId,
    destination_operation: crate::zuir::ids::OperationId,

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
    pub const fn source_operation(
        &self,
    ) -> crate::zuir::ids::OperationId {
        self.source_operation
    }

    /// Returns the destination operation.
    #[must_use]
    pub const fn destination_operation(
        &self,
    ) -> crate::zuir::ids::OperationId {
        self.destination_operation
    }

    /// Returns the source partition segment.
    #[must_use]
    pub const fn source_segment(
        &self,
    ) -> PartitionSegmentId {
        self.source_segment
    }

    /// Returns the destination partition segment.
    #[must_use]
    pub const fn destination_segment(
        &self,
    ) -> PartitionSegmentId {
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

    /// Returns the underlying distributed routing plan.
    #[must_use]
    pub fn route(&self) -> &RoutePlan<TargetId> {
        &self.route
    }

    /// Returns the original abstract communication work estimate.
    ///
    /// This estimate is not automatically added to physical route cost.
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
// ROUTING REPORT
// ============================================================================

/// Complete compiler-level routing report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilerRoutingReport {
    routes: Vec<BoundaryRoute>,
    partition_boundary_count: usize,
    total_route_cost: u128,
    total_hops: usize,
}

impl CompilerRoutingReport {
    fn new(routes: Vec<BoundaryRoute>) -> Result<Self, CompilerRoutingError> {
        let mut total_route_cost = 0_u128;
        let mut total_hops = 0_usize;

        for route in &routes {
            total_route_cost = total_route_cost
                .checked_add(route.route_cost())
                .ok_or(CompilerRoutingError::ArithmeticOverflow)?;

            total_hops = total_hops
                .checked_add(route.hop_count())
                .ok_or(CompilerRoutingError::ArithmeticOverflow)?;
        }

        Ok(Self {
            partition_boundary_count: routes.len(),
            routes,
            total_route_cost,
            total_hops,
        })
    }

    /// Returns all successfully routed partition boundaries.
    #[must_use]
    pub fn routes(&self) -> &[BoundaryRoute] {
        &self.routes
    }

    /// Returns the number of routed partition boundaries.
    #[must_use]
    pub const fn boundary_count(&self) -> usize {
        self.partition_boundary_count
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

    /// Returns whether the report contains no boundary routes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.routes.is_empty()
    }

    /// Returns whether every supplied partition boundary was routed.
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

    /// The requested boundary-route budget was exceeded.
    BoundaryBudgetExceeded {
        /// Number of required boundary routes.
        required: usize,
        /// Configured maximum.
        maximum: usize,
    },

    /// Distributed routing could not find a valid route.
    PathRoutingFailed {
        /// Source target.
        source: TargetId,
        /// Destination target.
        destination: TargetId,
        /// Underlying routing failure.
        cause: RoutingError,
    },

    /// Checked arithmetic overflowed.
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
    /// assignments and an authoritative routing graph.
    ///
    /// The placement map must contain the selected target for every
    /// partition segment referenced by a boundary.
    ///
    /// Every selected target must belong to the corresponding segment's
    /// candidate-target set.
    ///
    /// The routing graph must contain the selected targets and the
    /// communication links available in the supplied topology snapshot.
    ///
    /// The injected evaluator determines whether individual directed
    /// edges are available and their effective costs.
    ///
    /// No physical resource is reserved.
    ///
    /// No network communication is performed.
    ///
    /// # Errors
    ///
    /// Returns an error for incomplete partitioning, missing placement,
    /// incompatible placement, an unavailable route, a failed evaluator,
    /// exhausted routing budgets, or arithmetic overflow.
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
        if !partitioning.is_complete() {
            return Err(CompilerRoutingError::IncompletePartitioning);
        }

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

        // Build an index once instead of repeatedly scanning partition
        // segments for every boundary.
        let segments: BTreeMap<_, _> = partitioning
            .segments()
            .iter()
            .map(|segment| (segment.id(), segment))
            .collect();

        let mut routes = Vec::with_capacity(boundaries.len());

        for boundary in boundaries {
            let source_target = self.resolve_placement(
                boundary.source_segment(),
                placements,
                &segments,
            )?;

            let destination_target = self.resolve_placement(
                boundary.destination_segment(),
                placements,
                &segments,
            )?;

            let route = find_route(
                graph,
                &source_target,
                &destination_target,
                options.path_policy(),
                evaluator,
            )
            .map_err(|cause| {
                CompilerRoutingError::PathRoutingFailed {
                    source: source_target.clone(),
                    destination: destination_target.clone(),
                    cause,
                }
            })?;

            route
                .validate_structure()
                .map_err(|cause| {
                    CompilerRoutingError::PathRoutingFailed {
                        source: source_target.clone(),
                        destination: destination_target.clone(),
                        cause,
                    }
                })?;

            routes.push(BoundaryRoute {
                source_operation: boundary.source(),
                destination_operation: boundary.destination(),

                source_segment: boundary.source_segment(),
                destination_segment: boundary.destination_segment(),

                source_target,
                destination_target,

                route,

                communication_work_estimate:
                    boundary.communication_cost(),
            });
        }

        CompilerRoutingReport::new(routes)
    }

    /// Routes one selected source target to one selected destination target.
    ///
    /// This lower-level entry point is useful when routing is requested
    /// independently of compiler partition-boundary orchestration.
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
        find_route(
            graph,
            source,
            destination,
            policy,
            evaluator,
        )
    }

    fn resolve_placement<'a>(
        &self,
        segment_id: PartitionSegmentId,
        placements: &'a PlacementAssignments,
        segments: &BTreeMap<
            PartitionSegmentId,
            &crate::compiler::planning::partitioning::PartitionSegment,
        >,
    ) -> Result<TargetId, CompilerRoutingError> {
        let target = placements
            .get(&segment_id)
            .ok_or(
                CompilerRoutingError::MissingPlacement {
                    segment: segment_id,
                },
            )?;

        let segment = segments
            .get(&segment_id)
            .ok_or(
                CompilerRoutingError::MissingPlacement {
                    segment: segment_id,
                },
            )?;

        if !segment.candidate_targets().contains(target) {
            return Err(
                CompilerRoutingError::IncompatiblePlacement {
                    segment: segment_id,
                    target: target.clone(),
                },
            );
        }

        Ok(target.clone())
    }
}

// ============================================================================
// CONVENIENCE FUNCTIONS
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

    use crate::compiler::configuration::target::TargetId;

    use crate::distributed::planning::routing::{
        BaseCostEvaluator,
        RouteEdge,
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
            "zamani.compiler.planning.routing"
        );

        assert_eq!(CompilerRouter::api_version(), 1);
    }

    #[test]
    fn unlimited_options_have_no_boundary_limit() {
        let options = CompilerRoutingOptions::unlimited();

        assert_eq!(
            options.budget().max_boundary_routes(),
            None
        );
    }

    #[test]
    fn routing_budget_accepts_optional_limits() {
        let budget =
            CompilerRoutingBudget::unlimited()
                .with_max_boundary_routes(Some(12));

        assert_eq!(
            budget.max_boundary_routes(),
            Some(12)
        );
    }

    #[test]
    fn distributed_graph_routes_between_targets() {
        let source = target("cpu:worker-a");
        let intermediate = target("network:relay-a");
        let destination = target("gpu:worker-b");

        let mut graph = RoutingGraph::new();

        graph.add_node(source.clone());
        graph.add_node(intermediate.clone());
        graph.add_node(destination.clone());

        graph
            .add_edge(
                &source,
                intermediate.clone(),
                4,
            )
            .expect("first edge");

        graph
            .add_edge(
                &intermediate,
                destination.clone(),
                6,
            )
            .expect("second edge");

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
            &[
                source,
                intermediate,
                destination,
            ]
        );
    }

    #[test]
    fn routing_selects_lowest_cost_path() {
        let source = target("cpu:source");
        let slow = target("network:slow");
        let fast = target("network:fast");
        let destination = target("gpu:destination");

        let mut graph = RoutingGraph::new();

        for node in [
            source.clone(),
            slow.clone(),
            fast.clone(),
            destination.clone(),
        ] {
            graph.add_node(node);
        }

        graph
            .add_edge(&source, slow.clone(), 50)
            .expect("slow edge");

        graph
            .add_edge(&slow, destination.clone(), 50)
            .expect("slow destination edge");

        graph
            .add_edge(&source, fast.clone(), 2)
            .expect("fast edge");

        graph
            .add_edge(&fast, destination.clone(), 3)
            .expect("fast destination edge");

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
            &[
                source,
                fast,
                destination,
            ]
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
    fn unknown_target_returns_error() {
        let source = target("cpu:source");
        let destination = target("gpu:destination");

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

        assert!(matches!(
            result,
            Err(RoutingError::UnknownDestination)
        ));
    }

    #[test]
    fn zero_cost_edges_are_supported() {
        let source = target("cpu:source");
        let destination = target("gpu:destination");

        let mut graph = RoutingGraph::new();

        graph.add_node(source.clone());
        graph.add_node(destination.clone());

        graph
            .add_edge(&source, destination.clone(), 0)
            .expect("zero-cost edge");

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
    fn duplicate_graph_edges_are_rejected() {
        let source = target("cpu:source");
        let destination = target("gpu:destination");

        let mut graph = RoutingGraph::new();

        graph.add_node(source.clone());
        graph.add_node(destination.clone());

        graph
            .add_edge(&source, destination.clone(), 1)
            .expect("first edge");

        assert!(
            graph
                .add_edge(&source, destination, 2)
                .is_err()
        );
    }

    #[test]
    fn route_plan_preserves_endpoints() {
        let source = target("cpu:source");
        let destination = target("gpu:destination");

        let mut graph = RoutingGraph::new();

        graph.add_node(source.clone());
        graph.add_node(destination.clone());

        graph
            .add_edge(&source, destination.clone(), 1)
            .expect("edge");

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
    fn boundary_budget_is_not_a_hardware_limit() {
        let budget =
            CompilerRoutingBudget::unlimited()
                .with_max_boundary_routes(None);

        assert_eq!(budget.max_boundary_routes(), None);
    }
}
