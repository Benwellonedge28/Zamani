//! Zamani Compiler — Production Diagnostic Engine
//!
//! Centralized diagnostics for the Zamani compiler pipeline.
//!
//! Design goals:
//! - deterministic diagnostics
//! - source-aware diagnostics
//! - structured severity levels
//! - stable diagnostic codes
//! - duplicate suppression
//! - configurable diagnostic limits
//! - terminal and non-terminal rendering
//! - compiler-library friendly API
//! - no panics during ordinary diagnostic handling
//!
//! Diagnostics are data first. Rendering is a presentation concern.

use crate::source_map::Span;
use std::collections::HashSet;
use std::fmt;

/// Diagnostic severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticLevel {
    /// Compilation cannot safely continue.
    Error,

    /// Compilation can continue, but the program may contain a problem.
    Warning,

    /// Additional information associated with a diagnostic.
    Note,

    /// Actionable advice for resolving a diagnostic.
    Help,
}

impl DiagnosticLevel {
    /// Stable textual representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Note => "note",
            Self::Help => "help",
        }
    }

    /// Whether this level prevents successful compilation.
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }

    /// Whether this level represents actionable advice.
    pub const fn is_help(self) -> bool {
        matches!(self, Self::Help)
    }
}

impl fmt::Display for DiagnosticLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Structured compiler diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// Severity.
    pub level: DiagnosticLevel,

    /// Stable compiler diagnostic code.
    ///
    /// Examples:
    /// - `E0001`
    /// - `E1004`
    /// - `W0001`
    pub code: String,

    /// Human-readable diagnostic message.
    pub message: String,

    /// Source location associated with the diagnostic.
    pub span: Option<Span>,

    /// Optional remediation suggestion.
    pub suggestion: Option<String>,

    /// Optional related note.
    pub note: Option<String>,
}

impl Diagnostic {
    /// Create a diagnostic.
    pub fn new(
        level: DiagnosticLevel,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            level,
            code: code.into(),
            message: message.into(),
            span: None,
            suggestion: None,
            note: None,
        }
    }

    /// Attach a source span.
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    /// Attach a suggestion.
    pub fn with_suggestion(
        mut self,
        suggestion: impl Into<String>,
    ) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Attach a note.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    /// Whether this diagnostic is an error.
    pub fn is_error(&self) -> bool {
        self.level.is_error()
    }

    /// Stable identity used for duplicate suppression.
    fn fingerprint(&self) -> DiagnosticFingerprint {
        DiagnosticFingerprint {
            level: self.level,
            code: self.code.clone(),
            message: self.message.clone(),
            span: self.span.clone(),
        }
    }
}

/// Internal diagnostic identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DiagnosticFingerprint {
    level: DiagnosticLevel,
    code: String,
    message: String,
    span: Option<Span>,
}

/// Diagnostic rendering configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticConfig {
    /// Enable ANSI terminal colours.
    pub color: bool,

    /// Suppress duplicate diagnostics.
    pub deduplicate: bool,

    /// Maximum total diagnostics retained.
    ///
    /// `None` means unlimited.
    pub max_diagnostics: Option<usize>,

    /// Maximum errors retained.
    ///
    /// `None` means unlimited.
    pub max_errors: Option<usize>,

    /// Maximum warnings retained.
    ///
    /// `None` means unlimited.
    pub max_warnings: Option<usize>,
}

impl Default for DiagnosticConfig {
    fn default() -> Self {
        Self {
            color: true,
            deduplicate: true,
            max_diagnostics: None,
            max_errors: None,
            max_warnings: None,
        }
    }
}

/// Errors raised by diagnostic configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticError {
    InvalidConfiguration(String),
    DiagnosticLimitReached,
}

impl fmt::Display for DiagnosticError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(
                    formatter,
                    "invalid diagnostic configuration: {}",
                    message
                )
            }

            Self::DiagnosticLimitReached => {
                write!(formatter, "diagnostic limit reached")
            }
        }
    }
}

impl std::error::Error for DiagnosticError {}

/// Central diagnostic collector for the compiler.
#[derive(Debug, Clone)]
pub struct DiagnosticEngine {
    /// Diagnostics emitted by the compiler.
    pub diagnostics: Vec<Diagnostic>,

    /// Whether at least one error has been emitted.
    pub has_errors: bool,

    /// Diagnostic configuration.
    pub config: DiagnosticConfig,

    /// Number of errors emitted.
    error_count: usize,

    /// Number of warnings emitted.
    warning_count: usize,

    /// Number of notes emitted.
    note_count: usize,

    /// Number of help diagnostics emitted.
    help_count: usize,

    /// Diagnostic identities already emitted.
    fingerprints: HashSet<DiagnosticFingerprint>,
}

impl Default for DiagnosticEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagnosticEngine {
    /// Create a diagnostic engine with production defaults.
    pub fn new() -> Self {
        Self::with_config(DiagnosticConfig::default())
            .expect("default diagnostic configuration must be valid")
    }

    /// Create a diagnostic engine with explicit configuration.
    pub fn with_config(
        config: DiagnosticConfig,
    ) -> Result<Self, DiagnosticError> {
        validate_config(&config)?;

        Ok(Self {
            diagnostics: Vec::new(),
            has_errors: false,
            config,
            error_count: 0,
            warning_count: 0,
            note_count: 0,
            help_count: 0,
            fingerprints: HashSet::new(),
        })
    }

    /// Emit a structured diagnostic.
    ///
    /// Returns `true` when the diagnostic was actually inserted.
    ///
    /// Returns `false` when duplicate suppression discarded it.
    pub fn emit(
        &mut self,
        level: DiagnosticLevel,
        code: impl Into<String>,
        message: impl Into<String>,
        span: Option<Span>,
        suggestion: Option<String>,
    ) -> bool {
        let diagnostic = Diagnostic {
            level,
            code: code.into(),
            message: message.into(),
            span,
            suggestion,
            note: None,
        };

        self.emit_diagnostic(diagnostic).unwrap_or(false)
    }

    /// Emit a complete diagnostic.
    pub fn emit_diagnostic(
        &mut self,
        diagnostic: Diagnostic,
    ) -> Result<bool, DiagnosticError> {
        if self.config.deduplicate {
            let fingerprint = diagnostic.fingerprint();

            if !self.fingerprints.insert(fingerprint) {
                return Ok(false);
            }
        }

        if !self.can_accept(&diagnostic) {
            return Err(DiagnosticError::DiagnosticLimitReached);
        }

        self.record(&diagnostic);

        self.diagnostics.push(diagnostic);

        Ok(true)
    }

    /// Emit an error.
    pub fn error(
        &mut self,
        code: impl Into<String>,
        message: impl Into<String>,
        span: Option<Span>,
    ) -> bool {
        self.emit(
            DiagnosticLevel::Error,
            code,
            message,
            span,
            None,
        )
    }

    /// Emit an error with a suggestion.
    pub fn error_with_suggestion(
        &mut self,
        code: impl Into<String>,
        message: impl Into<String>,
        span: Option<Span>,
        suggestion: impl Into<String>,
    ) -> bool {
        self.emit(
            DiagnosticLevel::Error,
            code,
            message,
            span,
            Some(suggestion.into()),
        )
    }

    /// Emit a warning.
    pub fn warning(
        &mut self,
        code: impl Into<String>,
        message: impl Into<String>,
        span: Option<Span>,
    ) -> bool {
        self.emit(
            DiagnosticLevel::Warning,
            code,
            message,
            span,
            None,
        )
    }

    /// Emit an informational note.
    pub fn note(
        &mut self,
        code: impl Into<String>,
        message: impl Into<String>,
        span: Option<Span>,
    ) -> bool {
        self.emit(
            DiagnosticLevel::Note,
            code,
            message,
            span,
            None,
        )
    }

    /// Emit help.
    pub fn help(
        &mut self,
        code: impl Into<String>,
        message: impl Into<String>,
        span: Option<Span>,
    ) -> bool {
        self.emit(
            DiagnosticLevel::Help,
            code,
            message,
            span,
            None,
        )
    }

    /// Attach a note to an existing diagnostic.
    pub fn add_note(
        &mut self,
        index: usize,
        note: impl Into<String>,
    ) -> bool {
        if let Some(diagnostic) = self.diagnostics.get_mut(index) {
            diagnostic.note = Some(note.into());
            true
        } else {
            false
        }
    }

    /// Whether errors have been emitted.
    pub fn has_errors(&self) -> bool {
        self.has_errors
    }

    /// Number of errors.
    pub fn error_count(&self) -> usize {
        self.error_count
    }

    /// Number of warnings.
    pub fn warning_count(&self) -> usize {
        self.warning_count
    }

    /// Number of notes.
    pub fn note_count(&self) -> usize {
        self.note_count
    }

    /// Number of help diagnostics.
    pub fn help_count(&self) -> usize {
        self.help_count
    }

    /// Total diagnostics.
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Whether no diagnostics exist.
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Whether compilation may proceed.
    pub fn can_compile(&self) -> bool {
        !self.has_errors
    }

    /// Clear all diagnostics and counters.
    pub fn clear(&mut self) {
        self.diagnostics.clear();
        self.fingerprints.clear();

        self.has_errors = false;
        self.error_count = 0;
        self.warning_count = 0;
        self.note_count = 0;
        self.help_count = 0;
    }

    /// Sort diagnostics deterministically.
    ///
    /// Diagnostics with source spans are ordered first by their span's
    /// debug representation. Diagnostics without spans are placed afterward.
    pub fn sort_deterministic(&mut self) {
        self.diagnostics.sort_by(|left, right| {
            let left_key = (
                left.span.is_none(),
                format!("{:?}", left.span),
                left.code.as_str(),
                left.message.as_str(),
            );

            let right_key = (
                right.span.is_none(),
                format!("{:?}", right.span),
                right.code.as_str(),
                right.message.as_str(),
            );

            left_key.cmp(&right_key)
        });
    }

    /// Render diagnostics using the configured terminal settings.
    pub fn render_report(&self) -> String {
        self.render_report_with_color(self.config.color)
    }

    /// Render diagnostics with explicit colour control.
    pub fn render_report_with_color(&self, color: bool) -> String {
        let mut output = String::new();

        if self.diagnostics.is_empty() {
            return output;
        }

        for diagnostic in &self.diagnostics {
            render_diagnostic(
                &mut output,
                diagnostic,
                color,
            );
        }

        output.push_str(&format!(
            "\n{} diagnostic(s): {} error(s), {} warning(s), {} note(s), {} help item(s)\n",
            self.diagnostics.len(),
            self.error_count,
            self.warning_count,
            self.note_count,
            self.help_count,
        ));

        output
    }

    /// Print the diagnostic report.
    ///
    /// Kept for compatibility with the previous implementation.
    pub fn print_report(&self) {
        print!("{}", self.render_report());
    }

    fn can_accept(&self, diagnostic: &Diagnostic) -> bool {
        if let Some(limit) = self.config.max_diagnostics {
            if self.diagnostics.len() >= limit {
                return false;
            }
        }

        match diagnostic.level {
            DiagnosticLevel::Error => {
                if let Some(limit) = self.config.max_errors {
                    if self.error_count >= limit {
                        return false;
                    }
                }
            }

            DiagnosticLevel::Warning => {
                if let Some(limit) = self.config.max_warnings {
                    if self.warning_count >= limit {
                        return false;
                    }
                }
            }

            DiagnosticLevel::Note | DiagnosticLevel::Help => {}
        }

        true
    }

    fn record(&mut self, diagnostic: &Diagnostic) {
        match diagnostic.level {
            DiagnosticLevel::Error => {
                self.error_count += 1;
                self.has_errors = true;
            }

            DiagnosticLevel::Warning => {
                self.warning_count += 1;
            }

            DiagnosticLevel::Note => {
                self.note_count += 1;
            }

            DiagnosticLevel::Help => {
                self.help_count += 1;
            }
        }
    }
}

/// Render one diagnostic.
fn render_diagnostic(
    output: &mut String,
    diagnostic: &Diagnostic,
    color: bool,
) {
    let level = diagnostic.level.as_str();

    if color {
        let ansi = match diagnostic.level {
            DiagnosticLevel::Error => "\x1b[31m",
            DiagnosticLevel::Warning => "\x1b[33m",
            DiagnosticLevel::Note => "\x1b[36m",
            DiagnosticLevel::Help => "\x1b[32m",
        };

        output.push_str(ansi);
        output.push_str(level);
        output.push_str(&format!("[{}]", diagnostic.code));
        output.push_str("\x1b[0m");
    } else {
        output.push_str(level);
        output.push_str(&format!("[{}]", diagnostic.code));
    }

    output.push_str(": ");
    output.push_str(&diagnostic.message);

    if let Some(span) = &diagnostic.span {
        output.push_str(&format!(" at {:?}", span));
    }

    output.push('\n');

    if let Some(note) = &diagnostic.note {
        if color {
            output.push_str("  = \x1b[36mnote\x1b[0m: ");
        } else {
            output.push_str("  = note: ");
        }

        output.push_str(note);
        output.push('\n');
    }

    if let Some(suggestion) = &diagnostic.suggestion {
        if color {
            output.push_str("  = \x1b[32mhelp\x1b[0m: ");
        } else {
            output.push_str("  = help: ");
        }

        output.push_str(suggestion);
        output.push('\n');
    }
}

/// Validate diagnostic configuration.
fn validate_config(
    config: &DiagnosticConfig,
) -> Result<(), DiagnosticError> {
    if matches!(config.max_diagnostics, Some(0)) {
        return Err(DiagnosticError::InvalidConfiguration(
            "max_diagnostics must be greater than zero".into(),
        ));
    }

    if matches!(config.max_errors, Some(0)) {
        return Err(DiagnosticError::InvalidConfiguration(
            "max_errors must be greater than zero".into(),
        ));
    }

    if matches!(config.max_warnings, Some(0)) {
        return Err(DiagnosticError::InvalidConfiguration(
            "max_warnings must be greater than zero".into(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_starts_empty() {
        let engine = DiagnosticEngine::new();

        assert!(engine.is_empty());
        assert!(!engine.has_errors());
        assert!(engine.can_compile());
    }

    #[test]
    fn error_sets_error_state() {
        let mut engine = DiagnosticEngine::new();

        assert!(engine.error(
            "E0001",
            "something went wrong",
            None,
        ));

        assert!(engine.has_errors());
        assert!(!engine.can_compile());
        assert_eq!(engine.error_count(), 1);
    }

    #[test]
    fn warning_does_not_block_compilation() {
        let mut engine = DiagnosticEngine::new();

        assert!(engine.warning(
            "W0001",
            "unused value",
            None,
        ));

        assert!(!engine.has_errors());
        assert!(engine.can_compile());
        assert_eq!(engine.warning_count(), 1);
    }

    #[test]
    fn notes_and_help_are_counted() {
        let mut engine = DiagnosticEngine::new();

        engine.note("N0001", "additional information", None);
        engine.help("H0001", "try this instead", None);

        assert_eq!(engine.note_count(), 1);
        assert_eq!(engine.help_count(), 1);
    }

    #[test]
    fn duplicate_diagnostics_are_suppressed() {
        let mut engine = DiagnosticEngine::new();

        let first = engine.error(
            "E0001",
            "same error",
            None,
        );

        let second = engine.error(
            "E0001",
            "same error",
            None,
        );

        assert!(first);
        assert!(!second);
        assert_eq!(engine.len(), 1);
        assert_eq!(engine.error_count(), 1);
    }

    #[test]
    fn different_codes_are_not_duplicates() {
        let mut engine = DiagnosticEngine::new();

        engine.error(
            "E0001",
            "same message",
            None,
        );

        engine.error(
            "E0002",
            "same message",
            None,
        );

        assert_eq!(engine.len(), 2);
    }

    #[test]
    fn suggestions_are_rendered() {
        let mut engine = DiagnosticEngine::new();

        engine.error_with_suggestion(
            "E0001",
            "invalid syntax",
            None,
            "add a semicolon",
        );

        let report =
            engine.render_report_with_color(false);

        assert!(report.contains("error[E0001]"));
        assert!(report.contains("invalid syntax"));
        assert!(report.contains("add a semicolon"));
    }

    #[test]
    fn notes_are_rendered() {
        let mut engine = DiagnosticEngine::new();

        engine.error(
            "E0001",
            "main error",
            None,
        );

        assert!(engine.add_note(
            0,
            "this happened because of X",
        ));

        let report =
            engine.render_report_with_color(false);

        assert!(report.contains("note"));
        assert!(report.contains(
            "this happened because of X"
        ));
    }

    #[test]
    fn color_can_be_disabled() {
        let config = DiagnosticConfig {
            color: false,
            ..DiagnosticConfig::default()
        };

        let mut engine =
            DiagnosticEngine::with_config(config)
                .unwrap();

        engine.error(
            "E0001",
            "plain diagnostic",
            None,
        );

        let report = engine.render_report();

        assert!(!report.contains("\x1b["));
    }

    #[test]
    fn maximum_diagnostic_limit_works() {
        let config = DiagnosticConfig {
            max_diagnostics: Some(1),
            ..DiagnosticConfig::default()
        };

        let mut engine =
            DiagnosticEngine::with_config(config)
                .unwrap();

        assert!(engine.error(
            "E0001",
            "first",
            None,
        ));

        let result = engine.emit_diagnostic(
            Diagnostic::new(
                DiagnosticLevel::Error,
                "E0002",
                "second",
            ),
        );

        assert!(matches!(
            result,
            Err(DiagnosticError::DiagnosticLimitReached)
        ));

        assert_eq!(engine.len(), 1);
    }

    #[test]
    fn maximum_error_limit_works() {
        let config = DiagnosticConfig {
            max_errors: Some(1),
            ..DiagnosticConfig::default()
        };

        let mut engine =
            DiagnosticEngine::with_config(config)
                .unwrap();

        assert!(engine.error(
            "E0001",
            "first",
            None,
        ));

        let result = engine.emit_diagnostic(
            Diagnostic::new(
                DiagnosticLevel::Error,
                "E0002",
                "second",
            ),
        );

        assert!(matches!(
            result,
            Err(DiagnosticError::DiagnosticLimitReached)
        ));
    }

    #[test]
    fn clear_resets_state() {
        let mut engine = DiagnosticEngine::new();

        engine.error(
            "E0001",
            "failure",
            None,
        );

        assert!(!engine.can_compile());

        engine.clear();

        assert!(engine.is_empty());
        assert!(engine.can_compile());
        assert_eq!(engine.error_count(), 0);
    }

    #[test]
    fn diagnostic_builder_works() {
        let diagnostic = Diagnostic::new(
            DiagnosticLevel::Error,
            "E1000",
            "invalid operation",
        )
        .with_suggestion("use a valid operation")
        .with_note("operations must be declared first");

        assert_eq!(diagnostic.code, "E1000");
        assert!(diagnostic.suggestion.is_some());
        assert!(diagnostic.note.is_some());
    }

    #[test]
    fn empty_report_is_empty() {
        let engine = DiagnosticEngine::new();

        assert_eq!(
            engine.render_report_with_color(false),
            ""
        );
    }

    #[test]
    fn diagnostic_level_order_is_stable() {
        assert!(
            DiagnosticLevel::Error
                < DiagnosticLevel::Warning
        );
    }
}