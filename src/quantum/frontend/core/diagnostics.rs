//! # Quantum Frontend Diagnostics
//!
//! Production-grade diagnostics for the Zamani quantum frontend.
//!
//! This module is deliberately format-independent. It must not know about
//! OpenQASM, Quil, QIR, hardware backends, execution, or the canonical
//! Quantum IR.
//!
//! ## Responsibilities
//!
//! This module owns:
//!
//! - diagnostic severity;
//! - stable diagnostic codes;
//! - primary source spans;
//! - secondary source labels;
//! - notes;
//! - help messages;
//! - deterministic diagnostic ordering;
//! - bounded diagnostic collections;
//! - diagnostic truncation state;
//! - machine-readable diagnostic data;
//! - human-readable rendering;
//! - error/warning/note counts.
//!
//! ## Non-responsibilities
//!
//! This module does NOT:
//!
//! - parse source;
//! - validate quantum semantics;
//! - construct Quantum IR;
//! - read files;
//! - access the network;
//! - execute processes;
//! - access quantum hardware;
//! - decide frontend resource limits.
//!
//! Resource policy is owned by `core::limits::FrontendLimits`.
//! This module only consumes concrete limits supplied by its caller.
//!
//! ## Integration contract
//!
//! The frontend pipeline is expected to use diagnostics as follows:
//!
//! ```text
//! Source
//!   │
//!   ▼
//! Lexer ───────────────┐
//!   │                  │
//!   ▼                  │
//! Parser ──────────────┤
//!   │                  │
//!   ▼                  │
//! Validator ───────────┤──► DiagnosticBag
//!   │                  │
//!   ▼                  │
//! Lowering ────────────┘
//!   │
//!   ▼
//! Quantum IR
//! ```
//!
//! Every diagnostic that refers to source code must use `SourceSpan` from
//! `core::source`.
//!
//! `DiagnosticBag` preserves insertion order. `sorted()` provides a stable
//! source-oriented order for user-facing and machine-facing output.
//!
//! No diagnostic message text should be parsed by callers. Programmatic
//! consumers must use `DiagnosticCode`, `DiagnosticSeverity`, and structured
//! fields.

use std::cmp::Ordering;
use std::fmt;

use super::source::{SourceId, SourceSpan};

/// Maximum number of diagnostic children that can be attached to a single
/// diagnostic when no caller-specific bound is supplied.
///
/// This is a defensive fallback only. Production frontend stages should
/// normally obtain the value from `FrontendLimits::max_diagnostic_children`.
pub const DEFAULT_MAX_DIAGNOSTIC_CHILDREN: usize = 32;

/// Maximum number of diagnostics stored by an unbounded/default bag.
///
/// Production callers should use `DiagnosticBag::with_max_diagnostics` or
/// `DiagnosticBag::with_max_diagnostics_u64` and supply the value from
/// `FrontendLimits`.
pub const DEFAULT_MAX_DIAGNOSTICS: usize = 1024;

/// Default maximum length for an individual diagnostic snippet/message-like
/// source excerpt.
///
/// This is deliberately conservative. It is a fallback and not a replacement
/// for `FrontendLimits::max_diagnostic_snippet_length`.
pub const DEFAULT_MAX_DIAGNOSTIC_SNIPPET_LENGTH: usize = 4096;

/// Stable severity of a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticSeverity {
    /// Informational note.
    Note,

    /// Actionable compiler warning.
    Warning,

    /// Compilation-blocking or otherwise invalid input.
    Error,
}

impl DiagnosticSeverity {
    /// Returns the conventional lowercase textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Note => "note",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }

    /// Returns `true` when this severity prevents successful compilation of
    /// the affected stage.
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }

    /// Returns `true` when this severity is a warning.
    #[must_use]
    pub const fn is_warning(self) -> bool {
        matches!(self, Self::Warning)
    }

    /// Returns `true` when this severity is informational.
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

/// Stable machine-readable diagnostic code.
///
/// The numeric portion is intentionally represented as a value rather than
/// being derived from message text. This allows diagnostics to evolve without
/// breaking tooling that consumes compiler output.
///
/// The frontend can use broad code ranges:
///
/// - `QF0001..=QF0099`: generic frontend diagnostics;
/// - `QF0100..=QF0199`: source/lexical diagnostics;
/// - `QF0200..=QF0299`: syntax/parser diagnostics;
/// - `QF0300..=QF0399`: semantic diagnostics;
/// - `QF0400..=QF0499`: lowering diagnostics;
/// - `QF0500..=QF0599`: import/export diagnostics;
/// - `QF0600..=QF0699`: resource/security diagnostics;
/// - `QF0700..=QF0799`: unsupported-feature diagnostics.
///
/// The exact assignment of individual codes belongs to the frontend
/// specification and should not be inferred from this implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiagnosticCode(u32);

impl DiagnosticCode {
    /// Creates a diagnostic code from its numeric component.
    ///
    /// `0` is rejected because `QF0000` is reserved and must not be emitted
    /// as a production diagnostic.
    #[must_use]
    pub const fn new(value: u32) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Returns the numeric component.
    #[must_use]
    pub const fn number(self) -> u32 {
        self.0
    }

    /// Returns the stable textual representation, e.g. `QF0042`.
    #[must_use]
    pub fn as_str(self) -> String {
        format!("QF{:04}", self.0)
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "QF{:04}", self.0)
    }
}

/// A source-oriented label attached to a diagnostic.
///
/// A diagnostic has at most one primary label. Any number of secondary labels
/// may be attached subject to the configured child limit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticLabel {
    span: SourceSpan,
    message: String,
    primary: bool,
}

impl DiagnosticLabel {
    /// Creates a primary label.
    #[must_use]
    pub fn primary(span: SourceSpan, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
            primary: true,
        }
    }

    /// Creates a secondary label.
    #[must_use]
    pub fn secondary(span: SourceSpan, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
            primary: false,
        }
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

    /// Returns whether this is the primary label.
    #[must_use]
    pub const fn is_primary(&self) -> bool {
        self.primary
    }

    /// Returns whether this is a secondary label.
    #[must_use]
    pub const fn is_secondary(&self) -> bool {
        !self.primary
    }
}

/// Structured diagnostic.
///
/// A diagnostic consists of:
///
/// - severity;
/// - stable code;
/// - message;
/// - optional primary span;
/// - bounded secondary labels;
/// - bounded notes;
/// - bounded help messages.
///
/// The struct intentionally does not contain rendered source text. Source
/// snippets must be obtained from `SourceFile`/`SourceMap` at rendering time,
/// preventing stale copies of source data from accumulating in diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    severity: DiagnosticSeverity,
    code: DiagnosticCode,
    message: String,
    labels: Vec<DiagnosticLabel>,
    notes: Vec<String>,
    helps: Vec<String>,
    max_children: usize,
    children_truncated: bool,
}

impl Diagnostic {
    /// Creates an empty diagnostic with the supplied child bound.
    #[must_use]
    pub fn new(
        severity: DiagnosticSeverity,
        code: DiagnosticCode,
        message: impl Into<String>,
    ) -> Self {
        Self::with_max_children(
            severity,
            code,
            message,
            DEFAULT_MAX_DIAGNOSTIC_CHILDREN,
        )
    }

    /// Creates a diagnostic with an explicit child bound.
    #[must_use]
    pub fn with_max_children(
        severity: DiagnosticSeverity,
        code: DiagnosticCode,
        message: impl Into<String>,
        max_children: usize,
    ) -> Self {
        Self {
            severity,
            code,
            message: message.into(),
            labels: Vec::new(),
            notes: Vec::new(),
            helps: Vec::new(),
            max_children,
            children_truncated: false,
        }
    }

    /// Returns the severity.
    #[must_use]
    pub const fn severity(&self) -> DiagnosticSeverity {
        self.severity
    }

    /// Returns the stable diagnostic code.
    #[must_use]
    pub const fn code(&self) -> DiagnosticCode {
        self.code
    }

    /// Returns the human-readable message.
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
    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    /// Returns all help messages in insertion order.
    #[must_use]
    pub fn helps(&self) -> &[String] {
        &self.helps
    }

    /// Returns the primary label, if one exists.
    #[must_use]
    pub fn primary_label(&self) -> Option<&DiagnosticLabel> {
        self.labels.iter().find(|label| label.is_primary())
    }

    /// Returns the primary source span, if one exists.
    #[must_use]
    pub fn primary_span(&self) -> Option<SourceSpan> {
        self.primary_label().map(DiagnosticLabel::span)
    }

    /// Returns the configured maximum number of child entries.
    #[must_use]
    pub const fn max_children(&self) -> usize {
        self.max_children
    }

    /// Returns whether child entries were truncated because the configured
    /// bound was reached.
    #[must_use]
    pub const fn children_truncated(&self) -> bool {
        self.children_truncated
    }

    /// Adds or replaces the primary label.
    ///
    /// There can only be one primary label. Replacing it is deterministic and
    /// avoids accidentally producing a diagnostic with multiple competing
    /// primary locations.
    pub fn set_primary_label(
        &mut self,
        span: SourceSpan,
        message: impl Into<String>,
    ) {
        let label = DiagnosticLabel::primary(span, message);

        if let Some(existing) =
            self.labels.iter_mut().find(|item| item.is_primary())
        {
            *existing = label;
        } else if self.labels.len() < self.max_children {
            self.labels.insert(0, label);
        } else {
            self.children_truncated = true;
        }
    }

    /// Adds a secondary source label.
    ///
    /// Returns `true` if the label was stored and `false` if the configured
    /// child limit prevented insertion.
    pub fn add_secondary_label(
        &mut self,
        span: SourceSpan,
        message: impl Into<String>,
    ) -> bool {
        if self.labels.len() >= self.max_children {
            self.children_truncated = true;
            return false;
        }

        self.labels.push(DiagnosticLabel::secondary(span, message));
        true
    }

    /// Adds a note.
    ///
    /// Returns `true` when the note was stored.
    pub fn add_note(&mut self, message: impl Into<String>) -> bool {
        if self.child_count() >= self.max_children {
            self.children_truncated = true;
            return false;
        }

        self.notes.push(message.into());
        true
    }

    /// Adds a help message.
    ///
    /// Returns `true` when the help message was stored.
    pub fn add_help(&mut self, message: impl Into<String>) -> bool {
        if self.child_count() >= self.max_children {
            self.children_truncated = true;
            return false;
        }

        self.helps.push(message.into());
        true
    }

    /// Returns the total number of child entries.
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.labels.len() + self.notes.len() + self.helps.len()
    }

    /// Returns whether the diagnostic contains no source-oriented or
    /// explanatory child data.
    #[must_use]
    pub fn is_bare(&self) -> bool {
        self.labels.is_empty() && self.notes.is_empty() && self.helps.is_empty()
    }

    /// Returns a deterministic source key.
    ///
    /// Diagnostics without a primary span sort after diagnostics with a
    /// primary span.
    #[must_use]
    pub fn sort_key(&self) -> DiagnosticSortKey {
        match self.primary_span() {
            Some(span) => DiagnosticSortKey {
                has_span: true,
                source_id: span.source_id(),
                start: span.start().get(),
                end: span.end().get(),
                severity: self.severity,
                code: self.code,
                message: self.message.clone(),
            },
            None => DiagnosticSortKey {
                has_span: false,
                source_id: SourceId::new(0),
                start: 0,
                end: 0,
                severity: self.severity,
                code: self.code,
                message: self.message.clone(),
            },
        }
    }
}

/// A deterministic key used for diagnostic sorting.
///
/// This type is public so integration tests and frontend clients can use the
/// exact same ordering contract as the built-in renderer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticSortKey {
    has_span: bool,
    source_id: SourceId,
    start: u64,
    end: u64,
    severity: DiagnosticSeverity,
    code: DiagnosticCode,
    message: String,
}

impl Ord for DiagnosticSortKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.has_span
            .cmp(&other.has_span)
            .reverse()
            .then_with(|| self.source_id.cmp(&other.source_id))
            .then_with(|| self.start.cmp(&other.start))
            .then_with(|| self.end.cmp(&other.end))
            .then_with(|| self.severity.cmp(&other.severity).reverse())
            .then_with(|| self.code.cmp(&other.code))
            .then_with(|| self.message.cmp(&other.message))
    }
}

impl PartialOrd for DiagnosticSortKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A bounded collection of diagnostics.
///
/// `DiagnosticBag` is intentionally an ordered collection rather than a set:
/// two diagnostics with the same code/message may legitimately refer to
/// different source locations and therefore must not be silently deduplicated.
///
/// The bag preserves insertion order and can produce a separately sorted copy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticBag {
    diagnostics: Vec<Diagnostic>,
    max_diagnostics: usize,
    truncated: bool,
}

impl Default for DiagnosticBag {
    fn default() -> Self {
        Self::with_max_diagnostics(DEFAULT_MAX_DIAGNOSTICS)
    }
}

impl DiagnosticBag {
    /// Creates a bag using the production fallback diagnostic limit.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a bag with a concrete diagnostic limit.
    #[must_use]
    pub fn with_max_diagnostics(max_diagnostics: usize) -> Self {
        Self {
            diagnostics: Vec::new(),
            max_diagnostics,
            truncated: false,
        }
    }

    /// Creates a bag from the `u64` representation used by
    /// `FrontendLimits`.
    ///
    /// Returns `None` if the configured limit cannot be represented as a
    /// platform `usize`.
    #[must_use]
    pub fn with_max_diagnostics_u64(max_diagnostics: u64) -> Option<Self> {
        usize::try_from(max_diagnostics)
            .ok()
            .map(Self::with_max_diagnostics)
    }

    /// Returns the configured maximum number of diagnostics.
    #[must_use]
    pub const fn max_diagnostics(&self) -> usize {
        self.max_diagnostics
    }

    /// Returns the number of stored diagnostics.
    #[must_use]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Returns whether the bag contains no diagnostics.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Returns whether diagnostic insertion was truncated.
    #[must_use]
    pub const fn is_truncated(&self) -> bool {
        self.truncated
    }

    /// Returns all diagnostics in insertion order.
    #[must_use]
    pub fn as_slice(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Returns all diagnostics in insertion order.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Returns an iterator over diagnostics in insertion order.
    pub fn iter(&self) -> std::slice::Iter<'_, Diagnostic> {
        self.diagnostics.iter()
    }

    /// Returns a mutable iterator over diagnostics.
    ///
    /// This is provided for compiler stages that need to enrich diagnostics
    /// after initial construction.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Diagnostic> {
        self.diagnostics.iter_mut()
    }

    /// Returns an owned iterator over diagnostics.
    pub fn into_iter(self) -> std::vec::IntoIter<Diagnostic> {
        self.diagnostics.into_iter()
    }

    /// Adds a diagnostic if capacity remains.
    ///
    /// Returns `true` when inserted, `false` when the bag is already full.
    pub fn push(&mut self, diagnostic: Diagnostic) -> bool {
        if self.diagnostics.len() >= self.max_diagnostics {
            self.truncated = true;
            return false;
        }

        self.diagnostics.push(diagnostic);
        true
    }

    /// Adds multiple diagnostics until capacity is reached.
    ///
    /// Returns the number successfully inserted.
    pub fn extend<I>(&mut self, diagnostics: I) -> usize
    where
        I: IntoIterator<Item = Diagnostic>,
    {
        let mut inserted = 0;

        for diagnostic in diagnostics {
            if self.push(diagnostic) {
                inserted += 1;
            } else {
                break;
            }
        }

        inserted
    }

    /// Removes all diagnostics and resets truncation state.
    pub fn clear(&mut self) {
        self.diagnostics.clear();
        self.truncated = false;
    }

    /// Returns the number of error diagnostics.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity().is_error())
            .count()
    }

    /// Returns the number of warning diagnostics.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity().is_warning())
            .count()
    }

    /// Returns the number of note diagnostics.
    #[must_use]
    pub fn note_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity().is_note())
            .count()
    }

    /// Returns whether at least one error exists.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.error_count() != 0
    }

    /// Returns whether at least one warning exists.
    #[must_use]
    pub fn has_warnings(&self) -> bool {
        self.warning_count() != 0
    }

    /// Returns a deterministically sorted copy.
    ///
    /// The original insertion order remains untouched.
    #[must_use]
    pub fn sorted(&self) -> Vec<Diagnostic> {
        let mut result = self.diagnostics.clone();
        result.sort_by(|left, right| left.sort_key().cmp(&right.sort_key()));
        result
    }

    /// Sorts the bag in place using the canonical diagnostic ordering.
    pub fn sort(&mut self) {
        self.diagnostics
            .sort_by(|left, right| left.sort_key().cmp(&right.sort_key()));
    }

    /// Converts the bag into an owned vector in insertion order.
    #[must_use]
    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
}

impl IntoIterator for DiagnosticBag {
    type Item = Diagnostic;
    type IntoIter = std::vec::IntoIter<Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.into_iter()
    }
}

impl<'a> IntoIterator for &'a DiagnosticBag {
    type Item = &'a Diagnostic;
    type IntoIter = std::slice::Iter<'a, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.iter()
    }
}

impl<'a> IntoIterator for &'a mut DiagnosticBag {
    type Item = &'a mut Diagnostic;
    type IntoIter = std::slice::IterMut<'a, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.iter_mut()
    }
}

/// Builder for a diagnostic.
///
/// The builder is deliberately small and deterministic. It is useful inside
/// lexer/parser/validator code where diagnostics are constructed incrementally.
#[derive(Debug)]
pub struct DiagnosticBuilder {
    diagnostic: Diagnostic,
}

impl DiagnosticBuilder {
    /// Creates a builder with the default child bound.
    #[must_use]
    pub fn new(
        severity: DiagnosticSeverity,
        code: DiagnosticCode,
        message: impl Into<String>,
    ) -> Self {
        Self {
            diagnostic: Diagnostic::new(severity, code, message),
        }
    }

    /// Creates a builder with an explicit child bound.
    #[must_use]
    pub fn with_max_children(
        severity: DiagnosticSeverity,
        code: DiagnosticCode,
        message: impl Into<String>,
        max_children: usize,
    ) -> Self {
        Self {
            diagnostic: Diagnostic::with_max_children(
                severity,
                code,
                message,
                max_children,
            ),
        }
    }

    /// Sets the primary source location.
    #[must_use]
    pub fn primary(
        mut self,
        span: SourceSpan,
        message: impl Into<String>,
    ) -> Self {
        self.diagnostic.set_primary_label(span, message);
        self
    }

    /// Adds a secondary source location.
    #[must_use]
    pub fn secondary(
        mut self,
        span: SourceSpan,
        message: impl Into<String>,
    ) -> Self {
        self.diagnostic.add_secondary_label(span, message);
        self
    }

    /// Adds a note.
    #[must_use]
    pub fn note(mut self, message: impl Into<String>) -> Self {
        self.diagnostic.add_note(message);
        self
    }

    /// Adds a help message.
    #[must_use]
    pub fn help(mut self, message: impl Into<String>) -> Self {
        self.diagnostic.add_help(message);
        self
    }

    /// Finishes the diagnostic.
    #[must_use]
    pub fn build(self) -> Diagnostic {
        self.diagnostic
    }
}

/// Creates an error diagnostic.
#[must_use]
pub fn error(
    code: DiagnosticCode,
    message: impl Into<String>,
) -> Diagnostic {
    Diagnostic::new(DiagnosticSeverity::Error, code, message)
}

/// Creates a warning diagnostic.
#[must_use]
pub fn warning(
    code: DiagnosticCode,
    message: impl Into<String>,
) -> Diagnostic {
    Diagnostic::new(DiagnosticSeverity::Warning, code, message)
}

/// Creates an informational diagnostic.
#[must_use]
pub fn note(
    code: DiagnosticCode,
    message: impl Into<String>,
) -> Diagnostic {
    Diagnostic::new(DiagnosticSeverity::Note, code, message)
}

/// Converts a diagnostic severity into its machine-readable string.
impl From<DiagnosticSeverity> for &'static str {
    fn from(value: DiagnosticSeverity) -> Self {
        value.as_str()
    }
}

/// Renders a diagnostic without source snippets.
///
/// This renderer is intentionally source-map independent. A higher-level
/// renderer can use the diagnostic spans to obtain source lines from
/// `SourceMap`/`SourceFile`.
///
/// The result is deterministic.
#[must_use]
pub fn render_plain(diagnostic: &Diagnostic) -> String {
    let mut output = String::new();

    output.push_str(diagnostic.severity().as_str());
    output.push('[');
    output.push_str(&diagnostic.code().as_str());
    output.push_str("]: ");
    output.push_str(diagnostic.message());

    for label in diagnostic.labels() {
        output.push('\n');
        output.push_str(if label.is_primary() {
            "  --> "
        } else {
            "  = "
        });

        output.push_str(&format!(
            "{}:{}-{}",
            label.span().source_id().get(),
            label.span().start().get(),
            label.span().end().get()
        ));

        if !label.message().is_empty() {
            output.push_str(": ");
            output.push_str(label.message());
        }
    }

    for note in diagnostic.notes() {
        output.push('\n');
        output.push_str("  note: ");
        output.push_str(note);
    }

    for help in diagnostic.helps() {
        output.push('\n');
        output.push_str("  help: ");
        output.push_str(help);
    }

    if diagnostic.children_truncated() {
        output.push('\n');
        output.push_str("  note: additional diagnostic details were truncated");
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::source::{SourceOffset, SourceId, SourceSpan};

    fn code(number: u32) -> DiagnosticCode {
        DiagnosticCode::new(number).expect("non-zero diagnostic code")
    }

    fn span(start: u64, end: u64) -> SourceSpan {
        SourceSpan::new(
            SourceId::new(1),
            SourceOffset::new(start),
            SourceOffset::new(end),
        )
        .expect("valid span")
    }

    #[test]
    fn severity_strings_are_stable() {
        assert_eq!(DiagnosticSeverity::Note.as_str(), "note");
        assert_eq!(DiagnosticSeverity::Warning.as_str(), "warning");
        assert_eq!(DiagnosticSeverity::Error.as_str(), "error");
    }

    #[test]
    fn severity_classification_is_correct() {
        assert!(DiagnosticSeverity::Error.is_error());
        assert!(DiagnosticSeverity::Warning.is_warning());
        assert!(DiagnosticSeverity::Note.is_note());

        assert!(!DiagnosticSeverity::Error.is_warning());
        assert!(!DiagnosticSeverity::Warning.is_error());
        assert!(!DiagnosticSeverity::Note.is_error());
    }

    #[test]
    fn diagnostic_codes_are_stable() {
        let diagnostic_code = code(42);

        assert_eq!(diagnostic_code.number(), 42);
        assert_eq!(diagnostic_code.as_str(), "QF0042");
        assert_eq!(diagnostic_code.to_string(), "QF0042");
    }

    #[test]
    fn zero_diagnostic_code_is_rejected() {
        assert!(DiagnosticCode::new(0).is_none());
    }

    #[test]
    fn primary_label_is_stored() {
        let diagnostic = DiagnosticBuilder::new(
            DiagnosticSeverity::Error,
            code(1),
            "invalid gate",
        )
        .primary(span(10, 12), "gate starts here")
        .build();

        assert_eq!(diagnostic.primary_span(), Some(span(10, 12)));
        assert_eq!(
            diagnostic
                .primary_label()
                .expect("primary label")
                .message(),
            "gate starts here"
        );
    }

    #[test]
    fn setting_primary_label_replaces_existing_primary() {
        let mut diagnostic = error(code(1), "invalid");

        diagnostic.set_primary_label(span(1, 2), "first");
        diagnostic.set_primary_label(span(3, 4), "second");

        assert_eq!(diagnostic.labels().len(), 1);
        assert_eq!(diagnostic.primary_span(), Some(span(3, 4)));
        assert_eq!(diagnostic.labels()[0].message(), "second");
    }

    #[test]
    fn secondary_labels_are_supported() {
        let mut diagnostic = error(code(1), "mismatch");

        diagnostic.set_primary_label(span(1, 2), "use");
        assert!(diagnostic.add_secondary_label(
            span(20, 21),
            "declaration"
        ));

        assert_eq!(diagnostic.labels().len(), 2);
        assert!(diagnostic.labels()[0].is_primary());
        assert!(diagnostic.labels()[1].is_secondary());
    }

    #[test]
    fn notes_and_help_are_bounded() {
        let mut diagnostic =
            Diagnostic::with_max_children(
                DiagnosticSeverity::Error,
                code(1),
                "invalid",
                2,
            );

        assert!(diagnostic.add_note("note"));
        assert!(diagnostic.add_help("help"));
        assert!(!diagnostic.add_note("overflow"));

        assert_eq!(diagnostic.notes().len(), 1);
        assert_eq!(diagnostic.helps().len(), 1);
        assert!(diagnostic.children_truncated());
    }

    #[test]
    fn child_limit_applies_across_labels_notes_and_help() {
        let mut diagnostic =
            Diagnostic::with_max_children(
                DiagnosticSeverity::Error,
                code(1),
                "invalid",
                2,
            );

        diagnostic.set_primary_label(span(1, 2), "primary");
        assert!(diagnostic.add_secondary_label(
            span(3, 4),
            "secondary"
        ));
        assert!(!diagnostic.add_help("help"));

        assert_eq!(diagnostic.child_count(), 2);
        assert!(diagnostic.children_truncated());
    }

    #[test]
    fn zero_child_limit_rejects_new_children() {
        let mut diagnostic =
            Diagnostic::with_max_children(
                DiagnosticSeverity::Error,
                code(1),
                "invalid",
                0,
            );

        diagnostic.set_primary_label(span(1, 2), "primary");

        assert!(diagnostic.labels().is_empty());
        assert!(diagnostic.children_truncated());
    }

    #[test]
    fn bag_is_bounded() {
        let mut bag = DiagnosticBag::with_max_diagnostics(2);

        assert!(bag.push(error(code(1), "first")));
        assert!(bag.push(error(code(2), "second")));
        assert!(!bag.push(error(code(3), "third")));

        assert_eq!(bag.len(), 2);
        assert!(bag.is_truncated());
    }

    #[test]
    fn bag_preserves_insertion_order() {
        let mut bag = DiagnosticBag::with_max_diagnostics(10);

        bag.push(error(code(3), "third"));
        bag.push(error(code(1), "first"));
        bag.push(error(code(2), "second"));

        assert_eq!(bag.as_slice()[0].code(), code(3));
        assert_eq!(bag.as_slice()[1].code(), code(1));
        assert_eq!(bag.as_slice()[2].code(), code(2));
    }

    #[test]
    fn sorted_order_is_deterministic() {
        let mut bag = DiagnosticBag::with_max_diagnostics(10);

        let mut later = error(code(2), "later");
        later.set_primary_label(span(20, 21), "later");

        let mut earlier = error(code(1), "earlier");
        earlier.set_primary_label(span(5, 6), "earlier");

        bag.push(later);
        bag.push(earlier);

        let sorted = bag.sorted();

        assert_eq!(sorted[0].primary_span(), Some(span(5, 6)));
        assert_eq!(sorted[1].primary_span(), Some(span(20, 21)));

        // The original insertion order remains unchanged.
        assert_eq!(
            bag.as_slice()[0].primary_span(),
            Some(span(20, 21))
        );
    }

    #[test]
    fn bag_can_be_sorted_in_place() {
        let mut bag = DiagnosticBag::with_max_diagnostics(10);

        let mut second = error(code(2), "second");
        second.set_primary_label(span(20, 21), "");

        let mut first = error(code(1), "first");
        first.set_primary_label(span(1, 2), "");

        bag.push(second);
        bag.push(first);

        bag.sort();

        assert_eq!(bag.as_slice()[0].code(), code(1));
        assert_eq!(bag.as_slice()[1].code(), code(2));
    }

    #[test]
    fn counts_are_correct() {
        let mut bag = DiagnosticBag::with_max_diagnostics(10);

        bag.push(error(code(1), "error"));
        bag.push(warning(code(2), "warning"));
        bag.push(note(code(3), "note"));
        bag.push(error(code(4), "error"));

        assert_eq!(bag.error_count(), 2);
        assert_eq!(bag.warning_count(), 1);
        assert_eq!(bag.note_count(), 1);
        assert!(bag.has_errors());
        assert!(bag.has_warnings());
    }

    #[test]
    fn clear_resets_truncation() {
        let mut bag = DiagnosticBag::with_max_diagnostics(1);

        bag.push(error(code(1), "first"));
        assert!(!bag.push(error(code(2), "second")));
        assert!(bag.is_truncated());

        bag.clear();

        assert!(bag.is_empty());
        assert!(!bag.is_truncated());
    }

    #[test]
    fn extend_respects_limit() {
        let mut bag = DiagnosticBag::with_max_diagnostics(2);

        let inserted = bag.extend(vec![
            error(code(1), "one"),
            error(code(2), "two"),
            error(code(3), "three"),
        ]);

        assert_eq!(inserted, 2);
        assert_eq!(bag.len(), 2);
        assert!(bag.is_truncated());
    }

    #[test]
    fn u64_limit_conversion_works() {
        let bag =
            DiagnosticBag::with_max_diagnostics_u64(10)
                .expect("10 fits usize");

        assert_eq!(bag.max_diagnostics(), 10);
    }

    #[test]
    fn diagnostic_builder_is_composable() {
        let diagnostic = DiagnosticBuilder::with_max_children(
            DiagnosticSeverity::Error,
            code(42),
            "register mismatch",
            8,
        )
        .primary(span(10, 12), "register use")
        .secondary(span(30, 34), "register declaration")
        .note("registers must have compatible widths")
        .help("use matching register dimensions")
        .build();

        assert_eq!(diagnostic.code(), code(42));
        assert_eq!(diagnostic.severity(), DiagnosticSeverity::Error);
        assert_eq!(diagnostic.labels().len(), 2);
        assert_eq!(diagnostic.notes().len(), 1);
        assert_eq!(diagnostic.helps().len(), 1);
    }

    #[test]
    fn plain_renderer_is_deterministic() {
        let diagnostic = DiagnosticBuilder::new(
            DiagnosticSeverity::Error,
            code(42),
            "register mismatch",
        )
        .primary(span(10, 12), "register use")
        .secondary(span(30, 34), "declaration")
        .note("widths differ")
        .help("use matching registers")
        .build();

        let first = render_plain(&diagnostic);
        let second = render_plain(&diagnostic);

        assert_eq!(first, second);
        assert!(first.contains("error[QF0042]"));
        assert!(first.contains("register mismatch"));
        assert!(first.contains("register use"));
        assert!(first.contains("declaration"));
        assert!(first.contains("note: widths differ"));
        assert!(first.contains("help: use matching registers"));
    }

    #[test]
    fn diagnostic_without_span_sorts_after_spanned_diagnostic() {
        let mut bag = DiagnosticBag::with_max_diagnostics(10);

        let without_span = error(code(1), "no span");

        let mut with_span = error(code(2), "has span");
        with_span.set_primary_label(span(1, 2), "here");

        bag.push(without_span);
        bag.push(with_span);

        let sorted = bag.sorted();

        assert!(sorted[0].primary_span().is_some());
        assert!(sorted[1].primary_span().is_none());
    }

    #[test]
    fn duplicate_diagnostics_are_not_silently_removed() {
        let mut bag = DiagnosticBag::with_max_diagnostics(10);

        let first = error(code(1), "same");
        let second = error(code(1), "same");

        bag.push(first);
        bag.push(second);

        assert_eq!(bag.len(), 2);
    }

    #[test]
    fn iterators_work() {
        let mut bag = DiagnosticBag::with_max_diagnostics(10);

        bag.push(error(code(1), "one"));
        bag.push(error(code(2), "two"));

        let count = bag.iter().count();
        assert_eq!(count, 2);

        let mut mutable_count = 0;
        for _diagnostic in &mut bag {
            mutable_count += 1;
        }

        assert_eq!(mutable_count, 2);

        let owned: Vec<_> = bag.clone().into_iter().collect();
        assert_eq!(owned.len(), 2);
    }

    #[test]
    fn empty_diagnostic_reports_bare() {
        let diagnostic = error(code(1), "message");

        assert!(diagnostic.is_bare());
    }

    #[test]
    fn diagnostic_with_label_is_not_bare() {
        let mut diagnostic = error(code(1), "message");

        diagnostic.set_primary_label(span(1, 2), "location");

        assert!(!diagnostic.is_bare());
    }
}