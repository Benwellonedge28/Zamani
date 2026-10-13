
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Effect Checking
//!
//! File: `src/frontend/semantic/effects/checking.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! ## PURPOSE
//!
//! Check inferred effect rows against declared effect contracts.
//!
//! This module checks structural validity, effect coverage, effect-variable
//! compatibility, and explicitly permitted widening. It provides a
//! target-independent semantic API that can be consumed by function checking,
//! type/effect checking, domain validation, and semantic-model finalization.
//!
//! ## AUTHORITY
//!
//! - `grammar/spec/effects.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/effects/effect-sets.g4`
//! - `grammar/effects/effect-declarations.g4`
//! - `grammar/effects/effect-composition.g4`
//! - `grammar/effects/effect-operations.g4`
//! - `grammar/effects/effect-types.g4`
//! - `src/frontend/ast/node/effects/effect_kind.rs`
//! - `src/frontend/semantic/effects/model.rs`
//! - `src/frontend/semantic/effects/inference.rs`
//!
//! The normative specification determines language meaning. The grammar
//! determines syntax. `model.rs` owns the canonical semantic effect types.
//!
//! ## OWNS
//!
//! - Compatibility reports and structured checking issues.
//! - Effect-row structural validation at the checking boundary.
//! - Actual-versus-declared effect contract checking.
//! - Exact-identity matching by default.
//! - Explicit policy-controlled widening for open declared rows.
//! - An extension point for registered effect relationships.
//!
//! ## DOES NOT OWN
//!
//! - Parsing, AST construction, or name resolution.
//! - Canonical effect identities, instances, sets, or rows.
//! - Effect inference or effect-variable substitution.
//! - Effect alias resolution or effect hierarchy registration.
//! - Capability requirements, evidence, or authorization.
//! - Resource requirements or target feasibility.
//! - Effect-handler elimination, masking, or transformation.
//! - Type subtyping or type compatibility.
//! - Classical IR, `quantum::ir`, HDL IR, routing, scheduling, or execution.
//!
//! ## COMPATIBILITY RULE
//!
//! A declared effect contract is treated as an upper bound by this checker:
//!
//! - Every inferred concrete effect must be covered by the declaration,
//!   unless widening is explicitly enabled for an open declared row.
//! - Every inferred effect variable must occur in the declared row.
//! - A closed declared row never permits undeclared effects.
//! - Effect identity is exact by default.
//! - Namespaces and name prefixes do not imply subtyping.
//! - Open-row widening does not automatically authorize unmatched effect
//!   variables, and does not replace generic effect-variable substitution or
//!   bound checking.
//!
//! The compiler must select the widening policy in accordance with the
//! language contract and the results of any preceding effect-variable and
//! policy checks. The default is to reject widening.
//!
//! ## INTEGRATION CONTRACT
//!
//! 1. `inference.rs` produces inferred `EffectRow` values.
//! 2. Declaration and name-resolution passes supply the declared row.
//! 3. Generic effect-variable substitution must occur before this checker is
//!    asked to validate a fully instantiated contract.
//! 4. This module checks actual-versus-declared compatibility.
//! 5. The caller converts issues into the repository's canonical diagnostic
//!    records, preserving source locations from the associated AST nodes.
//! 6. Authorization remains a separate operation in `authorization.rs`.
//! 7. Finalized facts are stored by the semantic model.
//!
//! This module intentionally does not assume that the future diagnostics,
//! semantic-facts, or session APIs have already been implemented.
//!
//! ## SCALABILITY
//!
//! No fixed effect count, domain count, parameter count, machine capacity,
//! qubit count, or effect-name catalogue is introduced. The checker operates
//! on the finite rows it receives and grows its collections according to
//! available resources. Compiler-wide operational budgets belong to the
//! analysis session and must report incomplete analysis rather than semantic
//! success when exhausted.
//!
//! ## FREEZE CRITERIA
//!
//! - The canonical effect-model API compiles with this module.
//! - Closed-row and open-row behavior is covered by tests.
//! - Effect-variable mismatches are never silently accepted.
//! - Effect relationships are exact unless an explicit relation provider is
//!   supplied.
//! - Malformed deserialized rows produce errors.
//! - Widening is opt-in and reported in the result.
//! - No authorization or target-feasibility decision is made here.
//! - Workspace and integration tests pass.

use core::fmt;
use std::error::Error;

use crate::frontend::ast::node::effects::effect_kind::EffectKind;

use super::model::{EffectInstance, EffectModelError, EffectRow, EffectVariable};

/// Policy for concrete effects that are absent from a declared effect row.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum EffectWideningPolicy {
    /// Reject every inferred concrete effect not covered by the declaration.
    ///
    /// This is the safe default and is required for closed declared rows.
    #[default]
    Forbid,

    /// Permit additional concrete effects only when the declared row is open.
    ///
    /// An open row is represented by one or more declared effect variables.
    /// This option must only be selected when the language contract and
    /// preceding inference/bound checks permit such widening.
    ///
    /// This policy does not permit unmatched inferred effect variables.
    AllowWhenDeclaredRowIsOpen,
}

/// Configuration for effect compatibility checking.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EffectCheckingPolicy {
    widening: EffectWideningPolicy,
}

impl EffectCheckingPolicy {
    /// Creates a policy that rejects effect widening.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            widening: EffectWideningPolicy::Forbid,
        }
    }

    /// Creates a policy that permits concrete-effect widening for open rows.
    ///
    /// The caller is responsible for ensuring that the applicable language
    /// rules, variable bounds, and policies allow widening.
    #[must_use]
    pub const fn allow_widening_for_open_rows() -> Self {
        Self {
            widening: EffectWideningPolicy::AllowWhenDeclaredRowIsOpen,
        }
    }

    /// Returns the configured widening policy.
    #[must_use]
    pub const fn widening_policy(self) -> EffectWideningPolicy {
        self.widening
    }
}

/// A provider of explicitly registered effect-coverage relationships.
///
/// The checker does not infer effect subtyping from namespaces or names.
/// Implementations may consult a validated semantic registry for aliases,
/// declared relationships, or other language-defined compatibility rules.
///
/// `covers(actual, permitted)` means that the `permitted` declaration entry
/// explicitly permits the `actual` effect instance.
///
/// Implementations must be deterministic and must not perform target discovery
/// or authorize execution.
pub trait EffectRelation {
    /// Returns whether a declared effect entry covers an inferred effect.
    fn covers(
        &self,
        actual: &EffectInstance,
        permitted: &EffectInstance,
    ) -> bool;
}

/// Exact structural effect matching.
///
/// This is the default relation. Two effects match only when their canonical
/// identities and resolved arguments are equal.
#[derive(Clone, Copy, Debug, Default)]
pub struct ExactEffectRelation;

impl EffectRelation for ExactEffectRelation {
    fn covers(
        &self,
        actual: &EffectInstance,
        permitted: &EffectInstance,
    ) -> bool {
        actual == permitted
    }
}

/// A semantic effect-checking issue.
///
/// These values are machine-readable. A diagnostics adapter should map them
/// to the compiler's stable diagnostic codes and attach source labels using
/// the relevant AST node IDs.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EffectCheckIssue {
    /// An inferred concrete effect is not covered by the declaration.
    UndeclaredEffect {
        /// The inferred effect that is not permitted.
        actual: EffectInstance,
    },

    /// An inferred effect variable is not present in the declaration.
    ///
    /// Effect-variable substitution must be performed before checking when
    /// the variable is expected to be bound by generic instantiation.
    UndeclaredEffectVariable {
        /// The inferred variable that is not permitted.
        actual: EffectVariable,
    },
}

impl fmt::Display for EffectCheckIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UndeclaredEffect { actual } => {
                write!(f, "inferred effect `{actual}` is not covered by the declared effect contract")
            }
            Self::UndeclaredEffectVariable { actual } => {
                write!(f, "inferred effect variable `{actual}` is not present in the declared effect contract")
            }
        }
    }
}

/// Concrete effects accepted through explicitly enabled open-row widening.
///
/// These effects are not errors, but the caller should preserve this
/// information in semantic facts or diagnostics so that widening is visible
/// to downstream contract, policy, and provenance checks.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EffectWidening {
    effects: Vec<EffectInstance>,
}

impl EffectWidening {
    /// Returns the accepted widened effects in deterministic order.
    #[must_use]
    pub fn effects(&self) -> &[EffectInstance] {
        &self.effects
    }

    /// Returns whether any concrete effects were accepted by widening.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }

    /// Returns the number of concrete effects accepted by widening.
    #[must_use]
    pub fn len(&self) -> usize {
        self.effects.len()
    }
}

/// Result of checking one inferred effect row against one declaration.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EffectCheckReport {
    issues: Vec<EffectCheckIssue>,
    widening: EffectWidening,
}

impl EffectCheckReport {
    /// Returns the compatibility issues.
    #[must_use]
    pub fn issues(&self) -> &[EffectCheckIssue] {
        &self.issues
    }

    /// Returns the accepted widening, if any.
    #[must_use]
    pub fn widening(&self) -> &EffectWidening {
        &self.widening
    }

    /// Returns whether the inferred row satisfies the declared contract.
    #[must_use]
    pub fn is_compatible(&self) -> bool {
        self.issues.is_empty()
    }

    /// Returns whether the check accepted any widening.
    #[must_use]
    pub fn widened(&self) -> bool {
        !self.widening.is_empty()
    }

    /// Returns whether the result contains compatibility issues.
    #[must_use]
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }
}

/// Errors indicating that a row could not be checked reliably.
///
/// A checking error is different from an incompatible effect row. For
/// example, a malformed row created through deserialization must not be
/// converted into an ordinary compatibility issue or treated as an empty
/// row.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EffectCheckingError {
    /// An inferred row is structurally malformed.
    InvalidActualRow(EffectModelError),

    /// A declared row is structurally malformed.
    InvalidDeclaredRow(EffectModelError),
}

impl fmt::Display for EffectCheckingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidActualRow(error) => {
                write!(f, "invalid inferred effect row: {error}")
            }
            Self::InvalidDeclaredRow(error) => {
                write!(f, "invalid declared effect row: {error}")
            }
        }
    }
}

impl Error for EffectCheckingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidActualRow(error) | Self::InvalidDeclaredRow(error) => {
                Some(error)
            }
        }
    }
}

/// Checks an inferred effect row against a declared effect contract.
///
/// This convenience entry point uses exact structural matching. Use
/// [`check_effect_compatibility_with`] when the semantic registry supplies
/// explicitly defined effect relationships.
///
/// # Behavior
///
/// - Closed declarations reject every unmatched inferred concrete effect.
/// - Declared effect variables must match inferred variables by identity.
/// - Open-row widening is disabled by default.
/// - Malformed rows return an error rather than an incompatible report.
///
/// This function does not perform effect-variable substitution, alias
/// resolution, authorization, or target-feasibility analysis.
pub fn check_effect_compatibility(
    actual: &EffectRow,
    declared: &EffectRow,
) -> Result<EffectCheckReport, EffectCheckingError> {
    check_effect_compatibility_with(
        actual,
        declared,
        EffectCheckingPolicy::strict(),
        &ExactEffectRelation,
    )
}

/// Checks effect compatibility using an explicit policy and effect relation.
///
/// The relation provider must be backed by validated semantic information.
/// Do not use namespace-prefix matching as a substitute for registered effect
/// relationships.
///
/// An unmatched concrete effect may be accepted through widening only when
/// both conditions hold:
///
/// 1. The declared row is open.
/// 2. The policy explicitly allows widening for open declared rows.
///
/// Unmatched inferred effect variables are always reported. They must be
/// resolved or proven compatible by the appropriate inference/substitution
/// pass before this checker runs.
pub fn check_effect_compatibility_with<R>(
    actual: &EffectRow,
    declared: &EffectRow,
    policy: EffectCheckingPolicy,
    relation: &R,
) -> Result<EffectCheckReport, EffectCheckingError>
where
    R: EffectRelation + ?Sized,
{
    actual
        .validate()
        .map_err(EffectCheckingError::InvalidActualRow)?;

    declared
        .validate()
        .map_err(EffectCheckingError::InvalidDeclaredRow)?;

    let mut report = EffectCheckReport::default();
    let declared_is_open = !declared.is_closed();
    let may_widen = declared_is_open
        && policy.widening_policy()
            == EffectWideningPolicy::AllowWhenDeclaredRowIsOpen;

    // Concrete effects are covered by exact identity unless the caller
    // supplies a validated, explicit effect-relation provider.
    for actual_effect in actual.effects().iter() {
        let covered = declared
            .effects()
            .iter()
            .any(|permitted| relation.covers(actual_effect, permitted));

        if covered {
            continue;
        }

        if may_widen {
            report
                .widening
                .effects
                .push(actual_effect.clone());
        } else {
            report.issues.push(EffectCheckIssue::UndeclaredEffect {
                actual: actual_effect.clone(),
            });
        }
    }

    // Effect variables represent polymorphic uncertainty. They are not
    // concrete effects and are not made compatible merely by enabling
    // concrete-effect widening.
    for actual_variable in actual.variables() {
        let declared_contains_variable = declared
            .variables()
            .any(|permitted| permitted == actual_variable);

        if !declared_contains_variable {
            report.issues.push(
                EffectCheckIssue::UndeclaredEffectVariable {
                    actual: actual_variable.clone(),
                },
            );
        }
    }

    // Both EffectSet and EffectRow use ordered collections, so the effects
    // and variables are already visited deterministically. Keep explicit
    // sorting here so the public report remains deterministic if the
    // collection implementation changes in the future.
    report.issues.sort_by(|left, right| {
        issue_sort_key(left).cmp(&issue_sort_key(right))
    });
    report.widening.effects.sort();

    Ok(report)
}

fn issue_sort_key(issue: &EffectCheckIssue) -> (u8, String) {
    match issue {
        EffectCheckIssue::UndeclaredEffect { actual } => {
            (0, actual.to_string())
        }
        EffectCheckIssue::UndeclaredEffectVariable { actual } => {
            (1, actual.to_string())
        }
    }
}

/// Validates the structural invariants of a semantic effect row.
///
/// This is useful when a row was deserialized or constructed by an external
/// integration. It does not prove that the row is declared, authorized, or
/// compatible with a function contract.
pub fn validate_effect_row(
    row: &EffectRow,
) -> Result<(), EffectModelError> {
    row.validate()
}

/// Returns whether a canonical effect kind matches another kind exactly.
///
/// This helper intentionally performs no alias lookup or hierarchy inference.
/// Use a registered [`EffectRelation`] for language-defined relationships.
#[must_use]
pub fn same_effect_kind(left: &EffectKind, right: &EffectKind) -> bool {
    left == right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::effects::model::EffectRow;

    fn effect(namespace: &str, name: &str) -> EffectInstance {
        let kind = EffectKind::new(namespace, name)
            .expect("test effect kind must be valid");
        EffectInstance::simple(kind)
    }

    fn row(effects: Vec<EffectInstance>) -> EffectRow {
        EffectRow::closed(effects).expect("test effect row must be valid")
    }

    fn variable(name: &str) -> EffectVariable {
        EffectVariable::new(name).expect("test variable must be valid")
    }

    fn open_row(
        effects: Vec<EffectInstance>,
        variables: Vec<EffectVariable>,
    ) -> EffectRow {
        EffectRow::new(effects, variables)
            .expect("test open effect row must be valid")
    }

    #[test]
    fn empty_actual_row_is_compatible_with_empty_declaration() {
        let report = check_effect_compatibility(
            &EffectRow::empty(),
            &EffectRow::empty(),
        )
        .expect("valid rows must be checked");

        assert!(report.is_compatible());
        assert!(!report.widened());
    }

    #[test]
    fn matching_effect_is_compatible() {
        let io = effect("example", "io");
        let report = check_effect_compatibility(
            &row(vec![io.clone()]),
            &row(vec![io]),
        )
        .expect("valid rows must be checked");

        assert!(report.is_compatible());
    }

    #[test]
    fn undeclared_effect_is_rejected_for_closed_contract() {
        let report = check_effect_compatibility(
            &row(vec![effect("example", "network")]),
            &row(vec![effect("example", "io")]),
        )
        .expect("valid rows must be checked");

        assert!(!report.is_compatible());
        assert_eq!(report.issues().len(), 1);

        assert!(matches!(
            report.issues()[0],
            EffectCheckIssue::UndeclaredEffect { .. }
        ));
    }

    #[test]
    fn empty_closed_declaration_does_not_mean_unknown_effects_are_allowed() {
        let report = check_effect_compatibility(
            &row(vec![effect("custom-domain", "operation")]),
            &EffectRow::empty(),
        )
        .expect("valid rows must be checked");

        assert!(!report.is_compatible());
    }

    #[test]
    fn declaration_may_allow_narrower_implementation() {
        let declared = row(vec![
            effect("example", "read"),
            effect("example", "write"),
        ]);
        let actual = row(vec![effect("example", "read")]);

        let report = check_effect_compatibility(&actual, &declared)
            .expect("valid rows must be checked");

        assert!(report.is_compatible());
    }

    #[test]
    fn widening_is_rejected_by_default_even_for_an_open_row() {
        let actual = row(vec![effect("example", "network")]);
        let declared = open_row(Vec::new(), vec![variable("e")]);

        let report = check_effect_compatibility(&actual, &declared)
            .expect("valid rows must be checked");

        assert!(!report.is_compatible());
        assert!(!report.widened());
    }

    #[test]
    fn explicitly_enabled_widening_is_reported_for_open_rows() {
        let network = effect("example", "network");
        let actual = row(vec![network.clone()]);
        let declared = open_row(Vec::new(), vec![variable("e")]);

        let report = check_effect_compatibility_with(
            &actual,
            &declared,
            EffectCheckingPolicy::allow_widening_for_open_rows(),
            &ExactEffectRelation,
        )
        .expect("valid rows must be checked");

        assert!(report.is_compatible());
        assert!(report.widened());
        assert_eq!(report.widening().effects(), &[network]);
    }

    #[test]
    fn closed_rows_cannot_be_widened_even_when_policy_allows_open_widening() {
        let actual = row(vec![effect("example", "network")]);
        let declared = row(vec![effect("example", "io")]);

        let report = check_effect_compatibility_with(
            &actual,
            &declared,
            EffectCheckingPolicy::allow_widening_for_open_rows(),
            &ExactEffectRelation,
        )
        .expect("valid rows must be checked");

        assert!(!report.is_compatible());
        assert!(!report.widened());
    }

    #[test]
    fn inferred_effect_variable_must_be_declared() {
        let actual = open_row(Vec::new(), vec![variable("actual")]);
        let declared = open_row(Vec::new(), vec![variable("declared")]);

        let report = check_effect_compatibility(&actual, &declared)
            .expect("valid rows must be checked");

        assert!(!report.is_compatible());
        assert!(matches!(
            report.issues()[0],
            EffectCheckIssue::UndeclaredEffectVariable { .. }
        ));
    }

    #[test]
    fn matching_effect_variables_are_compatible() {
        let actual = open_row(Vec::new(), vec![variable("e")]);
        let declared = open_row(Vec::new(), vec![variable("e")]);

        let report = check_effect_compatibility(&actual, &declared)
            .expect("valid rows must be checked");

        assert!(report.is_compatible());
    }

    #[test]
    fn widening_does_not_authorize_unmatched_effect_variables() {
        let actual = open_row(Vec::new(), vec![variable("actual")]);
        let declared = open_row(Vec::new(), vec![variable("declared")]);

        let report = check_effect_compatibility_with(
            &actual,
            &declared,
            EffectCheckingPolicy::allow_widening_for_open_rows(),
            &ExactEffectRelation,
        )
        .expect("valid rows must be checked");

        assert!(!report.is_compatible());
    }

    #[test]
    fn namespaces_do_not_implicitly_define_subtyping() {
        let actual = effect("network", "read");
        let declared = effect("network", "all");

        let report = check_effect_compatibility(
            &row(vec![actual]),
            &row(vec![declared]),
        )
        .expect("valid rows must be checked");

        assert!(!report.is_compatible());
    }

    #[test]
    fn effect_kinds_are_compared_exactly() {
        let left = EffectKind::new("example", "read")
            .expect("valid effect kind");
        let right = EffectKind::new("example", "write")
            .expect("valid effect kind");

        assert!(!same_effect_kind(&left, &right));
        assert!(same_effect_kind(&left, &left));
    }

    #[test]
    fn reports_are_deterministic() {
        let actual = row(vec![
            effect("z-domain", "second"),
            effect("a-domain", "first"),
        ]);
        let declared = EffectRow::empty();

        let first = check_effect_compatibility(&actual, &declared)
            .expect("valid rows must be checked");
        let second = check_effect_compatibility(&actual, &declared)
            .expect("valid rows must be checked");

        assert_eq!(first, second);
    }
}
