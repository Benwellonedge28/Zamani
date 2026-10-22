
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Structured Diagnostics
//!
//! Path: `src/hybrid/diagnostics.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//!
//! ## Purpose
//!
//! Defines structured, source-aware diagnostics for the hybrid-composition
//! subsystem. Diagnostics explain problems encountered while parsing,
//! validating, composing, exchanging data between, or preparing computations
//! across classical, quantum, accelerator, tensor, AI/ML, HDL, distributed,
//! networking, and future computational domains.
//!
//! ## Ownership
//!
//! This module owns:
//! - Diagnostic severity and stable diagnostic-code representation.
//! - Primary and related source locations.
//! - Human-readable messages, labels, notes, and remediation suggestions.
//! - Conversion from the canonical `HybridError` representation.
//! - Diagnostic collections, deterministic ordering, and budget enforcement.
//!
//! This module does not own:
//! - Source text, source-file storage, or source-map allocation.
//! - Lexing, parsing, AST construction, name resolution, or type checking.
//! - The canonical semantic model or any domain IR.
//! - Compiler-wide diagnostic rendering or IDE protocol implementations.
//! - Hardware discovery, allocation, scheduling, routing, QEC, or execution.
//! - The canonical error vocabulary in `errors.rs`.
//!
//! ## Integration contract
//!
//! - Uses `super::errors::{HybridError, HybridErrorCode}`.
//! - Uses `super::config::{BudgetKind, HybridConfig}` for explicit diagnostic
//!   collection budgets.
//! - Uses the canonical frontend `Span` through a loss-aware adapter.
//! - Does not own or fabricate a source-file identifier when the caller has
//!   not supplied one.
//! - Compiler, parser, semantic, and runtime adapters should translate their
//!   errors into this representation at subsystem boundaries.
//! - Renderers must consume structured fields rather than parse `Display`
//!   output.
//!
//! ## Scalability
//!
//! This module defines no universal maximum diagnostic count, message length,
//! source size, source-line count, or number of related locations.
//!
//! Collections grow according to available resources unless the caller
//! configures an explicit operational budget. `Some(0)` is a valid policy.
//! An exhausted budget returns a structured error; diagnostics are never
//! silently discarded by this module.
//!
//! This does not imply infinite physical memory. Allocator and platform
//! limitations remain real.
//!
//! ## Determinism
//!
//! Diagnostic creation preserves insertion order. Callers that require a
//! canonical presentation order may call `sort_deterministically`.
//! Canonical ordering uses source identity, byte range, severity, code, and
//! message. No hash-map iteration order is involved.
//!
//! Sorting changes presentation order, not the meaning of a diagnostic.
//!
//! ## Compatibility
//!
//! `DiagnosticCode` stores a stable machine-readable code. Existing
//! `HybridErrorCode` identifiers are preserved when converting errors.
//! Human-readable messages and rendered output are not serialization
//! protocols.
//!
//! Adding public enum variants may affect exhaustive downstream matches.
//! Downstream code should use wildcard matching where forward compatibility
//! is required.

use core::cmp::Ordering;
use core::fmt;
use core::str::FromStr;

use serde::{Deserialize, Serialize};

use super::config::{BudgetKind, HybridConfig};
use super::errors::{HybridError, HybridErrorCode};
use crate::frontend::lexer::span::Span;

/// Result type used by fallible diagnostic operations.
pub type DiagnosticResult<T> = Result<T, HybridError>;

/// Severity of a diagnostic.
///
/// Severity describes the reporting importance of a diagnostic. It does not
/// itself determine whether compilation must stop. The owning compiler stage
/// decides how errors, warnings, and informational diagnostics affect its
/// workflow.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DiagnosticSeverity {
    /// Informational information that does not indicate a problem.
    Note,

    /// A possible issue that may deserve attention.
    Warning,

    /// An error that prevents the current operation from being accepted.
    Error,

    /// A fatal failure for which the current stage cannot safely continue.
    Fatal,
}

impl DiagnosticSeverity {
    /// Returns the stable textual representation of this severity.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Note => "note",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Fatal => "fatal",
        }
    }

    /// Returns whether this severity represents an error condition.
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error | Self::Fatal)
    }
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A validated, stable diagnostic identifier.
///
/// Codes are opaque to consumers except for their documented stability.
/// Existing hybrid error identifiers such as `HYB001` are retained when
/// adapting a `HybridError`.
///
/// The accepted syntax is one or more ASCII letters, digits, hyphens,
/// underscores, or periods. The module does not require every future
/// diagnostic to use one fixed prefix.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DiagnosticCode(String);

/// Failure to construct a diagnostic code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DiagnosticCodeError {
    /// The code was empty.
    Empty,

    /// The code contained a character outside the supported syntax.
    InvalidCharacter,
}

impl fmt::Display for DiagnosticCodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("diagnostic code must not be empty"),
            Self::InvalidCharacter => formatter.write_str(
                "diagnostic code may contain only ASCII letters, digits, '-', '_' or '.'",
            ),
        }
    }
}

impl std::error::Error for DiagnosticCodeError {}

impl DiagnosticCode {
    /// Constructs a diagnostic code after validating its syntax.
    pub fn new(value: impl Into<String>) -> Result<Self, DiagnosticCodeError> {
        let value = value.into();

        if value.is_empty() {
            return Err(DiagnosticCodeError::Empty);
        }

        if !value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.')
        }) {
            return Err(DiagnosticCodeError::InvalidCharacter);
        }

        Ok(Self(value))
    }

    /// Returns the machine-readable code.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Creates a code from the canonical hybrid error vocabulary.
    #[must_use]
    pub fn from_hybrid_error(code: HybridErrorCode) -> Self {
        // `HybridErrorCode::as_str` is a compile-time-controlled identifier
        // whose syntax is guaranteed by the canonical error module.
        Self(code.as_str().to_owned())
    }
}

impl FromStr for DiagnosticCode {
    type Err = DiagnosticCodeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Source identity supplied by the subsystem that owns the source map.
///
/// This is deliberately an opaque string rather than a competing source-file
/// ID type. Use a stable, canonical source identifier when one is available.
/// A display path may be supplied for presentation, but should not be treated
/// as a globally unique identity unless the caller guarantees that property.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DiagnosticSourceId(String);

impl DiagnosticSourceId {
    /// Creates an opaque source identity.
    ///
    /// Empty identities are rejected because they make source ordering and
    /// source association ambiguous.
    pub fn new(value: impl Into<String>) -> Result<Self, DiagnosticSourceIdError> {
        let value = value.into();

        if value.is_empty() {
            return Err(DiagnosticSourceIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the underlying source identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Failure to construct a source identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticSourceIdError {
    /// A source identity must not be empty.
    Empty,
}

impl fmt::Display for DiagnosticSourceIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("diagnostic source identity must not be empty"),
        }
    }
}

impl std::error::Error for DiagnosticSourceIdError {}

/// A source range associated with a diagnostic.
///
/// Byte offsets use the source's UTF-8 byte coordinate system. Lines and
/// columns use the frontend convention: one-based lines and one-based
/// Unicode-scalar columns. Ranges are half-open: `[start_byte, end_byte)`.
///
/// A source range does not contain source text and cannot independently prove
/// that its offsets lie on UTF-8 character boundaries. The source owner is
/// responsible for that guarantee.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DiagnosticSpan {
    source: Option<DiagnosticSourceId>,
    start_byte: usize,
    end_byte: usize,
    start_line: usize,
    start_column: usize,
    end_line: usize,
    end_column: usize,
}

impl DiagnosticSpan {
    /// Creates a validated source range.
    ///
    /// `source` may be absent when a parser or adapter has a valid span but
    /// does not yet know the source identity. The range and coordinates must
    /// still be internally ordered.
    pub fn new(
        source: Option<DiagnosticSourceId>,
        start_byte: usize,
        end_byte: usize,
        start_line: usize,
        start_column: usize,
        end_line: usize,
        end_column: usize,
    ) -> Result<Self, DiagnosticSpanError> {
        if end_byte < start_byte {
            return Err(DiagnosticSpanError::ReversedByteRange);
        }

        if start_line == 0 || end_line == 0 || start_column == 0 || end_column == 0 {
            return Err(DiagnosticSpanError::ZeroLineOrColumn);
        }

        if (end_line, end_column) < (start_line, start_column) {
            return Err(DiagnosticSpanError::ReversedPositionRange);
        }

        Ok(Self {
            source,
            start_byte,
            end_byte,
            start_line,
            start_column,
            end_line,
            end_column,
        })
    }

    /// Adapts the canonical frontend span without inventing a source ID.
    #[must_use]
    pub const fn from_frontend_span(span: Span) -> Self {
        Self {
            source: None,
            start_byte: span.start_byte(),
            end_byte: span.end_byte(),
            start_line: span.start_line(),
            start_column: span.start_column(),
            end_line: span.end_line(),
            end_column: span.end_column(),
        }
    }

    /// Associates this span with a source identity.
    #[must_use]
    pub fn with_source(mut self, source: DiagnosticSourceId) -> Self {
        self.source = Some(source);
        self
    }

    /// Returns the optional source identity.
    #[must_use]
    pub fn source(&self) -> Option<&DiagnosticSourceId> {
        self.source.as_ref()
    }

    /// Returns the start byte offset.
    #[must_use]
    pub const fn start_byte(&self) -> usize {
        self.start_byte
    }

    /// Returns the exclusive end byte offset.
    #[must_use]
    pub const fn end_byte(&self) -> usize {
        self.end_byte
    }

    /// Returns the number of bytes covered by this range.
    #[must_use]
    pub const fn byte_len(&self) -> usize {
        self.end_byte - self.start_byte
    }

    /// Returns the start line (one-based).
    #[must_use]
    pub const fn start_line(&self) -> usize {
        self.start_line
    }

    /// Returns the start column (one-based Unicode-scalar column).
    #[must_use]
    pub const fn start_column(&self) -> usize {
        self.start_column
    }

    /// Returns the exclusive end line.
    #[must_use]
    pub const fn end_line(&self) -> usize {
        self.end_line
    }

    /// Returns the exclusive end column.
    #[must_use]
    pub const fn end_column(&self) -> usize {
        self.end_column
    }

    /// Returns whether this range has zero byte width.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.start_byte == self.end_byte
    }

    /// Returns a copy with source identity removed.
    #[must_use]
    pub fn without_source(mut self) -> Self {
        self.source = None;
        self
    }
}

impl From<Span> for DiagnosticSpan {
    fn from(span: Span) -> Self {
        Self::from_frontend_span(span)
    }
}

impl fmt::Display for DiagnosticSpan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(source) = &self.source {
            write!(formatter, "{}:", source.as_str())?;
        }

        write!(
            formatter,
            "{}:{}-{}:{}",
            self.start_line,
            self.start_column,
            self.end_line,
            self.end_column
        )
    }
}

/// Failure to construct a diagnostic source range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DiagnosticSpanError {
    /// The exclusive end byte offset precedes the start byte offset.
    ReversedByteRange,

    /// A line or column was zero even though coordinates are one-based.
    ZeroLineOrColumn,

    /// The end source position precedes the start source position.
    ReversedPositionRange,
}

impl fmt::Display for DiagnosticSpanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReversedByteRange => {
                formatter.write_str("diagnostic span has a reversed byte range")
            }
            Self::ZeroLineOrColumn => {
                formatter.write_str("diagnostic line and column values are one-based")
            }
            Self::ReversedPositionRange => {
                formatter.write_str("diagnostic span has a reversed source position")
            }
        }
    }
}

impl std::error::Error for DiagnosticSpanError {}

/// A secondary source location that helps explain a diagnostic.
///
/// Related locations can identify a declaration, dependency, contract, or
/// other source element involved in the problem.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RelatedLocation {
    span: DiagnosticSpan,
    message: String,
}

impl RelatedLocation {
    /// Creates a related source location.
    #[must_use]
    pub fn new(span: DiagnosticSpan, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }

    /// Returns the source range.
    #[must_use]
    pub fn span(&self) -> &DiagnosticSpan {
        &self.span
    }

    /// Returns the explanatory label.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// A structured diagnostic emitted by a hybrid subsystem operation.
///
/// Fields are private so callers construct diagnostics through validated
/// constructors and can extend them through builder-style methods.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    code: DiagnosticCode,
    severity: DiagnosticSeverity,
    message: String,
    primary_span: Option<DiagnosticSpan>,
    primary_label: Option<String>,
    related: Vec<RelatedLocation>,
    notes: Vec<String>,
    help: Option<String>,
}

impl Diagnostic {
    /// Creates a diagnostic without a source location.
    ///
    /// This is appropriate for configuration, compatibility, and target
    /// feasibility issues that do not originate at one source expression.
    #[must_use]
    pub fn new(
        code: DiagnosticCode,
        severity: DiagnosticSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            severity,
            message: message.into(),
            primary_span: None,
            primary_label: None,
            related: Vec::new(),
            notes: Vec::new(),
            help: None,
        }
    }

    /// Creates a diagnostic from the canonical hybrid error vocabulary.
    ///
    /// The error's stable code is preserved. The message is copied for
    /// reporting; callers must not subsequently parse it to identify errors.
    #[must_use]
    pub fn from_hybrid_error(error: &HybridError) -> Self {
        let severity = match error.code() {
            HybridErrorCode::InternalInvariantViolation
            | HybridErrorCode::RuntimeFailure
            | HybridErrorCode::CompilationFailure => DiagnosticSeverity::Fatal,

            _ => DiagnosticSeverity::Error,
        };

        let mut diagnostic = Self::new(
            DiagnosticCode::from_hybrid_error(error.code()),
            severity,
            error.message(),
        );

        for context in error.context() {
            let mut note = context.component().to_owned();

            if let Some(entity) = context.entity() {
                note.push_str(": ");
                note.push_str(entity);
            }

            if let Some(detail) = context.detail() {
                if !note.is_empty() {
                    note.push_str(" — ");
                }
                note.push_str(detail);
            }

            diagnostic.notes.push(note);
        }

        diagnostic
    }

    /// Returns the stable machine-readable diagnostic code.
    #[must_use]
    pub fn code(&self) -> &DiagnosticCode {
        &self.code
    }

    /// Returns the severity.
    #[must_use]
    pub const fn severity(&self) -> DiagnosticSeverity {
        self.severity
    }

    /// Returns the main diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the optional primary source range.
    #[must_use]
    pub fn primary_span(&self) -> Option<&DiagnosticSpan> {
        self.primary_span.as_ref()
    }

    /// Returns the optional label for the primary range.
    #[must_use]
    pub fn primary_label(&self) -> Option<&str> {
        self.primary_label.as_deref()
    }

    /// Returns the related source locations.
    #[must_use]
    pub fn related(&self) -> &[RelatedLocation] {
        &self.related
    }

    /// Returns explanatory notes in insertion order.
    #[must_use]
    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    /// Returns the optional remediation suggestion.
    #[must_use]
    pub fn help(&self) -> Option<&str> {
        self.help.as_deref()
    }

    /// Attaches a primary source range.
    #[must_use]
    pub fn with_primary_span(mut self, span: DiagnosticSpan) -> Self {
        self.primary_span = Some(span);
        self
    }

    /// Attaches the canonical frontend span.
    #[must_use]
    pub fn with_frontend_span(self, span: Span) -> Self {
        self.with_primary_span(DiagnosticSpan::from_frontend_span(span))
    }

    /// Adds a label to the primary source range.
    #[must_use]
    pub fn with_primary_label(mut self, label: impl Into<String>) -> Self {
        self.primary_label = Some(label.into());
        self
    }

    /// Adds a related source location.
    #[must_use]
    pub fn with_related(mut self, location: RelatedLocation) -> Self {
        self.related.push(location);
        self
    }

    /// Adds a related location using a canonical frontend span.
    #[must_use]
    pub fn with_related_frontend_span(
        self,
        span: Span,
        message: impl Into<String>,
    ) -> Self {
        self.with_related(RelatedLocation::new(
            DiagnosticSpan::from_frontend_span(span),
            message,
        ))
    }

    /// Adds an explanatory note.
    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Adds a remediation suggestion.
    #[must_use]
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Returns whether this diagnostic represents an error condition.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        self.severity.is_error()
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(span) = &self.primary_span {
            write!(formatter, "{span}: ")?;
        }

        write!(
            formatter,
            "{}[{}]: {}",
            self.severity, self.code, self.message
        )?;

        if let Some(label) = &self.primary_label {
            write!(formatter, "\n  {label}")?;
        }

        for related in &self.related {
            write!(
                formatter,
                "\n  related: {}: {}",
                related.span, related.message
            )?;
        }

        for note in &self.notes {
            write!(formatter, "\n  note: {note}")?;
        }

        if let Some(help) = &self.help {
            write!(formatter, "\n  help: {help}")?;
        }

        Ok(())
    }
}

/// A collection of diagnostics produced by one logical operation.
///
/// The collection does not impose a default maximum. Use
/// [`DiagnosticSet::push_with_config`] when an explicit operational budget
/// must be enforced.
///
/// The set intentionally does not own a source map or render terminal
/// decorations. It is suitable for use by compiler, parser, semantic,
/// validation, and integration layers.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DiagnosticSet {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticSet {
    /// Creates an empty diagnostic collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    /// Returns the number of diagnostics currently stored.
    #[must_use]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Returns the diagnostics in their current order.
    #[must_use]
    pub fn as_slice(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Returns an iterator over the diagnostics in their current order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &Diagnostic> {
        self.diagnostics.iter()
    }

    /// Returns whether any diagnostic is an error or fatal diagnostic.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(Diagnostic::is_error)
    }

    /// Adds a diagnostic without applying an operational budget.
    ///
    /// Use this only when the caller has intentionally chosen not to apply a
    /// diagnostic-count limit. It does not imply that allocation cannot fail.
    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Adds a diagnostic while enforcing the configured diagnostic-count
    /// budget before mutating the collection.
    ///
    /// A limit of `Some(0)` rejects the first diagnostic. A missing limit
    /// means this configuration does not impose a count limit.
    ///
    /// On failure, the existing collection remains unchanged.
    pub fn push_with_config(
        &mut self,
        diagnostic: Diagnostic,
        config: &HybridConfig,
    ) -> DiagnosticResult<()> {
        config.check_budget(
            BudgetKind::DiagnosticCount,
            self.diagnostics.len() as u128 + 1,
        )?;

        self.diagnostics.push(diagnostic);
        Ok(())
    }

    /// Extends the collection while checking the configured budget for each
    /// diagnostic. If a later item exceeds the budget, earlier items from
    /// this call remain inserted.
    ///
    /// This incremental behavior is deliberate: it avoids buffering an
    /// additional unbounded intermediate collection. Callers needing
    /// all-or-nothing batch insertion should validate the batch count first
    /// and use a separately configured staging collection.
    pub fn extend_with_config<I>(
        &mut self,
        diagnostics: I,
        config: &HybridConfig,
    ) -> DiagnosticResult<()>
    where
        I: IntoIterator<Item = Diagnostic>,
    {
        for diagnostic in diagnostics {
            self.push_with_config(diagnostic, config)?;
        }

        Ok(())
    }

    /// Consumes the collection and returns its diagnostics.
    #[must_use]
    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    /// Sorts diagnostics into a deterministic presentation order.
    ///
    /// Ordering is based on source identity, start/end byte offsets, severity,
    /// code, and message. Diagnostics without source locations sort before
    /// source-associated diagnostics. Ties remain equivalent under the
    /// comparator; the stable slice sort preserves their relative order.
    ///
    /// The ordering is a presentation policy and does not establish causal
    /// order or semantic priority.
    pub fn sort_deterministically(&mut self) {
        self.diagnostics.sort_by(compare_diagnostics);
    }
}

impl IntoIterator for DiagnosticSet {
    type Item = Diagnostic;
    type IntoIter = std::vec::IntoIter<Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.into_iter()
    }
}

impl<'a> IntoIterator for &'a DiagnosticSet {
    type Item = &'a Diagnostic;
    type IntoIter = std::slice::Iter<'a, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.iter()
    }
}

fn compare_diagnostics(left: &Diagnostic, right: &Diagnostic) -> Ordering {
    compare_optional_spans(left.primary_span(), right.primary_span())
        .then_with(|| left.severity.cmp(&right.severity))
        .then_with(|| left.code.cmp(&right.code))
        .then_with(|| left.message.cmp(&right.message))
}

fn compare_optional_spans(
    left: Option<&DiagnosticSpan>,
    right: Option<&DiagnosticSpan>,
) -> Ordering {
    match (left, right) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(left), Some(right)) => left
            .source
            .cmp(&right.source)
            .then_with(|| left.start_byte.cmp(&right.start_byte))
            .then_with(|| left.end_byte.cmp(&right.end_byte))
            .then_with(|| left.start_line.cmp(&right.start_line))
            .then_with(|| left.start_column.cmp(&right.start_column))
            .then_with(|| left.end_line.cmp(&right.end_line))
            .then_with(|| left.end_column.cmp(&right.end_column)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::lexer::cursor::CursorPosition;

    fn frontend_span() -> Span {
        Span::new(
            CursorPosition::new(0, 1, 1),
            CursorPosition::new(5, 1, 6),
        )
    }

    fn diagnostic(code: &str, severity: DiagnosticSeverity, message: &str) -> Diagnostic {
        Diagnostic::new(
            DiagnosticCode::new(code).expect("test code is valid"),
            severity,
            message,
        )
    }

    #[test]
    fn diagnostic_code_rejects_empty_and_invalid_values() {
        assert_eq!(
            DiagnosticCode::new(""),
            Err(DiagnosticCodeError::Empty)
        );

        assert_eq!(
            DiagnosticCode::new("HYB bad"),
            Err(DiagnosticCodeError::InvalidCharacter)
        );

        assert_eq!(
            DiagnosticCode::new("HYB-100").expect("valid code").as_str(),
            "HYB-100"
        );
    }

    #[test]
    fn frontend_span_adapter_preserves_coordinates() {
        let span = DiagnosticSpan::from_frontend_span(frontend_span());

        assert_eq!(span.start_byte(), 0);
        assert_eq!(span.end_byte(), 5);
        assert_eq!(span.start_line(), 1);
        assert_eq!(span.start_column(), 1);
        assert_eq!(span.end_line(), 1);
        assert_eq!(span.end_column(), 6);
    }

    #[test]
    fn diagnostic_span_rejects_reversed_byte_ranges() {
        assert_eq!(
            DiagnosticSpan::new(None, 10, 9, 1, 1, 1, 2),
            Err(DiagnosticSpanError::ReversedByteRange)
        );
    }

    #[test]
    fn diagnostic_span_rejects_zero_based_line_or_column() {
        assert_eq!(
            DiagnosticSpan::new(None, 0, 0, 0, 1, 0, 1),
            Err(DiagnosticSpanError::ZeroLineOrColumn)
        );
    }

    #[test]
    fn diagnostic_span_accepts_zero_width_ranges() {
        let span = DiagnosticSpan::new(None, 7, 7, 2, 4, 2, 4)
            .expect("zero-width insertion spans are valid");

        assert!(span.is_empty());
        assert_eq!(span.byte_len(), 0);
    }

    #[test]
    fn diagnostics_preserve_source_spans_and_labels() {
        let diagnostic = diagnostic(
            "HYB-SYNTAX-001",
            DiagnosticSeverity::Error,
            "expected a domain invocation",
        )
        .with_frontend_span(frontend_span())
        .with_primary_label("this construct is incomplete")
        .with_help("complete the hybrid expression");

        assert_eq!(
            diagnostic.primary_span().expect("span exists").start_byte(),
            0
        );
        assert_eq!(
            diagnostic.primary_label(),
            Some("this construct is incomplete")
        );
        assert_eq!(
            diagnostic.help(),
            Some("complete the hybrid expression")
        );
    }

    #[test]
    fn conversion_preserves_the_canonical_hybrid_error_code() {
        let error = HybridError::invalid_dependency("dependency is unresolved")
            .in_component("hybrid.graph.verify");

        let diagnostic = Diagnostic::from_hybrid_error(&error);

        assert_eq!(diagnostic.code().as_str(), "HYB005");
        assert_eq!(diagnostic.severity(), DiagnosticSeverity::Error);
        assert!(diagnostic.message().contains("dependency is unresolved"));
        assert_eq!(diagnostic.notes().len(), 1);
        assert!(diagnostic.notes()[0].contains("hybrid.graph.verify"));
    }

    #[test]
    fn diagnostic_set_has_no_implicit_count_ceiling() {
        let mut set = DiagnosticSet::new();

        for index in 0..128 {
            set.push(diagnostic(
                "HYB-TEST",
                DiagnosticSeverity::Note,
                &format!("diagnostic {index}"),
            ));
        }

        assert_eq!(set.len(), 128);
        assert!(!set.has_errors());
    }

    #[test]
    fn configured_diagnostic_budget_is_enforced_before_mutation() {
        let config = HybridConfig {
            budgets: super::super::config::OperationalBudgets {
                max_diagnostic_count: Some(1),
                ..Default::default()
            },
            ..HybridConfig::default()
        };

        let mut set = DiagnosticSet::new();

        set.push_with_config(
            diagnostic("HYB-ONE", DiagnosticSeverity::Note, "first"),
            &config,
        )
        .expect("first diagnostic fits");

        let error = set
            .push_with_config(
                diagnostic("HYB-TWO", DiagnosticSeverity::Note, "second"),
                &config,
            )
            .expect_err("second diagnostic exceeds the configured limit");

        assert_eq!(error.code_str(), "HYB024");
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn zero_diagnostic_budget_rejects_the_first_item() {
        let config = HybridConfig {
            budgets: super::super::config::OperationalBudgets {
                max_diagnostic_count: Some(0),
                ..Default::default()
            },
            ..HybridConfig::default()
        };

        let mut set = DiagnosticSet::new();
        let result = set.push_with_config(
            diagnostic("HYB-ZERO", DiagnosticSeverity::Note, "not accepted"),
            &config,
        );

        assert!(result.is_err());
        assert!(set.is_empty());
    }

    #[test]
    fn canonical_sorting_is_independent_of_insertion_order() {
        let first = diagnostic(
            "HYB-B",
            DiagnosticSeverity::Warning,
            "second location",
        )
        .with_primary_span(
            DiagnosticSpan::new(None, 20, 21, 1, 21, 1, 22)
                .expect("valid span"),
        );

        let second = diagnostic(
            "HYB-A",
            DiagnosticSeverity::Error,
            "first location",
        )
        .with_primary_span(
            DiagnosticSpan::new(None, 1, 2, 1, 2, 1, 3)
                .expect("valid span"),
        );

        let mut left = DiagnosticSet::new();
        left.push(first.clone());
        left.push(second.clone());

        let mut right = DiagnosticSet::new();
        right.push(second);
        right.push(first);

        left.sort_deterministically();
        right.sort_deterministically();

        assert_eq!(left, right);
        assert_eq!(left.as_slice()[0].message(), "first location");
    }

    #[test]
    fn display_output_contains_code_severity_and_message() {
        let rendered = diagnostic(
            "HYB-TEST-1",
            DiagnosticSeverity::Warning,
            "a portability preference cannot be guaranteed",
        )
        .to_string();

        assert!(rendered.contains("warning"));
        assert!(rendered.contains("HYB-TEST-1"));
        assert!(rendered.contains("portability preference"));
    }

    #[test]
    fn serde_round_trip_preserves_structured_diagnostic_data() {
        let original = diagnostic(
            "HYB-TEST-2",
            DiagnosticSeverity::Error,
            "a contract is incompatible",
        )
        .with_frontend_span(frontend_span())
        .with_primary_label("incompatible declaration")
        .with_note("check the domain contract version")
        .with_help("provide a compatible adapter");

        let encoded =
            serde_json::to_string(&original).expect("diagnostic serializes");
        let decoded: Diagnostic =
            serde_json::from_str(&encoded).expect("diagnostic deserializes");

        assert_eq!(decoded, original);
    }
}
