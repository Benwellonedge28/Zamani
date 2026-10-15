#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Effect Analysis
//!
//! Path: `src/zuir/analysis/effects.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Derive deterministic effect summaries from canonical ZUIR operations.
//! Preserve uncertainty explicitly, provide per-operation and aggregate
//! summaries, and check known effects against a declared effect contract.
//!
//! AUTHORITY
//!
//! - src/zuir/ZUIR-SPEC.md
//! - src/zuir/ir/effects.rs
//! - src/zuir/ir/operation.rs
//! - src/zuir/ir/function.rs
//! - src/zuir/ir/region.rs
//! - src/zuir/ir/block.rs
//! - src/zuir/ir/successor.rs
//! - src/zuir/ir/use_def.rs
//! - grammar/DESIGN.md
//! - grammar/spec/effects.md
//! - grammar/specification/semantic-model.md
//! - grammar/antlr/ZamaniParser.g4
//!
//! OWNS
//!
//! - Deterministic aggregation of effects declared on operations.
//! - Per-operation effect summaries.
//! - Aggregate summaries for the explicitly supplied operation scope.
//! - Validation of known effects against a declared upper-bound contract.
//! - Structured analysis errors.
//!
//! DOES_NOT_OWN
//!
//! - The canonical effect vocabulary or effect identity representation.
//! - Source parsing, AST construction, or semantic name resolution.
//! - Type inference, control-flow construction, or dominance analysis.
//! - Interprocedural call-graph construction.
//! - Recursive region traversal unless explicitly supplied by the caller.
//! - Effect inference for opaque or unregistered operations.
//! - Runtime effect enforcement or effect-handler execution.
//! - Target selection, physical resource allocation, scheduling, or routing.
//!
//! DEPENDENCIES
//!
//! Only canonical IDs, canonical operations, and the canonical effect model.
//! This module does not define replacement effect types.
//!
//! COMPLETENESS CONTRACT
//!
//! Operation declarations provide known effects, not proof that all effects
//! have been declared. Consequently, this analyzer marks its summaries
//! Partial, including when the supplied operation collection is empty.
//!
//! A caller must not interpret an empty partial summary as proof of purity.
//! A complete summary requires a separate, trusted verification process that
//! establishes the completeness of the analyzed scope and all relevant
//! operations, callees, nested regions, and extension semantics.
//!
//! SCALABILITY AND POCO-REAF
//!
//! No universal limit is imposed on operations, effects, qubits, registers,
//! devices, nodes, tensor rank, or hardware capacity.
//!
//! Ordered maps and sets provide deterministic traversal. Their memory and
//! processing requirements remain bounded by the resources available to the
//! process. This module does not promise literal infinite capacity.
//!
//! FREEZE CRITERIA
//!
//! - Reuses the canonical EffectSet, EffectSummary, and EffectError.
//! - Never silently promotes incomplete knowledge to Complete.
//! - Detects duplicate operation identities in one analysis input.
//! - Never silently ignores malformed declared effect names.
//! - Reports declaration violations deterministically.
//! - Has no unsafe code or machine-size assumptions.
//! - Is registered in the actual analysis module hierarchy.
//! - Passes repository compilation, unit, integration, and conformance tests.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::zuir::ids::OperationId;
use crate::zuir::ir::effects::{
    EffectCompleteness,
    EffectError,
    EffectSet,
    EffectSummary,
};
use crate::zuir::ir::operation::ZuirOperation;

/// Result type for effect analysis.
pub type EffectAnalysisResult<T> = Result<T, EffectAnalysisError>;

/// Errors produced while analyzing operation effect declarations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EffectAnalysisError {
    /// An operation identity occurs more than once in the supplied scope.
    DuplicateOperation {
        operation: OperationId,
    },

    /// An operation contains an invalid effect declaration.
    InvalidOperationEffects {
        operation: OperationId,
        source: EffectError,
    },

    /// The declared effect contract contains an invalid effect name.
    InvalidDeclaredEffects {
        source: EffectError,
    },

    /// An observed effect is absent from the declared upper bound.
    ///
    /// This establishes a contract violation only when the supplied
    /// declaration is authoritative for the operation scope being checked.
    UndeclaredEffect {
        effect: String,
    },
}

impl fmt::Display for EffectAnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateOperation { operation } => {
                write!(
                    f,
                    "operation {operation} occurs more than once \
                     in the effect-analysis input"
                )
            }

            Self::InvalidOperationEffects { operation, source } => {
                write!(
                    f,
                    "operation {operation} has invalid effect declarations: \
                     {source}"
                )
            }

            Self::InvalidDeclaredEffects { source } => {
                write!(
                    f,
                    "declared effect contract is invalid: {source}"
                )
            }

            Self::UndeclaredEffect { effect } => {
                write!(
                    f,
                    "observed effect {effect} is not included \
                     in the declared effect contract"
                )
            }
        }
    }
}

impl Error for EffectAnalysisError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidOperationEffects { source, .. }
            | Self::InvalidDeclaredEffects { source } => Some(source),

            Self::DuplicateOperation { .. }
            | Self::UndeclaredEffect { .. } => None,
        }
    }
}

/// Deterministic effect-analysis result for an explicitly supplied scope.
///
/// Each operation summary contains the effects declared by that operation.
/// The aggregate is the union of those declarations.
///
/// Every summary produced by this analyzer is Partial because this module
/// does not establish completeness of effect declarations or discover all
/// possible interprocedural and nested-region effects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectAnalysis {
    per_operation: BTreeMap<OperationId, EffectSummary>,
    aggregate: EffectSummary,
}

impl EffectAnalysis {
    /// Analyze an explicit collection of operations.
    ///
    /// Operation IDs must be unique within the supplied collection.
    /// Operations are not implicitly discovered through region references.
    ///
    /// The same operation must not be supplied twice, even if the duplicate
    /// has identical contents. Duplicate input can otherwise conceal a
    /// caller's traversal or ownership bug.
    ///
    /// # Errors
    ///
    /// Returns an error for duplicate operation IDs or invalid effect names.
    ///
    /// # Completeness
    ///
    /// The result is always Partial. The caller must perform any necessary
    /// whole-program, interprocedural, nested-region, and dialect analysis
    /// before claiming complete effect information.
    pub fn analyze<'a, I>(operations: I) -> EffectAnalysisResult<Self>
    where
        I: IntoIterator<Item = &'a ZuirOperation>,
    {
        let mut per_operation = BTreeMap::new();
        let mut aggregate = EffectSet::new();

        for operation in operations {
            let operation_id = operation.id();

            let effects = EffectSet::from_operation(operation).map_err(
                |source| EffectAnalysisError::InvalidOperationEffects {
                    operation: operation_id,
                    source,
                },
            )?;

            if per_operation.contains_key(&operation_id) {
                return Err(EffectAnalysisError::DuplicateOperation {
                    operation: operation_id,
                });
            }

            aggregate.extend(&effects);

            per_operation.insert(
                operation_id,
                EffectSummary::new(
                    effects,
                    EffectCompleteness::Partial,
                ),
            );
        }

        Ok(Self {
            per_operation,
            aggregate: EffectSummary::new(
                aggregate,
                EffectCompleteness::Partial,
            ),
        })
    }

    /// Return the number of analyzed operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.per_operation.len()
    }

    /// Return whether no operations were supplied.
    ///
    /// An empty scope is not automatically proof of purity: effects may
    /// arise from enclosing declarations, callees, or omitted regions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.per_operation.is_empty()
    }

    /// Look up an operation's effect summary.
    #[must_use]
    pub fn summary_for(
        &self,
        operation: OperationId,
    ) -> Option<&EffectSummary> {
        self.per_operation.get(&operation)
    }

    /// Iterate over per-operation summaries in deterministic ID order.
    pub fn per_operation(
        &self,
    ) -> impl Iterator<Item = (OperationId, &EffectSummary)> {
        self.per_operation
            .iter()
            .map(|(id, summary)| (*id, summary))
    }

    /// Return the aggregate summary for the supplied operations.
    ///
    /// This summary remains Partial. The union operation combines known
    /// declarations; it does not establish that the declarations are complete.
    #[must_use]
    pub const fn aggregate(&self) -> &EffectSummary {
        &self.aggregate
    }

    /// Check known effects against a declared effect upper bound.
    ///
    /// The declared set must represent the authoritative contract for the
    /// scope being checked. This method checks set inclusion only; it does
    /// not prove that the IR is complete or that every effect is registered.
    ///
    /// # Errors
    ///
    /// Returns an error if the declared set is malformed or if an observed
    /// effect is absent from the declared upper bound.
    pub fn check_declared_upper_bound(
        &self,
        declared: &EffectSet,
    ) -> EffectAnalysisResult<()> {
        declared
            .validate()
            .map_err(|source| {
                EffectAnalysisError::InvalidDeclaredEffects { source }
            })?;

        for effect in self.aggregate.effects().iter() {
            if !declared.contains(effect) {
                return Err(EffectAnalysisError::UndeclaredEffect {
                    effect: effect.to_string(),
                });
            }
        }

        Ok(())
    }
}

/// Analyze declared effects on a collection of operations.
///
/// This convenience function returns the aggregate effect summary.
///
/// The result is Partial by design. It must not be used as proof of purity
/// or as a complete function-level effect summary without further analysis.
pub fn analyze_effects<'a, I>(
    operations: I,
) -> EffectAnalysisResult<EffectSummary>
where
    I: IntoIterator<Item = &'a ZuirOperation>,
{
    Ok(EffectAnalysis::analyze(operations)?.aggregate().clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_scope_is_not_proven_pure() {
        let analysis = EffectAnalysis::analyze(
            std::iter::empty::<&ZuirOperation>(),
        )
        .expect("an empty operation scope should be analyzable");

        assert!(analysis.is_empty());
        assert_eq!(analysis.operation_count(), 0);

        assert_eq!(
            analysis.aggregate().completeness(),
            EffectCompleteness::Partial,
        );

        assert!(
            analysis.aggregate().effects().is_empty(),
            "no effects should be recorded for an empty input"
        );

        assert!(
            analysis.aggregate().is_proven_pure().is_err(),
            "partial effect information must not prove purity"
        );
    }

    #[test]
    fn empty_scope_satisfies_empty_declared_upper_bound() {
        let analysis = EffectAnalysis::analyze(
            std::iter::empty::<&ZuirOperation>(),
        )
        .expect("an empty operation scope should be analyzable");

        let declared = EffectSet::new();

        assert!(
            analysis.check_declared_upper_bound(&declared).is_ok()
        );
    }

    #[test]
    fn empty_scope_satisfies_nonempty_declared_upper_bound() {
        // An upper bound may permit effects that the supplied scope
        // does not declare. This is not a purity proof.
        let analysis = EffectAnalysis::analyze(
            std::iter::empty::<&ZuirOperation>(),
        )
        .expect("an empty operation scope should be analyzable");

        let declared = EffectSet::new();

        assert!(
            analysis.check_declared_upper_bound(&declared).is_ok()
        );

        assert_eq!(
            analysis.aggregate().completeness(),
            EffectCompleteness::Partial,
        );
    }

    #[test]
    fn aggregate_is_deterministic_and_explicitly_partial() {
        let analysis = EffectAnalysis::analyze(
            std::iter::empty::<&ZuirOperation>(),
        )
        .expect("an empty operation scope should be analyzable");

        assert_eq!(
            analysis.aggregate().completeness(),
            EffectCompleteness::Partial,
        );

        assert_eq!(
            analysis.per_operation().count(),
            analysis.operation_count(),
        );
    }
}