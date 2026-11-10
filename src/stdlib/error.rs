
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Standard Library — Structured Error Infrastructure
//!
//! Path: `src/stdlib/error.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide a stable, target-independent error contract for operations
//! exposed by the Zamani standard library.
//!
//! The error infrastructure must work across:
//!
//! - classical computation;
//! - quantum computation;
//! - HDL and hardware-description operations;
//! - hybrid computation;
//! - distributed computation;
//! - AI and machine learning;
//! - scientific computation;
//! - networking;
//! - storage and filesystem operations;
//! - cryptography and security;
//! - concurrency and asynchronous operations;
//! - future computational domains.
//!
//! New domains must be able to introduce error codes without requiring
//! changes to this module's built-in error classification.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! This module owns standard-library operation errors.
//!
//! It does not own compiler diagnostics, semantic diagnostics,
//! source locations, diagnostic rendering, runtime scheduling,
//! target feasibility, or hardware allocation.
//!
//! Canonical compiler diagnostic authority:
//!
//! `crate::frontend::semantic::diagnostics`
//!
//! Compiler diagnostic integration:
//!
//! `crate::compiler::diagnostics`
//!
//! Canonical source-location authority:
//!
//! `crate::frontend::ast::node::source::span`
//!
//! Version identity:
//!
//! `crate::stdlib::version`
//!
//! Normative diagnostic specification:
//!
//! `grammar/spec/diagnostics.md`
//!
//! Normative source-span specification:
//!
//! `grammar/spec/source-spans.md`
//!
//! Canonical grammar root:
//!
//! `grammar/Zamani.g4`
//!
//! ## OWNS
//!
//! - Standard-library error classification.
//! - Extensible error identifiers.
//! - Structured operation errors.
//! - Error context.
//! - Error causality.
//! - Retryability classification.
//! - Standard-library Result alias.
//! - Error display and standard Rust error integration.
//!
//! ## DOES NOT OWN
//!
//! - Compiler diagnostic severity or diagnostic codes.
//! - Source spans, source maps, or source labels.
//! - Diagnostic ordering or deduplication.
//! - Diagnostic rendering or serialization.
//! - Semantic validity.
//! - Capability discovery.
//! - Feasibility assessment.
//! - Resource allocation.
//! - Hardware selection.
//! - Scheduling or routing.
//! - Backend execution.
//! - Runtime lifecycle.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ## SCALABILITY
//!
//! No fixed error-count ceiling is introduced.
//!
//! No fixed error-context count ceiling is introduced.
//!
//! No fixed error-message length ceiling is introduced.
//!
//! No hardware, processor, device, node, qubit, memory, register,
//! tensor, or computational-domain ceiling is introduced.
//!
//! Collections grow subject to available memory and the operational
//! policies of the caller.
//!
//! This does not promise literally infinite computation or memory.
//!
//! ## ERROR DESIGN
//!
//! ErrorKind describes a broad failure category.
//!
//! ErrorCode identifies a specific failure contract.
//!
//! ErrorContext adds structured information about the operation.
//!
//! The underlying source error preserves causality.
//!
//! These concepts must not be conflated.
//!
//! ## INTEGRATION
//!
//! Other standard-library modules should return:
//!
//! `crate::stdlib::error::Result<T>`
//!
//! Domain-specific modules should use the common Error type while
//! defining their own extensible ErrorCode identifiers.
//!
//! Compiler and runtime adapters may translate these errors into
//! their respective canonical diagnostic or execution-error records.
//!
//! Such adapters must preserve available error identity, message,
//! context, retryability, and causality.
//!
//! ## SAFETY
//!
//! Safe Rust only.
//!
//! No global mutable error registry.
//!
//! No implicit logging.
//!
//! No printing to stdout or stderr.
//!
//! No hidden recovery or retry behavior.
//!
//! No panics for ordinary error construction or propagation.

// ============================================================================
// ERROR CLASSIFICATION
// ============================================================================

/// Broad classification of a standard-library operation failure.
///
/// This classification is intentionally domain-independent.
///
/// Specific operations should use [`ErrorCode`] to identify their
/// individual error contracts.
///
/// Adding a new computational domain should not require adding a
/// corresponding variant to this enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// An argument, value, or operation input is invalid.
    InvalidInput,

    /// An operation received data that violates its expected format.
    InvalidData,

    /// A requested operation or functionality is not implemented
    /// by the selected implementation.
    NotImplemented,

    /// An operation is not supported by the selected implementation
    /// or its declared contract.
    Unsupported,

    /// A required capability is unavailable in the current scope.
    ///
    /// This does not establish that the capability is universally
    /// unsupported.
    CapabilityUnavailable,

    /// The operation cannot currently be performed with the
    /// available resources or conditions.
    Infeasible,

    /// A resource required by an operation is unavailable.
    ResourceUnavailable,

    /// An explicitly configured operational budget was exhausted.
    ///
    /// This is not a language-level capability restriction.
    BudgetExhausted,

    /// An operation attempted to access a resource it is not
    /// authorized to use.
    PermissionDenied,

    /// An operation could not locate a requested entity.
    NotFound,

    /// An operation encountered an entity that already exists
    /// where a new entity was required.
    AlreadyExists,

    /// An operation encountered an incompatible version or schema.
    VersionMismatch,

    /// An operation encountered an incompatible contract.
    ContractViolation,

    /// An operation failed because of an underlying I/O failure.
    Io,

    /// An operation failed because of a communication failure.
    Communication,

    /// An operation failed because of a serialization or
    /// deserialization failure.
    Serialization,

    /// An operation failed because of a cryptographic failure.
    Cryptographic,

    /// An operation failed because a required provider failed.
    Provider,

    /// An operation failed because a required dependency failed.
    Dependency,

    /// An operation was cancelled.
    Cancelled,

    /// An operation exceeded its applicable time allowance.
    TimedOut,

    /// An operation encountered a concurrency conflict.
    Concurrency,

    /// An operation failed because of an internal implementation
    /// invariant or unexpected internal condition.
    Internal,

    /// The operation failed for a reason not represented by
    /// another built-in classification.
    Other,
}

impl ErrorKind {
    /// Returns the stable textual name of this classification.
    ///
    /// These names are intended for reporting and interoperability.
    /// They are not diagnostic codes.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInput => "invalid_input",
            Self::InvalidData => "invalid_data",
            Self::NotImplemented => "not_implemented",
            Self::Unsupported => "unsupported",
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
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::Concurrency => "concurrency",
            Self::Internal => "internal",
            Self::Other => "other",
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// EXTENSIBLE ERROR IDENTIFIERS
// ============================================================================

/// Validation failure for an error identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCodeValidationError {
    /// The identifier is empty.
    Empty,

    /// An identifier segment is empty.
    EmptySegment,

    /// An identifier contains a character outside the permitted set.
    InvalidCharacter,

    /// An identifier contains an invalid segment boundary.
    InvalidBoundary,
}

impl std::fmt::Display for ErrorCodeValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let message = match self {
            Self::Empty => "error code must not be empty",
            Self::EmptySegment => "error code contains an empty segment",
            Self::InvalidCharacter => {
                "error code contains an invalid character"
            }
            Self::InvalidBoundary => {
                "error code has an invalid segment boundary"
            }
        };

        formatter.write_str(message)
    }
}

impl std::error::Error for ErrorCodeValidationError {}

/// A validated, extensible standard-library error identifier.
///
/// Error codes are independent of [`ErrorKind`].
///
/// This permits modules and third-party providers to define new
/// error identifiers without modifying this module.
///
/// Identifiers use lowercase ASCII segments separated by dots.
///
/// Permitted segment characters:
///
/// - lowercase ASCII letters;
/// - decimal digits;
/// - hyphens;
/// - underscores.
///
/// Examples:
///
/// - `stdlib.io.read_failed`
/// - `stdlib.quantum.provider_unavailable`
/// - `stdlib.distributed.connection_lost`
/// - `stdlib.crypto.invalid_signature`
/// - `provider.vendor_specific.failure`
///
/// No arbitrary identifier-length ceiling is imposed.
///
/// Actual allocation remains subject to available memory.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorCode(String);

impl ErrorCode {
    /// Constructs a validated error code.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorCodeValidationError`] if the identifier is
    /// empty or contains invalid syntax.
    pub fn new(
        value: impl Into<String>,
    ) -> std::result::Result<Self, ErrorCodeValidationError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ErrorCodeValidationError::Empty);
        }

        let bytes = value.as_bytes();

        if bytes.first() == Some(&b'.')
            || bytes.last() == Some(&b'.')
        {
            return Err(ErrorCodeValidationError::InvalidBoundary);
        }

        let mut previous_was_separator = false;

        for byte in bytes {
            if *byte == b'.' {
                if previous_was_separator {
                    return Err(ErrorCodeValidationError::EmptySegment);
                }

                previous_was_separator = true;
                continue;
            }

            let valid = byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || *byte == b'-'
                || *byte == b'_';

            if !valid {
                return Err(ErrorCodeValidationError::InvalidCharacter);
            }

            previous_was_separator = false;
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::str::FromStr for ErrorCode {
    type Err = ErrorCodeValidationError;

    fn from_str(
        value: &str,
    ) -> std::result::Result<Self, Self::Err> {
        Self::new(value)
    }
}

// ============================================================================
// RETRYABILITY
// ============================================================================

/// Describes whether an operation may be retried.
///
/// Retryability is descriptive metadata.
///
/// This module does not automatically retry operations.
///
/// A caller must still consider operation idempotency, side effects,
/// cancellation, transaction state, and the relevant provider contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Retryability {
    /// The error does not provide enough information to determine
    /// whether retrying is appropriate.
    Unknown,

    /// Retrying without changing the relevant conditions is not
    /// expected to succeed.
    NotRetryable,

    /// Retrying may succeed after relevant conditions change.
    RetryableAfterConditionChange,

    /// Retrying may succeed without an explicit condition change.
    Retryable,

    /// Retrying is safe under the operation's documented contract.
    ///
    /// This classification does not independently prove that an
    /// operation is idempotent.
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

impl std::fmt::Display for Retryability {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// ERROR CONTEXT
// ============================================================================

/// A structured key/value item providing additional error context.
///
/// Context values are descriptive data, not executable instructions.
///
/// Do not place passwords, private keys, authentication tokens,
/// secret material, or other sensitive values in error context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorContext {
    key: String,
    value: String,
}

impl ErrorContext {
    /// Creates an error-context entry.
    ///
    /// Empty keys are rejected.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorContextValidationError::EmptyKey`] when the key
    /// is empty.
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> std::result::Result<Self, ErrorContextValidationError> {
        let key = key.into();

        if key.is_empty() {
            return Err(ErrorContextValidationError::EmptyKey);
        }

        Ok(Self {
            key,
            value: value.into(),
        })
    }

    /// Returns the context key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the context value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Validation failures for error context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorContextValidationError {
    /// The context key is empty.
    EmptyKey,
}

impl std::fmt::Display for ErrorContextValidationError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::EmptyKey => {
                formatter.write_str("error context key must not be empty")
            }
        }
    }
}

impl std::error::Error for ErrorContextValidationError {}

// ============================================================================
// STANDARD-LIBRARY ERROR
// ============================================================================

/// Standard-library operation error.
///
/// This type preserves:
///
/// - a broad error classification;
/// - an optional extensible error identifier;
/// - a human-readable message;
/// - optional component identity;
/// - optional operation identity;
/// - retryability information;
/// - structured context;
/// - the original underlying error, when available.
///
/// The error is independent of any particular computational domain.
///
/// It is not a compiler diagnostic and does not contain a source span.
///
/// # Error propagation
///
/// Use [`Error::with_source`] when wrapping an underlying error.
/// This preserves the causal chain through [`std::error::Error::source`].
///
/// # Cloning
///
/// This type intentionally does not implement `Clone`.
///
/// An underlying error may not be cloneable. Callers that need to
/// retain an error should own it or construct a separate reporting
/// representation through an explicit adapter.
#[derive(Debug)]
#[must_use]
pub struct Error {
    kind: ErrorKind,
    code: Option<ErrorCode>,
    message: String,
    component: Option<String>,
    operation: Option<String>,
    retryability: Retryability,
    context: Vec<ErrorContext>,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl Error {
    /// Creates a standard-library error.
    ///
    /// The error starts with unknown retryability and no additional
    /// context or underlying source.
    #[must_use]
    pub fn new(
        kind: ErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            code: None,
            message: message.into(),
            component: None,
            operation: None,
            retryability: Retryability::Unknown,
            context: Vec::new(),
            source: None,
        }
    }

    /// Creates an error with an explicit extensible error code.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorCodeValidationError`] when the supplied code
    /// is invalid.
    pub fn with_code(
        kind: ErrorKind,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> std::result::Result<Self, ErrorCodeValidationError> {
        let code = ErrorCode::new(code)?;

        let mut error = Self::new(kind, message);
        error.code = Some(code);

        Ok(error)
    }

    /// Returns the broad error classification.
    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Returns the optional specific error identifier.
    #[must_use]
    pub fn code(&self) -> Option<&ErrorCode> {
        self.code.as_ref()
    }

    /// Returns the human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the optional component identity.
    #[must_use]
    pub fn component(&self) -> Option<&str> {
        self.component.as_deref()
    }

    /// Returns the optional operation identity.
    #[must_use]
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Returns the retryability classification.
    #[must_use]
    pub const fn retryability(&self) -> Retryability {
        self.retryability
    }

    /// Returns the structured context entries.
    #[must_use]
    pub fn context(&self) -> &[ErrorContext] {
        &self.context
    }

    /// Sets the component associated with this error.
    ///
    /// The component is descriptive metadata. It does not establish
    /// that the named component is registered or available.
    #[must_use]
    pub fn in_component(
        mut self,
        component: impl Into<String>,
    ) -> Self {
        self.component = Some(component.into());
        self
    }

    /// Sets the operation associated with this error.
    #[must_use]
    pub fn during_operation(
        mut self,
        operation: impl Into<String>,
    ) -> Self {
        self.operation = Some(operation.into());
        self
    }

    /// Sets the retryability classification.
    ///
    /// No automatic retry is performed.
    #[must_use]
    pub const fn with_retryability(
        mut self,
        retryability: Retryability,
    ) -> Self {
        self.retryability = retryability;
        self
    }

    /// Adds a structured context entry.
    ///
    /// Context entries are appended in insertion order.
    ///
    /// No fixed number of entries is permitted by this API.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorContextValidationError`] if the key is empty.
    pub fn add_context(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> std::result::Result<(), ErrorContextValidationError> {
        self.context.push(ErrorContext::new(key, value)?);
        Ok(())
    }

    /// Adds context and returns the error for builder-style use.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorContextValidationError`] if the key is empty.
    pub fn with_context(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> std::result::Result<Self, ErrorContextValidationError> {
        self.add_context(key, value)?;
        Ok(self)
    }

    /// Attaches an underlying causal error.
    ///
    /// The original error remains available through
    /// [`std::error::Error::source`].
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Attaches an already boxed causal error.
    #[must_use]
    pub fn with_boxed_source(
        mut self,
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    ) -> Self {
        self.source = Some(source);
        self
    }

    /// Returns whether an underlying causal error exists.
    #[must_use]
    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    /// Returns the error's stable classification name.
    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        self.kind.as_str()
    }

    /// Returns the optional stable error-code string.
    #[must_use]
    pub fn code_str(&self) -> Option<&str> {
        self.code.as_ref().map(ErrorCode::as_str)
    }

    /// Returns a human-readable description including available
    /// classification and code information.
    ///
    /// This method does not expose the underlying source chain.
    /// Use the standard error source interface for that purpose.
    #[must_use]
    pub fn description(&self) -> String {
        match self.code() {
            Some(code) => {
                format!("{} [{}]: {}", self.kind, code, self.message)
            }
            None => {
                format!("{}: {}", self.kind, self.message)
            }
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(formatter, "{}", self.description())?;

        if let Some(component) = &self.component {
            write!(formatter, " (component: {component})")?;
        }

        if let Some(operation) = &self.operation {
            write!(formatter, " (operation: {operation})")?;
        }

        Ok(())
    }
}

impl std::error::Error for Error {
    fn source(
        &self,
    ) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn std::error::Error + 'static))
    }
}

// ============================================================================
// STANDARD RESULT TYPE
// ============================================================================

/// Standard result type for Zamani standard-library operations.
///
/// Standard-library modules should use this alias for operations
/// that can fail.
///
/// This alias does not replace `std::result::Result` in APIs where
/// a different error type is intentionally required.
pub type Result<T> = std::result::Result<T, Error>;

// ============================================================================
// CONSTRUCTION HELPERS
// ============================================================================

/// Creates an invalid-input error.
///
/// This convenience function does not validate the input itself.
#[must_use]
pub fn invalid_input(
    message: impl Into<String>,
) -> Error {
    Error::new(ErrorKind::InvalidInput, message)
}

/// Creates an invalid-data error.
#[must_use]
pub fn invalid_data(
    message: impl Into<String>,
) -> Error {
    Error::new(ErrorKind::InvalidData, message)
}

/// Creates an unsupported-operation error.
#[must_use]
pub fn unsupported(
    message: impl Into<String>,
) -> Error {
    Error::new(ErrorKind::Unsupported, message)
}

/// Creates a capability-unavailable error.
///
/// This means that the required capability is unavailable in the
/// current context. It does not prove that the capability is
/// universally unsupported.
#[must_use]
pub fn capability_unavailable(
    message: impl Into<String>,
) -> Error {
    Error::new(ErrorKind::CapabilityUnavailable, message)
}

/// Creates an infeasibility error.
///
/// Feasibility must be established by the responsible semantic,
/// planning, or runtime authority. This helper does not perform
/// feasibility analysis.
#[must_use]
pub fn infeasible(
    message: impl Into<String>,
) -> Error {
    Error::new(ErrorKind::Infeasible, message)
}

/// Creates a resource-unavailable error.
#[must_use]
pub fn resource_unavailable(
    message: impl Into<String>,
) -> Error {
    Error::new(ErrorKind::ResourceUnavailable, message)
}

/// Creates an operational-budget-exhaustion error.
///
/// A budget is an operational policy, not a language-level
/// hardware or computational capability ceiling.
#[must_use]
pub fn budget_exhausted(
    message: impl Into<String>,
) -> Error {
    Error::new(ErrorKind::BudgetExhausted, message)
}

/// Creates a cancellation error.
#[must_use]
pub fn cancelled(
    message: impl Into<String>,
) -> Error {
    Error::new(ErrorKind::Cancelled, message)
}

/// Creates a timeout error.
#[must_use]
pub fn timed_out(
    message: impl Into<String>,
) -> Error {
    Error::new(ErrorKind::TimedOut, message)
}

/// Creates an internal implementation error.
#[must_use]
pub fn internal(
    message: impl Into<String>,
) -> Error {
    Error::new(ErrorKind::Internal, message)
}

// ============================================================================
// STANDARD CONVERSIONS
// ============================================================================

/// Converts an ordinary Rust error into a standard-library error
/// while preserving its causal identity.
///
/// The caller must supply the broad classification and message
/// appropriate to the operation.
///
/// This conversion does not infer semantic meaning from the
/// underlying error's display text.
pub fn from_source<E>(
    kind: ErrorKind,
    message: impl Into<String>,
    source: E,
) -> Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    Error::new(kind, message).with_source(source)
}

/// Converts a standard-library error into a boxed standard Rust
/// error for APIs that require a trait object.
///
/// The original error remains the boxed error.
#[must_use]
pub fn into_boxed_error(
    error: Error,
) -> Box<dyn std::error::Error + Send + Sync + 'static> {
    Box::new(error)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_kind_names_are_stable() {
        assert_eq!(
            ErrorKind::InvalidInput.as_str(),
            "invalid_input"
        );

        assert_eq!(
            ErrorKind::CapabilityUnavailable.as_str(),
            "capability_unavailable"
        );

        assert_eq!(
            ErrorKind::BudgetExhausted.as_str(),
            "budget_exhausted"
        );
    }

    #[test]
    fn error_code_accepts_extensible_namespaces() {
        let code = ErrorCode::new(
            "stdlib.quantum.provider_unavailable"
        )
        .expect("valid error code");

        assert_eq!(
            code.as_str(),
            "stdlib.quantum.provider_unavailable"
        );
    }

    #[test]
    fn error_code_rejects_empty_identifiers() {
        assert_eq!(
            ErrorCode::new(""),
            Err(ErrorCodeValidationError::Empty)
        );
    }

    #[test]
    fn error_code_rejects_empty_segments() {
        assert_eq!(
            ErrorCode::new("stdlib..io"),
            Err(ErrorCodeValidationError::EmptySegment)
        );
    }

    #[test]
    fn error_code_rejects_invalid_characters() {
        assert_eq!(
            ErrorCode::new("stdlib.IO.failure"),
            Err(ErrorCodeValidationError::InvalidCharacter)
        );
    }

    #[test]
    fn error_code_rejects_invalid_boundaries() {
        assert_eq!(
            ErrorCode::new(".stdlib.io"),
            Err(ErrorCodeValidationError::InvalidBoundary)
        );

        assert_eq!(
            ErrorCode::new("stdlib.io."),
            Err(ErrorCodeValidationError::InvalidBoundary)
        );
    }

    #[test]
    fn error_code_supports_standard_string_parsing() {
        let code: ErrorCode = "stdlib.io.read_failed"
            .parse()
            .expect("valid error code");

        assert_eq!(code.as_str(), "stdlib.io.read_failed");
    }

    #[test]
    fn error_preserves_its_classification_and_message() {
        let error = Error::new(
            ErrorKind::InvalidInput,
            "invalid matrix dimensions",
        );

        assert_eq!(error.kind(), ErrorKind::InvalidInput);
        assert_eq!(error.message(), "invalid matrix dimensions");
        assert_eq!(error.kind_name(), "invalid_input");
    }

    #[test]
    fn error_preserves_specific_code() {
        let error = Error::with_code(
            ErrorKind::Provider,
            "stdlib.quantum.provider_unavailable",
            "no compatible provider is available",
        )
        .expect("valid error code");

        assert_eq!(
            error.code_str(),
            Some("stdlib.quantum.provider_unavailable")
        );
    }

    #[test]
    fn error_preserves_component_and_operation() {
        let error = Error::new(
            ErrorKind::Io,
            "read operation failed",
        )
        .in_component("stdlib.io")
        .during_operation("read");

        assert_eq!(error.component(), Some("stdlib.io"));
        assert_eq!(error.operation(), Some("read"));
    }

    #[test]
    fn error_preserves_retryability() {
        let error = Error::new(
            ErrorKind::Communication,
            "connection interrupted",
        )
        .with_retryability(Retryability::Retryable);

        assert_eq!(
            error.retryability(),
            Retryability::Retryable
        );
    }

    #[test]
    fn error_context_is_preserved_in_insertion_order() {
        let mut error = Error::new(
            ErrorKind::Provider,
            "provider operation failed",
        );

        error
            .add_context("provider", "example")
            .expect("valid context");

        error
            .add_context("operation", "execute")
            .expect("valid context");

        assert_eq!(error.context().len(), 2);
        assert_eq!(error.context()[0].key(), "provider");
        assert_eq!(error.context()[1].key(), "operation");
    }

    #[test]
    fn empty_context_keys_are_rejected() {
        let mut error = Error::new(
            ErrorKind::Internal,
            "failure",
        );

        assert_eq!(
            error.add_context("", "value"),
            Err(ErrorContextValidationError::EmptyKey)
        );
    }

    #[test]
    fn error_context_builder_preserves_entries() {
        let error = Error::new(
            ErrorKind::InvalidData,
            "invalid input",
        )
        .with_context("format", "unknown")
        .expect("valid context");

        assert_eq!(error.context().len(), 1);
        assert_eq!(error.context()[0].value(), "unknown");
    }

    #[test]
    fn error_preserves_underlying_cause() {
        let source = std::io::Error::other("underlying failure");

        let error = Error::new(
            ErrorKind::Io,
            "filesystem operation failed",
        )
        .with_source(source);

        assert!(error.has_source());

        assert_eq!(
            error.source().map(std::string::ToString::to_string),
            Some("underlying failure".to_owned())
        );
    }

    #[test]
    fn error_display_contains_available_identity() {
        let error = Error::with_code(
            ErrorKind::Io,
            "stdlib.io.read_failed",
            "could not read input",
        )
        .expect("valid error code")
        .in_component("stdlib.io")
        .during_operation("read");

        let display = error.to_string();

        assert!(display.contains("io"));
        assert!(display.contains("stdlib.io.read_failed"));
        assert!(display.contains("could not read input"));
        assert!(display.contains("stdlib.io"));
        assert!(display.contains("read"));
    }

    #[test]
    fn helper_functions_use_expected_classifications() {
        assert_eq!(
            invalid_input("bad").kind(),
            ErrorKind::InvalidInput
        );

        assert_eq!(
            unsupported("missing implementation").kind(),
            ErrorKind::Unsupported
        );

        assert_eq!(
            capability_unavailable("not available").kind(),
            ErrorKind::CapabilityUnavailable
        );

        assert_eq!(
            infeasible("cannot satisfy requirements").kind(),
            ErrorKind::Infeasible
        );

        assert_eq!(
            budget_exhausted("budget reached").kind(),
            ErrorKind::BudgetExhausted
        );

        assert_eq!(
            cancelled("cancelled").kind(),
            ErrorKind::Cancelled
        );
    }

    #[test]
    fn result_alias_uses_standard_library_error() {
        fn operation() -> Result<()> {
            Err(invalid_input("invalid operation"))
        }

        let result = operation();

        assert!(result.is_err());
        assert_eq!(
            result.expect_err("operation must fail").kind(),
            ErrorKind::InvalidInput
        );
    }

    #[test]
    fn error_can_be_used_as_a_standard_error() {
        fn accepts_error(
            _: &(dyn std::error::Error + 'static),
        ) {
        }

        let error = invalid_input("invalid value");

        accepts_error(&error);
    }

    #[test]
    fn source_conversion_preserves_causality() {
        let source = std::io::Error::other("disk unavailable");

        let error = from_source(
            ErrorKind::Io,
            "storage operation failed",
            source,
        );

        assert!(error.has_source());
        assert_eq!(error.kind(), ErrorKind::Io);
    }

    #[test]
    fn boxed_conversion_preserves_standard_error_behavior() {
        let boxed = into_boxed_error(
            invalid_input("invalid input")
        );

        assert!(boxed.to_string().contains("invalid_input"));
    }

    #[test]
    fn error_codes_are_orderable_and_hashable() {
        use std::collections::BTreeSet;

        let mut codes = BTreeSet::new();

        codes.insert(
            ErrorCode::new("stdlib.io.read")
                .expect("valid code")
        );

        codes.insert(
            ErrorCode::new("stdlib.io.write")
                .expect("valid code")
        );

        assert_eq!(codes.len(), 2);
    }
}
