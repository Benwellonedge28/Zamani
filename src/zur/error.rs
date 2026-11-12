
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Runtime — Canonical Runtime Error Infrastructure
//!
//! Path: `src/runtime/error.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Defines the canonical operational error contract for the Zamani runtime.
//!
//! This module provides a domain-neutral error infrastructure for:
//!
//! - Classical execution.
//! - Quantum execution.
//! - HDL and hardware execution.
//! - Hybrid execution.
//! - Distributed execution.
//! - AI and tensor execution.
//! - Accelerator execution.
//! - JIT and adaptive execution.
//! - Memory management.
//! - Concurrency and scheduling.
//! - Platform integration.
//! - Runtime services.
//! - Security and isolation.
//! - Reliability and recovery.
//! - I/O and interoperability.
//! - Real-time execution.
//! - Future computational domains.
//!
//! # ARCHITECTURAL OWNERSHIP
//!
//! This module owns runtime operational errors.
//!
//! It does not own:
//!
//! - Source-language semantic diagnostics.
//! - Parser or lexer diagnostics.
//! - Compiler diagnostics.
//! - ZUIR errors.
//! - Classical IR errors.
//! - Quantum IR errors.
//! - QEC error models.
//! - ZQN fault models.
//! - Hardware capability definitions.
//! - Physical resource allocation.
//! - Backend code-generation diagnostics.
//! - Distributed execution plans.
//! - Hybrid semantic models.
//!
//! Those responsibilities remain with their canonical subsystems.
//!
//! # DEPENDENCY DIRECTION
//!
//! ```text
//! Frontend / Semantic
//!         |
//!         v
//!       Compiler
//!         |
//!         v
//!       Backend
//!         |
//!         v
//!       Runtime
//!         |
//!         +--> Memory
//!         +--> Concurrency
//!         +--> Scheduling
//!         +--> JIT
//!         +--> Classical execution
//!         +--> Quantum execution
//!         +--> HDL execution
//!         +--> Hybrid execution
//!         +--> Distributed execution
//!         +--> Platform providers
//!         +--> Future domain providers
//! ```
//!
//! Domain-specific errors are adapted into runtime errors at their
//! operational integration boundaries.
//!
//! This module must not depend on those domain implementations.
//!
//! # ERROR MODEL
//!
//! RuntimeErrorKind
//!     Broad operational classification.
//!
//! RuntimeErrorCode
//!     Extensible machine-readable error identity.
//!
//! RuntimeError
//!     Complete runtime failure record.
//!
//! RuntimeErrorContext
//!     Structured information about the operation.
//!
//! RuntimeErrorSource
//!     Underlying causal error.
//!
//! Retryability
//!     Describes whether retrying may be appropriate.
//!
//! RecoveryDisposition
//!     Describes the recovery state or requirement.
//!
//! RuntimeSeverity
//!     Describes operational severity.
//!
//! These concepts are deliberately independent.
//!
//! # SCALABILITY
//!
//! No arbitrary limits are imposed on:
//!
//! - Number of error codes.
//! - Number of error context entries.
//! - Number of providers.
//! - Number of execution sessions.
//! - Number of computational domains.
//! - Number of devices.
//! - Number of distributed participants.
//! - Number of hardware resources.
//! - Error message length.
//!
//! Collections grow subject to available memory and operating-system
//! allocation capabilities.
//!
//! No finite implementation promises infinite computation or memory.
//!
//! # SAFETY
//!
//! This implementation uses safe Rust only.
//!
//! It does not:
//!
//! - Dereference raw pointers.
//! - Introduce unsafe implementations.
//! - Maintain global mutable error registries.
//! - Perform hidden recovery.
//! - Retry operations automatically.
//! - Print errors.
//! - Log implicitly.
//! - Suppress causal errors.
//!
//! # INTEGRATION
//!
//! Runtime modules should use:
//!
//! ```rust
//! use crate::runtime::error::{
//!     RuntimeError,
//!     RuntimeErrorKind,
//!     RuntimeErrorCode,
//!     RuntimeResult,
//! };
//! ```
//!
//! Domain-specific implementations retain their own error authorities.
//!
//! Conversion into RuntimeError occurs at runtime integration boundaries.
//!
//! # COMPATIBILITY
//!
//! Runtime package version, language version, compiler version,
//! IR schema version, artifact version and runtime ABI version
//! remain separate compatibility concepts.

use std::error::Error;
use std::fmt;
use std::io;
use std::str::FromStr;

// ============================================================================
// RESULT TYPE
// ============================================================================

/// Canonical result type for fallible runtime operations.
///
/// All runtime-owned fallible APIs should prefer this alias.
///
/// Domain-specific error types remain owned by their respective subsystems
/// and should be adapted when crossing into runtime-owned operations.
pub type RuntimeResult<T> = Result<T, RuntimeError>;

// ============================================================================
// ERROR CODE VALIDATION
// ============================================================================

/// Validation failures for runtime error identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RuntimeErrorCodeValidationError {
    /// The supplied identifier is empty.
    Empty,

    /// The identifier contains an empty dot-separated segment.
    EmptySegment,

    /// The identifier contains an invalid character.
    InvalidCharacter,

    /// The identifier begins or ends with an invalid boundary.
    InvalidBoundary,
}

impl fmt::Display for RuntimeErrorCodeValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Empty => "runtime error code must not be empty",
            Self::EmptySegment => {
                "runtime error code contains an empty segment"
            }
            Self::InvalidCharacter => {
                "runtime error code contains an invalid character"
            }
            Self::InvalidBoundary => {
                "runtime error code has an invalid boundary"
            }
        };

        formatter.write_str(message)
    }
}

impl Error for RuntimeErrorCodeValidationError {}

/// An extensible, validated runtime error identifier.
///
/// Error codes are independent of error classification.
///
/// A new domain or provider can introduce a new code without modifying
/// RuntimeErrorKind.
///
/// # Format
///
/// Codes use lowercase ASCII segments separated by dots.
///
/// Permitted segment characters:
///
/// - Lowercase ASCII letters.
/// - Decimal digits.
/// - Hyphens.
/// - Underscores.
///
/// Examples:
///
/// ```text
/// runtime.lifecycle.initialization_failed
/// runtime.memory.allocation_failed
/// runtime.concurrency.task_cancelled
/// runtime.quantum.provider_unavailable
/// runtime.hdl.device_programming_failed
/// runtime.distributed.connection_lost
/// runtime.jit.compilation_failed
/// runtime.security.permission_denied
/// provider.example.execution_failed
/// photonics.calibration.unavailable
/// ```
///
/// No arbitrary code-length ceiling is imposed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuntimeErrorCode(String);

impl RuntimeErrorCode {
    /// Creates a validated runtime error code.
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied identifier does not follow
    /// the runtime error-code syntax.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, RuntimeErrorCodeValidationError> {
        let value = value.into();

        if value.is_empty() {
            return Err(RuntimeErrorCodeValidationError::Empty);
        }

        let mut segment_has_character = false;
        let mut previous_was_dot = false;

        for byte in value.bytes() {
            if byte == b'.' {
                if !segment_has_character {
                    return Err(
                        RuntimeErrorCodeValidationError::EmptySegment,
                    );
                }

                segment_has_character = false;
                previous_was_dot = true;
                continue;
            }

            let valid = byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'-'
                || byte == b'_';

            if !valid {
                return Err(
                    RuntimeErrorCodeValidationError::InvalidCharacter,
                );
            }

            if previous_was_dot && (byte == b'-' || byte == b'_') {
                return Err(
                    RuntimeErrorCodeValidationError::InvalidBoundary,
                );
            }

            segment_has_character = true;
            previous_was_dot = false;
        }

        if !segment_has_character {
            return Err(
                RuntimeErrorCodeValidationError::InvalidBoundary,
            );
        }

        if value.ends_with('-') || value.ends_with('_') {
            return Err(
                RuntimeErrorCodeValidationError::InvalidBoundary,
            );
        }

        Ok(Self(value))
    }

    /// Returns the code as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the code and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for RuntimeErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for RuntimeErrorCode {
    type Err = RuntimeErrorCodeValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

// ============================================================================
// ERROR CLASSIFICATION
// ============================================================================

/// Broad classification of runtime operational failures.
///
/// This enum is intentionally domain-neutral.
///
/// It must not enumerate every possible computing technology.
/// New domains use RuntimeErrorCode to introduce specific identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RuntimeErrorKind {
    /// Invalid runtime input.
    InvalidInput,

    /// Invalid or malformed runtime data.
    InvalidData,

    /// An operation is not implemented by the selected implementation.
    NotImplemented,

    /// The selected implementation does not support an operation.
    Unsupported,

    /// A required service is unavailable.
    ServiceUnavailable,

    /// A required capability is unavailable in the current environment.
    CapabilityUnavailable,

    /// An operation is infeasible under current conditions.
    Infeasible,

    /// A required resource is unavailable.
    ResourceUnavailable,

    /// An explicitly configured operational budget was exhausted.
    BudgetExhausted,

    /// A required permission was denied.
    PermissionDenied,

    /// A requested runtime entity was not found.
    NotFound,

    /// An entity already exists where a new entity was required.
    AlreadyExists,

    /// An ABI, API, schema or version is incompatible.
    VersionMismatch,

    /// A runtime contract was violated.
    ContractViolation,

    /// An operation failed because of I/O.
    Io,

    /// An operation failed because of communication.
    Communication,

    /// Serialization or deserialization failed.
    Serialization,

    /// A cryptographic operation failed.
    Cryptographic,

    /// A provider failed.
    Provider,

    /// A required dependency failed.
    Dependency,

    /// A runtime lifecycle operation failed.
    Lifecycle,

    /// An artifact could not be loaded or validated.
    Artifact,

    /// A memory operation failed.
    Memory,

    /// A concurrency operation failed.
    Concurrency,

    /// Scheduling or execution coordination failed.
    Scheduling,

    /// Execution failed.
    Execution,

    /// Runtime compilation or JIT execution failed.
    Compilation,

    /// An operation was cancelled.
    Cancelled,

    /// An operation exceeded its applicable time allowance.
    TimedOut,

    /// A runtime security or isolation violation occurred.
    Security,

    /// A distributed operation failed.
    Distributed,

    /// A hybrid execution operation failed.
    Hybrid,

    /// Recovery or cleanup failed.
    Recovery,

    /// An internal runtime invariant was violated.
    Internal,

    /// A failure does not fit another built-in classification.
    Other,
}

impl RuntimeErrorKind {
    /// Returns the stable textual classification identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInput => "invalid_input",
            Self::InvalidData => "invalid_data",
            Self::NotImplemented => "not_implemented",
            Self::Unsupported => "unsupported",
            Self::ServiceUnavailable => "service_unavailable",
            Self::CapabilityUnavailable => "capability_unavailable",
            Self::Infeasible => "infeasible",
            Self::ResourceUnavailable => "resource_unavailable",
            Self::BudgetExhausted => "budget_exhausted",
            Self::PermissionDenied => "permission_denied",
            Self::NotFound => "not_found",
            Self::AlreadyExists => "already_exists",
            Self::VersionMismatch => "version_mismatch",
            Self::ContractViolation => "contract_violation",
            Self::Io => "io",
            Self::Communication => "communication",
            Self::Serialization => "serialization",
            Self::Cryptographic => "cryptographic",
            Self::Provider => "provider",
            Self::Dependency => "dependency",
            Self::Lifecycle => "lifecycle",
            Self::Artifact => "artifact",
            Self::Memory => "memory",
            Self::Concurrency => "concurrency",
            Self::Scheduling => "scheduling",
            Self::Execution => "execution",
            Self::Compilation => "compilation",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::Security => "security",
            Self::Distributed => "distributed",
            Self::Hybrid => "hybrid",
            Self::Recovery => "recovery",
            Self::Internal => "internal",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for RuntimeErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// SEVERITY
// ============================================================================

/// Operational severity of a runtime error.
///
/// Severity describes impact. It does not independently determine
/// whether recovery is possible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeSeverity {
    /// Informational operational condition.
    Informational,

    /// An operation failed but the runtime may continue normally.
    Warning,

    /// An execution or service operation failed.
    Error,

    /// A critical runtime operation or service failed.
    Critical,

    /// Runtime integrity or continued operation is compromised.
    Fatal,
}

impl RuntimeSeverity {
    /// Returns the stable textual representation.
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

impl fmt::Display for RuntimeSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// RETRYABILITY
// ============================================================================

/// Describes whether an operation may be retried.
///
/// This is descriptive metadata, not an automatic retry instruction.
///
/// Before retrying, the caller must consider:
///
/// - Operation idempotency.
/// - Existing side effects.
/// - Transaction state.
/// - Resource availability.
/// - Cancellation.
/// - Provider contracts.
/// - Applicable retry policies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Retryability {
    /// Available information is insufficient.
    Unknown,

    /// Retrying without changing relevant conditions is not appropriate.
    NotRetryable,

    /// Retrying may succeed after conditions change.
    RetryableAfterConditionChange,

    /// Retrying may succeed.
    Retryable,

    /// The operation's documented contract explicitly permits safe retry.
    SafeToRetry,
}

impl Retryability {
    /// Returns the stable textual representation.
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

impl fmt::Display for Retryability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// RECOVERY DISPOSITION
// ============================================================================

/// Describes the recovery disposition of a failed operation.
///
/// This does not execute recovery.
///
/// Actual recovery policies belong to the reliability and resilience
/// subsystems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryDisposition {
    /// Recovery requirements have not been determined.
    Unspecified,

    /// The failed operation does not require recovery.
    NotRequired,

    /// The operation may be retried under the applicable policy.
    RetryMayBeAppropriate,

    /// Conditions must change before another attempt.
    RequiresConditionChange,

    /// A recovery strategy must be selected.
    RecoveryRequired,

    /// A checkpoint or previously established state may be needed.
    CheckpointRequired,

    /// Recovery is not possible under the current contract.
    RecoveryUnavailable,

    /// The operation requires intervention outside the runtime.
    ExternalInterventionRequired,
}

impl RecoveryDisposition {
    /// Returns the stable textual representation.
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

impl fmt::Display for RecoveryDisposition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// CONTEXT VISIBILITY
// ============================================================================

/// Visibility classification for structured error context.
///
/// Sensitive values are excluded from ordinary Debug output.
///
/// Callers should still avoid placing passwords, authentication tokens,
/// private keys, or other secrets into error context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextVisibility {
    /// Context may be displayed in diagnostic output.
    Public,

    /// Context must be redacted from ordinary diagnostic output.
    Sensitive,
}

// ============================================================================
// STRUCTURED ERROR CONTEXT
// ============================================================================

/// A structured key/value entry attached to a runtime error.
#[derive(Clone, PartialEq, Eq)]
pub struct RuntimeErrorContext {
    key: String,
    value: String,
    visibility: ContextVisibility,
}

impl RuntimeErrorContext {
    /// Creates a public context entry.
    ///
    /// # Errors
    ///
    /// Returns an error if the key is empty.
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> RuntimeResult<Self> {
        Self::with_visibility(
            key,
            value,
            ContextVisibility::Public,
        )
    }

    /// Creates a context entry with explicit visibility.
    ///
    /// # Errors
    ///
    /// Returns an error if the key is empty.
    pub fn with_visibility(
        key: impl Into<String>,
        value: impl Into<String>,
        visibility: ContextVisibility,
    ) -> RuntimeResult<Self> {
        let key = key.into();

        if key.trim().is_empty() {
            return Err(RuntimeError::invalid_input(
                "runtime error context key must not be empty",
            ));
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
    /// Consumers are responsible for respecting visibility.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the visibility classification.
    #[must_use]
    pub const fn visibility(&self) -> ContextVisibility {
        self.visibility
    }

    /// Returns whether this entry is sensitive.
    #[must_use]
    pub const fn is_sensitive(&self) -> bool {
        matches!(self.visibility, ContextVisibility::Sensitive)
    }
}

impl fmt::Debug for RuntimeErrorContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self.visibility {
            ContextVisibility::Public => self.value.as_str(),
            ContextVisibility::Sensitive => "[REDACTED]",
        };

        formatter
            .debug_struct("RuntimeErrorContext")
            .field("key", &self.key)
            .field("value", &value)
            .field("visibility", &self.visibility)
            .finish()
    }
}

// ============================================================================
// RUNTIME ERROR
// ============================================================================

/// Canonical structured runtime operational error.
///
/// This type preserves:
///
/// - Error classification.
/// - Stable machine-readable identity.
/// - Human-readable message.
/// - Component identity.
/// - Operation identity.
/// - Provider identity.
/// - Execution identity.
/// - Structured context.
/// - Severity.
/// - Retryability.
/// - Recovery disposition.
/// - Underlying causal error.
///
/// # Ownership
///
/// RuntimeError owns its message and context.
///
/// Its source is owned through a boxed standard Rust error.
///
/// # Cloning
///
/// RuntimeError intentionally does not implement Clone.
///
/// An arbitrary causal error cannot generally be cloned without losing
/// its original identity or implementation-specific information.
///
/// If an error must be shared, use Arc around the complete error.
pub struct RuntimeError {
    kind: RuntimeErrorKind,
    code: RuntimeErrorCode,
    message: String,

    component: Option<String>,
    operation: Option<String>,
    provider: Option<String>,
    execution: Option<String>,

    severity: RuntimeSeverity,
    retryability: Retryability,
    recovery: RecoveryDisposition,

    context: Vec<RuntimeErrorContext>,

    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl RuntimeError {
    /// Creates a runtime error.
    ///
    /// The supplied code must already have been validated.
    ///
    /// # Arguments
    ///
    /// - `kind`: broad runtime failure classification.
    /// - `code`: stable extensible error identifier.
    /// - `message`: human-readable explanation.
    #[must_use]
    pub fn new(
        kind: RuntimeErrorKind,
        code: RuntimeErrorCode,
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

            severity: RuntimeSeverity::Error,
            retryability: Retryability::Unknown,
            recovery: RecoveryDisposition::Unspecified,

            context: Vec::new(),

            source: None,
        }
    }

    /// Creates an error with a dynamically supplied code.
    ///
    /// # Errors
    ///
    /// Returns an error if the code is invalid.
    pub fn with_code(
        kind: RuntimeErrorKind,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> RuntimeResult<Self> {
        let code = RuntimeErrorCode::new(code).map_err(|error| {
            RuntimeError::invalid_input(error.to_string())
        })?;

        Ok(Self::new(kind, code, message))
    }

    /// Creates a general runtime error.
    ///
    /// This constructor is intended for failures that do not fit
    /// another built-in classification.
    pub fn other(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> RuntimeResult<Self> {
        Self::with_code(RuntimeErrorKind::Other, code, message)
    }

    /// Creates an invalid-input error.
    #[must_use]
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::standard(
            RuntimeErrorKind::InvalidInput,
            "runtime.input.invalid",
            message,
        )
    }

    /// Creates an invalid-data error.
    #[must_use]
    pub fn invalid_data(message: impl Into<String>) -> Self {
        Self::standard(
            RuntimeErrorKind::InvalidData,
            "runtime.data.invalid",
            message,
        )
    }

    /// Creates an unsupported-operation error.
    #[must_use]
    pub fn unsupported(message: impl Into<String>) -> Self {
        Self::standard(
            RuntimeErrorKind::Unsupported,
            "runtime.operation.unsupported",
            message,
        )
    }

    /// Creates an unavailable-service error.
    #[must_use]
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::standard(
            RuntimeErrorKind::ServiceUnavailable,
            "runtime.service.unavailable",
            message,
        )
    }

    /// Creates an unavailable-resource error.
    #[must_use]
    pub fn resource_unavailable(message: impl Into<String>) -> Self {
        Self::standard(
            RuntimeErrorKind::ResourceUnavailable,
            "runtime.resource.unavailable",
            message,
        )
    }

    /// Creates a configured-budget exhaustion error.
    #[must_use]
    pub fn budget_exhausted(message: impl Into<String>) -> Self {
        Self::standard(
            RuntimeErrorKind::BudgetExhausted,
            "runtime.resource.budget_exhausted",
            message,
        )
    }

    /// Creates a permission-denied error.
    #[must_use]
    pub fn permission_denied(message: impl Into<String>) -> Self {
        Self::standard(
            RuntimeErrorKind::PermissionDenied,
            "runtime.security.permission_denied",
            message,
        )
    }

    /// Creates an operation-cancelled error.
    #[must_use]
    pub fn cancelled(message: impl Into<String>) -> Self {
        Self::standard(
            RuntimeErrorKind::Cancelled,
            "runtime.execution.cancelled",
            message,
        )
    }

    /// Creates a timeout error.
    #[must_use]
    pub fn timed_out(message: impl Into<String>) -> Self {
        Self::standard(
            RuntimeErrorKind::TimedOut,
            "runtime.execution.timed_out",
            message,
        )
    }

    /// Creates an internal runtime error.
    #[must_use]
    pub fn internal(message: impl Into<String>) -> Self {
        Self::standard(
            RuntimeErrorKind::Internal,
            "runtime.internal.failure",
            message,
        )
    }

    /// Internal helper for standard error constructors.
    fn standard(
        kind: RuntimeErrorKind,
        code: &'static str,
        message: impl Into<String>,
    ) -> Self {
        let code = RuntimeErrorCode::new(code)
            .expect("built-in runtime error code must be valid");

        Self::new(kind, code, message)
    }

    /// Attaches the component that produced the error.
    #[must_use]
    pub fn with_component(
        mut self,
        component: impl Into<String>,
    ) -> Self {
        self.component = Some(component.into());
        self
    }

    /// Attaches the operation that failed.
    #[must_use]
    pub fn with_operation(
        mut self,
        operation: impl Into<String>,
    ) -> Self {
        self.operation = Some(operation.into());
        self
    }

    /// Attaches the provider identity.
    #[must_use]
    pub fn with_provider(
        mut self,
        provider: impl Into<String>,
    ) -> Self {
        self.provider = Some(provider.into());
        self
    }

    /// Attaches the execution identity.
    ///
    /// The identity is kept as an opaque string so this foundational
    /// module does not depend on runtime IDs that are being implemented
    /// in separate files.
    #[must_use]
    pub fn with_execution(
        mut self,
        execution: impl Into<String>,
    ) -> Self {
        self.execution = Some(execution.into());
        self
    }

    /// Sets operational severity.
    #[must_use]
    pub fn with_severity(
        mut self,
        severity: RuntimeSeverity,
    ) -> Self {
        self.severity = severity;
        self
    }

    /// Sets retryability metadata.
    #[must_use]
    pub fn with_retryability(
        mut self,
        retryability: Retryability,
    ) -> Self {
        self.retryability = retryability;
        self
    }

    /// Sets recovery disposition metadata.
    #[must_use]
    pub fn with_recovery(
        mut self,
        recovery: RecoveryDisposition,
    ) -> Self {
        self.recovery = recovery;
        self
    }

    /// Adds a structured context entry.
    ///
    /// Context is appended rather than silently replacing previous entries.
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied context key is empty.
    pub fn add_context(
        &mut self,
        context: RuntimeErrorContext,
    ) {
        self.context.push(context);
    }

    /// Adds a public context entry.
    ///
    /// # Errors
    ///
    /// Returns an error if the key is empty.
    pub fn with_context(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> RuntimeResult<Self> {
        self.context.push(RuntimeErrorContext::new(key, value)?);
        Ok(self)
    }

    /// Adds a sensitive context entry.
    ///
    /// Sensitive values are redacted from Debug output.
    ///
    /// # Errors
    ///
    /// Returns an error if the key is empty.
    pub fn with_sensitive_context(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> RuntimeResult<Self> {
        self.context.push(RuntimeErrorContext::with_visibility(
            key,
            value,
            ContextVisibility::Sensitive,
        )?);

        Ok(self)
    }

    /// Attaches an underlying causal error.
    ///
    /// The original error is preserved as the standard Rust error source.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Returns the broad error classification.
    #[must_use]
    pub const fn kind(&self) -> RuntimeErrorKind {
        self.kind
    }

    /// Returns the stable error identifier.
    #[must_use]
    pub fn code(&self) -> &RuntimeErrorCode {
        &self.code
    }

    /// Returns the human-readable error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the originating component, if known.
    #[must_use]
    pub fn component(&self) -> Option<&str> {
        self.component.as_deref()
    }

    /// Returns the failed operation, if known.
    #[must_use]
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Returns the provider identity, if known.
    #[must_use]
    pub fn provider(&self) -> Option<&str> {
        self.provider.as_deref()
    }

    /// Returns the execution identity, if known.
    #[must_use]
    pub fn execution(&self) -> Option<&str> {
        self.execution.as_deref()
    }

    /// Returns operational severity.
    #[must_use]
    pub const fn severity(&self) -> RuntimeSeverity {
        self.severity
    }

    /// Returns retryability metadata.
    #[must_use]
    pub const fn retryability(&self) -> Retryability {
        self.retryability
    }

    /// Returns recovery disposition.
    #[must_use]
    pub const fn recovery(&self) -> RecoveryDisposition {
        self.recovery
    }

    /// Returns all structured context entries.
    #[must_use]
    pub fn context(&self) -> &[RuntimeErrorContext] {
        &self.context
    }

    /// Returns the underlying causal error.
    #[must_use]
    pub fn source_error(&self) -> Option<&(dyn Error + Send + Sync + 'static)> {
        self.source.as_deref()
    }

    /// Returns the first context entry matching the supplied key.
    #[must_use]
    pub fn context_value(&self, key: &str) -> Option<&str> {
        self.context
            .iter()
            .find(|entry| entry.key() == key)
            .map(RuntimeErrorContext::value)
    }

    /// Returns whether the error indicates a condition requiring
    /// external intervention.
    #[must_use]
    pub const fn requires_external_intervention(&self) -> bool {
        matches!(
            self.recovery,
            RecoveryDisposition::ExternalInterventionRequired
        )
    }
}

// ============================================================================
// DEBUG AND DISPLAY
// ============================================================================

impl fmt::Debug for RuntimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RuntimeError")
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
            .field("source", &self.source)
            .finish()
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} [{}]: {}",
            self.kind,
            self.code,
            self.message
        )?;

        if let Some(component) = &self.component {
            write!(formatter, " (component: {component})")?;
        }

        if let Some(operation) = &self.operation {
            write!(formatter, " (operation: {operation})")?;
        }

        if let Some(provider) = &self.provider {
            write!(formatter, " (provider: {provider})")?;
        }

        if let Some(execution) = &self.execution {
            write!(formatter, " (execution: {execution})")?;
        }

        Ok(())
    }
}

impl Error for RuntimeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn Error + 'static))
    }
}

// ============================================================================
// STANDARD LIBRARY INTEGRATION
// ============================================================================

/// Converts an operating-system I/O error into a runtime error.
///
/// The original I/O error remains available through source chaining.
impl From<io::Error> for RuntimeError {
    fn from(source: io::Error) -> Self {
        let message = source.to_string();

        Self::standard(
            RuntimeErrorKind::Io,
            "runtime.io.operation_failed",
            message,
        )
        .with_source(source)
    }
}

/// Converts a runtime error into a standard boxed error.
///
/// The original runtime error remains the causal object.
impl From<RuntimeError> for Box<dyn Error + Send + Sync> {
    fn from(error: RuntimeError) -> Self {
        Box::new(error)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_error_codes_are_accepted() {
        let code = RuntimeErrorCode::new(
            "runtime.quantum.provider_unavailable",
        )
        .expect("valid error code");

        assert_eq!(
            code.as_str(),
            "runtime.quantum.provider_unavailable"
        );
    }

    #[test]
    fn future_domain_error_codes_are_accepted() {
        let code = RuntimeErrorCode::new(
            "photonics.calibration.unavailable",
        )
        .expect("future domain error code");

        assert_eq!(
            code.as_str(),
            "photonics.calibration.unavailable"
        );
    }

    #[test]
    fn empty_error_codes_are_rejected() {
        assert_eq!(
            RuntimeErrorCode::new(""),
            Err(RuntimeErrorCodeValidationError::Empty)
        );
    }

    #[test]
    fn malformed_error_codes_are_rejected() {
        assert_eq!(
            RuntimeErrorCode::new("runtime..memory"),
            Err(RuntimeErrorCodeValidationError::EmptySegment)
        );

        assert_eq!(
            RuntimeErrorCode::new("Runtime.memory"),
            Err(RuntimeErrorCodeValidationError::InvalidCharacter)
        );

        assert_eq!(
            RuntimeErrorCode::new("runtime.memory-"),
            Err(RuntimeErrorCodeValidationError::InvalidBoundary)
        );

        assert_eq!(
            RuntimeErrorCode::new("runtime.-memory"),
            Err(RuntimeErrorCodeValidationError::InvalidBoundary)
        );
    }

    #[test]
    fn error_code_from_str_works() {
        let code: RuntimeErrorCode =
            "runtime.execution.failed"
                .parse()
                .expect("valid error code");

        assert_eq!(code.as_str(), "runtime.execution.failed");
    }

    #[test]
    fn runtime_error_preserves_classification_and_message() {
        let error = RuntimeError::invalid_input("invalid execution request");

        assert_eq!(error.kind(), RuntimeErrorKind::InvalidInput);

        assert_eq!(
            error.code().as_str(),
            "runtime.input.invalid"
        );

        assert_eq!(
            error.message(),
            "invalid execution request"
        );
    }

    #[test]
    fn runtime_error_preserves_operational_metadata() {
        let error = RuntimeError::service_unavailable(
            "requested service is unavailable",
        )
        .with_component("runtime.services")
        .with_operation("activate")
        .with_provider("example.provider")
        .with_execution("execution-identity")
        .with_severity(RuntimeSeverity::Critical)
        .with_retryability(Retryability::RetryableAfterConditionChange)
        .with_recovery(RecoveryDisposition::RequiresConditionChange);

        assert_eq!(error.component(), Some("runtime.services"));
        assert_eq!(error.operation(), Some("activate"));
        assert_eq!(error.provider(), Some("example.provider"));
        assert_eq!(error.execution(), Some("execution-identity"));

        assert_eq!(error.severity(), RuntimeSeverity::Critical);

        assert_eq!(
            error.retryability(),
            Retryability::RetryableAfterConditionChange
        );

        assert_eq!(
            error.recovery(),
            RecoveryDisposition::RequiresConditionChange
        );
    }

    #[test]
    fn context_entries_are_preserved() {
        let error = RuntimeError::internal("execution invariant failed")
            .with_context("stage", "dispatch")
            .expect("valid context")
            .with_context("domain", "classical")
            .expect("valid context");

        assert_eq!(error.context().len(), 2);

        assert_eq!(
            error.context_value("stage"),
            Some("dispatch")
        );

        assert_eq!(
            error.context_value("domain"),
            Some("classical")
        );
    }

    #[test]
    fn sensitive_context_is_redacted_from_debug_output() {
        let error = RuntimeError::internal("operation failed")
            .with_sensitive_context(
                "credential",
                "sensitive-value",
            )
            .expect("valid context");

        let debug = format!("{error:?}");

        assert!(!debug.contains("sensitive-value"));
        assert!(debug.contains("[REDACTED]"));
    }

    #[test]
    fn sensitive_context_remains_available_to_authorized_consumers() {
        let error = RuntimeError::internal("operation failed")
            .with_sensitive_context(
                "credential",
                "sensitive-value",
            )
            .expect("valid context");

        let entry = &error.context()[0];

        assert_eq!(entry.value(), "sensitive-value");
        assert!(entry.is_sensitive());
    }

    #[test]
    fn empty_context_keys_are_rejected() {
        let result = RuntimeErrorContext::new("", "value");

        assert!(result.is_err());

        let result = RuntimeErrorContext::new("   ", "value");

        assert!(result.is_err());
    }

    #[test]
    fn causal_source_is_preserved() {
        let source = io::Error::new(
            io::ErrorKind::NotFound,
            "underlying file does not exist",
        );

        let error = RuntimeError::internal("artifact loading failed")
            .with_source(source);

        assert!(error.source_error().is_some());

        assert!(Error::source(&error).is_some());
    }

    #[test]
    fn io_errors_convert_into_runtime_errors() {
        let source = io::Error::new(
            io::ErrorKind::PermissionDenied,
            "permission denied",
        );

        let error = RuntimeError::from(source);

        assert_eq!(error.kind(), RuntimeErrorKind::Io);

        assert_eq!(
            error.code().as_str(),
            "runtime.io.operation_failed"
        );

        assert!(error.source_error().is_some());
    }

    #[test]
    fn external_intervention_is_explicit() {
        let error = RuntimeError::internal("manual intervention required")
            .with_recovery(
                RecoveryDisposition::ExternalInterventionRequired,
            );

        assert!(error.requires_external_intervention());
    }

    #[test]
    fn display_does_not_expose_sensitive_context() {
        let error = RuntimeError::internal("operation failed")
            .with_sensitive_context(
                "credential",
                "private-value",
            )
            .expect("valid context");

        let display = error.to_string();

        assert!(!display.contains("private-value"));
    }

    #[test]
    fn error_codes_have_no_artificial_length_limit() {
        let long_segment = "a".repeat(16_384);

        let value = format!("runtime.{long_segment}.failure");

        let code = RuntimeErrorCode::new(value.clone())
            .expect("valid dynamically sized code");

        assert_eq!(code.as_str(), value);
    }

    #[test]
    fn result_alias_accepts_runtime_errors() {
        fn operation() -> RuntimeResult<()> {
            Err(RuntimeError::cancelled("operation cancelled"))
        }

        let result = operation();

        assert!(result.is_err());
    }

    #[test]
    fn new_domains_do_not_require_new_error_kind_variants() {
        let error = RuntimeError::with_code(
            RuntimeErrorKind::Provider,
            "photonics.device.calibration_failed",
            "photonic provider calibration failed",
        )
        .expect("valid extensible error");

        assert_eq!(
            error.code().as_str(),
            "photonics.device.calibration_failed"
        );

        assert_eq!(error.kind(), RuntimeErrorKind::Provider);
    }

    #[test]
    fn display_contains_stable_error_identity() {
        let error = RuntimeError::invalid_input("invalid request");

        let displayed = error.to_string();

        assert!(displayed.contains("invalid_input"));
        assert!(displayed.contains("runtime.input.invalid"));
        assert!(displayed.contains("invalid request"));
    }
}
