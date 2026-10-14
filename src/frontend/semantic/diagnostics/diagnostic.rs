
#![forbid(unsafe_code)]

//! Structured semantic diagnostics for the native Zamani frontend.
//!
//! File: `src/frontend/semantic/diagnostics/diagnostic.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Represent semantic diagnostics as structured, deterministic, source-aware
//! data. Diagnostics describe conditions reported by semantic analysis; they
//! do not define language semantics, perform recovery, or decide whether a
//! target can realize a program.
//!
//! # Authority
//!
//! The normative diagnostic contract is `grammar/spec/diagnostics.md`.
//! The canonical source-location authority is
//! `src/frontend/ast/node/source/span.rs`.
//! The canonical syntax authority is `grammar/Zamani.g4`, interpreted
//! together with the normative specifications and feature contracts.
//!
//! # Ownership
//!
//! This file owns:
//! - the semantic diagnostic record and its foundational enums;
//! - stable textual diagnostic-code representation;
//! - primary and related source labels;
//! - structured fix-it edits;
//! - typed, deterministic diagnostic metadata;
//! - deterministic ordering and identity helpers;
//! - display formatting for human-readable output.
//!
//! It does not own:
//! - AST nodes, source maps, or source-coordinate conversion;
//! - parser or lexer errors;
//! - type, effect, capability, resource, or domain rules;
//! - semantic recovery decisions;
//! - target feasibility, resource allocation, or backend diagnostics;
//! - diagnostic collection budgets or session state.
//!
//! # Integration contract
//!
//! - Uses the canonical AST `Span`; it defines no replacement span type.
//! - `codes.rs` may define constants for stable code strings.
//! - `labels.rs` may re-export or later own label-related APIs through an
//!   explicit compatibility migration.
//! - `ordering.rs` may delegate to `Diagnostic::ordering_key` and
//!   `sort_diagnostics`; ordering logic must have one authoritative owner.
//! - The semantic session owns diagnostic accumulation and configured budgets.
//! - The semantic pipeline propagates incomplete analysis explicitly.
//! - The parser's existing diagnostic types remain parser-owned until an
//!   explicit, tested frontend-wide diagnostic migration is approved.
//! - No dependency on the legacy `crate::semantic` analyzer is permitted.
//! - No dependency on IR, quantum hardware, scheduling, routing, runtime,
//!   backend SDKs, or physical resource handles is permitted.
//!
//! # Scalability
//!
//! No hardware-capacity or computational-domain ceiling is introduced here.
//! Source coordinates retain the canonical `u64`-based Span representation.
//! Collections use normal Rust allocation and are therefore constrained by
//! the host's available resources. Callers processing untrusted input should
//! apply explicit diagnostic and analysis budgets in their owning session.
//!
//! # Determinism
//!
//! Ordering never depends on hash-map iteration or allocation addresses.
//! Metadata uses an ordered map. Labels, notes, and edits retain source order.
//! Equal ordering keys are left equivalent; callers requiring a stable order
//! for otherwise indistinguishable records should preserve deterministic
//! pass emission order or use the full record as a secondary comparison.
//!
//! # Error policy
//!
//! A diagnostic is a report, not proof that analysis succeeded or failed
//! completely. The semantic result must separately represent validity and
//! completeness. Resource-budget exhaustion must not be reported as successful
//! complete analysis.

use core::cmp::Ordering;
use core::fmt;
use std::collections::BTreeMap;

use crate::frontend::ast::node::source::span::Span;

/// A stable, machine-readable semantic diagnostic code.
///
/// Codes are strings rather than a closed enum so extensions can introduce
/// namespaced diagnostics without modifying this foundational record type.
/// Stability and uniqueness of registered codes are enforced by the code
/// registry, not by parsing human-readable messages.
///
/// Prefer namespaced codes such as `semantic.name.unresolved` or
/// `semantic.quantum.invalid_measurement`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DiagnosticCode(String);

impl DiagnosticCode {
    /// Construct a code without restricting the set of future extensions.
    ///
    /// Empty values are retained rather than causing a panic. Registries and
    /// API boundaries should reject empty codes when registering or publishing
    /// diagnostics, where that failure can be reported explicitly.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the stable code text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether this code contains non-whitespace text.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.0.trim().is_empty()
    }
}

impl From<&str> for DiagnosticCode {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for DiagnosticCode {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The compilation phase that emitted a diagnostic.
///
/// This is diagnostic provenance, not a command to execute a compiler pass.
/// `Extension` allows registered domains and future phases to report their
/// identity without requiring a core-enum edit.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum DiagnosticPhase {
    Source,
    Lexing,
    Parsing,
    AstValidation,
    NameResolution,
    TypeAnalysis,
    EffectAnalysis,
    OwnershipAnalysis,
    CapabilityAnalysis,
    ResourceAnalysis,
    ContractAnalysis,
    DomainAnalysis,
    SemanticFinalization,
    IrVerification,
    Lowering,
    TargetPlanning,
    Runtime,
    Extension(String),
}

impl DiagnosticPhase {
    /// Stable machine-readable phase name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Source => "source",
            Self::Lexing => "lexing",
            Self::Parsing => "parsing",
            Self::AstValidation => "ast_validation",
            Self::NameResolution => "name_resolution",
            Self::TypeAnalysis => "type_analysis",
            Self::EffectAnalysis => "effect_analysis",
            Self::OwnershipAnalysis => "ownership_analysis",
            Self::CapabilityAnalysis => "capability_analysis",
            Self::ResourceAnalysis => "resource_analysis",
            Self::ContractAnalysis => "contract_analysis",
            Self::DomainAnalysis => "domain_analysis",
            Self::SemanticFinalization => "semantic_finalization",
            Self::IrVerification => "ir_verification",
            Self::Lowering => "lowering",
            Self::TargetPlanning => "target_planning",
            Self::Runtime => "runtime",
            Self::Extension(name) => name,
        }
    }
}

/// Diagnostic severity.
///
/// Severity is independent of recovery and compilation status. In
/// particular, a warning does not prove that a program is portable, and an
/// error does not itself specify whether analysis can continue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum DiagnosticSeverity {
    Hint,
    Note,
    Information,
    Warning,
    Error,
    Fatal,
}

impl DiagnosticSeverity {
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

    /// Stable machine-readable severity name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hint => "hint",
            Self::Note => "note",
            Self::Information => "information",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Fatal => "fatal",
        }
    }
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Broad diagnostic classification.
///
/// Categories describe the nature of the report without replacing the
/// responsible subsystem's semantic rules.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum DiagnosticCategory {
    Syntax,
    NameResolution,
    Type,
    Effect,
    Ownership,
    Capability,
    Resource,
    Contract,
    Domain,
    Portability,
    Configuration,
    Compatibility,
    OperationalLimit,
    Internal,
    Extension(String),
}

impl DiagnosticCategory {
    /// Stable machine-readable category name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Syntax => "syntax",
            Self::NameResolution => "name_resolution",
            Self::Type => "type",
            Self::Effect => "effect",
            Self::Ownership => "ownership",
            Self::Capability => "capability",
            Self::Resource => "resource",
            Self::Contract => "contract",
            Self::Domain => "domain",
            Self::Portability => "portability",
            Self::Configuration => "configuration",
            Self::Compatibility => "compatibility",
            Self::OperationalLimit => "operational_limit",
            Self::Internal => "internal",
            Self::Extension(name) => name,
        }
    }
}

/// Recovery classification associated with a diagnostic.
///
/// This records the producer's recovery assessment. It does not perform
/// recovery or authorize an invalid semantic result to be treated as valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum RecoveryClassification {
    /// The issue is reported; analysis may continue without repair.
    Continue,
    /// The producer or a later pass may recover with an explicit action.
    Recoverable,
    /// Further analysis is unsafe or not meaningful.
    Unrecoverable,
    /// The producer cannot yet determine whether recovery is possible.
    Undetermined,
}

impl RecoveryClassification {
    /// Stable machine-readable recovery name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Continue => "continue",
            Self::Recoverable => "recoverable",
            Self::Unrecoverable => "unrecoverable",
            Self::Undetermined => "undetermined",
        }
    }
}

/// Typed metadata value for diagnostic APIs and tools.
///
/// Metadata is deliberately data-only. Consumers must not parse display text
/// to discover expected types, capabilities, resource requirements, or other
/// information that can be represented structurally.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DiagnosticValue {
    Null,
    Boolean(bool),
    Integer(i128),
    Unsigned(u128),
    Text(String),
    Sequence(Vec<DiagnosticValue>),
    Object(BTreeMap<String, DiagnosticValue>),
}

impl DiagnosticValue {
    /// Construct an ordered metadata object.
    #[must_use]
    pub fn object() -> Self {
        Self::Object(BTreeMap::new())
    }

    /// Insert a key into an object.
    ///
    /// Returns the previous value when the key was already present. For a
    /// non-object value, the method returns the supplied value unchanged and
    /// does not silently convert the metadata shape.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: DiagnosticValue,
    ) -> Option<DiagnosticValue> {
        match self {
            Self::Object(entries) => entries.insert(key.into(), value),
            _ => None,
        }
    }

    /// Get an object field.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&DiagnosticValue> {
        match self {
            Self::Object(entries) => entries.get(key),
            _ => None,
        }
    }
}

impl From<bool> for DiagnosticValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<i128> for DiagnosticValue {
    fn from(value: i128) -> Self {
        Self::Integer(value)
    }
}

impl From<u128> for DiagnosticValue {
    fn from(value: u128) -> Self {
        Self::Unsigned(value)
    }
}

impl From<String> for DiagnosticValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for DiagnosticValue {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

/// A source label attached to a diagnostic.
///
/// The primary location is stored directly on [`Diagnostic`]. This type is
/// for additional locations, such as the previous declaration in a duplicate
/// definition diagnostic or a related expression in a type mismatch.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiagnosticLabel {
    span: Span,
    message: String,
}

impl DiagnosticLabel {
    /// Construct a secondary source label.
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

    /// Return the label text.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// One source edit proposed as part of a diagnostic fix-it.
///
/// The replacement applies to the half-open source span. The consumer must
/// verify source identity, source version, and edit compatibility before
/// applying it. This structure never edits source text itself.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FixItEdit {
    span: Span,
    replacement: String,
}

impl FixItEdit {
    /// Construct a source edit.
    #[must_use]
    pub fn new(span: Span, replacement: impl Into<String>) -> Self {
        Self {
            span,
            replacement: replacement.into(),
        }
    }

    /// Return the span replaced by this edit.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Return the replacement text.
    #[must_use]
    pub fn replacement(&self) -> &str {
        &self.replacement
    }
}

/// A proposed source-level fix.
///
/// A fix is only a suggestion. The diagnostic producer must not mark a fix
/// safe unless it has enough information to establish that the edit addresses
/// the reported issue without silently changing unrelated semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FixIt {
    title: String,
    edits: Vec<FixItEdit>,
    is_safe: bool,
}

impl FixIt {
    /// Construct a fix-it from an ordered set of edits.
    #[must_use]
    pub fn new(
        title: impl Into<String>,
        edits: Vec<FixItEdit>,
        is_safe: bool,
    ) -> Self {
        Self {
            title: title.into(),
            edits,
            is_safe,
        }
    }

    /// Return the human-readable action title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Return edits in the producer's deterministic order.
    #[must_use]
    pub fn edits(&self) -> &[FixItEdit] {
        &self.edits
    }

    /// Whether the producer asserts that the fix is safe.
    #[must_use]
    pub const fn is_safe(&self) -> bool {
        self.is_safe
    }

    /// Whether this fix contains at least one edit.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }
}

/// One structured semantic diagnostic.
///
/// Fields are private so future internal representation changes do not force
/// downstream code to construct inconsistent records by direct field access.
/// Use the constructor and builder methods to create a diagnostic.
///
/// The record intentionally has no target-specific resource allocation,
/// physical device identity, or backend handle.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Diagnostic {
    code: DiagnosticCode,
    phase: DiagnosticPhase,
    category: DiagnosticCategory,
    severity: DiagnosticSeverity,
    message: String,
    primary_span: Option<Span>,
    labels: Vec<DiagnosticLabel>,
    notes: Vec<String>,
    help: Option<String>,
    fixes: Vec<FixIt>,
    recovery: RecoveryClassification,
    metadata: BTreeMap<String, DiagnosticValue>,
}

impl Diagnostic {
    /// Construct a diagnostic with its required identity and message.
    ///
    /// A missing source span is valid for session-level, configuration, or
    /// integration diagnostics that do not originate at a source location.
    #[must_use]
    pub fn new(
        code: impl Into<DiagnosticCode>,
        phase: DiagnosticPhase,
        category: DiagnosticCategory,
        severity: DiagnosticSeverity,
        message: impl Into<String>,
        primary_span: Option<Span>,
    ) -> Self {
        Self {
            code: code.into(),
            phase,
            category,
            severity,
            message: message.into(),
            primary_span,
            labels: Vec::new(),
            notes: Vec::new(),
            help: None,
            fixes: Vec::new(),
            recovery: RecoveryClassification::Undetermined,
            metadata: BTreeMap::new(),
        }
    }

    /// Return the stable machine-readable code.
    #[must_use]
    pub fn code(&self) -> &DiagnosticCode {
        &self.code
    }

    /// Return the emitting phase.
    #[must_use]
    pub const fn phase(&self) -> &DiagnosticPhase {
        &self.phase
    }

    /// Return the diagnostic category.
    #[must_use]
    pub const fn category(&self) -> &DiagnosticCategory {
        &self.category
    }

    /// Return the severity.
    #[must_use]
    pub const fn severity(&self) -> DiagnosticSeverity {
        self.severity
    }

    /// Return the concise human-readable message.
    ///
    /// Tooling must use structured fields and metadata for programmatic
    /// decisions rather than parsing this text.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the primary source span, if available.
    #[must_use]
    pub const fn primary_span(&self) -> Option<Span> {
        self.primary_span
    }

    /// Return secondary source labels in insertion order.
    #[must_use]
    pub fn labels(&self) -> &[DiagnosticLabel] {
        &self.labels
    }

    /// Return explanatory notes in insertion order.
    #[must_use]
    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    /// Return optional remediation guidance.
    #[must_use]
    pub fn help(&self) -> Option<&str> {
        self.help.as_deref()
    }

    /// Return structured fix-it suggestions.
    #[must_use]
    pub fn fixes(&self) -> &[FixIt] {
        &self.fixes
    }

    /// Return the recovery classification.
    #[must_use]
    pub const fn recovery(&self) -> RecoveryClassification {
        self.recovery
    }

    /// Return the ordered machine-readable metadata map.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, DiagnosticValue> {
        &self.metadata
    }

    /// Look up one metadata value.
    #[must_use]
    pub fn metadata_value(&self, key: &str) -> Option<&DiagnosticValue> {
        self.metadata.get(key)
    }

    /// Whether this diagnostic has error severity.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        self.severity.is_error()
    }

    /// Whether this diagnostic is fatal.
    #[must_use]
    pub const fn is_fatal(&self) -> bool {
        self.severity.is_fatal()
    }

    /// Whether the stable code is non-empty.
    ///
    /// API publication and registry boundaries should reject records for
    /// which this returns `false`.
    #[must_use]
    pub fn has_valid_code(&self) -> bool {
        self.code.is_valid()
    }

    /// Attach a secondary source label.
    #[must_use]
    pub fn with_label(
        mut self,
        span: Span,
        message: impl Into<String>,
    ) -> Self {
        self.labels.push(DiagnosticLabel::new(span, message));
        self
    }

    /// Append an explanatory note.
    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Set remediation guidance.
    #[must_use]
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Append a structured fix-it.
    #[must_use]
    pub fn with_fix(mut self, fix: FixIt) -> Self {
        self.fixes.push(fix);
        self
    }

    /// Set the producer's recovery assessment.
    #[must_use]
    pub const fn with_recovery(
        mut self,
        recovery: RecoveryClassification,
    ) -> Self {
        self.recovery = recovery;
        self
    }

    /// Insert machine-readable metadata.
    ///
    /// Keys are ordered lexicographically by the underlying `BTreeMap`.
    /// Inserting an existing key replaces its previous value and returns it.
    #[must_use]
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<DiagnosticValue>,
    ) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Insert an already structured metadata value.
    #[must_use]
    pub fn with_metadata_value(
        mut self,
        key: impl Into<String>,
        value: DiagnosticValue,
    ) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }

    /// Compare diagnostics by the standardized deterministic ordering key.
    ///
    /// Ordering policy: source identity and byte offset first, then phase,
    /// severity, code, category, and message. Source-less diagnostics sort
    /// before source-located diagnostics. This policy is independent of
    /// machine capacity and hash-map iteration.
    #[must_use]
    pub fn compare_for_reporting(&self, other: &Self) -> Ordering {
        diagnostic_ordering_key(self).cmp(&diagnostic_ordering_key(other))
    }

    /// Return whether these diagnostics have the same report identity.
    ///
    /// This conservative identity includes code, phase, category, severity,
    /// primary span, and message. Distinct source locations or messages are
    /// not collapsed. Metadata, notes, labels, and fixes are supplemental
    /// details and do not by themselves create a different report identity.
    #[must_use]
    pub fn same_report_identity(&self, other: &Self) -> bool {
        self.code == other.code
            && self.phase == other.phase
            && self.category == other.category
            && self.severity == other.severity
            && self.primary_span == other.primary_span
            && self.message == other.message
    }
}

fn diagnostic_ordering_key(
    diagnostic: &Diagnostic,
) -> (
    Option<(u64, u64, u64)>,
    &DiagnosticPhase,
    DiagnosticSeverity,
    &DiagnosticCode,
    &DiagnosticCategory,
    &str,
) {
    let source_location = diagnostic.primary_span.map(|span| {
        (
            span.source().as_raw(),
            span.start_raw(),
            span.end_raw(),
        )
    });

    (
        source_location,
        &diagnostic.phase,
        diagnostic.severity,
        &diagnostic.code,
        &diagnostic.category,
        &diagnostic.message,
    )
}

/// Sort diagnostics in deterministic reporting order.
///
/// The sort key never reads global state, pointer addresses, wall-clock time,
/// or hash iteration order. Equal keys compare equal; use deterministic pass
/// emission order to resolve exact ties where required.
pub fn sort_diagnostics(diagnostics: &mut [Diagnostic]) {
    diagnostics.sort_by(Diagnostic::compare_for_reporting);
}

/// Remove repeated diagnostics with identical report identity.
///
/// This deliberately preserves diagnostics that differ by source location,
/// phase, category, severity, or message. It does not attempt fuzzy matching
/// or suppress independent semantic causes.
///
/// Complexity is quadratic in the number of diagnostics. For very large
/// diagnostic sets, use an external ordered identity index in the collection
/// owner; this foundational record type imposes no retention ceiling.
pub fn deduplicate_diagnostics(diagnostics: &mut Vec<Diagnostic>) {
    let mut unique: Vec<Diagnostic> = Vec::with_capacity(diagnostics.len());

    for diagnostic in diagnostics.drain(..) {
        if !unique
            .iter()
            .any(|existing| existing.same_report_identity(&diagnostic))
        {
            unique.push(diagnostic);
        }
    }

    *diagnostics = unique;
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}[{}]: {}",
            self.severity,
            self.code,
            self.message
        )?;

        if let Some(help) = &self.help {
            write!(f, "\n  help: {help}")?;
        }

        for note in &self.notes {
            write!(f, "\n  note: {note}")?;
        }

        for label in &self.labels {
            write!(f, "\n  related: {}", label.message())?;
        }

        for fix in &self.fixes {
            write!(f, "\n  fix: {}", fix.title())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span(start: u64, end: u64) -> Span {
        use crate::frontend::ast::node::source::span::SourceOffset;
        use crate::frontend::ast::node::source::source_id::SourceId;

        Span::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(start),
            SourceOffset::from_raw(end),
        )
    }

    fn diagnostic(
        code: &str,
        severity: DiagnosticSeverity,
        location: Option<Span>,
    ) -> Diagnostic {
        Diagnostic::new(
            code,
            DiagnosticPhase::TypeAnalysis,
            DiagnosticCategory::Type,
            severity,
            "test diagnostic",
            location,
        )
    }

    #[test]
    fn code_is_machine_readable_and_not_message_derived() {
        let item = diagnostic(
            "semantic.type.mismatch",
            DiagnosticSeverity::Error,
            Some(span(2, 5)),
        );

        assert_eq!(item.code().as_str(), "semantic.type.mismatch");
        assert_eq!(item.message(), "test diagnostic");
        assert!(item.has_valid_code());
    }

    #[test]
    fn empty_code_is_reported_as_invalid_without_panicking() {
        let item = diagnostic("", DiagnosticSeverity::Error, None);

        assert!(!item.has_valid_code());
    }

    #[test]
    fn metadata_is_deterministic_and_typed() {
        let item = diagnostic(
            "semantic.resource.symbolic",
            DiagnosticSeverity::Information,
            None,
        )
        .with_metadata("required", DiagnosticValue::Unsigned(u128::MAX))
        .with_metadata("symbolic", true);

        assert_eq!(
            item.metadata_value("required"),
            Some(&DiagnosticValue::Unsigned(u128::MAX))
        );
        assert_eq!(
            item.metadata_value("symbolic"),
            Some(&DiagnosticValue::Boolean(true))
        );
    }

    #[test]
    fn sorting_is_repeatable() {
        let mut items = vec![
            diagnostic("z.code", DiagnosticSeverity::Warning, Some(span(8, 9))),
            diagnostic("b.code", DiagnosticSeverity::Error, Some(span(1, 2))),
            diagnostic("a.code", DiagnosticSeverity::Error, None),
        ];

        sort_diagnostics(&mut items);
        let first: Vec<String> = items
            .iter()
            .map(|item| item.code().as_str().to_owned())
            .collect();

        sort_diagnostics(&mut items);
        let second: Vec<String> = items
            .iter()
            .map(|item| item.code().as_str().to_owned())
            .collect();

        assert_eq!(first, second);
        assert_eq!(first[0], "a.code");
    }

    #[test]
    fn deduplication_keeps_distinct_source_locations() {
        let original = diagnostic(
            "semantic.name.unresolved",
            DiagnosticSeverity::Error,
            Some(span(1, 2)),
        );
        let duplicate = original.clone();
        let distinct = diagnostic(
            "semantic.name.unresolved",
            DiagnosticSeverity::Error,
            Some(span(4, 5)),
        );

        let mut items = vec![original, duplicate, distinct];
        deduplicate_diagnostics(&mut items);

        assert_eq!(items.len(), 2);
        assert_ne!(items[0].primary_span(), items[1].primary_span());
    }

    #[test]
    fn fix_its_are_structured_and_do_not_mutate_source() {
        let fix = FixIt::new(
            "Insert a missing delimiter",
            vec![FixItEdit::new(span(3, 3), ")")],
            true,
        );

        let item = diagnostic(
            "semantic.syntax.expected_delimiter",
            DiagnosticSeverity::Error,
            Some(span(3, 3)),
        )
        .with_fix(fix);

        assert_eq!(item.fixes().len(), 1);
        assert!(item.fixes()[0].is_safe());
        assert_eq!(item.fixes()[0].edits()[0].replacement(), ")");
    }

    #[test]
    fn recovery_is_not_inferred_from_severity() {
        let item = diagnostic(
            "semantic.type.mismatch",
            DiagnosticSeverity::Error,
            Some(span(1, 2)),
        )
        .with_recovery(RecoveryClassification::Recoverable);

        assert!(item.is_error());
        assert_eq!(item.recovery(), RecoveryClassification::Recoverable);
    }

    #[test]
    fn display_is_human_readable_but_code_remains_separate() {
        let item = diagnostic(
            "semantic.type.mismatch",
            DiagnosticSeverity::Error,
            Some(span(1, 2)),
        )
        .with_help("Check the expression's declared type.")
        .with_note("This is a source-level type error.");

        let rendered = item.to_string();

        assert!(rendered.contains("semantic.type.mismatch"));
        assert!(rendered.contains("help:"));
        assert!(rendered.contains("note:"));
        assert_eq!(item.code().as_str(), "semantic.type.mismatch");
    }
}
