//! Zamani Quantum Frontend — canonical frontend error model.
//!
//! This module owns the format-independent control-flow error contract for
//! `quantum::frontend`. It deliberately does not know about OpenQASM, QIR,
//! Quil, hardware, runtime execution, or canonical Quantum IR semantics.
//!
//! # Architectural boundary
//!
//! The dependency direction is intentionally one-way:
//!
//! ```text
//! core::limits ───────────────► core::errors
//! core::source  ───────────────► core::diagnostics
//!                                  ▲
//!                                  │
//!                         frontend producers
//! ```
//!
//! `limits.rs` owns resource-limit identity and violation data. This module
//! re-exports that violation type so callers have one stable error-facing
//! import path without creating a duplicate limit model.
//!
//! `source.rs` owns source spans and `diagnostics.rs` owns user-facing
//! diagnostic composition. Consequently `FrontendError` does not contain a
//! source span. A producer can report a `FrontendError` for control flow and
//! a `Diagnostic` for source-oriented presentation without coupling the two
//! abstractions.
//!
//! # Error-code contract
//!
//! Format-independent codes use the `FE-*` namespace. Independently removable
//! formats own their own namespaces, for example `QASM-E001`. Error codes are
//! stable API identifiers: changing the meaning of an existing code is a
//! compatibility break; introduce a new code instead.
//!
//! # Security contract
//!
//! - malformed source is represented as data, never as a required panic;
//! - no I/O is performed by this module;
//! - error formatting is deterministic and contains no timestamps or random
//!   identifiers;
//! - error context is ordered rather than hash-map based;
//! - resource-limit failures retain the exact configured limit and observed
//!   value;
//! - errors do not execute, resolve, or inspect external source references.
//!
//! Error messages are intended for diagnostics, logs, and developer tooling.
//! Callers must use `code()`/`kind()` rather than parsing message text.
//!
//! # Rust compatibility
//!
//! Rust 2021 / Rust 1.97.1. Standard library only.

use std::error::Error;
use std::fmt;

use super::limits::FrontendLimitKind;

/// Canonical frontend resource-limit violation.
///
/// `limits.rs` is the single owner of this type. Re-exporting it here keeps
/// the error API convenient without creating a second limit representation.
pub use super::limits::FrontendLimitViolation;

/// Canonical result type for public quantum frontend operations.
pub type FrontendResult<T> = Result<T, FrontendError>;

/// Stable high-level classification of a frontend failure.
///
/// Format implementations map local failures into this vocabulary while
/// retaining format-specific stable error codes where appropriate.
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

impl FrontendErrorKind {
    /// Returns the stable machine-readable spelling of the error kind.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
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
        }
    }

    /// Returns whether the error represents malformed source syntax.
    #[must_use]
    pub const fn is_parse_failure(self) -> bool {
        matches!(self, Self::Lexical | Self::Syntax)
    }

    /// Returns whether the error represents semantic validation failure.
    #[must_use]
    pub const fn is_semantic_failure(self) -> bool {
        matches!(self, Self::Semantic)
    }

    /// Returns whether processing exceeded a configured resource boundary.
    #[must_use]
    pub const fn is_limit_failure(self) -> bool {
        matches!(self, Self::LimitExceeded)
    }
}

impl fmt::Display for FrontendErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Stable machine-readable frontend error code.
///
/// The type stores a static string so format implementations can declare
/// codes as constants without a registry or allocation. `new()` is intended
/// for trusted, compile-time-defined codes such as `QASM-E001`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct FrontendErrorCode(&'static str);

impl FrontendErrorCode {
    /// Invalid caller input.
    pub const INVALID_INPUT: Self = Self("FE-001");

    /// Unsupported format/version/feature.
    pub const UNSUPPORTED: Self = Self("FE-002");

    /// Frontend resource limit exceeded.
    pub const LIMIT_EXCEEDED: Self = Self("FE-003");

    /// External representation import failed.
    pub const IMPORT: Self = Self("FE-004");

    /// Canonical IR export failed.
    pub const EXPORT: Self = Self("FE-005");

    /// Validated representation could not be lowered.
    pub const LOWERING: Self = Self("FE-006");

    /// Frontend invariant failed.
    pub const INTERNAL: Self = Self("FE-007");

    /// Diagnostic construction/rendering failed.
    pub const DIAGNOSTIC: Self = Self("FE-008");

    /// Generic lexical failure.
    pub const LEXICAL: Self = Self("FE-009");

    /// Generic syntax failure.
    pub const SYNTAX: Self = Self("FE-010");

    /// Generic semantic validation failure.
    pub const SEMANTIC: Self = Self("FE-011");

    /// Creates a trusted static code.
    ///
    /// This is the extension point for independently removable formats.
    /// Format modules should normally expose their codes as constants.
    pub const fn new(code: &'static str) -> Self {
        Self(code)
    }

    /// Returns the stable code string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }

    /// Returns whether the code uses the allowed machine-readable syntax.
    ///
    /// Valid characters are ASCII letters, digits, `-`, `_`, and `.`.
    /// Namespace semantics are intentionally left to the owning subsystem.
    #[must_use]
    pub fn is_well_formed(self) -> bool {
        !self.0.is_empty()
            && self.0.bytes().all(|byte| {
                byte.is_ascii_alphanumeric()
                    || matches!(byte, b'-' | b'_' | b'.')
            })
    }
}

impl AsRef<str> for FrontendErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
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
/// iteration order. Context is intended for small, bounded metadata such as
/// `format=OpenQASM` or `version=3.1`, not source-file contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontendErrorContext {
    key: String,
    value: String,
}

impl FrontendErrorContext {
    /// Creates a context item.
    #[must_use]
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
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

/// Canonical, format-independent quantum frontend error.
///
/// Source spans deliberately remain outside this type. Use `diagnostics.rs`
/// to associate an error code/message with one or more `SourceSpan` values.
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
    #[must_use]
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

    /// Creates a lexical error.
    #[must_use]
    pub fn lexical(message: impl Into<String>) -> Self {
        Self::new(
            FrontendErrorKind::Lexical,
            FrontendErrorCode::LEXICAL,
            message,
        )
    }

    /// Creates a syntax error.
    #[must_use]
    pub fn syntax(message: impl Into<String>) -> Self {
        Self::new(
            FrontendErrorKind::Syntax,
            FrontendErrorCode::SYNTAX,
            message,
        )
    }

    /// Creates a semantic validation error.
    #[must_use]
    pub fn semantic(message: impl Into<String>) -> Self {
        Self::new(
            FrontendErrorKind::Semantic,
            FrontendErrorCode::SEMANTIC,
            message,
        )
    }

    /// Creates an invalid-input error.
    #[must_use]
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::new(
            FrontendErrorKind::InvalidInput,
            FrontendErrorCode::INVALID_INPUT,
            message,
        )
    }

    /// Creates an unsupported-feature/representation error.
    #[must_use]
    pub fn unsupported(message: impl Into<String>) -> Self {
        Self::new(
            FrontendErrorKind::Unsupported,
            FrontendErrorCode::UNSUPPORTED,
            message,
        )
    }

    /// Creates an import error.
    #[must_use]
    pub fn import(message: impl Into<String>) -> Self {
        Self::new(
            FrontendErrorKind::Import,
            FrontendErrorCode::IMPORT,
            message,
        )
    }

    /// Creates an export error.
    #[must_use]
    pub fn export(message: impl Into<String>) -> Self {
        Self::new(
            FrontendErrorKind::Export,
            FrontendErrorCode::EXPORT,
            message,
        )
    }

    /// Creates a lowering error.
    #[must_use]
    pub fn lowering(message: impl Into<String>) -> Self {
        Self::new(
            FrontendErrorKind::Lowering,
            FrontendErrorCode::LOWERING,
            message,
        )
    }

    /// Creates a diagnostic-system error.
    #[must_use]
    pub fn diagnostic(message: impl Into<String>) -> Self {
        Self::new(
            FrontendErrorKind::Diagnostic,
            FrontendErrorCode::DIAGNOSTIC,
            message,
        )
    }

    /// Creates an internal frontend invariant error.
    #[must_use]
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(
            FrontendErrorKind::Internal,
            FrontendErrorCode::INTERNAL,
            message,
        )
    }

    /// Creates a resource-limit error using the canonical limit model from
    /// `limits.rs`.
    #[must_use]
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

    /// Creates a resource-limit error directly from its identity and values.
    #[must_use]
    pub fn limit(
        kind: FrontendLimitKind,
        actual: u64,
        maximum: u64,
    ) -> Self {
        Self::limit_exceeded(
            FrontendLimitViolation::new(
                kind,
                actual,
                maximum,
            ),
        )
    }

    /// Creates an error using a caller-supplied stable code.
    ///
    /// This is the extension point for independently removable formats.
    #[must_use]
    pub fn with_code(
        kind: FrontendErrorKind,
        code: FrontendErrorCode,
        message: impl Into<String>,
    ) -> Self {
        Self::new(kind, code, message)
    }

    /// Adds deterministic structured context.
    #[must_use]
    pub fn context(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.contexts.push(
            FrontendErrorContext::new(key, value)
        );
        self
    }

    /// Adds an existing context item.
    #[must_use]
    pub fn with_context(
        mut self,
        context: FrontendErrorContext,
    ) -> Self {
        self.contexts.push(context);
        self
    }

    /// Returns the high-level error category.
    #[must_use]
    pub const fn kind(&self) -> FrontendErrorKind {
        self.kind
    }

    /// Returns the stable machine-readable code.
    #[must_use]
    pub const fn code(&self) -> FrontendErrorCode {
        self.code
    }

    /// Returns the primary human-readable message without context suffixes.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns structured context in deterministic insertion order.
    #[must_use]
    pub fn contexts(&self) -> &[FrontendErrorContext] {
        &self.contexts
    }

    /// Returns structured limit information when this is a limit error.
    #[must_use]
    pub fn limit_violation(
        &self,
    ) -> Option<&FrontendLimitViolation> {
        self.limit.as_ref()
    }

    /// Returns whether this represents an unsupported feature or
    /// representation.
    #[must_use]
    pub const fn is_unsupported(&self) -> bool {
        matches!(
            self.kind,
            FrontendErrorKind::Unsupported
        )
    }

    /// Returns whether a configured frontend limit was exceeded.
    #[must_use]
    pub const fn is_limit_exceeded(&self) -> bool {
        matches!(
            self.kind,
            FrontendErrorKind::LimitExceeded
        )
    }

    /// Returns whether an internal frontend invariant failed.
    #[must_use]
    pub const fn is_internal(&self) -> bool {
        matches!(
            self.kind,
            FrontendErrorKind::Internal
        )
    }

    /// Returns whether this error represents a lexical or syntactic
    /// parse failure.
    #[must_use]
    pub const fn is_parse_failure(&self) -> bool {
        self.kind.is_parse_failure()
    }

    /// Returns whether this error represents semantic validation failure.
    #[must_use]
    pub const fn is_semantic_failure(&self) -> bool {
        self.kind.is_semantic_failure()
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

/// Converts an I/O failure into the frontend import error domain.
///
/// This conversion does not perform I/O. It only classifies an already
/// observed standard-library error. Format-specific include resolvers may add
/// bounded context after conversion.
impl From<std::io::Error> for FrontendError {
    fn from(error: std::io::Error) -> Self {
        Self::import(error.to_string())
            .context("source", "io")
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

        assert!(
            FrontendErrorCode::INVALID_INPUT
                .is_well_formed()
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

        assert_eq!(
            error.code().as_str(),
            "QASM-E001"
        );

        assert_eq!(
            error.kind(),
            FrontendErrorKind::Syntax
        );

        assert_eq!(
            error.message(),
            "expected ';'"
        );

        assert!(error.is_parse_failure());
    }

    #[test]
    fn context_formatting_is_deterministic() {
        let error = FrontendError::invalid_input(
            "invalid source",
        )
        .context("format", "OpenQASM")
        .context("version", "3.1");

        assert_eq!(
            error.to_string(),
            "invalid_input [FE-001]: invalid source; \
             format=OpenQASM; version=3.1"
                .replace("\n", "")
                .replace("             ", " ")
        );
    }

    #[test]
    fn limit_errors_use_the_single_canonical_limit_type() {
        let violation =
            FrontendLimitViolation::new(
                FrontendLimitKind::SourceBytes,
                1025,
                1024,
            );

        let error =
            FrontendError::limit_exceeded(violation);

        assert!(error.is_limit_exceeded());

        assert_eq!(
            error.code(),
            FrontendErrorCode::LIMIT_EXCEEDED
        );

        assert_eq!(
            error
                .limit_violation()
                .map(|value| value.kind()),
            Some(FrontendLimitKind::SourceBytes)
        );

        assert_eq!(
            error
                .limit_violation()
                .map(|value| value.actual()),
            Some(1025)
        );

        assert_eq!(
            error
                .limit_violation()
                .map(|value| value.maximum()),
            Some(1024)
        );
    }

    #[test]
    fn direct_limit_constructor_is_deterministic() {
        let error = FrontendError::limit(
            FrontendLimitKind::Tokens,
            11,
            10,
        );

        assert_eq!(
            error.to_string(),
            "limit_exceeded [FE-003]: frontend resource limit \
             `tokens` exceeded: 11 > 10"
                .replace("\n", "")
                .replace("             ", " ")
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
    fn errors_are_cloneable_and_comparable() {
        let error = FrontendError::unsupported(
            "calibration block",
        )
        .context("format", "OpenQASM")
        .context("version", "3.1");

        assert_eq!(error.clone(), error);
    }

    #[test]
    fn error_kind_display_and_predicates_are_stable() {
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

        assert!(
            FrontendErrorKind::Lexical
                .is_parse_failure()
        );

        assert!(
            FrontendErrorKind::Syntax
                .is_parse_failure()
        );

        assert!(
            FrontendErrorKind::Semantic
                .is_semantic_failure()
        );

        assert!(
            FrontendErrorKind::LimitExceeded
                .is_limit_failure()
        );
    }

    #[test]
    fn custom_context_is_preserved_in_order() {
        let error = FrontendError::internal(
            "invariant",
        )
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
            FrontendError::lexical("x").kind(),
            FrontendErrorKind::Lexical
        );

        assert_eq!(
            FrontendError::syntax("x").kind(),
            FrontendErrorKind::Syntax
        );

        assert_eq!(
            FrontendError::semantic("x").kind(),
            FrontendErrorKind::Semantic
        );

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
            FrontendError::diagnostic("x").kind(),
            FrontendErrorKind::Diagnostic
        );

        assert_eq!(
            FrontendError::internal("x").kind(),
            FrontendErrorKind::Internal
        );
    }
}