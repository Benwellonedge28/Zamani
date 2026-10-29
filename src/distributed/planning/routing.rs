
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Provider-neutral routing for Zamani distributed computation.
//!
//! # Ownership
//!
//! This module owns:
//! - A directed graph representation for one routing snapshot.
//! - Deterministic, cost-aware shortest-path search.
//! - Per-request routing budgets.
//! - Injectable edge eligibility and cost evaluation.
//! - Route validation and diagnostic errors.
//!
//! It does not own:
//! - Source syntax, ASTs, or semantic analysis.
//! - ZUIR, ZCIR, quantum::ir, HDLIR, or any other domain IR.
//! - Resource discovery, capability discovery, or physical allocation.
//! - The authoritative topology or membership registry.
//! - Network protocols, sockets, message serialization, or transport I/O.
//! - Task partitioning, placement, scheduling, or execution.
//! - Quantum routing, gate transformation, or physical device control.
//!
//! # Integration
//!
//! The topology adapter builds a `RoutingGraph` from a validated snapshot.
//! The routing policy supplies optional budgets. An injected
//! `RouteEdgeEvaluator` applies provider-owned eligibility constraints and
//! cost models. The returned `RoutePlan` is a proposed path, not a network
//! reservation or proof that the path remains available.
//!
//! The caller must revalidate the plan and acquire any required resources
//! before executing communication.
//!
//! `planning::placement` remains responsible for assigning work to targets.
//! `planning::scheduling` remains responsible for execution ordering.
//! The networking and transport layers remain responsible for performing
//! communication.
//!
//! # Cost semantics
//!
//! All edge costs in one request must use the same non-negative unit.
//! The unit is chosen by the caller and may represent latency, monetary
//! cost, energy, a weighted objective, or another documented metric.
//!
//! Multi-objective policies should combine metrics in a domain-appropriate,
//! documented evaluator. This module does not assume that unlike physical
//! quantities can be added meaningfully.
//!
//! Zero-cost edges are supported. A path's total cost is accumulated using
//! checked arithmetic. Overflow is reported, never wrapped.
//!
//! # Scalability
//!
//! There is no universal maximum number of nodes, links, or hops.
//! Callers may impose explicit search budgets. Such budgets are operational
//! policy, not language-level semantic limits.
//!
//! Actual memory, address-space, integer, and execution limits remain
//! finite. Literal infinite execution capacity is not promised.
//!
//! # Determinism
//!
//! For a fixed graph snapshot, evaluator, and policy, node identifiers
//! implementing `Ord` provide deterministic tie-breaking. Evaluators must
//! themselves be deterministic if reproducible plans are required.
//!
//! # Safety
//!
//! This implementation uses safe Rust only.

use core::cmp::Reverse;
use core::fmt;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

/// Version of the public in-memory routing API.
///
/// This is not a wire-protocol version.
pub const ROUTING_API_VERSION: u16 = 1;

/// Cost of traversing a route edge.
///
/// Units are caller-defined and must be consistent within a request.
pub type RoutingCost = u128;

/// A directed edge in the routing graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteEdge<NodeId> {
    destination: NodeId,
    base_cost: RoutingCost,
}

impl<NodeId> RouteEdge<NodeId> {
    /// Creates an edge with a non-negative base cost.
    #[must_use]
    pub const fn new(destination: NodeId, base_cost: RoutingCost) -> Self {
        Self {
            destination,
            base_cost,
        }
    }

    /// Returns the destination node.
    #[must_use]
    pub const fn destination(&self) -> &NodeId {
        &self.destination
    }

    /// Returns the unmodified cost supplied by the graph builder.
    #[must_use]
    pub const fn base_cost(&self) -> RoutingCost {
        self.base_cost
    }
}

/// A directed graph snapshot used for route planning.
///
/// Node identifiers are logical identifiers supplied by the integration
/// layer. They need not be hostnames, IP addresses, device IDs, or process
/// IDs.
///
/// Nodes with no outgoing edges are supported. The graph is not required
/// to be connected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutingGraph<NodeId> {
    adjacency: BTreeMap<NodeId, Vec<RouteEdge<NodeId>>>,
}

/// Errors from graph construction and structural validation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum GraphError {
    /// An edge references a node that has not been registered.
    UnknownNode,

    /// An attempt was made to add the same directed edge more than once.
    DuplicateEdge,

    /// An edge was added with a source equal to its destination when
    /// self-edges were disallowed by the caller.
    SelfEdgeDisallowed,
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownNode => {
                f.write_str("routing edge references an unknown node")
            }
            Self::DuplicateEdge => {
                f.write_str("duplicate directed routing edge")
            }
            Self::SelfEdgeDisallowed => {
                f.write_str("routing self-edge is disallowed")
            }
        }
    }
}

impl std::error::Error for GraphError {}

impl<NodeId> Default for RoutingGraph<NodeId>
where
    NodeId: Ord,
{
    fn default() -> Self {
        Self {
            adjacency: BTreeMap::new(),
        }
    }
}

impl<NodeId> RoutingGraph<NodeId>
where
    NodeId: Clone + Ord,
{
    /// Creates an empty routing graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a node.
    ///
    /// Registering an existing node is idempotent.
    pub fn add_node(&mut self, node: NodeId) {
        self.adjacency.entry(node).or_default();
    }

    /// Adds a directed edge.
    ///
    /// Both endpoints must already be registered. Duplicate edges are
    /// rejected rather than silently overwriting an existing cost.
    ///
    /// A self-edge is permitted. The caller's evaluator determines whether
    /// such an edge is useful or admissible.
    ///
    /// # Errors
    ///
    /// Returns `GraphError::UnknownNode` if either endpoint is absent, or
    /// `GraphError::DuplicateEdge` if the same directed edge already exists.
    pub fn add_edge(
        &mut self,
        source: &NodeId,
        destination: NodeId,
        base_cost: RoutingCost,
    ) -> Result<(), GraphError> {
        if !self.adjacency.contains_key(source)
            || !self.adjacency.contains_key(&destination)
        {
            return Err(GraphError::UnknownNode);
        }

        let edges = self
            .adjacency
            .get_mut(source)
            .expect("source existence was checked above");

        if edges
            .iter()
            .any(|edge| edge.destination == destination)
        {
            return Err(GraphError::DuplicateEdge);
        }

        edges.push(RouteEdge::new(destination, base_cost));

        // Sort after insertion so graph traversal order is independent of
        // the order in which callers supplied edges.
        edges.sort_by(|left, right| {
            left.destination.cmp(&right.destination)
        });

        Ok(())
    }

    /// Adds an edge in both directions.
    ///
    /// The operation is atomic with respect to this graph: if either
    /// direction cannot be added, neither direction is retained.
    ///
    /// Both endpoints must exist, and neither direction may already exist.
    ///
    /// # Errors
    ///
    /// Returns a graph validation error if either edge is invalid.
    pub fn add_bidirectional_edge(
        &mut self,
        left: &NodeId,
        right: &NodeId,
        cost: RoutingCost,
    ) -> Result<(), GraphError> {
        if !self.adjacency.contains_key(left)
            || !self.adjacency.contains_key(right)
        {
            return Err(GraphError::UnknownNode);
        }

        if left == right {
            return Err(GraphError::SelfEdgeDisallowed);
        }

        let left_has_edge = self
            .adjacency
            .get(left)
            .expect("left existence was checked above")
            .iter()
            .any(|edge| edge.destination == *right);

        let right_has_edge = self
            .adjacency
            .get(right)
            .expect("right existence was checked above")
            .iter()
            .any(|edge| edge.destination == *left);

        if left_has_edge || right_has_edge {
            return Err(GraphError::DuplicateEdge);
        }

        // Both validations completed before either mutation.
        self.adjacency
            .get_mut(left)
            .expect("left existence was checked above")
            .push(RouteEdge::new(right.clone(), cost));

        self.adjacency
            .get_mut(right)
            .expect("right existence was checked above")
            .push(RouteEdge::new(left.clone(), cost));

        self.adjacency
            .get_mut(left)
            .expect("left existence was checked above")
            .sort_by(|a, b| a.destination.cmp(&b.destination));

        self.adjacency
            .get_mut(right)
            .expect("right existence was checked above")
            .sort_by(|a, b| a.destination.cmp(&b.destination));

        Ok(())
    }

    /// Returns whether a node is registered.
    #[must_use]
    pub fn contains_node(&self, node: &NodeId) -> bool {
        self.adjacency.contains_key(node)
    }

    /// Returns the number of registered nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.adjacency.len()
    }

    /// Returns the number of directed edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.adjacency.values().map(Vec::len).sum()
    }

    /// Returns the outgoing edges of a registered node.
    #[must_use]
    pub fn outgoing_edges(
        &self,
        node: &NodeId,
    ) -> Option<&[RouteEdge<NodeId>]> {
        self.adjacency.get(node).map(Vec::as_slice)
    }

    /// Returns registered node identifiers in deterministic order.
    pub fn nodes(&self) -> impl Iterator<Item = &NodeId> {
        self.adjacency.keys()
    }

    /// Validates graph references and duplicate directed edges.
    ///
    /// Construction already maintains these invariants. This method is
    /// useful at integration boundaries after obtaining a graph snapshot.
    pub fn validate(&self) -> Result<(), GraphError> {
        for (source, edges) in &self.adjacency {
            let mut destinations = BTreeSet::new();

            for edge in edges {
                if !self.adjacency.contains_key(&edge.destination) {
                    return Err(GraphError::UnknownNode);
                }

                if !destinations.insert(&edge.destination) {
                    return Err(GraphError::DuplicateEdge);
                }

                let _ = source;
            }
        }

        Ok(())
    }
}

/// Optional operational budgets for one route search.
///
/// `None` means no additional limit for that dimension. These are not
/// semantic restrictions on a Zamani program.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RoutingBudget {
    max_expansions: Option<usize>,
    max_hops: Option<usize>,
    max_total_cost: Option<RoutingCost>,
}

impl RoutingBudget {
    /// Creates a routing budget without caller-imposed limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_expansions: None,
            max_hops: None,
            max_total_cost: None,
        }
    }

    /// Sets the maximum number of expanded search states.
    ///
    /// The source state counts as an expansion when it is processed.
    #[must_use]
    pub const fn with_max_expansions(
        mut self,
        limit: Option<usize>,
    ) -> Self {
        self.max_expansions = limit;
        self
    }

    /// Sets the maximum number of edges in a route.
    ///
    /// A source-to-itself route has zero hops.
    #[must_use]
    pub const fn with_max_hops(mut self, limit: Option<usize>) -> Self {
        self.max_hops = limit;
        self
    }

    /// Sets the maximum permitted total route cost.
    #[must_use]
    pub const fn with_max_total_cost(
        mut self,
        limit: Option<RoutingCost>,
    ) -> Self {
        self.max_total_cost = limit;
        self
    }

    /// Returns the expansion budget.
    #[must_use]
    pub const fn max_expansions(self) -> Option<usize> {
        self.max_expansions
    }

    /// Returns the hop budget.
    #[must_use]
    pub const fn max_hops(self) -> Option<usize> {
        self.max_hops
    }

    /// Returns the maximum permitted total cost.
    #[must_use]
    pub const fn max_total_cost(self) -> Option<RoutingCost> {
        self.max_total_cost
    }
}

/// Policy for a single routing request.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RoutingPolicy {
    budget: RoutingBudget,
}

impl RoutingPolicy {
    /// Creates a policy without caller-imposed operational limits.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            budget: RoutingBudget::unlimited(),
        }
    }

    /// Replaces the routing budget.
    #[must_use]
    pub const fn with_budget(mut self, budget: RoutingBudget) -> Self {
        self.budget = budget;
        self
    }

    /// Returns this policy's routing budget.
    #[must_use]
    pub const fn budget(self) -> RoutingBudget {
        self.budget
    }
}

/// Result of evaluating an edge for a particular route request.
///
/// `Unavailable` means that the edge cannot be used for this request.
/// It does not mean that the physical edge does not exist.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EdgeDecision {
    /// The edge is usable with this effective cost.
    Allowed {
        /// Effective non-negative traversal cost.
        cost: RoutingCost,
    },

    /// The edge is not admissible for this route request.
    Unavailable,
}

/// Evaluates edge eligibility and effective cost.
///
/// Implementations may incorporate link quality, observed latency, security
/// policy, trust requirements, transport compatibility, energy, congestion,
/// or other documented routing criteria.
///
/// Implementations must not mutate the graph while routing. For deterministic
/// results, evaluation must be deterministic for a fixed snapshot and
/// request. All returned costs must use the request's chosen unit.
pub trait RouteEdgeEvaluator<NodeId> {
    /// Evaluates one directed edge.
    ///
    /// # Errors
    ///
    /// Return `RoutingError::EvaluationFailed` when authoritative evaluation
    /// cannot be completed. Do not report an unknown required constraint as
    /// allowed.
    fn evaluate(
        &self,
        source: &NodeId,
        edge: &RouteEdge<NodeId>,
    ) -> Result<EdgeDecision, RoutingError>;
}

/// Evaluator that uses the base cost of each graph edge.
#[derive(Clone, Copy, Debug, Default)]
pub struct BaseCostEvaluator;

impl<NodeId> RouteEdgeEvaluator<NodeId> for BaseCostEvaluator {
    fn evaluate(
        &self,
        _source: &NodeId,
        edge: &RouteEdge<NodeId>,
    ) -> Result<EdgeDecision, RoutingError> {
        Ok(EdgeDecision::Allowed {
            cost: edge.base_cost,
        })
    }
}

/// Typed failures produced by route planning.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RoutingError {
    /// The source node is not present in the graph snapshot.
    UnknownSource,

    /// The destination node is not present in the graph snapshot.
    UnknownDestination,

    /// No admissible route exists under the supplied graph and evaluator.
    NoRoute,

    /// The route would exceed a configured cost budget.
    CostBudgetExceeded,

    /// The route search exhausted its configured expansion budget.
    SearchBudgetExceeded,

    /// No route satisfying the configured hop budget was found.
    HopBudgetExceeded,

    /// Checked cost accumulation overflowed the supported numeric type.
    CostOverflow,

    /// The route's hop count cannot be represented by the chosen type.
    HopCountOverflow,

    /// The search expansion count cannot be represented by the chosen type.
    ExpansionCountOverflow,

    /// An injected edge evaluator could not make a valid routing decision.
    EvaluationFailed(String),

    /// The returned route violated a structural invariant.
    InvalidRoutePlan,

    /// The graph snapshot failed structural validation.
    InvalidGraph(GraphError),
}

impl fmt::Display for RoutingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSource => {
                f.write_str("routing source is absent from the graph")
            }
            Self::UnknownDestination => {
                f.write_str("routing destination is absent from the graph")
            }
            Self::NoRoute => {
                f.write_str("no admissible route exists")
            }
            Self::CostBudgetExceeded => {
                f.write_str("route exceeds the configured cost budget")
            }
            Self::SearchBudgetExceeded => {
                f.write_str("route search exceeds its expansion budget")
            }
            Self::HopBudgetExceeded => {
                f.write_str("no route satisfies the configured hop budget")
            }
            Self::CostOverflow => {
                f.write_str("route cost arithmetic overflowed")
            }
            Self::HopCountOverflow => {
                f.write_str("route hop count overflowed")
            }
            Self::ExpansionCountOverflow => {
                f.write_str("route search expansion count overflowed")
            }
            Self::EvaluationFailed(message) => {
                write!(f, "route edge evaluation failed: {message}")
            }
            Self::InvalidRoutePlan => {
                f.write_str("route plan failed structural validation")
            }
            Self::InvalidGraph(error) => {
                write!(f, "invalid routing graph: {error}")
            }
        }
    }
}

impl std::error::Error for RoutingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidGraph(error) => Some(error),
            _ => None,
        }
    }
}

impl From<GraphError> for RoutingError {
    fn from(error: GraphError) -> Self {
        Self::InvalidGraph(error)
    }
}

/// A successfully computed route over one graph snapshot.
///
/// The route is a planning result. It does not reserve bandwidth, establish
/// a connection, authenticate remote peers, or guarantee future availability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutePlan<NodeId> {
    source: NodeId,
    destination: NodeId,
    nodes: Vec<NodeId>,
    total_cost: RoutingCost,
    expansions: usize,
}

impl<NodeId> RoutePlan<NodeId> {
    /// Returns the route source.
    #[must_use]
    pub const fn source(&self) -> &NodeId {
        &self.source
    }

    /// Returns the route destination.
    #[must_use]
    pub const fn destination(&self) -> &NodeId {
        &self.destination
    }

    /// Returns the ordered node sequence, including both endpoints.
    #[must_use]
    pub fn nodes(&self) -> &[NodeId] {
        &self.nodes
    }

    /// Returns the accumulated route cost.
    #[must_use]
    pub const fn total_cost(&self) -> RoutingCost {
        self.total_cost
    }

    /// Returns the number of edges in this route.
    ///
    /// The result is zero when source and destination are identical.
    #[must_use]
    pub fn hops(&self) -> usize {
        self.nodes.len().saturating_sub(1)
    }

    /// Returns the number of expanded search states.
    #[must_use]
    pub const fn expansions(&self) -> usize {
        self.expansions
    }

    /// Checks route structure independently of the original graph.
    ///
    /// The graph and evaluator must still be consulted to establish whether
    /// every edge remains available and its cost remains valid.
    pub fn validate_structure(&self) -> Result<(), RoutingError>
    where
        NodeId: Eq,
    {
        let Some(first) = self.nodes.first() else {
            return Err(RoutingError::InvalidRoutePlan);
        };

        let Some(last) = self.nodes.last() else {
            return Err(RoutingError::InvalidRoutePlan);
        };

        if first != &self.source || last != &self.destination {
            return Err(RoutingError::InvalidRoutePlan);
        }

        if self.nodes.len() == 1 && self.source != self.destination {
            return Err(RoutingError::InvalidRoutePlan);
        }

        Ok(())
    }
}

/// Computes the lowest-cost route under the supplied evaluator and policy.
///
/// The algorithm is Dijkstra's shortest-path algorithm. It supports
/// directed edges, zero-cost edges, and disconnected graphs.
///
/// For a fixed graph and deterministic evaluator, ties are resolved using
/// ordered node IDs and stable path-cost/hop comparisons.
///
/// # Errors
///
/// Returns a typed error for unknown endpoints, invalid graph structure,
/// evaluation failure, exhausted search budgets, arithmetic overflow, or
/// the absence of an admissible route.
///
/// A configured budget is never silently ignored. No partial route is
/// returned as if it were a complete result.
pub fn find_route<NodeId, Evaluator>(
    graph: &RoutingGraph<NodeId>,
    source: &NodeId,
    destination: &NodeId,
    policy: RoutingPolicy,
    evaluator: &Evaluator,
) -> Result<RoutePlan<NodeId>, RoutingError>
where
    NodeId: Clone + Ord,
    Evaluator: RouteEdgeEvaluator<NodeId>,
{
    graph.validate()?;

    if !graph.contains_node(source) {
        return Err(RoutingError::UnknownSource);
    }

    if !graph.contains_node(destination) {
        return Err(RoutingError::UnknownDestination);
    }

    if source == destination {
        let plan = RoutePlan {
            source: source.clone(),
            destination: destination.clone(),
            nodes: vec![source.clone()],
            total_cost: 0,
            expansions: 0,
        };

        plan.validate_structure()?;
        return Ok(plan);
    }

    let budget = policy.budget();

    // Hop limits are included in the search state. Keeping only one
    // distance per node would be incorrect when a lower-cost arrival at
    // that node uses more hops than another admissible arrival.
    let start_state = (source.clone(), 0_usize);

    let mut best: BTreeMap<(NodeId, usize), RoutingCost> = BTreeMap::new();
    let mut predecessor: BTreeMap<(NodeId, usize), (NodeId, usize)> =
        BTreeMap::new();

    // Reverse converts BinaryHeap's max ordering into a min-heap.
    // Cost first, then hop count, then ordered node ID gives deterministic
    // tie-breaking.
    let mut queue: BinaryHeap<Reverse<(RoutingCost, usize, NodeId)>> =
        BinaryHeap::new();

    best.insert(start_state.clone(), 0);
    queue.push(Reverse((0, 0, source.clone())));

    let mut expansions = 0_usize;
    let mut saw_hop_limited_edge = false;
    let mut saw_cost_limited_edge = false;

    while let Some(Reverse((cost, hops, node))) = queue.pop() {
        let state = (node.clone(), hops);

        // Ignore stale queue entries.
        if best.get(&state).copied() != Some(cost) {
            continue;
        }

        if budget
            .max_expansions()
            .is_some_and(|limit| expansions >= limit)
        {
            return Err(RoutingError::SearchBudgetExceeded);
        }

        expansions = expansions
            .checked_add(1)
            .ok_or(RoutingError::ExpansionCountOverflow)?;

        if node == *destination {
            let mut states = vec![state.clone()];
            let mut current = state;

            while current != start_state {
                current = predecessor
                    .get(&current)
                    .cloned()
                    .ok_or(RoutingError::InvalidRoutePlan)?;

                states.push(current.clone());
            }

            states.reverse();

            let nodes: Vec<NodeId> =
                states.into_iter().map(|(id, _)| id).collect();

            let plan = RoutePlan {
                source: source.clone(),
                destination: destination.clone(),
                nodes,
                total_cost: cost,
                expansions,
            };

            plan.validate_structure()?;

            if budget
                .max_total_cost()
                .is_some_and(|limit| cost > limit)
            {
                return Err(RoutingError::CostBudgetExceeded);
            }

            return Ok(plan);
        }

        let edges = graph
            .outgoing_edges(&node)
            .ok_or(RoutingError::InvalidRoutePlan)?;

        for edge in edges {
            let decision = evaluator.evaluate(&node, edge)?;

            let EdgeDecision::Allowed { cost: edge_cost } = decision
            else {
                continue;
            };

            let next_hops = hops
                .checked_add(1)
                .ok_or(RoutingError::HopCountOverflow)?;

            if budget
                .max_hops()
                .is_some_and(|limit| next_hops > limit)
            {
                saw_hop_limited_edge = true;
                continue;
            }

            let next_cost = match cost.checked_add(edge_cost) {
                Some(value) => value,
                None => {
                    // An overflowing candidate is not allowed to wrap
                    // into an apparently cheap route.
                    saw_cost_limited_edge = true;
                    continue;
                }
            };

            if budget
                .max_total_cost()
                .is_some_and(|limit| next_cost > limit)
            {
                saw_cost_limited_edge = true;
                continue;
            }

            let next_state = (edge.destination.clone(), next_hops);

            let improves = best
                .get(&next_state)
                .is_none_or(|existing| next_cost < *existing);

            if improves {
                best.insert(next_state.clone(), next_cost);
                predecessor.insert(next_state, (node.clone(), hops));

                queue.push(Reverse((
                    next_cost,
                    next_hops,
                    edge.destination.clone(),
                )));
            }
        }
    }

    if saw_cost_limited_edge {
        return Err(RoutingError::CostBudgetExceeded);
    }

    if saw_hop_limited_edge {
        return Err(RoutingError::HopBudgetExceeded);
    }

    Err(RoutingError::NoRoute)
}

/// Computes a route using each edge's base cost.
pub fn find_route_by_base_cost<NodeId>(
    graph: &RoutingGraph<NodeId>,
    source: &NodeId,
    destination: &NodeId,
    policy: RoutingPolicy,
) -> Result<RoutePlan<NodeId>, RoutingError>
where
    NodeId: Clone + Ord,
{
    find_route(
        graph,
        source,
        destination,
        policy,
        &BaseCostEvaluator,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug)]
    struct BlockNode(u32);

    struct RejectNode {
        id: u32,
    }

    impl RouteEdgeEvaluator<u32> for RejectNode {
        fn evaluate(
            &self,
            _source: &u32,
            edge: &RouteEdge<u32>,
        ) -> Result<EdgeDecision, RoutingError> {
            if edge.destination == self.id {
                Ok(EdgeDecision::Unavailable)
            } else {
                Ok(EdgeDecision::Allowed {
                    cost: edge.base_cost,
                })
            }
        }
    }

    fn graph() -> RoutingGraph<u32> {
        let mut graph = RoutingGraph::new();

        for node in 1..=5 {
            graph.add_node(node);
        }

        graph.add_edge(&1, 2, 10).unwrap();
        graph.add_edge(&1, 3, 2).unwrap();
        graph.add_edge(&3, 2, 2).unwrap();
        graph.add_edge(&2, 4, 1).unwrap();
        graph.add_edge(&3, 4, 20).unwrap();

        graph
    }

    #[test]
    fn chooses_lowest_cost_route() {
        let graph = graph();

        let route =
            find_route_by_base_cost(&graph, &1, &4, RoutingPolicy::new())
                .unwrap();

        assert_eq!(route.nodes(), &[1, 3, 2, 4]);
        assert_eq!(route.total_cost(), 5);
        assert_eq!(route.hops(), 3);
        route.validate_structure().unwrap();
    }

    #[test]
    fn handles_source_equal_to_destination() {
        let graph = graph();

        let route =
            find_route_by_base_cost(&graph, &2, &2, RoutingPolicy::new())
                .unwrap();

        assert_eq!(route.nodes(), &[2]);
        assert_eq!(route.total_cost(), 0);
        assert_eq!(route.hops(), 0);
    }

    #[test]
    fn rejects_unknown_endpoints() {
        let graph = graph();

        assert_eq!(
            find_route_by_base_cost(&graph, &99, &2, RoutingPolicy::new())
                .unwrap_err(),
            RoutingError::UnknownSource
        );

        assert_eq!(
            find_route_by_base_cost(&graph, &1, &99, RoutingPolicy::new())
                .unwrap_err(),
            RoutingError::UnknownDestination
        );
    }

    #[test]
    fn reports_disconnected_destinations() {
        let graph = graph();

        assert_eq!(
            find_route_by_base_cost(&graph, &1, &5, RoutingPolicy::new())
                .unwrap_err(),
            RoutingError::NoRoute
        );
    }

    #[test]
    fn rejects_duplicate_edges() {
        let mut graph = RoutingGraph::new();
        graph.add_node(1);
        graph.add_node(2);

        graph.add_edge(&1, 2, 4).unwrap();

        assert_eq!(
            graph.add_edge(&1, 2, 8).unwrap_err(),
            GraphError::DuplicateEdge
        );
    }

    #[test]
    fn supports_bidirectional_edges() {
        let mut graph = RoutingGraph::new();
        graph.add_node(1);
        graph.add_node(2);

        graph.add_bidirectional_edge(&1, &2, 7).unwrap();

        let forward =
            find_route_by_base_cost(&graph, &1, &2, RoutingPolicy::new())
                .unwrap();

        let reverse =
            find_route_by_base_cost(&graph, &2, &1, RoutingPolicy::new())
                .unwrap();

        assert_eq!(forward.total_cost(), 7);
        assert_eq!(reverse.total_cost(), 7);
    }

    #[test]
    fn evaluator_can_reject_an_edge() {
        let graph = graph();
        let evaluator = RejectNode { id: 2 };

        let route = find_route(
            &graph,
            &1,
            &4,
            RoutingPolicy::new(),
            &evaluator,
        )
        .unwrap();

        assert_eq!(route.nodes(), &[1, 3, 4]);
        assert_eq!(route.total_cost(), 22);
    }

    #[test]
    fn enforces_hop_budget() {
        let graph = graph();

        let policy = RoutingPolicy::new().with_budget(
            RoutingBudget::unlimited().with_max_hops(Some(2)),
        );

        assert_eq!(
            find_route_by_base_cost(&graph, &1, &4, policy).unwrap_err(),
            RoutingError::HopBudgetExceeded
        );
    }

    #[test]
    fn enforces_total_cost_budget() {
        let graph = graph();

        let policy = RoutingPolicy::new().with_budget(
            RoutingBudget::unlimited().with_max_total_cost(Some(4)),
        );

        assert_eq!(
            find_route_by_base_cost(&graph, &1, &4, policy).unwrap_err(),
            RoutingError::CostBudgetExceeded
        );
    }

    #[test]
    fn enforces_expansion_budget() {
        let graph = graph();

        let policy = RoutingPolicy::new().with_budget(
            RoutingBudget::unlimited().with_max_expansions(Some(1)),
        );

        assert_eq!(
            find_route_by_base_cost(&graph, &1, &4, policy).unwrap_err(),
            RoutingError::SearchBudgetExceeded
        );
    }

    #[test]
    fn detects_cost_overflow_without_wrapping() {
        let mut graph = RoutingGraph::new();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        graph.add_edge(&1, 2, u128::MAX).unwrap();
        graph.add_edge(&2, 3, 1).unwrap();

        assert_eq!(
            find_route_by_base_cost(&graph, &1, &3, RoutingPolicy::new())
                .unwrap_err(),
            RoutingError::CostBudgetExceeded
        );
    }

    #[test]
    fn deterministic_for_identical_graphs() {
        let mut graph = RoutingGraph::new();

        for node in [1, 2, 3, 4] {
            graph.add_node(node);
        }

        graph.add_edge(&1, 3, 1).unwrap();
        graph.add_edge(&1, 2, 1).unwrap();
        graph.add_edge(&2, 4, 1).unwrap();
        graph.add_edge(&3, 4, 1).unwrap();

        let first =
            find_route_by_base_cost(&graph, &1, &4, RoutingPolicy::new())
                .unwrap();

        let second =
            find_route_by_base_cost(&graph, &1, &4, RoutingPolicy::new())
                .unwrap();

        assert_eq!(first, second);
    }
}
