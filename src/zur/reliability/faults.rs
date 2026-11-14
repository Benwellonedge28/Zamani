
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Fault Infrastructure
//!
//! Path: src/zur/reliability/faults.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97+
//!
//! PURPOSE
//!
//! Provides structured classification and attribution of operational
//! failures observed by the Zamani Universal Runtime (ZUR).
//!
//! Supports:
//! - Classical execution.
//! - Quantum execution.
//! - HDL and hardware execution.
//! - AI and tensor execution.
//! - Hybrid execution.
//! - Distributed execution.
//! - Accelerators.
//! - Embedded execution.
//! - Future computational domains.
//!
//! OWNERSHIP
//!
//! This module owns:
//! - Domain-neutral runtime fault categories.
//! - Open-ended failure-model identifiers.
//! - Fault origin identification.
//! - Structured fault records.
//! - Execution-outcome certainty.
//! - Fault severity.
//! - Fault evidence references.
//! - Fault attribution.
//! - Fault-to-ZurError adaptation.
//!
//! It does NOT own:
//! - Source-language failure semantics.
//! - Grammar interpretation.
//! - Semantic validation.
//! - Capability definitions.
//! - Resource feasibility.
//! - Physical resource reservations.
//! - Hardware diagnostics.
//! - Quantum noise or fault models.
//! - Quantum error correction.
//! - Distributed fault detection.
//! - Distributed retry decisions.
//! - Recovery execution.
//! - Checkpoint storage.
//! - Security authorization.
//! - Execution lifecycle.
//!
//! Those responsibilities remain with their authoritative subsystems.
//!
//! INTEGRATION
//!
//! - crate::zur::error
//! - crate::zur::ids
//! - crate::zur::execution::result
//! - crate::zur::execution::cancellation
//! - crate::zur::reliability::checkpoint
//! - crate::zur::reliability::recovery
//! - crate::zur::reliability::cleanup
//! - crate::distributed::error
//! - crate::distributed::resilience
//! - crate::hybrid
//! - crate::quantum
//!
//! SCALABILITY
//!
//! No arbitrary limits are imposed on:
//! - Failure-model identifiers.
//! - Fault records.
//! - Evidence references.
//! - Providers.
//! - Execution sessions.
//! - Computational domains.
//! - Hardware resources.
//! - Distributed participants.
//!
//! Collections grow subject to actual memory and allocator availability.
//!
//! This does not promise literal infinite computation.
//!
//! SAFETY
//!
//! Safe Rust only.
//! No unsafe.
//! No raw pointers.
//! No global mutable state.
//! No implicit retry.
//! No fabricated recovery.
//! No fabricated execution results.
//! No implicit resource allocation.

use core::fmt;
use core::str::FromStr;

use std::error::Error as StdError;
use std::sync::Arc;

use crate::zur::error::{
    ErrorCode,
    ErrorKind,
    RecoveryDisposition,
    Retryability,
    Severity,
    ZurError,
    ZurResult,
};

use crate::zur::ids::{
    ExecutionId,
    RuntimeOperationId,
    ServiceInstanceId,
    SessionId,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.reliability.faults";

// ============================================================================
// VALIDATION ERRORS
// ============================================================================

/// Failure-model identifier validation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FaultIdentifierError {
    /// Identifier is empty.
    Empty,

    /// Identifier contains whitespace.
    Whitespace,

    /// Identifier contains unsupported characters.
    InvalidCharacter,

    /// Identifier contains an invalid namespace boundary.
    InvalidBoundary,
}

impl fmt::Display for FaultIdentifierError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Empty => "fault identifier must not be empty",
            Self::Whitespace => "fault identifier must not contain whitespace",
            Self::InvalidCharacter => "fault identifier contains invalid characters",
            Self::InvalidBoundary => "fault identifier contains an invalid namespace boundary",
        })
    }
}

impl StdError for FaultIdentifierError {}

// ============================================================================
// OPEN-WORLD IDENTIFIERS
// ============================================================================

/// An extensible failure-model identifier.
///
/// Examples:
///
/// - transient
/// - permanent
/// - process_failure
/// - node_failure
/// - service_failure
/// - communication_failure
/// - storage_failure
/// - resource_exhaustion
/// - corruption
/// - timeout
/// - cancellation
/// - vendor.custom_failure
/// - photonics.device_failure
/// - future.domain.failure
///
/// This is intentionally not a closed enumeration.
///
/// Unknown identifiers remain representable.
///
/// Representability does not imply that a failure model is understood,
/// supported, verified or executable.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FailureModel(String);

impl FailureModel {
    /// Creates a validated failure-model identifier.
    ///
    /// Namespace components are extensible.
    ///
    /// No arbitrary identifier-length ceiling is imposed.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, FaultIdentifierError> {
        let value = value.into();

        validate_fault_identifier(&value)?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for FailureModel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for FailureModel {
    type Err = FaultIdentifierError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// An extensible identifier describing where a fault originated.
///
/// Examples:
///
/// - zamani.execution
/// - zamani.provider
/// - zamani.distributed
/// - vendor.accelerator
/// - photonics.controller
///
/// This identifier describes attribution, not physical location.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FaultOrigin(String);

impl FaultOrigin {
    /// Creates a validated fault-origin identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, FaultIdentifierError> {
        let value = value.into();

        validate_fault_identifier(&value)?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FaultOrigin {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Validates an extensible fault identifier.
///
/// Accepted characters:
/// - ASCII letters.
/// - ASCII digits.
/// - Underscores.
/// - Hyphens.
/// - Dots.
/// - Namespace separators `::`.
///
/// Whitespace, control characters and empty namespace segments
/// are rejected.
fn validate_fault_identifier(
    value: &str,
) -> Result<(), FaultIdentifierError> {
    if value.is_empty() {
        return Err(FaultIdentifierError::Empty);
    }

    if value.chars().any(char::is_whitespace) {
        return Err(FaultIdentifierError::Whitespace);
    }

    let mut segment_has_character = false;
    let mut previous_separator = false;

    let bytes = value.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        let byte = bytes[index];

        if byte == b'.' {
            if !segment_has_character {
                return Err(FaultIdentifierError::InvalidBoundary);
            }

            segment_has_character = false;
            previous_separator = true;
            index += 1;
            continue;
        }

        if byte == b':' {
            if index + 1 >= bytes.len() || bytes[index + 1] != b':' {
                return Err(FaultIdentifierError::InvalidBoundary);
            }

            if !segment_has_character {
                return Err(FaultIdentifierError::InvalidBoundary);
            }

            segment_has_character = false;
            previous_separator = true;
            index += 2;
            continue;
        }

        let valid = byte.is_ascii_alphanumeric()
            || byte == b'_'
            || byte == b'-';

        if !valid {
            return Err(FaultIdentifierError::InvalidCharacter);
        }

        if previous_separator && (byte == b'_' || byte == b'-') {
            return Err(FaultIdentifierError::InvalidBoundary);
        }

        segment_has_character = true;
        previous_separator = false;
        index += 1;
    }

    if !segment_has_character
        || value.ends_with('_')
        || value.ends_with('-')
    {
        return Err(FaultIdentifierError::InvalidBoundary);
    }

    Ok(())
}

// ============================================================================
// FAULT CATEGORY
// ============================================================================

/// Broad operational classification.
///
/// These categories describe ZUR operational failures.
///
/// They do not enumerate computational domains.
///
/// New domains must reuse these categories or provide an open-world
/// FailureModel identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FaultCategory {
    /// General execution failure.
    Execution,

    /// Execution provider failure.
    Provider,

    /// Runtime service failure.
    Service,

    /// Resource acquisition or availability failure.
    Resource,

    /// Memory operation failure.
    Memory,

    /// Communication or transport failure.
    Communication,

    /// Data or artifact integrity failure.
    Integrity,

    /// Security enforcement failure.
    Security,

    /// Compatibility failure.
    Compatibility,

    /// Runtime lifecycle failure.
    Lifecycle,

    /// Cancellation-related failure.
    Cancellation,

    /// Execution deadline or timeout failure.
    Deadline,

    /// Checkpoint operation failure.
    Checkpoint,

    /// Recovery operation failure.
    Recovery,

    /// Unknown or unclassified operational failure.
    Unknown,
}

impl FaultCategory {
    /// Returns the corresponding canonical ZUR error classification.
    #[must_use]
    pub const fn error_kind(self) -> ErrorKind {
        match self {
            Self::Execution => ErrorKind::Execution,
            Self::Provider => ErrorKind::Provider,
            Self::Service => ErrorKind::ServiceUnavailable,
            Self::Resource => ErrorKind::ResourceUnavailable,
            Self::Memory => ErrorKind::Memory,
            Self::Communication => ErrorKind::Communication,
            Self::Integrity => ErrorKind::Integrity,
            Self::Security => ErrorKind::Security,
            Self::Compatibility => ErrorKind::Compatibility,
            Self::Lifecycle => ErrorKind::Lifecycle,
            Self::Cancellation => ErrorKind::Cancelled,
            Self::Deadline => ErrorKind::TimedOut,
            Self::Checkpoint => ErrorKind::Checkpoint,
            Self::Recovery => ErrorKind::Recovery,
            Self::Unknown => ErrorKind::Other,
        }
    }

    /// Returns the stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Execution => "execution",
            Self::Provider => "provider",
            Self::Service => "service",
            Self::Resource => "resource",
            Self::Memory => "memory",
            Self::Communication => "communication",
            Self::Integrity => "integrity",
            Self::Security => "security",
            Self::Compatibility => "compatibility",
            Self::Lifecycle => "lifecycle",
            Self::Cancellation => "cancellation",
            Self::Deadline => "deadline",
            Self::Checkpoint => "checkpoint",
            Self::Recovery => "recovery",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for FaultCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// EXECUTION OUTCOME CERTAINTY
// ============================================================================

/// Describes what is known about the affected execution.
///
/// These are operational observations, not execution lifecycle states.
///
/// Unknown outcomes must be reconciled by the authoritative execution
/// subsystem before any potentially duplicate submission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ExecutionEffect {
    /// Execution is established not to have started.
    NotStarted,

    /// Execution may have started, but this is not established.
    MayHaveStarted,

    /// Execution is known to have started.
    Started,

    /// Some effects may already have been applied.
    PartiallyApplied,

    /// Execution completion is established.
    Completed,

    /// Available evidence cannot establish the execution outcome.
    Unknown,

    /// The fault is not associated with an execution.
    NotApplicable,
}

impl ExecutionEffect {
    /// Returns whether authoritative reconciliation is necessary
    /// before considering another execution attempt.
    #[must_use]
    pub const fn requires_reconciliation(self) -> bool {
        matches!(
            self,
            Self::MayHaveStarted
                | Self::Started
                | Self::PartiallyApplied
                | Self::Unknown
        )
    }

    /// Returns whether execution is established not to have started.
    #[must_use]
    pub const fn is_known_not_started(self) -> bool {
        matches!(self, Self::NotStarted)
    }

    /// Returns whether completion is established.
    #[must_use]
    pub const fn is_completed(self) -> bool {
        matches!(self, Self::Completed)
    }
}

// ============================================================================
// FAULT EVIDENCE
// ============================================================================

/// Reference to evidence maintained by an authoritative subsystem.
///
/// The referenced record is not copied into the fault record.
///
/// This type does not authenticate evidence.
///
/// Evidence verification remains the responsibility of its owner.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FaultEvidence {
    authority: FaultOrigin,
    reference: String,
}

impl FaultEvidence {
    /// Creates an evidence reference.
    pub fn new(
        authority: FaultOrigin,
        reference: impl Into<String>,
    ) -> ZurResult<Self> {
        let reference = reference.into();

        if reference.trim().is_empty() {
            return Err(
                ZurError::invalid_input(
                    "fault evidence reference must not be empty",
                )
                .with_component(COMPONENT),
            );
        }

        Ok(Self {
            authority,
            reference,
        })
    }

    /// Returns the evidence authority.
    #[must_use]
    pub fn authority(&self) -> &FaultOrigin {
        &self.authority
    }

    /// Returns the external evidence reference.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }
}

// ============================================================================
// FAULT RECORD
// ============================================================================

/// Structured observation of an operational failure.
///
/// A FaultRecord describes a failure that has been observed or reported
/// by an operational authority.
///
/// It does not independently prove the correctness of that observation.
///
/// A record may be associated with:
/// - One execution.
/// - One execution session.
/// - One runtime operation.
/// - One service instance.
/// - One external provider.
///
/// All associations are optional because not every fault belongs to
/// an execution or a particular service.
///
/// The record deliberately does not own recovery execution.
#[derive(Debug)]
pub struct FaultRecord {
    category: FaultCategory,
    model: FailureModel,
    origin: FaultOrigin,

    severity: Severity,
    message: String,

    effect: ExecutionEffect,

    execution_id: Option<ExecutionId>,
    session_id: Option<SessionId>,
    operation_id: Option<RuntimeOperationId>,
    service_instance_id: Option<ServiceInstanceId>,

    provider: Option<String>,
    operation: Option<String>,

    retryability: Retryability,
    recovery: RecoveryDisposition,

    evidence: Vec<FaultEvidence>,

    cause: Option<ZurError>,
}

impl FaultRecord {
    /// Creates a new structured fault observation.
    ///
    /// This constructor does not claim that a recovery operation
    /// is possible or that execution has stopped.
    pub fn new(
        category: FaultCategory,
        model: FailureModel,
        origin: FaultOrigin,
        severity: Severity,
        message: impl Into<String>,
    ) -> ZurResult<Self> {
        let message = message.into();

        if message.trim().is_empty() {
            return Err(
                ZurError::invalid_input(
                    "fault message must not be empty",
                )
                .with_component(COMPONENT),
            );
        }

        Ok(Self {
            category,
            model,
            origin,
            severity,
            message,
            effect: ExecutionEffect::Unknown,

            execution_id: None,
            session_id: None,
            operation_id: None,
            service_instance_id: None,

            provider: None,
            operation: None,

            retryability: Retryability::Unknown,
            recovery: RecoveryDisposition::Unspecified,

            evidence: Vec::new(),

            cause: None,
        })
    }

    /// Associates the fault with an execution.
    #[must_use]
    pub fn with_execution(
        mut self,
        execution_id: ExecutionId,
    ) -> Self {
        self.execution_id = Some(execution_id);
        self
    }

    /// Associates the fault with an execution session.
    #[must_use]
    pub fn with_session(
        mut self,
        session_id: SessionId,
    ) -> Self {
        self.session_id = Some(session_id);
        self
    }

    /// Associates the fault with a runtime operation.
    #[must_use]
    pub fn with_operation_id(
        mut self,
        operation_id: RuntimeOperationId,
    ) -> Self {
        self.operation_id = Some(operation_id);
        self
    }

    /// Associates the fault with a runtime service instance.
    #[must_use]
    pub fn with_service_instance(
        mut self,
        service_instance_id: ServiceInstanceId,
    ) -> Self {
        self.service_instance_id = Some(service_instance_id);
        self
    }

    /// Associates the fault with an external provider.
    ///
    /// The provider identifier is descriptive attribution.
    /// It does not establish provider registration or trust.
    pub fn with_provider(
        mut self,
        provider: impl Into<String>,
    ) -> ZurResult<Self> {
        let provider = provider.into();

        if provider.trim().is_empty() {
            return Err(
                ZurError::invalid_input(
                    "fault provider identifier must not be empty",
                )
                .with_component(COMPONENT),
            );
        }

        self.provider = Some(provider);
        Ok(self)
    }

    /// Associates the fault with an operation name.
    pub fn with_operation(
        mut self,
        operation: impl Into<String>,
    ) -> ZurResult<Self> {
        let operation = operation.into();

        if operation.trim().is_empty() {
            return Err(
                ZurError::invalid_input(
                    "fault operation identifier must not be empty",
                )
                .with_component(COMPONENT),
            );
        }

        self.operation = Some(operation);
        Ok(self)
    }

    /// Records the observed execution effect.
    ///
    /// The caller must supply evidence-backed knowledge.
    /// This method does not inspect an execution provider.
    #[must_use]
    pub fn with_execution_effect(
        mut self,
        effect: ExecutionEffect,
    ) -> Self {
        self.effect = effect;
        self
    }

    /// Records retryability information from an authoritative assessment.
    ///
    /// Retryability is advisory metadata, not permission to retry.
    #[must_use]
    pub fn with_retryability(
        mut self,
        retryability: Retryability,
    ) -> Self {
        self.retryability = retryability;
        self
    }

    /// Records the recovery disposition supplied by an authoritative
    /// policy or recovery assessment.
    ///
    /// This does not execute or confirm recovery.
    #[must_use]
    pub fn with_recovery_disposition(
        mut self,
        recovery: RecoveryDisposition,
    ) -> Self {
        self.recovery = recovery;
        self
    }

    /// Associates an underlying canonical ZUR error.
    ///
    /// The original error and its causal chain are preserved.
    #[must_use]
    pub fn with_cause(
        mut self,
        cause: ZurError,
    ) -> Self {
        self.cause = Some(cause);
        self
    }

    /// Adds an external evidence reference.
    ///
    /// No arbitrary evidence-count ceiling is imposed.
    ///
    /// Allocation failure is returned explicitly.
    pub fn try_add_evidence(
        &mut self,
        evidence: FaultEvidence,
    ) -> ZurResult<()> {
        self.evidence.try_reserve(1).map_err(|_| {
            ZurError::resource_unavailable(
                "unable to allocate storage for fault evidence",
            )
            .with_component(COMPONENT)
        })?;

        self.evidence.push(evidence);

        Ok(())
    }

    /// Returns the fault category.
    #[must_use]
    pub const fn category(&self) -> FaultCategory {
        self.category
    }

    /// Returns the open-ended failure model.
    #[must_use]
    pub fn model(&self) -> &FailureModel {
        &self.model
    }

    /// Returns the fault origin.
    #[must_use]
    pub fn origin(&self) -> &FaultOrigin {
        &self.origin
    }

    /// Returns the severity.
    #[must_use]
    pub const fn severity(&self) -> Severity {
        self.severity
    }

    /// Returns the descriptive message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the observed execution effect.
    #[must_use]
    pub const fn execution_effect(&self) -> ExecutionEffect {
        self.effect
    }

    /// Returns the associated execution identity.
    #[must_use]
    pub fn execution_id(&self) -> Option<&ExecutionId> {
        self.execution_id.as_ref()
    }

    /// Returns the associated session identity.
    #[must_use]
    pub fn session_id(&self) -> Option<&SessionId> {
        self.session_id.as_ref()
    }

    /// Returns the associated operation identity.
    #[must_use]
    pub fn operation_id(&self) -> Option<&RuntimeOperationId> {
        self.operation_id.as_ref()
    }

    /// Returns the associated service instance.
    #[must_use]
    pub fn service_instance_id(&self) -> Option<&ServiceInstanceId> {
        self.service_instance_id.as_ref()
    }

    /// Returns the attributed provider.
    #[must_use]
    pub fn provider(&self) -> Option<&str> {
        self.provider.as_deref()
    }

    /// Returns the attributed operation.
    #[must_use]
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Returns retryability information.
    #[must_use]
    pub const fn retryability(&self) -> Retryability {
        self.retryability
    }

    /// Returns the recovery disposition.
    #[must_use]
    pub const fn recovery_disposition(&self) -> RecoveryDisposition {
        self.recovery
    }

    /// Returns the external evidence references.
    #[must_use]
    pub fn evidence(&self) -> &[FaultEvidence] {
        &self.evidence
    }

    /// Returns the underlying causal ZUR error.
    #[must_use]
    pub fn cause(&self) -> Option<&ZurError> {
        self.cause.as_ref()
    }

    /// Returns whether authoritative reconciliation is necessary
    /// before another execution attempt can be considered.
    #[must_use]
    pub const fn requires_reconciliation(&self) -> bool {
        self.effect.requires_reconciliation()
    }

    /// Converts the fault into the canonical ZUR operational error.
    ///
    /// This preserves the classification, attribution, recovery metadata,
    /// retryability and original causal error.
    ///
    /// Evidence references remain external references. This conversion
    /// does not authenticate or independently verify them.
    pub fn into_error(self) -> ZurResult<ZurError> {
        let code = ErrorCode::new(format!(
            "zur.fault.{}",
            self.category.as_str(),
        ))
        .map_err(|_| {
            ZurError::internal(
                "failed to construct canonical fault error code",
            )
            .with_component(COMPONENT)
        })?;

        let mut error = ZurError::new(
            self.category.error_kind(),
            code,
            self.message,
        )
        .with_component(COMPONENT)
        .with_severity(self.severity)
        .with_retryability(self.retryability)
        .with_recovery(self.recovery);

        if let Some(execution) = self.execution_id {
            error = error.with_execution(execution.to_string());
        }

        if let Some(provider) = self.provider {
            error = error.with_provider(provider);
        }

        if let Some(operation) = self.operation {
            error = error.with_operation(operation);
        }

        error = error
            .with_context("fault.model", self.model.as_str())
            .map_err(|_| {
                ZurError::internal(
                    "failed to attach fault model context",
                )
                .with_component(COMPONENT)
            })?;

        error = error
            .with_context("fault.origin", self.origin.as_str())
            .map_err(|_| {
                ZurError::internal(
                    "failed to attach fault origin context",
                )
                .with_component(COMPONENT)
            })?;

        error = error
            .with_context(
                "fault.execution_effect",
                format!("{:?}", self.effect),
            )
            .map_err(|_| {
                ZurError::internal(
                    "failed to attach fault execution context",
                )
                .with_component(COMPONENT)
            })?;

        if let Some(session) = self.session_id {
            error = error
                .with_context("fault.session", session.to_string())
                .map_err(|_| {
                    ZurError::internal(
                        "failed to attach session context",
                    )
                    .with_component(COMPONENT)
                })?;
        }

        if let Some(operation_id) = self.operation_id {
            error = error
                .with_context(
                    "fault.operation_id",
                    operation_id.to_string(),
                )
                .map_err(|_| {
                    ZurError::internal(
                        "failed to attach operation identity",
                    )
                    .with_component(COMPONENT)
                })?;
        }

        if let Some(service) = self.service_instance_id {
            error = error
                .with_context(
                    "fault.service_instance",
                    service.to_string(),
                )
                .map_err(|_| {
                    ZurError::internal(
                        "failed to attach service identity",
                    )
                    .with_component(COMPONENT)
                })?;
        }

        for (index, evidence) in self.evidence.iter().enumerate() {
            let key = format!("fault.evidence.{index}");

            let value = format!(
                "{}:{}",
                evidence.authority(),
                evidence.reference(),
            );

            error = error
                .with_context(key, value)
                .map_err(|_| {
                    ZurError::internal(
                        "failed to attach fault evidence reference",
                    )
                    .with_component(COMPONENT)
                })?;
        }

        if let Some(cause) = self.cause {
            error = error.with_source(cause);
        }

        Ok(error)
    }
}

// ============================================================================
// FAULT CLASSIFICATION RESULT
// ============================================================================

/// A classification decision produced by an explicit fault classifier.
///
/// Classification describes the operational interpretation of an
/// observation. It does not prove that the underlying event occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaultClassification {
    category: FaultCategory,
    model: FailureModel,
    confidence: ClassificationConfidence,
}

/// Confidence of a fault classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ClassificationConfidence {
    /// The available information is insufficient.
    Unknown,

    /// The classification is provisional.
    Tentative,

    /// The classification is supported by supplied evidence.
    EvidenceSupported,

    /// An authoritative subsystem supplied the classification.
    Authoritative,
}

impl FaultClassification {
    /// Constructs an explicit fault classification.
    ///
    /// The caller is responsible for supplying the appropriate
    /// confidence level and evidence basis.
    #[must_use]
    pub fn new(
        category: FaultCategory,
        model: FailureModel,
        confidence: ClassificationConfidence,
    ) -> Self {
        Self {
            category,
            model,
            confidence,
        }
    }

    /// Returns the category.
    #[must_use]
    pub const fn category(&self) -> FaultCategory {
        self.category
    }

    /// Returns the failure model.
    #[must_use]
    pub fn model(&self) -> &FailureModel {
        &self.model
    }

    /// Returns the classification confidence.
    #[must_use]
    pub const fn confidence(&self) -> ClassificationConfidence {
        self.confidence
    }
}

// ============================================================================
// CLASSIFICATION INPUT
// ============================================================================

/// Explicit information supplied to a fault classifier.
///
/// The classifier does not inspect providers, hardware, networks,
/// clocks, execution state or global registries.
#[derive(Debug, Clone)]
pub struct FaultClassificationInput {
    category: FaultCategory,
    model: FailureModel,
    confidence: ClassificationConfidence,
}

impl FaultClassificationInput {
    /// Creates explicit classification input.
    #[must_use]
    pub fn new(
        category: FaultCategory,
        model: FailureModel,
        confidence: ClassificationConfidence,
    ) -> Self {
        Self {
            category,
            model,
            confidence,
        }
    }
}

/// Stateless fault classifier.
///
/// This type performs no automatic detection or recovery.
#[derive(Debug, Default, Clone, Copy)]
pub struct FaultClassifier;

impl FaultClassifier {
    /// Creates a fault classifier.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Classifies supplied fault information.
    ///
    /// This operation is deterministic and performs no external I/O.
    #[must_use]
    pub fn classify(
        &self,
        input: FaultClassificationInput,
    ) -> FaultClassification {
        FaultClassification::new(
            input.category,
            input.model,
            input.confidence,
        )
    }
}

// ============================================================================
// FAULT COLLECTION
// ============================================================================

/// A collection of fault observations.
///
/// This collection does not imply that the faults are independent,
/// simultaneous, causally related or successfully recovered.
///
/// It introduces no fixed maximum record count.
#[derive(Debug, Default)]
pub struct FaultCollection {
    records: Vec<FaultRecord>,
}

impl FaultCollection {
    /// Creates an empty fault collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    /// Adds a fault record with explicit allocation-error handling.
    pub fn try_push(
        &mut self,
        record: FaultRecord,
    ) -> ZurResult<()> {
        self.records.try_reserve(1).map_err(|_| {
            ZurError::resource_unavailable(
                "unable to allocate storage for fault records",
            )
            .with_component(COMPONENT)
        })?;

        self.records.push(record);

        Ok(())
    }

    /// Returns the number of recorded faults.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Returns an iterator over fault records.
    pub fn iter(&self) -> impl Iterator<Item = &FaultRecord> {
        self.records.iter()
    }

    /// Returns the fault records as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[FaultRecord] {
        &self.records
    }

    /// Consumes the collection and returns its records.
    #[must_use]
    pub fn into_records(self) -> Vec<FaultRecord> {
        self.records
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn model(value: &str) -> FailureModel {
        FailureModel::new(value).expect("valid failure model")
    }

    fn origin(value: &str) -> FaultOrigin {
        FaultOrigin::new(value).expect("valid fault origin")
    }

    fn fault() -> FaultRecord {
        FaultRecord::new(
            FaultCategory::Provider,
            model("provider_failure"),
            origin("zamani.provider"),
            Severity::Error,
            "Provider became unavailable",
        )
        .expect("valid fault")
    }

    // ------------------------------------------------------------------------
    // IDENTIFIER VALIDATION
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_standard_failure_models() {
        assert!(FailureModel::new("transient").is_ok());
        assert!(FailureModel::new("permanent").is_ok());
        assert!(FailureModel::new("node_failure").is_ok());
        assert!(FailureModel::new("resource_exhaustion").is_ok());
    }

    #[test]
    fn accepts_future_domain_failure_models() {
        assert!(
            FailureModel::new("photonics.device_failure").is_ok()
        );

        assert!(
            FailureModel::new("future::domain::failure").is_ok()
        );

        assert!(
            FailureModel::new("vendor.custom_failure").is_ok()
        );
    }

    #[test]
    fn rejects_empty_failure_model() {
        assert_eq!(
            FailureModel::new(""),
            Err(FaultIdentifierError::Empty),
        );
    }

    #[test]
    fn rejects_whitespace_in_identifiers() {
        assert_eq!(
            FailureModel::new("node failure"),
            Err(FaultIdentifierError::Whitespace),
        );
    }

    #[test]
    fn rejects_invalid_namespace_boundaries() {
        assert!(
            FailureModel::new("provider..failure").is_err()
        );

        assert!(
            FailureModel::new("provider::").is_err()
        );

        assert!(
            FailureModel::new("provider_").is_err()
        );
    }

    #[test]
    fn accepts_arbitrarily_extended_identifier_namespaces() {
        let value = format!(
            "future.{}.failure",
            "domain".repeat(128),
        );

        assert!(FailureModel::new(value).is_ok());
    }

    // ------------------------------------------------------------------------
    // CLASSIFICATION
    // ------------------------------------------------------------------------

    #[test]
    fn classifier_preserves_open_world_failure_models() {
        let input = FaultClassificationInput::new(
            FaultCategory::Provider,
            model("future.provider_failure"),
            ClassificationConfidence::EvidenceSupported,
        );

        let result = FaultClassifier::new().classify(input);

        assert_eq!(
            result.category(),
            FaultCategory::Provider,
        );

        assert_eq!(
            result.model().as_str(),
            "future.provider_failure",
        );

        assert_eq!(
            result.confidence(),
            ClassificationConfidence::EvidenceSupported,
        );
    }

    #[test]
    fn categories_map_to_canonical_error_kinds() {
        assert_eq!(
            FaultCategory::Security.error_kind(),
            ErrorKind::Security,
        );

        assert_eq!(
            FaultCategory::Memory.error_kind(),
            ErrorKind::Memory,
        );

        assert_eq!(
            FaultCategory::Deadline.error_kind(),
            ErrorKind::TimedOut,
        );

        assert_eq!(
            FaultCategory::Recovery.error_kind(),
            ErrorKind::Recovery,
        );
    }

    // ------------------------------------------------------------------------
    // EXECUTION CERTAINTY
    // ------------------------------------------------------------------------

    #[test]
    fn unknown_execution_outcome_requires_reconciliation() {
        assert!(
            ExecutionEffect::Unknown.requires_reconciliation()
        );

        assert!(
            ExecutionEffect::PartiallyApplied.requires_reconciliation()
        );
    }

    #[test]
    fn known_not_started_does_not_require_outcome_reconciliation() {
        assert!(
            !ExecutionEffect::NotStarted.requires_reconciliation()
        );
    }

    #[test]
    fn completed_execution_is_not_misclassified_as_unknown() {
        assert!(
            ExecutionEffect::Completed.is_completed()
        );

        assert!(
            !ExecutionEffect::Completed.requires_reconciliation()
        );
    }

    // ------------------------------------------------------------------------
    // FAULT RECORDS
    // ------------------------------------------------------------------------

    #[test]
    fn creates_structured_fault() {
        let record = fault();

        assert_eq!(
            record.category(),
            FaultCategory::Provider,
        );

        assert_eq!(
            record.model().as_str(),
            "provider_failure",
        );

        assert_eq!(
            record.origin().as_str(),
            "zamani.provider",
        );
    }

    #[test]
    fn rejects_empty_fault_message() {
        assert!(
            FaultRecord::new(
                FaultCategory::Execution,
                model("execution_failure"),
                origin("zamani.execution"),
                Severity::Error,
                " ",
            ).is_err()
        );
    }

    #[test]
    fn provider_attribution_is_preserved() {
        let record = fault()
            .with_provider("vendor.provider")
            .expect("valid provider");

        assert_eq!(
            record.provider(),
            Some("vendor.provider"),
        );
    }

    #[test]
    fn operation_attribution_is_preserved() {
        let record = fault()
            .with_operation("submit")
            .expect("valid operation");

        assert_eq!(
            record.operation(),
            Some("submit"),
        );
    }

    #[test]
    fn unknown_execution_effect_is_conservative() {
        let record = fault();

        assert_eq!(
            record.execution_effect(),
            ExecutionEffect::Unknown,
        );

        assert!(record.requires_reconciliation());
    }

    #[test]
    fn explicit_not_started_observation_is_preserved() {
        let record = fault()
            .with_execution_effect(ExecutionEffect::NotStarted);

        assert!(
            !record.requires_reconciliation()
        );
    }

    // ------------------------------------------------------------------------
    // RETRY AND RECOVERY SEPARATION
    // ------------------------------------------------------------------------

    #[test]
    fn fault_classification_does_not_authorize_retry() {
        let record = fault();

        assert_eq!(
            record.retryability(),
            Retryability::Unknown,
        );
    }

    #[test]
    fn recovery_is_not_claimed_without_evidence() {
        let record = fault();

        assert_eq!(
            record.recovery_disposition(),
            RecoveryDisposition::Unspecified,
        );
    }

    #[test]
    fn explicit_retryability_is_preserved() {
        let record = fault()
            .with_retryability(
                Retryability::RetryableAfterConditionChange,
            );

        assert_eq!(
            record.retryability(),
            Retryability::RetryableAfterConditionChange,
        );
    }

    // ------------------------------------------------------------------------
    // EVIDENCE
    // ------------------------------------------------------------------------

    #[test]
    fn evidence_reference_preserves_authority() {
        let evidence = FaultEvidence::new(
            origin("zamani.distributed"),
            "execution-observation-001",
        )
        .expect("valid evidence");

        assert_eq!(
            evidence.authority().as_str(),
            "zamani.distributed",
        );

        assert_eq!(
            evidence.reference(),
            "execution-observation-001",
        );
    }

    #[test]
    fn rejects_empty_evidence_reference() {
        assert!(
            FaultEvidence::new(
                origin("zamani.execution"),
                "",
            ).is_err()
        );
    }

    #[test]
    fn accepts_multiple_evidence_records() {
        let mut record = fault();

        for index in 0..8 {
            record
                .try_add_evidence(
                    FaultEvidence::new(
                        origin("zamani.execution"),
                        format!("observation-{index}"),
                    )
                    .expect("valid evidence"),
                )
                .expect("evidence allocation");
        }

        assert_eq!(record.evidence().len(), 8);
    }

    // ------------------------------------------------------------------------
    // COLLECTION
    // ------------------------------------------------------------------------

    #[test]
    fn fault_collection_starts_empty() {
        let collection = FaultCollection::new();

        assert!(collection.is_empty());
        assert_eq!(collection.len(), 0);
    }

    #[test]
    fn fault_collection_accepts_records() {
        let mut collection = FaultCollection::new();

        collection
            .try_push(fault())
            .expect("fault should be recorded");

        assert_eq!(collection.len(), 1);
        assert!(!collection.is_empty());
    }

    #[test]
    fn collection_preserves_record_order() {
        let mut collection = FaultCollection::new();

        collection
            .try_push(
                FaultRecord::new(
                    FaultCategory::Memory,
                    model("memory_failure"),
                    origin("zamani.memory"),
                    Severity::Error,
                    "Memory failure",
                )
                .expect("valid fault"),
            )
            .expect("insert");

        collection
            .try_push(
                FaultRecord::new(
                    FaultCategory::Provider,
                    model("provider_failure"),
                    origin("zamani.provider"),
                    Severity::Critical,
                    "Provider failure",
                )
                .expect("valid fault"),
            )
            .expect("insert");

        assert_eq!(
            collection.as_slice()[0].category(),
            FaultCategory::Memory,
        );

        assert_eq!(
            collection.as_slice()[1].category(),
            FaultCategory::Provider,
        );
    }
}
