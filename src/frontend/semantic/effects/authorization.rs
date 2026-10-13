
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Effect Authorization
//!
//! File: `src/frontend/semantic/effects/authorization.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! ## PURPOSE
//!
//! Authorize inferred semantic effects against an explicit, deterministic
//! policy. An effect is permitted only when an applicable allow rule exists
//! and no applicable deny rule prohibits it.
//!
//! ## AUTHORITY
//!
//! - `grammar/spec/effects.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/effects/effect-sets.g4`
//! - `grammar/effects/effect-composition.g4`
//! - `grammar/effects/effect-declarations.g4`
//! - `grammar/effects/effect-operations.g4`
//! - `grammar/policies/policy.g4`
//! - `grammar/policies/permissions.g4`
//! - `grammar/policies/prohibitions.g4`
//! - `grammar/policies/scopes.g4`
//! - `src/frontend/ast/node/effects/effect_kind.rs`
//! - `src/frontend/semantic/effects/model.rs`
//! - `src/frontend/semantic/effects/inference.rs`
//! - `src/frontend/semantic/effects/checking.rs`
//!
//! Grammar defines source syntax. The effect specification defines language
//! semantics. Policy parsing, policy declaration resolution, and policy-scope
//! construction remain responsibilities of their respective frontend passes.
//!
//! ## OWNS
//!
//! - Immutable-after-construction policy data as observed during evaluation.
//! - Exact-instance and effect-kind authorization rules.
//! - Deterministic rule precedence.
//! - Structured authorization decisions and reports.
//! - Fail-closed handling of effects without an applicable allow rule.
//! - Explicit reporting of unresolved effect-row variables.
//!
//! ## DOES NOT OWN
//!
//! - Parsing, AST construction, or name resolution.
//! - Effect identity, effect instances, effect rows, or effect inference.
//! - Declared-effect compatibility, effect subtyping, or generic substitution.
//! - Policy-language parsing, policy inheritance, or policy-scope resolution.
//! - Capability evidence, resource requirements, or target feasibility.
//! - Runtime identity, device access, or execution-time enforcement.
//! - Effect-handler semantics or effect discharge.
//! - Classical IR, quantum IR, HDL IR, routing, scheduling, or lowering.
//!
//! ## SECURITY CONTRACT
//!
//! - Default behavior is deny.
//! - A valid effect identity is not proof of authorization.
//! - A successfully inferred effect is not proof of authorization.
//! - An unknown or unconfigured effect is denied by default.
//! - A deny rule for an effect kind or exact instance overrides any applicable
//!   allow rule, including a more specific allow rule.
//! - Exact-instance rules are more specific than kind rules when no deny
//!   rule applies.
//! - Open effect-row variables are unresolved obligations, not effects that
//!   may be silently ignored or treated as authorized.
//! - Malformed effect rows produce an error, not a successful authorization
//!   report.
//! - Authorization does not establish capability availability or resource
//!   feasibility.
//!
//! This module implements a small deterministic policy evaluator, not the
//! complete policy language. The policy-resolution pass must translate
//! validated policy declarations and scopes into the explicit rules consumed
//! here. Policy provenance is retained on each rule through its policy ID,
//! optional revision, and rationale.
//!
//! ## SCALABILITY
//!
//! There is no fixed catalogue of effect kinds, domains, devices, operations,
//! or effect-row sizes. Ordered collections grow according to available
//! resources. The analyzer must enforce any configured operational budgets
//! outside this module and report incomplete analysis if those budgets prevent
//! required authorization checks from finishing.
//!
//! No finite implementation can guarantee processing an actually infinite
//! input. This module imposes no artificial machine-capacity restriction on
//! the meaning of a source program.
//!
//! ## DETERMINISM
//!
//! Rules are stored in ordered maps and effect rows are traversed in their
//! canonical order. The same row and policy contents produce the same report.
//! Policy IDs, revisions, and rationales are metadata, not rule precedence.
//!
//! ## INTEGRATION CONTRACT
//!
//! 1. `inference.rs` produces the inferred `EffectRow` for a semantic node.
//! 2. `checking.rs` separately checks inferred effects against declarations.
//! 3. The policy pass resolves policy declarations and scopes.
//! 4. That pass constructs authorization rules from the resolved policy.
//! 5. This module evaluates the inferred row against those rules.
//! 6. The caller converts report records into canonical semantic diagnostics,
//!    preserving source locations from the relevant AST nodes.
//! 7. Capability and resource passes evaluate their own obligations.
//! 8. The semantic model stores the report or derived authorization facts.
//! 9. Finalization must not certify a result as valid while required
//!    authorization obligations remain denied or unresolved.
//!
//! The module intentionally does not assume that the future semantic
//! diagnostics, policy registry, or semantic-facts APIs have been completed.
//!
//! ## FREEZE CRITERIA
//!
//! - The canonical effect-model API compiles with this module.
//! - Missing allow rules fail closed.
//! - Deny rules override allow rules.
//! - Exact-instance and kind-level rules have deterministic precedence.
//! - Unresolved effect variables are reported.
//! - Malformed deserialized rows return errors.
//! - Policy provenance is preserved in decisions.
//! - Quantum, classical, HDL, and future effect names use the same mechanism.
//! - Unit, integration, determinism, and workspace tests pass.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::frontend::ast::node::effects::effect_kind::EffectKind;

use super::model::{EffectInstance, EffectModelError, EffectRow, EffectVariable};

// =============================================================================
// Policy rules
// =============================================================================

/// Whether an explicit authorization rule permits or prohibits an effect.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AuthorizationDecision {
    /// The rule permits the matching effect, subject to deny overrides.
    Allow,

    /// The rule prohibits the matching effect.
    Deny,
}

impl fmt::Display for AuthorizationDecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allow => f.write_str("allow"),
            Self::Deny => f.write_str("deny"),
        }
    }
}

/// Provenance attached to an explicit authorization rule.
///
/// `policy_id` must be a stable identifier supplied by the policy-resolution
/// layer. `revision` should identify the policy version or immutable snapshot
/// when the policy system supports revisions. `rationale` is explanatory
/// metadata; it never changes authorization behavior.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizationRule {
    decision: AuthorizationDecision,
    policy_id: String,
    revision: Option<String>,
    rationale: Option<String>,
}

impl AuthorizationRule {
    /// Creates an authorization rule with optional provenance metadata.
    ///
    /// # Errors
    ///
    /// Returns [`AuthorizationPolicyError::InvalidRuleField`] when the policy
    /// ID, revision, or rationale is empty, contains NUL, or contains control
    /// characters.
    pub fn new(
        decision: AuthorizationDecision,
        policy_id: impl Into<String>,
        revision: Option<String>,
        rationale: Option<String>,
    ) -> Result<Self, AuthorizationPolicyError> {
        let policy_id = policy_id.into();
        validate_rule_text("policy ID", &policy_id)?;

        if let Some(value) = revision.as_deref() {
            validate_rule_text("policy revision", value)?;
        }

        if let Some(value) = rationale.as_deref() {
            validate_rule_text("policy rationale", value)?;
        }

        Ok(Self {
            decision,
            policy_id,
            revision,
            rationale,
        })
    }

    /// Creates an allow rule.
    pub fn allow(
        policy_id: impl Into<String>,
    ) -> Result<Self, AuthorizationPolicyError> {
        Self::new(AuthorizationDecision::Allow, policy_id, None, None)
    }

    /// Creates a deny rule.
    pub fn deny(
        policy_id: impl Into<String>,
    ) -> Result<Self, AuthorizationPolicyError> {
        Self::new(AuthorizationDecision::Deny, policy_id, None, None)
    }

    /// Returns the rule's decision.
    #[must_use]
    pub const fn decision(&self) -> AuthorizationDecision {
        self.decision
    }

    /// Returns the stable identifier of the originating policy.
    #[must_use]
    pub fn policy_id(&self) -> &str {
        &self.policy_id
    }

    /// Returns the originating policy revision, if supplied.
    #[must_use]
    pub fn revision(&self) -> Option<&str> {
        self.revision.as_deref()
    }

    /// Returns the explanatory rationale, if supplied.
    #[must_use]
    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }
}

fn validate_rule_text(
    field: &'static str,
    value: &str,
) -> Result<(), AuthorizationPolicyError> {
    if value.trim().is_empty()
        || value.contains('\0')
        || value.chars().any(char::is_control)
    {
        return Err(AuthorizationPolicyError::InvalidRuleField { field });
    }

    Ok(())
}

// =============================================================================
// Policy representation
// =============================================================================

/// An explicit policy for authorizing semantic effects.
///
/// The policy supports two matching levels:
///
/// - effect-kind rules match every instance of a canonical `EffectKind`;
/// - exact-instance rules match one fully parameterized `EffectInstance`.
///
/// The policy has no implicit allow-all mode. An effect with no matching allow
/// rule is denied. A deny at either matching level overrides an allow at
/// either level.
///
/// Construct the policy from already-resolved policy declarations. The
/// policy parser and resolver remain responsible for source-level scopes,
/// inheritance, imports, policy conflicts, and applicable principals.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EffectAuthorizationPolicy {
    kind_rules: BTreeMap<EffectKind, AuthorizationRule>,
    instance_rules: BTreeMap<EffectInstance, AuthorizationRule>,
}

impl EffectAuthorizationPolicy {
    /// Creates an empty, default-deny authorization policy.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a rule matching every instance of an effect kind.
    ///
    /// The canonical kind is validated before insertion. A second rule for
    /// the same kind is rejected instead of silently replacing policy.
    ///
    /// # Errors
    ///
    /// Returns an error for a malformed kind or duplicate rule.
    pub fn add_kind_rule(
        &mut self,
        kind: EffectKind,
        rule: AuthorizationRule,
    ) -> Result<(), AuthorizationPolicyError> {
        EffectKind::new(kind.namespace(), kind.name()).map_err(|error| {
            AuthorizationPolicyError::InvalidEffectKind {
                message: error.to_string(),
            }
        })?;

        if self.kind_rules.contains_key(&kind) {
            return Err(AuthorizationPolicyError::DuplicateKindRule {
                effect_kind: kind.qualified_name(),
            });
        }

        self.kind_rules.insert(kind, rule);
        Ok(())
    }

    /// Adds a rule matching one exact effect instance.
    ///
    /// Exact matching includes the effect's canonical identity and all
    /// resolved arguments. It does not perform subtyping, wildcard matching,
    /// or implicit argument conversion.
    ///
    /// # Errors
    ///
    /// Returns an error for a malformed effect instance or duplicate rule.
    pub fn add_instance_rule(
        &mut self,
        effect: EffectInstance,
        rule: AuthorizationRule,
    ) -> Result<(), AuthorizationPolicyError> {
        effect.validate().map_err(|error| {
            AuthorizationPolicyError::InvalidEffect {
                message: error.to_string(),
            }
        })?;

        if self.instance_rules.contains_key(&effect) {
            return Err(AuthorizationPolicyError::DuplicateInstanceRule {
                effect: effect.to_string(),
            });
        }

        self.instance_rules.insert(effect, rule);
        Ok(())
    }

    /// Returns the number of kind-level rules.
    #[must_use]
    pub fn kind_rule_count(&self) -> usize {
        self.kind_rules.len()
    }

    /// Returns the number of exact-instance rules.
    #[must_use]
    pub fn instance_rule_count(&self) -> usize {
        self.instance_rules.len()
    }

    /// Returns whether the policy has no explicit rules.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.kind_rules.is_empty() && self.instance_rules.is_empty()
    }

    /// Evaluates one inferred effect row against this policy.
    ///
    /// Every concrete effect is evaluated independently. Any open effect
    /// variables are retained as unresolved authorization obligations.
    /// Consequently, a report containing unresolved variables cannot be
    /// considered fully authorized.
    ///
    /// This operation is deterministic and does not perform I/O, target
    /// discovery, capability discovery, or runtime enforcement.
    ///
    /// # Errors
    ///
    /// Returns [`EffectAuthorizationError::InvalidEffectRow`] if the row is
    /// structurally malformed, including when a deserialized value bypassed
    /// the model constructors.
    pub fn authorize(
        &self,
        row: &EffectRow,
    ) -> Result<EffectAuthorizationReport, EffectAuthorizationError> {
        row.validate()
            .map_err(EffectAuthorizationError::InvalidEffectRow)?;

        let mut decisions = Vec::new();

        for effect in row.effects().iter() {
            decisions.push(self.authorize_instance(effect));
        }

        let unresolved_variables = row.variables().cloned().collect();

        Ok(EffectAuthorizationReport {
            decisions,
            unresolved_variables,
        })
    }

    /// Evaluates one exact effect instance.
    ///
    /// Precedence:
    ///
    /// 1. An exact-instance deny rule.
    /// 2. An effect-kind deny rule.
    /// 3. An exact-instance allow rule.
    /// 4. An effect-kind allow rule.
    /// 5. Default deny when no allow rule applies.
    ///
    /// Deny-overrides is intentional: a more specific allow cannot override a
    /// prohibition inherited from a broader policy rule.
    #[must_use]
    pub fn authorize_instance(
        &self,
        effect: &EffectInstance,
    ) -> EffectAuthorizationRecord {
        let instance_rule = self.instance_rules.get(effect);
        let kind_rule = self.kind_rules.get(effect.kind());

        let selected = instance_rule
            .filter(|rule| rule.decision() == AuthorizationDecision::Deny)
            .or_else(|| {
                kind_rule.filter(|rule| {
                    rule.decision() == AuthorizationDecision::Deny
                })
            })
            .or_else(|| {
                instance_rule.filter(|rule| {
                    rule.decision() == AuthorizationDecision::Allow
                })
            })
            .or_else(|| {
                kind_rule.filter(|rule| {
                    rule.decision() == AuthorizationDecision::Allow
                })
            });

        match selected {
            Some(rule) => EffectAuthorizationRecord {
                effect: effect.clone(),
                decision: rule.decision(),
                source: if instance_rule.is_some_and(|candidate| {
                    core::ptr::eq(candidate, rule)
                }) {
                    AuthorizationDecisionSource::ExactInstanceRule
                } else {
                    AuthorizationDecisionSource::EffectKindRule
                },
                policy_id: Some(rule.policy_id().to_owned()),
                policy_revision: rule.revision().map(str::to_owned),
                rationale: rule.rationale().map(str::to_owned),
            },
            None => EffectAuthorizationRecord {
                effect: effect.clone(),
                decision: AuthorizationDecision::Deny,
                source: AuthorizationDecisionSource::DefaultDeny,
                policy_id: None,
                policy_revision: None,
                rationale: Some(
                    "no applicable explicit allow rule exists".to_owned(),
                ),
            },
        }
    }
}

// =============================================================================
// Decisions and reports
// =============================================================================

/// Identifies the rule source used to make an authorization decision.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AuthorizationDecisionSource {
    /// A rule matching the complete effect instance was selected.
    ExactInstanceRule,

    /// A rule matching the effect kind was selected.
    EffectKindRule,

    /// No applicable allow rule existed; default deny was applied.
    DefaultDeny,
}

/// The decision and provenance for one concrete effect instance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectAuthorizationRecord {
    effect: EffectInstance,
    decision: AuthorizationDecision,
    source: AuthorizationDecisionSource,
    policy_id: Option<String>,
    policy_revision: Option<String>,
    rationale: Option<String>,
}

impl EffectAuthorizationRecord {
    /// Returns the effect that was evaluated.
    #[must_use]
    pub fn effect(&self) -> &EffectInstance {
        &self.effect
    }

    /// Returns the authorization decision.
    #[must_use]
    pub const fn decision(&self) -> AuthorizationDecision {
        self.decision
    }

    /// Returns the source of the decision.
    #[must_use]
    pub const fn source(&self) -> AuthorizationDecisionSource {
        self.source
    }

    /// Returns the originating policy ID, when an explicit rule was used.
    #[must_use]
    pub fn policy_id(&self) -> Option<&str> {
        self.policy_id.as_deref()
    }

    /// Returns the originating policy revision, if supplied.
    #[must_use]
    pub fn policy_revision(&self) -> Option<&str> {
        self.policy_revision.as_deref()
    }

    /// Returns the rule rationale or default-deny explanation.
    #[must_use]
    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }

    /// Returns whether this effect was explicitly permitted.
    #[must_use]
    pub const fn is_allowed(&self) -> bool {
        matches!(self.decision, AuthorizationDecision::Allow)
    }
}

/// The result of authorizing an effect row.
///
/// A report is fully authorized only if every concrete effect is allowed and
/// no effect variables remain unresolved. An empty, closed row is authorized
/// because it contains no effect requiring permission; an open row is not.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EffectAuthorizationReport {
    decisions: Vec<EffectAuthorizationRecord>,
    unresolved_variables: Vec<EffectVariable>,
}

impl EffectAuthorizationReport {
    /// Returns all decisions in deterministic effect order.
    #[must_use]
    pub fn decisions(&self) -> &[EffectAuthorizationRecord] {
        &self.decisions
    }

    /// Returns unresolved effect variables in deterministic order.
    #[must_use]
    pub fn unresolved_variables(&self) -> &[EffectVariable] {
        &self.unresolved_variables
    }

    /// Returns whether all effects are permitted and no variables remain
    /// unresolved.
    #[must_use]
    pub fn is_authorized(&self) -> bool {
        self.unresolved_variables.is_empty()
            && self
                .decisions
                .iter()
                .all(EffectAuthorizationRecord::is_allowed)
    }

    /// Returns whether any concrete effect was denied.
    #[must_use]
    pub fn has_denials(&self) -> bool {
        self.decisions
            .iter()
            .any(|record| !record.is_allowed())
    }

    /// Returns whether authorization is incomplete because an effect-row
    /// variable has not been resolved.
    #[must_use]
    pub fn has_unresolved_variables(&self) -> bool {
        !self.unresolved_variables.is_empty()
    }

    /// Returns the number of denied concrete effects.
    #[must_use]
    pub fn denied_count(&self) -> usize {
        self.decisions
            .iter()
            .filter(|record| !record.is_allowed())
            .count()
    }

    /// Returns the number of concrete effects evaluated.
    #[must_use]
    pub fn evaluated_count(&self) -> usize {
        self.decisions.len()
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors raised while constructing an authorization policy.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AuthorizationPolicyError {
    /// A policy field was empty or contained forbidden control characters.
    InvalidRuleField {
        /// Field name.
        field: &'static str,
    },

    /// A malformed effect kind was supplied.
    InvalidEffectKind {
        /// Structural validation message.
        message: String,
    },

    /// A malformed effect instance was supplied.
    InvalidEffect {
        /// Structural validation message.
        message: String,
    },

    /// A second rule was registered for the same effect kind.
    DuplicateKindRule {
        /// Canonical effect-kind identity.
        effect_kind: String,
    },

    /// A second rule was registered for the same exact effect instance.
    DuplicateInstanceRule {
        /// Canonical effect-instance display.
        effect: String,
    },
}

impl fmt::Display for AuthorizationPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRuleField { field } => {
                write!(
                    f,
                    "authorization rule {field} must be non-empty and \
                     contain no control characters"
                )
            }
            Self::InvalidEffectKind { message } => {
                write!(f, "invalid authorization effect kind: {message}")
            }
            Self::InvalidEffect { message } => {
                write!(f, "invalid authorization effect instance: {message}")
            }
            Self::DuplicateKindRule { effect_kind } => {
                write!(
                    f,
                    "duplicate authorization rule for effect kind \
                     `{effect_kind}`"
                )
            }
            Self::DuplicateInstanceRule { effect } => {
                write!(
                    f,
                    "duplicate authorization rule for effect instance \
                     `{effect}`"
                )
            }
        }
    }
}

impl Error for AuthorizationPolicyError {}

/// Errors raised when authorization cannot reliably evaluate an effect row.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum EffectAuthorizationError {
    /// The supplied effect row is structurally malformed.
    InvalidEffectRow(EffectModelError),
}

impl fmt::Display for EffectAuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEffectRow(error) => {
                write!(f, "cannot authorize malformed effect row: {error}")
            }
        }
    }
}

impl Error for EffectAuthorizationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidEffectRow(error) => Some(error),
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::effects::model::EffectVariable;

    fn kind(namespace: &str, name: &str) -> EffectKind {
        EffectKind::new(namespace, name)
            .expect("test effect kind should be structurally valid")
    }

    fn effect(namespace: &str, name: &str) -> EffectInstance {
        EffectInstance::simple(kind(namespace, name))
    }

    fn rule(
        decision: AuthorizationDecision,
        policy_id: &str,
    ) -> AuthorizationRule {
        AuthorizationRule::new(decision, policy_id, None, None)
            .expect("test rule should be valid")
    }

    #[test]
    fn empty_closed_row_is_authorized() {
        let policy = EffectAuthorizationPolicy::new();
        let report = policy
            .authorize(&EffectRow::empty())
            .expect("empty row should be structurally valid");

        assert!(report.is_authorized());
        assert_eq!(report.evaluated_count(), 0);
        assert_eq!(report.denied_count(), 0);
    }

    #[test]
    fn an_effect_without_an_allow_rule_is_denied() {
        let policy = EffectAuthorizationPolicy::new();
        let row = EffectRow::closed([effect("quantum", "measurement")])
            .expect("test row should be valid");

        let report = policy.authorize(&row).unwrap();

        assert!(!report.is_authorized());
        assert!(report.has_denials());
        assert_eq!(report.denied_count(), 1);
        assert_eq!(
            report.decisions()[0].source(),
            AuthorizationDecisionSource::DefaultDeny
        );
    }

    #[test]
    fn kind_rule_allows_open_world_effects() {
        let mut policy = EffectAuthorizationPolicy::new();
        policy
            .add_kind_rule(
                kind("future_domain", "compute"),
                rule(AuthorizationDecision::Allow, "policy.future"),
            )
            .unwrap();

        let row = EffectRow::closed([
            effect("future_domain", "compute"),
        ])
        .unwrap();

        let report = policy.authorize(&row).unwrap();

        assert!(report.is_authorized());
        assert_eq!(
            report.decisions()[0].source(),
            AuthorizationDecisionSource::EffectKindRule
        );
        assert_eq!(
            report.decisions()[0].policy_id(),
            Some("policy.future")
        );
    }

    #[test]
    fn exact_instance_rule_is_more_specific_than_kind_allow() {
        let mut policy = EffectAuthorizationPolicy::new();
        let item = effect("domain", "operation");

        policy
            .add_kind_rule(
                kind("domain", "operation"),
                rule(AuthorizationDecision::Allow, "policy.kind"),
            )
            .unwrap();
        policy
            .add_instance_rule(
                item.clone(),
                rule(AuthorizationDecision::Allow, "policy.instance"),
            )
            .unwrap();

        let record = policy.authorize_instance(&item);

        assert!(record.is_allowed());
        assert_eq!(
            record.source(),
            AuthorizationDecisionSource::ExactInstanceRule
        );
        assert_eq!(record.policy_id(), Some("policy.instance"));
    }

    #[test]
    fn deny_overrides_an_exact_instance_allow() {
        let mut policy = EffectAuthorizationPolicy::new();
        let item = effect("security", "credential");

        policy
            .add_kind_rule(
                kind("security", "credential"),
                rule(AuthorizationDecision::Deny, "policy.security"),
            )
            .unwrap();
        policy
            .add_instance_rule(
                item.clone(),
                rule(AuthorizationDecision::Allow, "policy.exception"),
            )
            .unwrap();

        let record = policy.authorize_instance(&item);

        assert!(!record.is_allowed());
        assert_eq!(
            record.source(),
            AuthorizationDecisionSource::EffectKindRule
        );
        assert_eq!(record.policy_id(), Some("policy.security"));
    }

    #[test]
    fn exact_instance_deny_overrides_a_kind_allow() {
        let mut policy = EffectAuthorizationPolicy::new();
        let item = effect("network", "request");

        policy
            .add_kind_rule(
                kind("network", "request"),
                rule(AuthorizationDecision::Allow, "policy.network"),
            )
            .unwrap();
        policy
            .add_instance_rule(
                item.clone(),
                rule(AuthorizationDecision::Deny, "policy.blocked"),
            )
            .unwrap();

        let record = policy.authorize_instance(&item);

        assert!(!record.is_allowed());
        assert_eq!(
            record.source(),
            AuthorizationDecisionSource::ExactInstanceRule
        );
        assert_eq!(record.policy_id(), Some("policy.blocked"));
    }

    #[test]
    fn unresolved_effect_variables_prevent_full_authorization() {
        let mut policy = EffectAuthorizationPolicy::new();
        policy
            .add_kind_rule(
                kind("domain", "known"),
                rule(AuthorizationDecision::Allow, "policy.known"),
            )
            .unwrap();

        let row = EffectRow::new(
            [effect("domain", "known")],
            [EffectVariable::new("e").unwrap()],
        )
        .unwrap();

        let report = policy.authorize(&row).unwrap();

        assert!(report.has_unresolved_variables());
        assert!(!report.is_authorized());
        assert_eq!(report.unresolved_variables().len(), 1);
    }

    #[test]
    fn duplicate_kind_rules_are_rejected() {
        let mut policy = EffectAuthorizationPolicy::new();
        let effect_kind = kind("domain", "operation");

        policy
            .add_kind_rule(
                effect_kind.clone(),
                rule(AuthorizationDecision::Allow, "policy.first"),
            )
            .unwrap();

        let result = policy.add_kind_rule(
            effect_kind,
            rule(AuthorizationDecision::Deny, "policy.second"),
        );

        assert!(matches!(
            result,
            Err(AuthorizationPolicyError::DuplicateKindRule { .. })
        ));
    }

    #[test]
    fn duplicate_instance_rules_are_rejected() {
        let mut policy = EffectAuthorizationPolicy::new();
        let item = effect("domain", "operation");

        policy
            .add_instance_rule(
                item.clone(),
                rule(AuthorizationDecision::Allow, "policy.first"),
            )
            .unwrap();

        let result = policy.add_instance_rule(
            item,
            rule(AuthorizationDecision::Deny, "policy.second"),
        );

        assert!(matches!(
            result,
            Err(AuthorizationPolicyError::DuplicateInstanceRule { .. })
        ));
    }

    #[test]
    fn rule_provenance_is_retained() {
        let rule = AuthorizationRule::new(
            AuthorizationDecision::Allow,
            "policy.audit",
            Some("revision-7".to_owned()),
            Some("approved by semantic policy".to_owned()),
        )
        .unwrap();

        assert_eq!(rule.policy_id(), "policy.audit");
        assert_eq!(rule.revision(), Some("revision-7"));
        assert_eq!(
            rule.rationale(),
            Some("approved by semantic policy")
        );
    }

    #[test]
    fn malformed_policy_metadata_is_rejected() {
        assert!(AuthorizationRule::allow(" ").is_err());
        assert!(AuthorizationRule::new(
            AuthorizationDecision::Deny,
            "policy.valid",
            None,
            Some("invalid\0rationale".to_owned()),
        )
        .is_err());
    }

    #[test]
    fn authorization_is_deterministic_for_the_same_policy_and_row() {
        let mut policy = EffectAuthorizationPolicy::new();

        policy
            .add_kind_rule(
                kind("domain", "z"),
                rule(AuthorizationDecision::Allow, "policy.z"),
            )
            .unwrap();
        policy
            .add_kind_rule(
                kind("domain", "a"),
                rule(AuthorizationDecision::Deny, "policy.a"),
            )
            .unwrap();

        let row = EffectRow::closed([
            effect("domain", "z"),
            effect("domain", "a"),
        ])
        .unwrap();

        let first = policy.authorize(&row).unwrap();
        let second = policy.authorize(&row).unwrap();

        assert_eq!(first, second);
        assert_eq!(
            first.decisions()[0].effect().kind().name(),
            "a"
        );
        assert_eq!(
            first.decisions()[1].effect().kind().name(),
            "z"
        );
    }

    #[test]
    fn domains_use_the_same_authorization_mechanism() {
        let mut policy = EffectAuthorizationPolicy::new();

        for (namespace, name) in [
            ("classical", "mutation"),
            ("quantum", "measurement"),
            ("hdl", "signal_write"),
            ("tensor", "compute"),
            ("distributed", "communicate"),
        ] {
            policy
                .add_kind_rule(
                    kind(namespace, name),
                    rule(
                        AuthorizationDecision::Allow,
                        "policy.domain-effects",
                    ),
                )
                .unwrap();
        }

        let row = EffectRow::closed([
            effect("classical", "mutation"),
            effect("quantum", "measurement"),
            effect("hdl", "signal_write"),
            effect("tensor", "compute"),
            effect("distributed", "communicate"),
        ])
        .unwrap();

        let report = policy.authorize(&row).unwrap();

        assert!(report.is_authorized());
        assert_eq!(report.evaluated_count(), 5);
    }
}
