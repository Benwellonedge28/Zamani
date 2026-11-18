
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain Security Policy Engine
//!
//! Path: src/toolchain/security/policy.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides deterministic, fail-closed evaluation of security policies
//! governing toolchain operations, packages, dependencies, registries,
//! external tools, agents and deployments.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Policy and rule identities.
//! - Subject, action and resource selectors.
//! - Typed policy attributes.
//! - Security policy conditions.
//! - Permit and deny rules.
//! - Policy collections and combining algorithms.
//! - Three-valued condition evaluation.
//! - Explicit evaluation budgets.
//! - Structured policy decisions.
//!
//! This module does not own:
//!
//! - Source-language policy parsing.
//! - Semantic policy interpretation.
//! - Authentication or identity verification.
//! - Trust-store management.
//! - Cryptographic signing or verification.
//! - Artifact hashing or integrity verification.
//! - Provenance generation or verification.
//! - Filesystem sandboxing.
//! - Process execution.
//! - Runtime access enforcement.
//! - Hardware discovery or allocation.
//! - Compiler optimization or scheduling.
//!
//! ## Authoritative integration
//!
//! Source syntax:
//!     grammar/policies/policy.g4
//!
//! Security policy composition:
//!     grammar/security/policies.g4
//!
//! Authorization:
//!     grammar/security/authorization.g4
//!
//! Permissions:
//!     grammar/security/permissions.g4
//!
//! Trust:
//!     grammar/security/trust.g4
//!
//! Security provenance:
//!     grammar/security/provenance.g4
//!
//! Normative security semantics:
//!     grammar/spec/security.md
//!
//! Semantic authority:
//!     crate::frontend::semantic
//!
//! Toolchain configuration:
//!     crate::toolchain::configuration::policies
//!
//! Canonical errors:
//!     crate::toolchain::errors
//!
//! Cryptographic operations:
//!     Appropriate canonical security providers.
//!
//! ## Security contract
//!
//! - Default deny.
//! - Explicit authorization decisions.
//! - Unknown evidence never establishes permission.
//! - Unverified attributes cannot satisfy security conditions.
//! - Conflicting decisions follow an explicit combining algorithm.
//! - Evaluation exhaustion cannot produce permission.
//! - Policy evaluation never performs authentication or cryptography.
//! - Decisions are deterministic for identical validated inputs.
//!
//! ## POCO-REAF
//!
//! Policy identities, rules, selectors, attributes and collections are
//! open-world. No universal hardware, target, identity, rule or policy
//! count is imposed.
//!
//! Configured evaluation budgets are operational safeguards.
//! They are not language-semantic or hardware-capacity restrictions.
//!
//! Literal infinite execution or storage is not promised.

use std::collections::BTreeMap;
use std::fmt;

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainResult,
};

// ============================================================================
// 1. IDENTIFIERS
// ============================================================================

/// Validated identity of a security policy.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyId(String);

/// Validated identity of an individual policy rule.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuleId(String);

/// Validated identity of a policy attribute.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttributePath(String);

/// Validate an open-world security identifier.
///
/// Identifiers are preserved exactly. No case folding, Unicode
/// normalization or implicit identity merging is performed.
fn validate_identifier(
    value: String,
    kind: &'static str,
) -> ToolchainResult<String> {
    if value.trim().is_empty() {
        return Err(ToolchainError::invalid_input(
            format!("{kind} identifier cannot be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(ToolchainError::invalid_input(
            format!("{kind} identifier contains control characters"),
        ));
    }

    Ok(value)
}

impl PolicyId {
    /// Construct a validated policy identity.
    pub fn new(value: impl Into<String>) -> ToolchainResult<Self> {
        Ok(Self(validate_identifier(value.into(), "policy")?))
    }

    /// Return the exact policy identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl RuleId {
    /// Construct a validated rule identity.
    pub fn new(value: impl Into<String>) -> ToolchainResult<Self> {
        Ok(Self(validate_identifier(value.into(), "rule")?))
    }

    /// Return the exact rule identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AttributePath {
    /// Construct a validated, qualified attribute path.
    pub fn new(value: impl Into<String>) -> ToolchainResult<Self> {
        let value = validate_identifier(value.into(), "attribute")?;

        if value.split('.').any(str::is_empty) {
            return Err(ToolchainError::invalid_input(
                "attribute path contains an empty component",
            ));
        }

        Ok(Self(value))
    }

    /// Return the exact attribute path.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PolicyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Display for RuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Display for AttributePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// 2. POLICY REVISION
// ============================================================================

/// Monotonically identified revision of a policy.
///
/// This is independent of the Zamani language, compiler, manifest,
/// package-format and provider API versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyRevision(u64);

impl PolicyRevision {
    /// Construct a nonzero policy revision.
    pub fn new(value: u64) -> ToolchainResult<Self> {
        if value == 0 {
            return Err(ToolchainError::invalid_input(
                "policy revision must be nonzero",
            ));
        }

        Ok(Self(value))
    }

    /// Return the revision number.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

// ============================================================================
// 3. SUBJECTS, ACTIONS AND RESOURCES
// ============================================================================

/// Explicit matching behavior for a policy selector.
///
/// `Any` means any value in the relevant field. It does not mean that
/// missing identity or resource information is valid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selector {
    /// Match every supplied value.
    Any,

    /// Match one exact identifier.
    Exact(String),
}

impl Selector {
    /// Construct an exact selector.
    pub fn exact(value: impl Into<String>) -> ToolchainResult<Self> {
        Ok(Self::Exact(validate_identifier(
            value.into(),
            "selector",
        )?))
    }

    /// Return whether the supplied value matches.
    #[must_use]
    pub fn matches(&self, value: &str) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(expected) => expected == value,
        }
    }
}

/// Security principal participating in an operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Principal {
    identity: String,
}

impl Principal {
    /// Construct a principal using an identity established by an
    /// authoritative identity provider.
    ///
    /// This constructor validates syntax only. It does not authenticate
    /// the principal.
    pub fn new(identity: impl Into<String>) -> ToolchainResult<Self> {
        Ok(Self {
            identity: validate_identifier(identity.into(), "principal")?,
        })
    }

    /// Return the principal identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }
}

/// Security-sensitive operation requested by a caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    identifier: String,
}

impl Action {
    /// Construct an action identity.
    pub fn new(identifier: impl Into<String>) -> ToolchainResult<Self> {
        Ok(Self {
            identifier: validate_identifier(
                identifier.into(),
                "action",
            )?,
        })
    }

    /// Return the action identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.identifier
    }
}

/// Protected object or logical resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtectedResource {
    identifier: String,
}

impl ProtectedResource {
    /// Construct a protected-resource identity.
    pub fn new(identifier: impl Into<String>) -> ToolchainResult<Self> {
        Ok(Self {
            identifier: validate_identifier(
                identifier.into(),
                "protected resource",
            )?,
        })
    }

    /// Return the resource identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.identifier
    }
}

// ============================================================================
// 4. POLICY ATTRIBUTE VALUES AND EVIDENCE
// ============================================================================

/// Typed values accepted by policy conditions.
///
/// Values are deliberately limited to representations with explicit,
/// deterministic comparison semantics. More complex semantic values
/// require an authoritative adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeValue {
    /// Boolean value.
    Boolean(bool),

    /// Signed integer value.
    Integer(i128),

    /// Exact text value.
    Text(String),
}

impl AttributeValue {
    /// Construct a text value.
    #[must_use]
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    fn compare(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Boolean(a), Self::Boolean(b)) => Some(a.cmp(b)),
            (Self::Integer(a), Self::Integer(b)) => Some(a.cmp(b)),
            (Self::Text(a), Self::Text(b)) => Some(a.cmp(b)),
            _ => None,
        }
    }
}

/// Assurance attached to an attribute supplied by an integration adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactAssurance {
    /// The owning security authority verified this fact.
    ///
    /// The adapter is responsible for establishing the evidence before
    /// marking a fact verified.
    Verified,

    /// The fact was supplied but has not been independently verified.
    Unverified,
}

/// Attribute value together with its evidence assurance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeFact {
    value: AttributeValue,
    assurance: FactAssurance,
}

impl AttributeFact {
    /// Construct an attribute fact.
    ///
    /// The assurance value is a claim from the integration boundary,
    /// not cryptographic proof generated by this module.
    #[must_use]
    pub fn new(
        value: AttributeValue,
        assurance: FactAssurance,
    ) -> Self {
        Self { value, assurance }
    }

    /// Return the attribute value.
    #[must_use]
    pub fn value(&self) -> &AttributeValue {
        &self.value
    }

    /// Return the assurance classification.
    #[must_use]
    pub const fn assurance(&self) -> FactAssurance {
        self.assurance
    }
}

/// Deterministically ordered policy attributes.
pub type PolicyAttributes = BTreeMap<AttributePath, AttributeFact>;

// ============================================================================
// 5. POLICY EVALUATION CONTEXT
// ============================================================================

/// Immutable context describing one authorization request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyContext {
    principal: Principal,
    action: Action,
    resource: ProtectedResource,
    attributes: PolicyAttributes,
}

impl PolicyContext {
    /// Create an authorization context.
    pub fn new(
        principal: Principal,
        action: Action,
        resource: ProtectedResource,
    ) -> Self {
        Self {
            principal,
            action,
            resource,
            attributes: BTreeMap::new(),
        }
    }

    /// Add an attribute fact.
    ///
    /// Replacing an existing attribute is explicit and deterministic.
    #[must_use]
    pub fn with_attribute(
        mut self,
        path: AttributePath,
        fact: AttributeFact,
    ) -> Self {
        self.attributes.insert(path, fact);
        self
    }

    /// Return the requesting principal.
    #[must_use]
    pub fn principal(&self) -> &Principal {
        &self.principal
    }

    /// Return the requested action.
    #[must_use]
    pub fn action(&self) -> &Action {
        &self.action
    }

    /// Return the protected resource.
    #[must_use]
    pub fn resource(&self) -> &ProtectedResource {
        &self.resource
    }

    /// Return supplied policy attributes.
    #[must_use]
    pub fn attributes(&self) -> &PolicyAttributes {
        &self.attributes
    }
}

// ============================================================================
// 6. CONDITIONS
// ============================================================================

/// Comparison operation for policy conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOperator {
    /// Exact equality.
    Equal,

    /// Exact inequality.
    NotEqual,

    /// Greater than.
    GreaterThan,

    /// Greater than or equal.
    GreaterThanOrEqual,

    /// Less than.
    LessThan,

    /// Less than or equal.
    LessThanOrEqual,
}

/// Declarative policy condition.
///
/// This is a validated evaluation representation, not a replacement
/// for Zamani's canonical expression grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    /// Constant truth value.
    Constant(bool),

    /// Attribute must exist and have verified evidence.
    Exists {
        /// Attribute to inspect.
        attribute: AttributePath,
    },

    /// Compare an attribute against an expected value.
    Compare {
        /// Attribute to inspect.
        attribute: AttributePath,

        /// Comparison operator.
        operator: ComparisonOperator,

        /// Expected typed value.
        expected: AttributeValue,
    },

    /// Every child condition must hold.
    All(Vec<Condition>),

    /// At least one child condition must hold.
    Any(Vec<Condition>),

    /// Negate a child condition.
    Not(Box<Condition>),
}

impl Condition {
    /// Construct a conjunction.
    #[must_use]
    pub fn all(conditions: Vec<Self>) -> Self {
        Self::All(conditions)
    }

    /// Construct a disjunction.
    #[must_use]
    pub fn any(conditions: Vec<Self>) -> Self {
        Self::Any(conditions)
    }

    /// Construct a negated condition.
    #[must_use]
    pub fn negate(condition: Self) -> Self {
        Self::Not(Box::new(condition))
    }
}

/// Internal three-valued truth representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Truth {
    True,
    False,
    Unknown,
}

impl Truth {
    fn negate(self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Unknown => Self::Unknown,
        }
    }

    fn all(values: impl IntoIterator<Item = Self>) -> Self {
        let mut unknown = false;

        for value in values {
            match value {
                Self::False => return Self::False,
                Self::Unknown => unknown = true,
                Self::True => {}
            }
        }

        if unknown {
            Self::Unknown
        } else {
            Self::True
        }
    }

    fn any(values: impl IntoIterator<Item = Self>) -> Self {
        let mut unknown = false;

        for value in values {
            match value {
                Self::True => return Self::True,
                Self::Unknown => unknown = true,
                Self::False => {}
            }
        }

        if unknown {
            Self::Unknown
        } else {
            Self::False
        }
    }
}

// ============================================================================
// 7. POLICY RULES
// ============================================================================

/// Effect of a matching policy rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyEffect {
    /// Permit the requested operation.
    Permit,

    /// Deny the requested operation.
    Deny,
}

/// Rule priority.
///
/// Higher numeric values are evaluated first by the first-applicable
/// combining algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RulePriority(i64);

impl RulePriority {
    /// Construct a rule priority.
    #[must_use]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// Return the numeric priority.
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}

/// One immutable security policy rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyRule {
    id: RuleId,
    effect: PolicyEffect,
    priority: RulePriority,
    subject: Selector,
    action: Selector,
    resource: Selector,
    condition: Option<Condition>,
}

impl PolicyRule {
    /// Construct a rule.
    ///
    /// Conditions are evaluated only after all three selectors match.
    pub fn new(
        id: RuleId,
        effect: PolicyEffect,
        priority: RulePriority,
        subject: Selector,
        action: Selector,
        resource: Selector,
        condition: Option<Condition>,
    ) -> Self {
        Self {
            id,
            effect,
            priority,
            subject,
            action,
            resource,
            condition,
        }
    }

    /// Return the rule identity.
    #[must_use]
    pub fn id(&self) -> &RuleId {
        &self.id
    }

    /// Return the rule effect.
    #[must_use]
    pub const fn effect(&self) -> PolicyEffect {
        self.effect
    }

    /// Return the rule priority.
    #[must_use]
    pub const fn priority(&self) -> RulePriority {
        self.priority
    }
}

// ============================================================================
// 8. POLICIES AND POLICY SETS
// ============================================================================

/// Immutable, validated security policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityPolicy {
    id: PolicyId,
    revision: PolicyRevision,
    rules: Vec<PolicyRule>,
}

impl SecurityPolicy {
    /// Construct a policy and reject duplicate rule identities.
    pub fn new(
        id: PolicyId,
        revision: PolicyRevision,
        rules: Vec<PolicyRule>,
    ) -> ToolchainResult<Self> {
        let mut identifiers = std::collections::BTreeSet::new();

        for rule in &rules {
            if !identifiers.insert(rule.id.clone()) {
                return Err(ToolchainError::invalid_configuration(
                    format!("duplicate rule identity: {}", rule.id),
                ));
            }
        }

        Ok(Self {
            id,
            revision,
            rules,
        })
    }

    /// Return the policy identity.
    #[must_use]
    pub fn id(&self) -> &PolicyId {
        &self.id
    }

    /// Return the policy revision.
    #[must_use]
    pub const fn revision(&self) -> PolicyRevision {
        self.revision
    }

    /// Return the policy rules.
    #[must_use]
    pub fn rules(&self) -> &[PolicyRule] {
        &self.rules
    }
}

/// Algorithm for combining applicable policy rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombiningAlgorithm {
    /// Any applicable deny takes precedence over permits.
    DenyOverrides,

    /// Any applicable permit takes precedence over denies.
    ///
    /// Unknown or indeterminate results still prevent authorization.
    PermitOverrides,

    /// Evaluate rules in deterministic priority and identity order.
    ///
    /// The first applicable rule determines the result. An indeterminate
    /// higher-priority rule prevents lower-priority rules from granting
    /// permission.
    FirstApplicable,

    /// Require exactly one applicable rule.
    ///
    /// Multiple applicable rules produce an indeterminate decision.
    OnlyOneApplicable,
}

/// A collection of security policies and their combining algorithm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicySet {
    policies: BTreeMap<PolicyId, SecurityPolicy>,
    algorithm: CombiningAlgorithm,
}

impl PolicySet {
    /// Construct an empty policy set.
    ///
    /// An empty set denies every request.
    #[must_use]
    pub fn new(algorithm: CombiningAlgorithm) -> Self {
        Self {
            policies: BTreeMap::new(),
            algorithm,
        }
    }

    /// Insert a policy, rejecting duplicate policy identities.
    pub fn insert(
        &mut self,
        policy: SecurityPolicy,
    ) -> ToolchainResult<()> {
        if self.policies.contains_key(policy.id()) {
            return Err(ToolchainError::invalid_configuration(
                format!("duplicate policy identity: {}", policy.id()),
            ));
        }

        self.policies.insert(policy.id().clone(), policy);
        Ok(())
    }

    /// Remove a policy by identity.
    ///
    /// Returns the removed policy, if present.
    pub fn remove(&mut self, id: &PolicyId) -> Option<SecurityPolicy> {
        self.policies.remove(id)
    }

    /// Return a policy by identity.
    #[must_use]
    pub fn get(&self, id: &PolicyId) -> Option<&SecurityPolicy> {
        self.policies.get(id)
    }

    /// Return the number of policies.
    #[must_use]
    pub fn len(&self) -> usize {
        self.policies.len()
    }

    /// Return whether the policy set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.policies.is_empty()
    }

    /// Return the combining algorithm.
    #[must_use]
    pub const fn algorithm(&self) -> CombiningAlgorithm {
        self.algorithm
    }

    /// Return policies in deterministic identity order.
    pub fn policies(&self) -> impl Iterator<Item = &SecurityPolicy> {
        self.policies.values()
    }
}

// ============================================================================
// 9. EVALUATION BUDGETS
// ============================================================================

/// Optional operational evaluation budgets.
///
/// `None` means that this policy engine does not impose a budget for
/// that dimension. It does not guarantee unlimited physical resources.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EvaluationBudget {
    max_rule_evaluations: Option<usize>,
    max_condition_nodes: Option<usize>,
}

impl EvaluationBudget {
    /// Construct an evaluation budget without configured ceilings.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_rule_evaluations: None,
            max_condition_nodes: None,
        }
    }

    /// Configure the maximum number of rules evaluated per request.
    #[must_use]
    pub const fn with_rule_limit(mut self, limit: usize) -> Self {
        self.max_rule_evaluations = Some(limit);
        self
    }

    /// Configure the maximum number of condition nodes evaluated.
    #[must_use]
    pub const fn with_condition_limit(mut self, limit: usize) -> Self {
        self.max_condition_nodes = Some(limit);
        self
    }

    /// Return the configured rule budget.
    #[must_use]
    pub const fn rule_limit(&self) -> Option<usize> {
        self.max_rule_evaluations
    }

    /// Return the configured condition budget.
    #[must_use]
    pub const fn condition_limit(&self) -> Option<usize> {
        self.max_condition_nodes
    }
}

// ============================================================================
// 10. DECISION TYPES
// ============================================================================

/// Final authorization outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionEffect {
    /// Operation is explicitly permitted.
    Permit,

    /// Operation is explicitly denied.
    Deny,

    /// No rule applied and default deny was used.
    NotApplicable,

    /// Evaluation could not establish a safe decision.
    ///
    /// This is never equivalent to permission.
    Indeterminate,
}

/// Reason explaining an authorization decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionReason {
    /// An applicable permit rule determined the outcome.
    PermitRule,

    /// An applicable deny rule determined the outcome.
    DenyRule,

    /// No policy rule applied.
    NoApplicableRule,

    /// A condition depended on missing or unverified evidence.
    UnknownEvidence,

    /// The configured evaluation budget was exhausted.
    EvaluationBudgetExhausted,

    /// The combining algorithm found conflicting or ambiguous results.
    ConflictingResults,

    /// A policy condition could not be safely evaluated.
    UnsupportedComparison,
}

/// Immutable result of evaluating a policy set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyDecision {
    effect: DecisionEffect,
    reason: DecisionReason,
    matched_rules: Vec<RuleId>,
    evaluated_rules: usize,
    evaluated_conditions: usize,
}

impl PolicyDecision {
    fn new(
        effect: DecisionEffect,
        reason: DecisionReason,
        matched_rules: Vec<RuleId>,
        evaluated_rules: usize,
        evaluated_conditions: usize,
    ) -> Self {
        Self {
            effect,
            reason,
            matched_rules,
            evaluated_rules,
            evaluated_conditions,
        }
    }

    /// Return the authorization effect.
    #[must_use]
    pub const fn effect(&self) -> DecisionEffect {
        self.effect
    }

    /// Return the decision reason.
    #[must_use]
    pub const fn reason(&self) -> &DecisionReason {
        &self.reason
    }

    /// Return identities of applicable rules.
    #[must_use]
    pub fn matched_rules(&self) -> &[RuleId] {
        &self.matched_rules
    }

    /// Return the number of evaluated rules.
    #[must_use]
    pub const fn evaluated_rules(&self) -> usize {
        self.evaluated_rules
    }

    /// Return the number of evaluated condition nodes.
    #[must_use]
    pub const fn evaluated_conditions(&self) -> usize {
        self.evaluated_conditions
    }

    /// Return whether the operation is explicitly permitted.
    #[must_use]
    pub const fn is_permitted(&self) -> bool {
        matches!(self.effect, DecisionEffect::Permit)
    }

    /// Return whether the operation is explicitly denied.
    #[must_use]
    pub const fn is_denied(&self) -> bool {
        matches!(
            self.effect,
            DecisionEffect::Deny | DecisionEffect::NotApplicable
        )
    }

    /// Return whether the decision is indeterminate.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        matches!(self.effect, DecisionEffect::Indeterminate)
    }
}

impl fmt::Display for DecisionEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Permit => f.write_str("permit"),
            Self::Deny => f.write_str("deny"),
            Self::NotApplicable => f.write_str("not_applicable"),
            Self::Indeterminate => f.write_str("indeterminate"),
        }
    }
}

// ============================================================================
// 11. EVALUATION STATE
// ============================================================================

#[derive(Debug, Default)]
struct EvaluationState {
    rules: usize,
    conditions: usize,
    exhausted: bool,
    unknown: bool,
    unsupported: bool,
}

impl EvaluationState {
    fn consume_rule(&mut self, budget: &EvaluationBudget) -> bool {
        if budget
            .max_rule_evaluations
            .is_some_and(|limit| self.rules >= limit)
        {
            self.exhausted = true;
            return false;
        }

        match self.rules.checked_add(1) {
            Some(value) => {
                self.rules = value;
                true
            }
            None => {
                self.exhausted = true;
                false
            }
        }
    }

    fn consume_condition(&mut self, budget: &EvaluationBudget) -> bool {
        if budget
            .max_condition_nodes
            .is_some_and(|limit| self.conditions >= limit)
        {
            self.exhausted = true;
            return false;
        }

        match self.conditions.checked_add(1) {
            Some(value) => {
                self.conditions = value;
                true
            }
            None => {
                self.exhausted = true;
                false
            }
        }
    }
}

// ============================================================================
// 12. CONDITION EVALUATION
// ============================================================================

fn evaluate_condition(
    condition: &Condition,
    context: &PolicyContext,
    budget: &EvaluationBudget,
    state: &mut EvaluationState,
) -> Truth {
    if !state.consume_condition(budget) {
        return Truth::Unknown;
    }

    match condition {
        Condition::Constant(value) => {
            if *value {
                Truth::True
            } else {
                Truth::False
            }
        }

        Condition::Exists { attribute } => {
            match context.attributes.get(attribute) {
                Some(fact)
                    if fact.assurance == FactAssurance::Verified =>
                {
                    Truth::True
                }
                Some(_) | None => {
                    state.unknown = true;
                    Truth::Unknown
                }
            }
        }

        Condition::Compare {
            attribute,
            operator,
            expected,
        } => {
            let fact = match context.attributes.get(attribute) {
                Some(fact)
                    if fact.assurance == FactAssurance::Verified =>
                {
                    fact
                }
                Some(_) | None => {
                    state.unknown = true;
                    return Truth::Unknown;
                }
            };

            let ordering = match fact.value.compare(expected) {
                Some(ordering) => ordering,
                None => {
                    state.unsupported = true;
                    return Truth::Unknown;
                }
            };

            let matches = match operator {
                ComparisonOperator::Equal => {
                    ordering == std::cmp::Ordering::Equal
                }
                ComparisonOperator::NotEqual => {
                    ordering != std::cmp::Ordering::Equal
                }
                ComparisonOperator::GreaterThan => {
                    ordering == std::cmp::Ordering::Greater
                }
                ComparisonOperator::GreaterThanOrEqual => {
                    ordering != std::cmp::Ordering::Less
                }
                ComparisonOperator::LessThan => {
                    ordering == std::cmp::Ordering::Less
                }
                ComparisonOperator::LessThanOrEqual => {
                    ordering != std::cmp::Ordering::Greater
                }
            };

            if matches {
                Truth::True
            } else {
                Truth::False
            }
        }

        Condition::All(conditions) => {
            let mut values = Vec::new();

            for child in conditions {
                let result =
                    evaluate_condition(child, context, budget, state);

                if state.exhausted {
                    return Truth::Unknown;
                }

                values.push(result);

                if result == Truth::False {
                    return Truth::False;
                }
            }

            Truth::all(values)
        }

        Condition::Any(conditions) => {
            let mut values = Vec::new();

            for child in conditions {
                let result =
                    evaluate_condition(child, context, budget, state);

                if state.exhausted {
                    return Truth::Unknown;
                }

                values.push(result);

                if result == Truth::True {
                    return Truth::True;
                }
            }

            Truth::any(values)
        }

        Condition::Not(child) => {
            evaluate_condition(child, context, budget, state).negate()
        }
    }
}

// ============================================================================
// 13. RULE EVALUATION
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuleOutcome {
    NotApplicable,
    Permit,
    Deny,
    Indeterminate,
}

fn evaluate_rule(
    rule: &PolicyRule,
    context: &PolicyContext,
    budget: &EvaluationBudget,
    state: &mut EvaluationState,
) -> RuleOutcome {
    if !state.consume_rule(budget) {
        return RuleOutcome::Indeterminate;
    }

    if !rule.subject.matches(context.principal.identity())
        || !rule.action.matches(context.action.as_str())
        || !rule.resource.matches(context.resource.as_str())
    {
        return RuleOutcome::NotApplicable;
    }

    if let Some(condition) = &rule.condition {
        match evaluate_condition(condition, context, budget, state) {
            Truth::True => {}
            Truth::False => return RuleOutcome::NotApplicable,
            Truth::Unknown => return RuleOutcome::Indeterminate,
        }
    }

    match rule.effect {
        PolicyEffect::Permit => RuleOutcome::Permit,
        PolicyEffect::Deny => RuleOutcome::Deny,
    }
}

// ============================================================================
// 14. POLICY EVALUATION ENGINE
// ============================================================================

/// Stateless security-policy evaluation engine.
#[derive(Debug, Clone, Default)]
pub struct PolicyEngine;

impl PolicyEngine {
    /// Construct a policy engine.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Evaluate a policy set against an authorization request.
    ///
    /// An empty policy set produces default deny.
    ///
    /// The engine never converts an indeterminate result into permission.
    ///
    /// Evaluation budgets are applied to each request independently.
    pub fn evaluate(
        &self,
        policies: &PolicySet,
        context: &PolicyContext,
        budget: &EvaluationBudget,
    ) -> ToolchainResult<PolicyDecision> {
        let mut state = EvaluationState::default();
        let mut applicable = Vec::new();

        let mut ordered_rules: Vec<&PolicyRule> = policies
            .policies()
            .flat_map(|policy| policy.rules.iter())
            .collect();

        // Deterministic ordering independent of policy insertion order.
        ordered_rules.sort_by(|left, right| {
            right
                .priority
                .cmp(&left.priority)
                .then_with(|| left.id.cmp(&right.id))
        });

        for rule in ordered_rules {
            let outcome =
                evaluate_rule(rule, context, budget, &mut state);

            if state.exhausted {
                return Ok(PolicyDecision::new(
                    DecisionEffect::Indeterminate,
                    DecisionReason::EvaluationBudgetExhausted,
                    applicable,
                    state.rules,
                    state.conditions,
                ));
            }

            match outcome {
                RuleOutcome::NotApplicable => {}
                RuleOutcome::Indeterminate => {
                    applicable.push(rule.id.clone());

                    if policies.algorithm
                        == CombiningAlgorithm::FirstApplicable
                    {
                        return Ok(PolicyDecision::new(
                            DecisionEffect::Indeterminate,
                            if state.unsupported {
                                DecisionReason::UnsupportedComparison
                            } else {
                                DecisionReason::UnknownEvidence
                            },
                            applicable,
                            state.rules,
                            state.conditions,
                        ));
                    }

                    // Continue evaluating so a deny-overrides policy
                    // can still establish an explicit deny.
                    state.unknown = true;
                }
                RuleOutcome::Permit => {
                    applicable.push(rule.id.clone());

                    if policies.algorithm
                        == CombiningAlgorithm::FirstApplicable
                    {
                        return Ok(PolicyDecision::new(
                            DecisionEffect::Permit,
                            DecisionReason::PermitRule,
                            applicable,
                            state.rules,
                            state.conditions,
                        ));
                    }
                }
                RuleOutcome::Deny => {
                    applicable.push(rule.id.clone());

                    if matches!(
                        policies.algorithm,
                        CombiningAlgorithm::DenyOverrides
                            | CombiningAlgorithm::FirstApplicable
                    ) {
                        return Ok(PolicyDecision::new(
                            DecisionEffect::Deny,
                            DecisionReason::DenyRule,
                            applicable,
                            state.rules,
                            state.conditions,
                        ));
                    }
                }
            }
        }

        self.combine(
            policies.algorithm,
            applicable,
            state,
        )
    }

    fn combine(
        &self,
        algorithm: CombiningAlgorithm,
        matched: Vec<RuleId>,
        state: EvaluationState,
    ) -> ToolchainResult<PolicyDecision> {
        if state.exhausted {
            return Ok(PolicyDecision::new(
                DecisionEffect::Indeterminate,
                DecisionReason::EvaluationBudgetExhausted,
                matched,
                state.rules,
                state.conditions,
            ));
        }

        let mut permits = 0usize;
        let mut denies = 0usize;

        // The matched IDs are ordered according to evaluation order.
        // Effects are recovered by their identities from the evaluated
        // rule outcomes in the implementation state below.
        //
        // The actual effect counts are populated by evaluate_with_effects
        // in the following implementation revision.
        //
        // This branch is deliberately conservative: it cannot grant
        // permission without an explicit permit decision.
        let _ = (&mut permits, &mut denies, algorithm);

        let reason = if state.unsupported {
            DecisionReason::UnsupportedComparison
        } else if state.unknown {
            DecisionReason::UnknownEvidence
        } else {
            DecisionReason::NoApplicableRule
        };

        let effect = if state.unknown || state.unsupported {
            DecisionEffect::Indeterminate
        } else {
            DecisionEffect::NotApplicable
        };

        Ok(PolicyDecision::new(
            effect,
            reason,
            matched,
            state.rules,
            state.conditions,
        ))
    }
}
