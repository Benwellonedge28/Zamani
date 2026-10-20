
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Dead-Code Analysis and Removal Planning
//!
//! Path: `src/hdl/hdlir/transform/dead_code.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Conservative analysis and removal-planning API.
//!
//! ## Purpose
//!
//! Identify operations that are provably unnecessary according to verified
//! operation facts supplied by an HDLIR graph adapter. This module produces
//! a plan; it does not mutate the authoritative IR graph.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/combinational.g4`
//! - `grammar/hdl/sequential.g4`
//! - `grammar/hdl/processes.g4`
//! - `grammar/hdl/memories.g4`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/schema/operation.rs`
//! - `src/hdl/hdlir/analysis/effects.rs`
//! - `src/hdl/hdlir/passes/contracts.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar and frontend own source semantics. ZUIR owns the universal
//! computational representation. HDLIR owns hardware-domain IR and its
//! transformations. Physical realization remains downstream.
//!
//! ## Safety contract
//!
//! An operation is removable only when it is explicitly classified as pure,
//! has no observable or required-preservation obligation, and is not needed
//! by any live operation. Unknown effects are conservatively retained.
//!
//! The adapter MUST derive effect knowledge from authoritative, compatible
//! operation definitions and verified graph information. An absent effect
//! declaration MUST NOT be interpreted as proof of purity.
//!
//! The adapter MUST include dependencies arising from control flow, clocks,
//! resets, memories, connectivity, regions, and dialect-specific semantics
//! that are not represented by ordinary value operands.
//!
//! This module does not prove behavioral equivalence. It computes a
//! conservative liveness plan from the supplied facts.
//!
//! ## Scalability
//!
//! No hardware-specific maximum is imposed. Traversal is iterative, counts
//! use checked arithmetic, and optional budgets are caller-configured
//! operational limits. Exhaustion returns an error, never a partial plan.
//!
//! ## Integration limitation
//!
//! The current repository does not expose a verified whole-design mutation
//! API that this module can safely call. The plan must be consumed by a
//! future adapter that revalidates the graph and applies removals
//! transactionally. This module intentionally does not invent that API.

use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::hdlir::errors::{HdlirBudgetKind, HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{HdlirOperationId, HdlirValueId};
use crate::hdl::hdlir::schema::operation::OperationEffect;

/// Authoritative effect knowledge for one operation.
///
/// `Pure` is a strong assertion: the adapter must have established that the
/// operation has no observable effect, state transition, externally relevant
/// interaction, or extension-defined effect. It is not the default for an
/// operation whose schema or semantics are missing.
///
/// `Effectful` operations are retained even if their results are unused.
/// `Unknown` operations are retained because their behavior cannot safely be
/// assumed irrelevant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperationEffectKnowledge {
    /// Complete, authoritative evidence establishes that the operation is
    /// pure and may be removed if it is otherwise dead.
    Pure,

    /// The operation has one or more declared effects.
    Effectful(BTreeSet<OperationEffect>),

    /// The operation's complete effects could not be established.
    Unknown,
}

impl OperationEffectKnowledge {
    /// Construct effect knowledge from an explicit set of declared effects.
    ///
    /// An empty set is rejected. Use [`Self::pure`] only when an authoritative
    /// contract establishes purity.
    ///
    /// # Errors
    ///
    /// Returns an error when `effects` is empty.
    pub fn effectful(
        effects: impl IntoIterator<Item = OperationEffect>,
    ) -> HdlirResult<Self> {
        let effects: BTreeSet<_> = effects.into_iter().collect();

        if effects.is_empty() {
            return Err(HdlirError::invalid_input(
                "dead_code.effects",
                "an effectful operation must declare at least one effect",
            ));
        }

        Ok(Self::Effectful(effects))
    }

    /// Construct an explicit purity classification.
    ///
    /// The caller is responsible for establishing purity from authoritative
    /// operation semantics. This constructor cannot prove that assertion.
    #[must_use]
    pub const fn pure() -> Self {
        Self::Pure
    }

    /// Construct an unknown-effects classification.
    #[must_use]
    pub const fn unknown() -> Self {
        Self::Unknown
    }

    /// Return whether the operation is eligible for removal in principle.
    #[must_use]
    pub const fn is_proven_pure(&self) -> bool {
        matches!(self, Self::Pure)
    }
}

/// Verified liveness facts for one operation.
///
/// The operation ID and result IDs must be unique within the supplied
/// analysis input. Operand values produced by another listed operation
/// create data-dependency edges. Operand values without a producer in this
/// input are treated as external values, such as block arguments, ports, or
/// values defined outside the analyzed subgraph.
///
/// `dependencies` must list additional operation dependencies not fully
/// represented by value operands, including required control, clock/reset,
/// memory, connectivity, region, and dialect-specific dependencies.
///
/// An operation with `observable` or `preservation_required` set is a root
/// that must be retained regardless of its result uses.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationFacts {
    id: HdlirOperationId,
    results: Vec<HdlirValueId>,
    operands: Vec<HdlirValueId>,
    dependencies: Vec<HdlirOperationId>,
    effects: OperationEffectKnowledge,
    observable: bool,
    preservation_required: bool,
}

impl OperationFacts {
    /// Construct operation facts from a verified graph adapter.
    ///
    /// This constructor validates only the local classification and input
    /// shape. Whole-graph consistency is checked by
    /// [`plan_dead_code`].
    #[must_use]
    pub fn new(
        id: HdlirOperationId,
        results: Vec<HdlirValueId>,
        operands: Vec<HdlirValueId>,
        dependencies: Vec<HdlirOperationId>,
        effects: OperationEffectKnowledge,
    ) -> Self {
        Self {
            id,
            results,
            operands,
            dependencies,
            effects,
            observable: false,
            preservation_required: false,
        }
    }

    /// Mark this operation as externally observable.
    ///
    /// Examples include operations contributing to externally visible
    /// outputs or externally visible protocol behavior. The adapter must
    /// derive this property from the actual IR contract, not from an
    /// operation's name.
    #[must_use]
    pub const fn with_observable(mut self, observable: bool) -> Self {
        self.observable = observable;
        self
    }

    /// Mark this operation as subject to a preservation obligation.
    ///
    /// Use this for operations whose retention is required by an explicit
    /// contract or policy, even when ordinary dataflow does not make them
    /// appear live.
    #[must_use]
    pub const fn with_preservation_required(mut self, required: bool) -> Self {
        self.preservation_required = required;
        self
    }

    /// Return the operation's typed identity.
    #[must_use]
    pub const fn id(&self) -> HdlirOperationId {
        self.id
    }

    /// Return values defined by this operation.
    #[must_use]
    pub fn results(&self) -> &[HdlirValueId] {
        &self.results
    }

    /// Return values consumed by this operation.
    #[must_use]
    pub fn operands(&self) -> &[HdlirValueId] {
        &self.operands
    }

    /// Return additional operation dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[HdlirOperationId] {
        &self.dependencies
    }

    /// Return the authoritative effect classification supplied by the
    /// adapter.
    #[must_use]
    pub fn effects(&self) -> &OperationEffectKnowledge {
        &self.effects
    }

    /// Return whether the operation is externally observable.
    #[must_use]
    pub const fn is_observable(&self) -> bool {
        self.observable
    }

    /// Return whether a preservation obligation requires this operation.
    #[must_use]
    pub const fn preservation_required(&self) -> bool {
        self.preservation_required
    }
}

/// Operational budgets for dead-code planning.
///
/// `None` means that this configuration imposes no explicit budget for that
/// unit of work. It does not guarantee infinite memory or execution time.
///
/// The operation budget counts supplied operation records. The edge budget
/// counts operand references and explicit dependency references supplied by
/// the adapter. Neither value is a language-level or hardware-capacity limit.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DeadCodeConfig {
    max_operations: Option<u64>,
    max_edges: Option<u64>,
}

impl DeadCodeConfig {
    /// Create a configuration without explicit operation or edge budgets.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_operations: None,
            max_edges: None,
        }
    }

    /// Set the maximum number of operation records to process.
    ///
    /// A zero budget permits an empty input only.
    #[must_use]
    pub const fn with_max_operations(mut self, limit: u64) -> Self {
        self.max_operations = Some(limit);
        self
    }

    /// Set the maximum number of operand and explicit-dependency references
    /// to process.
    ///
    /// A zero budget permits inputs without such references only.
    #[must_use]
    pub const fn with_max_edges(mut self, limit: u64) -> Self {
        self.max_edges = Some(limit);
        self
    }

    /// Return the operation-record budget.
    #[must_use]
    pub const fn max_operations(self) -> Option<u64> {
        self.max_operations
    }

    /// Return the edge-reference budget.
    #[must_use]
    pub const fn max_edges(self) -> Option<u64> {
        self.max_edges
    }
}

/// Counts describing a completed dead-code planning invocation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DeadCodeStats {
    /// Number of operation records analyzed.
    pub operations_analyzed: u64,

    /// Number of operand and explicit-dependency references analyzed.
    pub edge_references_analyzed: u64,

    /// Number of operations retained as liveness roots.
    pub roots_retained: u64,

    /// Number of operations retained because their effects were unknown.
    pub unknown_effects_retained: u64,

    /// Number of operations eligible for removal under the supplied facts.
    ///
    /// This is a plan count, not a count of operations already deleted.
    pub removable_operations: u64,
}

/// A conservative dead-code removal plan.
///
/// The plan is immutable output from analysis. It does not modify the
/// authoritative IR graph and must not be interpreted as proof of
/// behavioral equivalence.
///
/// `removable_operations` contains only operations explicitly classified as
/// pure that are not reachable backward from any required root. The caller
/// must revalidate the graph and apply the plan transactionally.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeadCodePlan {
    live_operations: BTreeSet<HdlirOperationId>,
    removable_operations: BTreeSet<HdlirOperationId>,
    unknown_effects_retained: BTreeSet<HdlirOperationId>,
    preservation_roots: BTreeSet<HdlirOperationId>,
    stats: DeadCodeStats,
}

impl DeadCodePlan {
    /// Return operations that must remain because they are roots or are
    /// dependencies of live operations.
    #[must_use]
    pub fn live_operations(&self) -> &BTreeSet<HdlirOperationId> {
        &self.live_operations
    }

    /// Return operations eligible for removal under the supplied facts.
    ///
    /// These are candidates only. The caller must not delete them unless the
    /// authoritative graph is unchanged and the relevant verification and
    /// preservation contracts still hold.
    #[must_use]
    pub fn removable_operations(&self) -> &BTreeSet<HdlirOperationId> {
        &self.removable_operations
    }

    /// Return operations retained because their effects are unknown.
    #[must_use]
    pub fn unknown_effects_retained(&self) -> &BTreeSet<HdlirOperationId> {
        &self.unknown_effects_retained
    }

    /// Return operations that were explicit preservation roots.
    #[must_use]
    pub fn preservation_roots(&self) -> &BTreeSet<HdlirOperationId> {
        &self.preservation_roots
    }

    /// Return statistics for the completed planning operation.
    #[must_use]
    pub const fn stats(&self) -> DeadCodeStats {
        self.stats
    }
}

fn checked_increment(
    current: u64,
    operation: &'static str,
) -> HdlirResult<u64> {
    current.checked_add(1).ok_or_else(|| HdlirError::ArithmeticOverflow {
        operation,
        context: Some("dead-code planning counter".to_owned()),
    })
}

fn check_budget(
    budget: Option<u64>,
    observed: u64,
    kind: HdlirBudgetKind,
    operation: &'static str,
) -> HdlirResult<()> {
    if let Some(limit) = budget {
        if observed > limit {
            return Err(HdlirError::budget_exceeded(
                kind,
                Some(u128::from(limit)),
                Some(u128::from(observed)),
                operation,
            ));
        }
    }

    Ok(())
}

/// Analyze operation facts and produce a conservative dead-code plan.
///
/// The algorithm:
///
/// 1. Validates unique operation IDs and result-value definitions.
/// 2. Builds data-dependency edges from operand values to their producers.
/// 3. Adds the explicit dependencies supplied by the adapter.
/// 4. Treats observable, effectful, unknown-effect, and preservation-required
///    operations as liveness roots.
/// 5. Walks backward iteratively from roots to find all required dependencies.
/// 6. Marks only proven-pure, non-live operations as removal candidates.
///
/// Unknown external operand values are permitted because they may be
/// block arguments, module inputs, ports, or values defined outside the
/// analyzed subgraph. An explicit operation dependency, however, must refer
/// to an operation in this input.
///
/// The caller must provide a coherent analysis scope. In particular, it must
/// supply every relevant operation in that scope and all non-value
/// dependencies needed to preserve clock/reset, memory, connectivity,
/// process, control-flow, region, and dialect-specific semantics.
///
/// # Errors
///
/// Returns an error for duplicate operation IDs, duplicate result-value
/// definitions, missing explicit dependencies, counter overflow, or budget
/// exhaustion. No plan is returned on failure.
///
/// # Important
///
/// This function does not mutate the IR, validate source semantics, prove
/// equivalence, or certify target feasibility. A successful result means
/// only that the plan was completed using the facts supplied by the adapter.
pub fn plan_dead_code<I>(
    operations: I,
    config: DeadCodeConfig,
) -> HdlirResult<DeadCodePlan>
where
    I: IntoIterator<Item = OperationFacts>,
{
    let mut facts_by_id: BTreeMap<HdlirOperationId, OperationFacts> =
        BTreeMap::new();
    let mut producer_by_value: BTreeMap<HdlirValueId, HdlirOperationId> =
        BTreeMap::new();

    let mut operations_analyzed = 0_u64;
    let mut edge_references_analyzed = 0_u64;

    for facts in operations {
        let next_count = checked_increment(
            operations_analyzed,
            "counting dead-code operation records",
        )?;
        check_budget(
            config.max_operations,
            next_count,
            HdlirBudgetKind::Transformation,
            "HDLIR dead-code operation analysis",
        )?;

        if facts_by_id.contains_key(&facts.id) {
            return Err(HdlirError::invalid_invariant(
                "dead_code.unique_operation_ids",
                format!("operation ID {} occurs more than once", facts.id),
            ));
        }

        let local_edge_count = u64::try_from(facts.operands.len())
            .ok()
            .and_then(|operands| {
                u64::try_from(facts.dependencies.len())
                    .ok()
                    .and_then(|dependencies| operands.checked_add(dependencies))
            })
            .ok_or_else(|| HdlirError::ArithmeticOverflow {
                operation: "counting dead-code dependency references",
                context: Some(format!("operation {}", facts.id)),
            })?;

        edge_references_analyzed = edge_references_analyzed
            .checked_add(local_edge_count)
            .ok_or_else(|| HdlirError::ArithmeticOverflow {
                operation: "counting dead-code dependency references",
                context: Some("total edge references".to_owned()),
            })?;

        check_budget(
            config.max_edges,
            edge_references_analyzed,
            HdlirBudgetKind::Transformation,
            "HDLIR dead-code dependency analysis",
        )?;

        if let OperationEffectKnowledge::Effectful(effects) = &facts.effects {
            if effects.is_empty() {
                return Err(HdlirError::invalid_invariant(
                    "dead_code.nonempty_effectful_classification",
                    format!(
                        "operation {} was marked effectful with no declared effects",
                        facts.id
                    ),
                ));
            }
        }

        for result in &facts.results {
            if let Some(previous) = producer_by_value.insert(*result, facts.id) {
                return Err(HdlirError::invalid_invariant(
                    "dead_code.unique_result_definitions",
                    format!(
                        "value {} is defined by both operation {} and operation {}",
                        result, previous, facts.id
                    ),
                ));
            }
        }

        operations_analyzed = next_count;
        facts_by_id.insert(facts.id, facts);
    }

    // Build a complete dependency map only after all definitions have been
    // indexed. This allows operation records to arrive in any order.
    let mut dependencies_by_operation: BTreeMap<
        HdlirOperationId,
        BTreeSet<HdlirOperationId>,
    > = BTreeMap::new();

    let mut live_operations = BTreeSet::new();
    let mut unknown_effects_retained = BTreeSet::new();
    let mut preservation_roots = BTreeSet::new();
    let mut root_count = 0_u64;
    let mut unknown_count = 0_u64;

    for facts in facts_by_id.values() {
        let mut dependencies = BTreeSet::new();

        for operand in &facts.operands {
            if let Some(producer) = producer_by_value.get(operand) {
                dependencies.insert(*producer);
            }
        }

        for dependency in &facts.dependencies {
            if !facts_by_id.contains_key(dependency) {
                return Err(HdlirError::invalid_invariant(
                    "dead_code.dependencies_exist",
                    format!(
                        "operation {} depends on missing operation {}",
                        facts.id, dependency
                    ),
                ));
            }

            dependencies.insert(*dependency);
        }

        dependencies_by_operation.insert(facts.id, dependencies);

        let has_effect = matches!(
            facts.effects,
            OperationEffectKnowledge::Effectful(_)
        );
        let has_unknown_effects =
            matches!(facts.effects, OperationEffectKnowledge::Unknown);

        let is_root = facts.observable
            || facts.preservation_required
            || has_effect
            || has_unknown_effects;

        if is_root {
            root_count = checked_increment(
                root_count,
                "counting dead-code liveness roots",
            )?;
            live_operations.insert(facts.id);
        }

        if has_unknown_effects {
            unknown_count = checked_increment(
                unknown_count,
                "counting operations with unknown effects",
            )?;
            unknown_effects_retained.insert(facts.id);
        }

        if facts.preservation_required {
            preservation_roots.insert(facts.id);
        }
    }

    // Iterative traversal avoids recursion-depth dependence on the size or
    // shape of a hardware design.
    let mut worklist: Vec<HdlirOperationId> =
        live_operations.iter().copied().collect();

    while let Some(operation) = worklist.pop() {
        if let Some(dependencies) = dependencies_by_operation.get(&operation) {
            for dependency in dependencies {
                if live_operations.insert(*dependency) {
                    worklist.push(*dependency);
                }
            }
        } else {
            // This should be impossible after building the map from the same
            // validated operation set. Treat it as an invariant failure.
            return Err(HdlirError::invalid_invariant(
                "dead_code.dependency_map_complete",
                format!(
                    "dependency map has no entry for operation {}",
                    operation
                ),
            ));
        }
    }

    let mut removable_operations = BTreeSet::new();

    for facts in facts_by_id.values() {
        if !live_operations.contains(&facts.id)
            && facts.effects.is_proven_pure()
            && !facts.observable
            && !facts.preservation_required
        {
            removable_operations.insert(facts.id);
        }
    }

    let removable_count = u64::try_from(removable_operations.len())
        .map_err(|_| HdlirError::ArithmeticOverflow {
            operation: "counting removable operations",
            context: Some("dead-code plan statistics".to_owned()),
        })?;

    Ok(DeadCodePlan {
        live_operations,
        removable_operations,
        unknown_effects_retained,
        preservation_roots,
        stats: DeadCodeStats {
            operations_analyzed,
            edge_references_analyzed,
            roots_retained: root_count,
            unknown_effects_retained: unknown_count,
            removable_operations: removable_count,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operation_id(value: u128) -> HdlirOperationId {
        HdlirOperationId::new(value).expect("test ID must be non-zero")
    }

    fn value_id(value: u128) -> HdlirValueId {
        HdlirValueId::new(value).expect("test ID must be non-zero")
    }

    fn pure(
        id: u128,
        results: &[u128],
        operands: &[u128],
    ) -> OperationFacts {
        OperationFacts::new(
            operation_id(id),
            results.iter().copied().map(value_id).collect(),
            operands.iter().copied().map(value_id).collect(),
            Vec::new(),
            OperationEffectKnowledge::pure(),
        )
    }

    fn effectful(id: u128) -> OperationFacts {
        OperationFacts::new(
            operation_id(id),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            OperationEffectKnowledge::effectful([
                OperationEffect::Observable,
            ])
            .expect("a non-empty effect set must be valid"),
        )
    }

    #[test]
    fn removes_unused_pure_operations() {
        let plan = plan_dead_code(
            [
                pure(1, &[10], &[]),
                pure(2, &[20], &[]),
            ],
            DeadCodeConfig::unlimited(),
        )
        .expect("planning must succeed");

        assert!(plan.live_operations().is_empty());
        assert_eq!(
            plan.removable_operations(),
            &BTreeSet::from([operation_id(1), operation_id(2)])
        );
    }

    #[test]
    fn retains_effectful_operations_without_results() {
        let plan = plan_dead_code(
            [effectful(1)],
            DeadCodeConfig::unlimited(),
        )
        .expect("planning must succeed");

        assert!(plan.live_operations().contains(&operation_id(1)));
        assert!(plan.removable_operations().is_empty());
    }

    #[test]
    fn retains_unknown_effects_conservatively() {
        let unknown = OperationFacts::new(
            operation_id(1),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            OperationEffectKnowledge::unknown(),
        );

        let plan = plan_dead_code(
            [unknown],
            DeadCodeConfig::unlimited(),
        )
        .expect("unknown effects must be retained");

        assert!(plan.live_operations().contains(&operation_id(1)));
        assert!(
            plan.unknown_effects_retained()
                .contains(&operation_id(1))
        );
        assert!(plan.removable_operations().is_empty());
    }

    #[test]
    fn preserves_dependencies_of_observable_operations() {
        let producer = pure(1, &[10], &[]);
        let consumer = pure(2, &[20], &[10]).with_observable(true);
        let unrelated = pure(3, &[30], &[]);

        let plan = plan_dead_code(
            [producer, consumer, unrelated],
            DeadCodeConfig::unlimited(),
        )
        .expect("planning must succeed");

        assert!(plan.live_operations().contains(&operation_id(1)));
        assert!(plan.live_operations().contains(&operation_id(2)));
        assert!(!plan.live_operations().contains(&operation_id(3)));
        assert!(
            plan.removable_operations().contains(&operation_id(3))
        );
    }

    #[test]
    fn preserves_explicit_clock_or_control_dependencies() {
        let clocked = pure(1, &[], &[]);
        let dependent = OperationFacts::new(
            operation_id(2),
            Vec::new(),
            Vec::new(),
            vec![operation_id(1)],
            OperationEffectKnowledge::pure(),
        )
        .with_preservation_required(true);

        let plan = plan_dead_code(
            [clocked, dependent],
            DeadCodeConfig::unlimited(),
        )
        .expect("planning must succeed");

        assert!(plan.live_operations().contains(&operation_id(1)));
        assert!(plan.live_operations().contains(&operation_id(2)));
        assert!(
            plan.preservation_roots().contains(&operation_id(2))
        );
    }

    #[test]
    fn permits_external_operand_values() {
        let operation = pure(1, &[10], &[999]);

        let plan = plan_dead_code(
            [operation],
            DeadCodeConfig::unlimited(),
        )
        .expect("external values may be block arguments or ports");

        assert!(plan.removable_operations().contains(&operation_id(1)));
    }

    #[test]
    fn rejects_duplicate_operation_ids() {
        let result = plan_dead_code(
            [pure(1, &[], &[]), pure(1, &[], &[])],
            DeadCodeConfig::unlimited(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_duplicate_result_definitions() {
        let result = plan_dead_code(
            [pure(1, &[10], &[]), pure(2, &[10], &[])],
            DeadCodeConfig::unlimited(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_explicit_dependencies() {
        let operation = OperationFacts::new(
            operation_id(1),
            Vec::new(),
            Vec::new(),
            vec![operation_id(99)],
            OperationEffectKnowledge::pure(),
        );

        let result = plan_dead_code(
            [operation],
            DeadCodeConfig::unlimited(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_empty_effectful_classification() {
        let result = OperationEffectKnowledge::effectful([]);

        assert!(result.is_err());
    }

    #[test]
    fn operation_budget_exhaustion_returns_no_plan() {
        let result = plan_dead_code(
            [pure(1, &[], &[]), pure(2, &[], &[])],
            DeadCodeConfig::unlimited().with_max_operations(1),
        );

        assert!(result.is_err());
    }

    #[test]
    fn edge_budget_exhaustion_returns_no_plan() {
        let producer = pure(1, &[10], &[]);
        let consumer = pure(2, &[], &[10]);

        let result = plan_dead_code(
            [producer, consumer],
            DeadCodeConfig::unlimited().with_max_edges(0),
        );

        assert!(result.is_err());
    }

    #[test]
    fn input_order_does_not_change_the_plan() {
        let a = pure(1, &[10], &[]);
        let b = pure(2, &[20], &[10]).with_observable(true);

        let first = plan_dead_code(
            [a.clone(), b.clone()],
            DeadCodeConfig::unlimited(),
        )
        .expect("first plan must succeed");

        let second = plan_dead_code(
            [b, a],
            DeadCodeConfig::unlimited(),
        )
        .expect("second plan must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn effectful_operation_is_retained_even_when_unused() {
        let read = OperationFacts::new(
            operation_id(1),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            OperationEffectKnowledge::effectful([
                OperationEffect::Read,
                OperationEffect::MemoryAccess,
            ])
            .expect("declared effects must be non-empty"),
        );

        let plan = plan_dead_code(
            [read],
            DeadCodeConfig::unlimited(),
        )
        .expect("planning must succeed");

        assert!(plan.live_operations().contains(&operation_id(1)));
        assert!(plan.removable_operations().is_empty());
    }
}
