
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Structured Diagnostics
//!
//! Path: `src/classical/diagnostics.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides structured, source-aware diagnostics for classical analysis,
//! validation, numerical computation, optimization, and lowering.
//!
//! # Ownership
//!
//! This module owns classical diagnostic metadata and the optional local
//! diagnostic collection. It does not own source spans, source-file storage,
//! classical error taxonomy, compiler-wide diagnostic rendering, parsing,
//! semantic rules, or canonical IR.
//!
//! # Authority
//!
//! - `crate::classical::error` owns classical error kinds and stable error codes.
//! - `crate::source_map` owns `FileId`, `BytePos`, and `Span`.
//! - `crate::compiler::diagnostics` remains the compiler-wide diagnostic API.
//! - `grammar/spec/diagnostics.md` and `grammar/spec/source-spans.md` define
//!   the normative diagnostic and source-location contracts.
//!
//! # POCO-REAF
//!
//! This module has no fixed maximum diagnostic count, source size, file count,
//! label count, note count, or fix-it count. Callers may opt into explicit
//! collection budgets. A configured budget is an operational constraint,
//! never a language-semantic limit.
//!
//! # Integration
//!
//! Classical validators and analysis modules construct `Diagnostic` values.
//! An integration adapter may translate these values into the existing
//! compiler diagnostic representation. Rendering and editor protocols remain
//! outside this module.
//!
//! # Freeze status
//!
//! Proposed implementation contract. Compilation and integration must be
//! verified against the repository's complete module declarations and CI.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde_json::{json, Value};

use crate::classical::error::{ClassicalError, ClassicalErrorSeverity};
use crate::source_map::Span;

/// Result returned by diagnostic construction and validation.
pub type DiagnosticResult<T> = Result<T, DiagnosticBuildError>;

/// A stable, machine-readable diagnostic identifier.
///
/// Codes are identifiers, not human-readable messages. Once published,
/// changing a code's meaning requires a compatibility review.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiagnosticCode(String);

impl DiagnosticCode {
    /// Constructs a code containing only ASCII letters, digits, `.`, `_`, or
    /// `-`. Empty codes and whitespace are rejected.
    pub fn new(value: impl Into<String>) -> DiagnosticResult<Self> {
        let value = value.into();

        if value.is_empty()
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        {
            return Err(DiagnosticBuildError::InvalidCode);
        }

        Ok(Self(value))
    }

    /// Returns the stable code as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Diagnostic severity, independent of the subsystem's error taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticSeverity {
    /// A correctness or validity failure.
    Error,
    /// A problem that does not necessarily invalidate the compilation.
    Warning,
    /// Non-actionable supplementary information.
    Information,
    /// A hint intended to help the user improve or understand the program.
    Hint,
}

impl DiagnosticSeverity {
    /// Stable machine-readable spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Information => "information",
            Self::Hint => "hint",
        }
    }

    /// Whether this diagnostic is an error.
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// The phase that detected a diagnostic.
///
/// This is intentionally an extensible identifier rather than a closed enum.
/// New compiler phases can be introduced without changing this data model.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiagnosticPhase(String);

impl DiagnosticPhase {
    pub const SOURCE: &'static str = "source";
    pub const LEXING: &'static str = "lexing";
    pub const PARSING: &'static str = "parsing";
    pub const STRUCTURAL_VALIDATION: &'static str = "structural_validation";
    pub const NAME_RESOLUTION: &'static str = "name_resolution";
    pub const TYPE_CHECKING: &'static str = "type_checking";
    pub const EFFECT_CHECKING: &'static str = "effect_checking";
    pub const RESOURCE_ANALYSIS: &'static str = "resource_analysis";
    pub const CLASSICAL: &'static str = "classical";
    pub const QUANTUM: &'static str = "quantum";
    pub const HDL: &'static str = "hdl";
    pub const LOWERING: &'static str = "lowering";
    pub const OPTIMIZATION: &'static str = "optimization";
    pub const CODE_GENERATION: &'static str = "code_generation";
    pub const INTEROPERABILITY: &'static str = "interoperability";
    pub const RUNTIME: &'static str = "runtime";
    pub const UNKNOWN: &'static str = "unknown";

    pub fn new(value: impl Into<String>) -> DiagnosticResult<Self> {
        Self::new_identifier(value, "phase")
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn new_identifier(
        value: impl Into<String>,
        kind: &'static str,
    ) -> DiagnosticResult<Self> {
        let value = value.into();

        if value.is_empty()
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        {
            return Err(DiagnosticBuildError::InvalidIdentifier { kind });
        }

        Ok(Self(value))
    }
}

/// The language or implementation domain associated with a diagnostic.
///
/// Examples include `classical`, `quantum`, `hdl`, `frontend`, and
/// `interoperability`. The registry of domain identifiers is not closed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiagnosticDomain(String);

impl DiagnosticDomain {
    pub fn new(value: impl Into<String>) -> DiagnosticResult<Self> {
        let value = value.into();

        if value.is_empty()
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        {
            return Err(DiagnosticBuildError::InvalidIdentifier { kind: "domain" });
        }

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Whether a label identifies the main issue or a related source location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LabelKind {
    Primary,
    Secondary,
}

impl LabelKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
        }
    }
}

/// A source label attached to a diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticLabel {
    pub kind: LabelKind,
    pub span: Span,
    pub message: String,
}

impl DiagnosticLabel {
    pub fn new(
        kind: LabelKind,
        span: Span,
        message: impl Into<String>,
    ) -> DiagnosticResult<Self> {
        validate_span(&span)?;

        Ok(Self {
            kind,
            span,
            message: message.into(),
        })
    }
}

/// How confidently an automated tool may apply a suggested edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FixApplicability {
    /// The edit is intended to be correct without additional user judgment.
    MachineApplicable,
    /// The edit is plausible but should be reviewed by the user.
    MaybeIncorrect,
    /// The edit is incomplete or requires additional changes.
    HasPlaceholders,
    /// The edit is provided for information and should not be applied
    /// automatically.
    Unspecified,
}

impl FixApplicability {
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

/// A proposed source replacement.
///
/// The replacement is data only. This module never edits source files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticFix {
    pub span: Span,
    pub replacement: String,
    pub applicability: FixApplicability,
    pub message: Option<String>,
}

impl DiagnosticFix {
    pub fn new(
        span: Span,
        replacement: impl Into<String>,
        applicability: FixApplicability,
    ) -> DiagnosticResult<Self> {
        validate_span(&span)?;

        Ok(Self {
            span,
            replacement: replacement.into(),
            applicability,
            message: None,
        })
    }

    #[must_use]
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

/// Recovery classification for downstream orchestration.
///
/// A classification does not itself perform recovery or authorize execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecoveryStatus {
    /// No recovery has been attempted or classified.
    Unspecified,
    /// Analysis may continue without treating the affected construct as valid.
    Recoverable,
    /// The current compilation unit cannot safely continue.
    Fatal,
}

impl RecoveryStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unspecified => "unspecified",
            Self::Recoverable => "recoverable",
            Self::Fatal => "fatal",
        }
    }
}

/// A structured classical diagnostic.
///
/// `labels` can contain at most one primary label. A diagnostic without a
/// source label is valid for global, configuration, resource, or backend
/// findings. Source locations are represented by the repository's canonical
/// `Span` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub code: DiagnosticCode,
    pub phase: DiagnosticPhase,
    pub domain: DiagnosticDomain,
    pub message: String,
    pub labels: Vec<DiagnosticLabel>,
    pub notes: Vec<String>,
    pub help: Vec<String>,
    pub fixes: Vec<DiagnosticFix>,
    /// Extensible string-valued fields for tools and domain-specific context.
    ///
    /// Use stable keys for machine-readable fields. Do not use this map as a
    /// replacement for fields with defined semantics.
    pub data: BTreeMap<String, String>,
    pub recovery: RecoveryStatus,
}

impl Diagnostic {
    pub fn new(
        severity: DiagnosticSeverity,
        code: DiagnosticCode,
        phase: DiagnosticPhase,
        domain: DiagnosticDomain,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code,
            phase,
            domain,
            message: message.into(),
            labels: Vec::new(),
            notes: Vec::new(),
            help: Vec::new(),
            fixes: Vec::new(),
            data: BTreeMap::new(),
            recovery: RecoveryStatus::Unspecified,
        }
    }

    /// Creates a diagnostic from a canonical classical error.
    ///
    /// This preserves the error's stable code and classifies severity. It
    /// deliberately does not parse `Display` text or infer source locations.
    pub fn from_classical_error(
        error: &ClassicalError,
        phase: DiagnosticPhase,
    ) -> DiagnosticResult<Self> {
        let severity = match error.severity() {
            ClassicalErrorSeverity::Input
            | ClassicalErrorSeverity::Configuration
            | ClassicalErrorSeverity::Validation
            | ClassicalErrorSeverity::Numerical
            | ClassicalErrorSeverity::Internal => DiagnosticSeverity::Error,
            ClassicalErrorSeverity::Resource
            | ClassicalErrorSeverity::Operational => DiagnosticSeverity::Error,
        };

        let code = DiagnosticCode::new(error.code())?;
        let domain = DiagnosticDomain::new("classical")?;

        Ok(Self::new(
            severity,
            code,
            phase,
            domain,
            error.to_string(),
        ))
    }

    /// Adds a primary source label. A second primary label is rejected.
    pub fn with_primary_label(
        mut self,
        span: Span,
        message: impl Into<String>,
    ) -> DiagnosticResult<Self> {
        if self.labels.iter().any(|label| label.kind == LabelKind::Primary) {
            return Err(DiagnosticBuildError::MultiplePrimaryLabels);
        }

        self.labels
            .push(DiagnosticLabel::new(LabelKind::Primary, span, message)?);
        Ok(self)
    }

    /// Adds a related source location.
    pub fn with_secondary_label(
        mut self,
        span: Span,
        message: impl Into<String>,
    ) -> DiagnosticResult<Self> {
        self.labels
            .push(DiagnosticLabel::new(LabelKind::Secondary, span, message)?);
        Ok(self)
    }

    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    #[must_use]
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help.push(help.into());
        self
    }

    pub fn with_fix(mut self, fix: DiagnosticFix) -> Self {
        self.fixes.push(fix);
        self
    }

    /// Adds or replaces a machine-readable field.
    ///
    /// Returns an error for an invalid key. Reusing a key deliberately
    /// replaces its prior value, making the behavior explicit and predictable.
    pub fn with_data(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> DiagnosticResult<Self> {
        let key = key.into();

        if key.is_empty()
            || !key
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        {
            return Err(DiagnosticBuildError::InvalidIdentifier { kind: "data key" });
        }

        self.data.insert(key, value.into());
        Ok(self)
    }

    #[must_use]
    pub fn with_recovery(mut self, recovery: RecoveryStatus) -> Self {
        self.recovery = recovery;
        self
    }

    #[must_use]
    pub fn is_error(&self) -> bool {
        self.severity.is_error()
    }

    #[must_use]
    pub fn primary_span(&self) -> Option<&Span> {
        self.labels
            .iter()
            .find(|label| label.kind == LabelKind::Primary)
            .map(|label| &label.span)
    }

    /// Converts the structured diagnostic into a stable JSON-compatible
    /// representation. Source positions are byte offsets; line and column
    /// values retain the conventions of `crate::source_map::Span`.
    #[must_use]
    pub fn to_json_value(&self) -> Value {
        let labels: Vec<Value> = self
            .labels
            .iter()
            .map(|label| {
                json!({
                    "kind": label.kind.as_str(),
                    "message": label.message,
                    "span": span_to_json(&label.span),
                })
            })
            .collect();

        let fixes: Vec<Value> = self
            .fixes
            .iter()
            .map(|fix| {
                json!({
                    "span": span_to_json(&fix.span),
                    "replacement": fix.replacement,
                    "applicability": fix.applicability.as_str(),
                    "message": fix.message,
                })
            })
            .collect();

        json!({
            "severity": self.severity.as_str(),
            "code": self.code.as_str(),
            "phase": self.phase.as_str(),
            "domain": self.domain.as_str(),
            "message": self.message,
            "labels": labels,
            "notes": self.notes,
            "help": self.help,
            "fixes": fixes,
            "data": self.data,
            "recovery": self.recovery.as_str(),
        })
    }

    fn fingerprint(&self) -> DiagnosticFingerprint {
        DiagnosticFingerprint {
            severity: self.severity,
            code: self.code.clone(),
            phase: self.phase.clone(),
            domain: self.domain.clone(),
            message: self.message.clone(),
            labels: self
                .labels
                .iter()
                .map(|label| {
                    (
                        label.kind,
                        SpanKey::from(&label.span),
                        label.message.clone(),
                    )
                })
                .collect(),
            notes: self.notes.clone(),
            help: self.help.clone(),
            fixes: self
                .fixes
                .iter()
                .map(|fix| {
                    (
                        SpanKey::from(&fix.span),
                        fix.replacement.clone(),
                        fix.applicability,
                        fix.message.clone(),
                    )
                })
                .collect(),
            data: self.data.clone(),
            recovery: self.recovery,
        }
    }
}

/// Explicit policy for retaining diagnostics.
///
/// `None` means this collection imposes no count limit. The default is
/// unbounded, subject to actual memory and other operational constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagnosticPolicy {
    pub max_diagnostics: Option<usize>,
    pub deduplicate: bool,
}

impl DiagnosticPolicy {
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_diagnostics: None,
            deduplicate: true,
        }
    }

    #[must_use]
    pub const fn with_limit(max_diagnostics: usize) -> Self {
        Self {
            max_diagnostics: Some(max_diagnostics),
            deduplicate: true,
        }
    }
}

impl Default for DiagnosticPolicy {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// Result of adding a diagnostic to a collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertOutcome {
    Inserted,
    DuplicateSuppressed,
}

/// Deterministic diagnostic collection.
///
/// Insertion order is retained until `sort_deterministically` is requested.
/// Duplicate suppression uses a structured ordered key rather than randomized
/// hash iteration. A configured limit is enforced explicitly; exceeding it
/// returns an error instead of silently dropping diagnostics.
#[derive(Debug, Clone)]
pub struct DiagnosticStore {
    policy: DiagnosticPolicy,
    diagnostics: Vec<Diagnostic>,
    fingerprints: BTreeSet<DiagnosticFingerprint>,
}

impl DiagnosticStore {
    #[must_use]
    pub fn new(policy: DiagnosticPolicy) -> Self {
        Self {
            policy,
            diagnostics: Vec::new(),
            fingerprints: BTreeSet::new(),
        }
    }

    #[must_use]
    pub fn policy(&self) -> DiagnosticPolicy {
        self.policy
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    #[must_use]
    pub fn as_slice(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn insert(&mut self, diagnostic: Diagnostic) -> Result<InsertOutcome, DiagnosticStoreError> {
        let fingerprint = diagnostic.fingerprint();

        if self.policy.deduplicate && self.fingerprints.contains(&fingerprint) {
            return Ok(InsertOutcome::DuplicateSuppressed);
        }

        if let Some(limit) = self.policy.max_diagnostics {
            if self.diagnostics.len() >= limit {
                return Err(DiagnosticStoreError::LimitReached { limit });
            }
        }

        if self.policy.deduplicate {
            self.fingerprints.insert(fingerprint);
        }

        self.diagnostics.push(diagnostic);
        Ok(InsertOutcome::Inserted)
    }

    /// Sorts diagnostics using a stable, source-first key. This operation is
    /// deterministic and independent of insertion order for distinct keys.
    pub fn sort_deterministically(&mut self) {
        self.diagnostics.sort_by_key(DiagnosticSortKey::from);
    }
}

impl Default for DiagnosticStore {
    fn default() -> Self {
        Self::new(DiagnosticPolicy::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SpanKey {
    file_id: usize,
    start: u32,
    end: u32,
    start_line: u32,
    start_column: u32,
}

impl From<&Span> for SpanKey {
    fn from(span: &Span) -> Self {
        Self {
            file_id: span.file_id.0,
            start: span.start.0,
            end: span.end.0,
            start_line: span.start_line,
            start_column: span.start_column,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct DiagnosticFingerprint {
    severity: DiagnosticSeverity,
    code: DiagnosticCode,
    phase: DiagnosticPhase,
    domain: DiagnosticDomain,
    message: String,
    labels: Vec<(LabelKind, SpanKey, String)>,
    notes: Vec<String>,
    help: Vec<String>,
    fixes: Vec<(SpanKey, String, FixApplicability, Option<String>)>,
    data: BTreeMap<String, String>,
    recovery: RecoveryStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct DiagnosticSortKey {
    has_source: bool,
    file_id: usize,
    start: u32,
    end: u32,
    severity: DiagnosticSeverity,
    code: DiagnosticCode,
    phase: DiagnosticPhase,
    domain: DiagnosticDomain,
    message: String,
}

impl From<&Diagnostic> for DiagnosticSortKey {
    fn from(diagnostic: &Diagnostic) -> Self {
        let source = diagnostic.primary_span().or_else(|| {
            diagnostic.labels.first().map(|label| &label.span)
        });

        match source {
            Some(span) => Self {
                has_source: false,
                file_id: span.file_id.0,
                start: span.start.0,
                end: span.end.0,
                severity: diagnostic.severity,
                code: diagnostic.code.clone(),
                phase: diagnostic.phase.clone(),
                domain: diagnostic.domain.clone(),
                message: diagnostic.message.clone(),
            },
            None => Self {
                has_source: true,
                file_id: 0,
                start: 0,
                end: 0,
                severity: diagnostic.severity,
                code: diagnostic.code.clone(),
                phase: diagnostic.phase.clone(),
                domain: diagnostic.domain.clone(),
                message: diagnostic.message.clone(),
            },
        }
    }
}

/// Construction failures. These indicate malformed diagnostic metadata, not
/// a failure in the Zamani program being analyzed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticBuildError {
    InvalidCode,
    InvalidIdentifier { kind: &'static str },
    InvalidSpan { start: u32, end: u32 },
    MultiplePrimaryLabels,
}

impl fmt::Display for DiagnosticBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCode => {
                formatter.write_str("diagnostic code must be a non-empty ASCII identifier")
            }
            Self::InvalidIdentifier { kind } => {
                write!(formatter, "diagnostic {kind} must be a non-empty ASCII identifier")
            }
            Self::InvalidSpan { start, end } => {
                write!(formatter, "invalid source span: start {start} exceeds end {end}")
            }
            Self::MultiplePrimaryLabels => {
                formatter.write_str("a diagnostic may have only one primary label")
            }
        }
    }
}

impl std::error::Error for DiagnosticBuildError {}

/// Collection failures. A limit error does not silently discard the attempted
/// diagnostic; the caller receives the error and decides how to report it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticStoreError {
    LimitReached { limit: usize },
}

impl fmt::Display for DiagnosticStoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LimitReached { limit } => {
                write!(formatter, "diagnostic collection limit reached ({limit})")
            }
        }
    }
}

impl std::error::Error for DiagnosticStoreError {}

fn validate_span(span: &Span) -> DiagnosticResult<()> {
    if span.start.0 > span.end.0 {
        return Err(DiagnosticBuildError::InvalidSpan {
            start: span.start.0,
            end: span.end.0,
        });
    }

    Ok(())
}

fn span_to_json(span: &Span) -> Value {
    json!({
        "file_id": span.file_id.0,
        "start_byte": span.start.0,
        "end_byte": span.end.0,
        "start_line": span.start_line,
        "start_column": span.start_column,
        "dummy": span.is_dummy(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source_map::{BytePos, FileId};

    fn sample_span(start: u32, end: u32) -> Span {
        Span::new(FileId::new(1), BytePos::new(start), BytePos::new(end), 1, start + 1)
    }

    fn sample_diagnostic(message: &str, start: u32) -> Diagnostic {
        Diagnostic::new(
            DiagnosticSeverity::Error,
            DiagnosticCode::new("classical.test.invalid").expect("valid test code"),
            DiagnosticPhase::new(DiagnosticPhase::CLASSICAL).expect("valid phase"),
            DiagnosticDomain::new("classical").expect("valid domain"),
            message,
        )
        .with_primary_label(sample_span(start, start + 1), "here")
        .expect("valid label")
    }

    #[test]
    fn rejects_empty_and_malformed_codes() {
        assert!(DiagnosticCode::new("").is_err());
        assert!(DiagnosticCode::new("bad code").is_err());
        assert!(DiagnosticCode::new("classical.invalid_input").is_ok());
    }

    #[test]
    fn supports_extensible_phase_and_domain_identifiers() {
        assert_eq!(
            DiagnosticPhase::new("future_analysis.phase-2")
                .expect("valid phase")
                .as_str(),
            "future_analysis.phase-2"
        );
        assert!(DiagnosticDomain::new("").is_err());
    }

    #[test]
    fn allows_global_diagnostics_without_source_labels() {
        let diagnostic = Diagnostic::new(
            DiagnosticSeverity::Error,
            DiagnosticCode::new("classical.resource_limit_exceeded").expect("valid code"),
            DiagnosticPhase::new(DiagnosticPhase::RESOURCE_ANALYSIS).expect("valid phase"),
            DiagnosticDomain::new("classical").expect("valid domain"),
            "configured work budget was exceeded",
        );

        assert!(diagnostic.primary_span().is_none());
    }

    #[test]
    fn rejects_reversed_spans() {
        assert!(DiagnosticLabel::new(
            LabelKind::Primary,
            sample_span(9, 3),
            "invalid",
        )
        .is_err());
    }

    #[test]
    fn rejects_multiple_primary_labels() {
        let diagnostic = sample_diagnostic("problem", 0);
        assert!(diagnostic
            .with_primary_label(sample_span(4, 5), "another primary")
            .is_err());
    }

    #[test]
    fn converts_classical_error_without_parsing_its_message() {
        let error = ClassicalError::invalid_input("example");
        let diagnostic = Diagnostic::from_classical_error(
            &error,
            DiagnosticPhase::new(DiagnosticPhase::CLASSICAL).expect("valid phase"),
        )
        .expect("valid conversion");

        assert_eq!(diagnostic.code.as_str(), error.code());
        assert_eq!(diagnostic.domain.as_str(), "classical");
        assert!(diagnostic.is_error());
    }

    #[test]
    fn json_output_contains_machine_readable_location_and_code() {
        let diagnostic = sample_diagnostic("problem", 3);
        let value = diagnostic.to_json_value();

        assert_eq!(value["code"], "classical.test.invalid");
        assert_eq!(value["labels"][0]["kind"], "primary");
        assert_eq!(value["labels"][0]["span"]["start_byte"], 3);
    }

    #[test]
    fn collection_suppresses_exact_duplicates() {
        let mut store = DiagnosticStore::default();

        assert_eq!(
            store.insert(sample_diagnostic("problem", 0)).expect("insert"),
            InsertOutcome::Inserted
        );
        assert_eq!(
            store.insert(sample_diagnostic("problem", 0)).expect("duplicate"),
            InsertOutcome::DuplicateSuppressed
        );
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn configured_limit_is_explicit_and_never_silently_ignored() {
        let mut store = DiagnosticStore::new(DiagnosticPolicy::with_limit(1));

        store.insert(sample_diagnostic("first", 0)).expect("first");
        assert_eq!(
            store.insert(sample_diagnostic("second", 2)),
            Err(DiagnosticStoreError::LimitReached { limit: 1 })
        );
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn deterministic_sort_uses_source_positions_and_stable_tiebreakers() {
        let mut store = DiagnosticStore::default();
        store.insert(sample_diagnostic("later", 20)).expect("insert");
        store.insert(sample_diagnostic("earlier", 2)).expect("insert");

        store.sort_deterministically();

        assert_eq!(store.as_slice()[0].message, "earlier");
        assert_eq!(store.as_slice()[1].message, "later");
    }

    #[test]
    fn fix_its_are_data_and_do_not_modify_source() {
        let fix = DiagnosticFix::new(
            sample_span(0, 1),
            "x",
            FixApplicability::MachineApplicable,
        )
        .expect("valid fix");

        let diagnostic = sample_diagnostic("problem", 0).with_fix(fix);
        assert_eq!(diagnostic.fixes[0].replacement, "x");
    }
}
