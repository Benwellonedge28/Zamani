#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # HDL Dependency Analysis
//!
//! Path: `src/hdl/analysis/dependencies.rs`
//! Rust: 1.97+
//! Edition: 2021
//! Safety: safe Rust only.
//!
//! ## Purpose
//!
//! Build a deterministic, target-independent dependency graph from the
//! repository's existing HDL semantic input contract. Detect combinational
//! dependency cycles without recursive graph traversal.
//!
//! ## Authority and ownership
//!
//! Source syntax is owned by `grammar/hdl/` and parsed through the canonical
//! frontend. HDL semantics are validated by
//! `crate::frontend::semantic::domains::hdl`.
//!
//! This module reuses `HdlId`, `HdlProgram`, `HdlProcess`, `HdlAssignment`,
//! and `HdlProcessKind`. It does not define another AST, semantic model,
//! source-span representation, or intermediate representation.
//!
//! ## Edge direction
//!
//! An edge `A -> B` means that a combinational assignment to signal `A`
//! reads signal `B`. Sequential process assignments are excluded from the
//! combinational graph: a register/state boundary must not be mistaken for
//! a combinational feedback cycle.
//!
//! ## Incomplete analysis
//!
//! Every operational budget is optional and caller-configured. No universal
//! signal, edge, width, hierarchy, or graph-size ceiling is imposed.
//! If a budget prevents completion, the result explicitly says that it is
//! incomplete. A partial result MUST NOT be treated as proof of cycle
//! absence.
//!
//! ## Integration
//!
//! - Input: canonical semantic `HdlProgram`.
//! - Output: dependency graph, cycle witnesses, and completion status.
//! - Existing `validate_hdl()` remains the semantic-validation authority.
//! - `analysis/cycles.rs` may consume this graph later rather than
//!   implementing another dependency-graph representation.
//! - Target feasibility and realization may consume completed analysis
//!   results, but are not dependencies of this module.
//! - `src/hdl/hdlir/` remains excluded from this implementation.
//!
//! ## Determinism
//!
//! Ordered collections make graph nodes, edges, and cycle witnesses stable
//! for equivalent semantic inputs. Process input order does not affect the
//! graph's canonical representation.
//!
//! ## Complexity
//!
//! Graph construction is O(V + E) expected to ordered-container factors;
//! traversal is O(V + E) expected to ordered-container factors. Memory use
//! is O(V + E). Actual completion depends on available resources and the
//! caller's configured budgets.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::frontend::semantic::domains::hdl::{
    HdlId, HdlProcessKind, HdlProgram,
};

/// Operational budgets for dependency graph construction and traversal.
///
/// `None` means that this analysis does not impose a limit for that
/// dimension. It does not guarantee that the host has unlimited memory,
/// time, or other resources.
///
/// These budgets constrain analysis work, not the semantics or legal size
/// of an HDL design.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DependencyAnalysisConfig {
    /// Maximum number of distinct graph nodes to admit.
    pub max_nodes: Option<usize>,
    /// Maximum number of distinct dependency edges to admit.
    pub max_edges: Option<usize>,
    /// Maximum number of graph-processing work units to perform.
    ///
    /// A work unit is charged for a node or edge examined by the analysis.
    /// The exact accounting is deterministic for a given canonical graph.
    pub max_work: Option<usize>,
}

/// Why a dependency analysis did not finish.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum IncompleteReason {
    /// The node budget was reached before all nodes could be admitted.
    NodeBudgetExceeded,
    /// The edge budget was reached before all edges could be admitted.
    EdgeBudgetExceeded,
    /// The work budget was reached before the analysis could finish.
    WorkBudgetExceeded,
}

/// Completion state of dependency analysis.
///
/// `Complete` means all graph construction and cycle traversal requested by
/// this API finished. `Incomplete` means the graph and findings may be
/// partial and cannot establish the absence of cycles.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnalysisStatus {
    /// Graph construction and cycle traversal completed.
    Complete,
    /// Analysis stopped because a configured operational budget was reached.
    Incomplete(IncompleteReason),
}

impl AnalysisStatus {
    /// Returns true only when the requested analysis completed.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// A directed dependency graph over canonical HDL signal identities.
///
/// An edge `source -> dependency` means the source signal's computed value
/// reads the dependency signal. Nodes and edges are kept in deterministic
/// order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DependencyGraph {
    edges: BTreeMap<HdlId, BTreeSet<HdlId>>,
}

impl DependencyGraph {
    /// Creates an empty dependency graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a node even when it has no outgoing dependencies.
    pub fn add_node(&mut self, id: HdlId) {
        self.edges.entry(id).or_default();
    }

    /// Adds a directed dependency edge.
    ///
    /// Both endpoints are admitted as nodes. Returns true if the edge was
    /// newly inserted and false if the edge already existed.
    pub fn add_edge(&mut self, source: HdlId, dependency: HdlId) -> bool {
        self.edges.entry(dependency.clone()).or_default();
        self.edges
            .entry(source)
            .or_default()
            .insert(dependency)
    }

    /// Returns the number of distinct graph nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns the number of distinct graph edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.values().map(BTreeSet::len).sum()
    }

    /// Returns true when the graph contains no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    /// Returns the dependencies of a node in deterministic order.
    ///
    /// An unknown node returns `None`. A known node without dependencies
    /// returns `Some` with an empty iterator.
    pub fn dependencies(
        &self,
        id: &HdlId,
    ) -> Option<impl Iterator<Item = &HdlId>> {
        self.edges.get(id).map(|dependencies| dependencies.iter())
    }

    /// Returns all nodes in deterministic order.
    pub fn nodes(&self) -> impl Iterator<Item = &HdlId> {
        self.edges.keys()
    }

    /// Returns all edges in deterministic order.
    pub fn edges(&self) -> impl Iterator<Item = (&HdlId, &HdlId)> {
        self.edges
            .iter()
            .flat_map(|(source, dependencies)| {
                dependencies.iter().map(move |dependency| (source, dependency))
            })
    }
}

/// A concrete cycle witness in a directed dependency graph.
///
/// The first node is repeated at the end, so consecutive elements describe
/// edges and the final edge closes the cycle. A self-dependency is
/// represented as `[A, A]`.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DependencyCycle {
    /// Ordered signal identities forming the cycle, including the repeated
    /// closing identity.
    pub path: Vec<HdlId>,
}

/// Result of building and analyzing an HDL dependency graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyAnalysis {
    /// Graph constructed from the semantic input.
    ///
    /// If `status` is incomplete, this graph may be partial.
    pub graph: DependencyGraph,
    /// Cycle witnesses discovered before traversal completed or stopped.
    ///
    /// If `status` is incomplete, additional cycles may remain undiscovered.
    pub cycles: Vec<DependencyCycle>,
    /// Whether graph construction and cycle traversal completed.
    pub status: AnalysisStatus,
}

impl DependencyAnalysis {
    /// Returns true only if analysis completed and no cycles were found.
    ///
    /// `false` does not necessarily mean a cycle exists: the analysis may
    /// instead be incomplete.
    #[must_use]
    pub fn is_acyclic_and_complete(&self) -> bool {
        self.status.is_complete() && self.cycles.is_empty()
    }
}

/// Errors in an explicitly constructed dependency graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DependencyGraphError {
    /// A cycle path was empty or did not repeat its first node at the end.
    InvalidCycleWitness,
}

impl fmt::Display for DependencyGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCycleWitness => {
                f.write_str("invalid dependency cycle witness")
            }
        }
    }
}

impl std::error::Error for DependencyGraphError {}

/// Builds a dependency graph and discovers combinational cycles.
///
/// Only assignments belonging to combinational processes are included.
/// Sequential process assignments are excluded because clocked state
/// boundaries break combinational dependency paths.
///
/// Unknown signal references are retained as graph nodes so this function
/// remains a graph analysis rather than a replacement semantic validator.
/// Call `validate_hdl()` for semantic diagnostics such as undeclared
/// references, invalid process configurations, and multiple drivers.
///
/// If a budget is reached, the returned status is incomplete. Partial
/// findings remain useful for diagnostics but must not be interpreted as
/// a proof of cycle absence.
#[must_use]
pub fn analyze_dependencies(
    program: &HdlProgram,
    config: &DependencyAnalysisConfig,
) -> DependencyAnalysis {
    let mut graph = DependencyGraph::new();
    let mut work = 0_usize;

    // Admit declared signals first, preserving isolated signals.
    for signal in &program.signals {
        if budget_reached(config.max_work, work) {
            return incomplete(graph, Vec::new(), IncompleteReason::WorkBudgetExceeded);
        }
        work = work.saturating_add(1);

        if !graph.edges.contains_key(&signal.id)
            && limit_reached(config.max_nodes, graph.node_count())
        {
            return incomplete(graph, Vec::new(), IncompleteReason::NodeBudgetExceeded);
        }
        graph.add_node(signal.id.clone());
    }

    // Only combinational process assignments create combinational edges.
    // Duplicate edges are naturally collapsed by the ordered set.
    for process in &program.processes {
        if process.kind != HdlProcessKind::Combinational {
            continue;
        }

        for assignment in &process.assignments {
            if budget_reached(config.max_work, work) {
                return incomplete(
                    graph,
                    Vec::new(),
                    IncompleteReason::WorkBudgetExceeded,
                );
            }
            work = work.saturating_add(1);

            let new_source = !graph.edges.contains_key(&assignment.target);
            let missing_targets = assignment
                .reads
                .iter()
                .filter(|read| !graph.edges.contains_key(*read))
                .collect::<BTreeSet<_>>();

            let additional_nodes = usize::from(new_source)
                .saturating_add(missing_targets.len());

            if limit_would_be_exceeded(
                config.max_nodes,
                graph.node_count(),
                additional_nodes,
            ) {
                return incomplete(
                    graph,
                    Vec::new(),
                    IncompleteReason::NodeBudgetExceeded,
                );
            }

            let new_edges = assignment
                .reads
                .iter()
                .filter(|read| {
                    !graph
                        .edges
                        .get(&assignment.target)
                        .is_some_and(|dependencies| dependencies.contains(*read))
                })
                .count();

            if limit_would_be_exceeded(
                config.max_edges,
                graph.edge_count(),
                new_edges,
            ) {
                return incomplete(
                    graph,
                    Vec::new(),
                    IncompleteReason::EdgeBudgetExceeded,
                );
            }

            graph.add_node(assignment.target.clone());
            for read in &assignment.reads {
                graph.add_edge(assignment.target.clone(), read.clone());
            }
        }
    }

    let (cycles, status) = find_cycles(&graph, config, &mut work);

    DependencyAnalysis {
        graph,
        cycles,
        status,
    }
}

fn limit_reached(limit: Option<usize>, current: usize) -> bool {
    limit.is_some_and(|maximum| current >= maximum)
}

fn limit_would_be_exceeded(
    limit: Option<usize>,
    current: usize,
    additional: usize,
) -> bool {
    limit.is_some_and(|maximum| {
        additional > maximum.saturating_sub(current)
    })
}

fn budget_reached(limit: Option<usize>, work: usize) -> bool {
    limit.is_some_and(|maximum| work >= maximum)
}

fn incomplete(
    graph: DependencyGraph,
    cycles: Vec<DependencyCycle>,
    reason: IncompleteReason,
) -> DependencyAnalysis {
    DependencyAnalysis {
        graph,
        cycles,
        status: AnalysisStatus::Incomplete(reason),
    }
}

/// Iterative depth-first cycle discovery.
///
/// Three colors are used: absent/white (unvisited), gray (on the active DFS
/// path), and black (finished). An edge to a gray node identifies a cycle.
/// Explicit frames avoid call-stack growth proportional to graph depth.
fn find_cycles(
    graph: &DependencyGraph,
    config: &DependencyAnalysisConfig,
    work: &mut usize,
) -> (Vec<DependencyCycle>, AnalysisStatus) {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Color {
        Gray,
        Black,
    }

    struct Frame<'a> {
        node: &'a HdlId,
        next_dependency: usize,
        dependencies: Vec<&'a HdlId>,
    }

    let mut colors = BTreeMap::<HdlId, Color>::new();
    let mut active_path = Vec::<HdlId>::new();
    let mut active_positions = BTreeMap::<HdlId, usize>::new();
    let mut cycles = BTreeSet::<DependencyCycle>::new();

    for root in graph.nodes() {
        if colors.contains_key(root) {
            continue;
        }

        if budget_reached(config.max_work, *work) {
            return (
                cycles.into_iter().collect(),
                AnalysisStatus::Incomplete(IncompleteReason::WorkBudgetExceeded),
            );
        }
        *work = work.saturating_add(1);

        colors.insert(root.clone(), Color::Gray);
        active_positions.insert(root.clone(), active_path.len());
        active_path.push(root.clone());

        let root_dependencies = graph
            .edges
            .get(root)
            .map(|items| items.iter().collect())
            .unwrap_or_default();

        let mut stack = vec![Frame {
            node: root,
            next_dependency: 0,
            dependencies: root_dependencies,
        }];

        while let Some(frame) = stack.last_mut() {
            if frame.next_dependency >= frame.dependencies.len() {
                let finished = frame.node.clone();
                stack.pop();
                colors.insert(finished.clone(), Color::Black);
                active_positions.remove(&finished);
                let _ = active_path.pop();
                continue;
            }

            if budget_reached(config.max_work, *work) {
                return (
                    cycles.into_iter().collect(),
                    AnalysisStatus::Incomplete(
                        IncompleteReason::WorkBudgetExceeded,
                    ),
                );
            }
            *work = work.saturating_add(1);

            let dependency = frame.dependencies[frame.next_dependency];
            frame.next_dependency += 1;

            match colors.get(dependency).copied() {
                Some(Color::Black) => {}
                Some(Color::Gray) => {
                    if let Some(&start) = active_positions.get(dependency) {
                        let mut path = active_path[start..].to_vec();
                        path.push(dependency.clone());
                        cycles.insert(DependencyCycle { path });
                    }
                }
                None => {
                    colors.insert(dependency.clone(), Color::Gray);
                    active_positions
                        .insert(dependency.clone(), active_path.len());
                    active_path.push(dependency.clone());

                    let dependencies = graph
                        .edges
                        .get(dependency)
                        .map(|items| items.iter().collect())
                        .unwrap_or_default();

                    stack.push(Frame {
                        node: dependency,
                        next_dependency: 0,
                        dependencies,
                    });
                }
            }
        }
    }

    (cycles.into_iter().collect(), AnalysisStatus::Complete)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::domains::hdl::{
        HdlAssignment, HdlDirection, HdlProcess, HdlSignal, HdlValueType,
    };

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test identifier must be valid")
    }

    fn signal(value: &str) -> HdlSignal {
        HdlSignal {
            id: id(value),
            name: value.to_owned(),
            direction: HdlDirection::Internal,
            value_type: HdlValueType::Bit,
            location: None,
        }
    }

    fn assignment(target: &str, reads: &[&str]) -> HdlAssignment {
        HdlAssignment {
            target: id(target),
            reads: reads.iter().map(|value| id(value)).collect(),
            continuous: false,
            location: None,
        }
    }

    fn combinational_process(
        process_id: &str,
        assignments: Vec<HdlAssignment>,
    ) -> HdlProcess {
        HdlProcess {
            id: id(process_id),
            kind: HdlProcessKind::Combinational,
            assignments,
            clocks: Vec::new(),
            resets: Vec::new(),
            location: None,
        }
    }

    fn program(
        signal_ids: &[&str],
        processes: Vec<HdlProcess>,
    ) -> HdlProgram {
        HdlProgram {
            signals: signal_ids.iter().map(|value| signal(value)).collect(),
            clocks: Vec::new(),
            resets: Vec::new(),
            processes,
        }
    }

    #[test]
    fn empty_program_is_complete_and_acyclic() {
        let result = analyze_dependencies(
            &HdlProgram::default(),
            &DependencyAnalysisConfig::default(),
        );

        assert!(result.status.is_complete());
        assert!(result.graph.is_empty());
        assert!(result.cycles.is_empty());
        assert!(result.is_acyclic_and_complete());
    }

    #[test]
    fn preserves_isolated_declared_signals() {
        let result = analyze_dependencies(
            &program(&["unused"], Vec::new()),
            &DependencyAnalysisConfig::default(),
        );

        assert_eq!(result.graph.node_count(), 1);
        assert_eq!(result.graph.edge_count(), 0);
        assert!(result.is_acyclic_and_complete());
    }

    #[test]
    fn builds_edges_in_target_to_dependency_direction() {
        let input = program(
            &["a", "b"],
            vec![combinational_process(
                "p",
                vec![assignment("a", &["b"])],
            )],
        );

        let result =
            analyze_dependencies(&input, &DependencyAnalysisConfig::default());

        assert!(result.status.is_complete());
        assert_eq!(result.graph.edge_count(), 1);
        assert_eq!(
            result.graph.dependencies(&id("a")).map(Iterator::collect),
            Some(vec![id("b")])
        );
        assert!(result.cycles.is_empty());
    }

    #[test]
    fn detects_a_self_dependency() {
        let input = program(
            &["a"],
            vec![combinational_process(
                "p",
                vec![assignment("a", &["a"])],
            )],
        );

        let result =
            analyze_dependencies(&input, &DependencyAnalysisConfig::default());

        assert_eq!(
            result.cycles,
            vec![DependencyCycle {
                path: vec![id("a"), id("a")],
            }]
        );
    }

    #[test]
    fn detects_a_multi_node_cycle() {
        let input = program(
            &["a", "b", "c"],
            vec![combinational_process(
                "p",
                vec![
                    assignment("a", &["b"]),
                    assignment("b", &["c"]),
                    assignment("c", &["a"]),
                ],
            )],
        );

        let result =
            analyze_dependencies(&input, &DependencyAnalysisConfig::default());

        assert!(result.status.is_complete());
        assert_eq!(result.cycles.len(), 1);
        assert_eq!(
            result.cycles[0].path,
            vec![id("a"), id("b"), id("c"), id("a")]
        );
    }

    #[test]
    fn excludes_sequential_process_dependencies() {
        let mut sequential = combinational_process(
            "clocked",
            vec![assignment("a", &["a"])],
        );
        sequential.kind = HdlProcessKind::Sequential;
        sequential.clocks.push(id("clk"));

        let input = program(&["a"], vec![sequential]);
        let result =
            analyze_dependencies(&input, &DependencyAnalysisConfig::default());

        assert!(result.status.is_complete());
        assert!(result.cycles.is_empty());
        assert_eq!(result.graph.edge_count(), 0);
    }

    #[test]
    fn duplicate_edges_are_collapsed() {
        let input = program(
            &["a", "b"],
            vec![combinational_process(
                "p",
                vec![assignment("a", &["b", "b"])],
            )],
        );

        let result =
            analyze_dependencies(&input, &DependencyAnalysisConfig::default());

        assert_eq!(result.graph.node_count(), 2);
        assert_eq!(result.graph.edge_count(), 1);
    }

    #[test]
    fn node_budget_exhaustion_is_explicit() {
        let input = program(&["a", "b"], Vec::new());
        let config = DependencyAnalysisConfig {
            max_nodes: Some(1),
            ..DependencyAnalysisConfig::default()
        };

        let result = analyze_dependencies(&input, &config);

        assert_eq!(
            result.status,
            AnalysisStatus::Incomplete(IncompleteReason::NodeBudgetExceeded)
        );
        assert!(!result.is_acyclic_and_complete());
    }

    #[test]
    fn edge_budget_exhaustion_is_explicit() {
        let input = program(
            &["a", "b"],
            vec![combinational_process(
                "p",
                vec![assignment("a", &["b"])],
            )],
        );
        let config = DependencyAnalysisConfig {
            max_edges: Some(0),
            ..DependencyAnalysisConfig::default()
        };

        let result = analyze_dependencies(&input, &config);

        assert_eq!(
            result.status,
            AnalysisStatus::Incomplete(IncompleteReason::EdgeBudgetExceeded)
        );
        assert!(!result.is_acyclic_and_complete());
    }

    #[test]
    fn work_budget_exhaustion_is_explicit() {
        let input = program(&["a"], Vec::new());
        let config = DependencyAnalysisConfig {
            max_work: Some(0),
            ..DependencyAnalysisConfig::default()
        };

        let result = analyze_dependencies(&input, &config);

        assert_eq!(
            result.status,
            AnalysisStatus::Incomplete(IncompleteReason::WorkBudgetExceeded)
        );
        assert!(!result.is_acyclic_and_complete());
    }

    #[test]
    fn graph_iteration_is_deterministic() {
        let first = program(
            &["c", "b", "a"],
            vec![combinational_process(
                "p",
                vec![
                    assignment("a", &["c", "b"]),
                    assignment("b", &["c"]),
                ],
            )],
        );
        let second = program(
            &["a", "b", "c"],
            vec![combinational_process(
                "p",
                vec![
                    assignment("b", &["c"]),
                    assignment("a", &["b", "c"]),
                ],
            )],
        );

        let left =
            analyze_dependencies(&first, &DependencyAnalysisConfig::default());
        let right =
            analyze_dependencies(&second, &DependencyAnalysisConfig::default());

        assert_eq!(left, right);
    }
}