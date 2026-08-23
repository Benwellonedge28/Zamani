//! Zamani Quantum Frontend — canonical frontend error model.
//!
//! This module defines the format-independent error contract for the quantum
//! frontend boundary. It is deliberately independent of OpenQASM, QIR, Quil,
//! the Quantum IR implementation modules, and source-location storage.
//!
//! # Architectural boundary
//!
//! The frontend is an untrusted-input boundary. This module owns stable error
//! classification and machine-readable codes; `source.rs` owns source spans;
//! `diagnostics.rs` owns user-facing diagnostic composition.
//!
//! The Quantum IR remains the canonical semantic representation. Frontend
//! errors do not duplicate Quantum IR error types or quantum-domain models.
//!
//! # Error-code contract
//!
//! Format-independent codes use the `FE-*` namespace. Format-specific
//! implementations must use their own stable namespace, such as `QASM-*`, so
//! adding or removing a format never requires editing this file.
//!
//! Error codes are API identifiers. Their meanings must not be silently
//! changed; introduce a new code when the contract changes materially.
//!
//! # Rust compatibility
//!
//! This module targets Rust 1.97.1 and uses only the standard library.

use std::error::Error;
use std::fmt;

/// Canonical result type for public quantum frontend operations.
pub type FrontendResult<T> = Result<T, FrontendError>;

/// Stable high-level classification of a frontend failure.
///
/// OpenQASM, QIR, Quil, and future formats map their local failures into this
/// vocabulary while retaining format-specific error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FrontendErrorKind {
    /// Input could not be tokenized according to the selected format.
    Lexical,

    /// Token sequence does not satisfy the selected format's grammar.
    Syntax,

    /// Parsed source violates the selected format's semantic rules.
    Semantic,

    /// Requested format, version, or feature is not supported.
    Unsupported,

    /// Caller supplied invalid frontend configuration or input.
    InvalidInput,

    /// A configured frontend resource limit was exceeded.
    LimitExceeded,

    /// Importing an external representation failed.
    Import,

    /// Exporting the canonical IR failed.
    Export,

    /// A validated representation cannot be lowered to the canonical IR.
    Lowering,

    /// Diagnostic construction or rendering failed.
    Diagnostic,

    /// A frontend-internal invariant was violated.
    Internal,
}

impl fmt::Display for FrontendErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Lexical => "lexical",
            Self::Syntax => "syntax",
            Self::Semantic => "semantic",
            Self::Unsupported => "unsupported",
            Self::InvalidInput => "invalid_input",
            Self::LimitExceeded => "limit_exceeded",
            Self::Import => "import",
            Self::Export => "export",
            Self::Lowering => "lowering",
            Self::Diagnostic => "diagnostic",
            Self::Internal => "internal",
        };

        f.write_str(name)
    }
}

/// Stable machine-readable frontend error code.
///
/// This is intentionally not a central enum. Each independently removable
/// format can declare constants such as `QASM-E001` without modifying the
/// shared frontend layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrontendErrorCode(&'static str);

impl FrontendErrorCode {
    /// Generic invalid-input error.
    pub const INVALID_INPUT: Self = Self("FE-001");

    /// Generic unsupported-feature error.
    pub const UNSUPPORTED: Self = Self("FE-002");

    /// Generic frontend resource-limit error.
    pub const LIMIT_EXCEEDED: Self = Self("FE-003");

    /// Generic import error.
    pub const IMPORT: Self = Self("FE-004");

    /// Generic export error.
    pub const EXPORT: Self = Self("FE-005");

    /// Generic lowering error.
    pub const LOWERING: Self = Self("FE-006");

    /// Generic internal error.
    pub const INTERNAL: Self = Self("FE-007");

    /// Creates a stable subsystem- or format-specific code.
    ///
    /// For example:
    ///
    /// `FrontendErrorCode::new("QASM-E001")`
    ///
    /// The caller owns the stability contract for custom codes.
    pub const fn new(code: &'static str) -> Self {
        Self(code)
    }

    /// Returns the stable code string.
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl AsRef<str> for FrontendErrorCode {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl fmt::Display for FrontendErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// Deterministic structured context attached to a frontend error.
///
/// A sequence is used instead of a map so formatting never depends on hash
/// iteration order. Context should be appended in semantic order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontendErrorContext {
    key: String,
    value: String,
}

impl FrontendErrorContext {
    /// Creates a context item.
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Returns the context key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the context value.
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Structured frontend resource-limit violation.
///
/// This is intentionally separate from `QuantumIrLimits`: frontend limits
/// protect parsing and importing before a canonical IR object necessarily
/// exists, while IR limits protect the canonical representation itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontendLimitViolation {
    limit: &'static str,
    actual: usize,
    maximum: usize,
}

impl FrontendLimitViolation {
    /// Creates a resource-limit violation.
    pub const fn new(
        limit: &'static str,
        actual: usize,
        maximum: usize,
    ) -> Self {
        Self {
            limit,
            actual,
            maximum,
        }
    }

    /// Name of the violated limit.
    pub const fn limit(&self) -> &'static str {
        self.limit
    }

    /// Observed value.
    pub const fn actual(&self) -> usize {
        self.actual
    }

    /// Configured maximum.
    pub const fn maximum(&self) -> usize {
        self.maximum
    }
}

impl fmt::Display for FrontendLimitViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "frontend limit `{}` exceeded: actual={}, maximum={}",
            self.limit, self.actual, self.maximum
        )
    }
}

/// Canonical, format-independent quantum frontend error.
///
/// Source spans are deliberately not stored here. The future `source.rs`
/// module owns source location data and `diagnostics.rs` owns diagnostic
/// composition. A caller can therefore attach a span without making this
/// error type depend on a particular source-map representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontendError {
    kind: FrontendErrorKind,
    code: FrontendErrorCode,
    message: String,
    contexts: Vec<FrontendErrorContext>,
    limit: Option<FrontendLimitViolation>,
}

impl FrontendError {
    /// Creates a structured frontend error.
    pub fn new(
        kind: FrontendErrorKind,
        code: FrontendErrorCode,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            code,
            message: message.into(),
            contexts: Vec::new(),
            limit: None,
        }
    }

    /// Creates an invalid-input error.
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::new(
            FrontendErrorKind::InvalidInput,
            FrontendErrorCode::INVALID_INPUT,
            message,
        )
    }

    /// Creates an unsupported-feature/representation error.
    pub fn unsupported(message: impl Into<String>) -> Self {
        Self::new(
            FrontendErrorKind::Unsupported,
            FrontendErrorCode::UNSUPPORTED,
            message,
        )
    }

    /// Creates an import error.
    pub fn import(message: impl Into<String>) -> Self {
        Self::new(
            FrontendErrorKind::Import,
            FrontendErrorCode::IMPORT,
            message,
        )
    }

    /// Creates an export error.
    pub fn export(message: impl Into<String>) -> Self {
        Self::new(
            FrontendErrorKind::Export,
            FrontendErrorCode::EXPORT,
            message,
        )
    }

    /// Creates a lowering error.
    pub fn lowering(message: impl Into<String>) -> Self {
        Self::new(
            FrontendErrorKind::Lowering,
            FrontendErrorCode::LOWERING,
            message,
        )
    }

    /// Creates an internal frontend invariant error.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(
            FrontendErrorKind::Internal,
            FrontendErrorCode::INTERNAL,
            message,
        )
    }

    /// Creates a resource-limit error with structured limit information.
    pub fn limit_exceeded(
        violation: FrontendLimitViolation,
    ) -> Self {
        Self {
            kind: FrontendErrorKind::LimitExceeded,
            code: FrontendErrorCode::LIMIT_EXCEEDED,
            message: violation.to_string(),
            contexts: Vec::new(),
            limit: Some(violation),
        }
    }

    /// Creates an error using a caller-supplied stable code.
    ///
    /// This is the intended extension point for independently removable
    /// formats. For example, OpenQASM can use `QASM-E001` without requiring a
    /// central frontend registry edit.
    pub fn with_code(
        kind: FrontendErrorKind,
        code: FrontendErrorCode,
        message: impl Into<String>,
    ) -> Self {
        Self::new(kind, code, message)
    }

    /// Adds deterministic structured context.
    pub fn context(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.contexts
            .push(FrontendErrorContext::new(key, value));

        self
    }

    /// Adds an existing context item.
    pub fn with_context(
        mut self,
        context: FrontendErrorContext,
    ) -> Self {
        self.contexts.push(context);
        self
    }

    /// Returns the high-level error category.
    pub const fn kind(&self) -> FrontendErrorKind {
        self.kind
    }

    /// Returns the stable machine-readable code.
    pub const fn code(&self) -> FrontendErrorCode {
        self.code
    }

    /// Returns the primary human-readable message without context suffixes.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns structured context in deterministic insertion order.
    pub fn contexts(&self) -> &[FrontendErrorContext] {
        &self.contexts
    }

    /// Returns structured limit information when this is a limit error.
    pub fn limit_violation(&self) -> Option<&FrontendLimitViolation> {
        self.limit.as_ref()
    }

    /// Returns whether this represents an unsupported feature or
    /// representation.
    pub const fn is_unsupported(&self) -> bool {
        matches!(self.kind, FrontendErrorKind::Unsupported)
    }

    /// Returns whether a configured frontend limit was exceeded.
    pub const fn is_limit_exceeded(&self) -> bool {
        matches!(self.kind, FrontendErrorKind::LimitExceeded)
    }

    /// Returns whether an internal frontend invariant failed.
    pub const fn is_internal(&self) -> bool {
        matches!(self.kind, FrontendErrorKind::Internal)
    }
}

impl fmt::Display for FrontendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{}]: {}",
            self.kind,
            self.code,
            self.message
        )?;

        for context in &self.contexts {
            write!(
                f,
                "; {}={}",
                context.key(),
                context.value()
            )?;
        }

        Ok(())
    }
}

impl Error for FrontendError {}

/// Converts a filesystem failure into the frontend import error domain.
///
/// Format-specific importers can add path/include context afterwards. No
/// filesystem access is performed by this error conversion itself.
impl From<std::io::Error> for FrontendError {
    fn from(error: std::io::Error) -> Self {
        Self::import(error.to_string()).context("source", "io")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes_are_stable_and_displayable() {
        assert_eq!(
            FrontendErrorCode::INVALID_INPUT.as_str(),
            "FE-001"
        );

        assert_eq!(
            FrontendErrorCode::UNSUPPORTED.to_string(),
            "FE-002"
        );
    }

    #[test]
    fn format_specific_codes_require_no_central_registry_change() {
        const QASM_SYNTAX: FrontendErrorCode =
            FrontendErrorCode::new("QASM-E001");

        let error = FrontendError::with_code(
            FrontendErrorKind::Syntax,
            QASM_SYNTAX,
            "expected ';'",
        );

        assert_eq!(error.code().as_str(), "QASM-E001");
        assert_eq!(error.kind(), FrontendErrorKind::Syntax);
        assert_eq!(error.message(), "expected ';'");
    }

    #[test]
    fn context_formatting_is_deterministic() {
        let error = FrontendError::invalid_input("invalid source")
            .context("format", "OpenQASM")
            .context("version", "3.1");

        assert_eq!(
            error.to_string(),
            "invalid_input [FE-001]: invalid source; \
             format=OpenQASM; version=3.1"
        );
    }

    #[test]
    fn limit_errors_preserve_structured_values() {
        let violation =
            FrontendLimitViolation::new(
                "max_source_bytes",
                1025,
                1024,
            );

        let error =
            FrontendError::limit_exceeded(violation.clone());

        assert!(error.is_limit_exceeded());

        assert_eq!(
            error.code(),
            FrontendErrorCode::LIMIT_EXCEEDED
        );

        assert_eq!(
            error.limit_violation(),
            Some(&violation)
        );

        assert_eq!(
            error.message(),
            violation.to_string()
        );
    }

    #[test]
    fn io_errors_become_import_errors() {
        let error = FrontendError::from(
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "missing source",
            ),
        );

        assert_eq!(
            error.kind(),
            FrontendErrorKind::Import
        );

        assert_eq!(
            error.code(),
            FrontendErrorCode::IMPORT
        );

        assert_eq!(
            error.message(),
            "missing source"
        );

        assert_eq!(
            error.contexts()[0].key(),
            "source"
        );

        assert_eq!(
            error.contexts()[0].value(),
            "io"
        );
    }

    #[test]
    fn errors_are_cloneable_and_comparable_for_deterministic_tests() {
        let error = FrontendError::unsupported(
            "calibration block",
        )
        .context("format", "OpenQASM")
        .context("version", "3.1");

        assert_eq!(error.clone(), error);
    }

    #[test]
    fn error_kind_display_is_stable() {
        assert_eq!(
            FrontendErrorKind::Lexical.to_string(),
            "lexical"
        );

        assert_eq!(
            FrontendErrorKind::Syntax.to_string(),
            "syntax"
        );

        assert_eq!(
            FrontendErrorKind::Semantic.to_string(),
            "semantic"
        );

        assert_eq!(
            FrontendErrorKind::Unsupported.to_string(),
            "unsupported"
        );

        assert_eq!(
            FrontendErrorKind::LimitExceeded.to_string(),
            "limit_exceeded"
        );

        assert_eq!(
            FrontendErrorKind::Lowering.to_string(),
            "lowering"
        );
    }

    #[test]
    fn custom_context_is_preserved_in_order() {
        let error = FrontendError::internal("invariant")
            .with_context(
                FrontendErrorContext::new(
                    "stage",
                    "lowering",
                ),
            )
            .with_context(
                FrontendErrorContext::new(
                    "format",
                    "OpenQASM",
                ),
            );

        assert_eq!(
            error.contexts()[0].key(),
            "stage"
        );

        assert_eq!(
            error.contexts()[1].key(),
            "format"
        );
    }

    #[test]
    fn convenience_constructors_have_expected_kinds() {
        assert_eq!(
            FrontendError::invalid_input("x").kind(),
            FrontendErrorKind::InvalidInput
        );

        assert_eq!(
            FrontendError::unsupported("x").kind(),
            FrontendErrorKind::Unsupported
        );

        assert_eq!(
            FrontendError::import("x").kind(),
            FrontendErrorKind::Import
        );

        assert_eq!(
            FrontendError::export("x").kind(),
            FrontendErrorKind::Export
        );

        assert_eq!(
            FrontendError::lowering("x").kind(),
            FrontendErrorKind::Lowering
        );

        assert_eq!(
            FrontendError::internal("x").kind(),
            FrontendErrorKind::Internal
        );
    }
}