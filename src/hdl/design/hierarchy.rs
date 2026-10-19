
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Logical Design Hierarchy
//!
//! Path: `src/hdl/design/hierarchy.rs`
//! Rust baseline: 1.97 or later; Rust 2021 edition.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Build and analyze the logical module hierarchy from the existing
//! `HdlModule` and `HdlModuleInstance` design records.
//!
//! ## Authority and ownership
//!
//! Source syntax is owned by `grammar/hdl/hardware-modules.g4` and the
//! canonical frontend parser. The normative architecture is defined by
//! `grammar/spec/hdl.md`. Canonical AST identities and source spans remain
//! owned by the frontend AST. `design/module.rs` owns module definitions;
//! `design/instance.rs` owns instance declarations and per-scope instance
//! collections.
//!
//! This file owns only a derived hierarchy index and its structural
//! analysis results. It does not duplicate AST nodes, resolve arbitrary
//! expressions, substitute generics, elaborate generated designs, or
//! perform type/port checking.
//!
//! ## Does not own
//!
//! - Parsing, AST construction, or semantic name-resolution policy.
//! - Generic evaluation or generated-instance expansion.
//! - Port compatibility, connectivity, driver analysis, or timing.
//! - Physical device selection, placement, routing, or resource allocation.
//! - Simulation, synthesis, or target realization.
//! - Any HDLIR structures, builders, passes, lowering, or serialization.
//!
//! All HDLIR implementation remains deferred to `src/hdl/hdlir/`.
//!
//! ## Completeness contract
//!
//! The caller supplies all module definitions in the design universe being
//! analyzed and one `HdlInstanceList` for every supplied module. The list
//! must contain every instance declaration in that module's body.
//! This module verifies that supplied instances are body members, but it
//! cannot discover omitted instances without consulting the canonical AST.
//!
//! References to module names absent from the supplied universe are retained
//! as unresolved references. They may represent imports, external modules,
//! generated modules not yet elaborated, or genuinely missing definitions.
//! This module does not guess which case applies.
//!
//! ## Scalability and determinism
//!
//! No language-level maximum is imposed on modules, instances, or hierarchy
//! depth. Traversals are iterative to avoid consuming the Rust call stack in
//! proportion to user-controlled hierarchy depth. Storage grows with the
//! supplied design and available resources.
//!
//! Results are deterministic for the same validated inputs and source-order
//! instance lists. Recursive-cycle output contains deterministic cycle
//! witnesses; it is not an enumeration of every possible simple cycle,
//! which can be exponentially large.
//!
//! Rust memory, address space, and representable collection sizes are finite.
//! POCO-REAF means this module introduces no additional artificial hardware
//! ceiling.
//!
//! ## Integration
//!
//! The frontend/semantic adapter constructs `HdlModule` and
//! `HdlInstanceList` values. A design owner calls `HdlHierarchy::build`
//! with the complete module list and a scope map keyed by module declaration
//! `NodeId`. Consumers read the resulting `HdlHierarchyReport`.
//!
//! This module depends only on the existing design records and canonical
//! AST identity type. It must not depend on parser internals, target
//! realization, runtime code, or HDLIR.
//!
//! ## Freeze criteria
//!
//! - Existing `ModuleName`, `HdlModule`, `HdlModuleInstance`,
//!   `HdlInstanceList`, and `NodeId` are reused.
//! - Missing module definitions remain explicit, not silently discarded.
//! - Missing or extraneous instance scopes are rejected.
//! - Hierarchy and cycle traversal are iterative.
//! - Tests cover roots, shared module definitions, unresolved references,
//!   recursion, deep hierarchy, duplicate definitions, and invalid scopes.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::frontend::ast::node::node_id::NodeId;

use super::instance::{HdlInstanceList, HdlModuleInstance};
use super::module::{HdlModule, ModuleName};

/// Version of this hierarchy analysis API.
///
/// Independent of the Zamani language, AST, semantic-model, and future
/// HDLIR schema versions.
pub const HDL_HIERARCHY_SCHEMA_VERSION: u16 = 1;

/// A validated logical module hierarchy.
///
/// Modules are indexed by their qualified names. Edges represent known
/// references from a containing module to a referenced module definition.
/// References without a supplied definition are retained separately.
#[derive(Clone, Debug)]
pub struct HdlHierarchy {
    modules: Vec<HdlModule>,
    edges: Vec<Vec<HierarchyEdge>>,
    index_by_name: BTreeMap<ModuleName, usize>,
    index_by_declaration: BTreeMap<NodeId, usize>,
    unresolved: Vec<UnresolvedModuleReference>,
}

/// An edge from one known module definition to another.
#[derive(Clone, Debug, Eq, PartialEq)]
struct HierarchyEdge {
    /// Canonical AST identity of the instance declaration.
    instance_declaration: NodeId,
    /// Local source-level instance name.
    instance_name: String,
    /// Index of the referenced module in `HdlHierarchy::modules`.
    target: usize,
}

/// A module instance whose referenced definition was not supplied.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnresolvedModuleReference {
    containing_module: ModuleName,
    instance_declaration: NodeId,
    instance_name: String,
    referenced_module: ModuleName,
}

impl UnresolvedModuleReference {
    /// Return the qualified name of the containing module.
    #[must_use]
    pub fn containing_module(&self) -> &ModuleName {
        &self.containing_module
    }

    /// Return the canonical AST identity of the instance declaration.
    #[must_use]
    pub const fn instance_declaration(&self) -> NodeId {
        self.instance_declaration
    }

    /// Return the local source-level instance name.
    #[must_use]
    pub fn instance_name(&self) -> &str {
        &self.instance_name
    }

    /// Return the qualified name of the referenced module.
    #[must_use]
    pub fn referenced_module(&self) -> &ModuleName {
        &self.referenced_module
    }
}

/// A report of structural hierarchy analysis.
///
/// A root is a supplied module with no incoming edges from any other supplied
/// module. Reachability is computed by following known edges from those
/// roots. Unresolved references do not create edges to unknown definitions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlHierarchyReport {
    roots: Vec<ModuleName>,
    reachable_modules: Vec<ModuleName>,
    unreachable_modules: Vec<ModuleName>,
    unresolved_references: Vec<UnresolvedModuleReference>,
    recursive_cycles: Vec<Vec<ModuleName>>,
}

impl HdlHierarchyReport {
    /// Return modules with no incoming edges from known supplied modules.
    ///
    /// The result is ordered by qualified module name.
    #[must_use]
    pub fn roots(&self) -> &[ModuleName] {
        &self.roots
    }

    /// Return modules reachable from at least one root.
    ///
    /// The result is ordered by qualified module name, not traversal order.
    #[must_use]
    pub fn reachable_modules(&self) -> &[ModuleName] {
        &self.reachable_modules
    }

    /// Return supplied modules not reachable from any root.
    ///
    /// A module in a closed recursive component may appear here because that
    /// component has no root. This is a structural observation, not by itself
    /// proof that the source program is invalid.
    #[must_use]
    pub fn unreachable_modules(&self) -> &[ModuleName] {
        &self.unreachable_modules
    }

    /// Return references whose target definitions were not supplied.
    #[must_use]
    pub fn unresolved_references(&self) -> &[UnresolvedModuleReference] {
        &self.unresolved_references
    }

    /// Return deterministic witnesses of recursive module-instantiation
    /// cycles. Each witness repeats its first module at the end.
    ///
    /// The witnesses prove that recursion exists, but are not guaranteed to
    /// enumerate every simple cycle in a graph with many overlapping cycles.
    #[must_use]
    pub fn recursive_cycles(&self) -> &[Vec<ModuleName>] {
        &self.recursive_cycles
    }

    /// Return whether every supplied reference resolved to a supplied module.
    #[must_use]
    pub fn has_unresolved_references(&self) -> bool {
        !self.unresolved_references.is_empty()
    }

    /// Return whether a recursive-instantiation cycle was detected.
    #[must_use]
    pub fn has_recursive_cycles(&self) -> bool {
        !self.recursive_cycles.is_empty()
    }

    /// Return whether every supplied module is reachable from a root.
    ///
    /// This is false for a closed recursive component with no root.
    #[must_use]
    pub fn all_modules_reachable(&self) -> bool {
        self.unreachable_modules.is_empty()
    }
}

impl HdlHierarchy {
    /// Build a hierarchy from validated design-layer records.
    ///
    /// `modules` is the complete set of module definitions for this analysis.
    /// `instances_by_module` maps each module declaration's canonical AST
    /// identity to that module's complete, source-ordered instance list.
    ///
    /// The build rejects duplicate module names or declaration identities,
    /// missing or extraneous scopes, invalid modules, invalid instances, and
    /// instances that are not listed among their containing module's body
    /// members. Unknown referenced module names are recorded in the analysis
    /// report instead of causing this function to fail.
    ///
    /// # Errors
    ///
    /// Returns [`HierarchyError`] if the supplied structural records are
    /// inconsistent or invalid.
    pub fn build(
        mut modules: Vec<HdlModule>,
        instances_by_module: BTreeMap<NodeId, HdlInstanceList>,
    ) -> Result<Self, HierarchyError> {
        for module in &modules {
            module.validate().map_err(|error| {
                HierarchyError::InvalidModule {
                    module: module.name().clone(),
                    reason: error.to_string(),
                }
            })?;
        }

        // Sorting first makes indexes and traversal order independent of the
        // order in which otherwise-identical module records were supplied.
        modules.sort_by(|left, right| left.name().cmp(right.name()));

        let mut index_by_name = BTreeMap::new();
        let mut index_by_declaration = BTreeMap::new();

        for (index, module) in modules.iter().enumerate() {
            let name = module.name().clone();
            let declaration = module.declaration_id();

            if index_by_name.insert(name.clone(), index).is_some() {
                return Err(HierarchyError::DuplicateModuleName { module: name });
            }

            if index_by_declaration
                .insert(declaration, index)
                .is_some()
            {
                return Err(HierarchyError::DuplicateModuleDeclaration {
                    declaration,
                    module: name,
                });
            }
        }

        for module in &modules {
            if !instances_by_module.contains_key(&module.declaration_id()) {
                return Err(HierarchyError::MissingInstanceScope {
                    module: module.name().clone(),
                    declaration: module.declaration_id(),
                });
            }
        }

        for declaration in instances_by_module.keys() {
            if !index_by_declaration.contains_key(declaration) {
                return Err(HierarchyError::UnexpectedInstanceScope {
                    declaration: *declaration,
                });
            }
        }

        let mut edges = vec![Vec::new(); modules.len()];
        let mut unresolved = Vec::new();

        for (source_index, module) in modules.iter().enumerate() {
            let declaration = module.declaration_id();
            let instance_list = instances_by_module
                .get(&declaration)
                .ok_or_else(|| HierarchyError::MissingInstanceScope {
                    module: module.name().clone(),
                    declaration,
                })?;

            for instance in instance_list.as_slice() {
                instance.validate().map_err(|error| {
                    HierarchyError::InvalidInstance {
                        containing_module: module.name().clone(),
                        instance_name: instance.name().to_owned(),
                        reason: error.to_string(),
                    }
                })?;

                if !module.body_members().contains(&instance.declaration_id()) {
                    return Err(HierarchyError::InstanceNotInModuleBody {
                        containing_module: module.name().clone(),
                        instance_name: instance.name().to_owned(),
                        instance_declaration: instance.declaration_id(),
                    });
                }

                if let Some(&target) = index_by_name.get(instance.module_reference()) {
                    edges[source_index].push(HierarchyEdge {
                        instance_declaration: instance.declaration_id(),
                        instance_name: instance.name().to_owned(),
                        target,
                    });
                } else {
                    unresolved.push(UnresolvedModuleReference {
                        containing_module: module.name().clone(),
                        instance_declaration: instance.declaration_id(),
                        instance_name: instance.name().to_owned(),
                        referenced_module: instance.module_reference().clone(),
                    });
                }
            }
        }

        // Modules are sorted by qualified name and each instance list retains
        // source order, so the unresolved-reference order is reproducible.
        Ok(Self {
            modules,
            edges,
            index_by_name,
            index_by_declaration,
            unresolved,
        })
    }

    /// Return the number of supplied module definitions.
    #[must_use]
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    /// Find a module by its qualified name.
    #[must_use]
    pub fn module(&self, name: &ModuleName) -> Option<&HdlModule> {
        self.index_by_name
            .get(name)
            .map(|&index| &self.modules[index])
    }

    /// Find a module by its canonical declaration identity.
    #[must_use]
    pub fn module_by_declaration(&self, declaration: NodeId) -> Option<&HdlModule> {
        self.index_by_declaration
            .get(&declaration)
            .map(|&index| &self.modules[index])
    }

    /// Return module definitions in qualified-name order.
    #[must_use]
    pub fn modules(&self) -> &[HdlModule] {
        &self.modules
    }

    /// Analyze roots, reachability, unresolved references, and recursion.
    ///
    /// This operation uses iterative graph traversals. It does not expand
    /// generic parameters or generated instances.
    #[must_use]
    pub fn analyze(&self) -> HdlHierarchyReport {
        let mut has_incoming = vec![false; self.modules.len()];

        for outgoing in &self.edges {
            for edge in outgoing {
                has_incoming[edge.target] = true;
            }
        }

        let roots: Vec<usize> = has_incoming
            .iter()
            .enumerate()
            .filter_map(|(index, incoming)| (!incoming).then_some(index))
            .collect();

        let mut reachable = vec![false; self.modules.len()];
        let mut worklist = roots.clone();

        while let Some(index) = worklist.pop() {
            if reachable[index] {
                continue;
            }

            reachable[index] = true;

            for edge in self.edges[index].iter().rev() {
                if !reachable[edge.target] {
                    worklist.push(edge.target);
                }
            }
        }

        let root_names = roots
            .iter()
            .map(|&index| self.modules[index].name().clone())
            .collect();

        let reachable_modules = reachable
            .iter()
            .enumerate()
            .filter_map(|(index, reached)| {
                reached.then(|| self.modules[index].name().clone())
            })
            .collect();

        let unreachable_modules = reachable
            .iter()
            .enumerate()
            .filter_map(|(index, reached)| {
                (!reached).then(|| self.modules[index].name().clone())
            })
            .collect();

        let recursive_cycles = self.find_cycle_witnesses();

        HdlHierarchyReport {
            roots: root_names,
            reachable_modules,
            unreachable_modules,
            unresolved_references: self.unresolved.clone(),
            recursive_cycles,
        }
    }

    /// Find deterministic witnesses for directed cycles using iterative DFS.
    ///
    /// The active DFS path is tracked explicitly, avoiding call-stack growth
    /// proportional to user-controlled hierarchy depth.
    fn find_cycle_witnesses(&self) -> Vec<Vec<ModuleName>> {
        // 0 = unseen, 1 = on active DFS path, 2 = fully explored.
        let mut color = vec![0_u8; self.modules.len()];
        let mut path = Vec::<usize>::new();
        let mut path_position = BTreeMap::<usize, usize>::new();
        let mut cycles = Vec::<Vec<ModuleName>>::new();

        for start in 0..self.modules.len() {
            if color[start] != 0 {
                continue;
            }

            color[start] = 1;
            path_position.insert(start, 0);
            path.push(start);

            // Each frame is (node index, next outgoing edge index).
            let mut frames = vec![(start, 0_usize)];

            while let Some((node, next_edge)) = frames.last_mut() {
                if *next_edge >= self.edges[*node].len() {
                    let finished = *node;
                    frames.pop();
                    color[finished] = 2;
                    path_position.remove(&finished);
                    let _ = path.pop();
                    continue;
                }

                let target = self.edges[*node][*next_edge].target;
                *next_edge += 1;

                match color[target] {
                    0 => {
                        color[target] = 1;
                        path_position.insert(target, path.len());
                        path.push(target);
                        frames.push((target, 0));
                    }
                    1 => {
                        if let Some(&cycle_start) = path_position.get(&target) {
                            let mut witness: Vec<ModuleName> = path[cycle_start..]
                                .iter()
                                .map(|&index| self.modules[index].name().clone())
                                .collect();

                            witness.push(self.modules[target].name().clone());
                            cycles.push(witness);
                        }
                    }
                    _ => {}
                }
            }
        }

        cycles
    }
}

/// Structural failures while building a logical hierarchy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HierarchyError {
    /// A module's own local invariants are invalid.
    InvalidModule {
        /// The module's qualified name.
        module: ModuleName,
        /// Explanation supplied by the module validator.
        reason: String,
    },
    /// Two supplied module definitions use the same qualified name.
    DuplicateModuleName {
        /// The duplicated qualified name.
        module: ModuleName,
    },
    /// Two supplied module definitions use the same AST declaration ID.
    DuplicateModuleDeclaration {
        /// The repeated declaration identity.
        declaration: NodeId,
        /// Name associated with the duplicate declaration.
        module: ModuleName,
    },
    /// A module does not have an instance list in the supplied scope map.
    MissingInstanceScope {
        /// The module needing a scope.
        module: ModuleName,
        /// Canonical module declaration identity.
        declaration: NodeId,
    },
    /// An instance list was supplied for a declaration not in the module set.
    UnexpectedInstanceScope {
        /// The declaration identity without a matching supplied module.
        declaration: NodeId,
    },
    /// An instance is not listed among its containing module's body members.
    InstanceNotInModuleBody {
        /// Containing module name.
        containing_module: ModuleName,
        /// Local instance name.
        instance_name: String,
        /// Instance declaration identity.
        instance_declaration: NodeId,
    },
    /// A supplied instance failed its local invariants.
    InvalidInstance {
        /// Containing module name.
        containing_module: ModuleName,
        /// Local instance name.
        instance_name: String,
        /// Explanation supplied by the instance validator.
        reason: String,
    },
}

impl fmt::Display for HierarchyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidModule { module, reason } => {
                write!(f, "invalid HDL module {module:?}: {reason}")
            }
            Self::DuplicateModuleName { module } => {
                write!(f, "duplicate HDL module name {module:?}")
            }
            Self::DuplicateModuleDeclaration {
                declaration,
                module,
            } => write!(
                f,
                "HDL module {module:?} reuses declaration identity {declaration:?}"
            ),
            Self::MissingInstanceScope {
                module,
                declaration,
            } => write!(
                f,
                "HDL module {module:?} (declaration {declaration:?}) has no instance scope"
            ),
            Self::UnexpectedInstanceScope { declaration } => write!(
                f,
                "instance scope refers to unknown module declaration {declaration:?}"
            ),
            Self::InstanceNotInModuleBody {
                containing_module,
                instance_name,
                instance_declaration,
            } => write!(
                f,
                "instance {instance_name:?} ({instance_declaration:?}) is not a body member of module {containing_module:?}"
            ),
            Self::InvalidInstance {
                containing_module,
                instance_name,
                reason,
            } => write!(
                f,
                "invalid instance {instance_name:?} in module {containing_module:?}: {reason}"
            ),
        }
    }
}

impl std::error::Error for HierarchyError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::source::{SourceId, Span};
    use crate::hdl::design::instance::HdlModuleInstance;
    use crate::hdl::design::module::ModuleName;

    fn node_id(raw: u64) -> NodeId {
        NodeId::new(raw).expect("test NodeId must be non-zero")
    }

    fn span() -> Span {
        Span::point_usize(SourceId::from_raw(1), 0)
            .expect("test span should be representable")
    }

    fn module(raw: u64, name: &str, body: Vec<NodeId>) -> HdlModule {
        HdlModule::new(
            node_id(raw),
            ModuleName::parse_qualified(name).expect("valid test module name"),
            span(),
            Vec::new(),
            body,
        )
        .expect("valid test module")
    }

    fn instance(raw: u64, name: &str, target: &str) -> HdlModuleInstance {
        HdlModuleInstance::new(
            node_id(raw),
            name,
            ModuleName::parse_qualified(target).expect("valid target name"),
            span(),
            None,
            None,
        )
        .expect("valid test instance")
    }

    fn scopes(
        entries: Vec<(u64, Vec<HdlModuleInstance>)>,
    ) -> BTreeMap<NodeId, HdlInstanceList> {
        entries
            .into_iter()
            .map(|(module_id, instances)| {
                (
                    node_id(module_id),
                    HdlInstanceList::from_instances(instances)
                        .expect("valid instance list"),
                )
            })
            .collect()
    }

    #[test]
    fn finds_root_and_reachable_children() {
        let top = module(1, "Top", vec![node_id(10)]);
        let child = module(2, "Child", vec![]);

        let hierarchy = HdlHierarchy::build(
            vec![child, top],
            scopes(vec![
                (1, vec![instance(10, "u_child", "Child")]),
                (2, vec![]),
            ]),
        )
        .expect("valid hierarchy");

        let report = hierarchy.analyze();

        assert_eq!(
            report.roots(),
            &[ModuleName::parse_qualified("Top").unwrap()]
        );
        assert_eq!(report.reachable_modules().len(), 2);
        assert!(report.unresolved_references().is_empty());
        assert!(!report.has_recursive_cycles());
    }

    #[test]
    fn records_unresolved_references_without_guessing() {
        let top = module(1, "Top", vec![node_id(10)]);

        let hierarchy = HdlHierarchy::build(
            vec![top],
            scopes(vec![(1, vec![instance(10, "external", "vendor::External")])]),
        )
        .expect("unknown targets are reportable, not structural build failures");

        let report = hierarchy.analyze();

        assert!(report.has_unresolved_references());
        assert_eq!(
            report.unresolved_references()[0].referenced_module(),
            &ModuleName::parse_qualified("vendor::External").unwrap()
        );
    }

    #[test]
    fn detects_direct_recursive_instantiation() {
        let top = module(1, "Top", vec![node_id(10)]);

        let hierarchy = HdlHierarchy::build(
            vec![top],
            scopes(vec![(1, vec![instance(10, "self", "Top")])]),
        )
        .expect("self-instantiation is represented for analysis");

        let report = hierarchy.analyze();

        assert!(report.has_recursive_cycles());
        assert_eq!(report.recursive_cycles().len(), 1);
        assert_eq!(report.recursive_cycles()[0].len(), 2);
    }

    #[test]
    fn detects_mutual_recursive_instantiation() {
        let a = module(1, "A", vec![node_id(10)]);
        let b = module(2, "B", vec![node_id(20)]);

        let hierarchy = HdlHierarchy::build(
            vec![a, b],
            scopes(vec![
                (1, vec![instance(10, "b", "B")]),
                (2, vec![instance(20, "a", "A")]),
            ]),
        )
        .expect("cycle can be reported by analysis");

        let report = hierarchy.analyze();

        assert!(report.has_recursive_cycles());
        assert_eq!(report.recursive_cycles()[0].first(),
                   report.recursive_cycles()[0].last());
    }

    #[test]
    fn shared_module_definition_is_not_a_duplicate() {
        let top = module(1, "Top", vec![node_id(10), node_id(11)]);
        let leaf = module(2, "Leaf", vec![]);

        let hierarchy = HdlHierarchy::build(
            vec![top, leaf],
            scopes(vec![
                (
                    1,
                    vec![
                        instance(10, "left", "Leaf"),
                        instance(11, "right", "Leaf"),
                    ],
                ),
                (2, vec![]),
            ]),
        )
        .expect("multiple instances may share one definition");

        assert_eq!(hierarchy.analyze().reachable_modules().len(), 2);
    }

    #[test]
    fn rejects_missing_instance_scope() {
        let top = module(1, "Top", vec![]);

        let result = HdlHierarchy::build(vec![top], BTreeMap::new());

        assert!(matches!(
            result,
            Err(HierarchyError::MissingInstanceScope { .. })
        ));
    }

    #[test]
    fn rejects_instance_not_in_containing_module_body() {
        let top = module(1, "Top", vec![]);

        let result = HdlHierarchy::build(
            vec![top],
            scopes(vec![(1, vec![instance(10, "child", "Child")])]),
        );

        assert!(matches!(
            result,
            Err(HierarchyError::InstanceNotInModuleBody { .. })
        ));
    }
}
