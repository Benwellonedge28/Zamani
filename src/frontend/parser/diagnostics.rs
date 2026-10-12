
#![forbid(unsafe_code)]

//! Structured parser diagnostics for the Zamani frontend.
//!
//! File: `src/frontend/parser/diagnostics.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Responsibilities
//!
//! - Convert canonical parser errors into structured diagnostics.
//! - Preserve stable codes, severity, source spans, and grammar contexts.
//! - Support secondary source labels, notes, and remediation suggestions.
//! - Retain diagnostics in deterministic reporting order.
//! - Bound diagnostic retention using the shared ResourceLimit contract.
//! - Track dropped diagnostics without silently replacing earlier entries.
//!
//! This module does not own lexical scanning, parser recovery, source
//! coordinate calculation, semantic analysis, or target realization.
//!
//! # Integration contract
//!
//! - `error.rs` owns ParseError, ParseErrorKind, and ParseErrorClass.
//! - `limits.rs` re-exports the canonical ResourceLimit type.
//! - `lexer/span.rs` supplies the canonical source Span.
//! - `session.rs` owns the collection for each parse invocation.
//! - `recovery.rs` decides whether and how parsing continues.
//! - `result.rs` returns retained diagnostics and truncation metadata.
//! - `api.rs` exposes the approved public diagnostic interface.
//! - `grammar/spec/diagnostics.md` governs diagnostic compatibility.
//!
//! Parser codes remain distinct from the lexer diagnostic registry.
//! Introducing a shared frontend diagnostic type must be an explicit,
//! reviewed integration change.
//!
//! No hardware-capacity or language-level limits are introduced here.
//! Unlimited retention does not imply unlimited physical memory.

use core::fmt;

use super::error::{ParseError, ParseErrorClass, ParseErrorKind};
use super::limits::ResourceLimit;
use super::super::lexer::span::Span;

/// Severity of a parser diagnostic.
///
/// Severity describes the report; it does not determine recovery behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ParserDiagnosticSeverity {
    /// Context that may explain another diagnostic.
    Note,
    /// A non-fatal condition worth reporting.
    Warning,
    /// A source or parser error.
    Error,
    /// Parsing cannot continue safely or meaningfully.
    Fatal,
}

impl ParserDiagnosticSeverity {
    /// Whether this severity represents an error condition.
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error | Self::Fatal)
    }

    /// Whether this severity represents a fatal condition.
    #[must_use]
    pub const fn is_fatal(self) -> bool {
        matches!(self, Self::Fatal)
    }
}

impl fmt::Display for ParserDiagnosticSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Note => "note",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Fatal => "fatal error",
        })
    }
}

/// A secondary source label attached to a diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParserDiagnosticLabel {
    span: Span,
    message: String,
}

impl ParserDiagnosticLabel {
    /// Create a source label.
    #[must_use]
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }

    /// Return the canonical source span.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Return the explanatory text.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// One structured parser diagnostic.
///
/// The primary span is optional because configuration and integration
/// failures may occur before a usable source position exists.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParserDiagnostic {
    code: String,
    severity: ParserDiagnosticSeverity,
    message: String,
    primary_span: Option<Span>,
    labels: Vec<ParserDiagnosticLabel>,
    notes: Vec<String>,
    help: Option<String>,
}

impl ParserDiagnostic {
    /// Construct a diagnostic from its core fields.
    ///
    /// The code is text so extension diagnostics can use their own stable
    /// identifiers without requiring an exhaustive central enum.
    #[must_use]
    pub fn new(
        code: impl Into<String>,
        severity: ParserDiagnosticSeverity,
        message: impl Into<String>,
        primary_span: Option<Span>,
    ) -> Self {
        Self {
            code: code.into(),
            severity,
            message: message.into(),
            primary_span,
            labels: Vec::new(),
            notes: Vec::new(),
            help: None,
        }
    }

    /// Convert a canonical parser error into a diagnostic.
    ///
    /// The original ParseError remains authoritative for programmatic
    /// inspection of the structured error kind and classification.
    #[must_use]
    pub fn from_error(error: &ParseError) -> Self {
        let severity = match error.class() {
            ParseErrorClass::Recoverable => ParserDiagnosticSeverity::Error,
            ParseErrorClass::Fatal | ParseErrorClass::Internal => {
                ParserDiagnosticSeverity::Fatal
            }
        };

        let mut diagnostic = Self::new(
            error.stable_code(),
            severity,
            error.to_string(),
            error.span(),
        );

        for context in error.contexts() {
            diagnostic
                .notes
                .push(format!("while parsing {context}"));
        }

        if matches!(error.kind(), ParseErrorKind::ResourceLimit(_)) {
            diagnostic.notes.push(
                "This is a configurable parser resource budget, not a \
                 language-level or hardware-capacity limit."
                    .to_owned(),
            );
        }

        if error.class() == ParseErrorClass::Internal {
            diagnostic.notes.push(
                "This indicates an implementation or integration failure; \
                 include it in a bug report."
                    .to_owned(),
            );
        }

        diagnostic
    }

    /// Return the stable machine-readable code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Return the severity.
    #[must_use]
    pub const fn severity(&self) -> ParserDiagnosticSeverity {
        self.severity
    }

    /// Return the human-readable message.
    ///
    /// Consumers should not parse this text to make programmatic decisions.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the primary source span, if available.
    #[must_use]
    pub const fn primary_span(&self) -> Option<Span> {
        self.primary_span
    }

    /// Return secondary labels in insertion order.
    #[must_use]
    pub fn labels(&self) -> &[ParserDiagnosticLabel] {
        &self.labels
    }

    /// Return supplemental notes in insertion order.
    #[must_use]
    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    /// Return the optional remediation suggestion.
    #[must_use]
    pub fn help(&self) -> Option<&str> {
        self.help.as_deref()
    }

    /// Whether the diagnostic represents an error.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        self.severity.is_error()
    }

    /// Add a secondary source label.
    #[must_use]
    pub fn with_label(
        mut self,
        span: Span,
        message: impl Into<String>,
    ) -> Self {
        self.labels
            .push(ParserDiagnosticLabel::new(span, message));
        self
    }

    /// Add an explanatory note.
    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Add a remediation suggestion.
    #[must_use]
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }
}

impl fmt::Display for ParserDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}]: {}", self.severity, self.code, self.message)?;

        if let Some(help) = &self.help {
            write!(f, "\n  help: {help}")?;
        }

        for note in &self.notes {
            write!(f, "\n  note: {note}")?;
        }

        Ok(())
    }
}

/// A deterministic collection of parser diagnostics with optional
/// retention limits.
///
/// Diagnostics remain in reporting order. When the retention limit is
/// reached, new diagnostics are dropped and counted; earlier diagnostics
/// are never silently replaced. The dropped counter saturates rather than
/// overflowing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticCollection {
    limit: ResourceLimit,
    diagnostics: Vec<ParserDiagnostic>,
    dropped_count: usize,
}

impl Default for DiagnosticCollection {
    /// Create a collection without an explicit retention budget.
    ///
    /// Applications parsing untrusted input should use `with_limit`.
    fn default() -> Self {
        Self::with_limit(ResourceLimit::Unlimited)
    }
}

impl DiagnosticCollection {
    /// Create an empty collection with the supplied retention policy.
    #[must_use]
    pub const fn with_limit(limit: ResourceLimit) -> Self {
        Self {
            limit,
            diagnostics: Vec::new(),
            dropped_count: 0,
        }
    }

    /// Attempt to retain a diagnostic.
    ///
    /// Returns `true` if retained, or `false` if the limit prevents
    /// retention. Allocation failure remains subject to the host's
    /// allocation behavior; this method does not promise recovery from OOM.
    pub fn push(&mut self, diagnostic: ParserDiagnostic) -> bool {
        let Some(next_len) = self.diagnostics.len().checked_add(1) else {
            self.note_dropped();
            return false;
        };

        if !self.limit.permits(next_len) {
            self.note_dropped();
            return false;
        }

        self.diagnostics.push(diagnostic);
        true
    }

    /// Convert a parser error and attempt to retain its diagnostic.
    pub fn push_error(&mut self, error: &ParseError) -> bool {
        self.push(ParserDiagnostic::from_error(error))
    }

    /// Return retained diagnostics in reporting order.
    #[must_use]
    pub fn diagnostics(&self) -> &[ParserDiagnostic] {
        &self.diagnostics
    }

    /// Return the number of diagnostics that could not be retained.
    #[must_use]
    pub const fn dropped_count(&self) -> usize {
        self.dropped_count
    }

    /// Whether at least one diagnostic was dropped.
    #[must_use]
    pub const fn is_truncated(&self) -> bool {
        self.dropped_count > 0
    }

    /// Return the configured retention policy.
    #[must_use]
    pub const fn limit(&self) -> ResourceLimit {
        self.limit
    }

    /// Return the number of retained diagnostics.
    #[must_use]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Whether no diagnostics have been retained.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Whether any retained diagnostic has error severity.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(ParserDiagnostic::is_error)
    }

    /// Consume the collection, returning retained diagnostics and the
    /// number of diagnostics dropped.
    #[must_use]
    pub fn into_parts(self) -> (Vec<ParserDiagnostic>, usize) {
        (self.diagnostics, self.dropped_count)
    }

    fn note_dropped(&mut self) {
        self.dropped_count = self.dropped_count.saturating_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::parser::error::{
        ParseErrorClass, ParseErrorKind,
    };

    fn unexpected_error() -> ParseError {
        ParseError::new(
            ParseErrorKind::UnexpectedToken {
                found: Some("}".to_owned()),
                expected: vec!["expression".to_owned()],
            },
            ParseErrorClass::Recoverable,
        )
        .with_context("function body")
    }

    #[test]
    fn converts_parser_error_and_preserves_context() {
        let error = unexpected_error();
        let diagnostic = ParserDiagnostic::from_error(&error);

        assert_eq!(diagnostic.code(), "parser.unexpected_token");
        assert_eq!(diagnostic.severity(), ParserDiagnosticSeverity::Error);
        assert!(diagnostic.message().contains("unexpected token"));
        assert_eq!(
            diagnostic.notes(),
            &["while parsing function body"]
        );
        assert_eq!(diagnostic.primary_span(), None);
    }

    #[test]
    fn internal_errors_become_fatal_diagnostics() {
        let error =
            ParseError::internal_invariant("cursor state invalid", None);
        let diagnostic = ParserDiagnostic::from_error(&error);

        assert_eq!(diagnostic.code(), "parser.internal_invariant");
        assert_eq!(diagnostic.severity(), ParserDiagnosticSeverity::Fatal);
        assert!(
            diagnostic.notes().iter().any(|note| note.contains("bug report"))
        );
    }

    #[test]
    fn bounded_collection_counts_dropped_diagnostics() {
        let limit =
            ResourceLimit::at_most(1).expect("one is a non-zero limit");
        let mut collection = DiagnosticCollection::with_limit(limit);

        let first = ParserDiagnostic::new(
            "parser.first",
            ParserDiagnosticSeverity::Error,
            "first",
            None,
        );
        let second = ParserDiagnostic::new(
            "parser.second",
            ParserDiagnosticSeverity::Error,
            "second",
            None,
        );

        assert!(collection.push(first));
        assert!(!collection.push(second));
        assert_eq!(collection.len(), 1);
        assert_eq!(collection.diagnostics()[0].code(), "parser.first");
        assert_eq!(collection.dropped_count(), 1);
        assert!(collection.is_truncated());
    }

    #[test]
    fn unlimited_collection_retains_supplied_diagnostics() {
        let mut collection = DiagnosticCollection::default();

        for index in 0..8 {
            assert!(collection.push(ParserDiagnostic::new(
                format!("parser.test.{index}"),
                ParserDiagnosticSeverity::Warning,
                "test",
                None,
            )));
        }

        assert_eq!(collection.len(), 8);
        assert_eq!(collection.dropped_count(), 0);
        assert!(!collection.is_truncated());
    }

    #[test]
    fn error_presence_is_derived_from_severity() {
        let mut collection = DiagnosticCollection::default();

        assert!(collection.push(ParserDiagnostic::new(
            "parser.warning",
            ParserDiagnosticSeverity::Warning,
            "warning",
            None,
        )));
        assert!(!collection.has_errors());

        assert!(collection.push(ParserDiagnostic::new(
            "parser.fatal",
            ParserDiagnosticSeverity::Fatal,
            "fatal",
            None,
        )));
        assert!(collection.has_errors());
    }

    #[test]
    fn display_includes_help_and_notes() {
        let diagnostic = ParserDiagnostic::new(
            "parser.unexpected_eof",
            ParserDiagnosticSeverity::Error,
            "unexpected end of input",
            None,
        )
        .with_help("close the block")
        .with_note("inside function body");

        let rendered = diagnostic.to_string();

        assert!(rendered.starts_with("error[parser.unexpected_eof]:"));
        assert!(rendered.contains("help: close the block"));
        assert!(rendered.contains("note: inside function body"));
    }
}
