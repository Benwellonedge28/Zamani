
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Effect Analysis.
//!
//! Path: src/compiler/analysis/effects.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust forbidden.
//!
//! PURPOSE
//!
//! Perform compiler-level effect analysis using the canonical ZUIR effect
//! model. This module extends operation-level effect analysis with
//! function-level and interprocedural propagation.
//!
//! RESPONSIBILITIES
//!
//! - Aggregate directly observed function effects.
//! - Propagate effects through function-call relationships.
//! - Support recursive and mutually recursive call graphs.
//! - Preserve Unknown, Partial, and Complete effect information.
//! - Validate inferred effects against declared upper bounds.
//! - Validate effects against caller-supplied authorization policies.
//! - Produce deterministic analysis results and structured errors.
//!
//! DOES NOT OWN
//!
//! - Source grammar or parsing.
//! - AST construction.
//! - Semantic effect identity.
//! - The canonical effect vocabulary.
//! - Canonical ZUIR operations or functions.
//! - Function name or symbol resolution.
//! - Call-graph discovery.
//! - Physical hardware selection.
//! - Resource allocation.
//! - Runtime effect enforcement.
//! - Effect-handler execution.
//!
//! CANONICAL AUTHORITIES
//!
//! crate::zuir::ids::FunctionId
//! crate::zuir::ir::effects
//! crate::zuir::analysis::effects
//!
//! The semantic layer discovers source-level effects.
//! ZUIR owns canonical effect declarations.
//! This module performs compiler-level aggregation and propagation.
//!
//! EFFECT COMPLETENESS
//!
//! Empty effects do not imply purity.
//!
//! Complete summaries are permitted only when the caller supplies
//! independently verified complete direct-effect information.
//!
//! An unknown external function must not be treated as pure.
//!
//! SCALABILITY
//!
//! No arbitrary maximum is imposed on functions, call edges, effects,
//! computing domains, devices, processors, or other hardware.
//!
//! The worklist algorithm is iterative and supports recursive call graphs.
//! Actual operation is bounded by available memory and processing capacity.
//!
//! INTEGRATION
//!
//! Parent: src/compiler/analysis/mod.rs
//!
//! Required declaration:
//!
//! pub mod effects;
//!
//! Parent: src/compiler/mod.rs
//!
//! Required declaration:
//!
//! pub mod analysis;
//!
//! The canonical operation analyzer remains:
//!
//! crate::zuir::analysis::effects::EffectAnalysis
//!
//! The canonical policy remains:
//!
//! crate::zuir::ir::effects::EffectPolicy

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::fmt;

use crate::zuir::ids::FunctionId;

use crate::zuir::ir::effects::{
    EffectCompleteness,
    EffectError,
    EffectPolicy,
    EffectSet,
    EffectSummary,
};

/// Result type for compiler effect analysis.
pub type CompilerEffectResult<T> =
    Result<T, CompilerEffectError>;

/// Compiler effect-analysis errors.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompilerEffectError {

    /// A function identity was supplied more than once.
    DuplicateFunction {
        function: FunctionId,
    },

    /// A function references a missing callee.
    UnknownCallee {
        caller: FunctionId,
        callee: FunctionId,
    },

    /// A function's direct effect summary is malformed.
    InvalidDirectEffects {
        function: FunctionId,
        source: EffectError,
    },

    /// A function's declared effect set is malformed.
    InvalidDeclaredEffects {
        function: FunctionId,
        source: EffectError,
    },

    /// An authorization policy rejected an effect.
    PolicyViolation {
        function: FunctionId,
        source: EffectError,
    },

    /// Internal monotonicity invariant was violated.
    ///
    /// Effect propagation must never remove a previously discovered
    /// effect or increase completeness.
    NonMonotonicPropagation {
        function: FunctionId,
    },
}

impl fmt::Display for CompilerEffectError {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        match self {

            Self::DuplicateFunction { function } => {
                write!(
                    formatter,
                    "duplicate function in effect analysis: {function}"
                )
            }

            Self::UnknownCallee { caller, callee } => {
                write!(
                    formatter,
                    "function {caller} references unknown callee {callee}"
                )
            }

            Self::InvalidDirectEffects {
                function,
                source,
            } => {
                write!(
                    formatter,
                    "invalid direct effects for function {function}: {source}"
                )
            }

            Self::InvalidDeclaredEffects {
                function,
                source,
            } => {
                write!(
                    formatter,
                    "invalid declared effects for function {function}: {source}"
                )
            }

            Self::PolicyViolation {
                function,
                source,
            } => {
                write!(
                    formatter,
                    "effect authorization failed for function {function}: {source}"
                )
            }

            Self::NonMonotonicPropagation { function } => {
                write!(
                    formatter,
                    "non-monotonic effect propagation detected for function {function}"
                )
            }
        }
    }
}

impl Error for CompilerEffectError {

    fn source(&self) -> Option<&(dyn Error + 'static)> {

        match self {

            Self::InvalidDirectEffects { source, .. }
            | Self::InvalidDeclaredEffects { source, .. }
            | Self::PolicyViolation { source, .. } => Some(source),

            _ => None,
        }
    }
}

// ============================================================================
// FUNCTION INPUT
// ============================================================================

/// Input describing the effect-related information for one function.
///
/// This is a compiler analysis input, not a replacement ZUIR function.
///
/// The caller obtains:
///
/// - Function identity from canonical ZUIR.
/// - Direct effects from canonical operation analysis.
/// - Callee relationships from resolved call-graph analysis.
/// - Declared effects from canonical function declarations.
///
/// The caller remains responsible for proving that these inputs describe
/// the correct complete function body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionEffectInput {

    function: FunctionId,

    direct_effects: EffectSummary,

    callees: BTreeSet<FunctionId>,

    declared_effects: Option<EffectSet>,
}

impl FunctionEffectInput {

    /// Construct an effect-analysis input.
    ///
    /// `declared_effects`:
    ///
    /// Some(set):
    ///     An explicit declared upper bound.
    ///
    /// None:
    ///     No declared upper bound was supplied.
    ///
    /// `direct_effects`:
    ///
    /// Must preserve the completeness established by the producer.
    /// Do not mark a summary Complete merely because operation traversal
    /// completed.
    pub fn new(
        function: FunctionId,
        direct_effects: EffectSummary,
        callees: impl IntoIterator<Item = FunctionId>,
        declared_effects: Option<EffectSet>,
    ) -> CompilerEffectResult<Self> {

        direct_effects
            .validate()
            .map_err(|source| {
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

    /// Return direct effect information.
    #[must_use]
    pub const fn direct_effects(&self) -> &EffectSummary {
        &self.direct_effects
    }

    /// Return resolved callees in deterministic order.
    #[must_use]
    pub fn callees(&self) -> &BTreeSet<FunctionId> {
        &self.callees
    }

    /// Return the optional declared effect upper bound.
    #[must_use]
    pub const fn declared_effects(&self) -> Option<&EffectSet> {
        self.declared_effects.as_ref()
    }
}

// ============================================================================
// FUNCTION RESULT
// ============================================================================

/// Inferred effects for one function.
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

    /// Return the declared effect upper bound.
    #[must_use]
    pub const fn declared_effects(&self) -> Option<&EffectSet> {
        self.declared_effects.as_ref()
    }

    /// Return whether the function has a complete effect summary.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.summary.is_complete()
    }

    /// Check whether purity is proven.
    ///
    /// Returns an error when completeness is insufficient.
    pub fn is_proven_pure(
        &self,
    ) -> Result<bool, EffectError> {

        self.summary.is_proven_pure()
    }
}

// ============================================================================
// CONTRACT VIOLATIONS
// ============================================================================

/// A known inferred effect missing from a function's declared upper bound.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectContractViolation {

    pub function: FunctionId,

    pub effect: String,
}

impl fmt::Display for EffectContractViolation {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        write!(
            formatter,
            "function {} does not declare inferred effect `{}`",
            self.function,
            self.effect
        )
    }
}

// ============================================================================
// ANALYSIS RESULT
// ============================================================================

/// Immutable result of interprocedural effect analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectAnalysis {

    functions: BTreeMap<FunctionId, FunctionEffectResult>,

    violations: Vec<EffectContractViolation>,
}

impl EffectAnalysis {

    /// Analyze an explicitly supplied resolved function graph.
    ///
    /// The algorithm uses an iterative reverse-dependency worklist.
    ///
    /// Recursive calls and mutually recursive calls are supported.
    ///
    /// Effects propagate from callees to callers until a fixed point
    /// is reached.
    ///
    /// A missing callee is an error rather than an assumption of purity.
    ///
    /// All results preserve effect completeness.
    pub fn analyze<I>(
        inputs: I,
    ) -> CompilerEffectResult<Self>
    where
        I: IntoIterator<Item = FunctionEffectInput>,
    {

        let mut inputs_by_id =
            BTreeMap::<FunctionId, FunctionEffectInput>::new();

        // ------------------------------------------------------------
        // 1. Register and validate all functions.
        // ------------------------------------------------------------

        for input in inputs {

            let id = input.function;

            if inputs_by_id.contains_key(&id) {

                return Err(
                    CompilerEffectError::DuplicateFunction {
                        function: id,
                    },
                );
            }

            input.direct_effects.validate().map_err(|source| {
                CompilerEffectError::InvalidDirectEffects {
                    function: id,
                    source,
                }
            })?;

            inputs_by_id.insert(id, input);
        }

        // ------------------------------------------------------------
        // 2. Validate all call references.
        // ------------------------------------------------------------

        let mut callers =
            BTreeMap::<FunctionId, BTreeSet<FunctionId>>::new();

        for (&caller, input) in &inputs_by_id {

            for &callee in &input.callees {

                if !inputs_by_id.contains_key(&callee) {

                    return Err(
                        CompilerEffectError::UnknownCallee {
                            caller,
                            callee,
                        },
                    );
                }

                callers
                    .entry(callee)
                    .or_default()
                    .insert(caller);
            }
        }

        // ------------------------------------------------------------
        // 3. Initialize summaries from direct effects.
        // ------------------------------------------------------------

        let mut summaries =
            BTreeMap::<FunctionId, EffectSummary>::new();

        for (&id, input) in &inputs_by_id {

            summaries.insert(
                id,
                input.direct_effects.clone(),
            );
        }

        // ------------------------------------------------------------
        // 4. Initialize deterministic worklist.
        // ------------------------------------------------------------

        let mut pending =
            VecDeque::<FunctionId>::new();

        let mut queued =
            BTreeSet::<FunctionId>::new();

        for &id in inputs_by_id.keys() {

            pending.push_back(id);

            queued.insert(id);
        }

        // ------------------------------------------------------------
        // 5. Propagate callee effects to callers.
        // ------------------------------------------------------------

        while let Some(function_id) = pending.pop_front() {

            queued.remove(&function_id);

            let input = inputs_by_id
                .get(&function_id)
                .expect("registered function must exist");

            let mut inferred =
                input.direct_effects.effects().clone();

            let mut completeness =
                input.direct_effects.completeness();

            for callee in &input.callees {

                let callee_summary = summaries
                    .get(callee)
                    .expect("validated callee must have a summary");

                inferred.extend(
                    callee_summary.effects()
                );

                completeness = combine_completeness(
                    completeness,
                    callee_summary.completeness(),
                );
            }

            let next = EffectSummary::new(
                inferred,
                completeness,
            );

            let previous = summaries
                .get(&function_id)
                .expect("registered function must have a summary");

            // --------------------------------------------------------
            // 6. Enforce monotonicity.
            // --------------------------------------------------------

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

                // Only callers can be affected by a changed summary.
                if let Some(dependents) = callers.get(&function_id) {

                    for &caller in dependents {

                        if queued.insert(caller) {
                            pending.push_back(caller);
                        }
                    }
                }
            }
        }

        // ------------------------------------------------------------
        // 7. Construct immutable function results.
        // ------------------------------------------------------------

        let mut functions =
            BTreeMap::<FunctionId, FunctionEffectResult>::new();

        for (id, input) in inputs_by_id {

            let summary = summaries
                .remove(&id)
                .expect("every registered function must be analyzed");

            functions.insert(
                id,
                FunctionEffectResult {
                    function: id,
                    summary,
                    declared_effects: input.declared_effects,
                },
            );
        }

        // ------------------------------------------------------------
        // 8. Validate declared upper bounds.
        // ------------------------------------------------------------

        let mut violations = Vec::new();

        for (&id, result) in &functions {

            let Some(declared) = &result.declared_effects else {
                continue;
            };

            for effect in result.summary.effects().iter() {

                if !declared.contains(effect) {

                    violations.push(
                        EffectContractViolation {
                            function: id,
                            effect: effect.to_string(),
                        }
                    );
                }
            }
        }

        Ok(Self {
            functions,
            violations,
        })
    }

    /// Return the number of analyzed functions.
    #[must_use]
    pub fn function_count(&self) -> usize {
        self.functions.len()
    }

    /// Return whether the analysis contains no functions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.functions.is_empty()
    }

    /// Look up a function's inferred effects.
    #[must_use]
    pub fn function(
        &self,
        id: FunctionId,
    ) -> Option<&FunctionEffectResult> {

        self.functions.get(&id)
    }

    /// Iterate over function results in deterministic ID order.
    pub fn functions(
        &self,
    ) -> impl Iterator<Item = (FunctionId, &FunctionEffectResult)> {

        self.functions
            .iter()
            .map(|(id, result)| (*id, result))
    }

    /// Return declared-effect contract violations.
    #[must_use]
    pub fn violations(
        &self,
    ) -> &[EffectContractViolation] {

        &self.violations
    }

    /// Return whether every supplied function satisfies its known
    /// declared effect upper bound.
    #[must_use]
    pub fn contracts_satisfied(&self) -> bool {
        self.violations.is_empty()
    }

    /// Return all known effects across analyzed functions.
    ///
    /// This is a union of function summaries, not an execution-order model.
    ///
    /// Completeness is conservative: if any function is incomplete,
    /// the aggregate is incomplete.
    pub fn aggregate(&self) -> EffectSummary {

        let mut effects = EffectSet::new();

        let mut completeness =
            EffectCompleteness::Complete;

        for result in self.functions.values() {

            effects.extend(
                result.summary.effects()
            );

            completeness = combine_completeness(
                completeness,
                result.summary.completeness(),
            );
        }

        EffectSummary::new(effects, completeness)
    }

    /// Validate all known effects against an authorization policy.
    ///
    /// Policies are checked independently for every function.
    ///
    /// This does not perform runtime authorization.
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
}

// ============================================================================
// COMPLETENESS
// ============================================================================

/// Combine effect knowledge conservatively.
///
/// Unknown dominates Partial.
/// Partial dominates Complete.
///
/// A composition is complete only when every contributing component
/// is complete.
fn combine_completeness(
    left: EffectCompleteness,
    right: EffectCompleteness,
) -> EffectCompleteness {

    match (left, right) {

        (
            EffectCompleteness::Unknown,
            _
        )
        | (
            _,
            EffectCompleteness::Unknown
        ) => EffectCompleteness::Unknown,

        (
            EffectCompleteness::Partial,
            _
        )
        | (
            _,
            EffectCompleteness::Partial
        ) => EffectCompleteness::Partial,

        (
            EffectCompleteness::Complete,
            EffectCompleteness::Complete
        ) => EffectCompleteness::Complete,
    }
}

/// Internal ordering for monotonicity verification.
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

    fn function_id(value: u128) -> FunctionId {

        FunctionId::new(value)
            .expect("nonzero function ID")
    }

    fn effects(
        names: &[&str],
    ) -> EffectSet {

        EffectSet::from_names(
            names.iter().map(|name| {

                crate::zuir::schema::types::QualifiedName::parse(
                    name
                )
                .expect("valid effect name")
            })
        )
        .expect("valid effect set")
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
            )
        ])
        .expect("analysis succeeds");

        let result = analysis
            .function(function_id(1))
            .expect("function exists");

        assert!(
            result.summary().effects().contains(
                &crate::zuir::schema::types::QualifiedName::parse(
                    "io::read"
                ).unwrap()
            )
        );

        assert!(result.is_complete());
    }

    #[test]
    fn effects_propagate_from_callee_to_caller() {

        let analysis = EffectAnalysis::analyze([
            input(
                1,
                &[],
                &[2],
                EffectCompleteness::Complete,
                Some(&["io::read"]),
            ),
            input(
                2,
                &["io::read"],
                &[],
                EffectCompleteness::Complete,
                Some(&["io::read"]),
            ),
        ])
        .expect("analysis succeeds");

        let caller = analysis
            .function(function_id(1))
            .expect("caller exists");

        assert!(
            caller.summary().effects().contains(
                &crate::zuir::schema::types::QualifiedName::parse(
                    "io::read"
                ).unwrap()
            )
        );
    }

    #[test]
    fn mutually_recursive_functions_converge() {

        let analysis = EffectAnalysis::analyze([
            input(
                1,
                &["memory::write"],
                &[2],
                EffectCompleteness::Complete,
                Some(&["memory::write", "io::read"]),
            ),
            input(
                2,
                &["io::read"],
                &[1],
                EffectCompleteness::Complete,
                Some(&["memory::write", "io::read"]),
            ),
        ])
        .expect("recursive analysis succeeds");

        let first = analysis
            .function(function_id(1))
            .unwrap();

        let second = analysis
            .function(function_id(2))
            .unwrap();

        assert_eq!(
            first.summary().effects(),
            second.summary().effects()
        );
    }

    #[test]
    fn unknown_effect_information_propagates() {

        let analysis = EffectAnalysis::analyze([
            input(
                1,
                &[],
                &[2],
                EffectCompleteness::Complete,
                None,
            ),
            input(
                2,
                &[],
                &[],
                EffectCompleteness::Unknown,
                None,
            ),
        ])
        .expect("analysis succeeds");

        assert_eq!(
            analysis.function(function_id(1))
                .unwrap()
                .summary()
                .completeness(),
            EffectCompleteness::Unknown
        );
    }

    #[test]
    fn partial_information_never_proves_purity() {

        let analysis = EffectAnalysis::analyze([
            input(
                1,
                &[],
                &[],
                EffectCompleteness::Partial,
                Some(&[]),
            )
        ])
        .expect("analysis succeeds");

        assert!(
            analysis.function(function_id(1))
                .unwrap()
                .is_proven_pure()
                .is_err()
        );
    }

    #[test]
    fn empty_complete_scope_can_prove_purity() {

        let analysis = EffectAnalysis::analyze([
            input(
                1,
                &[],
                &[],
                EffectCompleteness::Complete,
                Some(&[]),
            )
        ])
        .expect("analysis succeeds");

        assert_eq!(
            analysis.function(function_id(1))
                .unwrap()
                .is_proven_pure(),
            Ok(true)
        );
    }

    #[test]
    fn undeclared_effects_are_reported() {

        let analysis = EffectAnalysis::analyze([
            input(
                1,
                &["network::request"],
                &[],
                EffectCompleteness::Complete,
                Some(&[]),
            )
        ])
        .expect("analysis succeeds");

        assert!(!analysis.contracts_satisfied());

        assert_eq!(analysis.violations().len(), 1);
    }

    #[test]
    fn unknown_callees_are_rejected() {

        let result = EffectAnalysis::analyze([
            input(
                1,
                &[],
                &[2],
                EffectCompleteness::Complete,
                None,
            )
        ]);

        assert!(matches!(
            result,
            Err(CompilerEffectError::UnknownCallee { .. })
        ));
    }

    #[test]
    fn duplicate_functions_are_rejected() {

        let result = EffectAnalysis::analyze([
            input(
                1,
                &[],
                &[],
                EffectCompleteness::Complete,
                None,
            ),
            input(
                1,
                &[],
                &[],
                EffectCompleteness::Complete,
                None,
            ),
        ]);

        assert!(matches!(
            result,
            Err(CompilerEffectError::DuplicateFunction { .. })
        ));
    }

    #[test]
    fn result_order_is_deterministic() {

        let analysis = EffectAnalysis::analyze([
            input(
                3,
                &[],
                &[],
                EffectCompleteness::Complete,
                None,
            ),
            input(
                1,
                &[],
                &[],
                EffectCompleteness::Complete,
                None,
            ),
            input(
                2,
                &[],
                &[],
                EffectCompleteness::Complete,
                None,
            ),
        ])
        .expect("analysis succeeds");

        let ids = analysis
            .functions()
            .map(|(id, _)| id)
            .collect::<Vec<_>>();

        assert_eq!(
            ids,
            vec![
                function_id(1),
                function_id(2),
                function_id(3),
            ]
        );
    }
}
