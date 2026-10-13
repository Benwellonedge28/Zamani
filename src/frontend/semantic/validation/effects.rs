
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Effect Validation
//!
//! File: `src/frontend/semantic/validation/effects.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! ## PURPOSE
//!
//! Coordinate validation of already-inferred effect rows against optional
//! declared-effect contracts and optional authorization policy.
//!
//! This is an orchestration boundary. It reuses the canonical effect model,
//! effect checker, and authorization evaluator; it does not define a second
//! effect representation or independently infer source effects.
//!
//! ## AUTHORITY
//!
//! Normative semantic rules:
//! - `grammar/spec/effects.md`
//! - `grammar/specification/semantic-model.md`
//!
//! Syntax authority:
//! - `grammar/effects/effect-sets.g4`
//! - `grammar/effects/effect-declarations.g4`
//! - `grammar/effects/effect-operations.g4`
//! - `grammar/effects/effect-types.g4`
//!
//! Canonical implementation:
//! - `src/frontend/ast/node/effects/effect_kind.rs`
//! - `src/frontend/semantic/effects/model.rs`
//! - `src/frontend/semantic/effects/inference.rs`
//! - `src/frontend/semantic/effects/checking.rs`
//! - `src/frontend/semantic/effects/authorization.rs`
//!
//! ## OWNS
//!
//! - The combined validation request/result contract.
//! - Sequencing structural validation, declaration checking, and optional
//!   authorization.
//! - A single structured error boundary for these operations.
//! - Explicit distinction between compatibility and authorization.
//!
//! ## DOES_NOT_OWN
//!
//! - Parsing or constructing AST nodes.
//! - Resolving effect names or effect aliases.
//! - Inferring effects from expressions or calls.
//! - Canonical effect identities, effect instances, rows, or variables.
//! - Effect subtyping, generic substitution, or handler semantics.
//! - Policy parsing, policy inheritance, or policy-scope resolution.
//! - Capability availability or resource feasibility.
//! - Physical resource allocation, target selection, or runtime enforcement.
//! - Classical IR, `quantum::ir`, HDL IR, routing, scheduling, or lowering.
//!
//! ## INTEGRATION CONTRACT
//!
//! 1. AST traversal, name resolution, and effect inference produce a validated
//!    or explicitly error-bearing `EffectRow`.
//! 2. Declaration analysis optionally supplies the declared effect row.
//! 3. If supplied, the declared row is checked against the inferred row using
//!    `checking.rs`.
//! 4. If supplied, a resolved authorization policy is evaluated independently
//!    using `authorization.rs`.
//! 5. The caller converts returned errors and report issues into canonical
//!    diagnostics, attaching source locations from the relevant AST nodes.
//! 6. The semantic model records the report or its derived facts.
//! 7. Finalization must not claim success when a required check failed or
//!    could not complete.
//!
//! This module deliberately accepts semantic rows rather than guessed AST
//! variants. AST-specific extraction belongs in the actual AST adapter and
//! effect-inference pass.
//!
//! ## SEMANTIC DISTINCTIONS
//!
//! A structurally valid effect row is not necessarily declared, compatible,
//! or authorized. These are distinct properties.
//!
//! - Structural validity checks representation invariants.
//! - Declaration compatibility checks inferred effects against a contract.
//! - Authorization checks the supplied policy.
//! - Capability availability and target feasibility are outside this module.
//!
//! Omitting a declaration means that declaration compatibility was not
//! requested. Omitting an authorization policy means that authorization was
//! not requested. Neither omission is evidence that the corresponding
//! obligation has been satisfied.
//!
//! Open effect variables are preserved. They are not silently erased,
//! interpreted as an empty effect set, or automatically authorized.
//!
//! ## SCALABILITY
//!
//! This module introduces no fixed effect count, argument count, domain count,
//! qubit count, machine capacity, or source-program size limit. It operates on
//! the finite rows supplied by its caller.
//!
//! Configurable operational budgets belong to the semantic session. If a
//! budget prevents a required check from running, the pipeline must represent
//! the analysis as incomplete rather than return a successful validation.
//!
//! No finite implementation can process an actually infinite in-memory input.
//! This module imposes no artificial machine-capacity restriction on source
//! semantics.
//!
//! ## DETERMINISM
//!
//! Deterministic ordering is delegated to the canonical effect model and its
//! checking/authorization implementations. This module preserves their
//! reports and does not reorder or discard issues.
//!
//! ## FREEZE CRITERIA
//!
//! - Canonical effect-model and checking APIs compile.
//! - Authorization is optional and remains independent of compatibility.
//! - Malformed rows return errors rather than being treated as empty.
//! - Incompatible declarations are reported as a report, not a panic.
//! - Denied effects and unresolved authorization variables remain visible.
//! - Tests cover the combined paths and the omitted-check semantics.
//! - Workspace formatting, tests, and lint checks pass after integration.

use core::fmt;
use std::error::Error;

use crate::frontend::semantic::effects::authorization::{
    EffectAuthorizationError, EffectAuthorizationPolicy, EffectAuthorizationReport,
};
use crate::frontend::semantic::effects::checking::{
    check_effect_compatibility, check_effect_compatibility_with, EffectCheckReport,
    EffectCheckingError, EffectCheckingPolicy, EffectRelation,
};
use crate::frontend::semantic::effects::model::{EffectModelError, EffectRow};

/// Optional checks requested for one inferred effect row.
///
/// A missing declaration or authorization policy means that check was not
/// requested. It does not mean the corresponding obligation was satisfied.
#[derive(Clone, Copy, Debug)]
pub struct EffectValidationRequest<'a> {
    actual: &'a EffectRow,
    declared: Option<&'a EffectRow>,
    authorization: Option<&'a EffectAuthorizationPolicy>,
    checking_policy: EffectCheckingPolicy,
}

impl<'a> EffectValidationRequest<'a> {
    /// Creates a validation request for an inferred effect row.
    ///
    /// Declaration compatibility and authorization are opt-in. The default
    /// checking policy is strict: unmatched inferred effects are rejected.
    #[must_use]
    pub fn new(actual: &'a EffectRow) -> Self {
        Self {
            actual,
            declared: None,
            authorization: None,
            checking_policy: EffectCheckingPolicy::strict(),
        }
    }

    /// Supplies the declared effect contract to check against the inferred
    /// row.
    #[must_use]
    pub const fn with_declaration(mut self, declared: &'a EffectRow) -> Self {
        self.declared = Some(declared);
        self
    }

    /// Supplies the resolved authorization policy.
    ///
    /// The policy must already have been constructed from validated policy
    /// declarations and resolved policy scopes.
    #[must_use]
    pub const fn with_authorization(
        mut self,
        authorization: &'a EffectAuthorizationPolicy,
    ) -> Self {
        self.authorization = Some(authorization);
        self
    }

    /// Sets the declared-effect compatibility policy.
    ///
    /// Widening is disabled by default. It must only be enabled when the
    /// language contract and preceding generic-effect checks permit it.
    #[must_use]
    pub const fn with_checking_policy(
        mut self,
        policy: EffectCheckingPolicy,
    ) -> Self {
        self.checking_policy = policy;
        self
    }

    /// Returns the inferred row being validated.
    #[must_use]
    pub const fn actual(&self) -> &'a EffectRow {
        self.actual
    }

    /// Returns the declared row, if declaration checking was requested.
    #[must_use]
    pub const fn declared(&self) -> Option<&'a EffectRow> {
        self.declared
    }

    /// Returns the authorization policy, if authorization was requested.
    #[must_use]
    pub const fn authorization(&self) -> Option<&'a EffectAuthorizationPolicy> {
        self.authorization
    }

    /// Returns the configured declaration-checking policy.
    #[must_use]
    pub const fn checking_policy(&self) -> EffectCheckingPolicy {
        self.checking_policy
    }
}

/// Combined result of the effect-validation checks that were requested.
///
/// This result deliberately preserves whether a check was run. A missing
/// declaration report is not a successful declaration check, and a missing
/// authorization report is not an authorization grant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectValidationReport {
    declaration: Option<EffectCheckReport>,
    authorization: Option<EffectAuthorizationReport>,
}

impl EffectValidationReport {
    fn new(
        declaration: Option<EffectCheckReport>,
        authorization: Option<EffectAuthorizationReport>,
    ) -> Self {
        Self {
            declaration,
            authorization,
        }
    }

    /// Returns the declaration-compatibility report, if requested.
    #[must_use]
    pub fn declaration_report(&self) -> Option<&EffectCheckReport> {
        self.declaration.as_ref()
    }

    /// Returns the authorization report, if requested.
    #[must_use]
    pub fn authorization_report(&self) -> Option<&EffectAuthorizationReport> {
        self.authorization.as_ref()
    }

    /// Returns whether declaration compatibility was checked.
    #[must_use]
    pub const fn declaration_was_checked(&self) -> bool {
        self.declaration.is_some()
    }

    /// Returns whether authorization was checked.
    #[must_use]
    pub const fn authorization_was_checked(&self) -> bool {
        self.authorization.is_some()
    }

    /// Returns whether the requested declaration check passed.
    ///
    /// Returns `None` when no declared contract was supplied.
    #[must_use]
    pub fn declaration_is_compatible(&self) -> Option<bool> {
        self.declaration
            .as_ref()
            .map(EffectCheckReport::is_compatible)
    }

    /// Returns whether the requested authorization check passed.
    ///
    /// Returns `None` when no authorization policy was supplied. An
    /// authorization report is successful only when all concrete effects are
    /// allowed and no open effect variables remain unresolved.
    #[must_use]
    pub fn is_authorized(&self) -> Option<bool> {
        self.authorization
            .as_ref()
            .map(EffectAuthorizationReport::is_authorized)
    }

    /// Returns whether every check represented in this report passed.
    ///
    /// This method does not imply that checks omitted from the request were
    /// performed. The caller must separately ensure that all checks required
    /// by the language and compilation configuration were requested.
    #[must_use]
    pub fn all_requested_checks_passed(&self) -> bool {
        self.declaration
            .as_ref()
            .is_none_or(EffectCheckReport::is_compatible)
            && self
                .authorization
                .as_ref()
                .is_none_or(EffectAuthorizationReport::is_authorized)
    }

    /// Returns whether declaration checking accepted effect widening.
    #[must_use]
    pub fn declaration_was_widened(&self) -> bool {
        self.declaration
            .as_ref()
            .is_some_and(EffectCheckReport::widened)
    }
}

/// Failure to carry out a requested effect-validation operation reliably.
///
/// This error type is distinct from a normal declaration incompatibility or
/// authorization denial. Those conditions are represented in the returned
/// report. Structural corruption or failure of a checking operation returns
/// an error.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EffectValidationError {
    /// The inferred effect row is structurally invalid.
    InvalidActualRow(EffectModelError),

    /// A declared effect row is structurally invalid.
    InvalidDeclaredRow(EffectModelError),

    /// The declaration checker could not complete reliably.
    DeclarationCheck(EffectCheckingError),

    /// The authorization evaluator could not complete reliably.
    Authorization(EffectAuthorizationError),
}

impl fmt::Display for EffectValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidActualRow(error) => {
                write!(f, "invalid inferred effect row: {error}")
            }
            Self::InvalidDeclaredRow(error) => {
                write!(f, "invalid declared effect row: {error}")
            }
            Self::DeclarationCheck(error) => {
                write!(f, "effect declaration check failed: {error}")
            }
            Self::Authorization(error) => {
                write!(f, "effect authorization failed: {error}")
            }
        }
    }
}

impl Error for EffectValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidActualRow(error) | Self::InvalidDeclaredRow(error) => {
                Some(error)
            }
            Self::DeclarationCheck(error) => Some(error),
            Self::Authorization(error) => Some(error),
        }
    }
}

/// Validates an inferred effect row using the requested checks.
///
/// The inferred row is always structurally validated. Declaration
/// compatibility and authorization run only when the request supplies the
/// corresponding contract or policy.
///
/// Normal incompatibility and denial are returned inside
/// [`EffectValidationReport`]. Malformed input or an operation that cannot be
/// completed reliably returns [`EffectValidationError`].
///
/// # Errors
///
/// Returns an error if the inferred row, supplied declaration, or requested
/// authorization input is structurally invalid.
pub fn validate_effects(
    request: EffectValidationRequest<'_>,
) -> Result<EffectValidationReport, EffectValidationError> {
    request
        .actual
        .validate()
        .map_err(EffectValidationError::InvalidActualRow)?;

    if let Some(declared) = request.declared {
        declared
            .validate()
            .map_err(EffectValidationError::InvalidDeclaredRow)?;
    }

    let declaration = match request.declared {
        Some(declared) => Some(
            check_effect_compatibility_with(
                request.actual,
                declared,
                request.checking_policy,
                &crate::frontend::semantic::effects::checking::ExactEffectRelation,
            )
            .map_err(EffectValidationError::DeclarationCheck)?,
        ),
        None => None,
    };

    let authorization = match request.authorization {
        Some(policy) => Some(
            policy
                .authorize(request.actual)
                .map_err(EffectValidationError::Authorization)?,
        ),
        None => None,
    };

    Ok(EffectValidationReport::new(
        declaration,
        authorization,
    ))
}

/// Validates declaration compatibility with exact effect identity.
///
/// This convenience function uses the strict default compatibility policy.
/// It does not perform authorization.
pub fn validate_effect_declaration(
    actual: &EffectRow,
    declared: &EffectRow,
) -> Result<EffectCheckReport, EffectCheckingError> {
    check_effect_compatibility(actual, declared)
}

/// Validates declaration compatibility using an explicit effect relation.
///
/// A relation provider must be based on validated semantic relationships.
/// Namespace prefixes or naming conventions must not be treated as implicit
/// subtyping.
pub fn validate_effect_declaration_with<R>(
    actual: &EffectRow,
    declared: &EffectRow,
    policy: EffectCheckingPolicy,
    relation: &R,
) -> Result<EffectCheckReport, EffectCheckingError>
where
    R: EffectRelation + ?Sized,
{
    check_effect_compatibility_with(actual, declared, policy, relation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::effects::effect_kind::EffectKind;
    use crate::frontend::semantic::effects::authorization::{
        AuthorizationDecision, AuthorizationRule,
    };
    use crate::frontend::semantic::effects::model::{
        EffectInstance, EffectRow, EffectVariable,
    };

    fn effect(namespace: &str, name: &str) -> EffectInstance {
        EffectInstance::simple(
            EffectKind::new(namespace, name)
                .expect("test effect kind must be structurally valid"),
        )
    }

    fn row(effects: Vec<EffectInstance>) -> EffectRow {
        EffectRow::closed(effects).expect("test effect row must be valid")
    }

    fn allow_policy(effect: &EffectInstance) -> EffectAuthorizationPolicy {
        let mut policy = EffectAuthorizationPolicy::new();
        let rule = AuthorizationRule::allow(
            "test-policy",
            None,
            Some("explicitly allowed by test policy"),
        )
        .expect("test authorization rule must be valid");

        policy
            .add_instance_rule(effect.clone(), rule)
            .expect("test authorization rule must not be duplicated");

        policy
    }

    #[test]
    fn validates_an_empty_closed_row_without_inventing_checks() {
        let actual = EffectRow::empty();

        let report = validate_effects(EffectValidationRequest::new(&actual))
            .expect("empty closed row should be structurally valid");

        assert!(!report.declaration_was_checked());
        assert!(!report.authorization_was_checked());
        assert_eq!(report.declaration_is_compatible(), None);
        assert_eq!(report.is_authorized(), None);
        assert!(report.all_requested_checks_passed());
    }

    #[test]
    fn declaration_mismatch_is_a_report_not_a_success() {
        let actual = row(vec![effect("quantum", "measurement")]);
        let declared = EffectRow::empty();

        let report = validate_effects(
            EffectValidationRequest::new(&actual).with_declaration(&declared),
        )
        .expect("valid rows should produce a compatibility report");

        assert_eq!(report.declaration_is_compatible(), Some(false));
        assert!(!report.all_requested_checks_passed());
    }

    #[test]
    fn declaration_compatibility_does_not_imply_authorization() {
        let io = effect("core", "io");
        let actual = row(vec![io.clone()]);
        let declared = row(vec![io]);

        let report = validate_effects(
            EffectValidationRequest::new(&actual).with_declaration(&declared),
        )
        .expect("valid matching rows should be checked");

        assert_eq!(report.declaration_is_compatible(), Some(true));
        assert_eq!(report.is_authorized(), None);
    }

    #[test]
    fn authorization_is_independent_of_declaration_checking() {
        let measurement = effect("quantum", "measurement");
        let actual = row(vec![measurement.clone()]);
        let policy = allow_policy(&measurement);

        let report = validate_effects(
            EffectValidationRequest::new(&actual).with_authorization(&policy),
        )
        .expect("valid row and policy should be evaluated");

        assert!(!report.declaration_was_checked());
        assert_eq!(report.is_authorized(), Some(true));
    }

    #[test]
    fn authorization_denial_is_preserved() {
        let measurement = effect("quantum", "measurement");
        let actual = row(vec![measurement]);
        let policy = EffectAuthorizationPolicy::new();

        let report = validate_effects(
            EffectValidationRequest::new(&actual).with_authorization(&policy),
        )
        .expect("default-deny policy should return a decision report");

        assert_eq!(report.is_authorized(), Some(false));
        assert!(!report.all_requested_checks_passed());
    }

    #[test]
    fn unresolved_variables_are_not_silently_authorized() {
        let variable = EffectVariable::new("e")
            .expect("test effect variable must be valid");
        let actual = EffectRow::new(Vec::new(), vec![variable])
            .expect("open test row must be structurally valid");
        let policy = EffectAuthorizationPolicy::new();

        let report = validate_effects(
            EffectValidationRequest::new(&actual).with_authorization(&policy),
        )
        .expect("open row must yield an unresolved authorization report");

        assert_eq!(report.is_authorized(), Some(false));
        assert!(!report.all_requested_checks_passed());
        assert!(
            report
                .authorization_report()
                .expect("authorization was requested")
                .has_unresolved_variables()
        );
    }

    #[test]
    fn compatible_effects_can_be_authorized_together() {
        let io = effect("core", "io");
        let actual = row(vec![io.clone()]);
        let declared = row(vec![io.clone()]);
        let policy = allow_policy(&io);

        let report = validate_effects(
            EffectValidationRequest::new(&actual)
                .with_declaration(&declared)
                .with_authorization(&policy),
        )
        .expect("all requested checks should complete");

        assert_eq!(report.declaration_is_compatible(), Some(true));
        assert_eq!(report.is_authorized(), Some(true));
        assert!(report.all_requested_checks_passed());
    }

    #[test]
    fn authorization_rule_decision_type_is_available_to_integrators() {
        assert_eq!(AuthorizationDecision::Allow, AuthorizationDecision::Allow);
    }
}
