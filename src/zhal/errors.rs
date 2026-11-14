
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer — Errors
//!
//! Path: `src/zhal/errors.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the canonical error contract for ZHAL-owned hardware
//! abstraction operations.
//!
//! It supports hardware integration across:
//!
//! - Classical processors and memory.
//! - Quantum processors and quantum-control hardware.
//! - HDL realization, FPGA and ASIC hardware.
//! - GPUs, TPUs, NPUs, DSPs and other accelerators.
//! - Embedded systems, sensors and actuators.
//! - Hybrid computational systems.
//! - Distributed and remote hardware.
//! - Future computational technologies.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - ZHAL error classification.
//! - ZHAL error results.
//! - Extensible ZHAL error identifiers through the standard library.
//! - Hardware-operation outcome classification.
//! - Retryability metadata.
//! - Structured, redaction-aware error context.
//! - Provider, device, session and operation attribution.
//! - Causal error preservation.
//! - Safe diagnostic formatting.
//! - A serializable diagnostic report.
//! - Standard-library cancellation integration.
//!
//! This module does not own:
//!
//! - Compiler diagnostics or source spans.
//! - Semantic errors.
//! - Standard-library error classification.
//! - Provider identity.
//! - Device identity.
//! - Capability identity.
//! - Provider lifecycle.
//! - Hardware discovery.
//! - Resource allocation or scheduling.
//! - Quantum error correction.
//! - Quantum noise or fault models.
//! - HDL synthesis or realization semantics.
//! - Distributed execution policy.
//! - Runtime error classification.
//! - Authentication or authorization.
//! - Automatic retry or recovery.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ## Canonical repository authorities
//!
//! Standard-library error codes:
//!     crate::stdlib::error::ErrorCode
//!
//! Standard-library operation context:
//!     crate::stdlib::context::OperationContext
//!
//! Standard-library provider identity:
//!     crate::stdlib::provider::ProviderId
//!
//! Hardware identities:
//!     crate::zhal::identity
//!
//! Standard-library capability contracts:
//!     crate::stdlib::capability
//!
//! Generic device I/O:
//!     crate::stdlib::io::devices
//!
//! Runtime errors:
//!     crate::zur::error
//!
//! Quantum hardware errors:
//!     crate::quantum::hardware
//!
//! HDL realization errors:
//!     crate::hdl::realization
//!
//! ## Error model
//!
//! An error has distinct:
//!
//! - Kind: broad classification.
//! - Code: optional specific, extensible identifier.
//! - Message: safe human-readable summary.
//! - Attribution: provider/device/session/operation references.
//! - Retryability: advisory classification.
//! - Outcome: what is known about the operation.
//! - Context: structured descriptive information.
//! - Cause: original underlying error.
//!
//! These fields must not be conflated.
//!
//! ## Operation outcome
//!
//! An operation failure and an unknown operation outcome are different.
//!
//! For example, a network connection can fail after a remote provider
//! accepted an operation but before the caller received its result.
//! In that situation, ZHAL must not report that the operation failed
//! to execute. Its outcome is unknown until reconciled.
//!
//! Partial completion must remain explicit.
//!
//! Cancellation is a request, not proof that physical execution stopped.
//!
//! ## Retryability
//!
//! Retryability is descriptive metadata.
//!
//! This module never retries an operation automatically.
//!
//! Retrying requires consideration of:
//!
//! - Operation idempotency.
//! - Provider guarantees.
//! - Transaction state.
//! - Physical side effects.
//! - Remote submission state.
//! - Cancellation state.
//! - Resource reservations.
//! - The caller's retry policy.
//!
//! An unknown outcome must not be blindly retried unless the operation
//! contract provides an appropriate idempotency or reconciliation method.
//!
//! ## Security
//!
//! Error messages should contain safe summaries, not credentials or secrets.
//!
//! Sensitive context is redacted from Debug output and diagnostic reports.
//!
//! Arbitrary causal errors are not formatted by this module's Debug or
//! Display implementations.
//!
//! Error identifiers and device identifiers do not establish authorization.
//!
//! ## Scalability
//!
//! This module imposes no arbitrary maximum on:
//!
//! - Error count.
//! - Error-context count.
//! - Message length.
//! - Identifier length.
//! - Provider count.
//! - Device count.
//! - Operation count.
//! - Hardware resource count.
//! - Computational domains.
//! - Hardware categories.
//!
//! Allocations are constrained by available memory and platform
//! representation limits.
//!
//! Operational budgets belong to explicit caller/provider/runtime policy.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Grammar boundary
//!
//! Source syntax and semantic meaning remain defined by the grammar,
//! frontend and semantic authorities.
//!
//! ZHAL errors are runtime/provider integration results.
//!
//! They must not be inserted into the source grammar as hardware-specific
//! source-language constructs.
//!
//! ## Safety
//!
//! No unsafe Rust.
//! No global mutable error registry.
//! No implicit logging.
//! No printing to stdout or stderr.
//! No hidden retry.
//! No fabricated recovery.
//! No suppressed causal errors.

use core::fmt;
use std::error::Error as StdError;
use std::io;

use serde::{Deserialize, Serialize};

use crate::stdlib::context::OperationContext;
use crate::stdlib::error::{
    Error as StdlibError,
    ErrorCode,
    ErrorKind as StdlibErrorKind,
    Retryability as StdlibRetryability,
};
use crate::stdlib::provider::ProviderId;
use crate::zhal::identity::{
    HardwareDeviceIdentity,
    HardwareOperationIdentity,
    HardwareSessionIdentity,
    IdentityError,
};

// ============================================================================
// MODULE CONTRACT
// ============================================================================

/// Stable component identifier for ZHAL error reporting.
pub const COMPONENT: &str = "zhal.errors";

/// Version of the serializable ZHAL error-report schema.
///
/// This is not the Cargo package version, language version,
/// provider API version or hardware firmware version.
pub const ERROR_REPORT_SCHEMA_VERSION: &str = "1";

// ============================================================================
// RESULT
// ============================================================================

/// Result type for ZHAL-owned fallible operations.
///
/// Domain-specific provider APIs may retain their own error types.
/// Their ZHAL adapters should preserve the original error as a cause
/// when converting it into this result type.
pub type ZhalResult<T> = Result<T, ZhalError>;

// ============================================================================
// ERROR CLASSIFICATION
// ============================================================================

/// Broad classification of a ZHAL hardware-abstraction failure.
///
/// This enum is intentionally independent of hardware vendors,
/// hardware categories and computational domains.
///
/// New providers and future hardware technologies should use
/// the existing classifications where appropriate and supply
/// extensible error codes for more specific conditions.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ZhalErrorKind {
    /// An operation argument or request is invalid.
    InvalidInput,

    /// Provider-supplied or hardware-observed data is invalid.
    InvalidData,

    /// ZHAL configuration is invalid.
    Configuration,

    /// A provider failed to satisfy its declared contract.
    ProviderFailure,

    /// A provider is currently unavailable.
    ProviderUnavailable,

    /// The requested device could not be found in the relevant scope.
    DeviceNotFound,

    /// The device exists but is currently unavailable.
    DeviceUnavailable,

    /// The device exists but access is not currently possible.
    DeviceInaccessible,

    /// Hardware discovery failed.
    DiscoveryFailed,

    /// A required capability is explicitly unsupported.
    CapabilityUnsupported,

    /// Capability support cannot be established from available evidence.
    CapabilityUnknown,

    /// The workload is incompatible with the available hardware contract.
    IncompatibleWorkload,

    /// A required physical resource is unavailable.
    ResourceUnavailable,

    /// An operational resource budget or available resource quantity
    /// was exhausted.
    ResourceExhausted,

    /// A hardware reservation could not be established.
    ReservationFailed,

    /// Hardware communication failed.
    HardwareCommunication,

    /// A hardware fault was reported or observed.
    HardwareFault,

    /// A hardware operation failed.
    OperationFailed,

    /// An operation exceeded its applicable deadline or time allowance.
    OperationTimedOut,

    /// An operation received a cooperative cancellation request.
    Cancelled,

    /// An operation completed only partially.
    PartialCompletion,

    /// The final outcome of an operation cannot currently be established.
    OutcomeUnknown,

    /// Access was denied by an authorization boundary.
    PermissionDenied,

    /// Authentication or credential verification failed.
    AuthenticationFailed,

    /// Provider or hardware trust verification failed.
    TrustFailure,

    /// A provider violated a declared interface or operation contract.
    ContractViolation,

    /// An API, protocol, schema or version is incompatible.
    VersionMismatch,

    /// Serialization or deserialization failed.
    Serialization,

    /// A concurrent state change prevented the requested operation.
    ConcurrencyConflict,

    /// An internal ZHAL implementation failure occurred.
    Internal,

    /// A failure does not fit another built-in classification.
    Other,
}

impl ZhalErrorKind {
    /// Returns the stable textual name of the error classification.
    ///
    /// These names are reporting identifiers, not source-language
    /// syntax or provider-specific error codes.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInput => "invalid_input",
            Self::InvalidData => "invalid_data",
            Self::Configuration => "configuration",
            Self::ProviderFailure => "provider_failure",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::DeviceNotFound => "device_not_found",
            Self::DeviceUnavailable => "device_unavailable",
            Self::DeviceInaccessible => "device_inaccessible",
            Self::DiscoveryFailed => "discovery_failed",
            Self::CapabilityUnsupported => "capability_unsupported",
            Self::CapabilityUnknown => "capability_unknown",
            Self::IncompatibleWorkload => "incompatible_workload",
            Self::ResourceUnavailable => "resource_unavailable",
            Self::ResourceExhausted => "resource_exhausted",
            Self::ReservationFailed => "reservation_failed",
            Self::HardwareCommunication => "hardware_communication",
            Self::HardwareFault => "hardware_fault",
            Self::OperationFailed => "operation_failed",
            Self::OperationTimedOut => "operation_timed_out",
            Self::Cancelled => "cancelled",
            Self::PartialCompletion => "partial_completion",
            Self::OutcomeUnknown => "outcome_unknown",
            Self::PermissionDenied => "permission_denied",
            Self::AuthenticationFailed => "authentication_failed",
            Self::TrustFailure => "trust_failure",
            Self::ContractViolation => "contract_violation",
            Self::VersionMismatch => "version_mismatch",
            Self::Serialization => "serialization",
            Self::ConcurrencyConflict => "concurrency_conflict",
            Self::Internal => "internal",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for ZhalErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// SEVERITY
// ============================================================================

/// Reporting severity of a ZHAL error.
///
/// Severity is descriptive. It does not itself trigger recovery,
/// shutdown, provider removal or process termination.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ZhalSeverity {
    /// Informational condition requiring no immediate intervention.
    Info,

    /// A condition that may require attention.
    Warning,

    /// An operation failed.
    Error,

    /// A serious hardware/provider condition requiring attention.
    Critical,
}

impl Default for ZhalSeverity {
    fn default() -> Self {
        Self::Error
    }
}

// ============================================================================
// RETRYABILITY
// ============================================================================

/// Describes whether retrying an operation may be appropriate.
///
/// This is advisory metadata only. ZHAL does not perform retries.
///
/// A caller must evaluate the operation's idempotency, side effects,
/// provider contract, resource state and current operation outcome.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ZhalRetryability {
    /// Available information is insufficient to determine retryability.
    Unknown,

    /// Retrying is not appropriate under the current operation contract.
    NotRetryable,

    /// Retrying may succeed after a relevant condition changes.
    AfterConditionChange,

    /// Retrying may succeed, but safety must be established by the caller.
    Retryable,

    /// The operation contract explicitly establishes safe retry behavior.
    ///
    /// This must only be selected when the relevant provider/operation
    /// contract establishes that retrying is safe.
    SafeToRetry,
}

impl Default for ZhalRetryability {
    fn default() -> Self {
        Self::Unknown
    }
}

impl ZhalRetryability {
    /// Returns the stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::NotRetryable => "not_retryable",
            Self::AfterConditionChange => "after_condition_change",
            Self::Retryable => "retryable",
            Self::SafeToRetry => "safe_to_retry",
        }
    }

    /// Returns whether this classification explicitly establishes
    /// that a retry is safe under the operation contract.
    ///
    /// `Retryable` alone does not establish idempotency.
    #[must_use]
    pub const fn is_safe_to_retry(self) -> bool {
        matches!(self, Self::SafeToRetry)
    }
}

impl From<StdlibRetryability> for ZhalRetryability {
    fn from(value: StdlibRetryability) -> Self {
        match value {
            StdlibRetryability::Unknown => Self::Unknown,
            StdlibRetryability::NotRetryable => Self::NotRetryable,
            StdlibRetryability::RetryableAfterConditionChange => {
                Self::AfterConditionChange
            }
            StdlibRetryability::Retryable => Self::Retryable,
            StdlibRetryability::SafeToRetry => Self::SafeToRetry,
        }
    }
}

// ============================================================================
// OPERATION OUTCOME
// ============================================================================

/// What is known about the physical operation when an error is reported.
///
/// This distinction is essential for remote and physical hardware
/// operations, where a communication failure can occur after the
/// provider has accepted or performed the requested operation.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ZhalOperationOutcome {
    /// The error is not associated with a submitted operation.
    NotApplicable,

    /// The operation is known not to have started.
    NotStarted,

    /// The operation is known to have failed.
    Failed,

    /// Some, but not all, requested work completed.
    PartiallyCompleted,

    /// The operation may have started or completed, but its final
    /// outcome cannot currently be established.
    Unknown,
}

impl Default for ZhalOperationOutcome {
    fn default() -> Self {
        Self::NotApplicable
    }
}

// ============================================================================
// CONTEXT VISIBILITY
// ============================================================================

/// Visibility classification for an error-context value.
///
/// Sensitive values are retained in memory for explicitly authorized
/// internal consumers but are redacted from Debug output and diagnostic
/// reports produced by this module.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ZhalContextVisibility {
    /// The value is approved for ordinary diagnostic reporting.
    Public,

    /// The value must be redacted from ordinary diagnostics.
    Sensitive,
}

// ============================================================================
// CONTEXT VALIDATION
// ============================================================================

/// Validation failures for structured ZHAL error context.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ZhalContextValidationError {
    /// A context key must not be empty.
    EmptyKey,
}

impl fmt::Display for ZhalContextValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKey => {
                formatter.write_str("ZHAL error context key must not be empty")
            }
        }
    }
}

impl StdError for ZhalContextValidationError {}

// ============================================================================
// STRUCTURED ERROR CONTEXT
// ============================================================================

/// One structured context entry attached to a ZHAL error.
///
/// Context is descriptive data. It must not be interpreted as executable
/// instructions, authorization evidence or proof of hardware availability.
///
/// Do not place credentials, private keys, access tokens or other secrets
/// in public context values.
#[derive(Clone, Eq, PartialEq)]
pub struct ZhalErrorContext {
    key: String,
    value: String,
    visibility: ZhalContextVisibility,
}

impl ZhalErrorContext {
    /// Creates a public diagnostic context entry.
    ///
    /// # Errors
    ///
    /// Returns `EmptyKey` if the supplied key is empty.
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ZhalContextValidationError> {
        Self::with_visibility(
            key,
            value,
            ZhalContextVisibility::Public,
        )
    }

    /// Creates a sensitive diagnostic context entry.
    ///
    /// Its value is redacted from Debug output and diagnostic reports.
    ///
    /// # Errors
    ///
    /// Returns `EmptyKey` if the supplied key is empty.
    pub fn sensitive(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ZhalContextValidationError> {
        Self::with_visibility(
            key,
            value,
            ZhalContextVisibility::Sensitive,
        )
    }

    /// Creates a context entry with an explicit visibility classification.
    ///
    /// # Errors
    ///
    /// Returns `EmptyKey` if the supplied key is empty.
    pub fn with_visibility(
        key: impl Into<String>,
        value: impl Into<String>,
        visibility: ZhalContextVisibility,
    ) -> Result<Self, ZhalContextValidationError> {
        let key = key.into();

        if key.is_empty() {
            return Err(ZhalContextValidationError::EmptyKey);
        }

        Ok(Self {
            key,
            value: value.into(),
            visibility,
        })
    }

    /// Returns the context key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the context value.
    ///
    /// Callers must respect the visibility classification.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the context visibility.
    #[must_use]
    pub const fn visibility(&self) -> ZhalContextVisibility {
        self.visibility
    }

    /// Returns whether the value is classified as sensitive.
    #[must_use]
    pub const fn is_sensitive(&self) -> bool {
        matches!(self.visibility, ZhalContextVisibility::Sensitive)
    }

    fn report_value(&self) -> &str {
        if self.is_sensitive() {
            "[REDACTED]"
        } else {
            self.value.as_str()
        }
    }
}

impl fmt::Debug for ZhalErrorContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ZhalErrorContext")
            .field("key", &self.key)
            .field("value", &self.report_value())
            .field("visibility", &self.visibility)
            .finish()
    }
}

// ============================================================================
// ERROR
// ============================================================================

/// Canonical ZHAL hardware-abstraction error.
///
/// This type intentionally does not implement `Clone`: an underlying
/// causal error may not be cloneable, and cloning it could lose causal
/// identity or provider-specific information.
///
/// The human-readable message must be a safe summary. Sensitive diagnostic
/// values belong in explicitly classified context entries.
///
/// The original underlying error is retained through
/// [`std::error::Error::source`].
#[must_use]
pub struct ZhalError {
    kind: ZhalErrorKind,
    code: Option<ErrorCode>,
    message: String,

    component: Option<String>,

    provider_id: Option<ProviderId>,
    device: Option<HardwareDeviceIdentity>,
    session: Option<HardwareSessionIdentity>,
    operation: Option<HardwareOperationIdentity>,

    severity: ZhalSeverity,
    retryability: ZhalRetryability,
    outcome: ZhalOperationOutcome,

    context: Vec<ZhalErrorContext>,

    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl ZhalError {
    /// Creates a ZHAL error with a broad classification and safe message.
    ///
    /// The error starts with:
    ///
    /// - no specific error code;
    /// - unknown retryability;
    /// - no operation outcome;
    /// - no provider/device attribution;
    /// - no context entries;
    /// - no causal source.
    #[must_use]
    pub fn new(
        kind: ZhalErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            code: None,
            message: message.into(),
            component: Some(COMPONENT.to_owned()),
            provider_id: None,
            device: None,
            session: None,
            operation: None,
            severity: ZhalSeverity::Error,
            retryability: ZhalRetryability::Unknown,
            outcome: ZhalOperationOutcome::NotApplicable,
            context: Vec::new(),
            source: None,
        }
    }

    /// Creates a ZHAL error with a specific extensible error code.
    ///
    /// Error-code syntax is owned by `crate::stdlib::error::ErrorCode`.
    /// No competing ZHAL error-code grammar is introduced.
    ///
    /// # Errors
    ///
    /// Returns the standard-library error-code validation error
    /// if the supplied code is invalid.
    pub fn with_code(
        kind: ZhalErrorKind,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<Self, crate::stdlib::error::ErrorCodeValidationError> {
        let code = ErrorCode::new(code)?;

        let mut error = Self::new(kind, message);
        error.code = Some(code);

        Ok(error)
    }

    /// Returns the broad error classification.
    #[must_use]
    pub const fn kind(&self) -> ZhalErrorKind {
        self.kind
    }

    /// Returns the optional specific error code.
    #[must_use]
    pub fn code(&self) -> Option<&ErrorCode> {
        self.code.as_ref()
    }

    /// Returns the optional error code as a string.
    #[must_use]
    pub fn code_str(&self) -> Option<&str> {
        self.code.as_ref().map(ErrorCode::as_str)
    }

    /// Returns the safe human-readable error summary.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the component associated with the error.
    #[must_use]
    pub fn component(&self) -> Option<&str> {
        self.component.as_deref()
    }

    /// Returns the provider identity, when known.
    #[must_use]
    pub fn provider_id(&self) -> Option<&ProviderId> {
        self.provider_id.as_ref()
    }

    /// Returns the device identity, when known.
    #[must_use]
    pub fn device(&self) -> Option<&HardwareDeviceIdentity> {
        self.device.as_ref()
    }

    /// Returns the session identity, when known.
    #[must_use]
    pub fn session(&self) -> Option<&HardwareSessionIdentity> {
        self.session.as_ref()
    }

    /// Returns the operation identity, when known.
    #[must_use]
    pub fn operation(&self) -> Option<&HardwareOperationIdentity> {
        self.operation.as_ref()
    }

    /// Returns the error severity.
    #[must_use]
    pub const fn severity(&self) -> ZhalSeverity {
        self.severity
    }

    /// Returns the retryability classification.
    #[must_use]
    pub const fn retryability(&self) -> ZhalRetryability {
        self.retryability
    }

    /// Returns the known operation outcome.
    #[must_use]
    pub const fn outcome(&self) -> ZhalOperationOutcome {
        self.outcome
    }

    /// Returns the structured context entries.
    #[must_use]
    pub fn context(&self) -> &[ZhalErrorContext] {
        &self.context
    }

    /// Returns the first context value for a key.
    ///
    /// The returned value may be sensitive. Callers must inspect
    /// the entry's visibility before exposing it.
    #[must_use]
    pub fn context_value(&self, key: &str) -> Option<&str> {
        self.context
            .iter()
            .find(|entry| entry.key() == key)
            .map(ZhalErrorContext::value)
    }

    /// Returns the first context entry for a key.
    #[must_use]
    pub fn context_entry(&self, key: &str) -> Option<&ZhalErrorContext> {
        self.context.iter().find(|entry| entry.key() == key)
    }

    /// Returns whether an underlying causal error exists.
    #[must_use]
    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    /// Returns the underlying causal error.
    ///
    /// The caller is responsible for deciding whether the source's
    /// own formatting is safe to expose.
    #[must_use]
    pub fn source_error(
        &self,
    ) -> Option<&(dyn StdError + Send + Sync + 'static)> {
        self.source.as_deref()
    }

    /// Sets the component associated with the error.
    #[must_use]
    pub fn in_component(
        mut self,
        component: impl Into<String>,
    ) -> Self {
        self.component = Some(component.into());
        self
    }

    /// Associates the error with a canonical provider identity.
    #[must_use]
    pub fn with_provider(
        mut self,
        provider_id: ProviderId,
    ) -> Self {
        self.provider_id = Some(provider_id);
        self
    }

    /// Associates the error with a provider-scoped device identity.
    ///
    /// The provider identity is also retained as the error's provider
    /// attribution.
    #[must_use]
    pub fn with_device(
        mut self,
        device: HardwareDeviceIdentity,
    ) -> Self {
        self.provider_id = Some(device.provider_id().clone());
        self.device = Some(device);
        self
    }

    /// Associates the error with a hardware session.
    ///
    /// The session's provider identity is retained as provider attribution.
    #[must_use]
    pub fn with_session(
        mut self,
        session: HardwareSessionIdentity,
    ) -> Self {
        self.provider_id = Some(session.provider_id().clone());
        self.session = Some(session);
        self
    }

    /// Associates the error with a hardware operation.
    ///
    /// The operation's provider identity is retained as provider attribution.
    #[must_use]
    pub fn with_operation(
        mut self,
        operation: HardwareOperationIdentity,
    ) -> Self {
        self.provider_id = Some(operation.provider_id().clone());
        self.operation = Some(operation);
        self
    }

    /// Sets the error severity.
    #[must_use]
    pub const fn with_severity(
        mut self,
        severity: ZhalSeverity,
    ) -> Self {
        self.severity = severity;
        self
    }

    /// Sets the retryability classification.
    ///
    /// This does not initiate a retry.
    #[must_use]
    pub const fn with_retryability(
        mut self,
        retryability: ZhalRetryability,
    ) -> Self {
        self.retryability = retryability;
        self
    }

    /// Sets the known operation outcome.
    ///
    /// Callers must not report `Failed` when the provider's actual
    /// operation outcome is unknown.
    #[must_use]
    pub const fn with_outcome(
        mut self,
        outcome: ZhalOperationOutcome,
    ) -> Self {
        self.outcome = outcome;
        self
    }

    /// Adds a public context entry.
    ///
    /// Context entries are retained in insertion order.
    ///
    /// No fixed context-entry count is imposed.
    ///
    /// # Errors
    ///
    /// Returns `EmptyKey` when the supplied key is empty.
    pub fn add_context(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), ZhalContextValidationError> {
        self.context.push(ZhalErrorContext::new(key, value)?);
        Ok(())
    }

    /// Adds a sensitive context entry.
    ///
    /// Its value is redacted from Debug output and diagnostic reports.
    ///
    /// # Errors
    ///
    /// Returns `EmptyKey` when the supplied key is empty.
    pub fn add_sensitive_context(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), ZhalContextValidationError> {
        self.context.push(ZhalErrorContext::sensitive(key, value)?);
        Ok(())
    }

    /// Adds public context using builder-style syntax.
    ///
    /// # Errors
    ///
    /// Returns `EmptyKey` when the supplied key is empty.
    pub fn with_context(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ZhalContextValidationError> {
        self.add_context(key, value)?;
        Ok(self)
    }

    /// Adds sensitive context using builder-style syntax.
    ///
    /// # Errors
    ///
    /// Returns `EmptyKey` when the supplied key is empty.
    pub fn with_sensitive_context(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ZhalContextValidationError> {
        self.add_sensitive_context(key, value)?;
        Ok(self)
    }

    /// Attaches an underlying causal error.
    ///
    /// The original error remains available through
    /// `std::error::Error::source`.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Attaches an already boxed causal error.
    #[must_use]
    pub fn with_boxed_source(
        mut self,
        source: Box<dyn StdError + Send + Sync + 'static>,
    ) -> Self {
        self.source = Some(source);
        self
    }

    /// Returns a serializable, redaction-aware diagnostic report.
    ///
    /// The original causal error is intentionally not serialized.
    /// Callers requiring causal details should handle that information
    /// through an explicitly trusted error-reporting integration.
    #[must_use]
    pub fn report(&self) -> ZhalErrorReport {
        let context = self
            .context
            .iter()
            .map(|entry| ZhalErrorContextReport {
                key: entry.key().to_owned(),
                value: entry.report_value().to_owned(),
                visibility: entry.visibility(),
            })
            .collect();

        ZhalErrorReport {
            schema_version: ERROR_REPORT_SCHEMA_VERSION.to_owned(),
            kind: self.kind,
            code: self.code_str().map(str::to_owned),
            message: self.message.clone(),
            component: self.component.clone(),
            provider_id: self.provider_id
                .as_ref()
                .map(|id| id.as_str().to_owned()),
            device: self.device.as_ref().map(ToString::to_string),
            session: self.session.as_ref().map(|identity| {
                format!(
                    "{}::{}",
                    identity.provider_id(),
                    identity.session_id()
                )
            }),
            operation: self.operation.as_ref().map(|identity| {
                format!(
                    "{}::{}",
                    identity.provider_id(),
                    identity.operation_id()
                )
            }),
            severity: self.severity,
            retryability: self.retryability,
            outcome: self.outcome,
            context,
        }
    }

    /// Creates a cancellation error from the canonical standard-library
    /// operation context.
    ///
    /// This method only observes cancellation. It does not forcibly
    /// interrupt hardware execution or create another cancellation token.
    ///
    /// # Errors
    ///
    /// Returns a cancellation error when the context has received
    /// a cancellation request.
    pub fn check_cancelled(
        context: &OperationContext,
    ) -> ZhalResult<()> {
        if !context.is_cancelled() {
            return Ok(());
        }

        let mut error = Self::new(
            ZhalErrorKind::Cancelled,
            "hardware operation received a cancellation request",
        )
        .with_retryability(ZhalRetryability::NotRetryable)
        .with_outcome(ZhalOperationOutcome::Unknown);

        if let Some(component) = context.component() {
            error = error.in_component(component);
        }

        if let Some(operation) = context.operation() {
            error = error.with_context(
                "caller.operation",
                operation,
            )
            .unwrap_or(error);
        }

        Err(error)
    }

    /// Creates an error representing a partially completed operation.
    ///
    /// The caller must supply a safe summary and attach any available
    /// operation identity and structured completion evidence.
    #[must_use]
    pub fn partial_completion(
        message: impl Into<String>,
    ) -> Self {
        Self::new(
            ZhalErrorKind::PartialCompletion,
            message,
        )
        .with_outcome(ZhalOperationOutcome::PartiallyCompleted)
    }

    /// Creates an error for an operation whose outcome is unknown.
    ///
    /// This is appropriate when submission or execution may have occurred
    /// but the caller cannot establish the final provider-side outcome.
    #[must_use]
    pub fn unknown_outcome(
        message: impl Into<String>,
    ) -> Self {
        Self::new(
            ZhalErrorKind::OutcomeUnknown,
            message,
        )
        .with_outcome(ZhalOperationOutcome::Unknown)
        .with_retryability(ZhalRetryability::Unknown)
    }
}

impl fmt::Display for ZhalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.kind)?;

        if let Some(code) = self.code() {
            write!(formatter, " [{code}]")?;
        }

        write!(formatter, ": {}", self.message)?;

        if let Some(provider) = &self.provider_id {
            write!(formatter, " (provider: {provider})")?;
        }

        if let Some(device) = &self.device {
            write!(formatter, " (device: {device})")?;
        }

        Ok(())
    }
}

impl fmt::Debug for ZhalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Do not format the arbitrary causal source. External error
        // implementations may include credentials or other secrets
        // in their Debug output.
        formatter
            .debug_struct("ZhalError")
            .field("kind", &self.kind)
            .field("code", &self.code)
            .field("message", &self.message)
            .field("component", &self.component)
            .field("provider_id", &self.provider_id)
            .field("device", &self.device)
            .field("session", &self.session)
            .field("operation", &self.operation)
            .field("severity", &self.severity)
            .field("retryability", &self.retryability)
            .field("outcome", &self.outcome)
            .field("context", &self.context)
            .field("has_source", &self.has_source())
            .finish()
    }
}

impl StdError for ZhalError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn StdError + 'static))
    }
}

// ============================================================================
// SERIALIZABLE DIAGNOSTIC REPORT
// ============================================================================

/// Redaction-aware serializable representation of a ZHAL error.
///
/// This is a reporting snapshot, not the canonical live error object.
///
/// It deliberately excludes arbitrary causal source objects and
/// redacts sensitive context values.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ZhalErrorReport {
    /// Error-report schema version.
    pub schema_version: String,

    /// Broad error classification.
    pub kind: ZhalErrorKind,

    /// Optional extensible error code.
    pub code: Option<String>,

    /// Safe human-readable summary.
    pub message: String,

    /// Component associated with the error.
    pub component: Option<String>,

    /// Provider identity, if known.
    pub provider_id: Option<String>,

    /// Provider-scoped device identity, if known.
    pub device: Option<String>,

    /// Provider-scoped session identity, if known.
    pub session: Option<String>,

    /// Provider-scoped operation identity, if known.
    pub operation: Option<String>,

    /// Error severity.
    pub severity: ZhalSeverity,

    /// Retryability classification.
    pub retryability: ZhalRetryability,

    /// Known operation outcome.
    pub outcome: ZhalOperationOutcome,

    /// Structured, redaction-aware context.
    pub context: Vec<ZhalErrorContextReport>,
}

/// Serializable representation of one diagnostic context entry.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ZhalErrorContextReport {
    /// Context key.
    pub key: String,

    /// Public value or redaction marker.
    pub value: String,

    /// Original visibility classification.
    pub visibility: ZhalContextVisibility,
}

// ============================================================================
// STANDARD-LIBRARY ERROR INTEGRATION
// ============================================================================

impl From<IdentityError> for ZhalError {
    fn from(error: IdentityError) -> Self {
        Self::new(
            ZhalErrorKind::InvalidInput,
            "hardware identity validation failed",
        )
        .with_context(
            "identity.component",
            error.component(),
        )
        .unwrap_or_else(|_| {
            Self::new(
                ZhalErrorKind::InvalidInput,
                "hardware identity validation failed",
            )
        })
        .with_context(
            "identity.error_kind",
            error.kind().to_string(),
        )
        .unwrap_or_else(|_| {
            Self::new(
                ZhalErrorKind::InvalidInput,
                "hardware identity validation failed",
            )
        })
        .with_source(error)
    }
}

impl From<StdlibError> for ZhalError {
    fn from(error: StdlibError) -> Self {
        let kind = match error.kind() {
            StdlibErrorKind::InvalidInput => ZhalErrorKind::InvalidInput,
            StdlibErrorKind::InvalidData => ZhalErrorKind::InvalidData,
            StdlibErrorKind::NotImplemented
            | StdlibErrorKind::Unsupported => {
                ZhalErrorKind::CapabilityUnsupported
            }
            StdlibErrorKind::CapabilityUnavailable => {
                ZhalErrorKind::CapabilityUnknown
            }
            StdlibErrorKind::Infeasible => {
                ZhalErrorKind::IncompatibleWorkload
            }
            StdlibErrorKind::ResourceUnavailable => {
                ZhalErrorKind::ResourceUnavailable
            }
            StdlibErrorKind::BudgetExhausted => {
                ZhalErrorKind::ResourceExhausted
            }
            StdlibErrorKind::PermissionDenied => {
                ZhalErrorKind::PermissionDenied
            }
            StdlibErrorKind::NotFound => ZhalErrorKind::DeviceNotFound,
            StdlibErrorKind::AlreadyExists => {
                ZhalErrorKind::ConcurrencyConflict
            }
            StdlibErrorKind::VersionMismatch => {
                ZhalErrorKind::VersionMismatch
            }
            StdlibErrorKind::ContractViolation => {
                ZhalErrorKind::ContractViolation
            }
            StdlibErrorKind::Io
            | StdlibErrorKind::Communication => {
                ZhalErrorKind::HardwareCommunication
            }
            StdlibErrorKind::Serialization => {
                ZhalErrorKind::Serialization
            }
            StdlibErrorKind::Cryptographic => {
                ZhalErrorKind::TrustFailure
            }
            StdlibErrorKind::Provider
            | StdlibErrorKind::Dependency => {
                ZhalErrorKind::ProviderFailure
            }
            StdlibErrorKind::Cancelled => ZhalErrorKind::Cancelled,
            StdlibErrorKind::TimedOut => ZhalErrorKind::OperationTimedOut,
            StdlibErrorKind::Concurrency => {
                ZhalErrorKind::ConcurrencyConflict
            }
            StdlibErrorKind::Internal => ZhalErrorKind::Internal,
            StdlibErrorKind::Other => ZhalErrorKind::Other,
            _ => ZhalErrorKind::Other,
        };

        let retryability = error.retryability().into();
        let message = error.message().to_owned();

        let mut result = Self::new(kind, message)
            .with_retryability(retryability)
            .with_source(error);

        if let Some(component) = result.source
            .as_ref()
            .and_then(|source| source.downcast_ref::<StdlibError>())
            .and_then(StdlibError::component)
        {
            result = result.in_component(component);
        }

        result
    }
}

// ============================================================================
// I/O INTEGRATION
// ============================================================================

impl From<io::Error> for ZhalError {
    fn from(error: io::Error) -> Self {
        Self::new(
            ZhalErrorKind::HardwareCommunication,
            "hardware I/O operation failed",
        )
        .with_retryability(ZhalRetryability::Unknown)
        .with_source(error)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;

    use crate::stdlib::context::{
        CancellationSignal,
        OperationContext,
    };

    use crate::zhal::identity::{
        HardwareDeviceId,
        HardwareOperationId,
        HardwareSessionId,
    };

    use std::sync::Arc;

    #[derive(Debug)]
    struct TestCause(&'static str);

    impl fmt::Display for TestCause {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str(self.0)
        }
    }

    impl StdError for TestCause {}

    #[derive(Debug)]
    struct TestCancellation;

    impl CancellationSignal for TestCancellation {
        fn is_cancelled(&self) -> bool {
            true
        }
    }

    fn provider(value: &str) -> ProviderId {
        ProviderId::new(value).expect("valid provider identity")
    }

    fn device_identity() -> HardwareDeviceIdentity {
        HardwareDeviceIdentity::new(
            provider("provider.test"),
            HardwareDeviceId::new("device.test")
                .expect("valid device identity"),
        )
    }

    // ------------------------------------------------------------------------
    // Classification
    // ------------------------------------------------------------------------

    #[test]
    fn error_kind_names_are_stable() {
        assert_eq!(
            ZhalErrorKind::HardwareCommunication.as_str(),
            "hardware_communication",
        );

        assert_eq!(
            ZhalErrorKind::OutcomeUnknown.as_str(),
            "outcome_unknown",
        );
    }

    #[test]
    fn error_preserves_its_classification() {
        let error = ZhalError::new(
            ZhalErrorKind::DeviceUnavailable,
            "device is currently unavailable",
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::DeviceUnavailable,
        );
    }

    #[test]
    fn unknown_capability_is_not_unsupported_capability() {
        let unknown = ZhalError::new(
            ZhalErrorKind::CapabilityUnknown,
            "capability evidence is unavailable",
        );

        let unsupported = ZhalError::new(
            ZhalErrorKind::CapabilityUnsupported,
            "provider explicitly does not support the capability",
        );

        assert_ne!(unknown.kind(), unsupported.kind());
    }

    #[test]
    fn unavailable_device_is_not_inaccessible_device() {
        let unavailable = ZhalError::new(
            ZhalErrorKind::DeviceUnavailable,
            "device is unavailable",
        );

        let inaccessible = ZhalError::new(
            ZhalErrorKind::DeviceInaccessible,
            "device access was denied",
        );

        assert_ne!(unavailable.kind(), inaccessible.kind());
    }

    // ------------------------------------------------------------------------
    // Error codes
    // ------------------------------------------------------------------------

    #[test]
    fn custom_error_codes_use_standard_library_validation() {
        let error = ZhalError::with_code(
            ZhalErrorKind::HardwareFault,
            "zhal.hardware.sensor_fault",
            "hardware reported a fault",
        )
        .expect("valid extensible error code");

        assert_eq!(
            error.code_str(),
            Some("zhal.hardware.sensor_fault"),
        );
    }

    #[test]
    fn invalid_error_codes_are_rejected() {
        assert!(
            ZhalError::with_code(
                ZhalErrorKind::Other,
                "ZHAL invalid code",
                "failure",
            )
            .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Identity attribution
    // ------------------------------------------------------------------------

    #[test]
    fn device_attribution_preserves_provider_identity() {
        let identity = device_identity();

        let error = ZhalError::new(
            ZhalErrorKind::HardwareFault,
            "device reported a fault",
        )
        .with_device(identity);

        assert_eq!(
            error.provider_id().map(ProviderId::as_str),
            Some("provider.test"),
        );

        assert_eq!(
            error.device().map(ToString::to_string),
            Some("provider.test::device.test".to_owned()),
        );
    }

    #[test]
    fn session_attribution_preserves_provider_identity() {
        let session = HardwareSessionIdentity::new(
            provider("provider.test"),
            HardwareSessionId::new("session.test")
                .expect("valid session identity"),
        );

        let error = ZhalError::new(
            ZhalErrorKind::DeviceInaccessible,
            "session access failed",
        )
        .with_session(session);

        assert_eq!(
            error.provider_id().map(ProviderId::as_str),
            Some("provider.test"),
        );

        assert!(error.session().is_some());
    }

    #[test]
    fn operation_attribution_preserves_provider_identity() {
        let operation = HardwareOperationIdentity::new(
            provider("provider.test"),
            HardwareOperationId::new("operation.test")
                .expect("valid operation identity"),
        );

        let error = ZhalError::new(
            ZhalErrorKind::OperationFailed,
            "operation failed",
        )
        .with_operation(operation);

        assert_eq!(
            error.provider_id().map(ProviderId::as_str),
            Some("provider.test"),
        );

        assert!(error.operation().is_some());
    }

    // ------------------------------------------------------------------------
    // Retryability and outcome
    // ------------------------------------------------------------------------

    #[test]
    fn retryability_defaults_to_unknown() {
        let error = ZhalError::new(
            ZhalErrorKind::ProviderFailure,
            "provider failed",
        );

        assert_eq!(
            error.retryability(),
            ZhalRetryability::Unknown,
        );
    }

    #[test]
    fn retryability_does_not_trigger_automatic_retry() {
        let error = ZhalError::new(
            ZhalErrorKind::HardwareCommunication,
            "communication failed",
        )
        .with_retryability(ZhalRetryability::SafeToRetry);

        assert!(error.retryability().is_safe_to_retry());
    }

    #[test]
    fn partial_completion_is_not_total_failure() {
        let error = ZhalError::partial_completion(
            "some requested hardware operations completed",
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::PartialCompletion,
        );

        assert_eq!(
            error.outcome(),
            ZhalOperationOutcome::PartiallyCompleted,
        );
    }

    #[test]
    fn unknown_remote_outcome_is_not_reported_as_failure() {
        let error = ZhalError::unknown_outcome(
            "provider outcome could not be reconciled",
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::OutcomeUnknown,
        );

        assert_eq!(
            error.outcome(),
            ZhalOperationOutcome::Unknown,
        );

        assert_eq!(
            error.retryability(),
            ZhalRetryability::Unknown,
        );
    }

    // ------------------------------------------------------------------------
    // Structured context
    // ------------------------------------------------------------------------

    #[test]
    fn context_rejects_empty_keys() {
        assert!(
            ZhalErrorContext::new("", "value").is_err()
        );
    }

    #[test]
    fn context_preserves_public_values() {
        let error = ZhalError::new(
            ZhalErrorKind::ProviderFailure,
            "provider failed",
        )
        .with_context("provider.phase", "initialization")
        .expect("valid context");

        assert_eq!(
            error.context_value("provider.phase"),
            Some("initialization"),
        );
    }

    #[test]
    fn sensitive_context_is_redacted_from_debug() {
        let error = ZhalError::new(
            ZhalErrorKind::AuthenticationFailed,
            "provider authentication failed",
        )
        .with_sensitive_context(
            "credential",
            "sensitive-test-value",
        )
        .expect("valid context");

        let debug = format!("{error:?}");

        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains("sensitive-test-value"));
    }

    #[test]
    fn sensitive_context_is_redacted_from_reports() {
        let error = ZhalError::new(
            ZhalErrorKind::AuthenticationFailed,
            "provider authentication failed",
        )
        .with_sensitive_context(
            "credential",
            "sensitive-test-value",
        )
        .expect("valid context");

        let report = error.report();

        assert_eq!(
            report.context[0].value,
            "[REDACTED]",
        );

        assert_eq!(
            report.context[0].visibility,
            ZhalContextVisibility::Sensitive,
        );
    }

    // ------------------------------------------------------------------------
    // Causality
    // ------------------------------------------------------------------------

    #[test]
    fn causal_source_is_preserved() {
        let error = ZhalError::new(
            ZhalErrorKind::ProviderFailure,
            "provider operation failed",
        )
        .with_source(TestCause("underlying provider failure"));

        assert!(error.has_source());

        assert_eq!(
            error.source().map(ToString::to_string),
            Some("underlying provider failure".to_owned()),
        );
    }

    #[test]
    fn debug_does_not_format_arbitrary_causal_source() {
        let error = ZhalError::new(
            ZhalErrorKind::ProviderFailure,
            "provider operation failed",
        )
        .with_source(TestCause("private-source-detail"));

        let debug = format!("{error:?}");

        assert!(debug.contains("has_source"));
        assert!(!debug.contains("private-source-detail"));
    }

    // ------------------------------------------------------------------------
    // Standard-library integration
    // ------------------------------------------------------------------------

    #[test]
    fn standard_library_error_is_adapted_without_losing_cause() {
        let source = StdlibError::new(
            StdlibErrorKind::Communication,
            "standard-library communication failure",
        );

        let error = ZhalError::from(source);

        assert_eq!(
            error.kind(),
            ZhalErrorKind::HardwareCommunication,
        );

        assert!(error.has_source());
    }

    #[test]
    fn identity_error_is_adapted_without_losing_cause() {
        let source = crate::zhal::identity::HardwareResourceId::new("")
            .expect_err("empty identity must be invalid");

        let error = ZhalError::from(source);

        assert_eq!(
            error.kind(),
            ZhalErrorKind::InvalidInput,
        );

        assert!(error.has_source());
    }

    #[test]
    fn io_error_is_adapted_without_losing_cause() {
        let source = io::Error::other("test I/O failure");
        let error = ZhalError::from(source);

        assert_eq!(
            error.kind(),
            ZhalErrorKind::HardwareCommunication,
        );

        assert!(error.has_source());
    }

    // ------------------------------------------------------------------------
    // Cancellation integration
    // ------------------------------------------------------------------------

    #[test]
    fn cancellation_check_succeeds_when_not_cancelled() {
        let context = OperationContext::default();

        assert!(
            ZhalError::check_cancelled(&context).is_ok()
        );
    }

    #[test]
    fn cancellation_check_uses_existing_operation_context() {
        let context = OperationContext::builder()
            .with_cancellation(Arc::new(TestCancellation))
            .build();

        let result = ZhalError::check_cancelled(&context);

        let error = result.expect_err("cancellation was requested");

        assert_eq!(
            error.kind(),
            ZhalErrorKind::Cancelled,
        );

        assert_eq!(
            error.outcome(),
            ZhalOperationOutcome::Unknown,
        );
    }

    // ------------------------------------------------------------------------
    // Diagnostic report
    // ------------------------------------------------------------------------

    #[test]
    fn report_contains_schema_and_classification() {
        let error = ZhalError::new(
            ZhalErrorKind::HardwareFault,
            "hardware fault reported",
        );

        let report = error.report();

        assert_eq!(
            report.schema_version,
            ERROR_REPORT_SCHEMA_VERSION,
        );

        assert_eq!(
            report.kind,
            ZhalErrorKind::HardwareFault,
        );
    }

    #[test]
    fn report_round_trips_through_serde() {
        let error = ZhalError::new(
            ZhalErrorKind::HardwareCommunication,
            "hardware communication failed",
        )
        .with_context("interface.kind", "control")
        .expect("valid context");

        let report = error.report();

        let encoded = serde_json::to_string(&report)
            .expect("report serializes");

        let decoded: ZhalErrorReport =
            serde_json::from_str(&encoded)
                .expect("report deserializes");

        assert_eq!(report, decoded);
    }

    #[test]
    fn diagnostic_report_does_not_contain_causal_source() {
        let error = ZhalError::new(
            ZhalErrorKind::ProviderFailure,
            "provider failed",
        )
        .with_source(TestCause("source-private-detail"));

        let encoded = serde_json::to_string(&error.report())
            .expect("report serializes");

        assert!(!encoded.contains("source-private-detail"));
    }

    // ------------------------------------------------------------------------
    // Scalability behavior
    // ------------------------------------------------------------------------

    #[test]
    fn long_error_messages_are_not_subject_to_arbitrary_limits() {
        let message = "hardware failure ".repeat(16_384);

        let error = ZhalError::new(
            ZhalErrorKind::HardwareFault,
            message.clone(),
        );

        assert_eq!(error.message(), message);
    }

    #[test]
    fn context_count_grows_without_a_hardware_specific_ceiling() {
        let mut error = ZhalError::new(
            ZhalErrorKind::ProviderFailure,
            "provider failed",
        );

        for index in 0..512 {
            error.add_context(
                format!("provider.observation.{index}"),
                format!("value-{index}"),
            )
            .expect("valid context");
        }

        assert_eq!(error.context().len(), 512);
    }

    #[test]
    fn diagnostic_report_preserves_context_order() {
        let mut error = ZhalError::new(
            ZhalErrorKind::ProviderFailure,
            "provider failed",
        );

        error.add_context("first", "1").unwrap();
        error.add_context("second", "2").unwrap();

        let report = error.report();

        assert_eq!(report.context[0].key, "first");
        assert_eq!(report.context[1].key, "second");
    }
}
