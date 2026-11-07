
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Compiler Diagnostic Reporting
//!
//! Path: `src/compiler/diagnostics/reporting.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Publish finalized compiler diagnostic aggregates through a stable,
//! deterministic, machine-readable JSON reporting interface.
//!
//! This module provides:
//!
//! - a versioned JSON report envelope;
//! - complete aggregation status;
//! - diagnostic summary statistics;
//! - canonical diagnostic serialization;
//! - canonical source-span serialization;
//! - structured labels, notes, help and fix-it serialization;
//! - deterministic structured metadata serialization;
//! - direct writer-based serialization;
//! - owned-string serialization for callers that request it.
//!
//! # Canonical authorities
//!
//! Diagnostic records:
//! `crate::frontend::semantic::diagnostics`
//!
//! Source coordinates:
//! `crate::frontend::ast::node::source::span`
//!
//! Aggregation:
//! `crate::compiler::diagnostics::aggregation`
//!
//! Human-readable presentation:
//! `crate::compiler::diagnostics::rendering`
//!
//! Normative specifications:
//! `grammar/spec/diagnostics.md`
//! `grammar/spec/source-spans.md`
//!
//! # Ownership
//!
//! This module owns the report envelope, summary calculation,
//! serialization and reporting errors.
//!
//! It does not own diagnostic construction, identity, ordering,
//! deduplication, suppression, source storage, source-map lookup,
//! source-coordinate conversion, human-readable rendering,
//! semantic validity, or compilation-stage execution.
//!
//! # Source-coordinate contract
//!
//! Serialized spans contain:
//!
//! - source_id;
//! - start_byte;
//! - end_byte.
//!
//! Offsets are zero-based UTF-8 byte offsets.
//! Spans are half-open: [start_byte, end_byte).
//!
//! They are not line/column positions.
//! Consumers must resolve source IDs against the matching source-map
//! snapshot before displaying or editing source.
//!
//! # Determinism
//!
//! - Diagnostic ordering is inherited from AggregatedDiagnostics.
//! - Diagnostic labels, notes and fixes retain producer order.
//! - Metadata objects use the canonical ordered map.
//! - JSON object fields have an explicitly declared order.
//! - No timestamps, process IDs, random values, addresses or worker IDs
//!   are introduced.
//!
//! # Scalability
//!
//! No arbitrary diagnostic-count, source-size, hardware-count,
//! computational-domain or target-count ceiling is introduced.
//!
//! `write_json` and `write_json_to` serialize through a caller-provided
//! writer and do not construct a second complete report string.
//!
//! `to_json_string` intentionally allocates an owned output string.
//!
//! Actual output capacity and memory availability remain physical and
//! operational constraints. This module does not promise literal infinite
//! memory or infinite output capacity.
//!
//! # Failure contract
//!
//! A serialization failure is returned to the caller.
//! A writer may have received a partial JSON document when an error occurs.
//! Such output must not be treated as a complete report.
//!
//! An incomplete aggregate remains explicitly marked incomplete.
//! Reporting never upgrades an incomplete aggregate to complete.
//!
//! # Security
//!
//! This module serializes data only.
//! It does not execute fix-its, open source files, interpret metadata
//! as instructions, or perform filesystem, network or process operations.

use core::fmt;
use serde::ser::{
    Serialize,
    SerializeMap,
    SerializeSeq,
    SerializeStruct,
    Serializer,
};
use std::io::Write;

use crate::compiler::diagnostics::aggregation::{
    AggregatedDiagnostics,
    AggregationStatus,
};

use crate::frontend::semantic::diagnostics::{
    Diagnostic,
    DiagnosticSeverity,
    DiagnosticValue,
    FixIt,
    FixItEdit,
};

use crate::frontend::ast::node::source::span::Span;

/// Stable identifier for the diagnostic-report JSON schema.
///
/// Backward-compatible additions may be made to this schema.
/// An incompatible change requires a new schema identifier.
pub const DIAGNOSTIC_REPORT_SCHEMA: &str = "zamani.diagnostics.v1";

/// Completion status of a published diagnostic report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReportStatus {
    /// Aggregation retained or merged all submitted diagnostics.
    Complete,

    /// Aggregation dropped diagnostics or encountered an infrastructure
    /// failure.
    Incomplete,
}

impl ReportStatus {
    /// Return the stable machine-readable status name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Incomplete => "incomplete",
        }
    }

    /// Whether the report represents complete aggregation.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

impl From<AggregationStatus> for ReportStatus {
    fn from(status: AggregationStatus) -> Self {
        match status {
            AggregationStatus::Complete => Self::Complete,
            AggregationStatus::Incomplete => Self::Incomplete,
        }
    }
}

/// Summary information included in every diagnostic report.
///
/// Counts use u128 for the report representation so that the report model
/// is not tied to the pointer width of the host.
///
/// `unclassified` counts diagnostic severities introduced by a future
/// non-exhaustive DiagnosticSeverity implementation that this version
/// does not recognize.
///
/// It prevents future severity variants from silently disappearing from
/// the summary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportSummary {
    submitted: u64,
    retained: u128,
    dropped: u64,
    duplicates: u64,

    hints: u128,
    notes: u128,
    information: u128,
    warnings: u128,
    errors: u128,
    fatal: u128,
    unclassified: u128,
}

impl ReportSummary {
    /// Build a summary from a finalized diagnostic aggregate.
    ///
    /// This method does not mutate the aggregate or alter diagnostic order.
    #[must_use]
    pub fn from_aggregate(
        aggregate: &AggregatedDiagnostics,
    ) -> Self {
        let statistics = aggregate.statistics();

        let mut summary = Self {
            submitted: statistics.submitted(),
            retained: aggregate.diagnostics().len() as u128,
            dropped: statistics.dropped(),
            duplicates: statistics.duplicates(),

            hints: 0,
            notes: 0,
            information: 0,
            warnings: 0,
            errors: 0,
            fatal: 0,
            unclassified: 0,
        };

        for diagnostic in aggregate.diagnostics() {
            match diagnostic.severity() {
                DiagnosticSeverity::Hint => {
                    summary.hints += 1;
                }
                DiagnosticSeverity::Note => {
                    summary.notes += 1;
                }
                DiagnosticSeverity::Information => {
                    summary.information += 1;
                }
                DiagnosticSeverity::Warning => {
                    summary.warnings += 1;
                }
                DiagnosticSeverity::Error => {
                    summary.errors += 1;
                }
                DiagnosticSeverity::Fatal => {
                    summary.fatal += 1;
                }
                _ => {
                    // DiagnosticSeverity is non-exhaustive.
                    // Future variants must remain visible in the report
                    // summary without being incorrectly classified.
                    summary.unclassified += 1;
                }
            }
        }

        summary
    }

    /// Number of diagnostic submissions accounted for by aggregation.
    #[must_use]
    pub const fn submitted(&self) -> u64 {
        self.submitted
    }

    /// Number of diagnostics retained in the aggregate.
    #[must_use]
    pub const fn retained(&self) -> u128 {
        self.retained
    }

    /// Number of diagnostics dropped during aggregation.
    #[must_use]
    pub const fn dropped(&self) -> u64 {
        self.dropped
    }

    /// Number of duplicate diagnostic submissions merged.
    #[must_use]
    pub const fn duplicates(&self) -> u64 {
        self.duplicates
    }

    /// Number of retained hint diagnostics.
    #[must_use]
    pub const fn hints(&self) -> u128 {
        self.hints
    }

    /// Number of retained note diagnostics.
    #[must_use]
    pub const fn notes(&self) -> u128 {
        self.notes
    }

    /// Number of retained informational diagnostics.
    #[must_use]
    pub const fn information(&self) -> u128 {
        self.information
    }

    /// Number of retained warning diagnostics.
    #[must_use]
    pub const fn warnings(&self) -> u128 {
        self.warnings
    }

    /// Number of retained error diagnostics.
    #[must_use]
    pub const fn errors(&self) -> u128 {
        self.errors
    }

    /// Number of retained fatal diagnostics.
    #[must_use]
    pub const fn fatal(&self) -> u128 {
        self.fatal
    }

    /// Number of diagnostics with an unclassified future severity.
    #[must_use]
    pub const fn unclassified(&self) -> u128 {
        self.unclassified
    }

    /// Whether a retained diagnostic has error or fatal severity.
    #[must_use]
    pub const fn has_errors(&self) -> bool {
        self.errors > 0 || self.fatal > 0
    }

    /// Whether a retained diagnostic has fatal severity.
    #[must_use]
    pub const fn has_fatal_errors(&self) -> bool {
        self.fatal > 0
    }
}

/// Borrowed view of a finalized diagnostic aggregate.
///
/// No diagnostic collection is cloned.
/// The report cannot outlive the aggregate from which it was created.
#[derive(Debug, Clone, Copy)]
pub struct DiagnosticReport<'a> {
    aggregate: &'a AggregatedDiagnostics,
}

impl<'a> DiagnosticReport<'a> {
    /// Create a report view over a finalized aggregate.
    #[must_use]
    pub const fn new(
        aggregate: &'a AggregatedDiagnostics,
    ) -> Self {
        Self { aggregate }
    }

    /// Return the underlying finalized aggregate.
    #[must_use]
    pub const fn aggregate(&self) -> &'a AggregatedDiagnostics {
        self.aggregate
    }

    /// Return the report completion status.
    #[must_use]
    pub fn status(&self) -> ReportStatus {
        self.aggregate.status().into()
    }

    /// Calculate the report summary.
    #[must_use]
    pub fn summary(&self) -> ReportSummary {
        ReportSummary::from_aggregate(self.aggregate)
    }

    /// Return diagnostics in canonical reporting order.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        self.aggregate.diagnostics()
    }

    /// Whether aggregation completed without dropping diagnostics.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.aggregate.is_complete()
    }

    /// Whether the aggregate contains error or fatal diagnostics.
    ///
    /// This is diagnostic information, not a substitute for the compiler
    /// pipeline's authoritative compilation outcome.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.aggregate.has_errors()
    }

    /// Whether the aggregate contains fatal diagnostics.
    #[must_use]
    pub fn has_fatal_errors(&self) -> bool {
        self.aggregate.has_fatal_errors()
    }

    /// Serialize directly to a caller-provided writer.
    ///
    /// This is the preferred API for large reports.
    ///
    /// The writer is not flushed or closed.
    ///
    /// If serialization fails, the writer may contain a partial JSON
    /// document. The caller must discard or otherwise mark that output
    /// as incomplete.
    pub fn write_json<W>(
        &self,
        writer: W,
    ) -> Result<(), ReportError>
    where
        W: Write,
    {
        serde_json::to_writer(writer, self)
            .map_err(ReportError::Serialization)
    }

    /// Serialize directly to a borrowed writer.
    ///
    /// This variant makes it explicit that the caller retains ownership
    /// of the writer after the operation.
    ///
    /// No complete intermediate JSON string is constructed.
    pub fn write_json_to<W>(
        &self,
        writer: &mut W,
    ) -> Result<(), ReportError>
    where
        W: Write + ?Sized,
    {
        serde_json::to_writer(writer, self)
            .map_err(ReportError::Serialization)
    }

    /// Serialize into an owned JSON string.
    ///
    /// This allocates memory proportional to the resulting document.
    /// Prefer `write_json` or `write_json_to` for large reports.
    pub fn to_json_string(&self) -> Result<String, ReportError> {
        serde_json::to_string(self)
            .map_err(ReportError::Serialization)
    }
}

/// Errors returned by diagnostic reporting.
#[derive(Debug)]
#[non_exhaustive]
pub enum ReportError {
    /// JSON serialization or the underlying output writer failed.
    Serialization(serde_json::Error),
}

impl ReportError {
    /// Return the underlying JSON serialization error.
    #[must_use]
    pub const fn serialization_error(
        &self,
    ) -> &serde_json::Error {
        match self {
            Self::Serialization(error) => error,
        }
    }

    /// Whether the failure originated from the output writer.
    #[must_use]
    pub fn is_writer_error(&self) -> bool {
        match self {
            Self::Serialization(error) => error.is_io(),
        }
    }
}

impl fmt::Display for ReportError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Serialization(error) => {
                write!(
                    formatter,
                    "diagnostic report serialization failed: {error}"
                )
            }
        }
    }
}

impl std::error::Error for ReportError {
    fn source(
        &self,
    ) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serialization(error) => Some(error),
        }
    }
}

// -----------------------------------------------------------------------------
// Stable report serialization
// -----------------------------------------------------------------------------

impl Serialize for DiagnosticReport<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let summary = self.summary();

        let mut state = serializer.serialize_struct(
            "DiagnosticReport",
            4,
        )?;

        state.serialize_field(
            "schema_version",
            DIAGNOSTIC_REPORT_SCHEMA,
        )?;

        state.serialize_field(
            "status",
            self.status().as_str(),
        )?;

        state.serialize_field(
            "summary",
            &summary,
        )?;

        state.serialize_field(
            "diagnostics",
            &DiagnosticsRef(self.diagnostics()),
        )?;

        state.end()
    }
}

impl Serialize for ReportSummary {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct(
            "DiagnosticReportSummary",
            13,
        )?;

        state.serialize_field("submitted", &self.submitted)?;
        state.serialize_field("retained", &self.retained)?;
        state.serialize_field("dropped", &self.dropped)?;
        state.serialize_field("duplicates", &self.duplicates)?;

        state.serialize_field("hints", &self.hints)?;
        state.serialize_field("notes", &self.notes)?;
        state.serialize_field("information", &self.information)?;
        state.serialize_field("warnings", &self.warnings)?;
        state.serialize_field("errors", &self.errors)?;
        state.serialize_field("fatal", &self.fatal)?;
        state.serialize_field("unclassified", &self.unclassified)?;

        state.serialize_field("has_errors", &self.has_errors())?;
        state.serialize_field(
            "has_fatal_errors",
            &self.has_fatal_errors(),
        )?;

        state.end()
    }
}

/// Canonical source span serialized as source identity and UTF-8 byte offsets.
///
/// These are not line/column coordinates.
/// Consumers must resolve the source identity against the source-map
/// snapshot associated with the compilation.
#[derive(Debug, Clone, Copy, serde::Serialize)]
struct SpanRecord {
    source_id: u64,
    start_byte: u64,
    end_byte: u64,
}

impl From<Span> for SpanRecord {
    fn from(span: Span) -> Self {
        Self {
            source_id: span.source().as_raw(),
            start_byte: span.start_raw(),
            end_byte: span.end_raw(),
        }
    }
}

/// Serialize an optional canonical source span.
#[derive(Debug, Clone, Copy)]
struct OptionalSpan(Option<Span>);

impl Serialize for OptionalSpan {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            Some(span) => SpanRecord::from(span).serialize(serializer),
            None => serializer.serialize_none(),
        }
    }
}

/// Borrowed view of the canonical diagnostic sequence.
struct DiagnosticsRef<'a>(&'a [Diagnostic]);

impl Serialize for DiagnosticsRef<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(
            Some(self.0.len()),
        )?;

        for diagnostic in self.0 {
            sequence.serialize_element(
                &DiagnosticRecord(diagnostic),
            )?;
        }

        sequence.end()
    }
}

/// Borrowed view of one canonical frontend diagnostic.
struct DiagnosticRecord<'a>(&'a Diagnostic);

impl Serialize for DiagnosticRecord<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let diagnostic = self.0;

        // Twelve fields are serialized below.
        let mut state = serializer.serialize_struct(
            "Diagnostic",
            12,
        )?;

        state.serialize_field(
            "code",
            diagnostic.code().as_str(),
        )?;

        state.serialize_field(
            "severity",
            diagnostic.severity().as_str(),
        )?;

        state.serialize_field(
            "category",
            diagnostic.category().as_str(),
        )?;

        state.serialize_field(
            "phase",
            diagnostic.phase().as_str(),
        )?;

        state.serialize_field(
            "message",
            diagnostic.message(),
        )?;

        state.serialize_field(
            "primary_span",
            &OptionalSpan(diagnostic.primary_span()),
        )?;

        state.serialize_field(
            "labels",
            &LabelsRef(diagnostic.labels()),
        )?;

        state.serialize_field(
            "notes",
            diagnostic.notes(),
        )?;

        state.serialize_field(
            "help",
            &diagnostic.help(),
        )?;

        state.serialize_field(
            "fixes",
            &FixesRef(diagnostic.fixes()),
        )?;

        state.serialize_field(
            "recovery",
            diagnostic.recovery().as_str(),
        )?;

        state.serialize_field(
            "metadata",
            diagnostic.metadata(),
        )?;

        state.end()
    }
}

/// Borrowed view of secondary diagnostic labels.
struct LabelsRef<'a>(
    &'a [
        crate::frontend::semantic::diagnostics::diagnostic::DiagnosticLabel
    ],
);

impl Serialize for LabelsRef<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(
            Some(self.0.len()),
        )?;

        for label in self.0 {
            sequence.serialize_element(&LabelRecord {
                span: SpanRecord::from(label.span()),
                message: label.message(),
            })?;
        }

        sequence.end()
    }
}

#[derive(serde::Serialize)]
struct LabelRecord<'a> {
    span: SpanRecord,
    message: &'a str,
}

/// Borrowed view of structured fix-it records.
struct FixesRef<'a>(&'a [FixIt]);

impl Serialize for FixesRef<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(
            Some(self.0.len()),
        )?;

        for fix in self.0 {
            sequence.serialize_element(&FixRecord(fix))?;
        }

        sequence.end()
    }
}

/// Borrowed view of one fix-it.
struct FixRecord<'a>(&'a FixIt);

impl Serialize for FixRecord<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let fix = self.0;

        let mut state = serializer.serialize_struct(
            "FixIt",
            3,
        )?;

        state.serialize_field("title", fix.title())?;
        state.serialize_field("is_safe", &fix.is_safe())?;
        state.serialize_field(
            "edits",
            &FixEditsRef(fix.edits()),
        )?;

        state.end()
    }
}

/// Borrowed view of fix-it edits.
struct FixEditsRef<'a>(&'a [FixItEdit]);

impl Serialize for FixEditsRef<'_> {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(
            Some(self.0.len()),
        )?;

        for edit in self.0 {
            sequence.serialize_element(&EditRecord {
                span: SpanRecord::from(edit.span()),
                replacement: edit.replacement(),
            })?;
        }

        sequence.end()
    }
}

#[derive(serde::Serialize)]
struct EditRecord<'a> {
    span: SpanRecord,
    replacement: &'a str,
}

// -----------------------------------------------------------------------------
// Canonical structured metadata serialization
// -----------------------------------------------------------------------------

impl Serialize for DiagnosticValue {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Null => serializer.serialize_none(),

            Self::Boolean(value) => {
                serializer.serialize_bool(*value)
            }

            Self::Integer(value) => {
                serializer.serialize_i128(*value)
            }

            Self::Unsigned(value) => {
                serializer.serialize_u128(*value)
            }

            Self::Text(value) => {
                serializer.serialize_str(value)
            }

            Self::Sequence(values) => {
                let mut sequence = serializer.serialize_seq(
                    Some(values.len()),
                )?;

                for value in values {
                    sequence.serialize_element(value)?;
                }

                sequence.end()
            }

            Self::Object(values) => {
                let mut map = serializer.serialize_map(
                    Some(values.len()),
                )?;

                // DiagnosticValue::Object uses BTreeMap, so this iteration
                // has deterministic key order.
                for (key, value) in values {
                    map.serialize_entry(key, value)?;
                }

                map.end()
            }

            _ => {
                // DiagnosticValue is non-exhaustive. A future variant
                // must not be silently serialized as an unrelated value.
                Err(serde::ser::Error::custom(
                    "unsupported diagnostic metadata value variant",
                ))
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::diagnostics::aggregation::{
        AggregationPolicy,
        DiagnosticAggregator,
    };

    use crate::frontend::semantic::diagnostics::{
        Diagnostic,
        DiagnosticCategory,
        DiagnosticPhase,
        DiagnosticSeverity,
        DiagnosticValue,
    };

    use std::io;

    fn sample_diagnostic() -> Diagnostic {
        Diagnostic::new(
            "ZMN-TYPE-MISMATCH",
            DiagnosticPhase::TypeAnalysis,
            DiagnosticCategory::Type,
            DiagnosticSeverity::Error,
            "incompatible types",
            None,
        )
        .with_note(
            "the expression has an incompatible type",
        )
        .with_help(
            "provide a compatible value",
        )
        .with_metadata(
            "expected",
            DiagnosticValue::Text("Int".to_owned()),
        )
        .with_metadata(
            "actual",
            DiagnosticValue::Text("Float".to_owned()),
        )
    }

    fn sample_report() -> AggregatedDiagnostics {
        let mut aggregator = DiagnosticAggregator::new(
            AggregationPolicy::unlimited(),
        );

        aggregator
            .push(sample_diagnostic())
            .expect("diagnostic insertion should succeed");

        aggregator.finish()
    }

    #[test]
    fn report_contains_schema_version() {
        let aggregate = sample_report();
        let report = DiagnosticReport::new(&aggregate);

        let json = report
            .to_json_string()
            .expect("report should serialize");

        let parsed: serde_json::Value =
            serde_json::from_str(&json)
                .expect("report should be valid JSON");

        assert_eq!(
            parsed["schema_version"],
            DIAGNOSTIC_REPORT_SCHEMA,
        );
    }

    #[test]
    fn report_contains_diagnostic_identity() {
        let aggregate = sample_report();
        let report = DiagnosticReport::new(&aggregate);

        let json = report
            .to_json_string()
            .expect("report should serialize");

        let parsed: serde_json::Value =
            serde_json::from_str(&json)
                .expect("report should be valid JSON");

        let diagnostic = &parsed["diagnostics"][0];

        assert_eq!(
            diagnostic["code"],
            "ZMN-TYPE-MISMATCH",
        );

        assert_eq!(
            diagnostic["phase"],
            "type_analysis",
        );

        assert_eq!(
            diagnostic["severity"],
            "error",
        );

        assert_eq!(
            diagnostic["message"],
            "incompatible types",
        );
    }

    #[test]
    fn report_contains_structured_metadata() {
        let aggregate = sample_report();
        let report = DiagnosticReport::new(&aggregate);

        let json = report
            .to_json_string()
            .expect("report should serialize");

        let parsed: serde_json::Value =
            serde_json::from_str(&json)
                .expect("report should be valid JSON");

        let metadata = &parsed["diagnostics"][0]["metadata"];

        assert_eq!(metadata["expected"], "Int");
        assert_eq!(metadata["actual"], "Float");
    }

    #[test]
    fn report_contains_aggregation_summary() {
        let aggregate = sample_report();
        let report = DiagnosticReport::new(&aggregate);

        let summary = report.summary();

        assert_eq!(summary.submitted(), 1);
        assert_eq!(summary.retained(), 1);
        assert_eq!(summary.errors(), 1);

        assert!(summary.has_errors());
        assert!(!summary.has_fatal_errors());
        assert!(report.is_complete());
        assert!(report.has_errors());
    }

    #[test]
    fn report_contains_fatal_summary_field() {
        let aggregate = sample_report();
        let report = DiagnosticReport::new(&aggregate);

        let json = report
            .to_json_string()
            .expect("report should serialize");

        let parsed: serde_json::Value =
            serde_json::from_str(&json)
                .expect("report should be valid JSON");

        let summary = &parsed["summary"];

        assert!(summary.get("has_fatal_errors").is_some());
        assert!(summary.get("unclassified").is_some());
    }

    #[test]
    fn report_preserves_incomplete_aggregation() {
        let policy = AggregationPolicy::with_limit(
            core::num::NonZeroUsize::new(1)
                .expect("one is non-zero"),
        );

        let mut aggregator = DiagnosticAggregator::new(policy);

        aggregator
            .push(sample_diagnostic())
            .expect("first diagnostic should be retained");

        aggregator
            .push(Diagnostic::new(
                "ZMN-SECOND",
                DiagnosticPhase::TypeAnalysis,
                DiagnosticCategory::Type,
                DiagnosticSeverity::Warning,
                "second diagnostic",
                None,
            ))
            .expect("budget drop should be accounted for");

        let aggregate = aggregator.finish();
        let report = DiagnosticReport::new(&aggregate);

        assert_eq!(
            report.status(),
            ReportStatus::Incomplete,
        );

        let json = report
            .to_json_string()
            .expect("incomplete report should still serialize");

        let parsed: serde_json::Value =
            serde_json::from_str(&json)
                .expect("report should be valid JSON");

        assert_eq!(
            parsed["status"],
            "incomplete",
        );

        assert_eq!(
            parsed["summary"]["dropped"],
            1,
        );
    }

    #[test]
    fn writer_api_emits_valid_json() {
        let aggregate = sample_report();
        let report = DiagnosticReport::new(&aggregate);

        let mut output = Vec::new();

        report
            .write_json(&mut output)
            .expect("writer serialization should succeed");

        let parsed: serde_json::Value =
            serde_json::from_slice(&output)
                .expect("output should be valid JSON");

        assert_eq!(
            parsed["schema_version"],
            DIAGNOSTIC_REPORT_SCHEMA,
        );
    }

    #[test]
    fn borrowed_writer_api_preserves_writer_ownership() {
        let aggregate = sample_report();
        let report = DiagnosticReport::new(&aggregate);

        let mut output = Vec::new();

        report
            .write_json_to(&mut output)
            .expect("borrowed writer serialization should succeed");

        assert!(!output.is_empty());

        let parsed: serde_json::Value =
            serde_json::from_slice(&output)
                .expect("output should be valid JSON");

        assert_eq!(
            parsed["status"],
            "complete",
        );
    }

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(
            &mut self,
            _buffer: &[u8],
        ) -> io::Result<usize> {
            Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "simulated writer failure",
            ))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn writer_failure_is_returned() {
        let aggregate = sample_report();
        let report = DiagnosticReport::new(&aggregate);

        let error = report
            .write_json(FailingWriter)
            .expect_err("writer failure must be returned");

        assert!(error.is_writer_error());
    }

    #[test]
    fn repeated_serialization_is_deterministic() {
        let aggregate = sample_report();
        let report = DiagnosticReport::new(&aggregate);

        let first = report
            .to_json_string()
            .expect("first serialization");

        let second = report
            .to_json_string()
            .expect("second serialization");

        assert_eq!(first, second);
    }

    #[test]
    fn metadata_objects_are_serialized_in_key_order() {
        let aggregate = sample_report();
        let report = DiagnosticReport::new(&aggregate);

        let json = report
            .to_json_string()
            .expect("report should serialize");

        let expected_position = json
            .find("\"actual\"")
            .expect("actual key should exist");

        let later_position = json
            .find("\"expected\"")
            .expect("expected key should exist");

        assert!(expected_position < later_position);
    }
}
