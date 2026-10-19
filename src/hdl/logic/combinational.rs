
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Combinational Behavior Analysis
//!
//! Path: `src/hdl/logic/combinational.rs`
//! Rust baseline: 1.97 or later.
//! Edition: 2021.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define target-independent analysis contracts for combinational regions:
//!
//! - definite, incomplete, and unproven output assignment coverage;
//! - dependency graphs and deterministic evaluation ordering;
//! - explicit detection of cyclic or cycle-blocked dependency graphs;
//! - caller-configurable operational budgets;
//! - deterministic, machine-readable analysis outcomes.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/combinational.g4`
//! - `grammar/hdl/sequential.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/expressions.rs`
//! - `src/hdl/logic/expression.rs`
//! - `src/hdl/logic/assignment.rs`
//! - `src/hdl/design/signal.rs`
//!
//! Grammar owns syntax. The canonical AST owns control-flow structure,
//! expression identity, and source locations. Semantic analysis owns name
//! resolution, types, assignment legality, and source-level meaning.
//!
//! The existing `validate_hdl` implementation already checks combinational
//! cycles and multiple process drivers. This module supplies a reusable
//! dependency-ordering utility and assignment-coverage analysis; it does not
//! replace that validator or define a competing semantic model.
//!
//! ## Does not own
//!
//! - Parsing, AST construction, source spans, or name resolution.
//! - Expression evaluation, type checking, or assignment validation.
//! - Control-flow reconstruction from flattened assignments.
//! - A replacement for `frontend::semantic::domains::hdl::validate_hdl`.
//! - Process scheduling, simulation time, synthesis, placement, or routing.
//! - Physical gates, registers, pins, or target-specific resource allocation.
//! - HDLIR structures, builders, passes, serialization, or lowering.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Integration contract
//!
//! An adapter over the canonical AST must determine which outputs require
//! complete assignment and whether each output is assigned on every relevant
//! control-flow path. It supplies those facts to `CombinationalRegion`.
//!
//! The adapter must preserve distinctions between a proven complete
//! assignment, a proven incomplete assignment, and a fact that cannot yet be
//! established. In particular, a flattened list of assignments is not proof
//! of complete assignment coverage.
//!
//! Dependencies use the convention `target depends on source`. Sequential
//! state feedback must not be represented as a combinational dependency.
//! The existing semantic validator remains authoritative for its own cycle
//! diagnostics and full-program driver checks.
//!
//! ## Scalability
//!
//! This module imposes no language-level maximum on signals, outputs,
//! dependencies, graph depth, or hardware width. Ordered dynamic collections
//! provide deterministic results. Optional budgets limit analysis work only;
//! exhausting a budget produces an incomplete result, never a false proof.
//!
//! Finite machine memory and execution time still apply. "Unrestricted" means
//! no additional caller-imposed limit, not physically infinite resources.
//!
//! ## Freeze criteria
//!
//! - AST adapters provide coverage facts derived from actual control flow.
//! - Semantic validation remains the authority for assignment legality and
//!   full-program driver checks.
//! - Unknown coverage never becomes a complete-coverage proof.
//! - Graph ordering is deterministic and does not use recursive traversal.
//! - Budget exhaustion is explicit.
//! - No HDLIR implementation is introduced.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::frontend::semantic::domains::hdl::HdlId;

/// Version of this module's public contract.
///
/// This is independent of the Zamani language and HDLIR schema versions.
pub const HDL_COMBINATIONAL_CONTRACT_VERSION: u16 = 1;

/// Caller-selected operational budgets for one combinational analysis.
///
/// These limits constrain analysis work, not legal source constructs or
/// hardware capacities. `None` means no additional limit for that measure.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CombinationalBudget {
    /// Maximum number of graph nodes to inspect, if configured.
    pub max_nodes: Option<usize>,

    /// Maximum number of dependency edges to inspect, if configured.
    pub max_edges: Option<usize>,
}

impl CombinationalBudget {
    /// Create a budget without caller-imposed node or edge limits.
    ///
    /// The analysis remains bounded by available execution time and memory.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_nodes: None,
            max_edges: None,
        }
    }

    /// Create a budget with explicit node and edge limits.
    #[must_use]
    pub const fn bounded(max_nodes: usize, max_edges: usize) -> Self {
        Self {
            max_nodes: Some(max_nodes),
            max_edges: Some(max_edges),
        }
    }
}

/// Result of determining whether a required output is assigned on every
/// relevant path through a combinational region.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AssignmentCoverage {
    /// The AST/control-flow analysis proved that every relevant path assigns
    /// the output before its value is observed.
    Complete,

    /// The AST/control-flow analysis proved that at least one relevant path
    /// leaves the output without a required assignment.
    Incomplete,

    /// The adapter could not prove either complete or incomplete coverage.
    ///
    /// Examples include unresolved control-flow facts, unsupported language
    /// extensions, or analysis that did not finish.
    Unknown,
}

/// Dependency graph for combinational values.
///
/// An edge `target -> source` means that calculating `target` reads `source`.
/// Nodes are canonical semantic identities, not physical wires or AST copies.
/// Ordered collections make iteration and reporting deterministic.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DependencyGraph {
    dependencies_by_target: BTreeMap<HdlId, BTreeSet<HdlId>>,
}

impl DependencyGraph {
    /// Create an empty dependency graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a node even when it has no dependencies.
    ///
    /// Registering a node does not prove that its declaration exists. The
    /// caller must perform declaration and name validation separately.
    pub fn add_node(&mut self, node: HdlId) {
        self.dependencies_by_target.entry(node).or_default();
    }

    /// Register that `target` depends on `source`.
    ///
    /// Duplicate edges are idempotent. This method does not decide whether
    /// either identity resolves to a declared signal.
    pub fn add_dependency(&mut self, target: HdlId, source: HdlId) {
        self.dependencies_by_target
            .entry(target)
            .or_default()
            .insert(source.clone());

        // A source may be a primary input or another node without its own
        // assignment. Registering it ensures it appears in graph ordering.
        self.dependencies_by_target.entry(source).or_default();
    }

    /// Return the graph's dependency relation in deterministic key order.
    #[must_use]
    pub fn dependencies(&self) -> &BTreeMap<HdlId, BTreeSet<HdlId>> {
        &self.dependencies_by_target
    }

    /// Return the number of distinct graph nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.dependencies_by_target.len()
    }

    /// Return the number of distinct dependency edges.
    ///
    /// This saturates only if the number of edges exceeds `usize::MAX`,
    /// which cannot be represented by an in-memory collection's length.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.dependencies_by_target
            .values()
            .fold(0usize, |total, edges| total.saturating_add(edges.len()))
    }

    /// Calculate a deterministic dependency-respecting order.
    ///
    /// Every source appears before each target that depends on it. If the
    /// graph is cyclic, returns the nodes that could not be ordered. Those
    /// nodes include cycle members and may include nodes downstream of a
    /// cycle; they are therefore called `blocked_nodes`, not a precise cycle
    /// witness.
    ///
    /// This method is iterative and does not consume call-stack space
    /// proportional to graph depth.
    #[must_use]
    pub fn evaluation_order(&self) -> EvaluationOrder {
        let mut dependents = BTreeMap::<HdlId, BTreeSet<HdlId>>::new();
        let mut remaining_dependencies = BTreeMap::<HdlId, usize>::new();

        for node in self.dependencies_by_target.keys() {
            dependents.entry(node.clone()).or_default();
            remaining_dependencies.insert(node.clone(), 0);
        }

        for (target, sources) in &self.dependencies_by_target {
            remaining_dependencies.insert(target.clone(), sources.len());

            for source in sources {
                dependents
                    .entry(source.clone())
                    .or_default()
                    .insert(target.clone());

                remaining_dependencies
                    .entry(source.clone())
                    .or_insert(0);
            }
        }

        let mut ready = BTreeSet::new();

        for (node, count) in &remaining_dependencies {
            if *count == 0 {
                ready.insert(node.clone());
            }
        }

        let mut ordered = Vec::with_capacity(remaining_dependencies.len());

        while let Some(node) = ready.pop_first() {
            ordered.push(node.clone());

            if let Some(next_nodes) = dependents.get(&node) {
                for dependent in next_nodes {
                    if let Some(count) = remaining_dependencies.get_mut(dependent) {
                        // Each dependent is registered once per distinct
                        // edge. A zero count here would indicate an internal
                        // graph invariant violation, not malformed source.
                        if *count > 0 {
                            *count -= 1;
                            if *count == 0 {
                                ready.insert(dependent.clone());
                            }
                        }
                    }
                }
            }
        }

        if ordered.len() == remaining_dependencies.len() {
            EvaluationOrder::Acyclic(ordered)
        } else {
            let ordered_set: BTreeSet<_> = ordered.iter().cloned().collect();
            let blocked_nodes = remaining_dependencies
                .keys()
                .filter(|node| !ordered_set.contains(*node))
                .cloned()
                .collect();

            EvaluationOrder::Cyclic { blocked_nodes }
        }
    }
}

/// Result of requesting a dependency-respecting evaluation order.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EvaluationOrder {
    /// All graph nodes can be ordered with dependencies before dependents.
    Acyclic(Vec<HdlId>),

    /// At least one cycle prevents a complete order.
    ///
    /// `blocked_nodes` includes nodes that cannot be emitted by topological
    /// ordering. It is not guaranteed to contain only the strongly connected
    /// component members.
    Cyclic {
        /// Nodes blocked by one or more dependency cycles.
        blocked_nodes: Vec<HdlId>,
    },
}

/// Source-level facts required to analyze a combinational region.
///
/// The AST adapter is responsible for deriving `coverage` from the actual
/// branch, case, loop, and nested-block structure. This type intentionally
/// does not attempt to reconstruct that structure from flattened assignments.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CombinationalRegion {
    /// Outputs or other values for which complete assignment is required.
    ///
    /// These identities must be resolved by the caller. An empty collection
    /// means no output-coverage requirements were supplied; it does not
    /// establish that a design's actual outputs are complete.
    pub required_outputs: BTreeSet<HdlId>,

    /// Coverage facts produced by control-flow analysis.
    ///
    /// A missing entry is treated as unknown, not as proof of non-assignment.
    pub coverage: BTreeMap<HdlId, AssignmentCoverage>,

    /// Dependencies between values in this combinational region.
    pub dependencies: DependencyGraph,
}

impl CombinationalRegion {
    /// Create an empty region.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Mark an output as requiring complete assignment.
    pub fn require_output(&mut self, output: HdlId) {
        self.required_outputs.insert(output);
    }

    /// Supply a control-flow-derived assignment coverage fact.
    ///
    /// If multiple analysis passes produce facts for the same output, the
    /// adapter must reconcile them before calling this method; this method
    /// intentionally replaces the previous fact.
    pub fn set_coverage(
        &mut self,
        output: HdlId,
        coverage: AssignmentCoverage,
    ) {
        self.coverage.insert(output, coverage);
    }

    /// Borrow the dependency graph for adding read relationships.
    pub fn dependencies_mut(&mut self) -> &mut DependencyGraph {
        &mut self.dependencies
    }
}

/// Stable diagnostic category for combinational analysis.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum CombinationalDiagnosticCode {
    /// A required output is proven not to be assigned on every relevant path.
    IncompleteOutputAssignment,

    /// Coverage facts were unavailable for a required output.
    UnknownAssignmentCoverage,

    /// The dependency graph contains a cycle or nodes blocked by a cycle.
    CombinationalDependencyCycle,

    /// A configured node or edge budget prevented complete analysis.
    AnalysisBudgetExceeded,
}

/// One deterministic combinational-analysis diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CombinationalDiagnostic {
    /// Stable machine-readable diagnostic category.
    pub code: CombinationalDiagnosticCode,

    /// Optional semantic identity associated with the diagnostic.
    pub subject: Option<HdlId>,

    /// Human-readable explanation.
    pub message: String,
}

impl fmt::Display for CombinationalDiagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.subject {
            Some(subject) => write!(formatter, "{}: {}", subject, self.message),
            None => formatter.write_str(&self.message),
        }
    }
}

/// Overall status of combinational analysis.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CombinationalAnalysisStatus {
    /// All requested checks completed and no violations were found.
    Valid,

    /// At least one requested check proved a source-level violation.
    Invalid,

    /// At least one requested check could not be completed or proven.
    Incomplete,
}

/// Result of analyzing one normalized combinational region.
///
/// `Valid` means only that the checks represented by this input completed
/// without violations. It does not replace whole-program semantic validation,
/// expression type checking, driver validation, or target feasibility.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CombinationalAnalysis {
    /// Overall analysis status.
    pub status: CombinationalAnalysisStatus,

    /// Deterministically ordered diagnostics.
    pub diagnostics: Vec<CombinationalDiagnostic>,

    /// Dependency order when the graph is acyclic and within budget.
    pub evaluation_order: Option<Vec<HdlId>>,

    /// Nodes blocked by a cycle, when cycle analysis completed.
    pub blocked_nodes: Vec<HdlId>,

    /// Whether every requested check represented by this result completed.
    pub complete: bool,
}

/// Analyze output assignment coverage and combinational dependencies.
///
/// This function consumes normalized facts. It does not parse source, infer
/// control flow from assignments, or resolve declarations.
///
/// Budget exhaustion takes precedence over any claim of successful
/// validation: if the requested graph exceeds the supplied budget, the
/// function returns an incomplete result without certifying coverage or
/// acyclicity.
///
/// # Examples
///
/// ```
/// use zamani_compiler::frontend::semantic::domains::hdl::HdlId;
/// use zamani_compiler::hdl::logic::combinational::{
///     analyze_combinational, AssignmentCoverage, CombinationalBudget,
///     CombinationalRegion, CombinationalAnalysisStatus,
/// };
///
/// let output = HdlId::new("out").expect("valid test identity");
/// let mut region = CombinationalRegion::new();
/// region.require_output(output.clone());
/// region.set_coverage(output, AssignmentCoverage::Complete);
///
/// let result = analyze_combinational(
///     &region,
///     CombinationalBudget::unrestricted(),
/// );
///
/// assert_eq!(result.status, CombinationalAnalysisStatus::Valid);
/// assert!(result.complete);
/// ```
#[must_use]
pub fn analyze_combinational(
    region: &CombinationalRegion,
    budget: CombinationalBudget,
) -> CombinationalAnalysis {
    let graph_nodes = region.dependencies.node_count();
    let graph_edges = region.dependencies.edge_count();

    let exceeds_node_budget = budget
        .max_nodes
        .is_some_and(|maximum| graph_nodes > maximum);

    let exceeds_edge_budget = budget
        .max_edges
        .is_some_and(|maximum| graph_edges > maximum);

    if exceeds_node_budget || exceeds_edge_budget {
        let mut diagnostics = vec![CombinationalDiagnostic {
            code: CombinationalDiagnosticCode::AnalysisBudgetExceeded,
            subject: None,
            message: format!(
                "combinational analysis budget exceeded (nodes: {graph_nodes}, \
                 edges: {graph_edges})"
            ),
        }];
        sort_diagnostics(&mut diagnostics);

        return CombinationalAnalysis {
            status: CombinationalAnalysisStatus::Incomplete,
            diagnostics,
            evaluation_order: None,
            blocked_nodes: Vec::new(),
            complete: false,
        };
    }

    let mut diagnostics = Vec::new();
    let mut incomplete = false;

    for output in &region.required_outputs {
        match region.coverage.get(output) {
            Some(AssignmentCoverage::Complete) => {}
            Some(AssignmentCoverage::Incomplete) => {
                diagnostics.push(CombinationalDiagnostic {
                    code: CombinationalDiagnosticCode::IncompleteOutputAssignment,
                    subject: Some(output.clone()),
                    message: "required output is not assigned on every relevant \
                              combinational path"
                        .to_owned(),
                });
            }
            Some(AssignmentCoverage::Unknown) | None => {
                incomplete = true;
                diagnostics.push(CombinationalDiagnostic {
                    code: CombinationalDiagnosticCode::UnknownAssignmentCoverage,
                    subject: Some(output.clone()),
                    message: "complete assignment coverage could not be \
                              established from the available control-flow facts"
                        .to_owned(),
                });
            }
        }
    }

    let (evaluation_order, blocked_nodes) =
        match region.dependencies.evaluation_order() {
            EvaluationOrder::Acyclic(order) => (Some(order), Vec::new()),
            EvaluationOrder::Cyclic { blocked_nodes } => {
                diagnostics.push(CombinationalDiagnostic {
                    code: CombinationalDiagnosticCode::CombinationalDependencyCycle,
                    subject: blocked_nodes.first().cloned(),
                    message: format!(
                        "combinational dependency graph contains a cycle or \
                         cycle-blocked nodes: {}",
                        blocked_nodes
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                });
                (None, blocked_nodes)
            }
        };

    sort_diagnostics(&mut diagnostics);

    let has_violation = diagnostics.iter().any(|diagnostic| {
        matches!(
            diagnostic.code,
            CombinationalDiagnosticCode::IncompleteOutputAssignment
                | CombinationalDiagnosticCode::CombinationalDependencyCycle
        )
    });

    let complete = !incomplete;
    let status = if has_violation {
        CombinationalAnalysisStatus::Invalid
    } else if !complete {
        CombinationalAnalysisStatus::Incomplete
    } else {
        CombinationalAnalysisStatus::Valid
    };

    CombinationalAnalysis {
        status,
        diagnostics,
        evaluation_order,
        blocked_nodes,
        complete,
    }
}

fn sort_diagnostics(diagnostics: &mut [CombinationalDiagnostic]) {
    diagnostics.sort_by(|left, right| {
        (&left.subject, left.code, &left.message).cmp(&(
            &right.subject,
            right.code,
            &right.message,
        ))
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test identity must be valid")
    }

    #[test]
    fn empty_region_does_not_invent_output_requirements() {
        let result = analyze_combinational(
            &CombinationalRegion::new(),
            CombinationalBudget::unrestricted(),
        );

        assert_eq!(result.status, CombinationalAnalysisStatus::Valid);
        assert!(result.complete);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn complete_output_coverage_is_accepted() {
        let output = id("out");
        let mut region = CombinationalRegion::new();
        region.require_output(output.clone());
        region.set_coverage(output, AssignmentCoverage::Complete);

        let result =
            analyze_combinational(&region, CombinationalBudget::unrestricted());

        assert_eq!(result.status, CombinationalAnalysisStatus::Valid);
        assert!(result.complete);
    }

    #[test]
    fn proven_incomplete_output_is_invalid() {
        let output = id("out");
        let mut region = CombinationalRegion::new();
        region.require_output(output.clone());
        region.set_coverage(output, AssignmentCoverage::Incomplete);

        let result =
            analyze_combinational(&region, CombinationalBudget::unrestricted());

        assert_eq!(result.status, CombinationalAnalysisStatus::Invalid);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == CombinationalDiagnosticCode::IncompleteOutputAssignment
        }));
    }

    #[test]
    fn missing_coverage_fact_is_not_assumed_complete() {
        let mut region = CombinationalRegion::new();
        region.require_output(id("out"));

        let result =
            analyze_combinational(&region, CombinationalBudget::unrestricted());

        assert_eq!(result.status, CombinationalAnalysisStatus::Incomplete);
        assert!(!result.complete);
        assert!(result.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == CombinationalDiagnosticCode::UnknownAssignmentCoverage
        }));
    }

    #[test]
    fn dependencies_are_ordered_source_before_target() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency(id("sum"), id("left"));
        graph.add_dependency(id("sum"), id("right"));

        let EvaluationOrder::Acyclic(order) = graph.evaluation_order() else {
            panic!("acyclic dependency graph must have an evaluation order");
        };

        let left = order.iter().position(|node| node == &id("left")).unwrap();
        let right = order.iter().position(|node| node == &id("right")).unwrap();
        let sum = order.iter().position(|node| node == &id("sum")).unwrap();

        assert!(left < sum);
        assert!(right < sum);
    }

    #[test]
    fn self_dependency_is_reported_as_cyclic() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency(id("feedback"), id("feedback"));

        assert_eq!(
            graph.evaluation_order(),
            EvaluationOrder::Cyclic {
                blocked_nodes: vec![id("feedback")],
            }
        );
    }

    #[test]
    fn multi_node_cycle_is_detected_deterministically() {
        let mut first = DependencyGraph::new();
        first.add_dependency(id("a"), id("b"));
        first.add_dependency(id("b"), id("a"));

        let mut second = DependencyGraph::new();
        second.add_dependency(id("b"), id("a"));
        second.add_dependency(id("a"), id("b"));

        assert_eq!(first.evaluation_order(), second.evaluation_order());

        let EvaluationOrder::Cyclic { blocked_nodes } = first.evaluation_order() else {
            panic!("cyclic dependency graph must be reported");
        };

        assert_eq!(blocked_nodes, vec![id("a"), id("b")]);
    }

    #[test]
    fn cycle_blocked_nodes_are_not_mislabeled_as_exact_cycle_members() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency(id("a"), id("b"));
        graph.add_dependency(id("b"), id("a"));
        graph.add_dependency(id("downstream"), id("a"));

        let EvaluationOrder::Cyclic { blocked_nodes } = graph.evaluation_order() else {
            panic!("graph contains a cycle");
        };

        assert!(blocked_nodes.contains(&id("a")));
        assert!(blocked_nodes.contains(&id("b")));
        assert!(blocked_nodes.contains(&id("downstream")));
    }

    #[test]
    fn budget_exhaustion_is_incomplete_not_valid() {
        let mut region = CombinationalRegion::new();
        region
            .dependencies_mut()
            .add_dependency(id("out"), id("input"));

        let result =
            analyze_combinational(&region, CombinationalBudget::bounded(1, 10));

        assert_eq!(result.status, CombinationalAnalysisStatus::Incomplete);
        assert!(!result.complete);
        assert!(result.evaluation_order.is_none());
        assert_eq!(
            result.diagnostics[0].code,
            CombinationalDiagnosticCode::AnalysisBudgetExceeded
        );
    }

    #[test]
    fn duplicate_dependency_is_idempotent() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency(id("out"), id("input"));
        graph.add_dependency(id("out"), id("input"));

        assert_eq!(graph.edge_count(), 1);
        assert_eq!(graph.node_count(), 2);
    }

    #[test]
    fn disconnected_nodes_are_ordered_deterministically() {
        let mut graph = DependencyGraph::new();
        graph.add_node(id("z"));
        graph.add_node(id("a"));
        graph.add_node(id("m"));

        assert_eq!(
            graph.evaluation_order(),
            EvaluationOrder::Acyclic(vec![id("a"), id("m"), id("z")])
        );
    }

    #[test]
    fn required_outputs_are_checked_in_stable_order() {
        let mut region = CombinationalRegion::new();
        region.require_output(id("z"));
        region.require_output(id("a"));

        let result =
            analyze_combinational(&region, CombinationalBudget::unrestricted());

        let subjects: Vec<_> = result
            .diagnostics
            .iter()
            .filter_map(|diagnostic| diagnostic.subject.as_ref())
            .cloned()
            .collect();

        assert_eq!(subjects, vec![id("a"), id("z")]);
    }
}
