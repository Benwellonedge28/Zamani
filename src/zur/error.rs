
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Error Infrastructure
//!
//! Path: src/zur/error.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//!
//! PURPOSE
//!
//! Defines the canonical operational error model for ZUR.
//!
//! Supports runtime operations across:
//! - Classical computing.
//! - Quantum computing.
//! - HDL and hardware.
//! - AI and tensor computation.
//! - Hybrid computing.
//! - Distributed computing.
//! - Accelerators.
//! - Memory.
//! - Concurrency.
//! - Scheduling.
//! - JIT execution.
//! - Platform integration.
//! - Future computational domains.
//!
//! OWNERSHIP
//!
//! This module owns operational errors produced by ZUR.
//!
//! It does not own:
//! - Source-language diagnostics.
//! - Semantic errors.
//! - Compiler diagnostics.
//! - ZUIR verification errors.
//! - Quantum error correction.
//! - Quantum noise and fault models.
//! - Hardware capability definitions.
//! - Backend code-generation errors.
//! - Distributed execution planning.
//! - Hybrid semantic models.
//!
//! Those errors are adapted at their ZUR integration boundaries.
//!
//! SCALABILITY
//!
//! No arbitrary hardware, device, provider, domain, or error-context
//! count is imposed.
//!
//! Storage grows subject to available resources.
//!
//! No finite machine promises literal infinite computation.
//!
//! SAFETY
//!
//! Safe Rust only.
//! No global mutable error registry.
//! No automatic retry.
//! No fabricated recovery.
//! No implicit logging.
//! No suppressed causal errors.

use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::str::FromStr;

// ============================================================================
// RESULT
// ============================================================================

/// Canonical result type for ZUR-owned fallible operations.
pub type ZurResult<T> = Result<T, ZurError>;

/// Compatibility name for consumers using Runtime terminology.
pub type RuntimeResult<T> = ZurResult<T>;

// ============================================================================
// ERROR CODE VALIDATION
// ============================================================================

/// Error-code validation failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorCodeValidationError {
    Empty,
    EmptySegment,
    InvalidCharacter,
    InvalidBoundary,
}

impl fmt::Display for ErrorCodeValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Empty => "error code must not be empty",
            Self::EmptySegment => "error code contains an empty segment",
            Self::InvalidCharacter => "error code contains an invalid character",
            Self::InvalidBoundary => "error code has an invalid boundary",
        })
    }
}

impl StdError for ErrorCodeValidationError {}

/// Extensible, validated machine-readable error identity.
///
/// Format:
///
/// lowercase.segment.identifier
///
/// Allowed characters:
/// - a-z
/// - 0-9
/// - hyphen
/// - underscore
/// - dot as a segment separator
///
/// There is no arbitrary code-length ceiling.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorCode(String);

impl ErrorCode {
    /// Constructs a validated error code.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, ErrorCodeValidationError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ErrorCodeValidationError::Empty);
        }

        let mut segment_has_character = false;
        let mut previous_was_separator = false;

        for byte in value.bytes() {
            if byte == b'.' {
                if !segment_has_character {
                    return Err(ErrorCodeValidationError::EmptySegment);
                }

                segment_has_character = false;
                previous_was_separator = true;
                continue;
            }

            let valid = byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'-'
                || byte == b'_';

            if !valid {
                return Err(ErrorCodeValidationError::InvalidCharacter);
            }

            if previous_was_separator && (byte == b'-' || byte == b'_') {
                return Err(ErrorCodeValidationError::InvalidBoundary);
            }

            segment_has_character = true;
            previous_was_separator = false;
        }

        if !segment_has_character
            || value.ends_with('-')
            || value.ends_with('_')
        {
            return Err(ErrorCodeValidationError::InvalidBoundary);
        }

        Ok(Self(value))
    }

    /// Returns the validated identifier.
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

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ErrorCode {
    type Err = ErrorCodeValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

// ============================================================================
// ERROR CLASSIFICATION
// ============================================================================

/// Broad operational error classification.
///
/// This enum deliberately does not enumerate computational domains.
/// Future providers introduce new ErrorCode values without modifying it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    Configuration,
    Validation,
    InvalidInput,
    InvalidData,
    Unsupported,
    NotImplemented,

    Lifecycle,
    Dependency,
    ServiceUnavailable,
    Provider,
    CapabilityUnavailable,
    Infeasible,

    ResourceUnavailable,
    BudgetExhausted,
    Memory,

    Compatibility,
    Artifact,
    ContractViolation,

    Io,
    Communication,
    Serialization,
    Cryptographic,

    Concurrency,
    Scheduling,
    Execution,
    Compilation,

    Cancelled,
    TimedOut,

    Security,
    PermissionDenied,

    Distributed,
    Hybrid,

    Checkpoint,
    Recovery,
    Integrity,

    Internal,
    Other,
}

impl ErrorKind {
    /// Returns the stable textual classification.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Configuration => "configuration",
            Self::Validation => "validation",
            Self::InvalidInput => "invalid_input",
            Self::InvalidData => "invalid_data",
            Self::Unsupported => "unsupported",
            Self::NotImplemented => "not_implemented",
            Self::Lifecycle => "lifecycle",
            Self::Dependency => "dependency",
            Self::ServiceUnavailable => "service_unavailable",
            Self::Provider => "provider",
            Self::CapabilityUnavailable => "capability_unavailable",
            Self::Infeasible => "infeasible",
            Self::ResourceUnavailable => "resource_unavailable",
            Self::BudgetExhausted => "budget_exhausted",
            Self::Memory => "memory",
            Self::Compatibility => "compatibility",
            Self::Artifact => "artifact",
            Self::ContractViolation => "contract_violation",
            Self::Io => "io",
            Self::Communication => "communication",
            Self::Serialization => "serialization",
            Self::Cryptographic => "cryptographic",
            Self::Concurrency => "concurrency",
            Self::Scheduling => "scheduling",
            Self::Execution => "execution",
            Self::Compilation => "compilation",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::Security => "security",
            Self::PermissionDenied => "permission_denied",
            Self::Distributed => "distributed",
            Self::Hybrid => "hybrid",
            Self::Checkpoint => "checkpoint",
            Self::Recovery => "recovery",
            Self::Integrity => "integrity",
            Self::Internal => "internal",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// SEVERITY
// ============================================================================

/// Operational severity.
///
/// Severity does not independently determine recoverability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    Informational,
    Warning,
    Error,
    Critical,
    Fatal,
}

impl Severity {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Informational => "informational",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Critical => "critical",
            Self::Fatal => "fatal",
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// RETRYABILITY
// ============================================================================

/// Retry metadata.
///
/// This is not an instruction to retry automatically.
/// Idempotency and side effects must be considered by the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Retryability {
    Unknown,
    NotRetryable,
    RetryableAfterConditionChange,
    Retryable,
    SafeToRetry,
}

impl Retryability {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::NotRetryable => "not_retryable",
            Self::RetryableAfterConditionChange => {
                "retryable_after_condition_change"
            }
            Self::Retryable => "retryable",
            Self::SafeToRetry => "safe_to_retry",
        }
    }
}

// ============================================================================
// RECOVERY
// ============================================================================

/// Recovery disposition.
///
/// Actual recovery belongs to ZUR reliability and the appropriate
/// domain-specific resilience subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryDisposition {
    Unspecified,
    NotRequired,
    RetryMayBeAppropriate,
    RequiresConditionChange,
    RecoveryRequired,
    CheckpointRequired,
    RecoveryUnavailable,
    ExternalInterventionRequired,
}

impl RecoveryDisposition {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unspecified => "unspecified",
            Self::NotRequired => "not_required",
            Self::RetryMayBeAppropriate => "retry_may_be_appropriate",
            Self::RequiresConditionChange => "requires_condition_change",
            Self::RecoveryRequired => "recovery_required",
            Self::CheckpointRequired => "checkpoint_required",
            Self::RecoveryUnavailable => "recovery_unavailable",
            Self::ExternalInterventionRequired => {
                "external_intervention_required"
            }
        }
    }
}

// ============================================================================
// CONTEXT VISIBILITY
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextVisibility {
    Public,
    Sensitive,
}

/// Validation errors for structured context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextValidationError {
    EmptyKey,
}

impl fmt::Display for ContextValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("runtime error context key must not be empty")
    }
}

impl StdError for ContextValidationError {}

/// Structured operational context.
#[derive(Clone, PartialEq, Eq)]
pub struct ErrorContext {
    key: String,
    value: String,
    visibility: ContextVisibility,
}

impl ErrorContext {
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ContextValidationError> {
        Self::with_visibility(key, value, ContextVisibility::Public)
    }

    pub fn sensitive(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ContextValidationError> {
        Self::with_visibility(key, value, ContextVisibility::Sensitive)
    }

    pub fn with_visibility(
        key: impl Into<String>,
        value: impl Into<String>,
        visibility: ContextVisibility,
    ) -> Result<Self, ContextValidationError> {
        let key = key.into();

        if key.trim().is_empty() {
            return Err(ContextValidationError::EmptyKey);
        }

        Ok(Self {
            key,
            value: value.into(),
            visibility,
        })
    }

    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the original value.
    ///
    /// Callers must respect the visibility classification.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    #[must_use]
    pub const fn visibility(&self) -> ContextVisibility {
        self.visibility
    }

    #[must_use]
    pub const fn is_sensitive(&self) -> bool {
        matches!(self.visibility, ContextVisibility::Sensitive)
    }
}

impl fmt::Debug for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = if self.is_sensitive() {
            "[REDACTED]"
        } else {
            self.value.as_str()
        };

        f.debug_struct("ErrorContext")
            .field("key", &self.key)
            .field("value", &value)
            .field("visibility", &self.visibility)
            .finish()
    }
}

// ============================================================================
// CANONICAL ERROR
// ============================================================================

/// Canonical ZUR operational error.
///
/// This type is intentionally not Clone because arbitrary causal errors
/// cannot generally be cloned without losing their identity.
pub struct ZurError {
    kind: ErrorKind,
    code: ErrorCode,
    message: String,

    component: Option<String>,
    operation: Option<String>,
    provider: Option<String>,
    execution: Option<String>,

    severity: Severity,
    retryability: Retryability,
    recovery: RecoveryDisposition,

    context: Vec<ErrorContext>,

    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl ZurError {
    /// Creates an error from an already validated code.
    #[must_use]
    pub fn new(
        kind: ErrorKind,
        code: ErrorCode,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            code,
            message: message.into(),
            component: None,
            operation: None,
            provider: None,
            execution: None,
            severity: Severity::Error,
            retryability: Retryability::Unknown,
            recovery: RecoveryDisposition::Unspecified,
            context: Vec::new(),
            source: None,
        }
    }

    /// Creates an error using a dynamically supplied code.
    pub fn with_code(
        kind: ErrorKind,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<Self, ErrorCodeValidationError> {
        let code = ErrorCode::new(code)?;
        Ok(Self::new(kind, code, message))
    }

    /// Creates an extensible error for an unclassified failure.
    pub fn other(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<Self, ErrorCodeValidationError> {
        Self::with_code(ErrorKind::Other, code, message)
    }

    fn standard(
        kind: ErrorKind,
        code: &'static str,
        message: impl Into<String>,
    ) -> Self {
        // These identifiers are private, fixed, reviewed constants.
        // Dynamic identifiers always pass through ErrorCode::new().
        Self::new(
            kind,
            ErrorCode(code.to_owned()),
            message,
        )
    }

    #[must_use]
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::standard(
            ErrorKind::InvalidInput,
            "zur.input.invalid",
            message,
        )
    }

    #[must_use]
    pub fn invalid_data(message: impl Into<String>) -> Self {
        Self::standard(
            ErrorKind::InvalidData,
            "zur.data.invalid",
            message,
        )
    }

    #[must_use]
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::standard(
            ErrorKind::Configuration,
            "zur.configuration.invalid",
            message,
        )
    }

    #[must_use]
    pub fn unsupported(message: impl Into<String>) -> Self {
        Self::standard(
            ErrorKind::Unsupported,
            "zur.operation.unsupported",
            message,
        )
    }

    #[must_use]
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::standard(
            ErrorKind::ServiceUnavailable,
            "zur.service.unavailable",
            message,
        )
    }

    #[must_use]
    pub fn capability_unavailable(message: impl Into<String>) -> Self {
        Self::standard(
            ErrorKind::CapabilityUnavailable,
            "zur.capability.unavailable",
            message,
        )
    }

    #[must_use]
    pub fn infeasible(message: impl Into<String>) -> Self {
        Self::standard(
            ErrorKind::Infeasible,
            "zur.execution.infeasible",
            message,
        )
    }

    #[must_use]
    pub fn resource_unavailable(message: impl Into<String>) -> Self {
        Self::standard(
            ErrorKind::ResourceUnavailable,
            "zur.resource.unavailable",
            message,
        )
    }

    #[must_use]
    pub fn budget_exhausted(message: impl Into<String>) -> Self {
        Self::standard(
            ErrorKind::BudgetExhausted,
            "zur.resource.budget_exhausted",
            message,
        )
    }

    #[must_use]
    pub fn permission_denied(message: impl Into<String>) -> Self {
        Self::standard(
            ErrorKind::PermissionDenied,
            "zur.security.permission_denied",
            message,
        )
    }

    #[must_use]
    pub fn cancelled(message: impl Into<String>) -> Self {
        Self::standard(
            ErrorKind::Cancelled,
            "zur.execution.cancelled",
            message,
        )
    }

    #[must_use]
    pub fn timed_out(message: impl Into<String>) -> Self {
        Self::standard(
            ErrorKind::TimedOut,
            "zur.execution.timed_out",
            message,
        )
    }

    #[must_use]
    pub fn compatibility(message: impl Into<String>) -> Self {
        Self::standard(
            ErrorKind::Compatibility,
            "zur.compatibility.failed",
            message,
        )
    }

    #[must_use]
    pub fn internal(message: impl Into<String>) -> Self {
        Self::standard(
            ErrorKind::Internal,
            "zur.internal.failure",
            message,
        )
    }

    // ------------------------------------------------------------------------
    // Metadata builders
    // ------------------------------------------------------------------------

    #[must_use]
    pub fn with_component(mut self, value: impl Into<String>) -> Self {
        self.component = Some(value.into());
        self
    }

    #[must_use]
    pub fn with_operation(mut self, value: impl Into<String>) -> Self {
        self.operation = Some(value.into());
        self
    }

    #[must_use]
    pub fn with_provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    #[must_use]
    pub fn with_execution(mut self, value: impl Into<String>) -> Self {
        self.execution = Some(value.into());
        self
    }

    #[must_use]
    pub fn with_severity(mut self, value: Severity) -> Self {
        self.severity = value;
        self
    }

    #[must_use]
    pub fn with_retryability(mut self, value: Retryability) -> Self {
        self.retryability = value;
        self
    }

    #[must_use]
    pub fn with_recovery(mut self, value: RecoveryDisposition) -> Self {
        self.recovery = value;
        self
    }

    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Appends an already validated context entry.
    pub fn add_context(&mut self, context: ErrorContext) {
        self.context.push(context);
    }

    /// Appends public context.
    pub fn with_context(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ContextValidationError> {
        self.context.push(ErrorContext::new(key, value)?);
        Ok(self)
    }

    /// Appends sensitive context.
    pub fn with_sensitive_context(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, ContextValidationError> {
        self.context.push(ErrorContext::sensitive(key, value)?);
        Ok(self)
    }

    // ------------------------------------------------------------------------
    // Accessors
    // ------------------------------------------------------------------------

    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    #[must_use]
    pub fn code(&self) -> &ErrorCode {
        &self.code
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub fn component(&self) -> Option<&str> {
        self.component.as_deref()
    }

    #[must_use]
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    #[must_use]
    pub fn provider(&self) -> Option<&str> {
        self.provider.as_deref()
    }

    #[must_use]
    pub fn execution(&self) -> Option<&str> {
        self.execution.as_deref()
    }

    #[must_use]
    pub const fn severity(&self) -> Severity {
        self.severity
    }

    #[must_use]
    pub const fn retryability(&self) -> Retryability {
        self.retryability
    }

    #[must_use]
    pub const fn recovery(&self) -> RecoveryDisposition {
        self.recovery
    }

    #[must_use]
    pub fn context(&self) -> &[ErrorContext] {
        &self.context
    }

    #[must_use]
    pub fn context_value(&self, key: &str) -> Option<&str> {
        self.context
            .iter()
            .find(|entry| entry.key() == key)
            .map(ErrorContext::value)
    }

    #[must_use]
    pub fn source_error(
        &self,
    ) -> Option<&(dyn StdError + Send + Sync + 'static)> {
        self.source.as_deref()
    }

    #[must_use]
    pub const fn requires_external_intervention(&self) -> bool {
        matches!(
            self.recovery,
            RecoveryDisposition::ExternalInterventionRequired
        )
    }
}

// ============================================================================
// FORMATTING
// ============================================================================

impl fmt::Debug for ZurError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Do not format arbitrary source errors here: external source
        // implementations may contain secrets in their Debug output.
        f.debug_struct("ZurError")
            .field("kind", &self.kind)
            .field("code", &self.code)
            .field("message", &self.message)
            .field("component", &self.component)
            .field("operation", &self.operation)
            .field("provider", &self.provider)
            .field("execution", &self.execution)
            .field("severity", &self.severity)
            .field("retryability", &self.retryability)
            .field("recovery", &self.recovery)
            .field("context", &self.context)
            .field("has_source", &self.source.is_some())
            .finish()
    }
}

impl fmt::Display for ZurError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{}]: {}",
            self.kind,
            self.code,
            self.message
        )?;

        if let Some(component) = &self.component {
            write!(f, " (component: {component})")?;
        }

        if let Some(operation) = &self.operation {
            write!(f, " (operation: {operation})")?;
        }

        if let Some(provider) = &self.provider {
            write!(f, " (provider: {provider})")?;
        }

        if let Some(execution) = &self.execution {
            write!(f, " (execution: {execution})")?;
        }

        Ok(())
    }
}

impl StdError for ZurError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn StdError + 'static))
    }
}

// ============================================================================
// STANDARD LIBRARY INTEGRATION
// ============================================================================

impl From<io::Error> for ZurError {
    fn from(source: io::Error) -> Self {
        let message = source.to_string();

        Self::standard(
            ErrorKind::Io,
            "zur.io.operation_failed",
            message,
        )
        .with_source(source)
    }
}

/// Compatibility alias for code using RuntimeError.
pub type RuntimeError = ZurError;

/// Compatibility aliases for existing runtime consumers.
pub type RuntimeErrorKind = ErrorKind;
pub type RuntimeErrorCode = ErrorCode;
pub type RuntimeSeverity = Severity;
pub type RuntimeErrorContext = ErrorContext;

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_standard_error_codes() {
        let code = ErrorCode::new("zur.execution.failed")
            .expect("valid code");

        assert_eq!(code.as_str(), "zur.execution.failed");
    }

    #[test]
    fn accepts_future_domain_codes() {
        let code = ErrorCode::new(
            "photonics.calibration.unavailable",
        )
        .expect("future domain code");

        assert_eq!(
            code.as_str(),
            "photonics.calibration.unavailable"
        );
    }

    #[test]
    fn rejects_invalid_error_codes() {
        assert_eq!(
            ErrorCode::new(""),
            Err(ErrorCodeValidationError::Empty)
        );

        assert_eq!(
            ErrorCode::new("zur..memory"),
            Err(ErrorCodeValidationError::EmptySegment)
        );

        assert_eq!(
            ErrorCode::new("Zur.memory"),
            Err(ErrorCodeValidationError::InvalidCharacter)
        );

        assert_eq!(
            ErrorCode::new("zur.-memory"),
            Err(ErrorCodeValidationError::InvalidBoundary)
        );
    }

    #[test]
    fn preserves_error_classification() {
        let error = ZurError::invalid_input("invalid request");

        assert_eq!(error.kind(), ErrorKind::InvalidInput);
        assert_eq!(error.code().as_str(), "zur.input.invalid");
        assert_eq!(error.message(), "invalid request");
    }

    #[test]
    fn preserves_operational_metadata() {
        let error = ZurError::service_unavailable("service unavailable")
            .with_component("zur.services")
            .with_operation("activate")
            .with_provider("example.provider")
            .with_execution("execution-identity")
            .with_severity(Severity::Critical)
            .with_retryability(Retryability::RetryableAfterConditionChange)
            .with_recovery(RecoveryDisposition::RequiresConditionChange);

        assert_eq!(error.component(), Some("zur.services"));
        assert_eq!(error.operation(), Some("activate"));
        assert_eq!(error.provider(), Some("example.provider"));
        assert_eq!(error.execution(), Some("execution-identity"));
        assert_eq!(error.severity(), Severity::Critical);
    }

    #[test]
    fn preserves_context() {
        let error = ZurError::internal("failure")
            .with_context("stage", "dispatch")
            .expect("valid context");

        assert_eq!(error.context_value("stage"), Some("dispatch"));
    }

    #[test]
    fn redacts_sensitive_context() {
        let error = ZurError::internal("failure")
            .with_sensitive_context("credential", "secret-value")
            .expect("valid context");

        let debug = format!("{error:?}");
        let display = error.to_string();

        assert!(!debug.contains("secret-value"));
        assert!(!display.contains("secret-value"));
        assert!(debug.contains("[REDACTED]"));
    }

    #[test]
    fn preserves_causal_source() {
        let source = io::Error::new(
            io::ErrorKind::NotFound,
            "file unavailable",
        );

        let error = ZurError::internal("operation failed")
            .with_source(source);

        assert!(error.source_error().is_some());
        assert!(StdError::source(&error).is_some());
    }

    #[test]
    fn converts_io_errors() {
        let source = io::Error::new(
            io::ErrorKind::PermissionDenied,
            "permission denied",
        );

        let error = ZurError::from(source);

        assert_eq!(error.kind(), ErrorKind::Io);
        assert!(error.source_error().is_some());
    }

    #[test]
    fn supports_future_domains_without_new_variants() {
        let error = ZurError::with_code(
            ErrorKind::Provider,
            "photonics.device.calibration_failed",
            "calibration failed",
        )
        .expect("valid provider error");

        assert_eq!(
            error.code().as_str(),
            "photonics.device.calibration_failed"
        );
    }

    #[test]
    fn supports_large_error_codes() {
        let segment = "a".repeat(16_384);
        let code = format!("zur.{segment}.failure");

        let parsed = ErrorCode::new(code.clone())
            .expect("valid dynamically sized code");

        assert_eq!(parsed.as_str(), code);
    }

    #[test]
    fn error_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ZurError>();
    }

    #[test]
    fn supports_result_alias() {
        fn operation() -> ZurResult<()> {
            Err(ZurError::cancelled("cancelled"))
        }

        assert!(operation().is_err());
    }

    #[test]
    fn external_intervention_is_explicit() {
        let error = ZurError::internal("manual intervention required")
            .with_recovery(
                RecoveryDisposition::ExternalInterventionRequired,
            );

        assert!(error.requires_external_intervention());
    }
}
