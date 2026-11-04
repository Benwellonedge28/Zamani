
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI Security — Audit
//!
//! Path: src/ai/security/audit.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Provides the AI-domain security audit boundary.
//!
//! This module records security-relevant AI activities and connects
//! those activities to the canonical identity, authorization,
//! provenance, execution, and audit-storage infrastructure.
//!
//! Supported activities include:
//!
//! - model access;
//! - model creation and modification;
//! - model training;
//! - inference;
//! - agent execution;
//! - agent coordination;
//! - dataset access;
//! - dataset transformation;
//! - pipeline execution;
//! - artifact creation and deployment;
//! - authorization decisions;
//! - security-policy evaluation outcomes;
//! - integrity verification;
//! - execution identity establishment;
//! - provenance-related security activities;
//! - distributed AI execution;
//! - AI security failures;
//! - AI security configuration changes.
//!
//! This module is an AI-domain integration boundary.
//!
//! It is not a general-purpose logging framework.
//!
//! ============================================================================
//! ARCHITECTURAL AUTHORITIES
//! ============================================================================
//!
//! Canonical owners:
//!
//! - ai/security/identity.rs
//!     AI identity and principal references.
//!
//! - ai/security/authorization.rs
//!     AI authorization integration.
//!
//! - ai/security/policies.rs
//!     AI security policy attachments.
//!
//! - ai/provenance.rs
//!     AI provenance adaptation.
//!
//! - frontend/semantic/model/provenance.rs
//!     Universal provenance representation and graph.
//!
//! - frontend/semantic/model/ids.rs
//!     Canonical semantic provenance identities.
//!
//! - distributed/security/authentication.rs
//!     Authentication and session lifecycle.
//!
//! - distributed/security/authorization.rs
//!     Distributed authorization orchestration.
//!
//! - distributed/security/policy.rs
//!     Policy evaluation and enforcement.
//!
//! - ai/observability/events.rs
//!     AI operational event integration.
//!
//! - runtime/
//!     Execution lifecycle and operational integration.
//!
//! - grammar/security/audit.g4
//!     Source-level security audit intent.
//!
//! - grammar/security/security.g4
//!     Security grammar composition.
//!
//! - grammar/ai/ai.g4
//!     AI grammar composition.
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - AI-specific audit event descriptions;
//! - AI security audit event classification;
//! - AI audit event construction;
//! - audit event identity and correlation;
//! - audit outcome representation;
//! - AI principal and resource references in audit records;
//! - canonical provenance references in audit records;
//! - audit metadata validation;
//! - audit sink integration;
//! - explicit persistence acknowledgement;
//! - audit recording failure propagation.
//!
//! ============================================================================
//! DOES NOT OWN
//! ============================================================================
//!
//! This module does not own:
//!
//! - identity allocation;
//! - identity authentication;
//! - credential management;
//! - cryptographic key management;
//! - trust establishment;
//! - authorization decisions;
//! - policy evaluation;
//! - permission grants;
//! - capability issuance;
//! - policy storage;
//! - canonical provenance graph storage;
//! - general logging;
//! - general telemetry;
//! - distributed tracing;
//! - hardware discovery;
//! - resource allocation;
//! - scheduling;
//! - model execution;
//! - distributed transport;
//! - audit database implementation;
//! - audit filesystem implementation;
//! - retention enforcement;
//! - cryptographic signature generation;
//! - cryptographic verification.
//!
//! Those responsibilities remain with their respective authorities.
//!
//! ============================================================================
//! SECURITY INVARIANTS
//! ============================================================================
//!
//! 1. An identity reference is not authentication.
//! 2. An audit event is not authorization.
//! 3. An audit event cannot grant permission.
//! 4. An AI model cannot authorize itself.
//! 5. A principal reference cannot prove a successful authentication.
//! 6. A policy decision cannot be changed by the audit subsystem.
//! 7. Denied operations must remain distinguishable from permitted ones.
//! 8. Indeterminate decisions must not become successful decisions.
//! 9. Failed audit persistence must not be reported as successful.
//! 10. Missing audit storage must not silently become successful storage.
//! 11. Audit metadata must not be used as a secret-storage mechanism.
//! 12. Raw credentials must never be deliberately recorded.
//! 13. Audit records must preserve their provenance references.
//! 14. An audit storage acknowledgement must identify the accepted record.
//! 15. An audit sink must not acknowledge a record before satisfying
//!     its declared append contract.
//! 16. No internal unbounded buffering is introduced.
//! 17. Audit events must not be silently discarded.
//! 18. Audit failure must remain distinguishable from operation failure.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! Audit describes logical AI security activities.
//!
//! It does not encode:
//!
//! - CPU architecture;
//! - GPU architecture;
//! - accelerator vendor;
//! - operating system;
//! - memory capacity;
//! - processor count;
//! - accelerator count;
//! - node count;
//! - worker count;
//! - cluster topology;
//! - physical hardware identity;
//! - quantum processor topology.
//!
//! The same logical AI audit event can be represented independently
//! of its eventual execution environment.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! This module imposes no arbitrary semantic maximum on:
//!
//! - AI principals;
//! - models;
//! - datasets;
//! - agents;
//! - pipelines;
//! - training runs;
//! - executions;
//! - audit event categories;
//! - audit records;
//! - provenance references;
//! - metadata fields;
//! - storage providers;
//! - distributed execution environments.
//!
//! Collections are dynamically sized.
//!
//! Actual memory, storage, representation, and provider limits remain
//! operational constraints.
//!
//! No language-level resource ceiling is introduced.
//!
//! Literal infinite physical storage or computation is not promised.
//!
//! ============================================================================
//! FAILURE MODEL
//! ============================================================================
//!
//! Audit persistence is explicit.
//!
//! A successful append means that the configured sink has accepted
//! responsibility for the record according to its documented durability
//! contract.
//!
//! An in-memory queue, successful serialization, or successful network
//! submission alone must not be represented as durable persistence.
//!
//! The caller must distinguish:
//!
//! - protected operation failure;
//! - audit recording failure;
//! - uncertain protected operation outcome;
//! - uncertain audit persistence outcome.
//!
//! This module does not make arbitrary external operations transactional.
//!
//! ============================================================================
//! DEPENDENCIES
//! ============================================================================
//!
//! Uses:
//!
//! - serde;
//! - std::collections::BTreeMap;
//! - std::error::Error;
//! - std::sync::Arc;
//! - std::fmt;
//! - std::time-independent timestamp representation.
//!
//! Reuses:
//!
//! - AiPrincipalRef;
//! - AiIdentityRef;
//! - AiExecutionRef;
//! - canonical ProvenanceId.
//!
//! ============================================================================
//! INTEGRATION
//! ============================================================================
//!
//! identity.rs
//!     -> authenticated-subject identity references.
//!
//! authorization.rs
//!     -> records AI authorization outcomes after canonical evaluation.
//!
//! policies.rs
//!     -> records policy-related audit references.
//!
//! provenance.rs
//!     -> preserves canonical provenance identifiers.
//!
//! frontend/semantic
//!     -> consumes source-level audit intent.
//!
//! distributed/security
//!     -> supplies authenticated identity and policy outcomes.
//!
//! observability/events.rs
//!     -> receives suitable operational event projections.
//!
//! runtime
//!     -> invokes the audit boundary at protected-operation boundaries.
//!
//! grammar/security/audit.g4
//!     -> source audit requirements and audit declarations.
//!
//! ============================================================================
//! FREEZE CRITERIA
//! ============================================================================
//!
//! - No duplicate identity implementation.
//! - No duplicate authorization implementation.
//! - No duplicate provenance graph.
//! - Audit events are structured.
//! - Audit outcomes are explicit.
//! - Event timestamps are supplied by the trusted caller.
//! - Audit persistence is provider-neutral.
//! - Audit failures propagate.
//! - Secret-bearing metadata names are rejected.
//! - No arbitrary semantic limits.
//! - No global mutable state.
//! - No unsafe Rust.

use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::ai::security::identity::{
    AiExecutionRef,
    AiIdentityRef,
    AiPrincipalRef,
};

use crate::frontend::semantic::model::ids::ProvenanceId;

// ============================================================================
// RESULT
// ============================================================================

/// Result returned by AI security audit operations.
pub type AiAuditResult<T> = Result<T, AiAuditError>;

// ============================================================================
// SCHEMA VERSION
// ============================================================================

/// Independent AI audit record schema version.
///
/// This is not the Zamani language version, AI API version,
/// semantic provenance version, or ZUIR dialect version.
pub const AI_AUDIT_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// EVENT IDENTIFIER
// ============================================================================

/// Opaque identifier supplied by the authoritative event producer.
///
/// The audit subsystem does not allocate globally unique event IDs.
///
/// The caller must provide an identifier unique within the configured
/// audit identity namespace.
///
/// Persistent global uniqueness is the responsibility of the configured
/// event identity provider.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
)]
pub struct AiAuditEventId(String);

impl AiAuditEventId {
    /// Construct a validated audit event identifier.
    pub fn new(value: impl Into<String>) -> AiAuditResult<Self> {
        let value = value.into();

        validate_nonempty_text("event identifier", &value)?;

        Ok(Self(value))
    }

    /// Return the event identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the event identifier.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for AiAuditEventId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// EVENT TIMESTAMP
// ============================================================================

/// Explicit UTC Unix timestamp.
///
/// The timestamp is supplied by a trusted runtime or event producer.
///
/// This module does not read the system clock.
///
/// Nanoseconds must be within the conventional subsecond interval.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct AiAuditTimestamp {
    unix_seconds: i64,
    nanoseconds: u32,
}

impl AiAuditTimestamp {
    /// Construct a timestamp.
    pub fn new(
        unix_seconds: i64,
        nanoseconds: u32,
    ) -> AiAuditResult<Self> {
        if nanoseconds >= 1_000_000_000 {
            return Err(AiAuditError::InvalidTimestamp);
        }

        Ok(Self {
            unix_seconds,
            nanoseconds,
        })
    }

    /// Return Unix seconds.
    pub const fn unix_seconds(&self) -> i64 {
        self.unix_seconds
    }

    /// Return the subsecond nanosecond component.
    pub const fn nanoseconds(&self) -> u32 {
        self.nanoseconds
    }
}

// ============================================================================
// AUDIT EVENT CATEGORY
// ============================================================================

/// Broad AI security activity classification.
///
/// This enumeration classifies established event families.
///
/// Event names and operation identifiers remain open-world so that
/// future AI computational domains do not require changes to this type.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum AiAuditCategory {
    /// AI model access, creation, modification, or removal.
    Model,

    /// AI model inference.
    Inference,

    /// AI model training and adaptation.
    Training,

    /// Dataset access or transformation.
    Dataset,

    /// AI agent activity.
    Agent,

    /// AI pipeline activity.
    Pipeline,

    /// AI artifact activity.
    Artifact,

    /// AI reasoning activity.
    Reasoning,

    /// AI authorization-related activity.
    Authorization,

    /// AI authentication-related activity.
    Authentication,

    /// AI integrity verification.
    Integrity,

    /// AI provenance-related activity.
    Provenance,

    /// AI deployment activity.
    Deployment,

    /// AI security configuration activity.
    SecurityConfiguration,

    /// Extensible AI security activity.
    Extension,
}

// ============================================================================
// SECURITY DECISION
// ============================================================================

/// Recorded authorization or security decision.
///
/// This is an immutable description of a decision made elsewhere.
///
/// Creating this value does not perform policy evaluation or grant access.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum AiAuditDecision {
    /// An authoritative policy explicitly permitted the request.
    Permit,

    /// An authoritative policy explicitly denied the request.
    Deny,

    /// No applicable policy produced a decision.
    NotApplicable,

    /// Evaluation could not produce a trustworthy decision.
    Indeterminate,

    /// No authorization decision was applicable to the event.
    NotEvaluated,
}

// ============================================================================
// OPERATION RESULT
// ============================================================================

/// Outcome of the audited operation.
///
/// This is distinct from the authorization decision.
///
/// A permitted operation can still fail during execution.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum AiAuditOutcome {
    /// The operation completed successfully.
    Succeeded,

    /// The operation was rejected.
    Rejected,

    /// The operation failed.
    Failed,

    /// The operation was cancelled.
    Cancelled,

    /// The operation exceeded its deadline.
    TimedOut,

    /// The final operation outcome is unknown.
    Unknown,
}

// ============================================================================
// AUDIT SOURCE REFERENCE
// ============================================================================

/// Portable reference to the source construct associated with an event.
///
/// The frontend owns canonical AST nodes and source spans.
///
/// This structure carries an external reference rather than defining
/// a second AST or source-map representation.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
)]
pub struct AiAuditSourceRef {
    source: String,
    line: Option<u64>,
    column: Option<u64>,
}

impl AiAuditSourceRef {
    /// Construct a source reference.
    ///
    /// Line and column must either both be present or both be absent.
    pub fn new(
        source: impl Into<String>,
        line: Option<u64>,
        column: Option<u64>,
    ) -> AiAuditResult<Self> {
        let source = source.into();

        validate_nonempty_text("source reference", &source)?;

        if line.is_some() != column.is_some() {
            return Err(AiAuditError::IncompleteSourceLocation);
        }

        if line == Some(0) || column == Some(0) {
            return Err(AiAuditError::InvalidSourceLocation);
        }

        Ok(Self {
            source,
            line,
            column,
        })
    }

    /// Return the source reference.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Return the optional source line.
    pub const fn line(&self) -> Option<u64> {
        self.line
    }

    /// Return the optional source column.
    pub const fn column(&self) -> Option<u64> {
        self.column
    }
}

// ============================================================================
// AUDIT METADATA
// ============================================================================

/// Validated audit metadata field name.
///
/// Metadata keys are open-ended but must not identify obvious
/// credential or secret-bearing fields.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
)]
pub struct AiAuditFieldName(String);

impl AiAuditFieldName {
    /// Construct a validated audit field name.
    pub fn new(value: impl Into<String>) -> AiAuditResult<Self> {
        let value = value.into();

        validate_nonempty_text("metadata field name", &value)?;

        if value.chars().any(char::is_whitespace) {
            return Err(AiAuditError::InvalidMetadataField);
        }

        if contains_sensitive_field_name(&value) {
            return Err(AiAuditError::SensitiveMetadataField);
        }

        Ok(Self(value))
    }

    /// Return the field name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validated descriptive metadata value.
///
/// Values must not contain credentials, private keys, access tokens,
/// unrestricted secret material, or raw private payloads.
///
/// The audit producer remains responsible for data classification.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
)]
pub struct AiAuditFieldValue(String);

impl AiAuditFieldValue {
    /// Construct a descriptive audit metadata value.
    pub fn new(value: impl Into<String>) -> AiAuditResult<Self> {
        let value = value.into();

        if value.chars().any(char::is_control) {
            return Err(AiAuditError::InvalidMetadataValue);
        }

        Ok(Self(value))
    }

    /// Return the descriptive value.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// AUDIT EVENT
// ============================================================================

/// Immutable structured AI security audit event.
///
/// The event captures logical activity, not the physical hardware
/// on which that activity occurred.
///
/// Principal and resource references remain distinct.
///
/// An identity reference does not establish authentication.
///
/// Provenance references point to the canonical semantic provenance
/// system. They do not create another provenance graph.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
)]
pub struct AiAuditEvent {
    schema_version: u16,

    event_id: AiAuditEventId,

    timestamp: AiAuditTimestamp,

    category: AiAuditCategory,

    action: String,

    principal: AiPrincipalRef,

    resource: Option<AiIdentityRef>,

    execution: Option<AiExecutionRef>,

    decision: AiAuditDecision,

    outcome: AiAuditOutcome,

    provenance: Vec<ProvenanceId>,

    source: Option<AiAuditSourceRef>,

    correlation_id: Option<String>,

    metadata: BTreeMap<AiAuditFieldName, AiAuditFieldValue>,
}

impl AiAuditEvent {
    /// Construct an AI security audit event.
    ///
    /// The event producer supplies the timestamp and identifier.
    ///
    /// The action is an open-ended symbolic identifier.
    ///
    /// The constructor does not authenticate the principal or
    /// authorize the operation.
    pub fn new(
        event_id: AiAuditEventId,
        timestamp: AiAuditTimestamp,
        category: AiAuditCategory,
        action: impl Into<String>,
        principal: AiPrincipalRef,
        decision: AiAuditDecision,
        outcome: AiAuditOutcome,
    ) -> AiAuditResult<Self> {
        let action = action.into();

        validate_nonempty_text("audit action", &action)?;

        if action.chars().any(char::is_whitespace) {
            return Err(AiAuditError::InvalidAction);
        }

        Ok(Self {
            schema_version: AI_AUDIT_SCHEMA_VERSION,
            event_id,
            timestamp,
            category,
            action,
            principal,
            resource: None,
            execution: None,
            decision,
            outcome,
            provenance: Vec::new(),
            source: None,
            correlation_id: None,
            metadata: BTreeMap::new(),
        })
    }

    /// Attach an AI resource reference.
    #[must_use]
    pub fn with_resource(
        mut self,
        resource: AiIdentityRef,
    ) -> Self {
        self.resource = Some(resource);
        self
    }

    /// Attach the execution identity reference.
    ///
    /// This reference does not prove that the execution has been
    /// authenticated. Authentication remains a runtime responsibility.
    #[must_use]
    pub fn with_execution(
        mut self,
        execution: AiExecutionRef,
    ) -> Self {
        self.execution = Some(execution);
        self
    }

    /// Attach a source reference.
    #[must_use]
    pub fn with_source(
        mut self,
        source: AiAuditSourceRef,
    ) -> Self {
        self.source = Some(source);
        self
    }

    /// Attach a correlation identifier.
    ///
    /// Correlation identifiers must be supplied by the caller.
    /// This module does not generate globally unique identifiers.
    pub fn with_correlation_id(
        mut self,
        value: impl Into<String>,
    ) -> AiAuditResult<Self> {
        let value = value.into();

        validate_nonempty_text("correlation identifier", &value)?;

        self.correlation_id = Some(value);

        Ok(self)
    }

    /// Append a canonical provenance reference.
    ///
    /// The referenced record must already exist in, or be coordinated
    /// with, the canonical provenance lifecycle.
    ///
    /// This method does not create or persist a provenance record.
    #[must_use]
    pub fn with_provenance(
        mut self,
        provenance: ProvenanceId,
    ) -> Self {
        if !self.provenance.contains(&provenance) {
            self.provenance.push(provenance);
        }

        self
    }

    /// Add validated descriptive metadata.
    ///
    /// Duplicate keys replace their previous descriptive value.
    ///
    /// Secret-bearing field names are rejected.
    pub fn insert_metadata(
        &mut self,
        name: AiAuditFieldName,
        value: AiAuditFieldValue,
    ) -> Option<AiAuditFieldValue> {
        self.metadata.insert(name, value)
    }

    /// Return the event schema version.
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Return the event identifier.
    pub fn event_id(&self) -> &AiAuditEventId {
        &self.event_id
    }

    /// Return the timestamp.
    pub const fn timestamp(&self) -> AiAuditTimestamp {
        self.timestamp
    }

    /// Return the event category.
    pub const fn category(&self) -> AiAuditCategory {
        self.category
    }

    /// Return the symbolic action.
    pub fn action(&self) -> &str {
        &self.action
    }

    /// Return the principal reference.
    pub fn principal(&self) -> &AiPrincipalRef {
        &self.principal
    }

    /// Return the optional AI resource.
    pub fn resource(&self) -> Option<&AiIdentityRef> {
        self.resource.as_ref()
    }

    /// Return the optional execution identity.
    pub fn execution(&self) -> Option<&AiExecutionRef> {
        self.execution.as_ref()
    }

    /// Return the authorization decision.
    pub const fn decision(&self) -> AiAuditDecision {
        self.decision
    }

    /// Return the operation outcome.
    pub const fn outcome(&self) -> AiAuditOutcome {
        self.outcome
    }

    /// Return canonical provenance references.
    pub fn provenance(&self) -> &[ProvenanceId] {
        &self.provenance
    }

    /// Return the optional source reference.
    pub fn source(&self) -> Option<&AiAuditSourceRef> {
        self.source.as_ref()
    }

    /// Return the optional correlation identifier.
    pub fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }

    /// Return the event metadata.
    pub fn metadata(
        &self,
    ) -> &BTreeMap<AiAuditFieldName, AiAuditFieldValue> {
        &self.metadata
    }
}

// ============================================================================
// AUDIT APPEND RECEIPT
// ============================================================================

/// Acknowledgement returned by an audit storage provider.
///
/// The receipt identifies the stored or accepted event according
/// to the provider's documented persistence contract.
///
/// It is not itself a cryptographic proof of authenticity.
///
/// Cryptographic attestation and tamper-evident verification remain
/// responsibilities of the configured storage and integrity providers.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
)]
pub struct AiAuditReceipt {
    event_id: AiAuditEventId,
    storage_reference: String,
    durability: AiAuditDurability,
}

impl AiAuditReceipt {
    /// Construct a storage acknowledgement.
    ///
    /// The configured sink must only call this after satisfying
    /// its declared append contract.
    pub fn new(
        event_id: AiAuditEventId,
        storage_reference: impl Into<String>,
        durability: AiAuditDurability,
    ) -> AiAuditResult<Self> {
        let storage_reference = storage_reference.into();

        validate_nonempty_text(
            "audit storage reference",
            &storage_reference,
        )?;

        Ok(Self {
            event_id,
            storage_reference,
            durability,
        })
    }

    /// Return the acknowledged event identifier.
    pub fn event_id(&self) -> &AiAuditEventId {
        &self.event_id
    }

    /// Return the provider's storage reference.
    pub fn storage_reference(&self) -> &str {
        &self.storage_reference
    }

    /// Return the acknowledged durability class.
    pub const fn durability(&self) -> AiAuditDurability {
        self.durability
    }
}

/// Declared durability level of an audit append acknowledgement.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum AiAuditDurability {
    /// The provider accepted the event but has not established
    /// durable persistence.
    Accepted,

    /// The provider confirms persistence according to its documented
    /// durable-storage contract.
    Durable,

    /// The provider confirms replication according to its documented
    /// replication contract.
    Replicated,
}

// ============================================================================
// AUDIT STORAGE PROVIDER
// ============================================================================

/// Provider-neutral AI audit append interface.
///
/// Implementations may integrate with:
///
/// - durable databases;
/// - append-only storage;
/// - remote audit services;
/// - distributed audit infrastructure;
/// - tamper-evident storage;
/// - externally managed security event systems.
///
/// Implementations must document:
///
/// - their durability semantics;
/// - their atomicity semantics;
/// - duplicate-event handling;
/// - concurrency behavior;
/// - partial-failure behavior;
/// - acknowledgement semantics;
/// - integrity guarantees;
/// - recovery behavior.
///
/// A provider must not report Durable or Replicated unless its
/// declared contract has actually been satisfied.
///
/// Implementations must not silently discard an event and report success.
pub trait AiAuditSink: Send + Sync {
    /// Append one audit event.
    ///
    /// The implementation must either:
    ///
    /// - return a valid receipt;
    /// - return an explicit error.
    ///
    /// A provider must not manufacture successful acknowledgements
    /// after an uncertain persistence outcome.
    fn append(
        &self,
        event: &AiAuditEvent,
    ) -> AiAuditResult<AiAuditReceipt>;
}

// ============================================================================
// AUDIT SERVICE
// ============================================================================

/// AI security audit service.
///
/// This service delegates persistence to an explicitly configured
/// audit sink.
///
/// It does not create an implicit in-memory fallback.
///
/// It does not maintain an internal unbounded event queue.
///
/// It does not own event identity allocation.
///
/// It does not implement cryptographic audit storage.
pub struct AiAuditService {
    sink: Arc<dyn AiAuditSink>,
}

impl AiAuditService {
    /// Construct the AI audit service.
    ///
    /// The supplied sink must implement the required persistence,
    /// concurrency, durability, and integrity contracts.
    ///
    /// No default allow-all or discard-only sink is created.
    pub fn new(
        sink: Arc<dyn AiAuditSink>,
    ) -> Self {
        Self {
            sink,
        }
    }

    /// Record one AI security audit event.
    ///
    /// Returns only after the configured provider acknowledges
    /// the append operation.
    ///
    /// The caller must inspect the receipt's durability class.
    ///
    /// A successful Accepted receipt must not be interpreted
    /// as durable persistence.
    pub fn record(
        &self,
        event: &AiAuditEvent,
    ) -> AiAuditResult<AiAuditReceipt> {
        let receipt = self.sink.append(event)?;

        if receipt.event_id() != event.event_id() {
            return Err(AiAuditError::ReceiptMismatch);
        }

        Ok(receipt)
    }

    /// Record an event and require the configured sink to confirm
    /// durable persistence or stronger.
    ///
    /// This is the preferred boundary for operations whose security
    /// contract requires durable audit evidence before proceeding.
    ///
    /// The caller must still handle the possibility that an external
    /// operation and its audit append cannot be made transactional.
    pub fn record_durable(
        &self,
        event: &AiAuditEvent,
    ) -> AiAuditResult<AiAuditReceipt> {
        let receipt = self.record(event)?;

        match receipt.durability() {
            AiAuditDurability::Durable
            | AiAuditDurability::Replicated => Ok(receipt),

            AiAuditDurability::Accepted => {
                Err(AiAuditError::DurabilityNotSatisfied)
            }
        }
    }
}

impl fmt::Debug for AiAuditService {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("AiAuditService")
            .field(
                "sink",
                &"[CONFIGURED AI AUDIT STORAGE PROVIDER]",
            )
            .finish()
    }
}

// ============================================================================
// AUDIT ERRORS
// ============================================================================

/// AI security audit failures.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
#[non_exhaustive]
pub enum AiAuditError {
    /// A required text field is empty.
    EmptyField {
        field: &'static str,
    },

    /// An invalid timestamp was supplied.
    InvalidTimestamp,

    /// Source location information is incomplete.
    IncompleteSourceLocation,

    /// Source location information is invalid.
    InvalidSourceLocation,

    /// Audit action is malformed.
    InvalidAction,

    /// Metadata field name is malformed.
    InvalidMetadataField,

    /// Metadata field name indicates secret-bearing content.
    SensitiveMetadataField,

    /// Metadata value contains prohibited control characters.
    InvalidMetadataValue,

    /// Storage provider acknowledged a different event.
    ReceiptMismatch,

    /// Required durable persistence was not established.
    DurabilityNotSatisfied,

    /// Audit storage failed.
    StorageFailure {
        message: String,
    },

    /// Audit storage reported an uncertain outcome.
    UncertainPersistence {
        message: String,
    },
}

impl fmt::Display for AiAuditError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(
                    formatter,
                    "AI audit {field} must not be empty"
                )
            }

            Self::InvalidTimestamp => {
                formatter.write_str(
                    "AI audit timestamp has an invalid nanosecond component",
                )
            }

            Self::IncompleteSourceLocation => {
                formatter.write_str(
                    "AI audit source line and column must be supplied together",
                )
            }

            Self::InvalidSourceLocation => {
                formatter.write_str(
                    "AI audit source line and column must be non-zero",
                )
            }

            Self::InvalidAction => {
                formatter.write_str(
                    "AI audit action contains whitespace",
                )
            }

            Self::InvalidMetadataField => {
                formatter.write_str(
                    "AI audit metadata field name is invalid",
                )
            }

            Self::SensitiveMetadataField => {
                formatter.write_str(
                    "AI audit metadata field may contain secret material",
                )
            }

            Self::InvalidMetadataValue => {
                formatter.write_str(
                    "AI audit metadata value contains prohibited control characters",
                )
            }

            Self::ReceiptMismatch => {
                formatter.write_str(
                    "AI audit storage receipt does not match the submitted event",
                )
            }

            Self::DurabilityNotSatisfied => {
                formatter.write_str(
                    "AI audit provider did not establish required durable persistence",
                )
            }

            Self::StorageFailure { message } => {
                write!(
                    formatter,
                    "AI audit storage failure: {message}"
                )
            }

            Self::UncertainPersistence { message } => {
                write!(
                    formatter,
                    "AI audit persistence outcome is uncertain: {message}"
                )
            }
        }
    }
}

impl Error for AiAuditError {}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

/// Validate non-empty descriptive text.
///
/// No arbitrary maximum length is imposed.
fn validate_nonempty_text(
    field: &'static str,
    value: &str,
) -> AiAuditResult<()> {
    if value.trim().is_empty() {
        return Err(AiAuditError::EmptyField { field });
    }

    if value.chars().any(char::is_control) {
        return Err(AiAuditError::InvalidMetadataValue);
    }

    Ok(())
}

/// Detect common credential-bearing field names.
///
/// This is a defensive field-name filter, not a complete secret detector.
///
/// Producers must still classify and redact sensitive values before
/// constructing an audit event.
fn contains_sensitive_field_name(
    value: &str,
) -> bool {
    let normalized: String = value
        .chars()
        .filter(|character| {
            character.is_ascii_alphanumeric()
        })
        .flat_map(char::to_lowercase)
        .collect();

    const SENSITIVE_NAMES: &[&str] = &[
        "password",
        "passwd",
        "privatekey",
        "secret",
        "accesstoken",
        "refreshtoken",
        "bearertoken",
        "authorizationheader",
        "credential",
        "clientsecret",
        "apikey",
        "signingkey",
        "recoverysecret",
    ];

    SENSITIVE_NAMES
        .iter()
        .any(|candidate| normalized.contains(candidate))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Mutex;

    use crate::ai::ids::{
        AiAgentId,
        AiArtifactId,
        AiDatasetId,
        AiModelId,
        AiPipelineId,
        AiTrainingRunId,
    };

    use crate::ai::security::identity::{
        AiEntityRef,
        IdentityAuthority,
    };

    // ------------------------------------------------------------------------
    // Test identity helpers
    // ------------------------------------------------------------------------

    fn principal() -> AiPrincipalRef {
        let authority = IdentityAuthority::new(
            "zamani::test",
        )
        .expect("valid authority");

        AiPrincipalRef::agent(
            authority,
            AiAgentId::new(1).expect("valid agent ID"),
        )
    }

    fn event() -> AiAuditEvent {
        AiAuditEvent::new(
            AiAuditEventId::new("audit-event-1")
                .expect("valid event ID"),
            AiAuditTimestamp::new(1_800_000_000, 0)
                .expect("valid timestamp"),
            AiAuditCategory::Inference,
            "ai.model.infer",
            principal(),
            AiAuditDecision::Permit,
            AiAuditOutcome::Succeeded,
        )
        .expect("valid audit event")
    }

    fn authority() -> IdentityAuthority {
        IdentityAuthority::new(
            "zamani::test",
        )
        .expect("valid authority")
    }

    fn identity_references_are_typed() {
        let _ = (
            AiEntityRef::Model(
                AiModelId::new(1).expect("model"),
            ),
            AiEntityRef::Dataset(
                AiDatasetId::new(1).expect("dataset"),
            ),
            AiEntityRef::Agent(
                AiAgentId::new(1).expect("agent"),
            ),
            AiEntityRef::TrainingRun(
                AiTrainingRunId::new(1).expect("training"),
            ),
            AiEntityRef::Pipeline(
                AiPipelineId::new(1).expect("pipeline"),
            ),
            AiEntityRef::Artifact(
                AiArtifactId::new(1).expect("artifact"),
            ),
        );
    }

    // ------------------------------------------------------------------------
    // Test sink
    // ------------------------------------------------------------------------

    #[derive(Default)]
    struct MemoryTestSink {
        events: Mutex<Vec<AiAuditEvent>>,
        durability: AiAuditDurability,
        fail: bool,
    }

    impl AiAuditSink for MemoryTestSink {
        fn append(
            &self,
            event: &AiAuditEvent,
        ) -> AiAuditResult<AiAuditReceipt> {
            if self.fail {
                return Err(
                    AiAuditError::StorageFailure {
                        message: "test storage failure".to_owned(),
                    },
                );
            }

            self.events
                .lock()
                .map_err(|_| AiAuditError::StorageFailure {
                    message: "test lock poisoned".to_owned(),
                })?
                .push(event.clone());

            AiAuditReceipt::new(
                event.event_id().clone(),
                "test-storage-reference",
                self.durability,
            )
        }
    }

    // ------------------------------------------------------------------------
    // Event tests
    // ------------------------------------------------------------------------

    #[test]
    fn constructs_structured_audit_event() {
        let record = event();

        assert_eq!(
            record.category(),
            AiAuditCategory::Inference
        );

        assert_eq!(
            record.action(),
            "ai.model.infer"
        );

        assert_eq!(
            record.decision(),
            AiAuditDecision::Permit
        );

        assert_eq!(
            record.outcome(),
            AiAuditOutcome::Succeeded
        );
    }

    #[test]
    fn rejects_empty_event_identifier() {
        assert!(
            AiAuditEventId::new("").is_err()
        );
    }

    #[test]
    fn rejects_empty_action() {
        assert!(
            AiAuditEvent::new(
                AiAuditEventId::new("event-1").unwrap(),
                AiAuditTimestamp::new(1, 0).unwrap(),
                AiAuditCategory::Model,
                "",
                principal(),
                AiAuditDecision::NotEvaluated,
                AiAuditOutcome::Unknown,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_action_whitespace() {
        assert!(
            AiAuditEvent::new(
                AiAuditEventId::new("event-1").unwrap(),
                AiAuditTimestamp::new(1, 0).unwrap(),
                AiAuditCategory::Model,
                "ai.model infer",
                principal(),
                AiAuditDecision::NotEvaluated,
                AiAuditOutcome::Unknown,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_invalid_timestamp() {
        assert!(
            AiAuditTimestamp::new(
                1,
                1_000_000_000,
            )
            .is_err()
        );
    }

    #[test]
    fn preserves_explicit_decision_and_operation_outcome() {
        let record = AiAuditEvent::new(
            AiAuditEventId::new("event-2").unwrap(),
            AiAuditTimestamp::new(100, 0).unwrap(),
            AiAuditCategory::Authorization,
            "ai.model.train",
            principal(),
            AiAuditDecision::Deny,
            AiAuditOutcome::Rejected,
        )
        .unwrap();

        assert_eq!(
            record.decision(),
            AiAuditDecision::Deny
        );

        assert_eq!(
            record.outcome(),
            AiAuditOutcome::Rejected
        );
    }

    #[test]
    fn identity_and_resource_are_distinct() {
        let record = event().with_resource(
            crate::ai::security::identity::AiIdentityRef::new(
                authority(),
                AiEntityRef::Model(
                    AiModelId::new(9).unwrap(),
                ),
            ),
        );

        assert!(record.resource().is_some());

        assert_eq!(
            record.resource().unwrap().local_id(),
            9
        );
    }

    #[test]
    fn source_location_requires_both_coordinates() {
        assert!(
            AiAuditSourceRef::new(
                "src/example.zm",
                Some(10),
                None,
            )
            .is_err()
        );

        assert!(
            AiAuditSourceRef::new(
                "src/example.zm",
                Some(10),
                Some(4),
            )
            .is_ok()
        );
    }

    // ------------------------------------------------------------------------
    // Metadata tests
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_open_world_metadata_names() {
        assert!(
            AiAuditFieldName::new(
                "ai.execution.domain"
            )
            .is_ok()
        );

        assert!(
            AiAuditFieldName::new(
                "future.custom.extension"
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_secret_bearing_metadata_names() {
        for name in [
            "password",
            "access_token",
            "private-key",
            "client_secret",
            "authorization_header",
            "api_key",
        ] {
            assert!(
                AiAuditFieldName::new(name).is_err(),
                "expected rejection of {name}"
            );
        }
    }

    #[test]
    fn accepts_descriptive_metadata() {
        let mut record = event();

        let name = AiAuditFieldName::new(
            "execution.domain",
        )
        .unwrap();

        let value = AiAuditFieldValue::new(
            "distributed",
        )
        .unwrap();

        record.insert_metadata(name, value);

        assert_eq!(
            record.metadata().len(),
            1
        );
    }

    #[test]
    fn metadata_rejects_control_characters() {
        assert!(
            AiAuditFieldValue::new(
                "invalid\nvalue"
            )
            .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Persistence tests
    // ------------------------------------------------------------------------

    #[test]
    fn records_event_with_durable_acknowledgement() {
        let sink = Arc::new(MemoryTestSink {
            durability: AiAuditDurability::Durable,
            ..Default::default()
        });

        let service = AiAuditService::new(sink.clone());

        let receipt = service
            .record_durable(&event())
            .expect("durable audit append");

        assert_eq!(
            receipt.durability(),
            AiAuditDurability::Durable
        );

        assert_eq!(
            sink.events.lock().unwrap().len(),
            1
        );
    }

    #[test]
    fn replicated_acknowledgement_satisfies_durability() {
        let sink = Arc::new(MemoryTestSink {
            durability: AiAuditDurability::Replicated,
            ..Default::default()
        });

        let service = AiAuditService::new(sink);

        assert!(
            service.record_durable(&event()).is_ok()
        );
    }

    #[test]
    fn accepted_only_does_not_satisfy_durable_requirement() {
        let sink = Arc::new(MemoryTestSink {
            durability: AiAuditDurability::Accepted,
            ..Default::default()
        });

        let service = AiAuditService::new(sink);

        assert_eq!(
            service.record_durable(&event()),
            Err(AiAuditError::DurabilityNotSatisfied)
        );
    }

    #[test]
    fn storage_failure_is_propagated() {
        let sink = Arc::new(MemoryTestSink {
            durability: AiAuditDurability::Durable,
            fail: true,
            ..Default::default()
        });

        let service = AiAuditService::new(sink);

        assert!(
            matches!(
                service.record(&event()),
                Err(AiAuditError::StorageFailure { .. })
            )
        );
    }

    #[test]
    fn no_success_is_reported_when_storage_fails() {
        let sink = Arc::new(MemoryTestSink {
            durability: AiAuditDurability::Durable,
            fail: true,
            ..Default::default()
        });

        let service = AiAuditService::new(sink);

        let result = service.record_durable(&event());

        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // Serialization tests
    // ------------------------------------------------------------------------

    #[test]
    fn event_serializes_to_structured_json() {
        let record = event();

        let serialized = serde_json::to_string(
            &record,
        )
        .expect("event serialization");

        assert!(
            serialized.contains("ai.model.infer")
        );

        assert!(
            serialized.contains("schema_version")
        );

        assert!(
            serialized.contains("permit")
        );
    }

    #[test]
    fn schema_version_is_explicit() {
        assert_eq!(
            event().schema_version(),
            AI_AUDIT_SCHEMA_VERSION
        );
    }

    #[test]
    fn metadata_field_detection_is_case_insensitive() {
        assert!(
            AiAuditFieldName::new(
                "AccessToken"
            )
            .is_err()
        );
    }

    #[test]
    fn metadata_field_detection_handles_separators() {
        assert!(
            AiAuditFieldName::new(
                "private-key"
            )
            .is_err()
        );

        assert!(
            AiAuditFieldName::new(
                "client.secret"
            )
            .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Extensibility tests
    // ------------------------------------------------------------------------

    #[test]
    fn supports_future_open_world_actions() {
        let record = AiAuditEvent::new(
            AiAuditEventId::new("future-event").unwrap(),
            AiAuditTimestamp::new(100, 0).unwrap(),
            AiAuditCategory::Extension,
            "ai.future.computational.domain.operation",
            principal(),
            AiAuditDecision::NotEvaluated,
            AiAuditOutcome::Unknown,
        )
        .unwrap();

        assert_eq!(
            record.action(),
            "ai.future.computational.domain.operation"
        );
    }

    #[test]
    fn provenance_references_are_deduplicated() {
        let id = ProvenanceId::new(42)
            .expect("valid provenance ID");

        let record = event()
            .with_provenance(id)
            .with_provenance(id);

        assert_eq!(
            record.provenance().len(),
            1
        );
    }

    #[test]
    fn correlation_reference_is_preserved() {
        let record = event()
            .with_correlation_id(
                "distributed-execution-42",
            )
            .unwrap();

        assert_eq!(
            record.correlation_id(),
            Some("distributed-execution-42")
        );
    }
}
