
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Structured Diagnostics
//!
//! Path: `src/zuir/diagnostics.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide the structured, deterministic diagnostic interface for ZUIR
//! construction, verification, transformation, serialization, interoperability,
//! and lowering.
//!
//! ## AUTHORITY
//!
//! This module conforms to:
//! - `grammar/DESIGN.md`
//! - `grammar/spec/diagnostics.md`
//! - `grammar/specification/semantic-model.md`
//! - `src/zuir/errors.rs`
//! - `src/zuir/ids.rs`
//! - `src/zuir/source_map.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! ## OWNS
//!
//! - ZUIR diagnostic codes, severity, and phase.
//! - Primary and secondary source labels.
//! - Notes, help, fix suggestions, and structured data.
//! - Deterministic ordering and deduplication.
//! - Optional bounded diagnostic retention and truncation accounting.
//! - Machine-readable JSON value conversion and plain-text display.
//!
//! ## DOES NOT OWN
//!
//! - Parsing, lexical analysis, source loading, or source-coordinate creation.
//! - Type checking, semantic rules, IR verification, or lowering algorithms.
//! - Source text storage or source-map identity allocation.
//! - Diagnostic policy for frontend-specific recovery.
//! - Terminal colors, editor protocols, or command-line output streams.
//! - Hardware discovery, allocation, scheduling, or runtime execution.
//!
//! ## DEPENDENCIES
//!
//! Uses canonical `ZuirError`, `ZuirErrorCode`, and `ErrorSubject` from
//! `errors.rs`, and canonical `SourceSpan` and `SourceMap` from `source_map.rs`.
//! It does not define competing source locations or IR entity identifiers.
//!
//! ## COORDINATE CONTRACT
//!
//! Source spans use the canonical half-open UTF-8 byte-offset convention
//! `[start_byte, end_byte)` defined by `source_map.rs`. Missing source
//! locations are valid for diagnostics arising outside source-backed work.
//!
//! ## DETERMINISM
//!
//! Diagnostics are stored in ordered collections. Their ordering and
//! deduplication do not depend on hash-map iteration, thread scheduling,
//! hardware availability, wall-clock time, or random state.
//!
//! ## SCALABILITY
//!
//! Diagnostic storage grows dynamically. An optional retention budget is an
//! explicit operational policy, not a language or hardware limit. When the
//! budget is reached, new unique diagnostics are rejected and counted rather
//! than replacing retained entries silently. No finite-memory implementation
//! can promise literal infinity.
//!
//! ## INTEGRATION
//!
//! - Frontend lexer and parser diagnostics remain owned by their subsystems.
//! - Semantic analysis supplies diagnostics at the semantic boundary.
//! - ZUIR builders and verifiers report established failures through this API.
//! - `source_map.rs` remains authoritative for source identity and spans.
//! - CLI, LSP, JSON, and editor-specific presentation belong to consumers.
//! - `api.rs` and `mod.rs` may re-export this module after their contracts are
//!   reconciled with the repository's existing public API.
//!
//! ## FREEZE CRITERIA
//!
//! Compile against the repository's actual module graph; test stable codes,
//! source-map validation, deterministic ordering, duplicate suppression,
//! configured retention, truncation accounting, JSON conversion, and error
//! conversion. Run formatting, linting, and the repository's relevant tests.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error as StdError;

use serde_json::{json, Value};

use crate::zuir::errors::{ErrorSubject, ZuirError, ZuirErrorCode};
use crate::zuir::source_map::{SourceMap, SourceSpan};

// =============================================================================
// Diagnostic codes
// =============================================================================

/// A validated, stable machine-readable diagnostic code.
///
/// Codes use the repository's `ZMN-...` naming convention. Standard codes
/// should be documented in the normative diagnostic registry. Extension
/// codes must follow the repository's extension registration policy.
///
/// Code syntax validation does not, by itself, register a code or establish
/// that its meaning is normatively approved.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiagnosticCode(ZuirErrorCode);

impl DiagnosticCode {
    /// Construct a code after validating its syntax.
    pub fn new(code: impl Into<String>) -> Result<Self, DiagnosticBuildError> {
        ZuirErrorCode::new(code.into())
            .map(Self)
            .map_err(|_| DiagnosticBuildError::InvalidCode)
    }

    /// Return the canonical code string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Severity and phase
// =============================================================================

/// Diagnostic severity.
///
/// Severity communicates the importance of a report. It does not determine
/// parser recovery, semantic validity, or whether a target can execute a
/// program.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum DiagnosticSeverity {
    /// Context normally attached to another diagnostic.
    Note,
    /// Non-fatal concern, including portability or deprecation.
    Warning,
    /// The requested compilation or operation cannot validly proceed.
    Error,
    /// The current operation cannot safely or meaningfully continue.
    Fatal,
    /// Informational compiler or tooling message.
    Information,
    /// Optional improvement suggestion.
    Hint,
}

impl DiagnosticSeverity {
    /// Whether this severity denotes an error condition.
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error | Self::Fatal)
    }

    /// Whether this severity normally prevents successful completion.
    #[must_use]
    pub const fn prevents_success(self) -> bool {
        self.is_error()
    }

    /// Stable lowercase representation for machine-readable output.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Note => "note",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Fatal => "fatal",
            Self::Information => "information",
            Self::Hint => "hint",
        }
    }
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Compiler phase that produced the diagnostic.
///
/// This is descriptive provenance. It must not be inferred from the message.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum DiagnosticPhase {
    Source,
    Lexical,
    Parse,
    Ast,
    Name,
    Type,
    Effect,
    Ownership,
    Resource,
    Capability,
    Domain,
    Semantic,
    IrGeneration,
    IrVerification,
    Optimization,
    Routing,
    Scheduling,
    Resilience,
    Qec,
    Zqn,
    Hal,
    Target,
    Runtime,
    Tooling,
    Compatibility,
    /// A registered or future phase not yet represented by a standard variant.
    Extension(String),
}

impl DiagnosticPhase {
    /// Stable machine-readable phase name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Source => "source",
            Self::Lexical => "lexical",
            Self::Parse => "parse",
            Self::Ast => "ast",
            Self::Name => "name",
            Self::Type => "type",
            Self::Effect => "effect",
            Self::Ownership => "ownership",
            Self::Resource => "resource",
            Self::Capability => "capability",
            Self::Domain => "domain",
            Self::Semantic => "semantic",
            Self::IrGeneration => "ir_generation",
            Self::IrVerification => "ir_verification",
            Self::Optimization => "optimization",
            Self::Routing => "routing",
            Self::Scheduling => "scheduling",
            Self::Resilience => "resilience",
            Self::Qec => "qec",
            Self::Zqn => "zqn",
            Self::Hal => "hal",
            Self::Target => "target",
            Self::Runtime => "runtime",
            Self::Tooling => "tooling",
            Self::Compatibility => "compatibility",
            Self::Extension(name) => name.as_str(),
        }
    }
}

impl fmt::Display for DiagnosticPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Labels and suggestions
// =============================================================================

/// A labeled source range.
///
/// The primary label explains the main location. Secondary labels connect the
/// diagnostic to other relevant source constructs, such as an earlier
/// declaration or another operand.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiagnosticLabel {
    span: SourceSpan,
    message: String,
}

impl DiagnosticLabel {
    /// Construct a source label.
    pub fn new(
        span: SourceSpan,
        message: impl Into<String>,
    ) -> Result<Self, DiagnosticBuildError> {
        let message = message.into();
        if message.trim().is_empty() {
            return Err(DiagnosticBuildError::EmptyLabelMessage);
        }
        Ok(Self { span, message })
    }

    #[must_use]
    pub fn span(&self) -> &SourceSpan {
        &self.span
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// How confidently a suggested edit can be applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SuggestionApplicability {
    /// The edit is expected to preserve intended meaning and can be applied
    /// without additional user judgment.
    MachineApplicable,
    /// The edit is likely correct but should be reviewed.
    MaybeIncorrect,
    /// The edit requires a human to select or complete the change.
    HasPlaceholders,
    /// The edit is informative and is not safe to apply automatically.
    Unspecified,
}

impl SuggestionApplicability {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MachineApplicable => "machine_applicable",
            Self::MaybeIncorrect => "maybe_incorrect",
            Self::HasPlaceholders => "has_placeholders",
            Self::Unspecified => "unspecified",
        }
    }
}

/// A source edit proposed by a diagnostic.
///
/// The replacement text is literal text to insert in place of the half-open
/// span. Applying edits to a source file is the responsibility of the
/// consumer, which must verify that edits do not conflict.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiagnosticFix {
    span: SourceSpan,
    replacement: String,
}

impl DiagnosticFix {
    #[must_use]
    pub fn new(span: SourceSpan, replacement: impl Into<String>) -> Self {
        Self {
            span,
            replacement: replacement.into(),
        }
    }

    #[must_use]
    pub fn span(&self) -> &SourceSpan {
        &self.span
    }

    #[must_use]
    pub fn replacement(&self) -> &str {
        &self.replacement
    }
}

/// A suggested fix, possibly containing several source edits.
///
/// A suggestion is metadata, not an instruction to mutate source automatically.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiagnosticSuggestion {
    message: String,
    applicability: SuggestionApplicability,
    fixes: Vec<DiagnosticFix>,
}

impl DiagnosticSuggestion {
    pub fn new(
        message: impl Into<String>,
        applicability: SuggestionApplicability,
        mut fixes: Vec<DiagnosticFix>,
    ) -> Result<Self, DiagnosticBuildError> {
        let message = message.into();
        if message.trim().is_empty() {
            return Err(DiagnosticBuildError::EmptySuggestionMessage);
        }

        // A canonical order makes serialized suggestions reproducible.
        fixes.sort();
        fixes.dedup();

        Ok(Self {
            message,
            applicability,
            fixes,
        })
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub const fn applicability(&self) -> SuggestionApplicability {
        self.applicability
    }

    #[must_use]
    pub fn fixes(&self) -> &[DiagnosticFix] {
        &self.fixes
    }
}

// =============================================================================
// Diagnostic construction errors
// =============================================================================

/// Error returned when a diagnostic cannot be constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticBuildError {
    InvalidCode,
    EmptyMessage,
    EmptyLabelMessage,
    EmptyNote,
    EmptyHelp,
    EmptySuggestionMessage,
    EmptyDataKey,
}

impl fmt::Display for DiagnosticBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidCode => "diagnostic code has invalid syntax",
            Self::EmptyMessage => "diagnostic message must not be empty",
            Self::EmptyLabelMessage => "diagnostic label message must not be empty",
            Self::EmptyNote => "diagnostic note must not be empty",
            Self::EmptyHelp => "diagnostic help text must not be empty",
            Self::EmptySuggestionMessage => "suggestion message must not be empty",
            Self::EmptyDataKey => "diagnostic data key must not be empty",
        })
    }
}

impl StdError for DiagnosticBuildError {}

// =============================================================================
// Diagnostic
// =============================================================================

/// A complete, structured ZUIR diagnostic.
///
/// The primary span is optional because configuration, version negotiation,
/// registry, or integration failures may occur without a source construct.
/// Fields used for ordering come first so `Ord` provides deterministic
/// source-oriented ordering, with source-free diagnostics ordered consistently.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Diagnostic {
    primary_span: Option<SourceSpan>,
    phase: DiagnosticPhase,
    severity: DiagnosticSeverity,
    code: DiagnosticCode,
    message: String,
    labels: Vec<DiagnosticLabel>,
    notes: Vec<String>,
    help: Option<String>,
    suggestions: Vec<DiagnosticSuggestion>,
    data: BTreeMap<String, String>,
}

impl Diagnostic {
    /// Construct a diagnostic with validated code and non-empty message.
    pub fn new(
        code: impl Into<String>,
        severity: DiagnosticSeverity,
        phase: DiagnosticPhase,
        message: impl Into<String>,
        primary_span: Option<SourceSpan>,
    ) -> Result<Self, DiagnosticBuildError> {
        let code = DiagnosticCode::new(code)?;
        let message = message.into();

        if message.trim().is_empty() {
            return Err(DiagnosticBuildError::EmptyMessage);
        }

        Ok(Self {
            primary_span,
            phase,
            severity,
            code,
            message,
            labels: Vec::new(),
            notes: Vec::new(),
            help: None,
            suggestions: Vec::new(),
            data: BTreeMap::new(),
        })
    }

    /// Convert an established ZUIR error into a diagnostic.
    ///
    /// The error subsystem remains authoritative for the error category,
    /// stable code, message, and structured entity context. The caller supplies
    /// the phase and source span because the error layer does not own source
    /// locations.
    pub fn from_zui_error(
        error: &ZuirError,
        phase: DiagnosticPhase,
        primary_span: Option<SourceSpan>,
    ) -> Self {
        // ZuirError::new creates its code from the canonical standard registry.
        // Validation should therefore succeed; retain a safe fallback should
        // an incompatible future error implementation violate that contract.
        let code = DiagnosticCode::new(error.code().as_str())
            .unwrap_or_else(|_| {
                DiagnosticCode::new("ZMN-IR-INTERNAL-DIAGNOSTIC-CODE")
                    .expect("the built-in diagnostic code is valid")
            });

        let mut diagnostic = Self {
            primary_span,
            phase,
            severity: DiagnosticSeverity::Error,
            code,
            message: error.message().to_owned(),
            labels: Vec::new(),
            notes: Vec::new(),
            help: None,
            suggestions: Vec::new(),
            data: BTreeMap::new(),
        };

        if let Some(subject) = error.context().subject() {
            diagnostic
                .data
                .insert("subject".to_owned(), subject.to_string());
        }

        for subject in error.context().related_subjects() {
            diagnostic
                .notes
                .push(format!("Related entity: {subject}"));
        }

        for note in error.context().notes() {
            if !note.trim().is_empty() {
                diagnostic.notes.push(note.clone());
            }
        }

        diagnostic.canonicalize();
        diagnostic
    }

    #[must_use]
    pub fn code(&self) -> &DiagnosticCode {
        &self.code
    }

    #[must_use]
    pub fn severity(&self) -> DiagnosticSeverity {
        self.severity
    }

    #[must_use]
    pub fn phase(&self) -> &DiagnosticPhase {
        &self.phase
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub fn primary_span(&self) -> Option<&SourceSpan> {
        self.primary_span.as_ref()
    }

    #[must_use]
    pub fn labels(&self) -> &[DiagnosticLabel] {
        &self.labels
    }

    #[must_use]
    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    #[must_use]
    pub fn help(&self) -> Option<&str> {
        self.help.as_deref()
    }

    #[must_use]
    pub fn suggestions(&self) -> &[DiagnosticSuggestion] {
        &self.suggestions
    }

    #[must_use]
    pub fn data(&self) -> &BTreeMap<String, String> {
        &self.data
    }

    /// Add a secondary source label.
    pub fn with_label(
        mut self,
        span: SourceSpan,
        message: impl Into<String>,
    ) -> Result<Self, DiagnosticBuildError> {
        self.labels.push(DiagnosticLabel::new(span, message)?);
        self.canonicalize();
        Ok(self)
    }

    /// Add a contextual note.
    pub fn with_note(
        mut self,
        note: impl Into<String>,
    ) -> Result<Self, DiagnosticBuildError> {
        let note = note.into();
        if note.trim().is_empty() {
            return Err(DiagnosticBuildError::EmptyNote);
        }
        self.notes.push(note);
        self.canonicalize();
        Ok(self)
    }

    /// Add remediation guidance.
    pub fn with_help(
        mut self,
        help: impl Into<String>,
    ) -> Result<Self, DiagnosticBuildError> {
        let help = help.into();
        if help.trim().is_empty() {
            return Err(DiagnosticBuildError::EmptyHelp);
        }
        self.help = Some(help);
        Ok(self)
    }

    /// Add a suggested fix or set of source edits.
    pub fn with_suggestion(mut self, suggestion: DiagnosticSuggestion) -> Self {
        self.suggestions.push(suggestion);
        self.canonicalize();
        self
    }

    /// Add a deterministic machine-readable key/value field.
    ///
    /// Keys must be non-empty. Reusing a key deliberately replaces its prior
    /// value, allowing callers to refine context without duplicate keys.
    pub fn with_data(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, DiagnosticBuildError> {
        let key = key.into();
        if key.trim().is_empty() {
            return Err(DiagnosticBuildError::EmptyDataKey);
        }
        self.data.insert(key, value.into());
        Ok(self)
    }

    /// Validate every source span against the supplied canonical source map.
    ///
    /// This checks source identity and known byte-length bounds. If a source's
    /// byte length is unknown, the map cannot establish an upper bound.
    pub fn validate_against(
        &self,
        source_map: &SourceMap,
    ) -> Result<(), DiagnosticValidationError> {
        if let Some(span) = self.primary_span.as_ref() {
            validate_span(span, source_map)?;
        }

        for label in &self.labels {
            validate_span(label.span(), source_map)?;
        }

        for suggestion in &self.suggestions {
            for fix in suggestion.fixes() {
                validate_span(fix.span(), source_map)?;
            }
        }

        Ok(())
    }

    /// Convert the diagnostic to a JSON value suitable for CLI, editor, or
    /// tooling adapters. This method does not choose an LSP or other protocol
    /// version; protocol-specific adapters remain responsible for that.
    #[must_use]
    pub fn to_json_value(&self) -> Value {
        json!({
            "code": self.code.as_str(),
            "severity": self.severity.as_str(),
            "phase": self.phase.as_str(),
            "message": self.message,
            "primary_span": self.primary_span.as_ref().map(span_to_json),
            "labels": self.labels.iter().map(|label| json!({
                "span": span_to_json(label.span()),
                "message": label.message(),
            })).collect::<Vec<_>>(),
            "notes": self.notes,
            "help": self.help,
            "suggestions": self.suggestions.iter().map(|suggestion| json!({
                "message": suggestion.message(),
                "applicability": suggestion.applicability().as_str(),
                "fixes": suggestion.fixes().iter().map(|fix| json!({
                    "span": span_to_json(fix.span()),
                    "replacement": fix.replacement(),
                })).collect::<Vec<_>>(),
            })).collect::<Vec<_>>(),
            "data": self.data,
        })
    }

    fn canonicalize(&mut self) {
        self.labels.sort();
        self.labels.dedup();
        self.notes.sort();
        self.notes.dedup();
        self.suggestions.sort();
        self.suggestions.dedup();
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}[{}] {}: {}",
            self.severity, self.code, self.phase, self.message
        )?;

        if let Some(span) = self.primary_span.as_ref() {
            write!(
                f,
                " (source {}, bytes {}..{})",
                span.source_id(),
                span.start_byte(),
                span.end_byte()
            )?;
        }

        for label in &self.labels {
            write!(
                f,
                "\n  label (source {}, bytes {}..{}): {}",
                label.span().source_id(),
                label.span().start_byte(),
                label.span().end_byte(),
                label.message()
            )?;
        }

        for note in &self.notes {
            write!(f, "\n  note: {note}")?;
        }

        if let Some(help) = self.help.as_deref() {
            write!(f, "\n  help: {help}")?;
        }

        for suggestion in &self.suggestions {
            write!(
                f,
                "\n  suggestion ({}): {}",
                suggestion.applicability().as_str(),
                suggestion.message()
            )?;
        }

        Ok(())
    }
}

fn validate_span(
    span: &SourceSpan,
    source_map: &SourceMap,
) -> Result<(), DiagnosticValidationError> {
    let document = source_map
        .source(span.source_id())
        .ok_or(DiagnosticValidationError::UnknownSource(span.source_id()))?;

    if let Some(length) = document.byte_length() {
        if span.end_byte() > length {
            return Err(DiagnosticValidationError::SpanOutOfBounds {
                source_id: span.source_id(),
                end_byte: span.end_byte(),
                source_length: length,
            });
        }
    }

    Ok(())
}

fn position_to_json(position: crate::zuir::source_map::SourcePosition) -> Value {
    json!({
        "line": position.line(),
        "column": position.column(),
        "column_unit": match position.column_unit() {
            crate::zuir::source_map::ColumnUnit::Utf8Byte => "utf8_byte",
            crate::zuir::source_map::ColumnUnit::UnicodeScalar => "unicode_scalar",
        }
    })
}

fn span_to_json(span: &SourceSpan) -> Value {
    json!({
        "source_id": span.source_id().get(),
        "start_byte": span.start_byte(),
        "end_byte": span.end_byte(),
        "start_position": span.start_position().map(position_to_json),
        "end_position": span.end_position().map(position_to_json),
    })
}

// =============================================================================
// Source validation errors
// =============================================================================

/// Error returned when a diagnostic references an invalid or unknown span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticValidationError {
    UnknownSource(crate::zuir::ids::SourceId),
    SpanOutOfBounds {
        source_id: crate::zuir::ids::SourceId,
        end_byte: u64,
        source_length: u64,
    },
}

impl fmt::Display for DiagnosticValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownSource(source_id) => {
                write!(f, "diagnostic references unregistered source {source_id}")
            }
            Self::SpanOutOfBounds {
                source_id,
                end_byte,
                source_length,
            } => write!(
                f,
                "diagnostic span for source {source_id} ends at {end_byte}, \
                 beyond its known length {source_length}"
            ),
        }
    }
}

impl StdError for DiagnosticValidationError {}

// =============================================================================
// Diagnostic collection
// =============================================================================

/// Outcome of attempting to retain a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InsertOutcome {
    /// A new unique diagnostic was retained.
    Inserted,
    /// An equivalent diagnostic was already retained.
    Duplicate,
    /// The retention budget was reached; this diagnostic was not retained.
    DroppedByBudget,
}

/// Deterministic diagnostic collection with optional retention budget.
///
/// The budget applies to retained diagnostic count, not to the language,
/// program size, or number of IR entities. `None` means no configured count
/// budget; available process memory remains finite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticBag {
    diagnostics: BTreeSet<Diagnostic>,
    max_diagnostics: Option<usize>,
    dropped_count: u64,
}

impl Default for DiagnosticBag {
    fn default() -> Self {
        Self::unbounded()
    }
}

impl DiagnosticBag {
    /// Create a collection with no explicit retention-count budget.
    #[must_use]
    pub fn unbounded() -> Self {
        Self {
            diagnostics: BTreeSet::new(),
            max_diagnostics: None,
            dropped_count: 0,
        }
    }

    /// Create a collection with an explicit maximum retained count.
    ///
    /// A budget of zero is valid and retains no diagnostics. Overflow in the
    /// dropped counter saturates rather than wrapping to a misleading value.
    #[must_use]
    pub fn with_limit(max_diagnostics: usize) -> Self {
        Self {
            diagnostics: BTreeSet::new(),
            max_diagnostics: Some(max_diagnostics),
            dropped_count: 0,
        }
    }

    /// Attempt to retain a diagnostic.
    ///
    /// Duplicate detection occurs before budget enforcement, so submitting
    /// an already-retained diagnostic does not count as a dropped item.
    pub fn push(&mut self, diagnostic: Diagnostic) -> InsertOutcome {
        if self.diagnostics.contains(&diagnostic) {
            return InsertOutcome::Duplicate;
        }

        if self
            .max_diagnostics
            .is_some_and(|limit| self.diagnostics.len() >= limit)
        {
            self.dropped_count = self.dropped_count.saturating_add(1);
            return InsertOutcome::DroppedByBudget;
        }

        self.diagnostics.insert(diagnostic);
        InsertOutcome::Inserted
    }

    /// Number of retained unique diagnostics.
    #[must_use]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Whether no diagnostics have been retained.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Number of unique diagnostics rejected by the retention budget.
    #[must_use]
    pub const fn dropped_count(&self) -> u64 {
        self.dropped_count
    }

    /// Whether at least one diagnostic was dropped.
    #[must_use]
    pub const fn is_truncated(&self) -> bool {
        self.dropped_count > 0
    }

    /// The configured retention budget, if any.
    #[must_use]
    pub const fn limit(&self) -> Option<usize> {
        self.max_diagnostics
    }

    /// Iterate over diagnostics in canonical deterministic order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &Diagnostic> {
        self.diagnostics.iter()
    }

    /// Consume the bag and return diagnostics in canonical deterministic order.
    #[must_use]
    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.diagnostics.into_iter().collect()
    }

    /// Validate all retained source spans against the supplied source map.
    pub fn validate_against(
        &self,
        source_map: &SourceMap,
    ) -> Result<(), DiagnosticValidationError> {
        for diagnostic in &self.diagnostics {
            diagnostic.validate_against(source_map)?;
        }
        Ok(())
    }

    /// Machine-readable summary of retained and dropped diagnostic counts.
    #[must_use]
    pub fn summary_to_json_value(&self) -> Value {
        json!({
            "retained_count": self.len(),
            "dropped_count": self.dropped_count,
            "truncated": self.is_truncated(),
            "retention_limit": self.max_diagnostics,
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::ids::SourceId;
    use crate::zuir::source_map::{
        SourceDocument, SourceDocumentKind, SourceMap, SourcePosition, SourceSpan, ColumnUnit,
    };

    fn source_map_with_document(length: Option<u64>) -> (SourceMap, SourceId) {
        let id = SourceId::new(1).expect("nonzero source ID");
        let document = SourceDocument::new(
            "memory://diagnostics-test",
            SourceDocumentKind::Virtual,
            length,
        )
        .expect("valid source document");

        let mut map = SourceMap::new();
        map.register_source(id, document)
            .expect("register source");
        (map, id)
    }

    fn span(source_id: SourceId, start: u64, end: u64) -> SourceSpan {
        SourceSpan::new(source_id, start, end, None, None).expect("valid span")
    }

    fn diagnostic(
        source_id: SourceId,
        start: u64,
        end: u64,
        message: &str,
    ) -> Diagnostic {
        Diagnostic::new(
            "ZMN-IR-TEST-DIAGNOSTIC",
            DiagnosticSeverity::Error,
            DiagnosticPhase::IrVerification,
            message,
            Some(span(source_id, start, end)),
        )
        .expect("valid diagnostic")
    }

    #[test]
    fn rejects_invalid_diagnostic_codes() {
        assert!(DiagnosticCode::new("not-a-code").is_err());
        assert!(DiagnosticCode::new("ZMN-IR-TEST").is_ok());
    }

    #[test]
    fn rejects_empty_messages() {
        let result = Diagnostic::new(
            "ZMN-IR-TEST-DIAGNOSTIC",
            DiagnosticSeverity::Error,
            DiagnosticPhase::IrVerification,
            "   ",
            None,
        );
        assert_eq!(result.unwrap_err(), DiagnosticBuildError::EmptyMessage);
    }

    #[test]
    fn validates_spans_against_source_map() {
        let (map, source_id) = source_map_with_document(Some(8));
        assert!(diagnostic(source_id, 2, 8, "valid").validate_against(&map).is_ok());

        let out_of_bounds = diagnostic(source_id, 2, 9, "invalid");
        assert!(matches!(
            out_of_bounds.validate_against(&map),
            Err(DiagnosticValidationError::SpanOutOfBounds { .. })
        ));
    }

    #[test]
    fn rejects_unregistered_sources() {
        let (map, _) = source_map_with_document(Some(8));
        let other_id = SourceId::new(2).expect("nonzero source ID");
        assert!(matches!(
            diagnostic(other_id, 0, 1, "unknown").validate_against(&map),
            Err(DiagnosticValidationError::UnknownSource(_))
        ));
    }

    #[test]
    fn bag_deduplicates_and_orders_deterministically() {
        let (map, source_id) = source_map_with_document(Some(100));
        let first = diagnostic(source_id, 10, 12, "later");
        let second = diagnostic(source_id, 1, 2, "earlier");

        let mut bag = DiagnosticBag::unbounded();
        assert_eq!(bag.push(first.clone()), InsertOutcome::Inserted);
        assert_eq!(bag.push(first), InsertOutcome::Duplicate);
        assert_eq!(bag.push(second), InsertOutcome::Inserted);
        assert_eq!(bag.len(), 2);
        assert!(bag.validate_against(&map).is_ok());

        let items = bag.into_vec();
        assert_eq!(
            items[0].primary_span().expect("span").start_byte(),
            1
        );
        assert_eq!(
            items[1].primary_span().expect("span").start_byte(),
            10
        );
    }

    #[test]
    fn bounded_bag_reports_truncation_without_replacing_items() {
        let (map, source_id) = source_map_with_document(Some(100));
        let mut bag = DiagnosticBag::with_limit(1);

        let first = diagnostic(source_id, 0, 1, "first");
        let second = diagnostic(source_id, 2, 3, "second");

        assert_eq!(bag.push(first.clone()), InsertOutcome::Inserted);
        assert_eq!(bag.push(first), InsertOutcome::Duplicate);
        assert_eq!(bag.push(second), InsertOutcome::DroppedByBudget);
        assert_eq!(bag.len(), 1);
        assert_eq!(bag.dropped_count(), 1);
        assert!(bag.is_truncated());
        assert!(bag.validate_against(&map).is_ok());
    }

    #[test]
    fn zero_budget_retains_nothing_and_reports_drops() {
        let (map, source_id) = source_map_with_document(Some(10));
        let mut bag = DiagnosticBag::with_limit(0);

        assert_eq!(
            bag.push(diagnostic(source_id, 0, 1, "dropped")),
            InsertOutcome::DroppedByBudget
        );
        assert!(bag.is_empty());
        assert_eq!(bag.dropped_count(), 1);
        assert!(bag.is_truncated());
        assert!(bag.validate_against(&map).is_ok());
    }

    #[test]
    fn json_output_has_machine_readable_fields() {
        let (map, source_id) = source_map_with_document(Some(10));
        let item = diagnostic(source_id, 3, 5, "invalid operation");

        assert!(item.validate_against(&map).is_ok());

        let value = item.to_json_value();
        assert_eq!(value["code"], "ZMN-IR-TEST-DIAGNOSTIC");
        assert_eq!(value["severity"], "error");
        assert_eq!(value["phase"], "ir_verification");
        assert_eq!(value["primary_span"]["start_byte"], 3);
        assert_eq!(value["primary_span"]["end_byte"], 5);
    }

    #[test]
    fn source_positions_preserve_explicit_column_units() {
        let position = SourcePosition::new(2, 4, ColumnUnit::UnicodeScalar)
            .expect("valid one-based position");
        let json = position_to_json(position);
        assert_eq!(json["line"], 2);
        assert_eq!(json["column"], 4);
        assert_eq!(json["column_unit"], "unicode_scalar");
    }

    #[test]
    fn suggestions_have_stable_edit_order() {
        let source_id = SourceId::new(1).expect("nonzero source ID");
        let second = DiagnosticFix::new(span(source_id, 5, 6), "b");
        let first = DiagnosticFix::new(span(source_id, 1, 2), "a");

        let suggestion = DiagnosticSuggestion::new(
            "Replace both tokens",
            SuggestionApplicability::MaybeIncorrect,
            vec![second, first.clone(), first],
        )
        .expect("valid suggestion");

        assert_eq!(suggestion.fixes().len(), 2);
        assert_eq!(suggestion.fixes()[0].span().start_byte(), 1);
        assert_eq!(suggestion.fixes()[1].span().start_byte(), 5);
    }
}
