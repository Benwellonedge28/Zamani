//! Zamani Classical Computing — Canonical Error Boundary
//!
//! Path: src/classical/error.rs
//! Status: Proposed canonical contract; repository integration pending.
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the canonical error vocabulary for classical computation,
//! including stable machine-readable codes, error classification,
//! severity, retryability, contextual propagation, and a subsystem Result.
//!
//! # Ownership
//!
//! This module owns:
//! - ClassicalError and ClassicalErrorKind;
//! - ClassicalErrorSeverity;
//! - ClassicalResult<T>;
//! - stable error codes and classification;
//! - constructors for common classical failures;
//! - contextual error propagation.
//!
//! It does not own:
//! - diagnostic rendering, source maps, or source spans;
//! - parser, lexer, AST, semantic-model, or canonical IR errors;
//! - hardware discovery, scheduling, or physical resource allocation;
//! - runtime recovery, retry execution, or fallback selection;
//! - numerical algorithms or optimization passes.
//!
//! # Architectural contract
//!
//! Callers must use `kind()` or `code()` for programmatic decisions.
//! Human-readable Display messages are not a machine interface.
//!
//! Errors describe failures; they do not perform retries, allocate resources,
//! change program semantics, or silently recover from invalid operations.
//!
//! Resource exhaustion is distinct from a language-imposed size ceiling.
//! The error reports a real operational constraint or explicitly configured
//! policy. It must not imply that larger computations are semantically invalid.
//!
//! # Integration
//!
//! - `classical/config.rs`: map configuration validation failures into this
//!   error boundary during the planned configuration-error migration.
//! - `classical/diagnostics.rs`: translate error kind, code, severity, and
//!   context into the shared diagnostic representation.
//! - `classical/validation/`: report invalid programs and violated invariants.
//! - `classical/values/`, `types/`, `collections/`, and `numeric/`: report
//!   invalid operations, representation failures, and arithmetic errors.
//! - `classical/optimization/`: report illegal transformations and optimizer
//!   failures without silently accepting an invalid result.
//! - `classical/execution/` and `concurrency/`: report cancellation, timeout,
//!   resource exhaustion, and execution failures.
//! - `classical/lowering/`: map lowering failures without redefining errors
//!   owned by canonical IR or target backends.
//! - `quantum::ir`: retains ownership of quantum-IR-specific errors.
//!
//! # Compatibility
//!
//! Error codes are stable public identifiers. Adding a new kind is normally
//! additive; changing an existing code or its documented meaning requires
//! compatibility review.
//!
//! Error text may improve between releases. Consumers must not parse it.
//!
//! # Safety
//!
//! This module forbids unsafe Rust and performs no I/O or execution.

#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use core::fmt;
use std::error::Error;

/// Canonical result type for classical subsystem operations.
pub type ClassicalResult<T> = std::result::Result<T, ClassicalError>;

/// Stable classification of a classical-computing failure.
///
/// Use this enum or [`ClassicalError::code`] for control flow instead of
/// inspecting formatted error messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClassicalErrorKind {
    /// Caller-provided input is invalid.
    InvalidInput,

    /// Configuration is invalid or internally inconsistent.
    InvalidConfiguration,

    /// A source or semantic validation rule was violated.
    ValidationFailed,

    /// A type or type-constraint rule was violated.
    TypeError,

    /// A value cannot be represented under the requested semantics.
    RepresentationFailure,

    /// An arithmetic operation overflowed or was otherwise invalid.
    ArithmeticError,

    /// A value violates the required numerical contract.
    NumericalError,

    /// A requested operation is not supported by the implementation.
    UnsupportedOperation,

    /// A required capability is unavailable.
    CapabilityUnavailable,

    /// A requested resource or operational budget is unavailable.
    ResourceLimitExceeded,

    /// A required backend or execution service is unavailable.
    BackendUnavailable,

    /// An operation failed during execution.
    ExecutionFailed,

    /// An operation exceeded its applicable time budget.
    Timeout,

    /// An operation was cancelled.
    Cancelled,

    /// A requested determinism or reproducibility guarantee was violated.
    DeterminismViolation,

    /// Serialization or deserialization failed.
    SerializationFailure,

    /// A schema, version, or compatibility contract was violated.
    VersionMismatch,

    /// An external interface or foreign call failed.
    InteroperabilityFailure,

    /// An internal invariant was violated.
    InternalInvariantViolation,

    /// A failure was wrapped with additional context.
    Context,
}

impl ClassicalErrorKind {
    /// Returns the stable machine-readable error code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInput => "classical.invalid_input",
            Self::InvalidConfiguration => "classical.invalid_configuration",
            Self::ValidationFailed => "classical.validation_failed",
            Self::TypeError => "classical.type_error",
            Self::RepresentationFailure => "classical.representation_failure",
            Self::ArithmeticError => "classical.arithmetic_error",
            Self::NumericalError => "classical.numerical_error",
            Self::UnsupportedOperation => "classical.unsupported_operation",
            Self::CapabilityUnavailable => "classical.capability_unavailable",
            Self::ResourceLimitExceeded => "classical.resource_limit_exceeded",
            Self::BackendUnavailable => "classical.backend_unavailable",
            Self::ExecutionFailed => "classical.execution_failed",
            Self::Timeout => "classical.timeout",
            Self::Cancelled => "classical.cancelled",
            Self::DeterminismViolation => "classical.determinism_violation",
            Self::SerializationFailure => "classical.serialization_failure",
            Self::VersionMismatch => "classical.version_mismatch",
            Self::InteroperabilityFailure => "classical.interoperability_failure",
            Self::InternalInvariantViolation => {
                "classical.internal_invariant_violation"
            }
            Self::Context => "classical.context",
        }
    }
}

impl fmt::Display for ClassicalErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Operational severity associated with an error.
///
/// Severity guides diagnostics and policy. It does not itself determine
/// whether execution should retry or terminate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClassicalErrorSeverity {
    /// Invalid caller input or program construction.
    Input,

    /// Configuration or compatibility failure.
    Configuration,

    /// Semantic validation or type failure.
    Validation,

    /// Numerical or arithmetic failure.
    Numerical,

    /// Resource exhaustion, cancellation, or timeout.
    Resource,

    /// Backend, execution, or interoperability failure.
    Operational,

    /// Internal invariant violation.
    Internal,
}

impl ClassicalErrorSeverity {
    /// Returns the stable machine-readable severity name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Input => "input",
            Self::Configuration => "configuration",
            Self::Validation => "validation",
            Self::Numerical => "numerical",
            Self::Resource => "resource",
            Self::Operational => "operational",
            Self::Internal => "internal",
        }
    }
}

/// Canonical classical-computing failure.
///
/// Variants carry structured data where useful. Free-form messages are
/// explanatory text, not stable identifiers.
///
/// `Context` preserves the underlying error so callers can add information
/// without losing its original classification or error chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalError {
    /// Invalid caller input.
    InvalidInput {
        /// Explanation suitable for a diagnostic message.
        message: String,
    },

    /// Invalid configuration.
    InvalidConfiguration {
        /// Explanation of the configuration failure.
        message: String,
    },

    /// Semantic or structural validation failed.
    ValidationFailed {
        /// Explanation of the violated rule.
        message: String,
    },

    /// A type rule or constraint was violated.
    TypeError {
        /// Explanation of the type failure.
        message: String,
    },

    /// The requested value cannot be represented.
    RepresentationFailure {
        /// Description of the representation requirement.
        message: String,
    },

    /// Arithmetic cannot complete under the requested semantics.
    ArithmeticError {
        /// Operation or arithmetic rule that failed.
        operation: String,

        /// Explanation of the failure.
        message: String,
    },

    /// Numerical requirements could not be met.
    NumericalError {
        /// Explanation of the numerical failure.
        message: String,
    },

    /// Requested functionality is unsupported.
    UnsupportedOperation {
        /// Stable or caller-supplied operation identifier.
        operation: String,

        /// Explanation of the limitation.
        message: String,
    },

    /// A required capability is unavailable.
    CapabilityUnavailable {
        /// Capability identifier from the applicable capability registry.
        capability: String,
    },

    /// A resource or explicit operational budget was exceeded.
    ResourceLimitExceeded {
        /// Resource identifier, such as `memory.bytes` or `work.units`.
        resource: String,

        /// Requested amount, if known.
        requested: Option<u64>,

        /// Available or configured amount, if known.
        available: Option<u64>,
    },

    /// Required execution backend is unavailable.
    BackendUnavailable {
        /// Backend or service identifier.
        backend: String,

        /// Explanation of the failure.
        message: String,
    },

    /// Execution failed after the request was accepted.
    ExecutionFailed {
        /// Explanation of the execution failure.
        message: String,
    },

    /// An operation exceeded its applicable time budget.
    Timeout {
        /// Operation that timed out.
        operation: String,
    },

    /// An operation was cancelled.
    Cancelled {
        /// Explanation or cancellation origin.
        reason: String,
    },

    /// A determinism or reproducibility requirement was violated.
    DeterminismViolation {
        /// Explanation of the violated guarantee.
        message: String,
    },

    /// Serialization or deserialization failed.
    SerializationFailure {
        /// Explanation of the serialization failure.
        message: String,
    },

    /// A version or compatibility requirement was not satisfied.
    VersionMismatch {
        /// Name of the schema, interface, or component.
        subject: String,

        /// Expected version or compatibility description.
        expected: String,

        /// Observed version or compatibility description.
        actual: String,
    },

    /// A foreign call or data-exchange boundary failed.
    InteroperabilityFailure {
        /// Interface or operation identifier.
        interface: String,

        /// Explanation of the failure.
        message: String,
    },

    /// An internal invariant was violated.
    InternalInvariantViolation {
        /// Explanation of the violated invariant.
        message: String,
    },

    /// Additional context around an existing failure.
    Context {
        /// Additional context from the calling boundary.
        context: String,

        /// Original failure.
        source: Box<ClassicalError>,
    },
}

impl ClassicalError {
    /// Constructs an invalid-input error.
    #[must_use]
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }

    /// Constructs an invalid-configuration error.
    #[must_use]
    pub fn invalid_configuration(message: impl Into<String>) -> Self {
        Self::InvalidConfiguration {
            message: message.into(),
        }
    }

    /// Constructs a validation failure.
    #[must_use]
    pub fn validation_failed(message: impl Into<String>) -> Self {
        Self::ValidationFailed {
            message: message.into(),
        }
    }

    /// Constructs a type error.
    #[must_use]
    pub fn type_error(message: impl Into<String>) -> Self {
        Self::TypeError {
            message: message.into(),
        }
    }

    /// Constructs an arithmetic error.
    #[must_use]
    pub fn arithmetic(
        operation: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::ArithmeticError {
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Constructs a resource-budget failure.
    ///
    /// Unknown quantities are represented by `None`; they are never
    /// fabricated from estimates or host assumptions.
    #[must_use]
    pub fn resource_limit(
        resource: impl Into<String>,
        requested: Option<u64>,
        available: Option<u64>,
    ) -> Self {
        Self::ResourceLimitExceeded {
            resource: resource.into(),
            requested,
            available,
        }
    }

    /// Constructs an unsupported-operation error.
    #[must_use]
    pub fn unsupported_operation(
        operation: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::UnsupportedOperation {
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Adds context while preserving the underlying failure.
    #[must_use]
    pub fn with_context(self, context: impl Into<String>) -> Self {
        Self::Context {
            context: context.into(),
            source: Box::new(self),
        }
    }

    /// Returns the underlying non-context error kind.
    ///
    /// This makes contextual wrapping transparent for programmatic handling.
    #[must_use]
    pub fn kind(&self) -> ClassicalErrorKind {
        match self {
            Self::InvalidInput { .. } => ClassicalErrorKind::InvalidInput,
            Self::InvalidConfiguration { .. } => {
                ClassicalErrorKind::InvalidConfiguration
            }
            Self::ValidationFailed { .. } => ClassicalErrorKind::ValidationFailed,
            Self::TypeError { .. } => ClassicalErrorKind::TypeError,
            Self::RepresentationFailure { .. } => {
                ClassicalErrorKind::RepresentationFailure
            }
            Self::ArithmeticError { .. } => ClassicalErrorKind::ArithmeticError,
            Self::NumericalError { .. } => ClassicalErrorKind::NumericalError,
            Self::UnsupportedOperation { .. } => {
                ClassicalErrorKind::UnsupportedOperation
            }
            Self::CapabilityUnavailable { .. } => {
                ClassicalErrorKind::CapabilityUnavailable
            }
            Self::ResourceLimitExceeded { .. } => {
                ClassicalErrorKind::ResourceLimitExceeded
            }
            Self::BackendUnavailable { .. } => {
                ClassicalErrorKind::BackendUnavailable
            }
            Self::ExecutionFailed { .. } => ClassicalErrorKind::ExecutionFailed,
            Self::Timeout { .. } => ClassicalErrorKind::Timeout,
            Self::Cancelled { .. } => ClassicalErrorKind::Cancelled,
            Self::DeterminismViolation { .. } => {
                ClassicalErrorKind::DeterminismViolation
            }
            Self::SerializationFailure { .. } => {
                ClassicalErrorKind::SerializationFailure
            }
            Self::VersionMismatch { .. } => ClassicalErrorKind::VersionMismatch,
            Self::InteroperabilityFailure { .. } => {
                ClassicalErrorKind::InteroperabilityFailure
            }
            Self::InternalInvariantViolation { .. } => {
                ClassicalErrorKind::InternalInvariantViolation
            }
            Self::Context { source, .. } => source.kind(),
        }
    }

    /// Returns the stable machine-readable code.
    #[must_use]
    pub fn code(&self) -> &'static str {
        self.kind().as_str()
    }

    /// Returns the severity associated with this failure.
    #[must_use]
    pub fn severity(&self) -> ClassicalErrorSeverity {
        match self.kind() {
            ClassicalErrorKind::InvalidInput => ClassicalErrorSeverity::Input,

            ClassicalErrorKind::InvalidConfiguration
            | ClassicalErrorKind::VersionMismatch => {
                ClassicalErrorSeverity::Configuration
            }

            ClassicalErrorKind::ValidationFailed
            | ClassicalErrorKind::TypeError
            | ClassicalErrorKind::RepresentationFailure
            | ClassicalErrorKind::UnsupportedOperation
            | ClassicalErrorKind::CapabilityUnavailable => {
                ClassicalErrorSeverity::Validation
            }

            ClassicalErrorKind::ArithmeticError
            | ClassicalErrorKind::NumericalError
            | ClassicalErrorKind::DeterminismViolation => {
                ClassicalErrorSeverity::Numerical
            }

            ClassicalErrorKind::ResourceLimitExceeded
            | ClassicalErrorKind::Timeout
            | ClassicalErrorKind::Cancelled => ClassicalErrorSeverity::Resource,

            ClassicalErrorKind::BackendUnavailable
            | ClassicalErrorKind::ExecutionFailed
            | ClassicalErrorKind::SerializationFailure
            | ClassicalErrorKind::InteroperabilityFailure => {
                ClassicalErrorSeverity::Operational
            }

            ClassicalErrorKind::InternalInvariantViolation => {
                ClassicalErrorSeverity::Internal
            }

            // `kind()` unwraps context; this branch is defensive if the
            // classification model is extended in the future.
            ClassicalErrorKind::Context => ClassicalErrorSeverity::Internal,
        }
    }

    /// Returns whether retrying the same operation without changing its
    /// inputs, configuration, or environment is generally meaningful.
    ///
    /// This is a conservative classification, not an automatic retry command.
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        match self.kind() {
            ClassicalErrorKind::ResourceLimitExceeded
            | ClassicalErrorKind::BackendUnavailable
            | ClassicalErrorKind::Timeout
            | ClassicalErrorKind::ExecutionFailed => true,

            ClassicalErrorKind::InvalidInput
            | ClassicalErrorKind::InvalidConfiguration
            | ClassicalErrorKind::ValidationFailed
            | ClassicalErrorKind::TypeError
            | ClassicalErrorKind::RepresentationFailure
            | ClassicalErrorKind::ArithmeticError
            | ClassicalErrorKind::NumericalError
            | ClassicalErrorKind::UnsupportedOperation
            | ClassicalErrorKind::CapabilityUnavailable
            | ClassicalErrorKind::Cancelled
            | ClassicalErrorKind::DeterminismViolation
            | ClassicalErrorKind::SerializationFailure
            | ClassicalErrorKind::VersionMismatch
            | ClassicalErrorKind::InteroperabilityFailure
            | ClassicalErrorKind::InternalInvariantViolation
            | ClassicalErrorKind::Context => false,
        }
    }
}

impl fmt::Display for ClassicalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput { message } => {
                write!(formatter, "invalid input: {message}")
            }
            Self::InvalidConfiguration { message } => {
                write!(formatter, "invalid configuration: {message}")
            }
            Self::ValidationFailed { message } => {
                write!(formatter, "validation failed: {message}")
            }
            Self::TypeError { message } => {
                write!(formatter, "type error: {message}")
            }
            Self::RepresentationFailure { message } => {
                write!(formatter, "representation failure: {message}")
            }
            Self::ArithmeticError { operation, message } => {
                write!(formatter, "arithmetic error in {operation}: {message}")
            }
            Self::NumericalError { message } => {
                write!(formatter, "numerical error: {message}")
            }
            Self::UnsupportedOperation { operation, message } => {
                write!(formatter, "unsupported operation {operation}: {message}")
            }
            Self::CapabilityUnavailable { capability } => {
                write!(formatter, "required capability unavailable: {capability}")
            }
            Self::ResourceLimitExceeded {
                resource,
                requested,
                available,
            } => {
                write!(formatter, "resource limit exceeded for {resource}")?;

                if let Some(requested) = requested {
                    write!(formatter, "; requested {requested}")?;
                }

                if let Some(available) = available {
                    write!(formatter, "; available {available}")?;
                }

                Ok(())
            }
            Self::BackendUnavailable { backend, message } => {
                write!(formatter, "backend {backend} unavailable: {message}")
            }
            Self::ExecutionFailed { message } => {
                write!(formatter, "execution failed: {message}")
            }
            Self::Timeout { operation } => {
                write!(formatter, "operation timed out: {operation}")
            }
            Self::Cancelled { reason } => {
                write!(formatter, "operation cancelled: {reason}")
            }
            Self::DeterminismViolation { message } => {
                write!(formatter, "determinism violation: {message}")
            }
            Self::SerializationFailure { message } => {
                write!(formatter, "serialization failure: {message}")
            }
            Self::VersionMismatch {
                subject,
                expected,
                actual,
            } => write!(
                formatter,
                "version mismatch for {subject}: expected {expected}, found {actual}"
            ),
            Self::InteroperabilityFailure { interface, message } => {
                write!(formatter, "interoperability failure at {interface}: {message}")
            }
            Self::InternalInvariantViolation { message } => {
                write!(formatter, "internal invariant violated: {message}")
            }
            Self::Context { context, source } => {
                write!(formatter, "{context}: {source}")
            }
        }
    }
}

impl Error for ClassicalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Context { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes_are_stable_and_machine_readable() {
        assert_eq!(
            ClassicalError::invalid_input("bad value").code(),
            "classical.invalid_input"
        );

        assert_eq!(
            ClassicalError::resource_limit("memory.bytes", Some(20), Some(10)).code(),
            "classical.resource_limit_exceeded"
        );
    }

    #[test]
    fn context_preserves_original_kind_and_code() {
        let error = ClassicalError::invalid_input("bad input")
            .with_context("while validating a function argument");

        assert_eq!(error.kind(), ClassicalErrorKind::InvalidInput);
        assert_eq!(error.code(), "classical.invalid_input");
        assert!(error.to_string().contains("while validating"));
    }

    #[test]
    fn context_preserves_source_chain() {
        let error = ClassicalError::invalid_configuration("invalid budget")
            .with_context("while preparing compilation");

        assert!(Error::source(&error).is_some());
    }

    #[test]
    fn resource_error_does_not_invent_unknown_quantities() {
        let error = ClassicalError::resource_limit("memory.bytes", None, None);

        assert_eq!(
            error,
            ClassicalError::ResourceLimitExceeded {
                resource: "memory.bytes".to_owned(),
                requested: None,
                available: None,
            }
        );
    }

    #[test]
    fn severity_is_independent_of_display_text() {
        let error = ClassicalError::invalid_configuration("anything");

        assert_eq!(error.severity(), ClassicalErrorSeverity::Configuration);
        assert_eq!(error.kind(), ClassicalErrorKind::InvalidConfiguration);
    }

    #[test]
    fn contextual_wrapping_does_not_change_retry_classification() {
        let error = ClassicalError::resource_limit(
            "memory.bytes",
            Some(200),
            Some(100),
        )
        .with_context("while evaluating an expression");

        assert!(error.is_retryable());
    }

    #[test]
    fn invalid_input_is_not_automatically_retryable() {
        let error = ClassicalError::invalid_input("malformed input");

        assert!(!error.is_retryable());
    }

    #[test]
    fn display_is_human_readable() {
        let error = ClassicalError::arithmetic("division", "division by zero");

        assert!(error.to_string().contains("division"));
        assert!(error.to_string().contains("division by zero"));
    }

    #[test]
    fn error_kinds_have_nonempty_codes() {
        let kinds = [
            ClassicalErrorKind::InvalidInput,
            ClassicalErrorKind::InvalidConfiguration,
            ClassicalErrorKind::ValidationFailed,
            ClassicalErrorKind::TypeError,
            ClassicalErrorKind::RepresentationFailure,
            ClassicalErrorKind::ArithmeticError,
            ClassicalErrorKind::NumericalError,
            ClassicalErrorKind::UnsupportedOperation,
            ClassicalErrorKind::CapabilityUnavailable,
            ClassicalErrorKind::ResourceLimitExceeded,
            ClassicalErrorKind::BackendUnavailable,
            ClassicalErrorKind::ExecutionFailed,
            ClassicalErrorKind::Timeout,
            ClassicalErrorKind::Cancelled,
            ClassicalErrorKind::DeterminismViolation,
            ClassicalErrorKind::SerializationFailure,
            ClassicalErrorKind::VersionMismatch,
            ClassicalErrorKind::InteroperabilityFailure,
            ClassicalErrorKind::InternalInvariantViolation,
        ];

        for kind in kinds {
            assert!(!kind.as_str().is_empty());
            assert!(kind.as_str().starts_with("classical."));
        }
    }
}