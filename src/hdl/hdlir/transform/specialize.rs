
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Parameter Specialization Planning
//!
//! Path: `src/hdl/hdlir/transform/specialize.rs`
//! Rust edition: 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Deterministic parameter-specialization planning; whole-design
//! mutation is intentionally not implemented without a verified design API.
//!
//! ## Purpose
//!
//! Build an immutable plan for specializing a parameterized HDL design.
//! Select values using the canonical HDLIR parameter API, discover
//! dependencies between declared parameters, and compute a deterministic
//! dependency order.
//!
//! The plan reports unresolved parameters, pending constraints, and
//! parameters blocked by dependency cycles. It does not evaluate expressions,
//! prove constraints, mutate a design, or claim that a target can realize it.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`.
//! - `grammar/spec/hdl.md`.
//! - `grammar/hdl/hdl.g4`.
//! - `grammar/hdl/hardware-generics.g4`.
//! - `grammar/hdl/hardware-modules.g4`.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`.
//! - `src/hdl/hdlir/values/parameters.rs`.
//! - `src/hdl/hdlir/values/expressions.rs`.
//! - `src/hdl/hdlir/types/bit_width.rs`.
//! - `src/zuir/ZUIR-SPEC.md`.
//!
//! Grammar defines syntax; the canonical semantic layer defines source
//! meaning. HDLIR owns hardware-domain representation. Physical allocation,
//! synthesis, placement, routing, and target feasibility remain downstream.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - A deterministic parameter-dependency graph for one planning invocation.
//! - Dependency ordering and cycle-blocked reporting.
//! - Optional operational budgets for parameter and dependency processing.
//! - A result that preserves the underlying parameter-resolution plan.
//!
//! It does not own:
//!
//! - Source parsing, name resolution, type inference, or semantic validation.
//! - Expression evaluation or constant folding.
//! - Proof or evaluation of parameter constraints.
//! - The authoritative HDLIR design/module graph.
//! - Mutation, cloning, or rewriting of modules or designs.
//! - Target selection, physical resource allocation, synthesis, or execution.
//!
//! ## Semantics and correctness
//!
//! A symbolic reference creates a dependency edge only when its name matches
//! a declared parameter in the same `ParameterSet`. Other symbols may belong
//! to namespaces this planner does not own and are not declared missing here.
//!
//! Width symbols and parameter-expression symbols are inspected without
//! evaluating them. Constraint dependencies are included in the graph, but
//! the constraints remain pending until an authoritative evaluator checks
//! them.
//!
//! A dependency cycle prevents a complete topological order. The planner
//! reports all parameters left blocked by that cycle, including parameters
//! downstream of a cycle; it does not claim every reported parameter is itself
//! a member of the minimal cycle.
//!
//! An order is a planning result, not proof that the design is fully
//! elaborated, semantically valid, equivalent after rewriting, or realizable.
//!
//! ## Scalability
//!
//! No universal parameter-count, width, module-count, or hardware-capacity
//! limit is imposed. Optional budgets are operational controls. Counts use
//! checked arithmetic, graph traversal is iterative, and exhaustion returns
//! an error rather than a successful partial plan.
//!
//! `None` means this API imposes no explicit budget for that unit of work;
//! it does not guarantee infinite resources. Standard-library allocation
//! remains subject to host resource availability.
//!
//! ## Integration
//!
//! - `values/parameters.rs` owns declarations, bindings, and resolution.
//! - `values/expressions.rs` owns expression graph structure and symbol
//!   discovery.
//! - `types/bit_width.rs` owns symbolic width representation.
//! - A future verified design/module API must consume the plan and perform
//!   substitutions transactionally, then run required verification.
//! - A future expression evaluator must evaluate expressions according to
//!   authoritative operator, width, signedness, and logic-state semantics.
//! - A future constraint evaluator must explicitly prove or reject pending
//!   constraints; this planner does not interpret their predicates.
//! - `passes/contracts.rs` must define preservation and invalidation rules
//!   before a pass manager registers this planning operation as a mutating pass.
//!
//! ## Freeze criteria
//!
//! The parent `transform/mod.rs` must register this module and export its
//! intended public API. Sibling APIs must compile together. Repository
//! integration tests must verify dependency ordering, cycle reporting,
//! unresolved values, pending constraints, budgets, and deterministic output.
//! Full-design specialization remains incomplete until a verified graph
//! mutation API and semantic-preservation tests exist.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::hdlir::types::bit_width::BitWidth;
use crate::hdl::hdlir::values::parameters::{
    ParameterName,
    ParameterResolution,
    ParameterSet,
    ParameterValue,
};

/// Optional operational budgets for specialization planning.
///
/// These limits apply only to this planning invocation. They do not restrict
/// the HDL language or the hardware a target may support.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpecializationConfig {
    max_parameters: Option<u64>,
    max_dependency_edges: Option<u64>,
}

impl SpecializationConfig {
    /// Create a configuration without explicit planning budgets.
    ///
    /// This does not promise infinite memory or execution time.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_parameters: None,
            max_dependency_edges: None,
        }
    }

    /// Set the maximum number of declared parameters to process.
    ///
    /// A zero budget permits an empty parameter set only.
    #[must_use]
    pub const fn with_max_parameters(mut self, limit: u64) -> Self {
        self.max_parameters = Some(limit);
        self
    }

    /// Set the maximum number of unique parameter dependency edges to process.
    ///
    /// A zero budget permits only a graph without dependency edges.
    #[must_use]
    pub const fn with_max_dependency_edges(mut self, limit: u64) -> Self {
        self.max_dependency_edges = Some(limit);
        self
    }

    /// Return the optional parameter-processing budget.
    #[must_use]
    pub const fn max_parameters(self) -> Option<u64> {
        self.max_parameters
    }

    /// Return the optional dependency-edge budget.
    #[must_use]
    pub const fn max_dependency_edges(self) -> Option<u64> {
        self.max_dependency_edges
    }
}

impl Default for SpecializationConfig {
    fn default() -> Self {
        Self::unlimited()
    }
}

/// Errors that prevent a complete specialization plan from being produced.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpecializationError {
    /// The number of declared parameters exceeds the configured budget.
    ParameterBudgetExceeded {
        /// Configured budget.
        limit: u64,
        /// Number of parameters requested.
        requested: u64,
    },

    /// The number of unique dependency edges exceeds the configured budget.
    DependencyBudgetExceeded {
        /// Configured budget.
        limit: u64,
        /// Number of unique edges requested.
        requested: u64,
    },

    /// A planning counter cannot be represented by `u64`.
    CountOverflow,
}

impl fmt::Display for SpecializationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParameterBudgetExceeded { limit, requested } => write!(
                formatter,
                "specialization parameter budget exceeded: \
                 limit {limit}, requested {requested}"
            ),
            Self::DependencyBudgetExceeded { limit, requested } => write!(
                formatter,
                "specialization dependency budget exceeded: \
                 limit {limit}, requested {requested}"
            ),
            Self::CountOverflow => {
                formatter.write_str("specialization planning counter overflow")
            }
        }
    }
}

impl std::error::Error for SpecializationError {}

/// Counts describing a completed specialization-planning invocation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpecializationStats {
    /// Number of declared parameters processed.
    pub parameters_processed: u64,

    /// Number of unique parameter dependency edges retained.
    pub dependency_edges: u64,

    /// Number of parameters placed in the dependency order.
    pub parameters_ordered: u64,

    /// Number of parameters blocked by a dependency cycle.
    ///
    /// This can include dependents of a cycle, not just nodes in the cycle.
    pub parameters_blocked_by_cycle: u64,
}

/// Immutable specialization-planning result.
///
/// `dependency_order` lists parameters whose dependencies can be ordered,
/// with each dependency appearing before its dependents. Parameters in
/// `blocked_by_cycle` are omitted from that order.
///
/// The retained [`ParameterResolution`] is the authoritative record of
/// selected values and unresolved parameters. Pending constraints remain
/// unproven.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpecializationPlan {
    resolution: ParameterResolution,
    dependencies: BTreeMap<ParameterName, BTreeSet<ParameterName>>,
    dependency_order: Vec<ParameterName>,
    blocked_by_cycle: BTreeSet<ParameterName>,
    stats: SpecializationStats,
}

impl SpecializationPlan {
    /// Return the parameter-selection result.
    #[must_use]
    pub const fn resolution(&self) -> &ParameterResolution {
        &self.resolution
    }

    /// Return each declared parameter's direct dependencies.
    ///
    /// Both the map and each dependency set use deterministic name ordering.
    #[must_use]
    pub const fn dependencies(
        &self,
    ) -> &BTreeMap<ParameterName, BTreeSet<ParameterName>> {
        &self.dependencies
    }

    /// Return the deterministic topological order for parameters not blocked
    /// by dependency cycles.
    ///
    /// This is not a claim that symbolic values have been evaluated.
    #[must_use]
    pub fn dependency_order(&self) -> &[ParameterName] {
        &self.dependency_order
    }

    /// Return parameters blocked by dependency cycles.
    ///
    /// The set can include parameters that depend transitively on a cycle.
    #[must_use]
    pub const fn blocked_by_cycle(&self) -> &BTreeSet<ParameterName> {
        &self.blocked_by_cycle
    }

    /// Return statistics for this completed planning invocation.
    #[must_use]
    pub const fn stats(&self) -> SpecializationStats {
        self.stats
    }

    /// Return whether every declared parameter has a selected, non-unresolved
    /// value.
    ///
    /// Symbolic expressions may still be unevaluated, and constraints may
    /// still be pending.
    #[must_use]
    pub fn has_no_unresolved_parameters(&self) -> bool {
        self.resolution.has_no_unresolved_parameters()
    }

    /// Return whether any parameter constraints remain unevaluated.
    #[must_use]
    pub fn has_pending_constraints(&self) -> bool {
        self.resolution.has_pending_constraints()
    }

    /// Return whether all parameters can be placed in dependency order.
    ///
    /// This does not mean the design is fully specialized or verified.
    #[must_use]
    pub fn has_complete_dependency_order(&self) -> bool {
        self.blocked_by_cycle.is_empty()
            && self.dependency_order.len() == self.dependencies.len()
    }

    /// Return whether the plan has no unresolved parameters, no pending
    /// constraints, and no dependency cycles.
    ///
    /// Even `true` does not prove expression evaluation, source-level type
    /// correctness, design verification, semantic equivalence after rewriting,
    /// or target feasibility.
    #[must_use]
    pub fn is_ready_for_downstream_evaluation(&self) -> bool {
        self.has_no_unresolved_parameters()
            && !self.has_pending_constraints()
            && self.has_complete_dependency_order()
    }
}

/// Create a deterministic specialization plan for a parameter set.
///
/// This function selects explicit bindings and defaults using
/// [`ParameterSet::resolve`], discovers references to other declared
/// parameters, and computes a deterministic dependency order.
///
/// It does not mutate `parameters`.
///
/// # Errors
///
/// Returns an error if a configured operational budget is exceeded or a
/// planning counter overflows. Parameter declaration and binding validation
/// are performed by the existing parameter API before this function is called.
pub fn plan_specialization(
    parameters: &ParameterSet,
    config: SpecializationConfig,
) -> Result<SpecializationPlan, SpecializationError> {
    let parameter_count = u64::try_from(parameters.len())
        .map_err(|_| SpecializationError::CountOverflow)?;

    if let Some(limit) = config.max_parameters {
        if parameter_count > limit {
            return Err(SpecializationError::ParameterBudgetExceeded {
                limit,
                requested: parameter_count,
            });
        }
    }

    let resolution = parameters.resolve();
    let declared_names: BTreeSet<ParameterName> =
        parameters.declarations().keys().cloned().collect();

    let mut dependencies =
        BTreeMap::<ParameterName, BTreeSet<ParameterName>>::new();
    let mut edge_count = 0_u64;

    for (name, declaration) in parameters.declarations() {
        let mut parameter_dependencies = BTreeSet::new();

        if let Some(selected) = resolution.parameters().get(name) {
            collect_value_dependencies(
                selected.value(),
                &declared_names,
                &mut parameter_dependencies,
            );
        }

        // Constraints contribute dependency edges but are not evaluated here.
        for constraint in declaration.constraints() {
            for symbol in constraint.predicate().referenced_symbols() {
                if let Some(dependency) = declared_parameter(&declared_names, &symbol)
                {
                    parameter_dependencies.insert(dependency);
                }
            }
        }

        edge_count = edge_count
            .checked_add(
                u64::try_from(parameter_dependencies.len())
                    .map_err(|_| SpecializationError::CountOverflow)?,
            )
            .ok_or(SpecializationError::CountOverflow)?;

        if let Some(limit) = config.max_dependency_edges {
            if edge_count > limit {
                return Err(SpecializationError::DependencyBudgetExceeded {
                    limit,
                    requested: edge_count,
                });
            }
        }

        dependencies.insert(name.clone(), parameter_dependencies);
    }

    let (dependency_order, blocked_by_cycle) =
        topological_order(&dependencies)?;

    let ordered_count = u64::try_from(dependency_order.len())
        .map_err(|_| SpecializationError::CountOverflow)?;
    let blocked_count = u64::try_from(blocked_by_cycle.len())
        .map_err(|_| SpecializationError::CountOverflow)?;

    Ok(SpecializationPlan {
        resolution,
        dependencies,
        dependency_order,
        blocked_by_cycle,
        stats: SpecializationStats {
            parameters_processed: parameter_count,
            dependency_edges: edge_count,
            parameters_ordered: ordered_count,
            parameters_blocked_by_cycle: blocked_count,
        },
    })
}

/// Add declared-parameter references from a selected value.
///
/// References to names outside the declared parameter set are intentionally
/// ignored: this planner does not own other symbol namespaces.
fn collect_value_dependencies(
    value: &ParameterValue,
    declared_names: &BTreeSet<ParameterName>,
    output: &mut BTreeSet<ParameterName>,
) {
    match value {
        ParameterValue::Constant(_) | ParameterValue::TypeReference(_) => {}

        ParameterValue::Width(width) => {
            collect_width_dependencies(width, declared_names, output);
        }

        ParameterValue::Expression(expression) => {
            for symbol in expression.referenced_symbols() {
                if let Some(dependency) =
                    declared_parameter(declared_names, &symbol)
                {
                    output.insert(dependency);
                }
            }
        }

        ParameterValue::Unresolved { .. } => {
            // An unresolved marker is not an evaluated expression. Its
            // contents must not be interpreted as a proven dependency.
        }
    }
}

/// Discover parameter references in a symbolic width without evaluating it.
///
/// The traversal is iterative so nested width expressions do not consume
/// call-stack space proportional to expression depth.
fn collect_width_dependencies(
    width: &BitWidth,
    declared_names: &BTreeSet<ParameterName>,
    output: &mut BTreeSet<ParameterName>,
) {
    let mut pending = vec![width];

    while let Some(current) = pending.pop() {
        match current {
            BitWidth::Known(_) => {}

            BitWidth::Symbol(symbol) | BitWidth::Unresolved(symbol) => {
                if let Some(dependency) =
                    declared_parameter(declared_names, symbol.as_str())
                {
                    output.insert(dependency);
                }
            }

            BitWidth::Add(lhs, rhs) | BitWidth::Multiply(lhs, rhs) => {
                pending.push(lhs);
                pending.push(rhs);
            }
        }
    }
}

/// Resolve a spelling to a declared parameter identity without performing
/// source-level name resolution or case normalization.
fn declared_parameter(
    declared_names: &BTreeSet<ParameterName>,
    spelling: &str,
) -> Option<ParameterName> {
    declared_names
        .iter()
        .find(|name| name.as_str() == spelling)
        .cloned()
}

/// Compute a deterministic topological order.
///
/// For each parameter, its indegree is the number of distinct parameters it
/// depends on. A sorted ready set makes the result deterministic when multiple
/// parameters are simultaneously eligible.
///
/// Any nodes left after Kahn's algorithm are reported as blocked by a cycle.
/// That set can include downstream dependents of cycle members.
fn topological_order(
    dependencies: &BTreeMap<ParameterName, BTreeSet<ParameterName>>,
) -> Result<
    (Vec<ParameterName>, BTreeSet<ParameterName>),
    SpecializationError,
> {
    let mut indegree = BTreeMap::<ParameterName, usize>::new();
    let mut dependents =
        BTreeMap::<ParameterName, BTreeSet<ParameterName>>::new();

    for name in dependencies.keys() {
        indegree.insert(name.clone(), 0);
        dependents.insert(name.clone(), BTreeSet::new());
    }

    for (name, parameter_dependencies) in dependencies {
        for dependency in parameter_dependencies {
            // Dependencies are restricted to declared names by the builder.
            // This check also keeps this helper correct if reused internally.
            if !dependencies.contains_key(dependency) {
                continue;
            }

            let current = indegree
                .get(name)
                .copied()
                .ok_or(SpecializationError::CountOverflow)?;

            let next = current
                .checked_add(1)
                .ok_or(SpecializationError::CountOverflow)?;

            indegree.insert(name.clone(), next);

            let dependents_for_dependency = dependents
                .get_mut(dependency)
                .ok_or(SpecializationError::CountOverflow)?;

            dependents_for_dependency.insert(name.clone());
        }
    }

    let mut ready = BTreeSet::<ParameterName>::new();

    for (name, degree) in &indegree {
        if *degree == 0 {
            ready.insert(name.clone());
        }
    }

    let mut order = Vec::with_capacity(dependencies.len());

    while let Some(next) = ready.pop_first() {
        order.push(next.clone());

        if let Some(next_dependents) = dependents.get(&next) {
            for dependent in next_dependents {
                let current = indegree
                    .get(dependent)
                    .copied()
                    .ok_or(SpecializationError::CountOverflow)?;

                let updated = current
                    .checked_sub(1)
                    .ok_or(SpecializationError::CountOverflow)?;

                indegree.insert(dependent.clone(), updated);

                if updated == 0 {
                    ready.insert(dependent.clone());
                }
            }
        }
    }

    let blocked_by_cycle = indegree
        .into_iter()
        .filter_map(|(name, degree)| (degree > 0).then_some(name))
        .collect();

    Ok((order, blocked_by_cycle))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdl::hdlir::values::parameters::{
        ParameterDeclaration,
        ParameterExpression,
        ParameterKind,
        ParameterValue,
    };
    use crate::hdl::hdlir::values::expressions::ExprGraph;

    fn expression_value(symbol: &str) -> ParameterValue {
        let mut graph = ExprGraph::new();
        let root = graph.symbol(symbol).expect("symbol node");
        let expression =
            ParameterExpression::new(graph, root).expect("valid expression");

        ParameterValue::Expression(expression)
    }

    #[test]
    fn dependency_order_places_dependencies_first() {
        let mut parameters = ParameterSet::new();

        parameters
            .declare(
                ParameterDeclaration::new("A", ParameterKind::Value)
                    .expect("A declaration"),
            )
            .expect("declare A");

        parameters
            .declare(
                ParameterDeclaration::new("B", ParameterKind::Value)
                    .expect("B declaration")
                    .with_default(expression_value("A"))
                    .expect("B default"),
            )
            .expect("declare B");

        let plan = plan_specialization(
            &parameters,
            SpecializationConfig::unlimited(),
        )
        .expect("plan");

        assert_eq!(
            plan.dependency_order()
                .iter()
                .map(ParameterName::as_str)
                .collect::<Vec<_>>(),
            vec!["A", "B"]
        );
        assert_eq!(plan.dependencies().len(), 2);
        assert_eq!(plan.stats().dependency_edges, 1);
        assert!(plan.has_complete_dependency_order());
        assert!(!plan.has_no_unresolved_parameters());
    }

    #[test]
    fn reports_dependency_cycles_without_claiming_success() {
        let mut parameters = ParameterSet::new();

        parameters
            .declare(
                ParameterDeclaration::new("A", ParameterKind::Value)
                    .expect("A declaration")
                    .with_default(expression_value("B"))
                    .expect("A default"),
            )
            .expect("declare A");

        parameters
            .declare(
                ParameterDeclaration::new("B", ParameterKind::Value)
                    .expect("B declaration")
                    .with_default(expression_value("A"))
                    .expect("B default"),
            )
            .expect("declare B");

        let plan = plan_specialization(
            &parameters,
            SpecializationConfig::unlimited(),
        )
        .expect("plan");

        assert!(plan.dependency_order().is_empty());
        assert_eq!(plan.blocked_by_cycle().len(), 2);
        assert!(!plan.has_complete_dependency_order());
    }

    #[test]
    fn enforces_parameter_budget_explicitly() {
        let mut parameters = ParameterSet::new();

        parameters
            .declare(
                ParameterDeclaration::new("A", ParameterKind::Value)
                    .expect("A declaration"),
            )
            .expect("declare A");

        let error = plan_specialization(
            &parameters,
            SpecializationConfig::unlimited().with_max_parameters(0),
        )
        .expect_err("zero budget must reject one parameter");

        assert_eq!(
            error,
            SpecializationError::ParameterBudgetExceeded {
                limit: 0,
                requested: 1,
            }
        );
    }

    #[test]
    fn enforces_dependency_budget_explicitly() {
        let mut parameters = ParameterSet::new();

        parameters
            .declare(
                ParameterDeclaration::new("A", ParameterKind::Value)
                    .expect("A declaration"),
            )
            .expect("declare A");

        parameters
            .declare(
                ParameterDeclaration::new("B", ParameterKind::Value)
                    .expect("B declaration")
                    .with_default(expression_value("A"))
                    .expect("B default"),
            )
            .expect("declare B");

        let error = plan_specialization(
            &parameters,
            SpecializationConfig::unlimited()
                .with_max_dependency_edges(0),
        )
        .expect_err("zero edge budget must reject one edge");

        assert_eq!(
            error,
            SpecializationError::DependencyBudgetExceeded {
                limit: 0,
                requested: 1,
            }
        );
    }

    #[test]
    fn independent_parameters_have_deterministic_order() {
        let mut parameters = ParameterSet::new();

        for name in ["Z", "B", "M"] {
            parameters
                .declare(
                    ParameterDeclaration::new(name, ParameterKind::Value)
                        .expect("declaration"),
                )
                .expect("declare");
        }

        let first = plan_specialization(
            &parameters,
            SpecializationConfig::unlimited(),
        )
        .expect("first plan");

        let second = plan_specialization(
            &parameters,
            SpecializationConfig::unlimited(),
        )
        .expect("second plan");

        assert_eq!(first, second);
        assert_eq!(
            first
                .dependency_order()
                .iter()
                .map(ParameterName::as_str)
                .collect::<Vec<_>>(),
            vec!["B", "M", "Z"]
        );
    }
}
