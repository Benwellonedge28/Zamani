
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Cycle Analysis
//!
//! Path: `src/hdl/analysis/cycles.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: safe Rust only.
//!
//! ## Purpose
//!
//! Analyze the existing HDL dependency graph and identify cyclic strongly
//! connected components. Produce deterministic cycle witnesses without
//! recursive graph traversal.
//!
//! ## Authority and ownership
//!
//! - `grammar/spec/hdl.md` owns normative HDL semantics.
//! - `grammar/hdl/hdl.g4` owns HDL grammar composition.
//! - `grammar/hdl/combinational.g4` owns combinational source syntax.
//! - `grammar/hdl/processes.g4` owns process source syntax.
//! - `frontend::semantic::domains::hdl` owns normalized HDL semantic inputs.
//! - `analysis/dependencies.rs` owns the canonical dependency graph.
//!
//! This module does not introduce another graph, AST, semantic model,
//! diagnostic registry, source-span type, or intermediate representation.
//!
//! ## Edge direction
//!
//! An edge `A -> B` means that computing A depends on B.
//!
//! A cycle is a directed path that returns to its starting node. A
//! strongly connected component is cyclic when it contains multiple nodes
//! or when its single node has an edge to itself.
//!
//! ## Incomplete analysis
//!
//! Configured node, edge, and work budgets are operational policies, not
//! HDL language limits. A budget exhaustion is reported explicitly.
//! Partial findings must never be treated as proof that the graph is
//! acyclic.
//!
//! ## Integration
//!
//! The caller supplies a `DependencyGraph`, normally built by
//! `analysis/dependencies.rs`. This module returns cyclic components,
//! deterministic witnesses when available, and completion status.
//!
//! Source-level semantic validation remains owned by `validate_hdl()`.
//! This module does not independently decide whether a cycle is legal in
//! every HDL construct; the owning semantic rule determines that.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Scalability
//!
//! There are no fixed graph-size, signal-count, or graph-depth ceilings.
//! Traversals are iterative, and storage grows with the graph and findings.
//! Actual completion remains bounded by available memory, execution time,
//! and any caller-configured budgets.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::frontend::semantic::domains::hdl::HdlId;

use super::dependencies::{
    AnalysisStatus, DependencyAnalysisConfig, DependencyCycle, DependencyGraph,
    IncompleteReason,
};

/// A cyclic strongly connected component.
///
/// Members are sorted by canonical HDL identity. A component is a maximal
/// group of nodes in which every node can reach every other node.
///
/// `witness` contains one directed cycle within the component when witness
/// construction completed. It can be absent if the work budget was exhausted
/// after the component had already been identified.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CyclicComponent {
    /// Sorted identities belonging to this cyclic component.
    pub members: Vec<HdlId>,

    /// One directed cycle, including the repeated closing identity.
    ///
    /// A self-loop has the form `[A, A]`.
    pub witness: Option<DependencyCycle>,
}

/// Result of cycle analysis.
///
/// When `status` is incomplete, the returned components are findings already
/// established, not an exhaustive list. An empty component list in an
/// incomplete result does not prove that the graph is acyclic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CycleAnalysis {
    /// Cyclic components discovered so far, in deterministic order.
    pub components: Vec<CyclicComponent>,

    /// Completion state shared with dependency analysis.
    pub status: AnalysisStatus,

    /// Number of work units consumed by this analysis.
    pub work_units: usize,
}

impl CycleAnalysis {
    /// Returns true only when analysis completed and no cycles exist.
    #[must_use]
    pub fn is_acyclic_and_complete(&self) -> bool {
        self.status.is_complete() && self.components.is_empty()
    }

    /// Returns true if at least one cyclic component has been established.
    ///
    /// This can be true even when the overall analysis is incomplete.
    #[must_use]
    pub fn has_cycles(&self) -> bool {
        !self.components.is_empty()
    }

    /// Returns true if all requested analysis completed.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.status.is_complete()
    }

    /// Returns all available cycle witnesses in component order.
    ///
    /// Components whose witness could not be constructed because of a
    /// budget limit are omitted from this iterator.
    pub fn witnesses(&self) -> impl Iterator<Item = &DependencyCycle> {
        self.components.iter().filter_map(|component| {
            component.witness.as_ref()
        })
    }
}

/// Analyze a dependency graph for cyclic strongly connected components.
///
/// The algorithm uses iterative Kosaraju SCC discovery. The graph's existing
/// node and dependency iterators are the only graph interface consumed here.
///
/// # Determinism
///
/// Ordered collections determine traversal order, component membership order,
/// component ordering, and witness selection. Equivalent graphs therefore
/// produce equivalent results regardless of their construction order.
///
/// # Budgets
///
/// `max_nodes` and `max_edges` are maximum accepted graph sizes for this
/// analysis. `max_work` limits deterministic node/edge processing work.
/// `None` means no caller-imposed limit for that dimension.
///
/// Work accounting includes reverse-graph construction, SCC traversal, and
/// cycle-witness construction. It is not a wall-clock or memory guarantee.
///
/// # Incomplete results
///
/// If a configured budget is exhausted, the result status is incomplete.
/// Components already established are retained, but callers must not
/// interpret the result as exhaustive.
///
/// # Complexity
///
/// Without budget interruption, time and storage are O(V + E), apart from
/// ordered-container lookup costs and the output size, where V is the number
/// of nodes and E is the number of edges.
///
/// # Examples
///
/// ```
/// use crate::frontend::semantic::domains::hdl::HdlId;
/// use crate::hdl::analysis::cycles::analyze_cycles;
/// use crate::hdl::analysis::dependencies::{
///     DependencyAnalysisConfig, DependencyGraph,
/// };
///
/// let a = HdlId::new("a").expect("valid identifier");
/// let b = HdlId::new("b").expect("valid identifier");
///
/// let mut graph = DependencyGraph::new();
/// graph.add_edge(a.clone(), b.clone());
/// graph.add_edge(b, a);
///
/// let result = analyze_cycles(&graph, &DependencyAnalysisConfig::default());
///
/// assert!(result.is_complete());
/// assert!(result.has_cycles());
/// ```
#[must_use]
pub fn analyze_cycles(
    graph: &DependencyGraph,
    config: &DependencyAnalysisConfig,
) -> CycleAnalysis {
    if config.max_nodes.is_some_and(|limit| graph.node_count() > limit) {
        return incomplete(IncompleteReason::NodeBudgetExceeded, 0);
    }

    if config.max_edges.is_some_and(|limit| graph.edge_count() > limit) {
        return incomplete(IncompleteReason::EdgeBudgetExceeded, 0);
    }

    let mut work = 0_usize;

    // Pass 1: construct the reverse graph. Its key set includes every node,
    // including isolated nodes, so the SCC traversal sees the full graph.
    let mut reverse = BTreeMap::<HdlId, BTreeSet<HdlId>>::new();

    for node in graph.nodes() {
        if !charge(config, &mut work) {
            return incomplete(IncompleteReason::WorkBudgetExceeded, work);
        }

        reverse.entry(node.clone()).or_default();
    }

    for (source, dependency) in graph.edges() {
        if !charge(config, &mut work) {
            return incomplete(IncompleteReason::WorkBudgetExceeded, work);
        }

        reverse
            .entry(dependency.clone())
            .or_default()
            .insert(source.clone());
    }

    // Pass 2: iterative DFS on the original graph to obtain finishing order.
    // Explicit frames prevent call-stack growth with graph depth.
    struct Frame<'a> {
        node: &'a HdlId,
        neighbors: Vec<&'a HdlId>,
        next: usize,
    }

    let mut visited = BTreeSet::<HdlId>::new();
    let mut finish_order = Vec::<HdlId>::new();

    for root in graph.nodes() {
        if visited.contains(root) {
            continue;
        }

        if !charge(config, &mut work) {
            return incomplete(IncompleteReason::WorkBudgetExceeded, work);
        }

        visited.insert(root.clone());

        let neighbors = graph
            .dependencies(root)
            .map(|items| items.collect::<Vec<_>>())
            .unwrap_or_default();

        let mut stack = vec![Frame {
            node: root,
            neighbors,
            next: 0,
        }];

        while let Some(frame) = stack.last_mut() {
            if frame.next >= frame.neighbors.len() {
                let completed = frame.node.clone();
                stack.pop();
                finish_order.push(completed);
                continue;
            }

            if !charge(config, &mut work) {
                return incomplete(IncompleteReason::WorkBudgetExceeded, work);
            }

            let neighbor = frame.neighbors[frame.next];
            frame.next += 1;

            if visited.insert(neighbor.clone()) {
                let neighbors = graph
                    .dependencies(neighbor)
                    .map(|items| items.collect::<Vec<_>>())
                    .unwrap_or_default();

                if !charge(config, &mut work) {
                    return incomplete(
                        IncompleteReason::WorkBudgetExceeded,
                        work,
                    );
                }

                stack.push(Frame {
                    node: neighbor,
                    neighbors,
                    next: 0,
                });
            }
        }
    }

    // Pass 3: traverse the reverse graph in decreasing finishing order.
    // Each traversal yields one complete SCC.
    let mut assigned = BTreeSet::<HdlId>::new();
    let mut components = Vec::<CyclicComponent>::new();

    for start in finish_order.into_iter().rev() {
        if assigned.contains(&start) {
            continue;
        }

        if !charge(config, &mut work) {
            return incomplete_with(
                components,
                IncompleteReason::WorkBudgetExceeded,
                work,
            );
        }

        assigned.insert(start.clone());

        let mut pending = vec![start.clone()];
        let mut members = Vec::<HdlId>::new();

        while let Some(node) = pending.pop() {
            members.push(node.clone());

            if let Some(neighbors) = reverse.get(&node) {
                for neighbor in neighbors.iter().rev() {
                    if !charge(config, &mut work) {
                        return incomplete_with(
                            components,
                            IncompleteReason::WorkBudgetExceeded,
                            work,
                        );
                    }

                    if assigned.insert(neighbor.clone()) {
                        pending.push(neighbor.clone());
                    }
                }
            }
        }

        members.sort();

        let cyclic = members.len() > 1
            || graph
                .dependencies(&members[0])
                .is_some_and(|items| items.any(|id| id == &members[0]));

        if !cyclic {
            continue;
        }

        let witness = match find_witness(graph, &members, config, &mut work) {
            WitnessResult::Found(witness) => Some(witness),
            WitnessResult::NotFound => {
                // A cyclic SCC mathematically contains a cycle. This branch
                // protects the API against inconsistent graph interfaces.
                return incomplete_with(
                    components,
                    IncompleteReason::WorkBudgetExceeded,
                    work,
                );
            }
            WitnessResult::BudgetExceeded => {
                components.push(CyclicComponent { members, witness: None });

                components.sort_by(|left, right| left.members.cmp(&right.members));

                return incomplete_with(
                    components,
                    IncompleteReason::WorkBudgetExceeded,
                    work,
                );
            }
        };

        components.push(CyclicComponent { members, witness });
    }

    components.sort_by(|left, right| left.members.cmp(&right.members));

    CycleAnalysis {
        components,
        status: AnalysisStatus::Complete,
        work_units: work,
    }
}

enum WitnessResult {
    Found(DependencyCycle),
    NotFound,
    BudgetExceeded,
}

/// Find one deterministic directed cycle inside a known cyclic SCC.
///
/// Starting from the smallest identity, the search considers sorted outgoing
/// edges and finds a path back to the start. A strongly connected component
/// guarantees such a path for any member.
fn find_witness(
    graph: &DependencyGraph,
    members: &[HdlId],
    config: &DependencyAnalysisConfig,
    work: &mut usize,
) -> WitnessResult {
    let Some(start) = members.first() else {
        return WitnessResult::NotFound;
    };

    let member_set = members.iter().cloned().collect::<BTreeSet<_>>();

    let Some(outgoing) = graph.dependencies(start) else {
        return WitnessResult::NotFound;
    };

    let outgoing = outgoing
        .filter(|node| member_set.contains(*node))
        .cloned()
        .collect::<Vec<_>>();

    for first in outgoing {
        if !charge(config, work) {
            return WitnessResult::BudgetExceeded;
        }

        if &first == start {
            return WitnessResult::Found(DependencyCycle {
                path: vec![start.clone(), start.clone()],
            });
        }

        // Breadth-first search makes the first shortest witness deterministic.
        let mut queue = VecDeque::new();
        let mut predecessor = BTreeMap::<HdlId, HdlId>::new();

        predecessor.insert(first.clone(), start.clone());
        queue.push_back(first.clone());

        let mut found = false;

        while let Some(node) = queue.pop_front() {
            if !charge(config, work) {
                return WitnessResult::BudgetExceeded;
            }

            let Some(neighbors) = graph.dependencies(&node) else {
                continue;
            };

            for neighbor in neighbors {
                if !member_set.contains(neighbor) {
                    continue;
                }

                if !charge(config, work) {
                    return WitnessResult::BudgetExceeded;
                }

                if neighbor == start {
                    predecessor.insert(start.clone(), node.clone());
                    found = true;
                    break;
                }

                if !predecessor.contains_key(neighbor) {
                    predecessor.insert(neighbor.clone(), node.clone());
                    queue.push_back(neighbor.clone());
                }
            }

            if found {
                break;
            }
        }

        if !found {
            continue;
        }

        // Reconstruct the path from `first` back to `start`.
        let mut reverse_path = vec![start.clone()];
        let mut current = predecessor.get(start).cloned();

        while let Some(node) = current {
            reverse_path.push(node.clone());

            if node == first {
                break;
            }

            current = predecessor.get(&node).cloned();
        }

        if reverse_path.last() != Some(&first) {
            return WitnessResult::NotFound;
        }

        reverse_path.reverse();

        let mut path = Vec::with_capacity(reverse_path.len() + 1);
        path.push(start.clone());
        path.extend(reverse_path);
        path.push(start.clone());

        return WitnessResult::Found(DependencyCycle { path });
    }

    WitnessResult::NotFound
}

fn charge(
    config: &DependencyAnalysisConfig,
    work: &mut usize,
) -> bool {
    if config.max_work.is_some_and(|limit| *work >= limit) {
        return false;
    }

    *work = work.saturating_add(1);
    true
}

fn incomplete(
    reason: IncompleteReason,
    work_units: usize,
) -> CycleAnalysis {
    incomplete_with(Vec::new(), reason, work_units)
}

fn incomplete_with(
    components: Vec<CyclicComponent>,
    reason: IncompleteReason,
    work_units: usize,
) -> CycleAnalysis {
    CycleAnalysis {
        components,
        status: AnalysisStatus::Incomplete(reason),
        work_units,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test identity must be valid")
    }

    fn graph_with_edges(edges: &[(&str, &str)]) -> DependencyGraph {
        let mut graph = DependencyGraph::new();

        for (source, dependency) in edges {
            graph.add_edge(id(source), id(dependency));
        }

        graph
    }

    #[test]
    fn empty_graph_is_acyclic_and_complete() {
        let graph = DependencyGraph::new();
        let result =
            analyze_cycles(&graph, &DependencyAnalysisConfig::default());

        assert!(result.is_complete());
        assert!(result.is_acyclic_and_complete());
        assert!(result.components.is_empty());
    }

    #[test]
    fn acyclic_chain_has_no_cyclic_components() {
        let graph = graph_with_edges(&[
            ("output", "middle"),
            ("middle", "input"),
        ]);

        let result =
            analyze_cycles(&graph, &DependencyAnalysisConfig::default());

        assert!(result.is_acyclic_and_complete());
        assert!(result.components.is_empty());
    }

    #[test]
    fn detects_a_self_loop() {
        let graph = graph_with_edges(&[("signal", "signal")]);

        let result =
            analyze_cycles(&graph, &DependencyAnalysisConfig::default());

        assert!(result.is_complete());
        assert_eq!(result.components.len(), 1);
        assert_eq!(result.components[0].members, vec![id("signal")]);
        assert_eq!(
            result.components[0].witness.as_ref().map(|w| &w.path),
            Some(&vec![id("signal"), id("signal")])
        );
    }

    #[test]
    fn detects_a_multi_node_cycle_with_a_closed_witness() {
        let graph = graph_with_edges(&[
            ("a", "b"),
            ("b", "c"),
            ("c", "a"),
        ]);

        let result =
            analyze_cycles(&graph, &DependencyAnalysisConfig::default());

        assert!(result.is_complete());
        assert_eq!(result.components.len(), 1);

        let witness = result.components[0]
            .witness
            .as_ref()
            .expect("completed analysis should have a witness");

        assert_eq!(witness.path.first(), witness.path.last());
        assert_eq!(witness.path.len(), 4);
    }

    #[test]
    fn separates_independent_cyclic_components() {
        let graph = graph_with_edges(&[
            ("a", "b"),
            ("b", "a"),
            ("x", "y"),
            ("y", "x"),
            ("tail", "a"),
        ]);

        let result =
            analyze_cycles(&graph, &DependencyAnalysisConfig::default());

        assert!(result.is_complete());
        assert_eq!(result.components.len(), 2);
        assert_eq!(
            result.components[0].members,
            vec![id("a"), id("b")]
        );
        assert_eq!(
            result.components[1].members,
            vec![id("x"), id("y")]
        );
    }

    #[test]
    fn respects_node_budget_without_claiming_acyclicity() {
        let graph = graph_with_edges(&[("a", "b"), ("b", "a")]);

        let config = DependencyAnalysisConfig {
            max_nodes: Some(1),
            ..DependencyAnalysisConfig::default()
        };

        let result = analyze_cycles(&graph, &config);

        assert!(!result.is_complete());
        assert!(!result.is_acyclic_and_complete());
        assert_eq!(
            result.status,
            AnalysisStatus::Incomplete(IncompleteReason::NodeBudgetExceeded)
        );
    }

    #[test]
    fn respects_edge_budget_without_claiming_acyclicity() {
        let graph = graph_with_edges(&[("a", "b"), ("b", "a")]);

        let config = DependencyAnalysisConfig {
            max_edges: Some(1),
            ..DependencyAnalysisConfig::default()
        };

        let result = analyze_cycles(&graph, &config);

        assert!(!result.is_complete());
        assert!(!result.is_acyclic_and_complete());
        assert_eq!(
            result.status,
            AnalysisStatus::Incomplete(IncompleteReason::EdgeBudgetExceeded)
        );
    }

    #[test]
    fn zero_work_budget_is_explicitly_incomplete() {
        let graph = graph_with_edges(&[("a", "a")]);

        let config = DependencyAnalysisConfig {
            max_work: Some(0),
            ..DependencyAnalysisConfig::default()
        };

        let result = analyze_cycles(&graph, &config);

        assert!(!result.is_complete());
        assert!(!result.is_acyclic_and_complete());
        assert_eq!(
            result.status,
            AnalysisStatus::Incomplete(IncompleteReason::WorkBudgetExceeded)
        );
    }

    #[test]
    fn graph_construction_order_does_not_change_results() {
        let first = graph_with_edges(&[
            ("a", "b"),
            ("b", "c"),
            ("c", "a"),
            ("x", "y"),
            ("y", "x"),
        ]);

        let second = graph_with_edges(&[
            ("y", "x"),
            ("c", "a"),
            ("x", "y"),
            ("b", "c"),
            ("a", "b"),
        ]);

        let config = DependencyAnalysisConfig::default();
        let left = analyze_cycles(&first, &config);
        let right = analyze_cycles(&second, &config);

        assert_eq!(left.components, right.components);
        assert_eq!(left.status, right.status);
        assert_eq!(left.work_units, right.work_units);
    }

    #[test]
    fn long_graph_does_not_require_recursive_traversal() {
        // This is a stress-test sample, not a language or graph-size limit.
        let mut graph = DependencyGraph::new();
        let count = 2_000_usize;

        for index in 0..count {
            graph.add_edge(id(&format!("n{index}")), id(&format!("n{}", index + 1)));
        }

        let config = DependencyAnalysisConfig::default();
        let result = analyze_cycles(&graph, &config);

        assert!(result.is_complete());
        assert!(result.is_acyclic_and_complete());
    }
}
