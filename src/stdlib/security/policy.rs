
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Security Policy and Authorization
//!
//! Path: `src/stdlib/security/policy.rs`
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//!
//! ## Purpose
//!
//! Defines portable security-policy, authorization and audit contracts.
//!
//! This module provides:
//!
//! - open-world policy and authorization identifiers;
//! - authorization request and decision representations;
//! - explicit policy obligations;
//! - a provider-neutral policy evaluator contract;
//! - structured security audit events;
//! - an extensible audit-sink contract;
//! - a fail-closed authorization service;
//! - integration with key_management::KeyAccessAuthorizer.
//!
//! ## Architectural ownership
//!
//! This module owns the standard-library authorization and audit API.
//!
//! It does not own:
//!
//! - source grammar;
//! - policy-language parsing;
//! - semantic policy meaning;
//! - identity authentication;
//! - principal creation;
//! - cryptographic algorithms;
//! - key storage or lifecycle;
//! - capability discovery;
//! - trust establishment;
//! - runtime scheduling;
//! - physical resource allocation;
//! - hardware discovery;
//! - policy persistence;
//! - distributed consensus;
//! - security-provider implementation.
//!
//! Policy syntax is owned by:
//!
//!     grammar/policies/policy.g4
//!
//! Authorization syntax is owned by:
//!
//!     grammar/security/authorization.g4
//!
//! Permission syntax is owned by:
//!
//!     grammar/security/permissions.g4
//!
//! Identity and principal types are owned by:
//!
//!     crate::stdlib::security::identity
//!
//! Semantic policy interpretation remains the responsibility of the
//! frontend semantic authority.
//!
//! Actual authorization enforcement belongs to trusted runtime and
//! provider implementations.
//!
//! ## Security invariants
//!
//! - Authorization is deny-by-default.
//! - Indeterminate decisions never authorize an operation.
//! - Not-applicable decisions never authorize an operation.
//! - Denials never become permits through fallback behavior.
//! - Obligations must be fulfilled before an operation is authorized.
//! - Evaluation failures are not successful authorization.
//! - Required audit failures prevent authorization.
//! - Audit records contain references and non-secret metadata only.
//! - Identity references do not establish authentication.
//! - Evaluator declarations do not constitute independent security
//!   certification.
//! - Runtime and resource providers must enforce their own security
//!   boundaries.
//!
//! ## Scalability
//!
//! This module imposes no language-level maximum on:
//!
//! - policies;
//! - principals;
//! - actions;
//! - resources;
//! - authorization requests;
//! - obligations;
//! - policy evaluators;
//! - audit events;
//! - audit providers;
//! - devices;
//! - processors;
//! - distributed nodes;
//! - computational domains.
//!
//! Practical limits are determined by available resources and explicitly
//! configured operational policies.
//!
//! Literal infinite storage or execution is not promised.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::sync::Arc;

use super::identity::PrincipalId;
use super::key_management::{
    KeyAccessAuthorizer,
    KeyAccessRequest,
};
use super::cryptography::{
    CryptoError,
    CryptoErrorKind,
    CryptoResult,
};

// ============================================================================
// VALIDATION
// ============================================================================

fn valid_identifier(value: &str) -> bool {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return false;
    }

    value.split("::").all(|component| {
        let mut bytes = component.bytes();

        let Some(first) = bytes.next() else {
            return false;
        };

        (first.is_ascii_alphanumeric() || first == b'_')
            && bytes.all(|byte| {
                byte.is_ascii_alphanumeric()
                    || matches!(byte, b'_' | b'-' | b'.')
            })
    })
}

fn valid_reference(value: &str) -> bool {
    !value.is_empty()
        && value.trim() == value
        && !value.chars().any(char::is_control)
        && !value.contains('\0')
}

// ============================================================================
// ERRORS
// ============================================================================

/// Classification of policy and authorization failures.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PolicyErrorKind {
    /// An identifier is malformed.
    InvalidIdentifier,

    /// An authorization request is malformed.
    InvalidRequest,

    /// A policy decision is malformed.
    InvalidDecision,

    /// No valid authorization decision could be established.
    Indeterminate,

    /// The operation was denied.
    Denied,

    /// A policy obligation has not been fulfilled.
    ObligationsPending,

    /// The policy evaluator is unavailable.
    EvaluatorUnavailable,

    /// Policy evaluation failed.
    EvaluationFailure,

    /// An audit sink is unavailable.
    AuditUnavailable,

    /// Audit persistence failed.
    AuditFailure,

    /// A provider returned an invalid result.
    InvalidProviderResult,
}

impl PolicyErrorKind {
    /// Returns the stable machine-readable error identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidIdentifier => "invalid_identifier",
            Self::InvalidRequest => "invalid_request",
            Self::InvalidDecision => "invalid_decision",
            Self::Indeterminate => "indeterminate",
            Self::Denied => "denied",
            Self::ObligationsPending => "obligations_pending",
            Self::EvaluatorUnavailable => "evaluator_unavailable",
            Self::EvaluationFailure => "evaluation_failure",
            Self::AuditUnavailable => "audit_unavailable",
            Self::AuditFailure => "audit_failure",
            Self::InvalidProviderResult => "invalid_provider_result",
        }
    }
}

impl fmt::Display for PolicyErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Structured policy error.
///
/// This error deliberately avoids retaining arbitrary provider diagnostics
/// that might contain confidential policy or identity information.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PolicyError {
    kind: PolicyErrorKind,
}

impl PolicyError {
    /// Creates a policy error.
    #[must_use]
    pub const fn new(kind: PolicyErrorKind) -> Self {
        Self { kind }
    }

    /// Returns the error classification.
    #[must_use]
    pub const fn kind(&self) -> PolicyErrorKind {
        self.kind
    }
}

impl fmt::Display for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "security policy operation failed: {}", self.kind)
    }
}

impl StdError for PolicyError {}

/// Result type for policy operations.
pub type PolicyResult<T> = Result<T, PolicyError>;

// ============================================================================
// OPEN-WORLD IDENTIFIERS
// ============================================================================

macro_rules! policy_identifier {
    ($name:ident) => {
        #[doc = concat!("Open-world identifier for `", stringify!($name), "`.")]
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// Creates a validated identifier.
            pub fn new(value: impl Into<String>) -> PolicyResult<Self> {
                let value = value.into();

                if !valid_identifier(&value) {
                    return Err(PolicyError::new(
                        PolicyErrorKind::InvalidIdentifier,
                    ));
                }

                Ok(Self(value))
            }

            /// Returns the identifier.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

policy_identifier!(PolicyId);
policy_identifier!(PolicyRevision);
policy_identifier!(PolicyAction);
policy_identifier!(PolicyEvaluatorId);
policy_identifier!(AuditSinkId);
policy_identifier!(PolicyReasonCode);
policy_identifier!(PolicyObligationId);

/// An opaque authorization resource reference.
///
/// This is an identifier only. It does not resolve, access or prove the
/// existence of the referenced resource.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PolicyResource(String);

impl PolicyResource {
    /// Creates a resource reference.
    pub fn new(value: impl Into<String>) -> PolicyResult<Self> {
        let value = value.into();

        if !valid_reference(&value) {
            return Err(PolicyError::new(
                PolicyErrorKind::InvalidIdentifier,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque resource reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// POLICY REFERENCES
// ============================================================================

/// A reference to a policy and, optionally, a specific policy revision.
///
/// Revisions are opaque identifiers. Their ordering and consistency
/// semantics are defined by the policy authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyReference {
    id: PolicyId,
    revision: Option<PolicyRevision>,
}

impl PolicyReference {
    /// Creates a policy reference.
    #[must_use]
    pub fn new(id: PolicyId) -> Self {
        Self {
            id,
            revision: None,
        }
    }

    /// Associates a specific policy revision.
    #[must_use]
    pub fn with_revision(mut self, revision: PolicyRevision) -> Self {
        self.revision = Some(revision);
        self
    }

    /// Returns the policy identifier.
    #[must_use]
    pub fn id(&self) -> &PolicyId {
        &self.id
    }

    /// Returns the optional revision.
    #[must_use]
    pub fn revision(&self) -> Option<&PolicyRevision> {
        self.revision.as_ref()
    }
}

// ============================================================================
// NON-SECRET POLICY ATTRIBUTES
// ============================================================================

/// An open-world policy attribute key.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PolicyAttributeKey(String);

impl PolicyAttributeKey {
    /// Creates a policy attribute key.
    pub fn new(value: impl Into<String>) -> PolicyResult<Self> {
        let value = value.into();

        if !valid_identifier(&value) {
            return Err(PolicyError::new(
                PolicyErrorKind::InvalidIdentifier,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the key.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A non-secret policy attribute value.
///
/// Callers must not place passwords, tokens, private keys, credentials,
/// secret seeds or other authentication material in policy attributes.
#[derive(Clone, Eq, PartialEq)]
pub struct PolicyAttributeValue(String);

impl PolicyAttributeValue {
    /// Creates a non-secret attribute value.
    pub fn new(value: impl Into<String>) -> PolicyResult<Self> {
        let value = value.into();

        if value.chars().any(char::is_control) {
            return Err(PolicyError::new(
                PolicyErrorKind::InvalidRequest,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the attribute value.
    ///
    /// The caller is responsible for ensuring that the value is not secret.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for PolicyAttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PolicyAttributeValue([REDACTED])")
    }
}

// ============================================================================
// AUTHORIZATION REQUEST
// ============================================================================

/// An immutable description of an authorization request.
///
/// The request contains references and non-secret metadata only.
/// It does not authenticate the principal or resolve the resource.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizationRequest {
    principal: PrincipalId,
    action: PolicyAction,
    resource: Option<PolicyResource>,
    attributes: BTreeMap<PolicyAttributeKey, PolicyAttributeValue>,
    correlation_id: Option<String>,
}

impl AuthorizationRequest {
    /// Creates an authorization request.
    #[must_use]
    pub fn new(
        principal: PrincipalId,
        action: PolicyAction,
    ) -> Self {
        Self {
            principal,
            action,
            resource: None,
            attributes: BTreeMap::new(),
            correlation_id: None,
        }
    }

    /// Associates the resource being accessed.
    #[must_use]
    pub fn with_resource(mut self, resource: PolicyResource) -> Self {
        self.resource = Some(resource);
        self
    }

    /// Adds or replaces non-secret evaluation metadata.
    pub fn with_attribute(
        mut self,
        key: PolicyAttributeKey,
        value: PolicyAttributeValue,
    ) -> Self {
        self.attributes.insert(key, value);
        self
    }

    /// Associates a non-secret correlation identifier.
    pub fn with_correlation_id(
        mut self,
        value: impl Into<String>,
    ) -> PolicyResult<Self> {
        let value = value.into();

        if !valid_reference(&value) {
            return Err(PolicyError::new(
                PolicyErrorKind::InvalidRequest,
            ));
        }

        self.correlation_id = Some(value);
        Ok(self)
    }

    /// Returns the principal reference.
    #[must_use]
    pub fn principal(&self) -> &PrincipalId {
        &self.principal
    }

    /// Returns the requested action.
    #[must_use]
    pub fn action(&self) -> &PolicyAction {
        &self.action
    }

    /// Returns the optional resource.
    #[must_use]
    pub fn resource(&self) -> Option<&PolicyResource> {
        self.resource.as_ref()
    }

    /// Returns non-secret evaluation attributes.
    #[must_use]
    pub fn attributes(
        &self,
    ) -> &BTreeMap<PolicyAttributeKey, PolicyAttributeValue> {
        &self.attributes
    }

    /// Returns the correlation identifier.
    #[must_use]
    pub fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }
}

// ============================================================================
// POLICY OBLIGATIONS
// ============================================================================

/// An obligation associated with a policy decision.
///
/// The obligation is descriptive. This module does not execute it.
/// Runtime enforcement must establish that every required obligation has
/// been fulfilled before the protected operation proceeds.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyObligation {
    id: PolicyObligationId,
    parameters: BTreeMap<PolicyAttributeKey, PolicyAttributeValue>,
}

impl PolicyObligation {
    /// Creates an obligation.
    #[must_use]
    pub fn new(id: PolicyObligationId) -> Self {
        Self {
            id,
            parameters: BTreeMap::new(),
        }
    }

    /// Adds a non-secret obligation parameter.
    pub fn with_parameter(
        mut self,
        key: PolicyAttributeKey,
        value: PolicyAttributeValue,
    ) -> Self {
        self.parameters.insert(key, value);
        self
    }

    /// Returns the obligation identifier.
    #[must_use]
    pub fn id(&self) -> &PolicyObligationId {
        &self.id
    }

    /// Returns the obligation parameters.
    #[must_use]
    pub fn parameters(
        &self,
    ) -> &BTreeMap<PolicyAttributeKey, PolicyAttributeValue> {
        &self.parameters
    }
}

// ============================================================================
// POLICY DECISIONS
// ============================================================================

/// The outcome of policy evaluation.
///
/// Only `Permit` can potentially authorize an operation.
/// `Permit` with outstanding obligations is not sufficient to authorize.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PolicyEffect {
    /// The evaluator permits the requested operation.
    Permit,

    /// The evaluator explicitly denies the operation.
    Deny,

    /// No applicable policy established a decision.
    NotApplicable,

    /// Evaluation could not establish a trustworthy decision.
    Indeterminate,
}

/// A policy evaluation result.
///
/// The evaluator is responsible for producing an accurate decision.
/// This data structure does not independently prove that the evaluator
/// applied the correct policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyDecision {
    effect: PolicyEffect,
    policies: Vec<PolicyReference>,
    reasons: Vec<PolicyReasonCode>,
    obligations: Vec<PolicyObligation>,
}

impl PolicyDecision {
    /// Creates a decision with the specified effect.
    #[must_use]
    pub fn new(effect: PolicyEffect) -> Self {
        Self {
            effect,
            policies: Vec::new(),
            reasons: Vec::new(),
            obligations: Vec::new(),
        }
    }

    /// Adds a policy reference supporting the decision.
    #[must_use]
    pub fn with_policy(mut self, policy: PolicyReference) -> Self {
        self.policies.push(policy);
        self
    }

    /// Adds a machine-readable decision reason.
    #[must_use]
    pub fn with_reason(mut self, reason: PolicyReasonCode) -> Self {
        self.reasons.push(reason);
        self
    }

    /// Adds an outstanding obligation.
    #[must_use]
    pub fn with_obligation(mut self, obligation: PolicyObligation) -> Self {
        self.obligations.push(obligation);
        self
    }

    /// Returns the decision effect.
    #[must_use]
    pub const fn effect(&self) -> PolicyEffect {
        self.effect
    }

    /// Returns the policies cited by the evaluator.
    #[must_use]
    pub fn policies(&self) -> &[PolicyReference] {
        &self.policies
    }

    /// Returns decision reason codes.
    #[must_use]
    pub fn reasons(&self) -> &[PolicyReasonCode] {
        &self.reasons
    }

    /// Returns outstanding obligations.
    #[must_use]
    pub fn obligations(&self) -> &[PolicyObligation] {
        &self.obligations
    }

    /// Returns whether the decision is an unconditional permit.
    ///
    /// A permit with obligations is not authorization to proceed.
    #[must_use]
    pub fn is_unconditionally_permitted(&self) -> bool {
        self.effect == PolicyEffect::Permit && self.obligations.is_empty()
    }
}

// ============================================================================
// POLICY EVALUATOR
// ============================================================================

/// Contract for an authoritative policy evaluator.
///
/// Implementations may adapt the canonical semantic policy model,
/// a policy decision service, a local policy engine, or a distributed
/// policy authority.
///
/// The evaluator must:
///
/// - evaluate the complete request;
/// - preserve mandatory constraints;
/// - never turn unknown information into a permit;
/// - return `Indeterminate` when it cannot establish a decision;
/// - report outstanding obligations;
/// - avoid treating preferences and hints as mandatory constraints;
/// - document its consistency and revision semantics.
///
/// The trait does not certify an implementation as trustworthy.
pub trait PolicyEvaluator: Send + Sync {
    /// Returns the evaluator identity.
    fn id(&self) -> &PolicyEvaluatorId;

    /// Evaluates an authorization request.
    fn evaluate(
        &self,
        request: &AuthorizationRequest,
    ) -> PolicyResult<PolicyDecision>;
}

// ============================================================================
// AUDIT EVENTS
// ============================================================================

/// The result or failure being recorded in an audit event.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AuditOutcome {
    /// Policy evaluation produced a decision.
    Decision(PolicyDecision),

    /// Evaluation failed before producing a decision.
    EvaluationFailure(PolicyErrorKind),
}

/// A structured, non-secret authorization audit event.
///
/// The audit event contains no credential bytes, private keys, passwords,
/// secret tokens or raw cryptographic material.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditEvent {
    request: AuthorizationRequest,
    evaluator: PolicyEvaluatorId,
    outcome: AuditOutcome,
}

impl AuditEvent {
    /// Creates an audit event for a completed policy evaluation.
    #[must_use]
    pub fn decision(
        request: AuthorizationRequest,
        evaluator: PolicyEvaluatorId,
        decision: PolicyDecision,
    ) -> Self {
        Self {
            request,
            evaluator,
            outcome: AuditOutcome::Decision(decision),
        }
    }

    /// Creates an audit event for an evaluation failure.
    #[must_use]
    pub fn evaluation_failure(
        request: AuthorizationRequest,
        evaluator: PolicyEvaluatorId,
        error: PolicyErrorKind,
    ) -> Self {
        Self {
            request,
            evaluator,
            outcome: AuditOutcome::EvaluationFailure(error),
        }
    }

    /// Returns the original authorization request.
    #[must_use]
    pub fn request(&self) -> &AuthorizationRequest {
        &self.request
    }

    /// Returns the evaluator identity.
    #[must_use]
    pub fn evaluator(&self) -> &PolicyEvaluatorId {
        &self.evaluator
    }

    /// Returns the recorded outcome.
    #[must_use]
    pub fn outcome(&self) -> &AuditOutcome {
        &self.outcome
    }
}

// ============================================================================
// AUDIT SINK
// ============================================================================

/// An opaque receipt returned after an audit event has been persisted.
///
/// The reference is assigned by the audit provider. It is not a timestamp,
/// global sequence number or proof of tamper resistance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditReceipt {
    sink: AuditSinkId,
    event_reference: String,
}

impl AuditReceipt {
    /// Creates a provider-returned audit receipt.
    pub fn new(
        sink: AuditSinkId,
        event_reference: impl Into<String>,
    ) -> PolicyResult<Self> {
        let event_reference = event_reference.into();

        if !valid_reference(&event_reference) {
            return Err(PolicyError::new(
                PolicyErrorKind::InvalidProviderResult,
            ));
        }

        Ok(Self {
            sink,
            event_reference,
        })
    }

    /// Returns the audit sink identity.
    #[must_use]
    pub fn sink(&self) -> &AuditSinkId {
        &self.sink
    }

    /// Returns the opaque event reference.
    #[must_use]
    pub fn event_reference(&self) -> &str {
        &self.event_reference
    }
}

/// Contract for durable or otherwise explicitly documented audit storage.
///
/// An implementation must not return success unless it has fulfilled its
/// documented persistence guarantee. A buffered or queued write must not
/// be described as durable persistence unless the provider's contract
/// actually guarantees it.
pub trait AuditSink: Send + Sync {
    /// Returns the audit sink identity.
    fn id(&self) -> &AuditSinkId;

    /// Appends an audit event.
    fn append(&self, event: &AuditEvent) -> PolicyResult<AuditReceipt>;
}

// ============================================================================
// AUDIT REQUIREMENT
// ============================================================================

/// Determines how authorization handles audit-sink failures.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AuditRequirement {
    /// Authorization fails when the audit event cannot be recorded.
    ///
    /// This is the default and recommended security setting.
    #[default]
    Required,

    /// Authorization may continue if audit recording fails.
    ///
    /// This explicitly weakens the audit guarantee. It must only be used
    /// where the governing security policy permits it.
    BestEffort,
}

/// The result of attempting to record an audit event.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AuditStatus {
    /// The audit sink returned a valid receipt.
    Recorded(AuditReceipt),

    /// The audit event could not be recorded.
    Unavailable(PolicyErrorKind),
}

// ============================================================================
// AUTHORIZATION RECEIPT
// ============================================================================

/// Result of evaluating and auditing an authorization request.
///
/// A receipt is evidence of what this service observed. It is not a
/// cryptographic proof or a substitute for runtime enforcement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizationReceipt {
    decision: PolicyDecision,
    audit: AuditStatus,
}

impl AuthorizationReceipt {
    /// Returns the policy decision.
    #[must_use]
    pub fn decision(&self) -> &PolicyDecision {
        &self.decision
    }

    /// Returns the audit status.
    #[must_use]
    pub fn audit_status(&self) -> &AuditStatus {
        &self.audit
    }

    /// Returns whether the decision permits immediate execution.
    #[must_use]
    pub fn is_authorized(&self) -> bool {
        self.decision.is_unconditionally_permitted()
    }
}

// ============================================================================
// AUTHORIZATION SERVICE
// ============================================================================

/// Provider-neutral authorization service.
///
/// The service does not contain policy rules or a policy database.
/// It delegates evaluation to an explicit authoritative evaluator and
/// records the result through an explicit audit sink.
///
/// Both dependencies are supplied by the application/runtime.
/// No implicit allow-all evaluator or no-op audit sink is provided.
#[derive(Clone)]
pub struct AuthorizationService {
    evaluator: Arc<dyn PolicyEvaluator>,
    audit_sink: Arc<dyn AuditSink>,
    audit_requirement: AuditRequirement,
}

impl AuthorizationService {
    /// Creates an authorization service.
    ///
    /// The evaluator and audit sink must be explicitly supplied.
    #[must_use]
    pub fn new(
        evaluator: Arc<dyn PolicyEvaluator>,
        audit_sink: Arc<dyn AuditSink>,
    ) -> Self {
        Self {
            evaluator,
            audit_sink,
            audit_requirement: AuditRequirement::Required,
        }
    }

    /// Selects the audit failure policy.
    ///
    /// The default is `AuditRequirement::Required`.
    #[must_use]
    pub fn with_audit_requirement(
        mut self,
        requirement: AuditRequirement,
    ) -> Self {
        self.audit_requirement = requirement;
        self
    }

    /// Returns the evaluator identity.
    #[must_use]
    pub fn evaluator_id(&self) -> &PolicyEvaluatorId {
        self.evaluator.id()
    }

    /// Returns the audit sink identity.
    #[must_use]
    pub fn audit_sink_id(&self) -> &AuditSinkId {
        self.audit_sink.id()
    }

    /// Evaluates and audits a request.
    ///
    /// This method returns a receipt for permit, deny, not-applicable and
    /// indeterminate decisions. It never treats those outcomes as equivalent.
    ///
    /// If evaluation itself fails, the service attempts to audit that failure
    /// before returning the evaluation error.
    pub fn evaluate_and_audit(
        &self,
        request: &AuthorizationRequest,
    ) -> PolicyResult<AuthorizationReceipt> {
        let decision = match self.evaluator.evaluate(request) {
            Ok(decision) => decision,
            Err(error) => {
                let event = AuditEvent::evaluation_failure(
                    request.clone(),
                    self.evaluator.id().clone(),
                    error.kind(),
                );

                let audit = self.record(&event)?;

                // Best-effort audit does not turn failed evaluation into
                // successful authorization.
                let _ = audit;

                return Err(error);
            }
        };

        let event = AuditEvent::decision(
            request.clone(),
            self.evaluator.id().clone(),
            decision.clone(),
        );

        let audit = self.record(&event)?;

        Ok(AuthorizationReceipt {
            decision,
            audit,
        })
    }

    /// Authorizes a request only when the decision is an unconditional permit.
    ///
    /// Deny, not-applicable, indeterminate and outstanding obligations all
    /// fail closed.
    pub fn authorize(
        &self,
        request: &AuthorizationRequest,
    ) -> PolicyResult<AuthorizationReceipt> {
        let receipt = self.evaluate_and_audit(request)?;

        match receipt.decision.effect() {
            PolicyEffect::Permit => {
                if !receipt.decision.obligations().is_empty() {
                    return Err(PolicyError::new(
                        PolicyErrorKind::ObligationsPending,
                    ));
                }

                Ok(receipt)
            }

            PolicyEffect::Deny => Err(PolicyError::new(
                PolicyErrorKind::Denied,
            )),

            PolicyEffect::NotApplicable => Err(PolicyError::new(
                PolicyErrorKind::Denied,
            )),

            PolicyEffect::Indeterminate => Err(PolicyError::new(
                PolicyErrorKind::Indeterminate,
            )),
        }
    }

    fn record(&self, event: &AuditEvent) -> PolicyResult<AuditStatus> {
        match self.audit_sink.append(event) {
            Ok(receipt) => {
                if receipt.sink() != self.audit_sink.id() {
                    return Err(PolicyError::new(
                        PolicyErrorKind::InvalidProviderResult,
                    ));
                }

                Ok(AuditStatus::Recorded(receipt))
            }

            Err(error) => match self.audit_requirement {
                AuditRequirement::Required => Err(PolicyError::new(
                    PolicyErrorKind::AuditUnavailable,
                )),

                AuditRequirement::BestEffort => {
                    Ok(AuditStatus::Unavailable(error.kind()))
                }
            },
        }
    }
}

// ============================================================================
// KEY-MANAGEMENT AUTHORIZATION ADAPTER
// ============================================================================

/// Adapts the canonical policy service to the existing key-management
/// authorization interface.
///
/// This adapter deliberately does not authenticate the principal.
/// Authentication and identity resolution must happen before the caller
/// constructs a KeyAccessContext.
///
/// Key-management requests are translated into generic authorization
/// requests. The policy evaluator remains the authority for the decision.
#[derive(Clone)]
pub struct KeyManagementPolicyAuthorizer {
    service: AuthorizationService,
}

impl KeyManagementPolicyAuthorizer {
    /// Creates the key-management policy adapter.
    #[must_use]
    pub fn new(service: AuthorizationService) -> Self {
        Self { service }
    }

    /// Returns the underlying authorization service.
    #[must_use]
    pub fn service(&self) -> &AuthorizationService {
        &self.service
    }
}

impl KeyAccessAuthorizer for KeyManagementPolicyAuthorizer {
    fn authorize(&self, request: &KeyAccessRequest) -> CryptoResult<()> {
        let principal = PrincipalId::new(request.principal())
            .map_err(|_| {
                CryptoError::new(CryptoErrorKind::InvalidRequest)
            })?;

        let action = PolicyAction::new(request.action().as_str())
            .map_err(|_| {
                CryptoError::new(CryptoErrorKind::InvalidRequest)
            })?;

        let mut authorization =
            AuthorizationRequest::new(principal, action);

        if let Some(key) = request.key() {
            let resource = PolicyResource::new(format!(
                "key::{}",
                key.as_str()
            ))
            .map_err(|_| {
                CryptoError::new(CryptoErrorKind::InvalidRequest)
            })?;

            authorization = authorization.with_resource(resource);
        }

        if let Some(algorithm) = request.algorithm() {
            let key = PolicyAttributeKey::new(
                "cryptography::algorithm",
            )
            .map_err(|_| {
                CryptoError::new(CryptoErrorKind::InvalidRequest)
            })?;

            let value = PolicyAttributeValue::new(
                algorithm.as_str(),
            )
            .map_err(|_| {
                CryptoError::new(CryptoErrorKind::InvalidRequest)
            })?;

            authorization = authorization.with_attribute(key, value);
        }

        if let Some(purpose) = request.purpose() {
            let key = PolicyAttributeKey::new(
                "security::key_purpose",
            )
            .map_err(|_| {
                CryptoError::new(CryptoErrorKind::InvalidRequest)
            })?;

            let value = PolicyAttributeValue::new(
                purpose.as_str(),
            )
            .map_err(|_| {
                CryptoError::new(CryptoErrorKind::InvalidRequest)
            })?;

            authorization = authorization.with_attribute(key, value);
        }

        if let Some(correlation_id) = request.correlation_id() {
            authorization = authorization
                .with_correlation_id(correlation_id)
                .map_err(|_| {
                    CryptoError::new(CryptoErrorKind::InvalidRequest)
                })?;
        }

        self.service.authorize(&authorization)
            .map(|_| ())
            .map_err(|error| {
                let kind = match error.kind() {
                    PolicyErrorKind::Denied
                    | PolicyErrorKind::ObligationsPending => {
                        CryptoErrorKind::Denied
                    }

                    PolicyErrorKind::InvalidIdentifier
                    | PolicyErrorKind::InvalidRequest => {
                        CryptoErrorKind::InvalidRequest
                    }

                    _ => CryptoErrorKind::ProviderFailure,
                };

                CryptoError::new(kind)
            })
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct TestEvaluator {
        id: PolicyEvaluatorId,
        decision: PolicyDecision,
    }

    impl PolicyEvaluator for TestEvaluator {
        fn id(&self) -> &PolicyEvaluatorId {
            &self.id
        }

        fn evaluate(
            &self,
            _request: &AuthorizationRequest,
        ) -> PolicyResult<PolicyDecision> {
            Ok(self.decision.clone())
        }
    }

    struct TestAuditSink {
        id: AuditSinkId,
        events: Mutex<Vec<AuditEvent>>,
    }

    impl AuditSink for TestAuditSink {
        fn id(&self) -> &AuditSinkId {
            &self.id
        }

        fn append(&self, event: &AuditEvent) -> PolicyResult<AuditReceipt> {
            self.events
                .lock()
                .map_err(|_| {
                    PolicyError::new(PolicyErrorKind::AuditFailure)
                })?
                .push(event.clone());

            AuditReceipt::new(
                self.id.clone(),
                "test::audit_event",
            )
        }
    }

    fn request() -> AuthorizationRequest {
        AuthorizationRequest::new(
            PrincipalId::new("application::worker").unwrap(),
            PolicyAction::new("storage::read").unwrap(),
        )
        .with_resource(
            PolicyResource::new("storage::dataset::primary").unwrap(),
        )
    }

    fn service(
        effect: PolicyEffect,
    ) -> (
        AuthorizationService,
        Arc<TestAuditSink>,
    ) {
        let evaluator = Arc::new(TestEvaluator {
            id: PolicyEvaluatorId::new(
                "test::policy_evaluator",
            )
            .unwrap(),
            decision: PolicyDecision::new(effect),
        });

        let sink = Arc::new(TestAuditSink {
            id: AuditSinkId::new("test::audit_sink").unwrap(),
            events: Mutex::new(Vec::new()),
        });

        (
            AuthorizationService::new(
                evaluator,
                sink.clone(),
            ),
            sink,
        )
    }

    #[test]
    fn open_world_identifiers_are_supported() {
        let action = PolicyAction::new(
            "future::quantum_resource_reconfiguration",
        );

        assert!(action.is_ok());
    }

    #[test]
    fn malformed_identifiers_are_rejected() {
        assert!(PolicyAction::new("").is_err());
        assert!(PolicyAction::new(" invalid").is_err());
        assert!(PolicyAction::new("security::").is_err());
    }

    #[test]
    fn unconditional_permit_authorizes() {
        let (service, _) = service(PolicyEffect::Permit);

        let receipt = service.authorize(&request()).unwrap();

        assert!(receipt.is_authorized());
        assert!(matches!(
            receipt.audit_status(),
            AuditStatus::Recorded(_)
        ));
    }

    #[test]
    fn deny_never_authorizes() {
        let (service, _) = service(PolicyEffect::Deny);

        let result = service.authorize(&request());

        assert_eq!(
            result.unwrap_err().kind(),
            PolicyErrorKind::Denied,
        );
    }

    #[test]
    fn not_applicable_never_authorizes() {
        let (service, _) = service(PolicyEffect::NotApplicable);

        let result = service.authorize(&request());

        assert_eq!(
            result.unwrap_err().kind(),
            PolicyErrorKind::Denied,
        );
    }

    #[test]
    fn indeterminate_never_authorizes() {
        let (service, _) = service(PolicyEffect::Indeterminate);

        let result = service.authorize(&request());

        assert_eq!(
            result.unwrap_err().kind(),
            PolicyErrorKind::Indeterminate,
        );
    }

    #[test]
    fn outstanding_obligations_prevent_authorization() {
        let evaluator = Arc::new(TestEvaluator {
            id: PolicyEvaluatorId::new("test::evaluator").unwrap(),
            decision: PolicyDecision::new(PolicyEffect::Permit)
                .with_obligation(
                    PolicyObligation::new(
                        PolicyObligationId::new(
                            "security::record_access",
                        )
                        .unwrap(),
                    ),
                ),
        });

        let sink = Arc::new(TestAuditSink {
            id: AuditSinkId::new("test::sink").unwrap(),
            events: Mutex::new(Vec::new()),
        });

        let service = AuthorizationService::new(evaluator, sink);

        let result = service.authorize(&request());

        assert_eq!(
            result.unwrap_err().kind(),
            PolicyErrorKind::ObligationsPending,
        );
    }

    #[test]
    fn decisions_are_audited() {
        let (service, sink) = service(PolicyEffect::Deny);

        let receipt = service
            .evaluate_and_audit(&request())
            .unwrap();

        assert_eq!(
            receipt.decision().effect(),
            PolicyEffect::Deny,
        );

        assert_eq!(sink.events.lock().unwrap().len(), 1);
    }

    #[test]
    fn audit_failure_prevents_authorization_by_default() {
        struct FailingAuditSink {
            id: AuditSinkId,
        }

        impl AuditSink for FailingAuditSink {
            fn id(&self) -> &AuditSinkId {
                &self.id
            }

            fn append(
                &self,
                _event: &AuditEvent,
            ) -> PolicyResult<AuditReceipt> {
                Err(PolicyError::new(
                    PolicyErrorKind::AuditFailure,
                ))
            }
        }

        let evaluator = Arc::new(TestEvaluator {
            id: PolicyEvaluatorId::new("test::evaluator").unwrap(),
            decision: PolicyDecision::new(PolicyEffect::Permit),
        });

        let sink = Arc::new(FailingAuditSink {
            id: AuditSinkId::new("test::failing_sink").unwrap(),
        });

        let service = AuthorizationService::new(evaluator, sink);

        let result = service.authorize(&request());

        assert_eq!(
            result.unwrap_err().kind(),
            PolicyErrorKind::AuditUnavailable,
        );
    }

    #[test]
    fn best_effort_audit_is_explicit() {
        struct FailingAuditSink {
            id: AuditSinkId,
        }

        impl AuditSink for FailingAuditSink {
            fn id(&self) -> &AuditSinkId {
                &self.id
            }

            fn append(
                &self,
                _event: &AuditEvent,
            ) -> PolicyResult<AuditReceipt> {
                Err(PolicyError::new(
                    PolicyErrorKind::AuditFailure,
                ))
            }
        }

        let evaluator = Arc::new(TestEvaluator {
            id: PolicyEvaluatorId::new("test::evaluator").unwrap(),
            decision: PolicyDecision::new(PolicyEffect::Permit),
        });

        let sink = Arc::new(FailingAuditSink {
            id: AuditSinkId::new("test::failing_sink").unwrap(),
        });

        let service = AuthorizationService::new(evaluator, sink)
            .with_audit_requirement(AuditRequirement::BestEffort);

        let receipt = service.authorize(&request()).unwrap();

        assert!(receipt.is_authorized());
        assert!(matches!(
            receipt.audit_status(),
            AuditStatus::Unavailable(_)
        ));
    }

    #[test]
    fn policy_attribute_debug_redacts_values() {
        let value = PolicyAttributeValue::new(
            "non-public-metadata",
        )
        .unwrap();

        assert!(
            !format!("{value:?}").contains("non-public-metadata")
        );
    }

    #[test]
    fn correlation_identifier_is_preserved() {
        let request = request()
            .with_correlation_id("trace::request_1")
            .unwrap();

        assert_eq!(
            request.correlation_id(),
            Some("trace::request_1"),
        );
    }

    #[test]
    fn audit_event_contains_no_secret_material_fields() {
        let event = AuditEvent::decision(
            request(),
            PolicyEvaluatorId::new("test::evaluator").unwrap(),
            PolicyDecision::new(PolicyEffect::Deny),
        );

        assert_eq!(
            event.outcome(),
            &AuditOutcome::Decision(
                PolicyDecision::new(PolicyEffect::Deny)
            ),
        );
    }
}
