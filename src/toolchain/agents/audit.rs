
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Agent Audit
//!
//! Path: src/toolchain/agents/audit.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides an immutable, structured audit-event model and a
//! pluggable audit-sink boundary for human and AI-agent toolchain
//! operations.
//!
//! The audit subsystem records:
//!
//! - Authenticated principal identity.
//! - Requested command identity.
//! - Protected resource identity.
//! - Request correlation.
//! - Authentication evidence references.
//! - Explicit authorization decisions.
//! - Actual operation outcomes.
//! - Event identity and externally supplied occurrence time.
//! - Optional non-authoritative client metadata.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Audit event representation.
//! - Audit authorization outcome representation.
//! - Operation outcome representation.
//! - Event validation.
//! - Audit sink integration contract.
//! - Audit recording orchestration.
//!
//! ## Does not own
//!
//! This module does not own:
//!
//! - Authentication.
//! - Authorization.
//! - Policy evaluation.
//! - Permission enforcement.
//! - Credential management.
//! - Cryptographic signing.
//! - Cryptographic verification.
//! - Hash generation.
//! - Provenance verification.
//! - Runtime execution.
//! - Command dispatch.
//! - Project discovery.
//! - Hardware discovery.
//! - Resource allocation.
//! - Log rotation.
//! - Storage implementation.
//! - Network transmission.
//! - Retention policy.
//! - Source-language audit parsing.
//!
//! Those responsibilities belong to their respective authoritative
//! subsystems and registered storage providers.
//!
//! ## Authoritative integration
//!
//! Authorization:
//!     crate::toolchain::agents::permissions
//!
//! Authentication evidence:
//!     crate::toolchain::agents::permissions::AuthorizationReceipt
//!
//! Agent requests:
//!     crate::toolchain::agents::requests
//!
//! Agent results:
//!     crate::toolchain::agents::results
//!
//! Agent context:
//!     crate::toolchain::agents::context
//!
//! Security policy:
//!     crate::toolchain::security::policy
//!
//! Security provenance:
//!     crate::toolchain::security::provenance
//!
//! Security policy validation:
//!     crate::toolchain::security::validation
//!
//! Source-level audit intent:
//!     grammar/security/audit.g4
//!
//! Source-level authorization:
//!     grammar/security/authorization.g4
//!
//! Source-level permissions:
//!     grammar/security/permissions.g4
//!
//! Normative security semantics:
//!     grammar/spec/security.md
//!
//! ## Security invariants
//!
//! 1. An agent-supplied identity is never treated as authenticated.
//! 2. Authorization evidence comes from AuthorizationReceipt.
//! 3. Audit recording never grants permission.
//! 4. Audit recording never executes an operation.
//! 5. An explicit authorization decision is preserved.
//! 6. Indeterminate authorization never becomes Permit.
//! 7. An unsuccessful operation is never recorded as successful.
//! 8. Storage failures are propagated.
//! 9. Event identifiers must be supplied by the caller.
//! 10. Occurrence timestamps must be supplied by a trusted caller.
//! 11. Secrets and credentials must never be recorded.
//! 12. Audit records must not contain executable agent instructions.
//!
//! ## POCO-REAF
//!
//! Audit events are independent of:
//!
//! - Classical processors.
//! - Quantum processors.
//! - Qubits.
//! - GPUs.
//! - FPGAs.
//! - ASICs.
//! - Distributed nodes.
//! - AI accelerators.
//! - Networking providers.
//! - Future computing domains.
//!
//! Event sources, commands, authorities, resources and provider
//! identifiers are open-ended strings.
//!
//! No universal maximum is imposed on the number of audit events,
//! principals, commands, resources, providers or computing domains.
//!
//! Actual storage and processing remain subject to available resources
//! and explicitly configured operational policies.
//!
//! Literal infinite storage is not promised.
//!
//! ## Determinism
//!
//! Events contain no implicitly generated random identifiers or
//! implicit timestamps.
//!
//! Identical validated inputs produce identical event contents.
//!
//! Event ordering and durable sequencing are responsibilities of
//! the selected audit-storage implementation.
//!
//! ## Storage contract
//!
//! AuditSink implementations must:
//!
//! - Preserve the supplied event.
//! - Treat event identifiers as idempotency keys.
//! - Reject conflicting reuse of an event identifier.
//! - Report storage failures.
//! - Provide durable acknowledgement when claiming durability.
//! - Avoid silently discarding records.
//! - Enforce their configured retention and access policies.
//!
//! A successful append response means the sink has accepted the
//! record according to its documented durability contract.

use serde::{Deserialize, Serialize};

use std::fmt;
use std::sync::Arc;

use crate::toolchain::agents::permissions::{
    AuthorizationReceipt,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainResult,
};

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// Stable audit component identity.
pub const COMPONENT: &str = "toolchain.agents.audit";

/// Stable audit-record validation operation.
pub const VALIDATE_OPERATION: &str =
    "toolchain.agents.audit.validate";

/// Stable audit-record submission operation.
pub const RECORD_OPERATION: &str =
    "toolchain.agents.audit.record";

/// Audit event schema version.
///
/// This version is independent of the language, compiler, toolchain
/// API, package format, policy and provider versions.
pub const AUDIT_SCHEMA_VERSION: u32 = 1;

// ============================================================================
// VALIDATION
// ============================================================================

fn validation_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::validation_failed(message)
        .with_operation(VALIDATE_OPERATION)
        .with_context(COMPONENT)
}

fn recording_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::invalid_input(message)
        .with_operation(RECORD_OPERATION)
        .with_context(COMPONENT)
}

fn validate_identifier(
    field: &str,
    value: &str,
) -> ToolchainResult<()> {
    if value.is_empty() || value.trim().is_empty() {
        return Err(validation_error(format!(
            "{field} must not be empty"
        )));
    }

    if value != value.trim() {
        return Err(validation_error(format!(
            "{field} must not contain surrounding whitespace"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(validation_error(format!(
            "{field} must not contain control characters"
        )));
    }

    Ok(())
}

fn validate_optional_identifier(
    field: &str,
    value: Option<&str>,
) -> ToolchainResult<()> {
    if let Some(value) = value {
        validate_identifier(field, value)?;
    }

    Ok(())
}

fn validate_timestamp(
    value: &str,
) -> ToolchainResult<()> {
    if value.trim().is_empty() {
        return Err(validation_error(
            "audit occurrence timestamp must not be empty",
        ));
    }

    if value != value.trim() {
        return Err(validation_error(
            "audit occurrence timestamp contains surrounding whitespace",
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(validation_error(
            "audit occurrence timestamp contains control characters",
        ));
    }

    Ok(())
}

// ============================================================================
// AUTHORIZATION OUTCOME
// ============================================================================

/// Authorization effect recorded in an audit event.
///
/// This is an audit representation of the authoritative policy
/// engine's decision. It does not implement policy evaluation.
///
/// Unknown future policy effects must not be interpreted as Permit.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AuditAuthorizationEffect {
    /// The canonical policy engine explicitly permitted the operation.
    Permit,

    /// The canonical policy engine explicitly denied the operation.
    Deny,

    /// No applicable authorization rule established permission.
    NotApplicable,

    /// Authorization could not establish a definitive decision.
    Indeterminate,
}

impl AuditAuthorizationEffect {
    /// Return the stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Permit => "permit",
            Self::Deny => "deny",
            Self::NotApplicable => "not_applicable",
            Self::Indeterminate => "indeterminate",
        }
    }

    /// Whether the recorded decision explicitly grants permission.
    #[must_use]
    pub const fn is_permitted(self) -> bool {
        matches!(self, Self::Permit)
    }
}

// ============================================================================
// OPERATION OUTCOME
// ============================================================================

/// Actual lifecycle outcome of the requested operation.
///
/// Authorization and execution are deliberately separate.
///
/// A Permit does not imply successful execution.
/// Successful execution must be established by the operation owner.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AuditOperationOutcome {
    /// The operation was accepted but has not started.
    Accepted,

    /// The operation has started.
    Started,

    /// The operation completed successfully.
    Succeeded,

    /// The operation completed with explicitly reported partial success.
    PartiallySucceeded,

    /// The operation failed.
    Failed,

    /// The operation was denied before execution.
    Denied,

    /// The operation was cancelled.
    Cancelled,

    /// The operation could not establish a definitive outcome.
    Unresolved,

    /// The operation was not evaluated.
    NotEvaluated,
}

impl AuditOperationOutcome {
    /// Return the stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Started => "started",
            Self::Succeeded => "succeeded",
            Self::PartiallySucceeded => "partially_succeeded",
            Self::Failed => "failed",
            Self::Denied => "denied",
            Self::Cancelled => "cancelled",
            Self::Unresolved => "unresolved",
            Self::NotEvaluated => "not_evaluated",
        }
    }

    /// Whether the operation has definitively completed successfully.
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Succeeded)
    }

    /// Whether the operation represents a terminal outcome.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Succeeded
                | Self::PartiallySucceeded
                | Self::Failed
                | Self::Denied
                | Self::Cancelled
        )
    }
}

// ============================================================================
// EVENT CLASSIFICATION
// ============================================================================

/// Category of the recorded audit event.
///
/// Event categories describe lifecycle observations, not
/// computational domains or implementation providers.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventKind {
    /// Authorization was evaluated.
    Authorization,

    /// The requested operation was accepted.
    OperationAccepted,

    /// The requested operation started.
    OperationStarted,

    /// The requested operation completed.
    OperationCompleted,

    /// The requested operation failed.
    OperationFailed,

    /// The requested operation was denied.
    OperationDenied,

    /// The requested operation was cancelled.
    OperationCancelled,

    /// The requested operation's outcome remains unresolved.
    OperationUnresolved,
}

impl AuditEventKind {
    /// Return the stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authorization => "authorization",
            Self::OperationAccepted => "operation_accepted",
            Self::OperationStarted => "operation_started",
            Self::OperationCompleted => "operation_completed",
            Self::OperationFailed => "operation_failed",
            Self::OperationDenied => "operation_denied",
            Self::OperationCancelled => "operation_cancelled",
            Self::OperationUnresolved => "operation_unresolved",
        }
    }
}

// ============================================================================
// AUTHENTICATION EVIDENCE
// ============================================================================

/// Non-secret authentication evidence reference.
///
/// The authoritative authentication provider supplies this reference
/// through AuthorizationReceipt.
///
/// The reference is not a credential, bearer token, password,
/// private key, session secret or authentication proof.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct AuthenticationEvidenceReference {
    reference: String,
}

impl AuthenticationEvidenceReference {
    /// Construct an authentication evidence reference.
    ///
    /// Callers must provide a non-secret reference obtained from
    /// the authoritative authentication provider.
    pub fn new(
        reference: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let reference = reference.into();

        validate_identifier(
            "authentication evidence reference",
            &reference,
        )?;

        Ok(Self { reference })
    }

    /// Return the opaque reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.reference
    }
}

// ============================================================================
// AUDIT EVENT
// ============================================================================

/// Immutable audit event for an authenticated toolchain operation.
///
/// The authenticated principal, requested command, protected resource,
/// correlation identifier and authorization effect are obtained from
/// the canonical AuthorizationReceipt.
///
/// No command payload, source code, credential, token, private key,
/// secret context or arbitrary agent-generated instruction is stored.
///
/// The occurrence timestamp and event identifier are supplied by
/// the caller because this module does not own time generation or
/// globally unique identifier generation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct AuditEvent {
    /// Audit event schema version.
    schema_version: u32,

    /// Stable event identity supplied by the caller.
    event_id: String,

    /// Externally supplied occurrence timestamp.
    ///
    /// The caller must use a documented canonical timestamp format,
    /// preferably RFC 3339 with an explicit timezone.
    occurred_at: String,

    /// Event lifecycle category.
    kind: AuditEventKind,

    /// Authenticated principal established by the authorization service.
    principal: String,

    /// Canonical requested command identity.
    command: String,

    /// Canonical protected resource identity.
    resource: String,

    /// Original request correlation identifier, when available.
    correlation_id: Option<String>,

    /// Authentication evidence reference, when available.
    authentication_reference: Option<AuthenticationEvidenceReference>,

    /// Authoritative authorization decision.
    authorization: AuditAuthorizationEffect,

    /// Reported operation outcome.
    outcome: AuditOperationOutcome,

    /// Optional client identifier supplied as descriptive metadata.
    ///
    /// This is not an authenticated identity.
    client_reference: Option<String>,

    /// Open-ended identifier of the audit event's originating subsystem.
    source: String,

    /// Optional non-secret provenance reference.
    provenance_reference: Option<String>,
}

impl AuditEvent {
    /// Construct an audit event from a canonical authorization receipt.
    ///
    /// This constructor is the primary authorization-to-audit boundary.
    ///
    /// Identity, command, resource, authorization effect, correlation
    /// and authentication evidence cannot be independently supplied
    /// by the requesting agent.
    ///
    /// The operation outcome must be supplied by the subsystem that
    /// actually observes the operation lifecycle.
    ///
    /// This method does not execute the operation or write to storage.
    pub fn from_authorization(
        event_id: impl Into<String>,
        occurred_at: impl Into<String>,
        receipt: &AuthorizationReceipt,
        outcome: AuditOperationOutcome,
    ) -> ToolchainResult<Self> {
        let effect = match receipt.effect() {
            crate::toolchain::security::policy::DecisionEffect::Permit => {
                AuditAuthorizationEffect::Permit
            }

            crate::toolchain::security::policy::DecisionEffect::Deny => {
                AuditAuthorizationEffect::Deny
            }

            crate::toolchain::security::policy::DecisionEffect::NotApplicable => {
                AuditAuthorizationEffect::NotApplicable
            }

            crate::toolchain::security::policy::DecisionEffect::Indeterminate => {
                AuditAuthorizationEffect::Indeterminate
            }
        };

        let authentication_reference = receipt
            .authentication_reference()
            .map(AuthenticationEvidenceReference::new)
            .transpose()?;

        let kind = match outcome {
            AuditOperationOutcome::Accepted => {
                AuditEventKind::OperationAccepted
            }

            AuditOperationOutcome::Started => {
                AuditEventKind::OperationStarted
            }

            AuditOperationOutcome::Succeeded
            | AuditOperationOutcome::PartiallySucceeded => {
                AuditEventKind::OperationCompleted
            }

            AuditOperationOutcome::Failed => {
                AuditEventKind::OperationFailed
            }

            AuditOperationOutcome::Denied => {
                AuditEventKind::OperationDenied
            }

            AuditOperationOutcome::Cancelled => {
                AuditEventKind::OperationCancelled
            }

            AuditOperationOutcome::Unresolved
            | AuditOperationOutcome::NotEvaluated => {
                AuditEventKind::OperationUnresolved
            }
        };

        let event = Self {
            schema_version: AUDIT_SCHEMA_VERSION,
            event_id: event_id.into(),
            occurred_at: occurred_at.into(),
            kind,
            principal: receipt.principal().to_owned(),
            command: receipt.command().to_owned(),
            resource: receipt.resource().to_owned(),
            correlation_id: receipt
                .correlation_id()
                .map(str::to_owned),
            authentication_reference,
            authorization: effect,
            outcome,
            client_reference: None,
            source: COMPONENT.to_owned(),
            provenance_reference: None,
        };

        event.validate()?;

        Ok(event)
    }

    /// Attach non-authoritative client metadata.
    ///
    /// This value must never be used as proof of authenticated identity.
    pub fn with_client_reference(
        mut self,
        reference: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let reference = reference.into();

        validate_identifier(
            "client reference",
            &reference,
        )?;

        self.client_reference = Some(reference);

        self.validate()?;

        Ok(self)
    }

    /// Attach an opaque provenance reference.
    ///
    /// The reference must identify provenance information owned
    /// and verified by its authoritative subsystem.
    ///
    /// This method does not create or verify provenance.
    pub fn with_provenance_reference(
        mut self,
        reference: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let reference = reference.into();

        validate_identifier(
            "provenance reference",
            &reference,
        )?;

        self.provenance_reference = Some(reference);

        self.validate()?;

        Ok(self)
    }

    /// Validate the complete audit event.
    ///
    /// This method must be called after deserialization and before
    /// storage or transmission.
    pub fn validate(&self) -> ToolchainResult<()> {
        if self.schema_version != AUDIT_SCHEMA_VERSION {
            return Err(validation_error(format!(
                "unsupported audit schema version: {}",
                self.schema_version
            )));
        }

        validate_identifier(
            "event identifier",
            &self.event_id,
        )?;

        validate_timestamp(&self.occurred_at)?;

        validate_identifier(
            "authenticated principal",
            &self.principal,
        )?;

        validate_identifier(
            "requested command",
            &self.command,
        )?;

        validate_identifier(
            "protected resource",
            &self.resource,
        )?;

        validate_identifier(
            "event source",
            &self.source,
        )?;

        validate_optional_identifier(
            "correlation identifier",
            self.correlation_id.as_deref(),
        )?;

        validate_optional_identifier(
            "client reference",
            self.client_reference.as_deref(),
        )?;

        validate_optional_identifier(
            "provenance reference",
            self.provenance_reference.as_deref(),
        )?;

        if let Some(reference) = &self.authentication_reference {
            validate_identifier(
                "authentication evidence reference",
                reference.as_str(),
            )?;
        }

        if !self.authorization.is_permitted()
            && matches!(
                self.outcome,
                AuditOperationOutcome::Started
                    | AuditOperationOutcome::Succeeded
                    | AuditOperationOutcome::PartiallySucceeded
            )
        {
            return Err(validation_error(
                "an operation cannot be recorded as started or successful without explicit authorization permission",
            ));
        }

        if !self.authorization.is_permitted()
            && !matches!(
                self.outcome,
                AuditOperationOutcome::Denied
                    | AuditOperationOutcome::Unresolved
                    | AuditOperationOutcome::NotEvaluated
            )
        {
            return Err(validation_error(
                "a non-permitted authorization decision has an inconsistent operation outcome",
            ));
        }

        if self.authorization.is_permitted()
            && self.outcome == AuditOperationOutcome::Denied
        {
            return Err(validation_error(
                "an explicitly permitted authorization decision cannot be represented as an authorization denial",
            ));
        }

        Ok(())
    }

    /// Return the audit schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Return the event identifier.
    #[must_use]
    pub fn event_id(&self) -> &str {
        &self.event_id
    }

    /// Return the occurrence timestamp.
    #[must_use]
    pub fn occurred_at(&self) -> &str {
        &self.occurred_at
    }

    /// Return the event category.
    #[must_use]
    pub const fn kind(&self) -> AuditEventKind {
        self.kind
    }

    /// Return the authenticated principal.
    #[must_use]
    pub fn principal(&self) -> &str {
        &self.principal
    }

    /// Return the requested command identity.
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Return the protected resource.
    #[must_use]
    pub fn resource(&self) -> &str {
        &self.resource
    }

    /// Return the optional request correlation identifier.
    #[must_use]
    pub fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }

    /// Return the optional authentication evidence reference.
    #[must_use]
    pub fn authentication_reference(
        &self,
    ) -> Option<&AuthenticationEvidenceReference> {
        self.authentication_reference.as_ref()
    }

    /// Return the authorization effect.
    #[must_use]
    pub const fn authorization(
        &self,
    ) -> AuditAuthorizationEffect {
        self.authorization
    }

    /// Return the operation outcome.
    #[must_use]
    pub const fn outcome(&self) -> AuditOperationOutcome {
        self.outcome
    }

    /// Return optional descriptive client metadata.
    #[must_use]
    pub fn client_reference(&self) -> Option<&str> {
        self.client_reference.as_deref()
    }

    /// Return the event source.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Return the optional provenance reference.
    #[must_use]
    pub fn provenance_reference(&self) -> Option<&str> {
        self.provenance_reference.as_deref()
    }
}

// ============================================================================
// AUDIT STORAGE CONTRACT
// ============================================================================

/// Result of appending an audit event.
///
/// The distinction supports idempotent retries without creating
/// duplicate logical audit records.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum AuditAppendOutcome {
    /// The event was newly accepted by the sink.
    Inserted,

    /// The identical event was already recorded.
    ///
    /// The sink must verify that the existing event has identical
    /// canonical contents before returning this result.
    AlreadyPresent,
}

/// Contract for an authoritative audit-storage provider.
///
/// Implementations may use a local append-only journal, database,
/// remote audit service, distributed ledger or another compatible
/// storage technology.
///
/// Storage implementation details remain outside this module.
///
/// Implementations must be thread-safe and must not silently convert
/// storage errors into successful acknowledgements.
pub trait AuditSink: Send + Sync + 'static {
    /// Append one validated audit event.
    ///
    /// Implementations must provide idempotency based on event_id.
    ///
    /// Reusing an event identifier for different event contents
    /// must return an error.
    ///
    /// A successful return must satisfy the durability guarantees
    /// documented by the implementation.
    fn append(
        &self,
        event: &AuditEvent,
    ) -> ToolchainResult<AuditAppendOutcome>;
}

// ============================================================================
// AUDIT RECORDER
// ============================================================================

/// Thread-safe audit recording service.
///
/// The recorder performs structural validation and delegates
/// persistence to the injected authoritative audit sink.
///
/// It does not authenticate, authorize, execute, sign, encrypt,
/// or independently verify the event.
#[derive(Clone)]
pub struct AuditRecorder {
    sink: Arc<dyn AuditSink>,
}

impl fmt::Debug for AuditRecorder {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("AuditRecorder")
            .finish_non_exhaustive()
    }
}

impl AuditRecorder {
    /// Construct an audit recorder using an explicit storage provider.
    ///
    /// There is deliberately no implicit in-memory or no-op fallback.
    pub fn new(
        sink: Arc<dyn AuditSink>,
    ) -> Self {
        Self { sink }
    }

    /// Validate and persist one audit event.
    ///
    /// Storage failures are propagated to the caller.
    ///
    /// A caller must not report that the audit was successfully
    /// recorded when this operation returns an error.
    pub fn record(
        &self,
        event: &AuditEvent,
    ) -> ToolchainResult<AuditAppendOutcome> {
        event.validate()?;

        self.sink
            .append(event)
            .map_err(|error| {
                ToolchainError::resource_exhausted(
                    "audit event could not be durably accepted by the configured audit sink",
                )
                .with_operation(RECORD_OPERATION)
                .with_source(error)
                .with_context(COMPONENT)
            })
    }

    /// Return a reference to the configured audit sink.
    ///
    /// This exposes the provider boundary, not a storage bypass.
    #[must_use]
    pub fn sink(&self) -> &Arc<dyn AuditSink> {
        &self.sink
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorization_effects_are_explicit() {
        assert!(
            AuditAuthorizationEffect::Permit.is_permitted()
        );

        assert!(
            !AuditAuthorizationEffect::Deny.is_permitted()
        );

        assert!(
            !AuditAuthorizationEffect::NotApplicable.is_permitted()
        );

        assert!(
            !AuditAuthorizationEffect::Indeterminate.is_permitted()
        );
    }

    #[test]
    fn authorization_effects_have_stable_names() {
        assert_eq!(
            AuditAuthorizationEffect::Permit.as_str(),
            "permit"
        );

        assert_eq!(
            AuditAuthorizationEffect::Deny.as_str(),
            "deny"
        );

        assert_eq!(
            AuditAuthorizationEffect::NotApplicable.as_str(),
            "not_applicable"
        );

        assert_eq!(
            AuditAuthorizationEffect::Indeterminate.as_str(),
            "indeterminate"
        );
    }

    #[test]
    fn operation_outcomes_are_distinct() {
        assert!(
            AuditOperationOutcome::Succeeded.is_success()
        );

        assert!(
            !AuditOperationOutcome::PartiallySucceeded.is_success()
        );

        assert!(
            AuditOperationOutcome::Failed.is_terminal()
        );

        assert!(
            !AuditOperationOutcome::Started.is_terminal()
        );
    }

    #[test]
    fn event_categories_have_stable_names() {
        assert_eq!(
            AuditEventKind::Authorization.as_str(),
            "authorization"
        );

        assert_eq!(
            AuditEventKind::OperationCompleted.as_str(),
            "operation_completed"
        );

        assert_eq!(
            AuditEventKind::OperationFailed.as_str(),
            "operation_failed"
        );
    }

    #[test]
    fn rejects_empty_event_identifiers() {
        assert!(
            validate_identifier("event identifier", "").is_err()
        );

        assert!(
            validate_identifier("event identifier", "  ").is_err()
        );
    }

    #[test]
    fn rejects_surrounding_identifier_whitespace() {
        assert!(
            validate_identifier(
                "principal",
                " user:alice "
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_control_characters() {
        assert!(
            validate_identifier(
                "principal",
                "user:\nalice"
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_empty_timestamps() {
        assert!(validate_timestamp("").is_err());
        assert!(validate_timestamp("   ").is_err());
    }

    #[test]
    fn accepts_non_secret_authentication_references() {
        let reference = AuthenticationEvidenceReference::new(
            "identity-provider:session-reference",
        )
        .expect("valid evidence reference");

        assert_eq!(
            reference.as_str(),
            "identity-provider:session-reference"
        );
    }

    #[test]
    fn rejects_empty_authentication_references() {
        assert!(
            AuthenticationEvidenceReference::new("").is_err()
        );
    }

    #[test]
    fn event_schema_version_is_explicit() {
        assert_eq!(AUDIT_SCHEMA_VERSION, 1);
    }

    #[test]
    fn audit_schema_serializes_without_implicit_data() {
        let effect = AuditAuthorizationEffect::Indeterminate;

        let encoded = serde_json::to_string(&effect)
            .expect("serialize authorization effect");

        assert_eq!(encoded, "\"indeterminate\"");

        let decoded: AuditAuthorizationEffect =
            serde_json::from_str(&encoded)
                .expect("deserialize authorization effect");

        assert_eq!(decoded, effect);
    }

    #[test]
    fn audit_outcomes_round_trip() {
        let outcomes = [
            AuditOperationOutcome::Accepted,
            AuditOperationOutcome::Started,
            AuditOperationOutcome::Succeeded,
            AuditOperationOutcome::PartiallySucceeded,
            AuditOperationOutcome::Failed,
            AuditOperationOutcome::Denied,
            AuditOperationOutcome::Cancelled,
            AuditOperationOutcome::Unresolved,
            AuditOperationOutcome::NotEvaluated,
        ];

        for outcome in outcomes {
            let encoded = serde_json::to_string(&outcome)
                .expect("serialize outcome");

            let decoded: AuditOperationOutcome =
                serde_json::from_str(&encoded)
                    .expect("deserialize outcome");

            assert_eq!(decoded, outcome);
        }
    }

    #[test]
    fn audit_event_kind_round_trips() {
        let kinds = [
            AuditEventKind::Authorization,
            AuditEventKind::OperationAccepted,
            AuditEventKind::OperationStarted,
            AuditEventKind::OperationCompleted,
            AuditEventKind::OperationFailed,
            AuditEventKind::OperationDenied,
            AuditEventKind::OperationCancelled,
            AuditEventKind::OperationUnresolved,
        ];

        for kind in kinds {
            let encoded = serde_json::to_string(&kind)
                .expect("serialize event kind");

            let decoded: AuditEventKind =
                serde_json::from_str(&encoded)
                    .expect("deserialize event kind");

            assert_eq!(decoded, kind);
        }
    }
}
