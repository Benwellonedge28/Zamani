
#![forbid(unsafe_code)]

//! Zamani Compiler Security Policy Engine
//!
//! Path:
//!     src/compiler/security/policy.rs
//!
//! Rust:
//!     Rust 1.97+
//!     Rust 2021
//!
//! Safety:
//!     No unsafe Rust.
//!
//! PURPOSE
//! =======
//!
//! Provides deterministic, target-independent evaluation of validated
//! security policies supplied by Zamani's semantic/security integration.
//!
//! This module consumes policy representations. It does not parse source,
//! define policy grammar, authenticate identities, issue permissions,
//! allocate hardware, execute security obligations, or enforce runtime
//! access controls.
//!
//! ARCHITECTURAL OWNERSHIP
//! =======================
//!
//! Source policy syntax:
//!     grammar/policies/policy.g4
//!
//! Security policy composition:
//!     grammar/security/policies.g4
//!
//! Authorization syntax:
//!     grammar/security/authorization.g4
//!
//! Permission syntax:
//!     grammar/security/permissions.g4
//!
//! Identity and trust:
//!     grammar/security/
//!
//! Source semantics:
//!     semantic subsystem
//!
//! Compiler policy evaluation:
//!     THIS FILE
//!
//! IR verification:
//!     src/zuir/ and authoritative domain verifiers
//!
//! Runtime enforcement:
//!     src/runtime/ and authoritative security infrastructure
//!
//! SECURITY PRINCIPLES
//! ===================
//!
//! - Default-deny behavior.
//! - Explicit permit and deny decisions.
//! - Deny-overrides and other explicit combining algorithms.
//! - Missing attributes never silently become successful conditions.
//! - Evaluation-budget exhaustion produces an indeterminate decision.
//! - Stable ordering and deterministic results.
//! - No vendor-specific or hardware-specific policy identifiers.
//! - No arbitrary universal hardware or program-size limits.
//! - Configurable operational budgets.
//! - No silent policy fallback.
//!
//! SCALABILITY
//! ===========
//!
//! Policies and rules are represented by dynamically sized collections.
//!
//! This module does not impose a universal maximum on:
//!
//! - policies;
//! - rules;
//! - principals;
//! - actions;
//! - resources;
//! - attributes;
//! - conditions;
//! - security domains;
//! - hardware targets;
//! - computing domains.
//!
//! Evaluation budgets are optional and explicitly configurable.
//! Actual scalability remains dependent on available memory, processing
//! capacity and configured operational budgets.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

// ============================================================================
// 1. ERRORS
// ============================================================================

/// Errors encountered while constructing policy representations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyError {
    EmptyIdentifier {
        kind: &'static str,
    },

    InvalidIdentifier {
        kind: &'static str,
        value: String,
    },

    DuplicatePolicy {
        id: String,
    },

    DuplicateRule {
        id: String,
    },

    InvalidPolicy {
        message: String,
    },
}

impl fmt::Display for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { kind } => {
                write!(f, "{kind} identifier cannot be empty")
            }

            Self::InvalidIdentifier { kind, value } => {
                write!(f, "invalid {kind} identifier: {value}")
            }

            Self::DuplicatePolicy { id } => {
                write!(f, "duplicate policy identifier: {id}")
            }

            Self::DuplicateRule { id } => {
                write!(f, "duplicate policy rule identifier: {id}")
            }

            Self::InvalidPolicy { message } => {
                write!(f, "invalid security policy: {message}")
            }
        }
    }
}

impl std::error::Error for PolicyError {}

/// Fail-closed identifier validation.
///
/// No arbitrary maximum identifier length is imposed.
/// Invalid control characters are rejected.
fn validate_identifier(
    value: &str,
    kind: &'static str,
) -> Result<String, PolicyError> {
    if value.trim().is_empty() {
        return Err(PolicyError::EmptyIdentifier { kind });
    }

    if value.chars().any(char::is_control) {
        return Err(PolicyError::InvalidIdentifier {
            kind,
            value: value.to_string(),
        });
    }

    // Preserve the original identifier.
    // Case normalization could incorrectly merge distinct identities.
    Ok(value.to_string())
}

// ============================================================================
// 2. IDENTIFIERS
// ============================================================================

/// Globally meaningful policy identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyId(String);

impl PolicyId {
    pub fn new(value: impl Into<String>) -> Result<Self, PolicyError> {
        Ok(Self(validate_identifier(
            &value.into(),
            "policy",
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PolicyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Globally meaningful policy rule identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuleId(String);

impl RuleId {
    pub fn new(value: impl Into<String>) -> Result<Self, PolicyError> {
        Ok(Self(validate_identifier(
            &value.into(),
            "rule",
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Qualified attribute path.
///
/// Examples:
///
///     identity.principal
///     execution.domain
///     security.trust
///     resource.classification
///     provenance.verified
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttributePath(String);

impl AttributePath {
    pub fn new(value: impl Into<String>) -> Result<Self, PolicyError> {
        Ok(Self(validate_identifier(
            &value.into(),
            "attribute path",
        )?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// 3. POLICY VALUES
// ============================================================================

/// Typed values available to policy conditions.
///
/// The semantic adapter is responsible for converting canonical semantic
/// values into this representation.
///
/// Complex semantic values must not be silently converted into strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeValue {
    Null,
    Boolean(bool),
    Integer(i128),
    Text(String),
}

impl AttributeValue {
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    fn compare(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Integer(a), Self::Integer(b)) => {
                Some(a.cmp(b))
            }

            (Self::Text(a), Self::Text(b)) => {
                Some(a.cmp(b))
            }

            _ => None,
        }
    }
}

/// Attributes supplied by semantic analysis or a trusted evaluation
/// environment.
///
/// BTreeMap provides deterministic ordering.
pub type PolicyAttributes = BTreeMap<AttributePath, AttributeValue>;

// ============================================================================
// 4. POLICY SUBJECTS, ACTIONS AND RESOURCES
// ============================================================================

/// Exact matching or unrestricted matching.
///
/// Matching is deliberately explicit. This implementation does not
/// introduce unsafe wildcard or implicit substring matching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selector {
    Any,
    Exact(String),
}

impl Selector {
    pub fn exact(
        value: impl Into<String>,
    ) -> Result<Self, PolicyError> {
        Ok(Self::Exact(validate_identifier(
            &value.into(),
            "selector",
        )?))
    }

    fn matches(&self, value: &str) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(expected) => expected == value,
        }
    }
}

// ============================================================================
// 5. POLICY CONDITIONS
// ============================================================================

/// Supported comparison operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

/// Declarative policy condition.
///
/// This is an evaluation representation, not a second source-language
/// expression grammar.
///
/// The frontend/semantic adapter must translate validated policy expressions
/// into these nodes, or reject unsupported expressions explicitly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    Constant(bool),

    Exists {
        attribute: AttributePath,
    },

    Compare {
        attribute: AttributePath,
        operator: ComparisonOperator,
        expected: AttributeValue,
    },

    All(Vec<Condition>),

    Any(Vec<Condition>),

    Not(Box<Condition>),
}

impl Condition {
    pub fn all(conditions: Vec<Self>) -> Self {
        Self::All(conditions)
    }

    pub fn any(conditions: Vec<Self>) -> Self {
        Self::Any(conditions)
    }

    pub fn negate(condition: Self) -> Self {
        Self::Not(Box::new(condition))
    }
}

/// Three-valued logic.
///
/// Unknown is essential for fail-closed evaluation when an attribute is
/// missing or a comparison cannot be evaluated safely.
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
// 6. EVALUATION LIMITS
// ============================================================================

/// Explicit operational evaluation budgets.
///
/// None means that this particular budget is not configured.
///
/// These are operational safeguards, not language-level or hardware limits.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PolicyEvaluationLimits {
    pub max_rule_evaluations: Option<usize>,
    pub max_condition_nodes: Option<usize>,
}

impl PolicyEvaluationLimits {
    pub fn unlimited() -> Self {
        Self::default()
    }

    pub fn with_rule_budget(mut self, budget: usize) -> Self {
        self.max_rule_evaluations = Some(budget);
        self
    }

    pub fn with_condition_budget(mut self, budget: usize) -> Self {
        self.max_condition_nodes = Some(budget);
        self
    }
}

// ============================================================================
// 7. POLICY RULE
// ============================================================================

/// Effect associated with a matching policy rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyEffect {
    Permit,
    Deny,
}

/// A complete declarative policy rule.
///
/// Rule identity is independent of the target hardware and computing domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyRule {
    id: RuleId,

    effect: PolicyEffect,

    subject: Selector,

    action: Selector,

    resource: Selector,

    condition: Option<Condition>,
}

impl PolicyRule {
    pub fn new(
        id: RuleId,
        effect: PolicyEffect,
        subject: Selector,
        action: Selector,
        resource: Selector,
        condition: Option<Condition>,
    ) -> Self {
        Self {
            id,
            effect,
            subject,
            action,
            resource,
            condition,
        }
    }

    pub fn id(&self) -> &RuleId {
        &self.id
    }

    pub fn effect(&self) -> PolicyEffect {
        self.effect
    }
}

// ============================================================================
// 8. POLICY
// ============================================================================

/// Validated policy representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    id: PolicyId,
    rules: Vec<PolicyRule>,
}

impl Policy {
    pub fn new(
        id: PolicyId,
        rules: Vec<PolicyRule>,
    ) -> Result<Self, PolicyError> {
        let mut identifiers = BTreeSet::new();

        for rule in &rules {
            if !identifiers.insert(rule.id.clone()) {
                return Err(PolicyError::DuplicateRule {
                    id: rule.id.to_string(),
                });
            }
        }

        Ok(Self {
            id,
            rules,
        })
    }

    pub fn id(&self) -> &PolicyId {
        &self.id
    }

    pub fn rules(&self) -> &[PolicyRule] {
        &self.rules
    }
}

// ============================================================================
// 9. POLICY COMBINATION
// ============================================================================

/// Determines how multiple applicable policy rules are combined.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombiningAlgorithm {
    /// A deny decision takes precedence.
    DenyOverrides,

    /// A permit decision takes precedence.
    PermitOverrides,

    /// The first applicable rule determines the outcome.
    ///
    /// Policy ordering is significant and preserved.
    FirstApplicable,

    /// Exactly one applicable rule is required.
    OnlyOneApplicable,
}

// ============================================================================
// 10. POLICY SET
// ============================================================================

/// Immutable collection of policies.
///
/// A PolicySet is constructed and validated before evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicySet {
    policies: Vec<Policy>,
    combining: CombiningAlgorithm,
    limits: PolicyEvaluationLimits,
}

impl PolicySet {
    pub fn new(
        policies: Vec<Policy>,
        combining: CombiningAlgorithm,
        limits: PolicyEvaluationLimits,
    ) -> Result<Self, PolicyError> {
        let mut policy_ids = BTreeSet::new();
        let mut rule_ids = BTreeSet::new();

        for policy in &policies {
            if !policy_ids.insert(policy.id.clone()) {
                return Err(PolicyError::DuplicatePolicy {
                    id: policy.id.to_string(),
                });
            }

            for rule in &policy.rules {
                if !rule_ids.insert(rule.id.clone()) {
                    return Err(PolicyError::DuplicateRule {
                        id: rule.id.to_string(),
                    });
                }
            }
        }

        Ok(Self {
            policies,
            combining,
            limits,
        })
    }

    pub fn policies(&self) -> &[Policy] {
        &self.policies
    }

    pub fn combining_algorithm(&self) -> CombiningAlgorithm {
        self.combining
    }

    pub fn limits(&self) -> &PolicyEvaluationLimits {
        &self.limits
    }
}

// ============================================================================
// 11. POLICY REQUEST
// ============================================================================

/// One policy evaluation request.
///
/// These fields represent a semantic request, not a physical device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyRequest {
    pub subject: String,
    pub action: String,
    pub resource: String,
    pub attributes: PolicyAttributes,
}

impl PolicyRequest {
    pub fn new(
        subject: impl Into<String>,
        action: impl Into<String>,
        resource: impl Into<String>,
    ) -> Result<Self, PolicyError> {
        Ok(Self {
            subject: validate_identifier(
                &subject.into(),
                "subject",
            )?,

            action: validate_identifier(
                &action.into(),
                "action",
            )?,

            resource: validate_identifier(
                &resource.into(),
                "resource",
            )?,

            attributes: BTreeMap::new(),
        })
    }

    pub fn with_attribute(
        mut self,
        path: AttributePath,
        value: AttributeValue,
    ) -> Self {
        self.attributes.insert(path, value);
        self
    }
}

// ============================================================================
// 12. EVALUATION RESULTS
// ============================================================================

/// Final policy evaluation decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecisionKind {
    /// An explicit permit was established.
    Permit,

    /// An explicit denial was established.
    Deny,

    /// No policy rule applied.
    NotApplicable,

    /// Evaluation could not establish a reliable decision.
    ///
    /// This includes missing required attributes, incompatible comparisons
    /// and exhausted evaluation budgets.
    Indeterminate,
}

impl PolicyDecisionKind {
    /// Only an explicit permit is authorization to proceed.
    ///
    /// Deny, NotApplicable and Indeterminate must never be treated as permit.
    pub fn is_permitted(self) -> bool {
        matches!(self, Self::Permit)
    }

    pub fn is_denied(self) -> bool {
        matches!(self, Self::Deny)
    }

    pub fn is_indeterminate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }
}

/// Evaluation issue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluationIssue {
    MissingAttribute {
        attribute: String,
    },

    IncompatibleAttributeTypes {
        attribute: String,
    },

    RuleBudgetExceeded,

    ConditionBudgetExceeded,
}

/// Complete, deterministic evaluation result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyDecision {
    pub decision: PolicyDecisionKind,

    pub matched_rules: Vec<RuleId>,

    pub issues: Vec<EvaluationIssue>,
}

impl PolicyDecision {
    fn new(
        decision: PolicyDecisionKind,
        matched_rules: Vec<RuleId>,
        issues: Vec<EvaluationIssue>,
    ) -> Self {
        Self {
            decision,
            matched_rules,
            issues,
        }
    }

    pub fn is_permitted(&self) -> bool {
        self.decision.is_permitted()
    }
}

// ============================================================================
// 13. CONDITION EVALUATION
// ============================================================================

enum EvaluationFrame<'a> {
    Visit(&'a Condition),
    ReduceAll(usize),
    ReduceAny(usize),
    ReduceNot,
}

/// Evaluates condition trees iteratively.
///
/// An explicit stack avoids relying on recursive call depth for deeply nested
/// condition trees.
fn evaluate_condition(
    condition: &Condition,
    attributes: &PolicyAttributes,
    budget: Option<usize>,
) -> Result<(Truth, Vec<EvaluationIssue>), EvaluationIssue> {
    let mut frames = vec![
        EvaluationFrame::Visit(condition),
    ];

    let mut values: Vec<Truth> = Vec::new();

    let mut issues = Vec::new();

    let mut visited = 0usize;

    while let Some(frame) = frames.pop() {
        match frame {
            EvaluationFrame::Visit(node) => {
                if let Some(maximum) = budget {
                    if visited >= maximum {
                        return Err(
                            EvaluationIssue::ConditionBudgetExceeded
                        );
                    }
                }

                visited = visited.saturating_add(1);

                match node {
                    Condition::Constant(value) => {
                        values.push(if *value {
                            Truth::True
                        } else {
                            Truth::False
                        });
                    }

                    Condition::Exists { attribute } => {
                        values.push(
                            if attributes.contains_key(attribute) {
                                Truth::True
                            } else {
                                Truth::False
                            }
                        );
                    }

                    Condition::Compare {
                        attribute,
                        operator,
                        expected,
                    } => {
                        match attributes.get(attribute) {
                            None => {
                                issues.push(
                                    EvaluationIssue::MissingAttribute {
                                        attribute: attribute
                                            .as_str()
                                            .to_string(),
                                    },
                                );

                                values.push(Truth::Unknown);
                            }

                            Some(actual) => {
                                let result = compare_attribute(
                                    actual,
                                    expected,
                                    *operator,
                                );

                                match result {
                                    Some(value) => {
                                        values.push(if value {
                                            Truth::True
                                        } else {
                                            Truth::False
                                        });
                                    }

                                    None => {
                                        issues.push(
                                            EvaluationIssue::
                                            IncompatibleAttributeTypes {
                                                attribute: attribute
                                                    .as_str()
                                                    .to_string(),
                                            },
                                        );

                                        values.push(Truth::Unknown);
                                    }
                                }
                            }
                        }
                    }

                    Condition::All(children) => {
                        if children.is_empty() {
                            values.push(Truth::True);
                        } else {
                            frames.push(
                                EvaluationFrame::ReduceAll(
                                    children.len(),
                                ),
                            );

                            for child in children.iter().rev() {
                                frames.push(
                                    EvaluationFrame::Visit(child),
                                );
                            }
                        }
                    }

                    Condition::Any(children) => {
                        if children.is_empty() {
                            values.push(Truth::False);
                        } else {
                            frames.push(
                                EvaluationFrame::ReduceAny(
                                    children.len(),
                                ),
                            );

                            for child in children.iter().rev() {
                                frames.push(
                                    EvaluationFrame::Visit(child),
                                );
                            }
                        }
                    }

                    Condition::Not(child) => {
                        frames.push(
                            EvaluationFrame::ReduceNot,
                        );

                        frames.push(
                            EvaluationFrame::Visit(child),
                        );
                    }
                }
            }

            EvaluationFrame::ReduceAll(count) => {
                let start = values.len() - count;

                let result = Truth::all(
                    values.drain(start..),
                );

                values.push(result);
            }

            EvaluationFrame::ReduceAny(count) => {
                let start = values.len() - count;

                let result = Truth::any(
                    values.drain(start..),
                );

                values.push(result);
            }

            EvaluationFrame::ReduceNot => {
                let value = values.pop().unwrap_or(Truth::Unknown);

                values.push(value.negate());
            }
        }
    }

    Ok((
        values.pop().unwrap_or(Truth::Unknown),
        issues,
    ))
}

/// Performs a typed comparison.
///
/// Unsupported cross-type comparisons return None and therefore become
/// Indeterminate instead of silently succeeding or coercing values.
fn compare_attribute(
    actual: &AttributeValue,
    expected: &AttributeValue,
    operator: ComparisonOperator,
) -> Option<bool> {
    if operator == ComparisonOperator::Equal {
        return Some(actual == expected);
    }

    if operator == ComparisonOperator::NotEqual {
        return Some(actual != expected);
    }

    let ordering = actual.compare(expected)?;

    Some(match operator {
        ComparisonOperator::Equal => ordering == Ordering::Equal,

        ComparisonOperator::NotEqual => ordering != Ordering::Equal,

        ComparisonOperator::GreaterThan => ordering == Ordering::Greater,

        ComparisonOperator::GreaterThanOrEqual => {
            ordering != Ordering::Less
        }

        ComparisonOperator::LessThan => ordering == Ordering::Less,

        ComparisonOperator::LessThanOrEqual => {
            ordering != Ordering::Greater
        }
    })
}

// ============================================================================
// 14. RULE EVALUATION
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuleOutcome {
    Permit,
    Deny,
    NotApplicable,
    Indeterminate,
}

fn evaluate_rule(
    rule: &PolicyRule,
    request: &PolicyRequest,
    limits: &PolicyEvaluationLimits,
    issues: &mut Vec<EvaluationIssue>,
) -> RuleOutcome {
    if !rule.subject.matches(&request.subject) {
        return RuleOutcome::NotApplicable;
    }

    if !rule.action.matches(&request.action) {
        return RuleOutcome::NotApplicable;
    }

    if !rule.resource.matches(&request.resource) {
        return RuleOutcome::NotApplicable;
    }

    if let Some(condition) = &rule.condition {
        match evaluate_condition(
            condition,
            &request.attributes,
            limits.max_condition_nodes,
        ) {
            Ok((Truth::True, condition_issues)) => {
                issues.extend(condition_issues);
            }

            Ok((Truth::False, condition_issues)) => {
                issues.extend(condition_issues);
                return RuleOutcome::NotApplicable;
            }

            Ok((Truth::Unknown, condition_issues)) => {
                issues.extend(condition_issues);
                return RuleOutcome::Indeterminate;
            }

            Err(issue) => {
                issues.push(issue);
                return RuleOutcome::Indeterminate;
            }
        }
    }

    match rule.effect {
        PolicyEffect::Permit => RuleOutcome::Permit,
        PolicyEffect::Deny => RuleOutcome::Deny,
    }
}

// ============================================================================
// 15. POLICY COMBINATION
// ============================================================================

fn combine(
    algorithm: CombiningAlgorithm,
    outcomes: &[RuleOutcome],
) -> PolicyDecisionKind {
    let applicable = outcomes.iter().any(|outcome| {
        !matches!(outcome, RuleOutcome::NotApplicable)
    });

    if !applicable {
        return PolicyDecisionKind::NotApplicable;
    }

    match algorithm {
        CombiningAlgorithm::DenyOverrides => {
            if outcomes.contains(&RuleOutcome::Deny) {
                PolicyDecisionKind::Deny
            } else if outcomes.contains(&RuleOutcome::Indeterminate) {
                PolicyDecisionKind::Indeterminate
            } else if outcomes.contains(&RuleOutcome::Permit) {
                PolicyDecisionKind::Permit
            } else {
                PolicyDecisionKind::NotApplicable
            }
        }

        CombiningAlgorithm::PermitOverrides => {
            if outcomes.contains(&RuleOutcome::Permit) {
                PolicyDecisionKind::Permit
            } else if outcomes.contains(&RuleOutcome::Indeterminate) {
                PolicyDecisionKind::Indeterminate
            } else if outcomes.contains(&RuleOutcome::Deny) {
                PolicyDecisionKind::Deny
            } else {
                PolicyDecisionKind::NotApplicable
            }
        }

        CombiningAlgorithm::FirstApplicable => {
            match outcomes.iter().find(|outcome| {
                !matches!(outcome, RuleOutcome::NotApplicable)
            }) {
                Some(RuleOutcome::Permit) => {
                    PolicyDecisionKind::Permit
                }

                Some(RuleOutcome::Deny) => {
                    PolicyDecisionKind::Deny
                }

                Some(RuleOutcome::Indeterminate) => {
                    PolicyDecisionKind::Indeterminate
                }

                _ => PolicyDecisionKind::NotApplicable,
            }
        }

        CombiningAlgorithm::OnlyOneApplicable => {
            let count = outcomes.iter().filter(|outcome| {
                !matches!(outcome, RuleOutcome::NotApplicable)
            }).count();

            if count != 1 {
                return PolicyDecisionKind::Indeterminate;
            }

            match outcomes.iter().find(|outcome| {
                !matches!(outcome, RuleOutcome::NotApplicable)
            }) {
                Some(RuleOutcome::Permit) => {
                    PolicyDecisionKind::Permit
                }

                Some(RuleOutcome::Deny) => {
                    PolicyDecisionKind::Deny
                }

                Some(RuleOutcome::Indeterminate) => {
                    PolicyDecisionKind::Indeterminate
                }

                _ => PolicyDecisionKind::NotApplicable,
            }
        }
    }
}

// ============================================================================
// 16. POLICY ENGINE
// ============================================================================

/// Stateless policy evaluation engine.
///
/// PolicySet instances are immutable and can be shared between compilation
/// workers without requiring mutable global policy state.
#[derive(Debug, Clone, Copy, Default)]
pub struct PolicyEvaluator;

impl PolicyEvaluator {
    pub fn new() -> Self {
        Self
    }

    /// Evaluate one request against one validated policy set.
    ///
    /// Security invariant:
    ///
    /// Only PolicyDecisionKind::Permit authorizes the compiler to proceed
    /// past this policy gate.
    ///
    /// NotApplicable and Indeterminate are never successful authorization.
    pub fn evaluate(
        &self,
        policy_set: &PolicySet,
        request: &PolicyRequest,
    ) -> PolicyDecision {
        let mut outcomes = Vec::new();

        let mut matched_rules = Vec::new();

        let mut issues = Vec::new();

        let mut evaluations = 0usize;

        for policy in &policy_set.policies {
            for rule in &policy.rules {
                if let Some(maximum) =
                    policy_set.limits.max_rule_evaluations
                {
                    if evaluations >= maximum {
                        issues.push(
                            EvaluationIssue::RuleBudgetExceeded,
                        );

                        return PolicyDecision::new(
                            PolicyDecisionKind::Indeterminate,
                            matched_rules,
                            issues,
                        );
                    }
                }

                evaluations = evaluations.saturating_add(1);

                let outcome = evaluate_rule(
                    rule,
                    request,
                    &policy_set.limits,
                    &mut issues,
                );

                if !matches!(
                    outcome,
                    RuleOutcome::NotApplicable
                ) {
                    matched_rules.push(rule.id.clone());
                }

                // FirstApplicable must stop at the first rule whose
                // evaluation produces a meaningful result.
                if policy_set.combining
                    == CombiningAlgorithm::FirstApplicable
                    && !matches!(
                        outcome,
                        RuleOutcome::NotApplicable
                    )
                {
                    outcomes.push(outcome);

                    return PolicyDecision::new(
                        combine(
                            policy_set.combining,
                            &outcomes,
                        ),
                        matched_rules,
                        issues,
                    );
                }

                outcomes.push(outcome);
            }
        }

        let decision = combine(
            policy_set.combining,
            &outcomes,
        );

        PolicyDecision::new(
            decision,
            matched_rules,
            issues,
        )
    }
}

// ============================================================================
// 17. TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> PolicyId {
        PolicyId::new(value).unwrap()
    }

    fn rule_id(value: &str) -> RuleId {
        RuleId::new(value).unwrap()
    }

    fn selector(value: &str) -> Selector {
        Selector::exact(value).unwrap()
    }

    fn attribute(value: &str) -> AttributePath {
        AttributePath::new(value).unwrap()
    }

    fn request() -> PolicyRequest {
        PolicyRequest::new(
            "principal::service",
            "compute",
            "resource::program",
        ).unwrap()
    }

    fn make_rule(
        name: &str,
        effect: PolicyEffect,
        condition: Option<Condition>,
    ) -> PolicyRule {
        PolicyRule::new(
            rule_id(name),
            effect,
            selector("principal::service"),
            selector("compute"),
            selector("resource::program"),
            condition,
        )
    }

    fn make_set(
        rules: Vec<PolicyRule>,
        algorithm: CombiningAlgorithm,
        limits: PolicyEvaluationLimits,
    ) -> PolicySet {
        let policy = Policy::new(
            id("security::compiler"),
            rules,
        ).unwrap();

        PolicySet::new(
            vec![policy],
            algorithm,
            limits,
        ).unwrap()
    }

    #[test]
    fn explicit_permit_is_permitted() {
        let set = make_set(
            vec![
                make_rule(
                    "permit.compute",
                    PolicyEffect::Permit,
                    None,
                ),
            ],
            CombiningAlgorithm::DenyOverrides,
            PolicyEvaluationLimits::unlimited(),
        );

        let result = PolicyEvaluator::new().evaluate(
            &set,
            &request(),
        );

        assert_eq!(
            result.decision,
            PolicyDecisionKind::Permit,
        );

        assert!(result.is_permitted());
    }

    #[test]
    fn no_matching_rule_is_not_applicable() {
        let set = make_set(
            vec![],
            CombiningAlgorithm::DenyOverrides,
            PolicyEvaluationLimits::unlimited(),
        );

        let result = PolicyEvaluator::new().evaluate(
            &set,
            &request(),
        );

        assert_eq!(
            result.decision,
            PolicyDecisionKind::NotApplicable,
        );

        assert!(!result.is_permitted());
    }

    #[test]
    fn explicit_deny_overrides_permit() {
        let set = make_set(
            vec![
                make_rule(
                    "permit",
                    PolicyEffect::Permit,
                    None,
                ),
                make_rule(
                    "deny",
                    PolicyEffect::Deny,
                    None,
                ),
            ],
            CombiningAlgorithm::DenyOverrides,
            PolicyEvaluationLimits::unlimited(),
        );

        let result = PolicyEvaluator::new().evaluate(
            &set,
            &request(),
        );

        assert_eq!(
            result.decision,
            PolicyDecisionKind::Deny,
        );
    }

    #[test]
    fn permit_overrides_deny() {
        let set = make_set(
            vec![
                make_rule(
                    "deny",
                    PolicyEffect::Deny,
                    None,
                ),
                make_rule(
                    "permit",
                    PolicyEffect::Permit,
                    None,
                ),
            ],
            CombiningAlgorithm::PermitOverrides,
            PolicyEvaluationLimits::unlimited(),
        );

        let result = PolicyEvaluator::new().evaluate(
            &set,
            &request(),
        );

        assert_eq!(
            result.decision,
            PolicyDecisionKind::Permit,
        );
    }

    #[test]
    fn missing_attribute_is_indeterminate() {
        let condition = Condition::Compare {
            attribute: attribute("security.trust"),
            operator: ComparisonOperator::Equal,
            expected: AttributeValue::Boolean(true),
        };

        let set = make_set(
            vec![
                make_rule(
                    "trust.required",
                    PolicyEffect::Permit,
                    Some(condition),
                ),
            ],
            CombiningAlgorithm::DenyOverrides,
            PolicyEvaluationLimits::unlimited(),
        );

        let result = PolicyEvaluator::new().evaluate(
            &set,
            &request(),
        );

        assert_eq!(
            result.decision,
            PolicyDecisionKind::Indeterminate,
        );

        assert!(!result.is_permitted());
    }

    #[test]
    fn matching_attribute_permits() {
        let condition = Condition::Compare {
            attribute: attribute("security.trust"),
            operator: ComparisonOperator::Equal,
            expected: AttributeValue::Boolean(true),
        };

        let set = make_set(
            vec![
                make_rule(
                    "trust.required",
                    PolicyEffect::Permit,
                    Some(condition),
                ),
            ],
            CombiningAlgorithm::DenyOverrides,
            PolicyEvaluationLimits::unlimited(),
        );

        let request = request().with_attribute(
            attribute("security.trust"),
            AttributeValue::Boolean(true),
        );

        let result = PolicyEvaluator::new().evaluate(
            &set,
            &request,
        );

        assert_eq!(
            result.decision,
            PolicyDecisionKind::Permit,
        );
    }

    #[test]
    fn incompatible_comparison_fails_closed() {
        let condition = Condition::Compare {
            attribute: attribute("security.trust"),
            operator: ComparisonOperator::GreaterThan,
            expected: AttributeValue::Boolean(true),
        };

        let set = make_set(
            vec![
                make_rule(
                    "invalid.comparison",
                    PolicyEffect::Permit,
                    Some(condition),
                ),
            ],
            CombiningAlgorithm::DenyOverrides,
            PolicyEvaluationLimits::unlimited(),
        );

        let request = request().with_attribute(
            attribute("security.trust"),
            AttributeValue::Boolean(false),
        );

        let result = PolicyEvaluator::new().evaluate(
            &set,
            &request,
        );

        assert_eq!(
            result.decision,
            PolicyDecisionKind::Indeterminate,
        );
    }

    #[test]
    fn only_one_applicable_detects_conflict() {
        let set = make_set(
            vec![
                make_rule(
                    "permit",
                    PolicyEffect::Permit,
                    None,
                ),
                make_rule(
                    "deny",
                    PolicyEffect::Deny,
                    None,
                ),
            ],
            CombiningAlgorithm::OnlyOneApplicable,
            PolicyEvaluationLimits::unlimited(),
        );

        let result = PolicyEvaluator::new().evaluate(
            &set,
            &request(),
        );

        assert_eq!(
            result.decision,
            PolicyDecisionKind::Indeterminate,
        );
    }

    #[test]
    fn first_applicable_preserves_order() {
        let set = make_set(
            vec![
                make_rule(
                    "first",
                    PolicyEffect::Deny,
                    None,
                ),
                make_rule(
                    "second",
                    PolicyEffect::Permit,
                    None,
                ),
            ],
            CombiningAlgorithm::FirstApplicable,
            PolicyEvaluationLimits::unlimited(),
        );

        let result = PolicyEvaluator::new().evaluate(
            &set,
            &request(),
        );

        assert_eq!(
            result.decision,
            PolicyDecisionKind::Deny,
        );

        assert_eq!(
            result.matched_rules.len(),
            1,
        );
    }

    #[test]
    fn evaluation_budget_exhaustion_fails_closed() {
        let set = make_set(
            vec![
                make_rule(
                    "first",
                    PolicyEffect::Permit,
                    None,
                ),
                make_rule(
                    "second",
                    PolicyEffect::Permit,
                    None,
                ),
            ],
            CombiningAlgorithm::DenyOverrides,
            PolicyEvaluationLimits::unlimited()
                .with_rule_budget(1),
        );

        let result = PolicyEvaluator::new().evaluate(
            &set,
            &request(),
        );

        assert_eq!(
            result.decision,
            PolicyDecisionKind::Indeterminate,
        );

        assert!(
            result.issues.contains(
                &EvaluationIssue::RuleBudgetExceeded
            )
        );
    }

    #[test]
    fn condition_budget_exhaustion_fails_closed() {
        let condition = Condition::all(vec![
            Condition::Constant(true),
            Condition::Constant(true),
        ]);

        let set = make_set(
            vec![
                make_rule(
                    "budget",
                    PolicyEffect::Permit,
                    Some(condition),
                ),
            ],
            CombiningAlgorithm::DenyOverrides,
            PolicyEvaluationLimits::unlimited()
                .with_condition_budget(1),
        );

        let result = PolicyEvaluator::new().evaluate(
            &set,
            &request(),
        );

        assert_eq!(
            result.decision,
            PolicyDecisionKind::Indeterminate,
        );
    }

    #[test]
    fn condition_logic_handles_unknown_values() {
        let condition = Condition::any(vec![
            Condition::Compare {
                attribute: attribute("security.missing"),
                operator: ComparisonOperator::Equal,
                expected: AttributeValue::Boolean(true),
            },
            Condition::Constant(true),
        ]);

        let set = make_set(
            vec![
                make_rule(
                    "unknown.or.true",
                    PolicyEffect::Permit,
                    Some(condition),
                ),
            ],
            CombiningAlgorithm::DenyOverrides,
            PolicyEvaluationLimits::unlimited(),
        );

        let result = PolicyEvaluator::new().evaluate(
            &set,
            &request(),
        );

        assert_eq!(
            result.decision,
            PolicyDecisionKind::Permit,
        );
    }

    #[test]
    fn duplicate_policy_identifiers_are_rejected() {
        let policy_a = Policy::new(
            id("security::same"),
            vec![],
        ).unwrap();

        let policy_b = Policy::new(
            id("security::same"),
            vec![],
        ).unwrap();

        assert!(
            matches!(
                PolicySet::new(
                    vec![policy_a, policy_b],
                    CombiningAlgorithm::DenyOverrides,
                    PolicyEvaluationLimits::unlimited(),
                ),
                Err(PolicyError::DuplicatePolicy { .. })
            )
        );
    }

    #[test]
    fn empty_identifiers_are_rejected() {
        assert!(
            PolicyId::new("").is_err()
        );

        assert!(
            RuleId::new("   ").is_err()
        );

        assert!(
            AttributePath::new("").is_err()
        );
    }

    #[test]
    fn identifiers_preserve_case_and_identity() {
        let lower = PolicyId::new("security::admin").unwrap();
        let upper = PolicyId::new("security::ADMIN").unwrap();

        assert_ne!(lower, upper);
    }

    #[test]
    fn deeply_nested_conditions_use_explicit_stack() {
        let mut condition = Condition::Constant(true);

        for _ in 0..512 {
            condition = Condition::negate(
                Condition::negate(condition),
            );
        }

        let set = make_set(
            vec![
                make_rule(
                    "nested",
                    PolicyEffect::Permit,
                    Some(condition),
                ),
            ],
            CombiningAlgorithm::DenyOverrides,
            PolicyEvaluationLimits::unlimited(),
        );

        let result = PolicyEvaluator::new().evaluate(
            &set,
            &request(),
        );

        assert_eq!(
            result.decision,
            PolicyDecisionKind::Permit,
        );
    }
}
