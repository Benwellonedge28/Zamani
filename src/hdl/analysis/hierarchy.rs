
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Structural Hierarchy Analysis
//!
//! Path: `src/hdl/analysis/hierarchy.rs`
//! Rust: 1.97+
//! Edition: 2021
//!
//! ## Purpose
//!
//! Analyze module definitions and their logical instances using the existing
//! target-independent HDL design model. Report unresolved and ambiguous
//! references, inferred or explicit roots, reachable and unreachable module
//! definitions, and recursive-instantiation cycles.
//!
//! ## Authority and ownership
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/hardware-modules.g4`
//! - `src/frontend/ast/node/node_id.rs`
//! - `src/hdl/design/module.rs`
//! - `src/hdl/design/instance.rs`
//!
//! The grammar owns syntax; the canonical AST owns source nodes and IDs;
//! semantic analysis owns source-level name resolution and meaning.
//! `HdlModule` owns module declaration records and `HdlModuleInstance`
//! owns instance records. This file owns only derived hierarchy findings.
//!
//! ## Important boundary
//!
//! An instance's `ModuleName` is a reference, not proof of successful name
//! resolution. This analysis resolves names only against the module
//! definitions explicitly supplied by its caller. It does not replace the
//! language's namespace/import resolver or elaborate generic parameters.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is deferred.
//!
//! ## Scalability
//!
//! There are no universal module, instance, or hierarchy-depth ceilings.
//! Optional budgets limit analysis work only. Traversals are iterative;
//! exhausted budgets produce an explicit incomplete status. A completed
//! analysis is not a claim that the host has infinite resources.
//!
//! ## Determinism
//!
//! Ordered maps, sets, and stable traversal ordering make results
//! deterministic for equivalent inputs. Input order is preserved by the
//! canonical design records but does not control graph traversal order.
//!
//! ## Integration
//!
//! The caller supplies validated module records and one instance list for
//! each containing module, keyed by that module's canonical declaration ID.
//! Instance lists must represent direct instances in the corresponding
//! module scope. This file does not inspect AST body nodes to discover
//! instances automatically.
//!
//! This module does not own parsing, AST construction, type checking,
//! port-connection checking, generic elaboration, combinational signal
//! dependency analysis, physical realization, or HDLIR.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::frontend::ast::node::node_id::NodeId;

use crate::hdl::design::instance::{HdlInstanceList, HdlModuleInstance};
use crate::hdl::design::module::{HdlModule, ModuleName};

/// Caller-configurable operational budgets for hierarchy analysis.
///
/// `None` means this configuration imposes no limit on that dimension.
/// It does not promise unlimited time, memory, address space, or host
/// resources. These budgets are not HDL language limits.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HierarchyAnalysisConfig {
    /// Explicit root module names.
    ///
    /// When empty, roots are inferred from the supplied module graph.
    /// When non-empty, only these names are selected as roots; each must
    /// resolve to exactly one supplied module definition.
    pub roots: Vec<ModuleName>,

    /// Maximum number of module definitions admitted for analysis.
    pub max_modules: Option<usize>,

    /// Maximum number of direct instances examined.
    pub max_instances: Option<usize>,

    /// Maximum number of analysis work units.
    ///
    /// Work is charged for module, instance, and graph-traversal visits.
    pub max_work: Option<usize>,
}

/// Reason a hierarchy analysis could not finish.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HierarchyIncompleteReason {
    /// The configured module budget prevented complete indexing.
    ModuleBudgetExceeded,
    /// The configured instance budget prevented complete graph construction.
    InstanceBudgetExceeded,
    /// The configured work budget prevented complete analysis.
    WorkBudgetExceeded,
}

/// Whether the requested computational analysis completed.
///
/// `Complete` means the configured work finished. It does not mean the
/// design has no hierarchy issues: completed analyses can report unresolved
/// references, duplicate module names, or cycles.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HierarchyAnalysisStatus {
    /// All requested indexing and graph traversals completed.
    Complete,

    /// Analysis stopped before all requested work completed.
    Incomplete(HierarchyIncompleteReason),
}

impl HierarchyAnalysisStatus {
    /// Return true only if the configured analysis completed.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// A resolved parent-to-child module-instantiation edge.
///
/// `parent_module` and `child_module` identify module definitions.
/// `instance_id` identifies the distinct instance declaration connecting
/// them. Multiple edges may point to the same child definition.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct HierarchyEdge {
    /// Canonical declaration ID of the containing module.
    pub parent_module: NodeId,

    /// Canonical declaration ID of the child module definition.
    pub child_module: NodeId,

    /// Canonical AST ID of the instance declaration.
    pub instance_id: NodeId,
}

/// A recursive-instantiation cycle witness.
///
/// `modules` is a closed path: its first module is repeated at the end.
/// `instances[i]` is the instance edge from `modules[i]` to
/// `modules[i + 1]`. Therefore, `instances.len() + 1 == modules.len()`.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct HierarchyCycle {
    /// Module declaration IDs forming the closed cycle path.
    pub modules: Vec<NodeId>,

    /// Instance declaration IDs forming the cycle edges.
    pub instances: Vec<NodeId>,
}

/// A structural issue discovered during hierarchy analysis.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum HierarchyIssue {
    /// Multiple module declarations use the same qualified name.
    DuplicateModuleName {
        /// The duplicated qualified module name.
        name: ModuleName,

        /// All matching module declaration IDs, in deterministic order.
        declarations: Vec<NodeId>,
    },

    /// An instance references a name absent from the supplied module set.
    UnresolvedModuleReference {
        /// Containing module declaration ID.
        parent_module: NodeId,

        /// Instance declaration ID.
        instance_id: NodeId,

        /// Unresolved qualified module name.
        referenced_name: ModuleName,
    },

    /// An instance's name matches multiple supplied module definitions.
    AmbiguousModuleReference {
        /// Containing module declaration ID.
        parent_module: NodeId,

        /// Instance declaration ID.
        instance_id: NodeId,

        /// Ambiguous qualified module name.
        referenced_name: ModuleName,

        /// Candidate module declarations, in deterministic order.
        candidates: Vec<NodeId>,
    },

    /// An instance list was supplied for a module not in the module set.
    UnknownInstanceListOwner {
        /// Module declaration ID used as the instance-list key.
        module_id: NodeId,
    },

    /// An explicit root name does not match any supplied definition.
    UnresolvedExplicitRoot {
        /// Requested root name.
        name: ModuleName,
    },

    /// An explicit root name matches more than one definition.
    AmbiguousExplicitRoot {
        /// Requested root name.
        name: ModuleName,

        /// Candidate declarations, in deterministic order.
        candidates: Vec<NodeId>,
    },
}

/// Result of structural hierarchy analysis.
///
/// If `status` is incomplete, the graph, root set, reachability results,
/// and issues may be partial. In particular, an incomplete result must not
/// be treated as proof that no recursive-instantiation cycle exists.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HierarchyAnalysis {
    /// Whether all configured analysis work completed.
    pub status: HierarchyAnalysisStatus,

    /// Selected root module declaration IDs, sorted and deduplicated.
    pub roots: Vec<NodeId>,

    /// Resolved parent-to-child instantiation edges.
    pub edges: Vec<HierarchyEdge>,

    /// Module declaration IDs reachable from selected roots.
    pub reachable_modules: Vec<NodeId>,

    /// Supplied module declaration IDs not reached from selected roots.
    ///
    /// With inferred roots, a rootless cyclic component will also be
    /// reported as unreachable.
    pub unreachable_modules: Vec<NodeId>,

    /// Recursive-instantiation cycle witnesses found during traversal.
    pub cycles: Vec<HierarchyCycle>,

    /// Structural issues found during indexing and reference resolution.
    pub issues: Vec<HierarchyIssue>,

    /// Number of module definitions admitted to the name index.
    pub modules_examined: usize,

    /// Number of instance declarations examined.
    pub instances_examined: usize,

    /// Number of charged analysis work units.
    pub work_units: usize,
}

impl HierarchyAnalysis {
    /// Return true only when analysis completed and found no issues or cycles.
    ///
    /// This is a structural result, not a substitute for semantic validation,
    /// type checking, port validation, or target feasibility analysis.
    #[must_use]
    pub fn is_structurally_clean(&self) -> bool {
        self.status.is_complete() && self.issues.is_empty() && self.cycles.is_empty()
    }

    /// Return true when analysis completed and no recursive-instantiation
    /// cycle was found.
    ///
    /// This does not imply that names resolved or that the design is valid.
    #[must_use]
    pub fn is_cycle_free_and_complete(&self) -> bool {
        self.status.is_complete() && self.cycles.is_empty()
    }
}

impl fmt::Display for HierarchyIncompleteReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ModuleBudgetExceeded => f.write_str("module budget exceeded"),
            Self::InstanceBudgetExceeded => f.write_str("instance budget exceeded"),
            Self::WorkBudgetExceeded => f.write_str("analysis work budget exceeded"),
        }
    }
}

impl fmt::Display for HierarchyAnalysisStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Complete => f.write_str("complete"),
            Self::Incomplete(reason) => write!(f, "incomplete: {reason}"),
        }
    }
}

/// Analyze the structural hierarchy of supplied HDL module definitions.
///
/// `modules` contains the module definitions in the analysis scope.
/// `instances_by_module` maps each containing module's declaration ID to
/// its direct instance declarations. The lists must describe direct lexical
/// module membership; nested generate scopes must be normalized by the
/// caller into the appropriate module's direct instance collection.
///
/// When `config.roots` is empty, roots are inferred as definitions with no
/// incoming edge among uniquely resolved module references. Definitions
/// that are candidates of an ambiguous reference are not inferred as roots,
/// because doing so would overstate what the analysis knows.
///
/// When explicit roots are supplied, each must resolve to exactly one
/// definition. Invalid or ambiguous root requests are reported as issues.
///
/// Module-name resolution here is local to `modules`. Imports, libraries,
/// namespaces, generic specialization, generated module families, and
/// external declarations must be resolved or elaborated by their owning
/// subsystem before their results can be represented accurately here.
///
/// # Determinism
///
/// Names, declaration IDs, edges, issues, roots, reachability results, and
/// cycle witnesses are returned in deterministic order.
///
/// # Budget behavior
///
/// If a configured budget is exhausted, the function returns the partial
/// findings collected so far with an explicit incomplete status. Partial
/// results must not be used to certify the absence of cycles or unreachable
/// definitions.
///
/// # Scalability
///
/// Traversal is iterative rather than recursive. No fixed hierarchy-depth,
/// module-count, or instance-count ceiling is imposed by this API.
///
/// # Examples
///
/// ```ignore
/// let result = analyze_hierarchy(
///     &modules,
///     &instances_by_module,
///     &HierarchyAnalysisConfig::default(),
/// );
/// assert!(result.status.is_complete());
/// ```
///
/// The ignored example illustrates the API without prescribing a particular
/// project setup or claiming that a caller has already constructed the model.
#[must_use]
pub fn analyze_hierarchy(
    modules: &[HdlModule],
    instances_by_module: &BTreeMap<NodeId, HdlInstanceList>,
    config: &HierarchyAnalysisConfig,
) -> HierarchyAnalysis {
    let mut result = HierarchyAnalysis {
        status: HierarchyAnalysisStatus::Complete,
        roots: Vec::new(),
        edges: Vec::new(),
        reachable_modules: Vec::new(),
        unreachable_modules: Vec::new(),
        cycles: Vec::new(),
        issues: Vec::new(),
        modules_examined: 0,
        instances_examined: 0,
        work_units: 0,
    };

    let mut modules_by_id = BTreeMap::<NodeId, &HdlModule>::new();
    let mut ids_by_name = BTreeMap::<ModuleName, Vec<NodeId>>::new();

    // Index definitions. A repeated declaration ID is treated as a
    // violated input contract: the same canonical definition was supplied
    // more than once. Keep the first record deterministically.
    for module in modules {
        if work_budget_reached(config.max_work, result.work_units) {
            result.status = HierarchyAnalysisStatus::Incomplete(
                HierarchyIncompleteReason::WorkBudgetExceeded,
            );
            return finish_result(result);
        }
        result.work_units = result.work_units.saturating_add(1);

        if budget_reached(config.max_modules, result.modules_examined) {
            result.status = HierarchyAnalysisStatus::Incomplete(
                HierarchyIncompleteReason::ModuleBudgetExceeded,
            );
            return finish_result(result);
        }

        result.modules_examined = result.modules_examined.saturating_add(1);

        let id = module.declaration_id();
        if modules_by_id.contains_key(&id) {
            result.issues.push(HierarchyIssue::DuplicateModuleName {
                name: module.name().clone(),
                declarations: vec![id],
            });
            continue;
        }

        modules_by_id.insert(id, module);
        ids_by_name
            .entry(module.name().clone())
            .or_default()
            .push(id);
    }

    for declarations in ids_by_name.values_mut() {
        declarations.sort();
        declarations.dedup();
    }

    for (name, declarations) in &ids_by_name {
        if declarations.len() > 1 {
            result
                .issues
                .push(HierarchyIssue::DuplicateModuleName {
                    name: name.clone(),
                    declarations: declarations.clone(),
                });
        }
    }

    // Instance-list owners must refer to module definitions in this input.
    for owner in instances_by_module.keys() {
        if !modules_by_id.contains_key(owner) {
            result
                .issues
                .push(HierarchyIssue::UnknownInstanceListOwner {
                    module_id: *owner,
                });
        }
    }

    let mut adjacency = BTreeMap::<NodeId, Vec<HierarchyEdge>>::new();
    let mut incoming = BTreeSet::<NodeId>::new();
    let mut possible_incoming = BTreeSet::<NodeId>::new();

    for id in modules_by_id.keys() {
        adjacency.entry(*id).or_default();
    }

    // Resolve direct instance references only against the supplied module
    // index. Do not guess which duplicate declaration was intended.
    for (parent_id, list) in instances_by_module {
        if !modules_by_id.contains_key(parent_id) {
            continue;
        }

        for instance in list.as_slice() {
            if work_budget_reached(config.max_work, result.work_units) {
                result.status = HierarchyAnalysisStatus::Incomplete(
                    HierarchyIncompleteReason::WorkBudgetExceeded,
                );
                return finish_result(result);
            }
            result.work_units = result.work_units.saturating_add(1);

            if budget_reached(config.max_instances, result.instances_examined) {
                result.status = HierarchyAnalysisStatus::Incomplete(
                    HierarchyIncompleteReason::InstanceBudgetExceeded,
                );
                return finish_result(result);
            }
            result.instances_examined =
                result.instances_examined.saturating_add(1);

            let referenced_name = instance.module_reference();
            match ids_by_name.get(referenced_name) {
                None => {
                    result.issues.push(
                        HierarchyIssue::UnresolvedModuleReference {
                            parent_module: *parent_id,
                            instance_id: instance.declaration_id(),
                            referenced_name: referenced_name.clone(),
                        },
                    );
                }
                Some(candidates) if candidates.len() == 1 => {
                    let child_id = candidates[0];
                    let edge = HierarchyEdge {
                        parent_module: *parent_id,
                        child_module: child_id,
                        instance_id: instance.declaration_id(),
                    };

                    incoming.insert(child_id);
                    adjacency.entry(*parent_id).or_default().push(edge.clone());
                    result.edges.push(edge);
                }
                Some(candidates) => {
                    possible_incoming.extend(candidates.iter().copied());
                    result.issues.push(
                        HierarchyIssue::AmbiguousModuleReference {
                            parent_module: *parent_id,
                            instance_id: instance.declaration_id(),
                            referenced_name: referenced_name.clone(),
                            candidates: candidates.clone(),
                        },
                    );
                }
            }
        }
    }

    for edges in adjacency.values_mut() {
        edges.sort();
        edges.dedup();
    }
    result.edges.sort();
    result.edges.dedup();

    // Resolve explicit roots or infer them from known edges. Ambiguous
    // references suppress automatic root inference for their candidates.
    let mut roots = BTreeSet::<NodeId>::new();

    if config.roots.is_empty() {
        for id in modules_by_id.keys() {
            if !incoming.contains(id) && !possible_incoming.contains(id) {
                roots.insert(*id);
            }
        }
    } else {
        let mut requested_roots = config.roots.clone();
        requested_roots.sort();
        requested_roots.dedup();

        for name in requested_roots {
            match ids_by_name.get(&name) {
                None => result
                    .issues
                    .push(HierarchyIssue::UnresolvedExplicitRoot { name }),
                Some(candidates) if candidates.len() == 1 => {
                    roots.insert(candidates[0]);
                }
                Some(candidates) => {
                    result
                        .issues
                        .push(HierarchyIssue::AmbiguousExplicitRoot {
                            name,
                            candidates: candidates.clone(),
                        });
                }
            }
        }
    }

    result.roots = roots.iter().copied().collect();

    // Reachability is iterative. If a budget is exhausted, preserve the
    // visited subset but do not report unvisited definitions as proven
    // unreachable because traversal did not finish.
    let mut reachable = BTreeSet::<NodeId>::new();
    let mut stack: Vec<NodeId> = result.roots.iter().rev().copied().collect();

    while let Some(current) = stack.pop() {
        if !reachable.insert(current) {
            continue;
        }

        if work_budget_reached(config.max_work, result.work_units) {
            result.status = HierarchyAnalysisStatus::Incomplete(
                HierarchyIncompleteReason::WorkBudgetExceeded,
            );
            result.reachable_modules = reachable.into_iter().collect();
            return finish_result(result);
        }
        result.work_units = result.work_units.saturating_add(1);

        if let Some(edges) = adjacency.get(&current) {
            // Reverse push order so the smallest edge is visited first
            // when popped from the LIFO stack.
            for edge in edges.iter().rev() {
                stack.push(edge.child_module);
            }
        }
    }

    result.reachable_modules = reachable.iter().copied().collect();
    result.unreachable_modules = modules_by_id
        .keys()
        .filter(|id| !reachable.contains(id))
        .copied()
        .collect();

    // Cycle discovery is a separate iterative DFS. Gray nodes are on the
    // current path; an edge to a gray node yields a closed cycle witness.
    let (cycles, cycle_status, work_units) =
        find_hierarchy_cycles(&adjacency, config.max_work, result.work_units);
    result.cycles = cycles;
    result.work_units = work_units;
    if let Some(reason) = cycle_status {
        result.status = HierarchyAnalysisStatus::Incomplete(reason);
    }

    finish_result(result)
}

fn budget_reached(limit: Option<usize>, current: usize) -> bool {
    limit.is_some_and(|maximum| current >= maximum)
}

fn work_budget_reached(limit: Option<usize>, work_units: usize) -> bool {
    budget_reached(limit, work_units)
}

fn finish_result(mut result: HierarchyAnalysis) -> HierarchyAnalysis {
    result.roots.sort();
    result.roots.dedup();
    result.edges.sort();
    result.edges.dedup();
    result.reachable_modules.sort();
    result.reachable_modules.dedup();
    result.unreachable_modules.sort();
    result.unreachable_modules.dedup();
    result.cycles.sort();
    result.cycles.dedup();
    result.issues.sort();
    result.issues.dedup();
    result
}

/// Find cycle witnesses using explicit DFS frames rather than recursive calls.
fn find_hierarchy_cycles(
    adjacency: &BTreeMap<NodeId, Vec<HierarchyEdge>>,
    max_work: Option<usize>,
    mut work_units: usize,
) -> (
    Vec<HierarchyCycle>,
    Option<HierarchyIncompleteReason>,
    usize,
) {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Color {
        Gray,
        Black,
    }

    struct Frame {
        module: NodeId,
        next_edge: usize,
    }

    let mut colors = BTreeMap::<NodeId, Color>::new();
    let mut active_modules = Vec::<NodeId>::new();
    let mut active_edges = Vec::<HierarchyEdge>::new();
    let mut active_positions = BTreeMap::<NodeId, usize>::new();
    let mut cycles = BTreeSet::<HierarchyCycle>::new();

    for root in adjacency.keys() {
        if colors.contains_key(root) {
            continue;
        }

        if work_budget_reached(max_work, work_units) {
            return (
                cycles.into_iter().collect(),
                Some(HierarchyIncompleteReason::WorkBudgetExceeded),
                work_units,
            );
        }
        work_units = work_units.saturating_add(1);

        colors.insert(*root, Color::Gray);
        active_positions.insert(*root, active_modules.len());
        active_modules.push(*root);

        let mut frames = vec![Frame {
            module: *root,
            next_edge: 0,
        }];

        while let Some(frame) = frames.last_mut() {
            let edges = adjacency
                .get(&frame.module)
                .map(Vec::as_slice)
                .unwrap_or(&[]);

            if frame.next_edge >= edges.len() {
                let finished = frame.module;
                frames.pop();
                colors.insert(finished, Color::Black);
                active_positions.remove(&finished);
                active_modules.pop();

                if !active_edges.is_empty() {
                    active_edges.pop();
                }
                continue;
            }

            if work_budget_reached(max_work, work_units) {
                return (
                    cycles.into_iter().collect(),
                    Some(HierarchyIncompleteReason::WorkBudgetExceeded),
                    work_units,
                );
            }
            work_units = work_units.saturating_add(1);

            let edge = edges[frame.next_edge].clone();
            frame.next_edge += 1;

            match colors.get(&edge.child_module).copied() {
                Some(Color::Black) => {}
                Some(Color::Gray) => {
                    if let Some(&start) = active_positions.get(&edge.child_module) {
                        let mut modules = active_modules[start..].to_vec();
                        modules.push(edge.child_module);

                        let mut instances: Vec<NodeId> = active_edges[start..]
                            .iter()
                            .map(|path_edge| path_edge.instance_id)
                            .collect();
                        instances.push(edge.instance_id);

                        cycles.insert(HierarchyCycle { modules, instances });
                    }
                }
                None => {
                    colors.insert(edge.child_module, Color::Gray);
                    active_positions
                        .insert(edge.child_module, active_modules.len());
                    active_modules.push(edge.child_module);
                    active_edges.push(edge);

                    frames.push(Frame {
                        module: active_modules[active_modules.len() - 1],
                        next_edge: 0,
                    });
                }
            }
        }
    }

    (cycles.into_iter().collect(), None, work_units)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::source::{SourceId, Span};
    use crate::frontend::ast::node::node_id::NodeId;
    use crate::hdl::design::instance::{HdlInstanceList, HdlModuleInstance};
    use crate::hdl::design::module::{HdlModule, ModuleName};

    fn node(raw: u64) -> NodeId {
        NodeId::new(raw).expect("test node ID must be non-zero")
    }

    fn span() -> Span {
        Span::point_usize(SourceId::from_raw(1), 0)
            .expect("test span should be valid")
    }

    fn name(value: &str) -> ModuleName {
        ModuleName::parse_qualified(value).expect("test module name must be valid")
    }

    fn module(raw: u64, module_name: &str) -> HdlModule {
        HdlModule::new(node(raw), name(module_name), span(), vec![], vec![])
            .expect("test module should be valid")
    }

    fn instance(raw: u64, instance_name: &str, module_name: &str) -> HdlModuleInstance {
        HdlModuleInstance::new(
            node(raw),
            instance_name,
            name(module_name),
            span(),
            None,
            None,
        )
        .expect("test instance should be valid")
    }

    fn instance_list(values: Vec<HdlModuleInstance>) -> HdlInstanceList {
        HdlInstanceList::from_instances(values)
            .expect("test instance list should be valid")
    }

    fn lists(
        values: impl IntoIterator<Item = (u64, Vec<HdlModuleInstance>)>,
    ) -> BTreeMap<NodeId, HdlInstanceList> {
        values
            .into_iter()
            .map(|(owner, instances)| (node(owner), instance_list(instances)))
            .collect()
    }

    #[test]
    fn empty_input_is_complete() {
        let result = analyze_hierarchy(
            &[],
            &BTreeMap::new(),
            &HierarchyAnalysisConfig::default(),
        );

        assert_eq!(result.status, HierarchyAnalysisStatus::Complete);
        assert!(result.roots.is_empty());
        assert!(result.cycles.is_empty());
        assert!(result.issues.is_empty());
    }

    #[test]
    fn infers_root_and_reachable_child() {
        let modules = vec![module(1, "Top"), module(2, "Child")];
        let instances = lists([(1, vec![instance(10, "child0", "Child")])]);

        let result = analyze_hierarchy(
            &modules,
            &instances,
            &HierarchyAnalysisConfig::default(),
        );

        assert_eq!(result.roots, vec![node(1)]);
        assert_eq!(result.reachable_modules, vec![node(1), node(2)]);
        assert!(result.unreachable_modules.is_empty());
        assert!(result.cycles.is_empty());
        assert!(result.issues.is_empty());
        assert!(result.status.is_complete());
    }

    #[test]
    fn reports_unresolved_module_reference() {
        let modules = vec![module(1, "Top")];
        let instances = lists([(1, vec![instance(10, "missing", "NotThere")])]);

        let result = analyze_hierarchy(
            &modules,
            &instances,
            &HierarchyAnalysisConfig::default(),
        );

        assert!(result.issues.iter().any(|issue| matches!(
            issue,
            HierarchyIssue::UnresolvedModuleReference {
                parent_module,
                instance_id,
                ..
            } if *parent_module == node(1) && *instance_id == node(10)
        )));
        assert!(result.status.is_complete());
    }

    #[test]
    fn detects_recursive_instantiation_without_recursion() {
        let modules = vec![
            module(1, "A"),
            module(2, "B"),
        ];
        let instances = lists([
            (1, vec![instance(11, "b", "B")]),
            (2, vec![instance(12, "a", "A")]),
        ]);

        let result = analyze_hierarchy(
            &modules,
            &instances,
            &HierarchyAnalysisConfig::default(),
        );

        assert_eq!(result.cycles.len(), 1);
        assert_eq!(result.cycles[0].modules.first(), result.cycles[0].modules.last());
        assert_eq!(
            result.cycles[0].instances.len() + 1,
            result.cycles[0].modules.len()
        );
    }

    #[test]
    fn repeated_instances_of_one_child_are_not_a_cycle() {
        let modules = vec![module(1, "Top"), module(2, "Child")];
        let instances = lists([(
            1,
            vec![
                instance(10, "first", "Child"),
                instance(11, "second", "Child"),
            ],
        )]);

        let result = analyze_hierarchy(
            &modules,
            &instances,
            &HierarchyAnalysisConfig::default(),
        );

        assert!(result.cycles.is_empty());
        assert_eq!(result.edges.len(), 2);
    }

    #[test]
    fn duplicate_module_names_are_ambiguous_not_guessed() {
        let modules = vec![
            module(1, "Top"),
            module(2, "Child"),
            module(3, "Child"),
        ];
        let instances = lists([(1, vec![instance(10, "child", "Child")])]);

        let result = analyze_hierarchy(
            &modules,
            &instances,
            &HierarchyAnalysisConfig::default(),
        );

        assert!(result.issues.iter().any(|issue| matches!(
            issue,
            HierarchyIssue::AmbiguousModuleReference { candidates, .. }
                if candidates == &vec![node(2), node(3)]
        )));
        assert!(result.issues.iter().any(|issue| matches!(
            issue,
            HierarchyIssue::DuplicateModuleName { .. }
        )));
    }

    #[test]
    fn explicit_root_is_respected() {
        let modules = vec![module(1, "A"), module(2, "B")];
        let config = HierarchyAnalysisConfig {
            roots: vec![name("B")],
            ..HierarchyAnalysisConfig::default()
        };

        let result =
            analyze_hierarchy(&modules, &BTreeMap::new(), &config);

        assert_eq!(result.roots, vec![node(2)]);
        assert_eq!(result.reachable_modules, vec![node(2)]);
        assert_eq!(result.unreachable_modules, vec![node(1)]);
    }

    #[test]
    fn exhausted_work_budget_is_explicit() {
        let modules = vec![module(1, "Top")];
        let config = HierarchyAnalysisConfig {
            max_work: Some(0),
            ..HierarchyAnalysisConfig::default()
        };

        let result =
            analyze_hierarchy(&modules, &BTreeMap::new(), &config);

        assert_eq!(
            result.status,
            HierarchyAnalysisStatus::Incomplete(
                HierarchyIncompleteReason::WorkBudgetExceeded
            )
        );
        assert!(!result.is_cycle_free_and_complete());
    }

    #[test]
    fn output_is_deterministic() {
        let modules = vec![
            module(1, "Top"),
            module(2, "B"),
            module(3, "A"),
        ];
        let instances = lists([(
            1,
            vec![
                instance(11, "b", "B"),
                instance(10, "a", "A"),
            ],
        )]);

        let first = analyze_hierarchy(
            &modules,
            &instances,
            &HierarchyAnalysisConfig::default(),
        );
        let second = analyze_hierarchy(
            &modules,
            &instances,
            &HierarchyAnalysisConfig::default(),
        );

        assert_eq!(first, second);
    }
}
