
#![forbid(unsafe_code)]

//! Structured, deterministic diagnostics for the Zamani frontend lexer.
//!
//! File: src/frontend/lexer/diagnostics.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide the canonical user-facing representation of lexical diagnostics.
//!
//! # Responsibilities
//!
//! This module owns:
//! - stable lexical diagnostic codes;
//! - diagnostic severity and classification;
//! - primary and secondary source labels;
//! - explanatory notes and remediation suggestions;
//! - bounded diagnostic collection;
//! - explicit reporting of diagnostic truncation;
//! - deterministic diagnostic ordering;
//! - conversion-independent formatting of diagnostic information.
//!
//! This module does not own:
//! - source text or source-file identity;
//! - cursor movement or source-coordinate calculation;
//! - token identity, token spelling, or token construction;
//! - lexical recognition or scanner state;
//! - recovery and synchronization algorithms;
//! - parser or semantic diagnostics;
//! - target discovery, hardware limits, or resource allocation;
//! - rendering terminal colors or editor-specific protocols.
//!
//! # Integration contract
//!
//! - `span.rs` provides the canonical half-open `Span` type.
//! - `cursor.rs` defines source-position coordinates.
//! - `config.rs` provides `LexerLimits` and `ResourceLimit`.
//! - `scanner.rs` remains responsible for recognizing malformed input
//!   and identifying the corresponding source range.
//! - `recovery.rs` remains responsible for continuing after malformed input.
//! - `api.rs` exposes diagnostics through the frontend API as required by
//!   the scanner's eventual public error contract.
//! - `tests/diagnostics.rs` verifies stable codes, spans, ordering, and limits.
//! - `grammar/specification/lexical.md` defines normative lexical behavior.
//! - `grammar/lexer/diagnostics.md` and `grammar/spec/diagnostics.md`
//!   are the intended specification-level diagnostic registries.
//!
//! Scanner failures and user-facing diagnostics are related but distinct.
//! This module does not replace `ScannerError` or `ScannerErrorKind`.
//!
//! # Resource policy
//!
//! Diagnostic limits are implementation budgets, not language-level limits.
//! `ResourceLimit::Unlimited` does not promise infinite physical memory.
//! When a bounded collection fills, additional diagnostics are rejected
//! and counted rather than silently replacing previously retained entries.
//! Callers can inspect `dropped_count()` and `is_truncated()`.
//!
//! No hardware capacity, token count, or computational-domain capacity is
//! hard-coded here.

use core::cmp::Ordering;
use core::fmt;
use std::error::Error;

use super::config::{LexerLimits, ResourceLimit};
use super::span::Span;

/// Stable classification of diagnostic importance.
///
/// Severity is presentation and tooling metadata. It does not itself decide
/// whether scanning continues or whether compilation succeeds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum DiagnosticSeverity {
    /// Additional information that does not indicate a problem.
    Note,

    /// A condition worth bringing to the user's attention.
    Warning,

    /// An error in the source or its lexical interpretation.
    Error,

    /// A fatal frontend condition that prevents the current operation
    /// from proceeding safely or meaningfully.
    Fatal,
}

impl DiagnosticSeverity {
    /// Returns whether this severity represents an error condition.
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error | Self::Fatal)
    }

    /// Returns whether this severity prevents ordinary successful
    /// completion of the current compilation stage.
    #[must_use]
    pub const fn prevents_success(self) -> bool {
        self.is_error()
    }
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Note => "note",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Fatal => "fatal error",
        })
    }
}

/// Stable machine-readable lexical diagnostic identifier.
///
/// The numeric identifiers are part of the diagnostic compatibility
/// contract. Their meanings must not be silently reassigned between
/// releases. New codes should be added without changing existing meanings.
///
/// Normative registry files should document the public lifecycle and
/// compatibility status of each code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum DiagnosticCode {
    /// An unrecognized or prohibited source character.
    InvalidCharacter,

    /// An invalid identifier or identifier encoding.
    InvalidIdentifier,

    /// An invalid numeric literal.
    InvalidNumber,

    /// An invalid string literal.
    InvalidString,

    /// An invalid character literal.
    InvalidCharacterLiteral,

    /// An unterminated string, character, or comment construct.
    UnterminatedConstruct,

    /// An invalid or unsupported escape sequence.
    InvalidEscape,

    /// An invalid operator or delimiter.
    InvalidOperator,

    /// An invalid annotation or attribute.
    InvalidAnnotation,

    /// A prohibited or malformed Unicode scalar.
    InvalidUnicode,

    /// A byte-order mark is prohibited at this position.
    InvalidByteOrderMark,

    /// A configured source-size budget was exceeded.
    SourceLimitExceeded,

    /// A configured token-count budget was exceeded.
    TokenLimitExceeded,

    /// A configured token-size budget was exceeded.
    TokenSizeLimitExceeded,

    /// A configured literal-size budget was exceeded.
    LiteralLimitExceeded,

    /// A configured numeric-digit budget was exceeded.
    NumericLimitExceeded,

    /// A configured lexical nesting-depth budget was exceeded.
    NestingLimitExceeded,

    /// The diagnostic-retention budget was reached.
    DiagnosticLimitExceeded,

    /// The scanner detected an internal invariant violation.
    InternalInvariant,

    /// The scanner's internal counters could not represent a result.
    CounterOverflow,

    /// A caller or frontend integration supplied inconsistent state.
    InvalidFrontendState,

    /// A compatibility adapter encountered an unsupported mapping.
    CompatibilityFailure,

    /// A domain-neutral catch-all for a lexical failure that has no
    /// more specific stable classification yet.
    Other,
}

impl DiagnosticCode {
    /// Returns the stable public diagnostic identifier.
    ///
    /// Codes are intentionally independent of message wording.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidCharacter => "LEX001",
            Self::InvalidIdentifier => "LEX002",
            Self::InvalidNumber => "LEX003",
            Self::InvalidString => "LEX004",
            Self::InvalidCharacterLiteral => "LEX005",
            Self::UnterminatedConstruct => "LEX006",
            Self::InvalidEscape => "LEX007",
            Self::InvalidOperator => "LEX008",
            Self::InvalidAnnotation => "LEX009",
            Self::InvalidUnicode => "LEX010",
            Self::InvalidByteOrderMark => "LEX011",
            Self::SourceLimitExceeded => "LEX012",
            Self::TokenLimitExceeded => "LEX013",
            Self::TokenSizeLimitExceeded => "LEX014",
            Self::LiteralLimitExceeded => "LEX015",
            Self::NumericLimitExceeded => "LEX016",
            Self::NestingLimitExceeded => "LEX017",
            Self::DiagnosticLimitExceeded => "LEX018",
            Self::InternalInvariant => "LEX019",
            Self::CounterOverflow => "LEX020",
            Self::InvalidFrontendState => "LEX021",
            Self::CompatibilityFailure => "LEX022",
            Self::Other => "LEX099",
        }
    }

    /// Returns a short, stable classification label.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::InvalidCharacter => "invalid-character",
            Self::InvalidIdentifier => "invalid-identifier",
            Self::InvalidNumber => "invalid-number",
            Self::InvalidString => "invalid-string",
            Self::InvalidCharacterLiteral => "invalid-character-literal",
            Self::UnterminatedConstruct => "unterminated-construct",
            Self::InvalidEscape => "invalid-escape",
            Self::InvalidOperator => "invalid-operator",
            Self::InvalidAnnotation => "invalid-annotation",
            Self::InvalidUnicode => "invalid-unicode",
            Self::InvalidByteOrderMark => "invalid-byte-order-mark",
            Self::SourceLimitExceeded => "source-limit-exceeded",
            Self::TokenLimitExceeded => "token-limit-exceeded",
            Self::TokenSizeLimitExceeded => "token-size-limit-exceeded",
            Self::LiteralLimitExceeded => "literal-limit-exceeded",
            Self::NumericLimitExceeded => "numeric-limit-exceeded",
            Self::NestingLimitExceeded => "nesting-limit-exceeded",
            Self::DiagnosticLimitExceeded => "diagnostic-limit-exceeded",
            Self::InternalInvariant => "internal-invariant",
            Self::CounterOverflow => "counter-overflow",
            Self::InvalidFrontendState => "invalid-frontend-state",
            Self::CompatibilityFailure => "compatibility-failure",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// The role of a source label attached to a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LabelKind {
    /// The main source range responsible for the diagnostic.
    Primary,

    /// An additional range that provides context.
    Secondary,
}

impl fmt::Display for LabelKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
        })
    }
}

/// A source label associated with a diagnostic.
///
/// Labels use `Span`, ensuring that the lexer and downstream frontend
/// components share one source-coordinate representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiagnosticLabel {
    kind: LabelKind,
    span: Span,
    message: String,
}

impl DiagnosticLabel {
    /// Creates a source label.
    #[must_use]
    pub fn new(
        kind: LabelKind,
        span: Span,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            span,
            message: message.into(),
        }
    }

    /// Returns the label role.
    #[must_use]
    pub const fn kind(&self) -> LabelKind {
        self.kind
    }

    /// Returns the labelled source range.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Returns the label's explanatory text.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// A supplemental note attached to a diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiagnosticNote {
    message: String,
}

impl DiagnosticNote {
    /// Creates a note.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Returns the note text.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// One structured lexical diagnostic.
///
/// The primary span is always available, including for zero-width locations
/// such as EOF. Labels may provide more specific rendering information, but
/// do not replace the canonical primary span.
///
/// Messages are intended for people. Consumers should use `code()` and
/// `severity()` for stable programmatic decisions rather than parsing text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Diagnostic {
    code: DiagnosticCode,
    severity: DiagnosticSeverity,
    message: String,
    primary_span: Span,
    labels: Vec<DiagnosticLabel>,
    notes: Vec<DiagnosticNote>,
    help: Option<String>,
}

impl Diagnostic {
    /// Creates a diagnostic with a stable code, severity, message, and span.
    #[must_use]
    pub fn new(
        code: DiagnosticCode,
        severity: DiagnosticSeverity,
        message: impl Into<String>,
        primary_span: Span,
    ) -> Self {
        Self {
            code,
            severity,
            message: message.into(),
            primary_span,
            labels: Vec::new(),
            notes: Vec::new(),
            help: None,
        }
    }

    /// Creates an error diagnostic.
    #[must_use]
    pub fn error(
        code: DiagnosticCode,
        message: impl Into<String>,
        primary_span: Span,
    ) -> Self {
        Self::new(code, DiagnosticSeverity::Error, message, primary_span)
    }

    /// Creates a fatal diagnostic.
    #[must_use]
    pub fn fatal(
        code: DiagnosticCode,
        message: impl Into<String>,
        primary_span: Span,
    ) -> Self {
        Self::new(code, DiagnosticSeverity::Fatal, message, primary_span)
    }

    /// Creates a warning diagnostic.
    #[must_use]
    pub fn warning(
        code: DiagnosticCode,
        message: impl Into<String>,
        primary_span: Span,
    ) -> Self {
        Self::new(code, DiagnosticSeverity::Warning, message, primary_span)
    }

    /// Creates an informational diagnostic.
    #[must_use]
    pub fn note(
        code: DiagnosticCode,
        message: impl Into<String>,
        primary_span: Span,
    ) -> Self {
        Self::new(code, DiagnosticSeverity::Note, message, primary_span)
    }

    /// Returns the stable diagnostic code.
    #[must_use]
    pub const fn code(&self) -> DiagnosticCode {
        self.code
    }

    /// Returns the stable code string.
    #[must_use]
    pub const fn code_str(&self) -> &'static str {
        self.code.as_str()
    }

    /// Returns the diagnostic severity.
    #[must_use]
    pub const fn severity(&self) -> DiagnosticSeverity {
        self.severity
    }

    /// Returns the human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the primary source span.
    #[must_use]
    pub const fn primary_span(&self) -> Span {
        self.primary_span
    }

    /// Returns all source labels in insertion order.
    #[must_use]
    pub fn labels(&self) -> &[DiagnosticLabel] {
        &self.labels
    }

    /// Returns all supplemental notes in insertion order.
    #[must_use]
    pub fn notes(&self) -> &[DiagnosticNote] {
        &self.notes
    }

    /// Returns the optional remediation suggestion.
    #[must_use]
    pub fn help(&self) -> Option<&str> {
        self.help.as_deref()
    }

    /// Returns whether the diagnostic is an error or fatal error.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        self.severity.is_error()
    }

    /// Adds a secondary source label.
    ///
    /// The primary span remains the diagnostic's canonical location.
    #[must_use]
    pub fn with_secondary_label(
        mut self,
        span: Span,
        message: impl Into<String>,
    ) -> Self {
        self.labels.push(DiagnosticLabel::new(
            LabelKind::Secondary,
            span,
            message,
        ));
        self
    }

    /// Adds a primary label.
    ///
    /// The diagnostic's `primary_span` remains authoritative; callers should
    /// normally use the same span for this label.
    #[must_use]
    pub fn with_primary_label(
        mut self,
        span: Span,
        message: impl Into<String>,
    ) -> Self {
        self.labels.push(DiagnosticLabel::new(
            LabelKind::Primary,
            span,
            message,
        ));
        self
    }

    /// Adds an explanatory note.
    #[must_use]
    pub fn with_note(mut self, message: impl Into<String>) -> Self {
        self.notes.push(DiagnosticNote::new(message));
        self
    }

    /// Adds a remediation suggestion.
    #[must_use]
    pub fn with_help(mut self, message: impl Into<String>) -> Self {
        self.help = Some(message.into());
        self
    }

    /// Compares diagnostics using the canonical deterministic ordering.
    ///
    /// This method does not sort or mutate either diagnostic.
    #[must_use]
    pub fn deterministic_cmp(&self, other: &Self) -> Ordering {
        self.primary_span
            .start_byte()
            .cmp(&other.primary_span.start_byte())
            .then_with(|| {
                self.primary_span
                    .end_byte()
                    .cmp(&other.primary_span.end_byte())
            })
            .then_with(|| self.code.cmp(&other.code))
            .then_with(|| self.severity.cmp(&other.severity))
            .then_with(|| self.message.cmp(&other.message))
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}[{}]: {} at {}",
            self.severity, self.code, self.message, self.primary_span
        )
    }
}

impl Error for Diagnostic {}

/// Result of attempting to retain a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PushOutcome {
    /// The diagnostic was retained.
    Added,

    /// The diagnostic was rejected because the configured retention budget
    /// had already been reached.
    LimitReached,

    /// The retention counter could not be incremented.
    ///
    /// This is a representational edge case, not a language restriction.
    CounterOverflow,
}

/// A bounded or unbounded collection of diagnostics.
///
/// The bag preserves insertion order until `sort_deterministically()` is
/// requested. It never silently evicts an earlier diagnostic to make room
/// for a later one.
///
/// A bounded bag does not append a synthetic truncation diagnostic after
/// reaching its limit, because doing so would violate the configured
/// maximum. Instead, callers inspect `is_truncated()` and `dropped_count()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticBag {
    limit: ResourceLimit,
    diagnostics: Vec<Diagnostic>,
    dropped_count: usize,
    counter_overflowed: bool,
}

impl DiagnosticBag {
    /// Creates an empty bag using an explicit retention policy.
    #[must_use]
    pub fn new(limit: ResourceLimit) -> Self {
        Self {
            limit,
            diagnostics: Vec::new(),
            dropped_count: 0,
            counter_overflowed: false,
        }
    }

    /// Creates an empty bag using the diagnostic budget in `LexerLimits`.
    #[must_use]
    pub fn from_limits(limits: &LexerLimits) -> Self {
        Self::new(limits.diagnostics)
    }

    /// Creates an unbounded bag.
    ///
    /// This disables the explicit diagnostic-count budget. It does not
    /// guarantee unlimited memory or successful allocation.
    #[must_use]
    pub fn unlimited() -> Self {
        Self::new(ResourceLimit::Unlimited)
    }

    /// Returns the configured retention limit.
    #[must_use]
    pub const fn limit(&self) -> ResourceLimit {
        self.limit
    }

    /// Returns retained diagnostics in their current order.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Returns the number of retained diagnostics.
    #[must_use]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Returns whether no diagnostics have been retained.
    ///
    /// A bag may be empty even if diagnostics were dropped.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Returns the number of diagnostics that were rejected.
    ///
    /// If this counter itself overflowed, the returned value is the largest
    /// representable count and `counter_overflowed()` reports the condition.
    #[must_use]
    pub const fn dropped_count(&self) -> usize {
        self.dropped_count
    }

    /// Returns whether one or more diagnostics were rejected.
    #[must_use]
    pub const fn is_truncated(&self) -> bool {
        self.dropped_count > 0 || self.counter_overflowed
    }

    /// Returns whether the dropped-diagnostic counter overflowed.
    #[must_use]
    pub const fn counter_overflowed(&self) -> bool {
        self.counter_overflowed
    }

    /// Returns whether at least one retained diagnostic is an error.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(Diagnostic::is_error)
    }

    /// Returns the number of retained error and fatal diagnostics.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.is_error())
            .count()
    }

    /// Attempts to retain a diagnostic.
    ///
    /// The retention limit is inclusive. If the bag has already retained
    /// the maximum permitted number, the new diagnostic is dropped and the
    /// dropped count is incremented.
    ///
    /// The caller remains responsible for deciding whether scanning or
    /// recovery should continue after a diagnostic is dropped.
    pub fn push(&mut self, diagnostic: Diagnostic) -> PushOutcome {
        if self.limit.is_exceeded_by(self.diagnostics.len()) {
            return self.record_drop();
        }

        self.diagnostics.push(diagnostic);
        PushOutcome::Added
    }

    /// Attempts to retain a diagnostic and returns whether it was added.
    ///
    /// Use `push()` instead when the reason for rejection matters.
    pub fn push_if_permitted(&mut self, diagnostic: Diagnostic) -> bool {
        self.push(diagnostic) == PushOutcome::Added
    }

    /// Appends diagnostics from another bag in the other bag's current order.
    ///
    /// Each diagnostic is subject to this bag's retention policy. The
    /// other bag's truncation metadata is not implicitly merged; inspect it
    /// separately if its dropped diagnostics must be accounted for.
    pub fn extend<I>(&mut self, diagnostics: I) -> usize
    where
        I: IntoIterator<Item = Diagnostic>,
    {
        let mut added = 0usize;

        for diagnostic in diagnostics {
            if self.push(diagnostic) == PushOutcome::Added {
                added = added.saturating_add(1);
            }
        }

        added
    }

    /// Sorts retained diagnostics into a deterministic source-oriented order.
    ///
    /// Equal comparison keys retain their relative order through stable
    /// sorting. Labels and notes retain their own insertion order.
    pub fn sort_deterministically(&mut self) {
        self.diagnostics
            .sort_by(Diagnostic::deterministic_cmp);
    }

    /// Removes all retained diagnostics and resets truncation metadata.
    ///
    /// The configured limit is preserved.
    pub fn clear(&mut self) {
        self.diagnostics.clear();
        self.dropped_count = 0;
        self.counter_overflowed = false;
    }

    fn record_drop(&mut self) -> PushOutcome {
        match self.dropped_count.checked_add(1) {
            Some(count) => {
                self.dropped_count = count;
                PushOutcome::LimitReached
            }
            None => {
                self.counter_overflowed = true;
                PushOutcome::CounterOverflow
            }
        }
    }
}

impl Default for DiagnosticBag {
    /// Defaults to an unbounded explicit diagnostic budget.
    ///
    /// Applications processing untrusted input should prefer
    /// `DiagnosticBag::from_limits` with a configured budget.
    fn default() -> Self {
        Self::unlimited()
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
    type IntoIter = core::slice::Iter<'a, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Diagnostic, DiagnosticBag, DiagnosticCode, DiagnosticSeverity, LabelKind,
        PushOutcome,
    };
    use crate::frontend::lexer::config::ResourceLimit;
    use crate::frontend::lexer::cursor::{Cursor, CursorPosition};
    use crate::frontend::lexer::span::Span;

    fn span(start: usize, end: usize) -> Span {
        let source = "abcdefghij";
        let mut cursor = Cursor::new(source);
        let start_position = cursor
            .position_at(start)
            .expect("start must be a valid source boundary");
        let end_position = cursor
            .position_at(end)
            .expect("end must be a valid source boundary");
        Span::try_new(start_position, end_position).expect("valid span")
    }

    fn diagnostic(
        code: DiagnosticCode,
        severity: DiagnosticSeverity,
        start: usize,
        end: usize,
        message: &str,
    ) -> Diagnostic {
        Diagnostic::new(code, severity, message, span(start, end))
    }

    #[test]
    fn diagnostic_codes_are_stable_and_distinct() {
        assert_eq!(DiagnosticCode::InvalidCharacter.as_str(), "LEX001");
        assert_eq!(DiagnosticCode::InvalidNumber.as_str(), "LEX003");
        assert_eq!(DiagnosticCode::SourceLimitExceeded.as_str(), "LEX012");
        assert_eq!(DiagnosticCode::Other.as_str(), "LEX099");

        assert_ne!(
            DiagnosticCode::InvalidCharacter.as_str(),
            DiagnosticCode::InvalidNumber.as_str()
        );
    }

    #[test]
    fn diagnostic_preserves_message_severity_and_span() {
        let item = diagnostic(
            DiagnosticCode::InvalidNumber,
            DiagnosticSeverity::Error,
            2,
            5,
            "invalid numeric literal",
        );

        assert_eq!(item.code(), DiagnosticCode::InvalidNumber);
        assert_eq!(item.code_str(), "LEX003");
        assert_eq!(item.severity(), DiagnosticSeverity::Error);
        assert!(item.is_error());
        assert_eq!(item.message(), "invalid numeric literal");
        assert_eq!(item.primary_span().byte_range(), 2..5);
    }

    #[test]
    fn permits_zero_width_eof_diagnostics() {
        let eof = span(10, 10);
        let item = Diagnostic::error(
            DiagnosticCode::UnterminatedConstruct,
            "expected a closing delimiter",
            eof,
        );

        assert!(item.primary_span().is_empty());
        assert_eq!(item.primary_span().start_byte(), 10);
    }

    #[test]
    fn supports_primary_and_secondary_labels() {
        let item = Diagnostic::error(
            DiagnosticCode::InvalidCharacter,
            "invalid source character",
            span(2, 3),
        )
        .with_primary_label(span(2, 3), "unexpected character")
        .with_secondary_label(span(0, 1), "related source context");

        assert_eq!(item.labels().len(), 2);
        assert_eq!(item.labels()[0].kind(), LabelKind::Primary);
        assert_eq!(item.labels()[1].kind(), LabelKind::Secondary);
        assert_eq!(item.labels()[1].span().byte_range(), 0..1);
    }

    #[test]
    fn supports_notes_and_help() {
        let item = Diagnostic::warning(
            DiagnosticCode::InvalidEscape,
            "unrecognized escape sequence",
            span(1, 3),
        )
        .with_note("Escape sequences are interpreted lexically.")
        .with_help("Use a supported escape sequence.");

        assert_eq!(item.notes().len(), 1);
        assert_eq!(
            item.notes()[0].message(),
            "Escape sequences are interpreted lexically."
        );
        assert_eq!(item.help(), Some("Use a supported escape sequence."));
    }

    #[test]
    fn formats_diagnostics_with_stable_code_and_location() {
        let item = diagnostic(
            DiagnosticCode::InvalidCharacter,
            DiagnosticSeverity::Error,
            0,
            1,
            "unexpected character",
        );

        let formatted = item.to_string();

        assert!(formatted.contains("error"));
        assert!(formatted.contains("LEX001"));
        assert!(formatted.contains("unexpected character"));
        assert!(formatted.contains("bytes 0..1"));
    }

    #[test]
    fn unbounded_bag_retains_all_diagnostics() {
        let mut bag = DiagnosticBag::unlimited();

        for offset in 0..100 {
            let item = diagnostic(
                DiagnosticCode::InvalidCharacter,
                DiagnosticSeverity::Error,
                offset % 10,
                (offset % 10) + 1,
                "invalid character",
            );
            assert_eq!(bag.push(item), PushOutcome::Added);
        }

        assert_eq!(bag.len(), 100);
        assert_eq!(bag.dropped_count(), 0);
        assert!(!bag.is_truncated());
        assert!(bag.has_errors());
        assert_eq!(bag.error_count(), 100);
    }

    #[test]
    fn bounded_bag_never_exceeds_its_retention_budget() {
        let limit = ResourceLimit::at_most(2).expect("positive limit");
        let mut bag = DiagnosticBag::new(limit);

        assert_eq!(
            bag.push(diagnostic(
                DiagnosticCode::InvalidCharacter,
                DiagnosticSeverity::Error,
                0,
                1,
                "first",
            )),
            PushOutcome::Added
        );
        assert_eq!(
            bag.push(diagnostic(
                DiagnosticCode::InvalidNumber,
                DiagnosticSeverity::Error,
                1,
                2,
                "second",
            )),
            PushOutcome::Added
        );
        assert_eq!(
            bag.push(diagnostic(
                DiagnosticCode::InvalidString,
                DiagnosticSeverity::Error,
                2,
                3,
                "third",
            )),
            PushOutcome::LimitReached
        );

        assert_eq!(bag.len(), 2);
        assert_eq!(bag.dropped_count(), 1);
        assert!(bag.is_truncated());
    }

    #[test]
    fn unlimited_budget_accepts_diagnostics() {
        let mut bag = DiagnosticBag::new(ResourceLimit::Unlimited);

        for _ in 0..5 {
            assert_eq!(
                bag.push(diagnostic(
                    DiagnosticCode::Other,
                    DiagnosticSeverity::Note,
                    0,
                    0,
                    "information",
                )),
                PushOutcome::Added
            );
        }

        assert_eq!(bag.len(), 5);
        assert!(!bag.has_errors());
    }

    #[test]
    fn deterministic_sort_uses_source_location_then_code() {
        let mut bag = DiagnosticBag::unlimited();

        bag.push(diagnostic(
            DiagnosticCode::InvalidNumber,
            DiagnosticSeverity::Error,
            4,
            6,
            "number",
        ));
        bag.push(diagnostic(
            DiagnosticCode::InvalidString,
            DiagnosticSeverity::Error,
            0,
            3,
            "string",
        ));
        bag.push(diagnostic(
            DiagnosticCode::InvalidCharacter,
            DiagnosticSeverity::Warning,
            0,
            3,
            "character",
        ));

        bag.sort_deterministically();

        assert_eq!(
            bag.diagnostics()[0].code(),
            DiagnosticCode::InvalidCharacter
        );
        assert_eq!(
            bag.diagnostics()[1].code(),
            DiagnosticCode::InvalidString
        );
        assert_eq!(
            bag.diagnostics()[2].code(),
            DiagnosticCode::InvalidNumber
        );
    }

    #[test]
    fn clear_resets_diagnostics_and_truncation_metadata() {
        let limit = ResourceLimit::at_most(1).expect("positive limit");
        let mut bag = DiagnosticBag::new(limit);

        bag.push(diagnostic(
            DiagnosticCode::Other,
            DiagnosticSeverity::Note,
            0,
            0,
            "retained",
        ));
        bag.push(diagnostic(
            DiagnosticCode::Other,
            DiagnosticSeverity::Note,
            0,
            0,
            "dropped",
        ));

        assert!(bag.is_truncated());

        bag.clear();

        assert!(bag.is_empty());
        assert_eq!(bag.dropped_count(), 0);
        assert!(!bag.is_truncated());
        assert_eq!(bag.limit(), limit);
    }

    #[test]
    fn severity_distinguishes_warnings_from_errors() {
        let warning = diagnostic(
            DiagnosticCode::Other,
            DiagnosticSeverity::Warning,
            0,
            1,
            "warning",
        );
        let fatal = diagnostic(
            DiagnosticCode::InternalInvariant,
            DiagnosticSeverity::Fatal,
            0,
            1,
            "fatal",
        );

        assert!(!warning.is_error());
        assert!(!warning.severity().prevents_success());
        assert!(fatal.is_error());
        assert!(fatal.severity().prevents_success());
    }

    #[test]
    fn span_positions_use_the_shared_cursor_contract() {
        let source = "aλ界";
        let cursor = Cursor::new(source);
        let start = cursor.position();
        let end = CursorPosition::new(source.len(), 1, 4);
        let source_span = Span::try_new(start, end).expect("valid span");

        let item = Diagnostic::error(
            DiagnosticCode::InvalidCharacter,
            "unexpected source character",
            source_span,
        );

        assert_eq!(item.primary_span().byte_range(), 0..source.len());
    }

    #[test]
    fn bag_iteration_preserves_current_order() {
        let mut bag = DiagnosticBag::unlimited();

        bag.push(diagnostic(
            DiagnosticCode::InvalidNumber,
            DiagnosticSeverity::Error,
            0,
            1,
            "number",
        ));
        bag.push(diagnostic(
            DiagnosticCode::InvalidString,
            DiagnosticSeverity::Error,
            1,
            2,
            "string",
        ));

        let codes: Vec<_> = bag.iter().map(Diagnostic::code).collect();

        assert_eq!(
            codes,
            vec![
                DiagnosticCode::InvalidNumber,
                DiagnosticCode::InvalidString
            ]
        );
    }
}
