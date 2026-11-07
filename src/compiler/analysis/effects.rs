
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Interprocedural Effect Analysis.
//!
//! Path: src/compiler/analysis/effects.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust forbidden.
//!
//! PURPOSE
//!
//! Analyze effects across explicitly resolved ZUIR function call graphs.
//! This module consumes canonical ZUIR effect summaries and identities.
//! It does not define a competing effect vocabulary or IR.
//!
//! RESPONSIBILITIES
//!
//! - Validate function effect-analysis inputs.
//! - Aggregate direct effects and callee effects.
//! - Propagate effects through recursive and mutually recursive calls.
//! - Accept explicitly supplied summaries for external functions.
//! - Preserve Unknown, Partial, and Complete effect knowledge.
//! - Check inferred known effects against declared upper bounds.
//! - Distinguish observed contract violations from proven compliance.
//! - Authorize effects against explicit policies.
//! - Support registry-backed effect authorization.
//! - Produce deterministic, immutable analysis results.
//!
//! DOES NOT OWN
//!
//! - Source grammar, parsing, AST construction, or name resolution.
//! - Semantic effect identity or the canonical effect vocabulary.
//! - ZUIR operations, functions, blocks, or regions.
//! - Call-graph discovery.
//! - Dialect or extension registry storage.
//! - Control-flow, dominance, alias, or ownership analysis.
//! - Target selection, hardware discovery, or resource allocation.
//! - Scheduling, routing, lowering, or backend execution.
//! - Runtime effect enforcement or effect-handler execution.
//!
//! CANONICAL AUTHORITIES
//!
//! - crate::zuir::ids::FunctionId
//! - crate::zuir::ir::effects
//! - crate::zuir::analysis::effects
//! - crate::zuir::ir::function
//!
//! The frontend semantic model owns source-level meaning.
//! ZUIR owns canonical effect declarations and summaries.
//! ZUIR analysis owns operation-level effect aggregation.
//! This module owns compiler-level interprocedural propagation.
//!
//! COMPLETENESS
//!
//! An empty effect set is not proof of purity.
//! A Complete summary is a claim supplied by a trusted producer;
//! this module does not independently prove that claim.
//! Incomplete summaries cannot establish declared-effect compliance.
//! Unknown external functions are never silently treated as pure.
//!
//! SCALABILITY
//!
//! No arbitrary universal limits are imposed on functions, call edges,
//! effects, domains, devices, processors, or hardware.
//! The algorithm is iterative and uses dynamically sized collections.
//! Actual execution remains bounded by available memory, address space,
//! processing capacity, and explicitly configured operational budgets.
//!
//! INTEGRATION
//!
//! Parent: src/compiler/analysis/mod.rs
//!
//! Existing declaration:
//!
//!     pub mod effects;
//!
//! Canonical operation analysis:
//!
//!     crate::zuir::analysis::effects::EffectAnalysis
//!
//! Canonical effect model:
//!
//!     crate::zuir::ir::effects
//!
//! The caller supplies resolved function identities, direct summaries,
//! call relationships, external summaries, and declared effect bounds.
//!
//! No grammar parsing or symbol resolution occurs here.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::fmt;

use crate::zuir::ids::FunctionId;

use crate::zuir::ir::effects::{
    EffectCompleteness,
    EffectDefinition,
    EffectError,
    EffectPolicy,
    EffectSet,
    EffectSummary,
};

/// Result type for compiler effect analysis.
pub type CompilerEffectResult<T> =
    Result<T, CompilerEffectError>;

/// Errors produced by compiler-level effect analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompilerEffectError {
    /// A function identity was supplied more than once.
    DuplicateFunction {
        function: FunctionId,
    },

    /// A function references a callee for which no summary exists.
    UnknownCallee {
        caller: FunctionId,
        callee: FunctionId,
    },

    /// An external summary conflicts with an internally analyzed function.
    ExternalSummaryConflictsWithFunction {
        function: FunctionId,
    },

    /// An external summary was invalid.
    InvalidExternalEffects {
        function: FunctionId,
        source: EffectError,
    },

    /// A function's direct effect summary was malformed.
    InvalidDirectEffects {
        function: FunctionId,
        source: EffectError,
    },

    /// A function's declared effect set was malformed.
    InvalidDeclaredEffects {
        function: FunctionId,
        source: EffectError,
    },

    /// An authorization policy rejected an effect.
    PolicyViolation {
        function: FunctionId,
        source: EffectError,
    },

    /// The propagation algorithm violated its monotonicity invariant.
    NonMonotonicPropagation {
        function: FunctionId,
    },

    /// A required internal analysis value was unexpectedly absent.
    InternalInvariant {
        function: FunctionId,
        description: &'static str,
    },
}

impl fmt::Display for CompilerEffectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateFunction { function } => {
                write!(
                    f,
                    "duplicate function in effect analysis: {function}"
                )
            }

            Self::UnknownCallee { caller, callee } => {
                write!(
                    f,
                    "function {caller} references unresolved callee {callee}"
                )
            }

            Self::ExternalSummaryConflictsWithFunction { function } => {
                write!(
                    f,
                    "external effect summary conflicts with internal function {function}"
                )
            }

            Self::InvalidExternalEffects { function, source } => {
                write!(
                    f,
                    "invalid external effects for function {function}: {source}"
                )
            }

            Self::InvalidDirectEffects { function, source } => {
                write!(
                    f,
                    "invalid direct effects for function {function}: {source}"
                )
            }

            Self::InvalidDeclaredEffects { function, source } => {
                write!(
                    f,
                    "invalid declared effects for function {function}: {source}"
                )
            }

            Self::PolicyViolation { function, source } => {
                write!(
                    f,
                    "effect authorization failed for function {function}: {source}"
                )
            }

            Self::NonMonotonicPropagation { function } => {
                write!(
                    f,
                    "non-monotonic effect propagation for function {function}"
                )
            }

            Self::InternalInvariant {
                function,
                description,
            } => {
                write!(
                    f,
                    "internal effect-analysis invariant failed for {function}: {description}"
                )
            }
        }
    }
}

impl Error for CompilerEffectError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidExternalEffects { source, .. }
            | Self::InvalidDirectEffects { source, .. }
            | Self::InvalidDeclaredEffects { source, .. }
            | Self::PolicyViolation { source, .. } => Some(source),

            _ => None,
        }
    }
}

// ============================================================================
// INPUT
// ============================================================================

/// Effect-analysis input for one resolved ZUIR function.
///
/// This is an analysis adapter, not another function representation.
///
/// The caller supplies:
///
/// - The canonical function identity.
/// - Direct effects from trusted operation/region analysis.
/// - Resolved callee identities.
/// - An optional declared upper bound.
///
/// The caller is responsible for ensuring that direct effects cover the
/// intended function scope. This module does not discover omitted regions,
/// operations, indirect calls, callbacks, or dynamically loaded functions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionEffectInput {
    function: FunctionId,
    direct_effects: EffectSummary,
    callees: BTreeSet<FunctionId>,
    declared_effects: Option<EffectSet>,
}

impl FunctionEffectInput {
    /// Construct a function effect-analysis input.
    ///
    /// `declared_effects` is an optional upper bound.
    ///
    /// `direct_effects` must preserve the completeness supplied by its
    /// trusted producer. Traversal completion alone does not establish
    /// semantic completeness.
    pub fn new(
        function: FunctionId,
        direct_effects: EffectSummary,
        callees: impl IntoIterator<Item = FunctionId>,
        declared_effects: Option<EffectSet>,
    ) -> CompilerEffectResult<Self> {
        direct_effects.validate().map_err(|source| {
            CompilerEffectError::InvalidDirectEffects {
                function,
                source,
            }
        })?;

        if let Some(declared) = &declared_effects {
            declared.validate().map_err(|source| {
                CompilerEffectError::InvalidDeclaredEffects {
                    function,
                    source,
                }
            })?;
        }

        Ok(Self {
            function,
            direct_effects,
            callees: callees.into_iter().collect(),
            declared_effects,
        })
    }

    /// Return the canonical function identity.
    #[must_use]
    pub const fn function(&self) -> FunctionId {
        self.function
    }

    /// Return the direct effect summary.
    #[must_use]
    pub const fn direct_effects(&self) -> &EffectSummary {
        &self.direct_effects
    }

    /// Return resolved callees in deterministic order.
    #[must_use]
    pub fn callees(&self) -> &BTreeSet<FunctionId> {
        &self.callees
    }

    /// Return the optional declared upper bound.
    #[must_use]
    pub const fn declared_effects(&self) -> Option<&EffectSet> {
        self.declared_effects.as_ref()
    }
}

// ============================================================================
// EXTERNAL FUNCTION SUMMARIES
// ============================================================================

/// Explicit effect information for a function outside the current
/// interprocedural analysis scope.
///
/// Examples include imported library functions, foreign functions,
/// separately compiled modules, and externally provided operations.
///
/// The summary must come from an explicit trusted source. An absent external
/// summary is not equivalent to an empty or pure summary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalFunctionEffects {
    function: FunctionId,
    summary: EffectSummary,
}

impl ExternalFunctionEffects {
    /// Construct an external function summary.
    pub fn new(
        function: FunctionId,
        summary: EffectSummary,
    ) -> CompilerEffectResult<Self> {
        summary.validate().map_err(|source| {
            CompilerEffectError::InvalidExternalEffects {
                function,
                source,
            }
        })?;

        Ok(Self { function, summary })
    }

    /// Return the canonical function identity.
    #[must_use]
    pub const fn function(&self) -> FunctionId {
        self.function
    }

    /// Return the supplied effect summary.
    #[must_use]
    pub const fn summary(&self) -> &EffectSummary {
        &self.summary
    }
}

// ============================================================================
// FUNCTION RESULTS
// ============================================================================

/// Inferred effect information for one function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionEffectResult {
    function: FunctionId,
    summary: EffectSummary,
    declared_effects: Option<EffectSet>,
}

impl FunctionEffectResult {
    /// Return the function identity.
    #[must_use]
    pub const fn function(&self) -> FunctionId {
        self.function
    }

    /// Return the inferred effect summary.
    #[must_use]
    pub const fn summary(&self) -> &EffectSummary {
        &self.summary
    }

    /// Return the optional declared upper bound.
    #[must_use]
    pub const fn declared_effects(&self) -> Option<&EffectSet> {
        self.declared_effects.as_ref()
    }

    /// Return whether the summary is marked complete.
    ///
    /// This preserves the completeness claim supplied by the trusted
    /// analysis producers; it does not independently prove it.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.summary.is_complete()
    }

    /// Return whether the summary establishes purity.
    ///
    /// Incomplete summaries return an error.
    pub fn is_proven_pure(&self) -> Result<bool, EffectError> {
        self.summary.is_proven_pure()
    }

    /// Return whether the declared upper bound exists.
    #[must_use]
    pub const fn has_declared_effects(&self) -> bool {
        self.declared_effects.is_some()
    }
}

// ============================================================================
// CONTRACT RESULTS
// ============================================================================

/// A known inferred effect missing from a declared upper bound.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectContractViolation {
    /// Function whose declaration is insufficient.
    pub function: FunctionId,

    /// Canonical qualified effect name in its display representation.
    pub effect: String,
}

impl fmt::Display for EffectContractViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "function {} does not declare inferred effect `{}`",
            self.function,
            self.effect
        )
    }
}

/// State of declared-effect contract verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectContractStatus {
    /// At least one known inferred effect violates its declared upper bound.
    Violated,

    /// No known violation exists, but at least one applicable summary
    /// is incomplete, so compliance cannot be established.
    Inconclusive,

    /// Every function with a declared upper bound has a complete summary
    /// and no known effect exceeds its declaration.
    Verified,
}

// ============================================================================
// ANALYSIS RESULT
// ============================================================================

/// Immutable result of compiler-level interprocedural effect analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectAnalysis {
    functions: BTreeMap<FunctionId, FunctionEffectResult>,
    violations: Vec<EffectContractViolation>,
}

impl EffectAnalysis {
    /// Analyze an explicitly supplied resolved function graph.
    ///
    /// All callees must be present in the supplied function collection.
    ///
    /// Use `analyze_with_external_summaries` when the graph contains
    /// separately compiled or external functions.
    pub fn analyze<I>(inputs: I) -> CompilerEffectResult<Self>
    where
        I: IntoIterator<Item = FunctionEffectInput>,
    {
        Self::analyze_with_external_summaries(
            inputs,
            std::iter::empty::<ExternalFunctionEffects>(),
        )
    }

    /// Analyze a resolved function graph with explicit external summaries.
    ///
    /// Internal functions are propagated to a fixed point. External
    /// summaries are treated as fixed inputs and are never assumed pure.
    ///
    /// Recursive and mutually recursive calls are supported.
    ///
    /// The algorithm is iterative and deterministic. It does not use
    /// recursion proportional to call-graph depth.
    pub fn analyze_with_external_summaries<I, E>(
        inputs: I,
        external_summaries: E,
    ) -> CompilerEffectResult<Self>
    where
        I: IntoIterator<Item = FunctionEffectInput>,
        E: IntoIterator<Item = ExternalFunctionEffects>,
    {
        let mut inputs_by_id =
            BTreeMap::<FunctionId, FunctionEffectInput>::new();

        // Register and validate internal functions.
        for input in inputs {
            let id = input.function;

            if inputs_by_id.contains_key(&id) {
                return Err(CompilerEffectError::DuplicateFunction {
                    function: id,
                });
            }

            input.direct_effects.validate().map_err(|source| {
                CompilerEffectError::InvalidDirectEffects {
                    function: id,
                    source,
                }
            })?;

            if let Some(declared) = &input.declared_effects {
                declared.validate().map_err(|source| {
                    CompilerEffectError::InvalidDeclaredEffects {
                        function: id,
                        source,
                    }
                })?;
            }

            inputs_by_id.insert(id, input);
        }

        // Register external summaries.
        let mut external_by_id =
            BTreeMap::<FunctionId, EffectSummary>::new();

        for external in external_summaries {
            let id = external.function;

            if inputs_by_id.contains_key(&id) {
                return Err(
                    CompilerEffectError::ExternalSummaryConflictsWithFunction {
                        function: id,
                    },
                );
            }

            external.summary.validate().map_err(|source| {
                CompilerEffectError::InvalidExternalEffects {
                    function: id,
                    source,
                }
            })?;

            if external_by_id.insert(id, external.summary).is_some() {
                return Err(
                    CompilerEffectError::InvalidExternalEffects {
                        function: id,
                        source: EffectError::IncompleteEffectInformation {
                            context: format!(
                                "duplicate external summary for function {id}"
                            ),
                        },
                    },
                );
            }
        }

        // Construct the reverse call graph:
        //
        // callee -> callers
        //
        // Only internal callers need to be rescheduled when a summary
        // changes. External summaries are immutable inputs.
        let mut callers =
            BTreeMap::<FunctionId, BTreeSet<FunctionId>>::new();

        for (&caller, input) in &inputs_by_id {
            for &callee in &input.callees {
                if !inputs_by_id.contains_key(&callee)
                    && !external_by_id.contains_key(&callee)
                {
                    return Err(CompilerEffectError::UnknownCallee {
                        caller,
                        callee,
                    });
                }

                callers.entry(callee).or_default().insert(caller);
            }
        }

        // Initialize internal summaries with direct effects.
        let mut summaries =
            BTreeMap::<FunctionId, EffectSummary>::new();

        for (&id, input) in &inputs_by_id {
            summaries.insert(id, input.direct_effects.clone());
        }

        // Deterministic worklist. Every internal function is initially
        // queued so direct and external effects are incorporated.
        let mut pending = VecDeque::<FunctionId>::new();
        let mut queued = BTreeSet::<FunctionId>::new();

        for &id in inputs_by_id.keys() {
            pending.push_back(id);
            queued.insert(id);
        }

        // Iterative fixed-point propagation.
        while let Some(function_id) = pending.pop_front() {
            queued.remove(&function_id);

            let input = inputs_by_id.get(&function_id).ok_or(
                CompilerEffectError::InternalInvariant {
                    function: function_id,
                    description: "queued function is not registered",
                },
            )?;

            let mut inferred = input.direct_effects.effects().clone();
            let mut completeness = input.direct_effects.completeness();

            for callee in &input.callees {
                let callee_summary = if let Some(internal) =
                    summaries.get(callee)
                {
                    internal
                } else if let Some(external) = external_by_id.get(callee) {
                    external
                } else {
                    return Err(CompilerEffectError::InternalInvariant {
                        function: function_id,
                        description: "validated callee summary disappeared",
                    });
                };

                inferred.extend(callee_summary.effects());

                completeness = combine_completeness(
                    completeness,
                    callee_summary.completeness(),
                );
            }

            let next = EffectSummary::new(inferred, completeness);

            let previous = summaries.get(&function_id).ok_or(
                CompilerEffectError::InternalInvariant {
                    function: function_id,
                    description: "function summary is missing",
                },
            )?;

            // The effect set may only grow.
            // Knowledge may only become more conservative.
            if !previous.effects().is_subset_of(next.effects())
                || completeness_rank(next.completeness())
                    < completeness_rank(previous.completeness())
            {
                return Err(
                    CompilerEffectError::NonMonotonicPropagation {
                        function: function_id,
                    },
                );
            }

            if *previous != next {
                summaries.insert(function_id, next);

                if let Some(dependents) = callers.get(&function_id) {
                    for &caller in dependents {
                        if queued.insert(caller) {
                            pending.push_back(caller);
                        }
                    }
                }
            }
        }

        // Construct immutable results and check declared upper bounds.
        let mut functions =
            BTreeMap::<FunctionId, FunctionEffectResult>::new();

        for (id, input) in inputs_by_id {
            let summary = summaries.remove(&id).ok_or(
                CompilerEffectError::InternalInvariant {
                    function: id,
                    description: "final function summary is missing",
                },
            )?;

            functions.insert(
                id,
                FunctionEffectResult {
                    function: id,
                    summary,
                    declared_effects: input.declared_effects,
                },
            );
        }

        let mut violations = Vec::new();

        for (&id, result) in &functions {
            let Some(declared) = &result.declared_effects else {
                continue;
            };

            for effect in result.summary.effects().iter() {
                if !declared.contains(effect) {
                    violations.push(EffectContractViolation {
                        function: id,
                        effect: effect.to_string(),
                    });
                }
            }
        }

        Ok(Self {
            functions,
            violations,
        })
    }

    /// Return the number of internally analyzed functions.
    #[must_use]
    pub fn function_count(&self) -> usize {
        self.functions.len()
    }

    /// Return whether no internal functions were analyzed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.functions.is_empty()
    }

    /// Look up a function result.
    #[must_use]
    pub fn function(
        &self,
        id: FunctionId,
    ) -> Option<&FunctionEffectResult> {
        self.functions.get(&id)
    }

    /// Iterate through results in deterministic function-ID order.
    pub fn functions(
        &self,
    ) -> impl Iterator<Item = (FunctionId, &FunctionEffectResult)> {
        self.functions
            .iter()
            .map(|(id, result)| (*id, result))
    }

    /// Return known declared-effect violations.
    #[must_use]
    pub fn violations(&self) -> &[EffectContractViolation] {
        &self.violations
    }

    /// Return whether no known declared-effect violation exists.
    ///
    /// This does not mean all contracts have been proven. Use
    /// `contract_status()` or `contracts_fully_verified()` for that.
    #[must_use]
    pub fn contracts_satisfied(&self) -> bool {
        self.violations.is_empty()
    }

    /// Determine the verification state of declared effect contracts.
    #[must_use]
    pub fn contract_status(&self) -> EffectContractStatus {
        if !self.violations.is_empty() {
            return EffectContractStatus::Violated;
        }

        let has_incomplete_declared_summary = self.functions.values().any(
            |result| {
                result.declared_effects.is_some()
                    && !result.summary.is_complete()
            },
        );

        if has_incomplete_declared_summary {
            EffectContractStatus::Inconclusive
        } else {
            EffectContractStatus::Verified
        }
    }

    /// Return whether all applicable declared-effect contracts are verified.
    #[must_use]
    pub fn contracts_fully_verified(&self) -> bool {
        self.contract_status() == EffectContractStatus::Verified
    }

    /// Return the aggregate known effects of internally analyzed functions.
    ///
    /// The aggregate is a union, not an execution-order model.
    ///
    /// An empty analysis returns Unknown completeness. It does not prove
    /// that an absent program or unexamined scope is pure.
    pub fn aggregate(&self) -> EffectSummary {
        let mut effects = EffectSet::new();

        if self.functions.is_empty() {
            return EffectSummary::unknown();
        }

        let mut completeness = EffectCompleteness::Complete;

        for result in self.functions.values() {
            effects.extend(result.summary.effects());

            completeness = combine_completeness(
                completeness,
                result.summary.completeness(),
            );
        }

        EffectSummary::new(effects, completeness)
    }

    /// Validate known effects against an authorization policy.
    ///
    /// Authorization is separate from effect inference and completeness.
    /// A policy check does not prove that the inferred effect information
    /// is complete.
    pub fn authorize(
        &self,
        policy: &EffectPolicy,
    ) -> CompilerEffectResult<()> {
        for (&id, result) in &self.functions {
            policy
                .validate(result.summary.effects())
                .map_err(|source| {
                    CompilerEffectError::PolicyViolation {
                        function: id,
                        source,
                    }
                })?;
        }

        Ok(())
    }

    /// Validate known effects against a policy and an authoritative
    /// effect-definition lookup.
    ///
    /// The lookup must query the canonical dialect/effect registry.
    /// This method does not own or cache that registry.
    pub fn authorize_with_registry<F>(
        &self,
        policy: &EffectPolicy,
        mut lookup: F,
    ) -> CompilerEffectResult<()>
    where
        F: FnMut(
            &crate::zuir::schema::types::QualifiedName,
        ) -> Option<EffectDefinition>,
    {
        for (&id, result) in &self.functions {
            policy
                .validate_with_registry(
                    result.summary.effects(),
                    |name| lookup(name),
                )
                .map_err(|source| {
                    CompilerEffectError::PolicyViolation {
                        function: id,
                        source,
                    }
                })?;
        }

        Ok(())
    }
}

// ============================================================================
// COMPLETENESS LATTICE
// ============================================================================

/// Combine effect knowledge conservatively.
///
/// Unknown dominates Partial.
/// Partial dominates Complete.
///
/// A combined summary is Complete only when every contributing summary
/// is Complete.
fn combine_completeness(
    left: EffectCompleteness,
    right: EffectCompleteness,
) -> EffectCompleteness {
    match (left, right) {
        (EffectCompleteness::Unknown, _)
        | (_, EffectCompleteness::Unknown) => {
            EffectCompleteness::Unknown
        }

        (EffectCompleteness::Partial, _)
        | (_, EffectCompleteness::Partial) => {
            EffectCompleteness::Partial
        }

        (
            EffectCompleteness::Complete,
            EffectCompleteness::Complete,
        ) => EffectCompleteness::Complete,
    }
}

/// Internal order for monotonicity checks.
///
/// Lower values represent stronger knowledge.
/// Propagation may only move toward equal or weaker knowledge.
const fn completeness_rank(
    completeness: EffectCompleteness,
) -> u8 {
    match completeness {
        EffectCompleteness::Complete => 0,
        EffectCompleteness::Partial => 1,
        EffectCompleteness::Unknown => 2,
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ir::effects::EffectCompleteness;
    use crate::zuir::schema::types::QualifiedName;

    fn function_id(value: u128) -> FunctionId {
        FunctionId::new(value).expect("nonzero test function ID")
    }

    fn effect(name: &str) -> QualifiedName {
        QualifiedName::parse(name).expect("valid test effect name")
    }

    fn effects(names: &[&str]) -> EffectSet {
        EffectSet::from_names(
            names.iter().map(|name| effect(name)),
        )
        .expect("valid test effect set")
    }

    fn input(
        id: u128,
        direct: &[&str],
        callees: &[u128],
        completeness: EffectCompleteness,
        declared: Option<&[&str]>,
    ) -> FunctionEffectInput {
        FunctionEffectInput::new(
            function_id(id),
            EffectSummary::new(
                effects(direct),
                completeness,
            ),
            callees.iter().map(|id| function_id(*id)),
            declared.map(effects),
        )
        .expect("valid function effect input")
    }

    #[test]
    fn direct_effects_are_preserved() {
        let analysis = EffectAnalysis::analyze([
            input(
                1,
                &["io::read"],
                &[],
                EffectCompleteness::Complete,
                Some(&["io::read"]),
            ),
        ])
        .expect("analysis succeeds");

        let result = analysis.function(function_id(1)).unwrap();

        assert!(result.summary().effects().contains(&effect("io::read")));
        assert!(result.is_complete());
        assert_eq!(
            analysis.contract_status(),
            EffectContractStatus::Verified
        );
    }

    #[test]
    fn callee_effects_propagate_to_callers() {
        let analysis = EffectAnalysis::analyze([
            input(
                1,
                &[],
                &[2],
                EffectCompleteness::Complete,
                Some(&["io::write"]),
            ),
            input(
                2,
                &["io::write"],
                &[],
                EffectCompleteness::Complete,
                Some(&["io::write"]),
            ),
        ])
        .expect("analysis succeeds");

        let caller = analysis.function(function_id(1)).unwrap();

        assert!(caller.summary().effects().contains(&effect("io::write")));
    }

    #[test]
    fn transitive_effects_propagate() {
        let analysis = EffectAnalysis::analyze([
            input(1, &[], &[2], EffectCompleteness::Complete, None),
            input(2, &[], &[3], EffectCompleteness::Complete, None),
            input(
                3,
                &["network::send"],
                &[],
                EffectCompleteness::Complete,
                None,
            ),
        ])
        .expect("analysis succeeds");

        assert!(
            analysis
                .function(function_id(1))
                .unwrap()
                .summary()
                .effects()
                .contains(&effect("network::send"))
        );
    }

    #[test]
    fn recursive_calls_converge() {
        let analysis = EffectAnalysis::analyze([
            input(
                1,
                &["memory::write"],
                &[2],
                EffectCompleteness::Complete,
                None,
            ),
            input(
                2,
                &["io::read"],
                &[1],
                EffectCompleteness::Complete,
                None,
            ),
        ])
        .expect("recursive analysis converges");

        for id in [1, 2] {
            let summary = analysis.function(function_id(id)).unwrap().summary();

            assert!(summary.effects().contains(&effect("memory::write")));
            assert!(summary.effects().contains(&effect("io::read")));
            assert!(summary.is_complete());
        }
    }

    #[test]
    fn incomplete_information_propagates_to_callers() {
        let analysis = EffectAnalysis::analyze([
            input(1, &[], &[2], EffectCompleteness::Complete, None),
            input(2, &[], &[], EffectCompleteness::Partial, None),
        ])
        .expect("analysis succeeds");

        assert_eq!(
            analysis
                .function(function_id(1))
                .unwrap()
                .summary()
                .completeness(),
            EffectCompleteness::Partial
        );
    }

    #[test]
    fn unknown_information_dominates_partial_information() {
        let analysis = EffectAnalysis::analyze([
            input(1, &[], &[2], EffectCompleteness::Partial, None),
            input(2, &[], &[], EffectCompleteness::Unknown, None),
        ])
        .expect("analysis succeeds");

        assert_eq!(
            analysis
                .function(function_id(1))
                .unwrap()
                .summary()
                .completeness(),
            EffectCompleteness::Unknown
        );
    }

    #[test]
    fn empty_analysis_does_not_prove_purity() {
        let analysis = EffectAnalysis::analyze(
            std::iter::empty::<FunctionEffectInput>(),
        )
        .expect("empty analysis is valid");

        assert!(analysis.is_empty());
        assert!(!analysis.aggregate().is_complete());
        assert!(analysis.aggregate().is_proven_pure().is_err());
    }

    #[test]
    fn unknown_callee_is_rejected() {
        let result = EffectAnalysis::analyze([
            input(1, &[], &[99], EffectCompleteness::Complete, None),
        ]);

        assert!(matches!(
            result,
            Err(CompilerEffectError::UnknownCallee { .. })
        ));
    }

    #[test]
    fn explicit_external_summary_is_propagated() {
        let external = ExternalFunctionEffects::new(
            function_id(99),
            EffectSummary::new(
                effects(&["foreign::call", "io::write"]),
                EffectCompleteness::Partial,
            ),
        )
        .expect("valid external summary");

        let analysis = EffectAnalysis::analyze_with_external_summaries(
            [
                input(
                    1,
                    &[],
                    &[99],
                    EffectCompleteness::Complete,
                    None,
                ),
            ],
            [external],
        )
        .expect("analysis succeeds");

        let summary = analysis.function(function_id(1)).unwrap().summary();

        assert!(summary.effects().contains(&effect("foreign::call")));
        assert!(summary.effects().contains(&effect("io::write")));
        assert_eq!(summary.completeness(), EffectCompleteness::Partial);
    }

    #[test]
    fn duplicate_function_is_rejected() {
        let result = EffectAnalysis::analyze([
            input(1, &[], &[], EffectCompleteness::Complete, None),
            input(1, &[], &[], EffectCompleteness::Complete, None),
        ]);

        assert!(matches!(
            result,
            Err(CompilerEffectError::DuplicateFunction { .. })
        ));
    }

    #[test]
    fn inferred_effect_outside_declared_upper_bound_is_reported() {
        let analysis = EffectAnalysis::analyze([
            input(
                1,
                &["io::write"],
                &[],
                EffectCompleteness::Complete,
                Some(&["io::read"]),
            ),
        ])
        .expect("analysis succeeds");

        assert_eq!(analysis.violations().len(), 1);
        assert_eq!(
            analysis.contract_status(),
            EffectContractStatus::Violated
        );
    }

    #[test]
    fn incomplete_declared_contract_is_inconclusive() {
        let analysis = EffectAnalysis::analyze([
            input(
                1,
                &[],
                &[],
                EffectCompleteness::Partial,
                Some(&[]),
            ),
        ])
        .expect("analysis succeeds");

        assert_eq!(
            analysis.contract_status(),
            EffectContractStatus::Inconclusive
        );

        assert!(!analysis.contracts_fully_verified());
    }

    #[test]
    fn effect_union_is_deterministic() {
        let analysis = EffectAnalysis::analyze([
            input(
                1,
                &["z::effect", "a::effect"],
                &[],
                EffectCompleteness::Complete,
                None,
            ),
        ])
        .expect("analysis succeeds");

        let names: Vec<String> = analysis
            .aggregate()
            .effects()
            .iter()
            .map(ToString::to_string)
            .collect();

        let mut sorted = names.clone();
        sorted.sort();

        assert_eq!(names, sorted);
    }
}
