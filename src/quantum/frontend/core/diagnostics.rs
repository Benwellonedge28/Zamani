//! Zamani Quantum Frontend — deterministic diagnostics.
//!
//! This module provides the format-independent diagnostic model shared by
//! OpenQASM, QIR, Quil, and future quantum frontends.
//!
//! # Architectural boundary
//!
//! `diagnostics.rs` owns reporting data, not language semantics. It knows
//! about source spans, severity, stable diagnostic codes, labels, notes, and
//! help text, but it does not know anything about OpenQASM, QIR, Quil, or the
//! canonical Quantum IR.
//!
//! The dependency direction is:
//!
//! ```text
//! lexer / parser / semantic validator
//!                |
//!                v
//!       +--------------------+
//!       | DiagnosticBuilder  |
//!       | Diagnostic         |
//!       | DiagnosticBag      |
//!       +---------+----------+
//!                 |
//!                 v
//!          SourceSpan / source.rs
//! ```
//!
//! Errors and diagnostics deliberately remain separate:
//!
//! - errors represent control-flow/API failure;
//! - diagnostics represent structured user-facing information.
//!
//! This prevents the diagnostic system from becoming coupled to a particular
//! error hierarchy and keeps all frontend formats independently removable.
//!
//! # Determinism
//!
//! `DiagnosticBag` preserves insertion order for normal iteration and provides
//! `sorted()` for deterministic presentation independent of producer order.
//!
//! Sorting never depends on:
//!
//! - hash-map iteration;
//! - timestamps;
//! - randomness;
//! - filesystem ordering;
//! - thread scheduling;
//! - external I/O.
//!
//! # Resource safety
//!
//! `DiagnosticBag::with_max_diagnostics` provides an explicit diagnostic-count
//! bound. Once the bound is reached, additional diagnostics are rejected and
//! the bag records that truncation occurred.
//!
//! The diagnostic-count policy is deliberately passed as a primitive rather
//! than importing `FrontendLimits`. The generic limits layer owns policy;
//! this module owns diagnostic behavior. The future frontend orchestrator can
//! pass `FrontendLimits::max_diagnostics()` without changing this file.
//!
//! # Source-location contract
//!
//! All locations use the canonical `SourceSpan` from `core::source`.
//!
//! No frontend format may define another source-location type.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1, Rust 2021.
//!
//! No nightly features.
//! No external crates.

use std::cmp::Ordering;
use std::fmt;

use super::source::SourceSpan;

// =============================================================================
// Stable diagnostic codes
// =============================================================================

/// Stable machine-readable diagnostic code.
///
/// Codes are strings rather than a global enum because independently
/// removable frontends must be able to own their namespaces.
///
/// Examples:
///
/// - `QASM-E001`
/// - `QASM-W001`
/// - `QIR-E001`
/// - `QUIL-E001`
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DiagnosticCode(String);

impl DiagnosticCode {
    /// Creates a diagnostic code.
    ///
    /// A code must:
    ///
    /// - be non-empty;
    /// - contain only ASCII letters, digits, `-`, `_`, or `.`.
    ///
    /// Restricting the syntax keeps codes stable and easy to consume from
    /// tooling, logs, JSON, LSP clients, and CI systems.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();

        if value.is_empty() || !is_valid_code(&value) {
            return None;
        }

        Some(Self(value))
    }

    /// Returns the machine-readable code.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for DiagnosticCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Severity
// =============================================================================

/// Diagnostic severity.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiagnosticSeverity {
    /// Informational information.
    Note,

    /// A suspicious or recoverable condition.
    Warning,

    /// A condition that makes the requested operation invalid.
    Error,
}

impl DiagnosticSeverity {
    /// Returns the stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Note => "note",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    /// Returns whether this is an error.
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }

    /// Returns whether this is a warning.
    #[must_use]
    pub const fn is_warning(self) -> bool {
        matches!(self, Self::Warning)
    }

    /// Returns whether this is a note.
    #[must_use]
    pub const fn is_note(self) -> bool {
        matches!(self, Self::Note)
    }
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Label kinds
// =============================================================================

/// Whether a source label is primary or contextual.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiagnosticLabelKind {
    /// The main location associated with the diagnostic.
    Primary,

    /// Additional contextual location.
    Secondary,
}

impl DiagnosticLabelKind {
    /// Returns the stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
        }
    }
}

// =============================================================================
// Diagnostic labels
// =============================================================================

/// A source-attached diagnostic label.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DiagnosticLabel {
    kind: DiagnosticLabelKind,
    span: SourceSpan,
    message: String,
}

impl DiagnosticLabel {
    /// Creates a primary label.
    #[must_use]
    pub fn primary(
        span: SourceSpan,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind: DiagnosticLabelKind::Primary,
            span,
            message: message.into(),
        }
    }

    /// Creates a secondary label.
    #[must_use]
    pub fn secondary(
        span: SourceSpan,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind: DiagnosticLabelKind::Secondary,
            span,
            message: message.into(),
        }
    }

    /// Returns the label kind.
    #[must_use]
    pub const fn kind(&self) -> DiagnosticLabelKind {
        self.kind
    }

    /// Returns the source span.
    #[must_use]
    pub const fn span(&self) -> SourceSpan {
        self.span
    }

    /// Returns the label message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// =============================================================================
// Notes
// =============================================================================

/// Additional explanatory information attached to a diagnostic.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DiagnosticNote(String);

impl DiagnosticNote {
    /// Creates a diagnostic note.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }

    /// Returns the note text.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.0
    }
}

// =============================================================================
// Help
// =============================================================================

/// Actionable help attached to a diagnostic.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DiagnosticHelp(String);

impl DiagnosticHelp {
    /// Creates a help message.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }

    /// Returns the help text.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.0
    }
}

// =============================================================================
// Diagnostic
// =============================================================================

/// Complete structured frontend diagnostic.
///
/// A diagnostic may have no labels for global conditions. When a meaningful
/// source location exists, frontends should provide exactly one primary label
/// and may provide any number of secondary labels.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Diagnostic {
    severity: DiagnosticSeverity,
    code: DiagnosticCode,
    message: String,
    labels: Vec<DiagnosticLabel>,
    notes: Vec<DiagnosticNote>,
    helps: Vec<DiagnosticHelp>,
}

impl Diagnostic {
    /// Creates a diagnostic with no labels, notes, or help.
    #[must_use]
    pub fn new(
        severity: DiagnosticSeverity,
        code: DiagnosticCode,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code,
            message: message.into(),
            labels: Vec::new(),
            notes: Vec::new(),
            helps: Vec::new(),
        }
    }

    /// Returns the severity.
    #[must_use]
    pub const fn severity(&self) -> DiagnosticSeverity {
        self.severity
    }

    /// Returns the stable code.
    #[must_use]
    pub fn code(&self) -> &DiagnosticCode {
        &self.code
    }

    /// Returns the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns all labels in insertion order.
    #[must_use]
    pub fn labels(&self) -> &[DiagnosticLabel] {
        &self.labels
    }

    /// Returns all notes in insertion order.
    #[must_use]
    pub fn notes(&self) -> &[DiagnosticNote] {
        &self.notes
    }

    /// Returns all help messages in insertion order.
    #[must_use]
    pub fn helps(&self) -> &[DiagnosticHelp] {
        &self.helps
    }

    /// Returns the primary label, if one exists.
    #[must_use]
    pub fn primary_label(&self) -> Option<&DiagnosticLabel> {
        self.labels
            .iter()
            .find(|label| {
                label.kind() == DiagnosticLabelKind::Primary
            })
    }

    /// Returns the primary source span, if one exists.
    #[must_use]
    pub fn primary_span(&self) -> Option<SourceSpan> {
        self.primary_label()
            .map(DiagnosticLabel::span)
    }

    /// Returns whether this diagnostic is an error.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        self.severity.is_error()
    }

    /// Returns whether this diagnostic is a warning.
    #[must_use]
    pub const fn is_warning(&self) -> bool {
        self.severity.is_warning()
    }

    /// Returns whether this diagnostic is a note.
    #[must_use]
    pub const fn is_note(&self) -> bool {
        self.severity.is_note()
    }

    /// Creates a diagnostic builder.
    #[must_use]
    pub fn builder(
        severity: DiagnosticSeverity,
        code: DiagnosticCode,
        message: impl Into<String>,
    ) -> DiagnosticBuilder {
        DiagnosticBuilder {
            diagnostic: Self::new(severity, code, message),
        }
    }

    fn add_label(
        &mut self,
        label: DiagnosticLabel,
    ) -> Result<(), DiagnosticBuildError> {
        if label.kind() == DiagnosticLabelKind::Primary
            && self.primary_label().is_some()
        {
            return Err(
                DiagnosticBuildError::MultiplePrimaryLabels
            );
        }

        self.labels.push(label);
        Ok(())
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}[{}]: {}",
            self.severity,
            self.code,
            self.message
        )?;

        for label in &self.labels {
            write!(
                formatter,
                "\n  {} {}: {}",
                label.kind().as_str(),
                label.span(),
                label.message()
            )?;
        }

        for note in &self.notes {
            write!(
                formatter,
                "\n  note: {}",
                note.message()
            )?;
        }

        for help in &self.helps {
            write!(
                formatter,
                "\n  help: {}",
                help.message()
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Diagnostic builder
// =============================================================================

/// Error produced when a diagnostic violates its structural invariants.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiagnosticBuildError {
    /// More than one primary label was supplied.
    MultiplePrimaryLabels,
}

impl fmt::Display for DiagnosticBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MultiplePrimaryLabels => formatter.write_str(
                "a diagnostic may contain at most one primary label",
            ),
        }
    }
}

impl std::error::Error for DiagnosticBuildError {}

/// Builder enforcing diagnostic invariants.
#[derive(Clone, Debug)]
pub struct DiagnosticBuilder {
    diagnostic: Diagnostic,
}

impl DiagnosticBuilder {
    /// Adds the primary source label.
    ///
    /// The operation fails if a primary label already exists.
    pub fn primary(
        mut self,
        span: SourceSpan,
        message: impl Into<String>,
    ) -> Result<Self, DiagnosticBuildError> {
        self.diagnostic.add_label(
            DiagnosticLabel::primary(span, message),
        )?;

        Ok(self)
    }

    /// Adds a secondary source label.
    #[must_use]
    pub fn secondary(
        mut self,
        span: SourceSpan,
        message: impl Into<String>,
    ) -> Self {
        self.diagnostic
            .labels
            .push(DiagnosticLabel::secondary(
                span,
                message,
            ));

        self
    }

    /// Adds an explanatory note.
    #[must_use]
    pub fn note(
        mut self,
        message: impl Into<String>,
    ) -> Self {
        self.diagnostic
            .notes
            .push(DiagnosticNote::new(message));

        self
    }

    /// Adds actionable help.
    #[must_use]
    pub fn help(
        mut self,
        message: impl Into<String>,
    ) -> Self {
        self.diagnostic
            .helps
            .push(DiagnosticHelp::new(message));

        self
    }

    /// Finalizes the diagnostic.
    #[must_use]
    pub fn build(self) -> Diagnostic {
        self.diagnostic
    }
}

// =============================================================================
// Diagnostic bag
// =============================================================================

/// Error returned when a diagnostic cannot be added to a bounded bag.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiagnosticPushError {
    /// The configured diagnostic limit has been reached.
    LimitExceeded,
}

impl fmt::Display for DiagnosticPushError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LimitExceeded => {
                formatter.write_str("diagnostic limit exceeded")
            }
        }
    }
}

impl std::error::Error for DiagnosticPushError {}

/// Ordered collection of diagnostics belonging to one frontend operation.
///
/// The bag preserves insertion order for normal processing while exposing
/// `sorted()` for deterministic presentation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DiagnosticBag {
    diagnostics: Vec<Diagnostic>,
    max_diagnostics: Option<usize>,
    truncated: bool,
}

impl DiagnosticBag {
    /// Creates an unbounded diagnostic bag.
    ///
    /// At an untrusted frontend boundary, prefer
    /// `with_max_diagnostics`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            max_diagnostics: None,
            truncated: false,
        }
    }

    /// Creates a bounded diagnostic bag.
    ///
    /// `0` is valid and means that no diagnostic can be retained.
    #[must_use]
    pub const fn with_max_diagnostics(
        max_diagnostics: usize,
    ) -> Self {
        Self {
            diagnostics: Vec::new(),
            max_diagnostics: Some(max_diagnostics),
            truncated: false,
        }
    }

    /// Returns the configured maximum, if bounded.
    #[must_use]
    pub const fn max_diagnostics(&self) -> Option<usize> {
        self.max_diagnostics
    }

    /// Returns the number of retained diagnostics.
    #[must_use]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Returns whether the bag contains no retained diagnostics.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Returns whether at least one diagnostic was rejected because the
    /// configured limit was reached.
    #[must_use]
    pub const fn is_truncated(&self) -> bool {
        self.truncated
    }

    /// Returns retained diagnostics in insertion order.
    #[must_use]
    pub fn as_slice(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Returns an insertion-order iterator.
    pub fn iter(
        &self,
    ) -> std::slice::Iter<'_, Diagnostic> {
        self.diagnostics.iter()
    }

    /// Adds one diagnostic.
    pub fn push(
        &mut self,
        diagnostic: Diagnostic,
    ) -> Result<(), DiagnosticPushError> {
        if let Some(maximum) = self.max_diagnostics {
            if self.diagnostics.len() >= maximum {
                self.truncated = true;
                return Err(
                    DiagnosticPushError::LimitExceeded
                );
            }
        }

        self.diagnostics.push(diagnostic);
        Ok(())
    }

    /// Attempts to add a diagnostic.
    ///
    /// Returns `false` when the configured limit has been reached.
    ///
    /// This is useful for parser recovery code where exceeding the diagnostic
    /// limit must not replace the original syntax/semantic failure.
    pub fn push_or_truncate(
        &mut self,
        diagnostic: Diagnostic,
    ) -> bool {
        self.push(diagnostic).is_ok()
    }

    /// Returns whether at least one retained diagnostic is an error.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(Diagnostic::is_error)
    }

    /// Returns whether at least one retained diagnostic is a warning.
    #[must_use]
    pub fn has_warnings(&self) -> bool {
        self.diagnostics.iter().any(Diagnostic::is_warning)
    }

    /// Returns a deterministically ordered copy.
    ///
    /// Ordering is:
    ///
    /// 1. diagnostics with primary source locations;
    /// 2. source ID;
    /// 3. start offset;
    /// 4. end offset;
    /// 5. severity;
    /// 6. code;
    /// 7. message;
    /// 8. labels;
    /// 9. notes;
    /// 10. help.
    ///
    /// The original insertion order remains unchanged.
    #[must_use]
    pub fn sorted(&self) -> Vec<Diagnostic> {
        let mut result = self.diagnostics.clone();
        result.sort_by(compare_diagnostics);
        result
    }

    /// Removes all diagnostics while retaining the configured limit.
    pub fn clear(&mut self) {
        self.diagnostics.clear();
        self.truncated = false;
    }
}

impl<'a> IntoIterator for &'a DiagnosticBag {
    type Item = &'a Diagnostic;
    type IntoIter = std::slice::Iter<'a, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.iter()
    }
}

// =============================================================================
// Deterministic ordering
// =============================================================================

fn compare_diagnostics(
    left: &Diagnostic,
    right: &Diagnostic,
) -> Ordering {
    match (
        left.primary_span(),
        right.primary_span(),
    ) {
        (Some(left_span), Some(right_span)) => {
            left_span
                .source_id()
                .cmp(&right_span.source_id())
                .then_with(|| {
                    left_span
                        .start()
                        .cmp(&right_span.start())
                })
                .then_with(|| {
                    left_span
                        .end()
                        .cmp(&right_span.end())
                })
        }

        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
    .then_with(|| {
        left.severity.cmp(&right.severity)
    })
    .then_with(|| left.code.cmp(&right.code))
    .then_with(|| left.message.cmp(&right.message))
    .then_with(|| left.labels.cmp(&right.labels))
    .then_with(|| left.notes.cmp(&right.notes))
    .then_with(|| left.helps.cmp(&right.helps))
}

// =============================================================================
// Helpers
// =============================================================================

fn is_valid_code(value: &str) -> bool {
    value.bytes().all(|byte| {
        byte.is_ascii_alphanumeric()
            || matches!(byte, b'-' | b'_' | b'.')
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::frontend::core::source::SourceId;

    fn code(value: &str) -> DiagnosticCode {
        DiagnosticCode::new(value)
            .expect("test diagnostic code must be valid")
    }

    fn span(start: usize, end: usize) -> SourceSpan {
        SourceSpan::new(
            SourceId::from_raw(0),
            start,
            end,
        )
        .expect("test span must be ordered")
    }

    #[test]
    fn diagnostic_code_rejects_invalid_values() {
        assert!(DiagnosticCode::new("").is_none());
        assert!(
            DiagnosticCode::new("QASM E001").is_none()
        );
        assert!(
            DiagnosticCode::new("QASM-E001").is_some()
        );
        assert!(
            DiagnosticCode::new("QASM_E001.v1").is_some()
        );
    }

    #[test]
    fn severity_is_machine_readable() {
        assert_eq!(
            DiagnosticSeverity::Error.as_str(),
            "error"
        );
        assert_eq!(
            DiagnosticSeverity::Warning.as_str(),
            "warning"
        );
        assert_eq!(
            DiagnosticSeverity::Note.as_str(),
            "note"
        );

        assert!(DiagnosticSeverity::Error.is_error());
        assert!(
            DiagnosticSeverity::Warning.is_warning()
        );
        assert!(DiagnosticSeverity::Note.is_note());
    }

    #[test]
    fn builder_enforces_one_primary_label() {
        let first = Diagnostic::builder(
            DiagnosticSeverity::Error,
            code("QASM-E001"),
            "invalid gate",
        )
        .primary(span(0, 1), "gate")
        .expect(
            "first primary label must be accepted",
        );

        let second =
            first.primary(span(1, 2), "second");

        assert_eq!(
            second,
            Err(
                DiagnosticBuildError::
                    MultiplePrimaryLabels
            )
        );
    }

    #[test]
    fn builder_preserves_structured_context() {
        let diagnostic = Diagnostic::builder(
            DiagnosticSeverity::Error,
            code("QASM-E002"),
            "unknown identifier",
        )
        .primary(
            span(2, 4),
            "unknown identifier",
        )
        .expect(
            "primary label must be accepted",
        )
        .secondary(
            span(0, 1),
            "declaration site",
        )
        .note(
            "the identifier must be declared before use",
        )
        .help(
            "declare the register before referencing it",
        )
        .build();

        assert_eq!(diagnostic.labels().len(), 2);
        assert_eq!(diagnostic.notes().len(), 1);
        assert_eq!(diagnostic.helps().len(), 1);
        assert_eq!(
            diagnostic.primary_span(),
            Some(span(2, 4))
        );
    }

    #[test]
    fn display_is_deterministic() {
        let diagnostic = Diagnostic::builder(
            DiagnosticSeverity::Error,
            code("QASM-E003"),
            "invalid operand",
        )
        .primary(span(3, 4), "operand")
        .expect(
            "primary label must be accepted",
        )
        .secondary(span(0, 1), "gate")
        .note("two qubits are required")
        .help("provide the missing operand")
        .build();

        assert_eq!(
            diagnostic.to_string(),
            "error[QASM-E003]: invalid operand\n\
             primary source:0:3..4: operand\n\
             secondary source:0:0..1: gate\n\
             note: two qubits are required\n\
             help: provide the missing operand"
        );
    }

    #[test]
    fn bounded_bag_rejects_after_limit() {
        let mut bag =
            DiagnosticBag::with_max_diagnostics(1);

        let first = Diagnostic::new(
            DiagnosticSeverity::Error,
            code("QASM-E001"),
            "first",
        );

        let second = Diagnostic::new(
            DiagnosticSeverity::Error,
            code("QASM-E002"),
            "second",
        );

        assert!(bag.push(first).is_ok());

        assert_eq!(
            bag.push(second),
            Err(
                DiagnosticPushError::LimitExceeded
            )
        );

        assert_eq!(bag.len(), 1);
        assert!(bag.is_truncated());
    }

    #[test]
    fn zero_limit_rejects_all_diagnostics() {
        let mut bag =
            DiagnosticBag::with_max_diagnostics(0);

        let diagnostic = Diagnostic::new(
            DiagnosticSeverity::Error,
            code("QASM-E001"),
            "error",
        );

        assert_eq!(
            bag.push(diagnostic),
            Err(
                DiagnosticPushError::LimitExceeded
            )
        );

        assert!(bag.is_empty());
        assert!(bag.is_truncated());
    }

    #[test]
    fn bag_reports_error_and_warning_presence() {
        let mut bag = DiagnosticBag::new();

        assert!(!bag.has_errors());
        assert!(!bag.has_warnings());

        bag.push(Diagnostic::new(
            DiagnosticSeverity::Warning,
            code("QASM-W001"),
            "warning",
        ))
        .expect(
            "unbounded bag must accept warning",
        );

        assert!(!bag.has_errors());
        assert!(bag.has_warnings());

        bag.push(Diagnostic::new(
            DiagnosticSeverity::Error,
            code("QASM-E001"),
            "error",
        ))
        .expect(
            "unbounded bag must accept error",
        );

        assert!(bag.has_errors());
    }

    #[test]
    fn sorted_order_is_deterministic() {
        let mut bag = DiagnosticBag::new();

        let later = Diagnostic::builder(
            DiagnosticSeverity::Error,
            code("QASM-E002"),
            "later",
        )
        .primary(span(20, 21), "later")
        .expect(
            "primary label must be accepted",
        )
        .build();

        let earlier = Diagnostic::builder(
            DiagnosticSeverity::Error,
            code("QASM-E001"),
            "earlier",
        )
        .primary(span(2, 3), "earlier")
        .expect(
            "primary label must be accepted",
        )
        .build();

        bag.push(later)
            .expect("bag must accept later diagnostic");

        bag.push(earlier)
            .expect(
                "bag must accept earlier diagnostic",
            );

        let sorted = bag.sorted();

        assert_eq!(
            sorted[0].code().as_str(),
            "QASM-E001"
        );
        assert_eq!(
            sorted[1].code().as_str(),
            "QASM-E002"
        );

        // Original insertion order remains unchanged.
        assert_eq!(
            bag.as_slice()[0].code().as_str(),
            "QASM-E002"
        );
    }

    #[test]
    fn source_span_is_canonical_location_type() {
        let label =
            DiagnosticLabel::primary(
                span(1, 2),
                "token",
            );

        assert_eq!(
            label.span().source_id(),
            SourceId::from_raw(0)
        );
        assert_eq!(label.span().start(), 1);
        assert_eq!(label.span().end(), 2);
    }

    #[test]
    fn clear_resets_truncation_state() {
        let mut bag =
            DiagnosticBag::with_max_diagnostics(0);

        let diagnostic = Diagnostic::new(
            DiagnosticSeverity::Error,
            code("QASM-E001"),
            "error",
        );

        assert!(
            bag.push(diagnostic).is_err()
        );
        assert!(bag.is_truncated());

        bag.clear();

        assert!(bag.is_empty());
        assert!(!bag.is_truncated());
        assert_eq!(
            bag.max_diagnostics(),
            Some(0)
        );
    }
}