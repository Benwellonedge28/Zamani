#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Structured diagnostics for Zamani backend operations.
//!
//! Path: `src/backend/diagnostics.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Define backend-specific diagnostic data and an explicit adapter to the
//! existing compiler-wide diagnostic model.
//!
//! # Architectural authority
//!
//! This module follows the contracts in:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/source_map.rs`
//! - `src/backend/errors.rs`
//! - `src/compiler/diagnostics.rs`
//! - `src/compiler/mod.rs`
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Backend diagnostic codes and their validation.
//! - Backend diagnostic severity.
//! - Target and operation context.
//! - Optional source spans and remediation.
//! - Related diagnostic notes.
//! - Conversion into the existing compiler diagnostic representation.
//!
//! # Non-goals
//!
//! This module does not own:
//!
//! - A second compiler-wide diagnostic collector.
//! - Diagnostic rendering, terminal colors, or console output.
//! - Source maps or source-span allocation.
//! - Semantic validation or IR verification.
//! - Target selection, resource allocation, or execution.
//! - Logging, telemetry, retries, or recovery policy.
//!
//! # Important invariants
//!
//! - Diagnostics are data, not console output.
//! - Diagnostic codes are independent of message wording.
//! - Unsupported target capabilities are not source-language errors.
//! - Resource exhaustion does not establish that a program is invalid.
//! - Missing source locations are represented explicitly.
//! - Diagnostic limits belong to the existing diagnostic engine or an
//!   explicit operational policy, not to a universal backend ceiling.
//! - Unknown future diagnostic codes remain representable.
//!
//! # Integration
//!
//! `src/backend/mod.rs` must declare `pub mod diagnostics;`.
//!
//! Backend operations should return structured errors from `errors.rs`.
//! Callers may adapt those errors to diagnostics using
//! `BackendDiagnostic::from_error`.
//!
//! The final compiler integration uses
//! `to_compiler_diagnostic()` to produce
//! `crate::compiler::diagnostics::Diagnostic`.
//!
//! The existing `DiagnosticEngine` remains responsible for collection,
//! duplicate suppression, diagnostic limits, and presentation workflows.

use core::fmt;

use crate::source_map::Span;

use super::errors::{BackendError, BackendErrorKind};

/// Stable identifier for a backend diagnostic.
///
/// Codes are extensible strings rather than a closed enum. This allows
/// new backends and future computational domains to define diagnostic
/// namespaces without changing this module.
///
/// Existing codes must retain their documented meanings. A code must not
/// be reused for an unrelated diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiagnosticCode(String);

impl DiagnosticCode {
    /// Construct a diagnostic code after validating its syntax.
    ///
    /// Accepted characters are ASCII letters, digits, `.`, `_`, `-`, and
    /// `:`. The first character must be an ASCII letter or digit.
    ///
    /// No arbitrary length ceiling is imposed. Callers should nevertheless
    /// use short, stable codes suitable for diagnostics and tooling.
    pub fn new(code: impl Into<String>) -> Result<Self, DiagnosticCodeError> {
        let code = code.into();

        let mut chars = code.chars();

        let Some(first) = chars.next() else {
            return Err(DiagnosticCodeError::Empty);
        };

        if !first.is_ascii_alphanumeric() {
            return Err(DiagnosticCodeError::InvalidFirstCharacter);
        }

        if !chars.all(|ch| {
            ch.is_ascii_alphanumeric()
                || matches!(ch, '.' | '_' | '-' | ':')
        }) {
            return Err(DiagnosticCodeError::InvalidCharacter);
        }

        Ok(Self(code))
    }

    /// Return the code as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Construct a code from a trusted, compile-time code literal.
    ///
    /// This is intended for constants controlled by the implementation,
    /// not arbitrary external input.
    pub const fn from_static(code: &'static str) -> Self {
        Self(String::new())
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Errors encountered while validating diagnostic codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCodeError {
    /// No code was supplied.
    Empty,

    /// The first character was not an ASCII letter or digit.
    InvalidFirstCharacter,

    /// The code contains a character outside the permitted alphabet.
    InvalidCharacter,
}

impl fmt::Display for DiagnosticCodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("diagnostic code cannot be empty"),
            Self::InvalidFirstCharacter => formatter.write_str(
                "diagnostic code must start with an ASCII letter or digit",
            ),
            Self::InvalidCharacter => formatter.write_str(
                "diagnostic code contains an unsupported character",
            ),
        }
    }
}

impl std::error::Error for DiagnosticCodeError {}

/// Severity of a backend diagnostic.
///
/// This is deliberately separate from error classification. For example,
/// a target optimization that is unavailable may be reported as a warning
/// when the optimization is optional, but as an error when it is mandatory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BackendDiagnosticLevel {
    /// Compilation cannot safely continue under the requested contract.
    Error,

    /// Compilation may continue without violating required semantics.
    Warning,

    /// Contextual information associated with another diagnostic.
    Note,

    /// Actionable remediation guidance.
    Help,
}

impl BackendDiagnosticLevel {
    /// Stable textual representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Note => "note",
            Self::Help => "help",
        }
    }

    /// Whether this severity prevents successful compilation.
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }

    /// Convert to the repository's compiler-wide diagnostic severity.
    pub const fn to_compiler_level(
        self,
    ) -> crate::compiler::diagnostics::DiagnosticLevel {
        match self {
            Self::Error => crate::compiler::diagnostics::DiagnosticLevel::Error,
            Self::Warning => {
                crate::compiler::diagnostics::DiagnosticLevel::Warning
            }
            Self::Note => crate::compiler::diagnostics::DiagnosticLevel::Note,
            Self::Help => crate::compiler::diagnostics::DiagnosticLevel::Help,
        }
    }
}

impl fmt::Display for BackendDiagnosticLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A structured diagnostic emitted by backend infrastructure.
///
/// This type deliberately owns backend context that the compiler-wide
/// `Diagnostic` type does not represent as dedicated fields.
///
/// When converting to the compiler-wide type, target, operation, cause,
/// and related context are preserved in the note rather than silently lost.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendDiagnostic {
    level: BackendDiagnosticLevel,
    code: DiagnosticCode,
    message: String,
    span: Option<Span>,
    target: Option<String>,
    operation: Option<String>,
    suggestion: Option<String>,
    notes: Vec<String>,
}

impl BackendDiagnostic {
    /// Construct a diagnostic with a validated code.
    pub fn new(
        level: BackendDiagnosticLevel,
        code: DiagnosticCode,
        message: impl Into<String>,
    ) -> Self {
        Self {
            level,
            code,
            message: message.into(),
            span: None,
            target: None,
            operation: None,
            suggestion: None,
            notes: Vec::new(),
        }
    }

    /// Construct an error-severity diagnostic.
    pub fn error(
        code: DiagnosticCode,
        message: impl Into<String>,
    ) -> Self {
        Self::new(BackendDiagnosticLevel::Error, code, message)
    }

    /// Construct a warning-severity diagnostic.
    pub fn warning(
        code: DiagnosticCode,
        message: impl Into<String>,
    ) -> Self {
        Self::new(BackendDiagnosticLevel::Warning, code, message)
    }

    /// Construct an informational diagnostic.
    pub fn note(
        code: DiagnosticCode,
        message: impl Into<String>,
    ) -> Self {
        Self::new(BackendDiagnosticLevel::Note, code, message)
    }

    /// Construct a remediation diagnostic.
    pub fn help(
        code: DiagnosticCode,
        message: impl Into<String>,
    ) -> Self {
        Self::new(BackendDiagnosticLevel::Help, code, message)
    }

    /// Adapt a structured backend error into a diagnostic.
    ///
    /// The error classification supplies the stable diagnostic code.
    /// The original message, target, operation, context, and cause are
    /// preserved without parsing the formatted error string.
    pub fn from_error(
        error: &BackendError,
        span: Option<Span>,
    ) -> Self {
        let code = DiagnosticCode(error.code().to_owned());

        let mut diagnostic = Self::new(
            BackendDiagnosticLevel::Error,
            code,
            error.message(),
        )
        .with_span_option(span);

        if let Some(target) = error.target() {
            diagnostic = diagnostic.with_target(target);
        }

        if let Some(operation) = error.operation() {
            diagnostic = diagnostic.with_operation(operation);
        }

        for context in error.context() {
            diagnostic = diagnostic.with_note(context);
        }

        if let Some(cause) = error.cause() {
            diagnostic = diagnostic.with_note(format!("caused by: {cause}"));
        }

        diagnostic
    }

    /// Return the diagnostic severity.
    pub const fn level(&self) -> BackendDiagnosticLevel {
        self.level
    }

    /// Return the diagnostic code.
    pub fn code(&self) -> &DiagnosticCode {
        &self.code
    }

    /// Return the explanatory message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the source span, if known.
    pub fn span(&self) -> Option<&Span> {
        self.span.as_ref()
    }

    /// Return the target name or identifier, if known.
    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    /// Return the operation name or identifier, if known.
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Return the remediation suggestion, if present.
    pub fn suggestion(&self) -> Option<&str> {
        self.suggestion.as_deref()
    }

    /// Return related notes in insertion order.
    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    /// Whether this diagnostic has error severity.
    pub const fn is_error(&self) -> bool {
        self.level.is_error()
    }

    /// Attach a source span.
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    /// Attach an optional source span.
    pub fn with_span_option(mut self, span: Option<Span>) -> Self {
        self.span = span;
        self
    }

    /// Attach a target identifier or name.
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Attach an operation identifier or name.
    pub fn with_operation(mut self, operation: impl Into<String>) -> Self {
        self.operation = Some(operation.into());
        self
    }

    /// Attach a remediation suggestion.
    pub fn with_suggestion(
        mut self,
        suggestion: impl Into<String>,
    ) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Append a related note.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Convert to the existing compiler-wide diagnostic representation.
    ///
    /// `src/compiler/diagnostics.rs` currently supports one optional note.
    /// This adapter combines backend context into that note so that target,
    /// operation, and causal information are not discarded.
    ///
    /// For callers requiring the full structured backend context, retain
    /// this `BackendDiagnostic` alongside the converted diagnostic.
    pub fn to_compiler_diagnostic(
        &self,
    ) -> crate::compiler::diagnostics::Diagnostic {
        use crate::compiler::diagnostics::Diagnostic;

        let mut diagnostic = Diagnostic::new(
            self.level.to_compiler_level(),
            self.code.as_str(),
            self.message.clone(),
        );

        if let Some(span) = &self.span {
            diagnostic = diagnostic.with_span(span.clone());
        }

        if let Some(suggestion) = &self.suggestion {
            diagnostic = diagnostic.with_suggestion(suggestion.clone());
        }

        let mut details = Vec::new();

        if let Some(target) = &self.target {
            details.push(format!("target: {target}"));
        }

        if let Some(operation) = &self.operation {
            details.push(format!("operation: {operation}"));
        }

        details.extend(self.notes.iter().cloned());

        if !details.is_empty() {
            diagnostic = diagnostic.with_note(details.join("\n"));
        }

        diagnostic
    }
}

impl fmt::Display for BackendDiagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}[{}]: {}",
            self.level,
            self.code.as_str(),
            self.message
        )?;

        if let Some(target) = &self.target {
            write!(formatter, " [target: {target}]")?;
        }

        if let Some(operation) = &self.operation {
            write!(formatter, " [operation: {operation}]")?;
        }

        if let Some(span) = &self.span {
            if !span.is_dummy() {
                write!(
                    formatter,
                    " [file: {}, line: {}, column: {}]",
                    span.file_id.0,
                    span.start_line,
                    span.start_column
                )?;
            }
        }

        for note in &self.notes {
            write!(formatter, "\n  note: {note}")?;
        }

        if let Some(suggestion) = &self.suggestion {
            write!(formatter, "\n  help: {suggestion}")?;
        }

        Ok(())
    }
}

/// Construct a diagnostic directly from a backend error.
///
/// This helper makes the adapter convenient at compiler pipeline boundaries
/// while keeping conversion explicit and predictable.
impl From<&BackendError> for BackendDiagnostic {
    fn from(error: &BackendError) -> Self {
        Self::from_error(error, None)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BackendDiagnostic, BackendDiagnosticLevel, DiagnosticCode,
        DiagnosticCodeError,
    };
    use crate::backend::errors::{BackendError, BackendErrorKind};
    use crate::source_map::{BytePos, FileId, Span};

    #[test]
    fn diagnostic_codes_validate_without_a_fixed_length_limit() {
        assert_eq!(
            DiagnosticCode::new("B0001")
                .expect("valid code")
                .as_str(),
            "B0001"
        );

        assert_eq!(
            DiagnosticCode::new("quantum.capability.unsupported")
                .expect("valid namespaced code")
                .as_str(),
            "quantum.capability.unsupported"
        );

        assert_eq!(
            DiagnosticCode::new("").expect_err("empty code rejected"),
            DiagnosticCodeError::Empty
        );

        assert_eq!(
            DiagnosticCode::new("-B0001")
                .expect_err("invalid initial character"),
            DiagnosticCodeError::InvalidFirstCharacter
        );

        assert_eq!(
            DiagnosticCode::new("B0001!")
                .expect_err("invalid character"),
            DiagnosticCodeError::InvalidCharacter
        );
    }

    #[test]
    fn backend_error_conversion_preserves_structured_context() {
        let error = BackendError::unsupported_capability(
            "quantum-target",
            "quantum.measurement",
        )
        .with_operation("lower-quantum-ir")
        .with_context("compatibility validation")
        .with_cause("target descriptor does not advertise the capability");

        let span = Span::new(
            FileId::new(7),
            BytePos::new(10),
            BytePos::new(20),
            3,
            5,
        );

        let diagnostic = BackendDiagnostic::from_error(&error, Some(span));

        assert_eq!(diagnostic.level(), BackendDiagnosticLevel::Error);
        assert_eq!(diagnostic.code().as_str(), "B0004");
        assert_eq!(diagnostic.target(), Some("quantum-target"));
        assert_eq!(diagnostic.operation(), Some("lower-quantum-ir"));
        assert_eq!(
            diagnostic.span().expect("span retained").start.0,
            10
        );
        assert!(diagnostic.message().contains("quantum.measurement"));
        assert!(diagnostic
            .notes()
            .iter()
            .any(|note| note == "compatibility validation"));
        assert!(diagnostic
            .notes()
            .iter()
            .any(|note| note.contains("caused by:")));
    }

    #[test]
    fn compiler_adapter_preserves_fields_supported_by_the_existing_api() {
        let diagnostic = BackendDiagnostic::warning(
            DiagnosticCode::new("wasm.optional.optimization")
                .expect("valid code"),
            "optional optimization is unavailable",
        )
        .with_target("wasm32")
        .with_operation("optimize")
        .with_suggestion("continue without the optional optimization")
        .with_note("program semantics are preserved");

        let compiler_diagnostic = diagnostic.to_compiler_diagnostic();

        assert_eq!(
            compiler_diagnostic.level,
            crate::compiler::diagnostics::DiagnosticLevel::Warning
        );
        assert_eq!(compiler_diagnostic.code, "wasm.optional.optimization");
        assert_eq!(
            compiler_diagnostic.message,
            "optional optimization is unavailable"
        );
        assert_eq!(
            compiler_diagnostic.suggestion.as_deref(),
            Some("continue without the optional optimization")
        );

        let note = compiler_diagnostic.note.expect("context preserved");
        assert!(note.contains("target: wasm32"));
        assert!(note.contains("operation: optimize"));
        assert!(note.contains("program semantics are preserved"));
    }

    #[test]
    fn diagnostic_severity_is_independent_of_error_classification() {
        let optional_feature = BackendDiagnostic::warning(
            DiagnosticCode::new("backend.optional.feature")
                .expect("valid code"),
            "optional feature unavailable",
        );

        let required_feature = BackendDiagnostic::error(
            DiagnosticCode::new("backend.required.feature")
                .expect("valid code"),
            "required feature unavailable",
        );

        assert!(!optional_feature.is_error());
        assert!(required_feature.is_error());
    }

    #[test]
    fn backend_error_kinds_remain_distinct() {
        assert_ne!(
            BackendErrorKind::UnsupportedCapability,
            BackendErrorKind::VerificationFailed
        );
        assert_ne!(
            BackendErrorKind::ResourceExhausted,
            BackendErrorKind::InvalidInput
        );
    }

    #[test]
    fn display_includes_target_operation_and_remediation() {
        let diagnostic = BackendDiagnostic::error(
            DiagnosticCode::new("B0003").expect("valid code"),
            "target is unavailable",
        )
        .with_target("aarch64")
        .with_operation("select-target")
        .with_suggestion("register a compatible target implementation");

        let rendered = diagnostic.to_string();

        assert!(rendered.contains("B0003"));
        assert!(rendered.contains("aarch64"));
        assert!(rendered.contains("select-target"));
        assert!(rendered.contains("register a compatible target implementation"));
    }
}