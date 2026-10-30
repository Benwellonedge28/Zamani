
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Distributed Computing — Runtime Security Policy Boundary.
//!
//! Path: src/distributed/security/policy.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Provides a configurable runtime policy-decision boundary for distributed
//! Zamani operations.
//!
//! This module supports:
//!
//! - authenticated policy subjects;
//! - open-ended policy identifiers;
//! - action/resource-scoped policy requests;
//! - evaluation attributes;
//! - pluggable policy engines;
//! - pluggable policy conditions;
//! - explicit policy-combining behavior;
//! - default-deny enforcement;
//! - explicit indeterminate outcomes;
//! - policy obligations;
//! - request-scoped enforcement permits.
//!
//! # Architectural ownership
//!
//! This module owns runtime policy evaluation interfaces and enforcement
//! decisions for the distributed security subsystem.
//!
//! It does NOT own:
//!
//! - source-language policy grammar;
//! - source-language authorization grammar;
//! - AST construction;
//! - semantic policy validation;
//! - expression parsing or evaluation;
//! - identity declaration syntax;
//! - cryptographic identity construction;
//! - peer authentication handshakes;
//! - trust establishment or trust-anchor management;
//! - credential storage;
//! - key generation, distribution, or revocation;
//! - permission declaration syntax;
//! - capability discovery;
//! - physical resource discovery;
//! - transport implementation;
//! - sandbox implementation;
//! - runtime execution;
//! - scheduling or placement;
//! - hardware or domain-specific IR.
//!
//! # Required security boundary
//!
//! Policy requests require an AuthenticatedSubject.
//!
//! AuthenticatedSubject has no public constructor. The authentication
//! subsystem must construct it only after successfully verifying the
//! identity proof and the applicable authentication protocol.
//!
//! A PeerIdentity document, discovered participant, endpoint, hostname,
//! process ID, or public key alone is not sufficient.
//!
//! # Default-deny
//!
//! A policy request is permitted only when an applicable policy explicitly
//! returns Permit.
//!
//! Deny, NotApplicable, Indeterminate, and evaluation errors never grant
//! permission.
//!
//! # Obligations
//!
//! A Permit may carry obligations. The PolicyEnforcer must successfully
//! discharge every obligation through a configured obligation handler before
//! it returns a PolicyPermit.
//!
//! If obligations exist and no handler is configured, enforcement fails.
//!
//! # Scalability
//!
//! No fixed number of policies, rules, subjects, actions, resources,
//! attributes, or obligations is imposed.
//!
//! Collections grow according to actual memory and configured operational
//! budgets. The implementation does not define universal language limits.
//!
//! # Integration
//!
//! - security/identity.rs supplies PrincipalId and PeerIdentity.
//! - security/authentication.rs must create AuthenticatedSubject after
//!   successful peer authentication.
//! - security/authorization.rs consumes PolicyEnforcer and PolicyPermit
//!   when enforcing protected operations.
//! - security/mod.rs exports this module.
//! - distributed/error.rs supplies DistributedError and DistributedResult.
//! - grammar/security/authorization.g4 supplies source authorization intent.
//! - grammar/security/policies.g4 composes security policy syntax.
//! - grammar/policies/policy.g4 remains the universal policy grammar.
//! - grammar/spec/security.md remains the normative security specification.
//! - runtime and transport consumers must enforce policy at the actual
//!   protected-operation boundary.
//!
//! No source-language policy syntax is parsed or redefined here.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use serde_json::Value;

use crate::distributed::error::{
    DistributedError,
    DistributedResult,
};

use crate::distributed::security::identity::{
    PeerIdentity,
    PrincipalId,
};

// ============================================================================
// Policy identifiers
// ============================================================================

/// Open-ended identifier used for runtime policy, rule, action, resource,
/// obligation, and authentication-method references.
///
/// This is not a source-language parser or semantic symbol table.
///
/// The frontend and semantic layer remain responsible for resolving
/// source-language qualified names and references.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PolicyIdentifier(String);

impl PolicyIdentifier {
    /// Construct a validated policy identifier.
    ///
    /// Identifiers:
    ///
    /// - must not be empty;
    /// - must not contain surrounding whitespace;
    /// - must not contain whitespace;
    /// - must not contain control characters.
    ///
    /// No arbitrary maximum length is imposed.
    pub fn new(value: impl Into<String>) -> DistributedResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(DistributedError::invalid_input(
                "policy identifier cannot be empty",
            ));
        }

        if value.trim() != value {
            return Err(DistributedError::invalid_input(
                "policy identifier cannot contain surrounding whitespace",
            ));
        }

        if value.chars().any(|character| {
            character.is_whitespace() || character.is_control()
        }) {
            return Err(DistributedError::invalid_input(
                "policy identifier cannot contain whitespace or control characters",
            ));
        }

        Ok(Self(value))
    }

    /// Return the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the identifier and return its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for PolicyIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Authenticated subject
// ============================================================================

/// A principal whose identity has already been authenticated.
///
/// This type deliberately has no public constructor.
///
/// The authentication module must call
/// `AuthenticatedSubject::from_verified_identity` only after:
///
/// 1. validating the received identity document;
/// 2. verifying the identity proof against a fresh challenge;
/// 3. applying the authentication protocol's replay protections;
/// 4. establishing that the authentication exchange succeeded.
///
/// This type is not a trust decision and does not imply authorization.
///
/// Trust evaluation and authorization remain separate operations.
#[derive(Clone)]
pub struct AuthenticatedSubject {
    principal: PrincipalId,
    public_key_fingerprint: String,
    authentication_method: PolicyIdentifier,
}

impl AuthenticatedSubject {
    /// Construct an authenticated subject from an identity that has already
    /// passed the authentication protocol.
    ///
    /// This is crate-private so external callers cannot turn a discovered
    /// public identity into an authenticated subject.
    ///
    /// The authentication module is responsible for ensuring that this
    /// function is called only after successful cryptographic verification.
    pub(crate) fn from_verified_identity(
        identity: &PeerIdentity,
        authentication_method: PolicyIdentifier,
    ) -> DistributedResult<Self> {
        identity.validate()?;

        Ok(Self {
            principal: identity.principal().clone(),
            public_key_fingerprint: identity.fingerprint().to_hex(),
            authentication_method,
        })
    }

    /// Return the authenticated principal.
    #[must_use]
    pub fn principal(&self) -> &PrincipalId {
        &self.principal
    }

    /// Return the public-key fingerprint associated with authentication.
    #[must_use]
    pub fn public_key_fingerprint(&self) -> &str {
        &self.public_key_fingerprint
    }

    /// Return the authentication method identifier.
    #[must_use]
    pub fn authentication_method(&self) -> &PolicyIdentifier {
        &self.authentication_method
    }
}

impl fmt::Debug for AuthenticatedSubject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuthenticatedSubject")
            .field("principal", &self.principal)
            .field(
                "public_key_fingerprint",
                &self.public_key_fingerprint,
            )
            .field(
                "authentication_method",
                &self.authentication_method,
            )
            .finish()
    }
}

// ============================================================================
// Subject selection
// ============================================================================

#[derive(Debug, Clone)]
enum SubjectSelectorKind {
    Any,
    Only(BTreeSet<PrincipalId>),
}

/// Selects which authenticated principals a rule applies to.
///
/// `Any` is an explicit broad selector. It must never be inferred from an
/// empty subject collection.
#[derive(Debug, Clone)]
pub struct SubjectSelector {
    kind: SubjectSelectorKind,
}

impl SubjectSelector {
    /// Match every authenticated principal.
    ///
    /// This is an explicit broad policy choice, not a default.
    #[must_use]
    pub fn any() -> Self {
        Self {
            kind: SubjectSelectorKind::Any,
        }
    }

    /// Match only the supplied principals.
    ///
    /// An empty set is rejected so that an accidentally empty selector
    /// cannot be confused with an unrestricted selector.
    pub fn only(
        principals: impl IntoIterator<Item = PrincipalId>,
    ) -> DistributedResult<Self> {
        let principals: BTreeSet<_> = principals.into_iter().collect();

        if principals.is_empty() {
            return Err(DistributedError::invalid_input(
                "a restricted subject selector must contain at least one principal",
            ));
        }

        Ok(Self {
            kind: SubjectSelectorKind::Only(principals),
        })
    }

    fn matches(&self, principal: &PrincipalId) -> bool {
        match &self.kind {
            SubjectSelectorKind::Any => true,
            SubjectSelectorKind::Only(principals) => {
                principals.contains(principal)
            }
        }
    }

    fn validate(&self) -> DistributedResult<()> {
        match &self.kind {
            SubjectSelectorKind::Any => Ok(()),
            SubjectSelectorKind::Only(principals) if principals.is_empty() => {
                Err(DistributedError::invalid_input(
                    "a restricted subject selector cannot be empty",
                ))
            }
            SubjectSelectorKind::Only(_) => Ok(()),
        }
    }
}

// ============================================================================
// Policy request
// ============================================================================

/// Runtime context supplied to a policy evaluator.
///
/// Attribute values are deliberately opaque to this module. A configured
/// condition evaluator decides how to interpret them.
///
/// Callers must not insert credentials, private keys, access tokens, secret
/// material, or unrestricted sensitive payloads into policy context.
pub type PolicyAttributes = BTreeMap<String, Value>;

/// A request to authorize one operation against one resource.
///
/// Requests are constructed from an AuthenticatedSubject, never from an
/// arbitrary principal string or an unauthenticated identity document.
#[derive(Clone)]
pub struct PolicyRequest {
    subject: AuthenticatedSubject,
    action: PolicyIdentifier,
    resource: PolicyIdentifier,
    attributes: PolicyAttributes,
}

impl PolicyRequest {
    /// Construct a policy request.
    pub fn new(
        subject: AuthenticatedSubject,
        action: PolicyIdentifier,
        resource: PolicyIdentifier,
    ) -> Self {
        Self {
            subject,
            action,
            resource,
            attributes: BTreeMap::new(),
        }
    }

    /// Return the authenticated subject.
    #[must_use]
    pub fn subject(&self) -> &AuthenticatedSubject {
        &self.subject
    }

    /// Return the requested action.
    #[must_use]
    pub fn action(&self) -> &PolicyIdentifier {
        &self.action
    }

    /// Return the requested resource.
    #[must_use]
    pub fn resource(&self) -> &PolicyIdentifier {
        &self.resource
    }

    /// Return all evaluation attributes.
    ///
    /// Attribute values may contain sensitive application data. Evaluators
    /// should inspect only the values required by their policy.
    #[must_use]
    pub fn attributes(&self) -> &PolicyAttributes {
        &self.attributes
    }

    /// Retrieve one evaluation attribute.
    #[must_use]
    pub fn attribute(&self, name: &str) -> Option<&Value> {
        self.attributes.get(name)
    }

    /// Insert or replace an evaluation attribute.
    ///
    /// Attribute names are open-ended but must be non-empty and must not
    /// contain whitespace or control characters.
    pub fn insert_attribute(
        &mut self,
        name: impl Into<String>,
        value: Value,
    ) -> DistributedResult<Option<Value>> {
        let name = PolicyIdentifier::new(name)?;

        Ok(self.attributes.insert(name.into_string(), value))
    }
}

impl fmt::Debug for PolicyRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let attribute_names: Vec<&str> = self
            .attributes
            .keys()
            .map(String::as_str)
            .collect();

        formatter
            .debug_struct("PolicyRequest")
            .field("subject", &self.subject)
            .field("action", &self.action)
            .field("resource", &self.resource)
            .field("attribute_names", &attribute_names)
            .field("attribute_values", &"[REDACTED]")
            .finish()
    }
}

// ============================================================================
// Policy outcomes
// ============================================================================

/// Effect of an applicable policy rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyEffect {
    /// The rule explicitly permits the requested operation.
    Permit,

    /// The rule explicitly denies the requested operation.
    Deny,
}

/// Result of evaluating a policy request.
///
/// Only `Permit` can lead to a PolicyPermit.
///
/// `NotApplicable` is not permission.
///
/// `Indeterminate` is not permission.
///
/// Evaluation errors are also not permission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    Permit {
        policy_id: PolicyIdentifier,
        rule_id: PolicyIdentifier,
        obligations: Vec<PolicyIdentifier>,
    },

    Deny {
        policy_id: PolicyIdentifier,
        rule_id: PolicyIdentifier,
    },

    NotApplicable {
        policy_id: PolicyIdentifier,
    },

    Indeterminate {
        policy_id: PolicyIdentifier,
        rule_id: Option<PolicyIdentifier>,
        reason_code: String,
    },
}

impl PolicyDecision {
    /// Return true only for an explicit Permit decision.
    #[must_use]
    pub fn is_permit(&self) -> bool {
        matches!(self, Self::Permit { .. })
    }
}

// ============================================================================
// Policy conditions
// ============================================================================

/// Extensible condition evaluator.
///
/// This trait deliberately does not define a second expression language.
///
/// A frontend/security semantic adapter or external policy provider can
/// implement this interface using the canonical expression representation.
///
/// Implementations must be deterministic when the policy contract requires
/// deterministic evaluation.
///
/// A condition failure must be returned as an error. It must never be
/// converted into `Ok(true)`.
pub trait PolicyCondition: Send + Sync {
    /// Evaluate a condition against a policy request.
    fn evaluate(
        &self,
        request: &PolicyRequest,
    ) -> DistributedResult<bool>;
}

/// Allow closures and function objects to implement policy conditions.
impl<F> PolicyCondition for F
where
    F: Fn(&PolicyRequest) -> DistributedResult<bool> + Send + Sync,
{
    fn evaluate(
        &self,
        request: &PolicyRequest,
    ) -> DistributedResult<bool> {
        self(request)
    }
}

// ============================================================================
// Policy rules
// ============================================================================

/// One runtime policy rule.
///
/// Rules are immutable after construction. Build a new PolicySet when policy
/// configuration changes, then replace the active provider through the
/// owning authorization/runtime lifecycle.
///
/// This avoids partially updated policy state during concurrent evaluation.
#[derive(Clone)]
pub struct PolicyRule {
    id: PolicyIdentifier,
    effect: PolicyEffect,
    subjects: SubjectSelector,
    actions: BTreeSet<PolicyIdentifier>,
    resources: BTreeSet<PolicyIdentifier>,
    condition: Option<Arc<dyn PolicyCondition>>,
    obligations: Vec<PolicyIdentifier>,
}

impl PolicyRule {
    /// Construct a rule with explicit subject, action, and resource scopes.
    ///
    /// Action and resource sets must be non-empty.
    ///
    /// Broad matching must be explicit through SubjectSelector::any().
    pub fn new(
        id: PolicyIdentifier,
        effect: PolicyEffect,
        subjects: SubjectSelector,
        actions: impl IntoIterator<Item = PolicyIdentifier>,
        resources: impl IntoIterator<Item = PolicyIdentifier>,
    ) -> DistributedResult<Self> {
        subjects.validate()?;

        let actions: BTreeSet<_> = actions.into_iter().collect();
        let resources: BTreeSet<_> = resources.into_iter().collect();

        if actions.is_empty() {
            return Err(DistributedError::invalid_input(
                "a policy rule must specify at least one action",
            ));
        }

        if resources.is_empty() {
            return Err(DistributedError::invalid_input(
                "a policy rule must specify at least one resource",
            ));
        }

        Ok(Self {
            id,
            effect,
            subjects,
            actions,
            resources,
            condition: None,
            obligations: Vec::new(),
        })
    }

    /// Attach a pluggable condition evaluator.
    #[must_use]
    pub fn with_condition(
        mut self,
        condition: Arc<dyn PolicyCondition>,
    ) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Attach obligations that must be fulfilled before enforcement succeeds.
    ///
    /// The PolicyEnforcer rejects a Permit containing obligations if no
    /// obligation handler is configured.
    #[must_use]
    pub fn with_obligations(
        mut self,
        obligations: impl IntoIterator<Item = PolicyIdentifier>,
    ) -> Self {
        self.obligations = obligations.into_iter().collect();
        self
    }

    /// Return the rule identifier.
    #[must_use]
    pub fn id(&self) -> &PolicyIdentifier {
        &self.id
    }

    /// Return the rule effect.
    #[must_use]
    pub fn effect(&self) -> PolicyEffect {
        self.effect
    }

    fn matches_scope(&self, request: &PolicyRequest) -> bool {
        self.subjects.matches(request.subject.principal())
            && self.actions.contains(&request.action)
            && self.resources.contains(&request.resource)
    }

    fn evaluate_condition(
        &self,
        request: &PolicyRequest,
    ) -> DistributedResult<bool> {
        match &self.condition {
            Some(condition) => condition.evaluate(request),
            None => Ok(true),
        }
    }
}

impl fmt::Debug for PolicyRule {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PolicyRule")
            .field("id", &self.id)
            .field("effect", &self.effect)
            .field("subjects", &self.subjects)
            .field("actions", &self.actions)
            .field("resources", &self.resources)
            .field("has_condition", &self.condition.is_some())
            .field("obligations", &self.obligations)
            .finish()
    }
}

// ============================================================================
// Policy combining
// ============================================================================

/// Rule-combination behavior.
///
/// DenyOverrides is the default and recommended algorithm.
///
/// FirstApplicable is available for deployments whose policy contract
/// explicitly defines ordered rule evaluation.
///
/// Rule order is significant only under FirstApplicable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyCombiningAlgorithm {
    /// Any applicable Deny takes precedence over Permit.
    DenyOverrides,

    /// The first applicable rule determines the result.
    FirstApplicable,
}

// ============================================================================
// Policy engine
// ============================================================================

/// Runtime policy evaluation provider.
///
/// A provider may adapt:
///
/// - a compiled security policy;
/// - an external policy decision point;
/// - a centrally managed policy service;
/// - a local immutable policy set;
/// - a future policy implementation.
///
/// Implementations must fail closed: errors and indeterminate outcomes must
/// never be represented as Permit.
///
/// The provider is trusted infrastructure and must itself be protected by
/// the authentication, integrity, and deployment controls of the runtime.
pub trait PolicyEngine: Send + Sync {
    /// Evaluate one policy request.
    fn evaluate(
        &self,
        request: &PolicyRequest,
    ) -> DistributedResult<PolicyDecision>;
}

// ============================================================================
// Local immutable policy set
// ============================================================================

/// Immutable collection of runtime policy rules.
///
/// This is a usable local policy provider, not a replacement for the
/// universal source-language policy model.
///
/// Source-level policy declarations must be validated by the frontend and
/// semantic layer before an adapter constructs this runtime representation.
#[derive(Debug, Clone)]
pub struct PolicySet {
    id: PolicyIdentifier,
    rules: Vec<PolicyRule>,
    combining_algorithm: PolicyCombiningAlgorithm,
}

impl PolicySet {
    /// Construct a policy set using DenyOverrides.
    ///
    /// Duplicate rule identifiers are rejected.
    ///
    /// An empty policy set is valid but denies every request through
    /// NotApplicable/default-deny behavior.
    pub fn new(
        id: PolicyIdentifier,
        rules: Vec<PolicyRule>,
    ) -> DistributedResult<Self> {
        let mut rule_ids = BTreeSet::new();

        for rule in &rules {
            if !rule_ids.insert(rule.id.clone()) {
                return Err(DistributedError::invalid_input(
                    "policy set contains duplicate rule identifiers",
                ));
            }
        }

        Ok(Self {
            id,
            rules,
            combining_algorithm: PolicyCombiningAlgorithm::DenyOverrides,
        })
    }

    /// Select the policy-combining algorithm explicitly.
    #[must_use]
    pub fn with_combining_algorithm(
        mut self,
        algorithm: PolicyCombiningAlgorithm,
    ) -> Self {
        self.combining_algorithm = algorithm;
        self
    }

    /// Return the policy identifier.
    #[must_use]
    pub fn id(&self) -> &PolicyIdentifier {
        &self.id
    }

    /// Return the number of configured rules.
    #[must_use]
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Return the selected combining algorithm.
    #[must_use]
    pub fn combining_algorithm(&self) -> PolicyCombiningAlgorithm {
        self.combining_algorithm
    }

    fn decision_for_rule(
        &self,
        rule: &PolicyRule,
    ) -> PolicyDecision {
        match rule.effect {
            PolicyEffect::Permit => PolicyDecision::Permit {
                policy_id: self.id.clone(),
                rule_id: rule.id.clone(),
                obligations: rule.obligations.clone(),
            },

            PolicyEffect::Deny => PolicyDecision::Deny {
                policy_id: self.id.clone(),
                rule_id: rule.id.clone(),
            },
        }
    }
}

impl PolicyEngine for PolicySet {
    fn evaluate(
        &self,
        request: &PolicyRequest,
    ) -> DistributedResult<PolicyDecision> {
        let mut first_permit: Option<PolicyDecision> = None;
        let mut first_deny: Option<PolicyDecision> = None;
        let mut first_indeterminate: Option<PolicyDecision> = None;

        for rule in &self.rules {
            if !rule.matches_scope(request) {
                continue;
            }

            match rule.evaluate_condition(request) {
                Ok(false) => continue,

                Ok(true) => {
                    let decision = self.decision_for_rule(rule);

                    if self.combining_algorithm
                        == PolicyCombiningAlgorithm::FirstApplicable
                    {
                        return Ok(decision);
                    }

                    match decision {
                        PolicyDecision::Permit { .. } => {
                            if first_permit.is_none() {
                                first_permit = Some(decision);
                            }
                        }

                        PolicyDecision::Deny { .. } => {
                            if first_deny.is_none() {
                                first_deny = Some(decision);
                            }

                            // DenyOverrides is already conclusively denied.
                            if self.combining_algorithm
                                == PolicyCombiningAlgorithm::DenyOverrides
                            {
                                return Ok(first_deny.expect(
                                    "a matching deny decision was just recorded",
                                ));
                            }
                        }

                        _ => {
                            return Err(
                                DistributedError::internal_invariant(
                                    "policy decision invariant",
                                    "a rule produced an unexpected decision variant",
                                ),
                            );
                        }
                    }
                }

                Err(error) => {
                    if first_indeterminate.is_none() {
                        first_indeterminate = Some(
                            PolicyDecision::Indeterminate {
                                policy_id: self.id.clone(),
                                rule_id: Some(rule.id.clone()),
                                reason_code: error.code().to_owned(),
                            },
                        );
                    }
                }
            }
        }

        match self.combining_algorithm {
            PolicyCombiningAlgorithm::DenyOverrides => {
                if let Some(deny) = first_deny {
                    return Ok(deny);
                }

                if let Some(indeterminate) = first_indeterminate {
                    return Ok(indeterminate);
                }

                if let Some(permit) = first_permit {
                    return Ok(permit);
                }
            }

            PolicyCombiningAlgorithm::FirstApplicable => {
                // FirstApplicable returns from the loop as soon as a rule
                // matches. Reaching this branch means no rule applied.
            }
        }

        Ok(PolicyDecision::NotApplicable {
            policy_id: self.id.clone(),
        })
    }
}

// ============================================================================
// Policy obligations
// ============================================================================

/// Executes policy obligations.
///
/// Implementations must execute the obligation according to its authoritative
/// semantic/runtime contract. Merely logging an obligation is not fulfillment
/// unless that obligation explicitly defines logging as its required action.
///
/// A failure prevents the PolicyEnforcer from returning a PolicyPermit.
pub trait PolicyObligationHandler: Send + Sync {
    /// Fulfil every obligation associated with the decision.
    fn fulfill(
        &self,
        request: &PolicyRequest,
        obligations: &[PolicyIdentifier],
    ) -> DistributedResult<()>;
}

// ============================================================================
// Enforcement permit
// ============================================================================

/// Evidence that one policy request was explicitly permitted and its
/// obligations, if any, were successfully fulfilled.
///
/// This is not:
///
/// - a cryptographic credential;
/// - a transferable token;
/// - a long-lived capability;
/// - proof of trust;
/// - proof that an operation has executed.
///
/// It must not be serialized, cached as a reusable authorization credential,
/// or reused for a different action/resource/request.
///
/// Protected operations must be authorized at their actual enforcement
/// boundary.
#[derive(Debug)]
pub struct PolicyPermit {
    principal: PrincipalId,
    action: PolicyIdentifier,
    resource: PolicyIdentifier,
    policy_id: PolicyIdentifier,
    rule_id: PolicyIdentifier,
}

impl PolicyPermit {
    /// Return the principal for which this request was permitted.
    #[must_use]
    pub fn principal(&self) -> &PrincipalId {
        &self.principal
    }

    /// Return the permitted action.
    #[must_use]
    pub fn action(&self) -> &PolicyIdentifier {
        &self.action
    }

    /// Return the protected resource.
    #[must_use]
    pub fn resource(&self) -> &PolicyIdentifier {
        &self.resource
    }

    /// Return the policy that produced the decision.
    #[must_use]
    pub fn policy_id(&self) -> &PolicyIdentifier {
        &self.policy_id
    }

    /// Return the rule that produced the decision.
    #[must_use]
    pub fn rule_id(&self) -> &PolicyIdentifier {
        &self.rule_id
    }
}

// ============================================================================
// Policy enforcer
// ============================================================================

/// Enforces decisions produced by a configured PolicyEngine.
///
/// The enforcer:
///
/// - accepts only authenticated subjects;
/// - delegates policy evaluation to an injected engine;
/// - treats all non-Permit outcomes as denial;
/// - requires obligation fulfillment before returning a permit;
/// - uses the canonical DistributedError::AuthorizationDenied variant.
///
/// It does not perform authentication, trust evaluation, or the protected
/// operation itself.
pub struct PolicyEnforcer {
    engine: Arc<dyn PolicyEngine>,
    obligation_handler: Option<Arc<dyn PolicyObligationHandler>>,
}

impl PolicyEnforcer {
    /// Construct an enforcer using a policy engine.
    ///
    /// No policy engine is silently selected. The caller must supply one.
    pub fn new(engine: Arc<dyn PolicyEngine>) -> Self {
        Self {
            engine,
            obligation_handler: None,
        }
    }

    /// Configure an obligation handler.
    ///
    /// If no handler is configured, decisions carrying obligations are
    /// rejected rather than silently ignoring those obligations.
    #[must_use]
    pub fn with_obligation_handler(
        mut self,
        handler: Arc<dyn PolicyObligationHandler>,
    ) -> Self {
        self.obligation_handler = Some(handler);
        self
    }

    /// Evaluate a request and return the raw policy decision.
    ///
    /// This method is useful for diagnostics and policy analysis. Callers
    /// protecting an operation should use `enforce`, not treat a raw
    /// decision as an enforcement permit.
    pub fn evaluate(
        &self,
        request: &PolicyRequest,
    ) -> DistributedResult<PolicyDecision> {
        self.engine.evaluate(request)
    }

    /// Enforce a request and return a request-scoped permit only when:
    ///
    /// 1. the policy engine explicitly returns Permit;
    /// 2. every associated obligation is fulfilled successfully.
    ///
    /// Every other decision is denied.
    pub fn enforce(
        &self,
        request: &PolicyRequest,
    ) -> DistributedResult<PolicyPermit> {
        let decision = self.engine.evaluate(request)?;

        match decision {
            PolicyDecision::Permit {
                policy_id,
                rule_id,
                obligations,
            } => {
                if !obligations.is_empty() {
                    let handler = self.obligation_handler.as_ref().ok_or_else(|| {
                        authorization_denied(
                            request,
                            "policy obligations have no configured handler",
                        )
                    })?;

                    handler
                        .fulfill(request, &obligations)
                        .map_err(|error| {
                            error.with_context(
                                "distributed security policy obligation fulfillment failed",
                            )
                        })?;
                }

                Ok(PolicyPermit {
                    principal: request.subject.principal().clone(),
                    action: request.action.clone(),
                    resource: request.resource.clone(),
                    policy_id,
                    rule_id,
                })
            }

            PolicyDecision::Deny { .. } => Err(authorization_denied(
                request,
                "the applicable security policy denied the operation",
            )),

            PolicyDecision::NotApplicable { .. } => {
                Err(authorization_denied(
                    request,
                    "no applicable policy explicitly permitted the operation",
                ))
            }

            PolicyDecision::Indeterminate { .. } => {
                Err(authorization_denied(
                    request,
                    "policy evaluation was indeterminate",
                ))
            }
        }
    }
}

impl fmt::Debug for PolicyEnforcer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PolicyEnforcer")
            .field("engine", &"[POLICY ENGINE]")
            .field(
                "has_obligation_handler",
                &self.obligation_handler.is_some(),
            )
            .finish()
    }
}

// ============================================================================
// Error mapping
// ============================================================================

fn authorization_denied(
    request: &PolicyRequest,
    _reason: &str,
) -> DistributedError {
    // Do not put policy context, credentials, secret values, or arbitrary
    // evaluator diagnostics into the canonical authorization error.
    //
    // The canonical error type owns the stable classification.
    DistributedError::AuthorizationDenied {
        principal: Some(request.subject.principal().to_string()),
        operation: request.action.to_string(),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn identifier(value: &str) -> PolicyIdentifier {
        PolicyIdentifier::new(value).expect("valid test identifier")
    }

    fn principal(value: &str) -> PrincipalId {
        PrincipalId::new(value).expect("valid test principal")
    }

    fn authenticated_subject(value: &str) -> AuthenticatedSubject {
        use ed25519_dalek::SigningKey;

        // This helper is test-only. Production construction is owned by
        // authentication.rs after successful challenge verification.
        let secret = [7_u8; 32];
        let signing_key = SigningKey::from_bytes(&secret);
        let identity = PeerIdentity::new(
            principal(value),
            signing_key.verifying_key().to_bytes(),
        )
        .expect("valid test identity");

        AuthenticatedSubject::from_verified_identity(
            &identity,
            identifier("authentication.ed25519.challenge"),
        )
        .expect("valid authenticated test subject")
    }

    fn request() -> PolicyRequest {
        PolicyRequest::new(
            authenticated_subject("principal::worker"),
            identifier("distributed::execute"),
            identifier("distributed::task"),
        )
    }

    fn rule(
        id: &str,
        effect: PolicyEffect,
    ) -> PolicyRule {
        PolicyRule::new(
            identifier(id),
            effect,
            SubjectSelector::any(),
            [identifier("distributed::execute")],
            [identifier("distributed::task")],
        )
        .expect("valid test rule")
    }

    #[test]
    fn policy_identifier_rejects_invalid_values() {
        assert!(PolicyIdentifier::new("").is_err());
        assert!(PolicyIdentifier::new(" leading").is_err());
        assert!(PolicyIdentifier::new("trailing ").is_err());
        assert!(PolicyIdentifier::new("contains space").is_err());
        assert!(PolicyIdentifier::new("invalid\nname").is_err());

        assert!(PolicyIdentifier::new(
            "security::distributed::execute"
        ).is_ok());
    }

    #[test]
    fn restricted_subject_selector_rejects_empty_set() {
        let selector = SubjectSelector::only(
            Vec::<PrincipalId>::new(),
        );

        assert!(selector.is_err());
    }

    #[test]
    fn policy_rule_rejects_empty_action_scope() {
        let result = PolicyRule::new(
            identifier("rule::empty-action"),
            PolicyEffect::Permit,
            SubjectSelector::any(),
            Vec::<PolicyIdentifier>::new(),
            [identifier("resource::task")],
        );

        assert!(result.is_err());
    }

    #[test]
    fn policy_rule_rejects_empty_resource_scope() {
        let result = PolicyRule::new(
            identifier("rule::empty-resource"),
            PolicyEffect::Permit,
            SubjectSelector::any(),
            [identifier("action::execute")],
            Vec::<PolicyIdentifier>::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn policy_set_rejects_duplicate_rule_ids() {
        let result = PolicySet::new(
            identifier("policy::duplicate"),
            vec![
                rule("rule::same", PolicyEffect::Permit),
                rule("rule::same", PolicyEffect::Deny),
            ],
        );

        assert!(result.is_err());
    }

    #[test]
    fn explicit_permit_produces_a_policy_permit() {
        let policy = PolicySet::new(
            identifier("policy::execution"),
            vec![rule("rule::allow-execution", PolicyEffect::Permit)],
        )
        .expect("valid policy");

        let enforcer = PolicyEnforcer::new(Arc::new(policy));

        let permit = enforcer.enforce(&request())
            .expect("explicit permit should be enforced");

        assert_eq!(
            permit.action().as_str(),
            "distributed::execute",
        );

        assert_eq!(
            permit.resource().as_str(),
            "distributed::task",
        );

        assert_eq!(
            permit.policy_id().as_str(),
            "policy::execution",
        );
    }

    #[test]
    fn no_matching_rule_is_default_deny() {
        let policy = PolicySet::new(
            identifier("policy::empty"),
            Vec::new(),
        )
        .expect("empty policy is valid and default-deny");

        let enforcer = PolicyEnforcer::new(Arc::new(policy));

        let result = enforcer.enforce(&request());

        assert!(matches!(
            result,
            Err(DistributedError::AuthorizationDenied { .. })
        ));
    }

    #[test]
    fn deny_overrides_permit() {
        let policy = PolicySet::new(
            identifier("policy::deny-overrides"),
            vec![
                rule("rule::allow", PolicyEffect::Permit),
                rule("rule::deny", PolicyEffect::Deny),
            ],
        )
        .expect("valid policy");

        let enforcer = PolicyEnforcer::new(Arc::new(policy));

        let result = enforcer.enforce(&request());

        assert!(matches!(
            result,
            Err(DistributedError::AuthorizationDenied { .. })
        ));
    }

    #[test]
    fn first_applicable_uses_declared_rule_order() {
        let policy = PolicySet::new(
            identifier("policy::first-applicable"),
            vec![
                rule("rule::first-allow", PolicyEffect::Permit),
                rule("rule::second-deny", PolicyEffect::Deny),
            ],
        )
        .expect("valid policy")
        .with_combining_algorithm(
            PolicyCombiningAlgorithm::FirstApplicable,
        );

        let enforcer = PolicyEnforcer::new(Arc::new(policy));

        assert!(enforcer.enforce(&request()).is_ok());
    }

    #[test]
    fn false_condition_does_not_grant_permission() {
        let conditional_rule = rule(
            "rule::conditional",
            PolicyEffect::Permit,
        )
        .with_condition(Arc::new(|_: &PolicyRequest| Ok(false)));

        let policy = PolicySet::new(
            identifier("policy::condition-false"),
            vec![conditional_rule],
        )
        .expect("valid policy");

        let enforcer = PolicyEnforcer::new(Arc::new(policy));

        assert!(matches!(
            enforcer.enforce(&request()),
            Err(DistributedError::AuthorizationDenied { .. })
        ));
    }

    #[test]
    fn condition_error_is_indeterminate_and_denied() {
        let conditional_rule = rule(
            "rule::condition-error",
            PolicyEffect::Permit,
        )
        .with_condition(Arc::new(|_: &PolicyRequest| {
            Err(DistributedError::invalid_input(
                "condition evaluation failed",
            ))
        }));

        let policy = PolicySet::new(
            identifier("policy::condition-error"),
            vec![conditional_rule],
        )
        .expect("valid policy");

        let decision = policy.evaluate(&request())
            .expect("policy evaluation returns an explicit decision");

        assert!(matches!(
            decision,
            PolicyDecision::Indeterminate { .. }
        ));

        let enforcer = PolicyEnforcer::new(Arc::new(policy));

        assert!(matches!(
            enforcer.enforce(&request()),
            Err(DistributedError::AuthorizationDenied { .. })
        ));
    }

    #[test]
    fn wrong_subject_does_not_match_restricted_rule() {
        let selector = SubjectSelector::only([
            principal("principal::trusted"),
        ])
        .expect("valid selector");

        let restricted_rule = PolicyRule::new(
            identifier("rule::trusted-only"),
            PolicyEffect::Permit,
            selector,
            [identifier("distributed::execute")],
            [identifier("distributed::task")],
        )
        .expect("valid rule");

        let policy = PolicySet::new(
            identifier("policy::restricted"),
            vec![restricted_rule],
        )
        .expect("valid policy");

        let enforcer = PolicyEnforcer::new(Arc::new(policy));

        assert!(matches!(
            enforcer.enforce(&request()),
            Err(DistributedError::AuthorizationDenied { .. })
        ));
    }

    #[test]
    fn obligations_require_a_handler() {
        let obligation_rule = rule(
            "rule::with-obligation",
            PolicyEffect::Permit,
        )
        .with_obligations([
            identifier("audit::record-execution"),
        ]);

        let policy = PolicySet::new(
            identifier("policy::obligations"),
            vec![obligation_rule],
        )
        .expect("valid policy");

        let enforcer = PolicyEnforcer::new(Arc::new(policy));

        assert!(matches!(
            enforcer.enforce(&request()),
            Err(DistributedError::AuthorizationDenied { .. })
        ));
    }

    struct TestObligationHandler;

    impl PolicyObligationHandler for TestObligationHandler {
        fn fulfill(
            &self,
            _request: &PolicyRequest,
            obligations: &[PolicyIdentifier],
        ) -> DistributedResult<()> {
            if obligations.iter().any(|obligation| {
                obligation.as_str() != "audit::record-execution"
            }) {
                return Err(DistributedError::invalid_input(
                    "unsupported test obligation",
                ));
            }

            Ok(())
        }
    }

    #[test]
    fn obligations_must_be_fulfilled_before_permit() {
        let obligation_rule = rule(
            "rule::with-obligation",
            PolicyEffect::Permit,
        )
        .with_obligations([
            identifier("audit::record-execution"),
        ]);

        let policy = PolicySet::new(
            identifier("policy::obligations"),
            vec![obligation_rule],
        )
        .expect("valid policy");

        let enforcer = PolicyEnforcer::new(Arc::new(policy))
            .with_obligation_handler(Arc::new(TestObligationHandler));

        assert!(enforcer.enforce(&request()).is_ok());
    }

    #[test]
    fn request_debug_does_not_expose_attribute_values() {
        let mut policy_request = request();

        policy_request
            .insert_attribute(
                "execution::secret-context",
                Value::String("sensitive-value".to_owned()),
            )
            .expect("valid attribute");

        let rendered = format!("{policy_request:?}");

        assert!(rendered.contains("execution::secret-context"));
        assert!(rendered.contains("[REDACTED]"));
        assert!(!rendered.contains("sensitive-value"));
    }

    #[test]
    fn authenticated_subject_debug_contains_no_secret_key_material() {
        let subject = authenticated_subject("principal::worker");
        let rendered = format!("{subject:?}");

        assert!(rendered.contains("principal::worker"));
        assert!(!rendered.contains("private_key"));
        assert!(!rendered.contains("secret"));
    }

    #[test]
    fn policy_set_supports_dynamically_sized_rule_collections() {
        let rules = (0..1_000)
            .map(|index| {
                rule(
                    &format!("rule::{index}"),
                    PolicyEffect::Deny,
                )
            })
            .collect();

        let policy = PolicySet::new(
            identifier("policy::large"),
            rules,
        )
        .expect("valid dynamically sized policy");

        assert_eq!(policy.rule_count(), 1_000);
    }
}
