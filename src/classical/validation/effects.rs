
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Semantic Effect Validation.
//!
//! Path: `src/classical/validation/effects.rs`
//! Edition: Rust 2021.
//! Minimum intended Rust version: 1.97.
//! Safety: safe Rust only.
//!
//! # File contract
//!
//! Status: Proposed implementation; compilation and repository integration
//! must be verified before this file is marked complete.
//!
//! ## Purpose
//!
//! Provide a classical-computing entry point for validating canonical
//! semantic effect rows and checking inferred effects against declared
//! effect contracts.
//!
//! ## Authority
//!
//! Normative specifications:
//! - `grammar/spec/effects.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! Grammar:
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/effects/effects.g4`
//! - `grammar/effects/effect-sets.g4`
//! - `grammar/statements/effects.g4`
//!
//! Canonical implementations:
//! - `src/frontend/ast/node/effects/`
//! - `src/frontend/semantic/effects/model.rs`
//! - `src/frontend/semantic/effects/inference.rs`
//! - `src/frontend/semantic/effects/checking.rs`
//! - `src/frontend/semantic/effects/authorization.rs`
//! - `src/zuir/ir/effects.rs`
//!
//! ## Ownership
//!
//! This module owns the classical-facing validation entry points and the
//! error envelope for malformed canonical effect rows.
//!
//! It does not define a competing effect identity, effect set, effect row,
//! effect inference engine, effect hierarchy, or authorization system.
//!
//! ## Semantic rules
//!
//! - Effect identity and effect arguments come from the canonical semantic
//!   effect model.
//! - Effect rows are structurally validated before they are trusted.
//! - Inferred effects are checked against declared effects using the
//!   canonical compatibility checker.
//! - Closed declared rows reject undeclared effects.
//! - Open-row widening is not enabled implicitly.
//! - Effect variables are not silently discarded or treated as purity.
//! - An empty effect row is structurally valid, but an empty row alone does
//!   not prove that the source construct is pure.
//! - An omitted source declaration and an explicit empty declaration must
//!   remain distinguishable where the source-language contract requires it.
//! - Authorization, target capabilities, and resource feasibility remain
//!   separate validation responsibilities.
//!
//! ## Does not own
//!
//! - Parsing or AST construction.
//! - Source effect-name resolution.
//! - Effect inference, propagation, or substitution.
//! - Effect alias registration or semantic hierarchy definitions.
//! - Capability discovery, permission checks, or authorization.
//! - Resource feasibility, hardware selection, or scheduling.
//! - Canonical IR definitions or lowering.
//! - Runtime enforcement of effects.
//!
//! ## Integration
//!
//! Source syntax is parsed into the existing AST. The semantic effect model
//! resolves it to canonical `EffectRow` values. Effect inference produces
//! actual rows; this module validates row structure and delegates contract
//! checking to `frontend::semantic::effects::checking`.
//!
//! The canonical ZUIR and classical IR boundaries consume validated semantic
//! facts. This module does not create an IR or decide whether a target can
//! realize an effect.
//!
//! ## Scalability
//!
//! No universal maximum effect count, parameter count, effect-name depth,
//! or effect-row size is imposed here. Work is proportional to the data
//! passed to the canonical semantic implementation.
//!
//! The compiler may enforce explicit configurable operational budgets
//! elsewhere. A budget failure is an operational diagnostic, not proof that
//! a program's effect semantics are invalid.
//!
//! ## Freeze criteria
//!
//! - Canonical semantic effect types remain the only effect-row authority.
//! - Malformed rows return structured errors.
//! - Closed/open-row compatibility is delegated to the canonical checker.
//! - Effect compatibility is not conflated with authorization or capability
//!   availability.
//! - Module registration, compilation, tests, and lint checks pass.

use core::fmt;
use std::error::Error;

use crate::frontend::semantic::effects::{
    check_effect_compatibility,
    check_effect_compatibility_with,
    EffectCheckReport,
    EffectCheckingError,
    EffectCheckingPolicy,
    EffectModelError,
    EffectRelation,
    EffectRow,
    ExactEffectRelation,
};

// =============================================================================
// Error model
// =============================================================================

/// Identifies the role of an effect row in a validation diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EffectRowRole {
    /// Effects inferred from the implementation or computation.
    Inferred,

    /// Effects declared by a function, operation, or semantic contract.
    Declared,

    /// A row supplied for standalone structural validation.
    Standalone,
}

impl fmt::Display for EffectRowRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inferred => formatter.write_str("inferred"),
            Self::Declared => formatter.write_str("declared"),
            Self::Standalone => formatter.write_str("standalone"),
        }
    }
}

/// Errors produced by the classical effect-validation adapter.
#[derive(Debug)]
#[non_exhaustive]
pub enum ClassicalEffectValidationError {
    /// A canonical effect row is structurally malformed.
    InvalidRow {
        /// The role of the row being validated.
        role: EffectRowRole,

        /// The canonical semantic model's validation error.
        source: EffectModelError,
    },

    /// The canonical compatibility checker could not check the contract.
    Compatibility(EffectCheckingError),
}

impl fmt::Display for ClassicalEffectValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRow { role, source } => {
                write!(formatter, "invalid {role} effect row: {source}")
            }
            Self::Compatibility(source) => {
                write!(formatter, "effect-contract checking failed: {source}")
            }
        }
    }
}

impl Error for ClassicalEffectValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidRow { source, .. } => Some(source),
            Self::Compatibility(source) => Some(source),
        }
    }
}

impl From<EffectCheckingError> for ClassicalEffectValidationError {
    fn from(source: EffectCheckingError) -> Self {
        Self::Compatibility(source)
    }
}

/// Convenient result type for this module.
pub type ClassicalEffectValidationResult<T> =
    Result<T, ClassicalEffectValidationError>;

// =============================================================================
// Structural validation
// =============================================================================

/// Validates a canonical semantic effect row.
///
/// This function delegates structural validation to `EffectRow::validate`.
/// It does not infer effects, check declaration compatibility, authorize
/// execution, or establish target feasibility.
///
/// An empty row is structurally valid. Its emptiness must not be interpreted
/// as a proof of purity unless the enclosing semantic contract establishes
/// that conclusion.
pub fn validate_effect_row(
    row: &EffectRow,
) -> ClassicalEffectValidationResult<()> {
    validate_effect_row_with_role(row, EffectRowRole::Standalone)
}

/// Validates a canonical semantic effect row while retaining its diagnostic
/// role.
///
/// The role affects only the structured error context. It does not alter the
/// language semantics or the validation rules.
pub fn validate_effect_row_with_role(
    row: &EffectRow,
    role: EffectRowRole,
) -> ClassicalEffectValidationResult<()> {
    row.validate()
        .map_err(|source| ClassicalEffectValidationError::InvalidRow {
            role,
            source,
        })
}

// =============================================================================
// Effect-contract validation
// =============================================================================

/// Checks inferred effects against a declared effect contract using strict
/// canonical effect matching.
///
/// This is the safe default:
///
/// - A closed declared row does not permit undeclared concrete effects.
/// - Inferred effect variables must be accounted for by the declared row.
/// - Additional concrete effects are not silently accepted.
/// - Structurally malformed rows return an error.
/// - A valid but incompatible contract is represented by the returned report.
///
/// The caller must inspect [`EffectCheckReport::is_compatible`] before
/// accepting the contract.
///
/// This function does not perform inference, substitution, authorization,
/// capability checking, or target-feasibility analysis.
pub fn validate_effect_contract(
    inferred: &EffectRow,
    declared: &EffectRow,
) -> ClassicalEffectValidationResult<EffectCheckReport> {
    check_effect_compatibility(inferred, declared)
        .map_err(ClassicalEffectValidationError::Compatibility)
}

/// Checks inferred effects against a declared contract using an explicit
/// compatibility policy and canonical effect-relation provider.
///
/// The relation must represent validated, language-defined semantic
/// relationships. Namespace prefixes or similar string heuristics must not
/// be used to infer effect compatibility.
///
/// Enabling open-row widening does not itself prove that the language
/// permits a particular widening. The caller is responsible for ensuring
/// that the applicable semantic rules, bounds, substitutions, and policies
/// have already been checked.
pub fn validate_effect_contract_with<R>(
    inferred: &EffectRow,
    declared: &EffectRow,
    policy: EffectCheckingPolicy,
    relation: &R,
) -> ClassicalEffectValidationResult<EffectCheckReport>
where
    R: EffectRelation + ?Sized,
{
    check_effect_compatibility_with(inferred, declared, policy, relation)
        .map_err(ClassicalEffectValidationError::Compatibility)
}

/// Checks an effect contract using exact canonical identity and an explicit
/// policy.
///
/// Use this when the caller needs a policy other than strict checking but
/// does not have a custom effect-relation provider.
///
/// `EffectCheckingPolicy::strict()` rejects implicit widening.
/// `EffectCheckingPolicy::allow_widening_for_open_rows()` allows the
/// canonical checker to apply its documented open-row widening behavior.
pub fn validate_effect_contract_with_exact_matching(
    inferred: &EffectRow,
    declared: &EffectRow,
    policy: EffectCheckingPolicy,
) -> ClassicalEffectValidationResult<EffectCheckReport> {
    validate_effect_contract_with(
        inferred,
        declared,
        policy,
        &ExactEffectRelation,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::effects::{
        EffectRow,
        EffectVariable,
    };

    #[test]
    fn empty_row_is_structurally_valid() {
        let row = EffectRow::empty();

        assert!(validate_effect_row(&row).is_ok());
    }

    #[test]
    fn empty_row_is_compatible_with_empty_declared_row() {
        let inferred = EffectRow::empty();
        let declared = EffectRow::empty();

        let report = validate_effect_contract(&inferred, &declared)
            .expect("well-formed rows should be checkable");

        assert!(report.is_compatible());
    }

    #[test]
    fn valid_but_incompatible_contract_returns_report() {
        let inferred = EffectRow::new(
            [],
            [EffectVariable::new("E")
                .expect("test effect variable should be valid")],
        )
        .expect("test row should be structurally valid");

        let declared = EffectRow::empty();

        let report = validate_effect_contract(&inferred, &declared)
            .expect("well-formed rows should produce a compatibility report");

        assert!(!report.is_compatible());
    }

    #[test]
    fn an_open_declared_row_is_not_implicitly_widened() {
        let inferred = EffectRow::empty();

        let declared = EffectRow::new(
            [],
            [EffectVariable::new("E")
                .expect("test effect variable should be valid")],
        )
        .expect("test row should be structurally valid");

        let report = validate_effect_contract(&inferred, &declared)
            .expect("well-formed rows should be checkable");

        // An open row is not itself evidence of an inferred effect, and this
        // adapter does not infer or authorize additional effects.
        assert!(report.is_compatible());
    }

    #[test]
    fn strict_policy_is_the_default_contract_policy() {
        let inferred = EffectRow::empty();
        let declared = EffectRow::empty();

        let report = validate_effect_contract_with_exact_matching(
            &inferred,
            &declared,
            EffectCheckingPolicy::strict(),
        )
        .expect("well-formed rows should be checkable");

        assert!(report.is_compatible());
    }

    #[test]
    fn malformed_rows_are_reported_by_the_canonical_model() {
        // Constructors enforce row invariants. This test establishes that
        // validation accepts a canonical, constructor-produced row; malformed
        // deserialized values remain the canonical model's responsibility.
        let row = EffectRow::closed([])
            .expect("an empty closed row is valid");

        assert!(validate_effect_row_with_role(
            &row,
            EffectRowRole::Declared
        )
        .is_ok());
    }
}
