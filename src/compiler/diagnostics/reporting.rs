
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler Diagnostic Reporting
//!
//! Path: `src/compiler/diagnostics/reporting.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Publish compiler diagnostic aggregates through a stable, structured,
//! machine-readable reporting interface.
//!
//! The reporting layer converts an existing `AggregatedDiagnostics` result
//! into a versioned JSON document.
//!
//! It preserves:
//!
//! - stable diagnostic codes;
//! - diagnostic severity;
//! - diagnostic category;
//! - emitting phase;
//! - primary source spans;
//! - secondary source labels;
//! - explanatory notes;
//! - help text;
//! - structured fix-it edits;
//! - recovery classification;
//! - machine-readable metadata;
//! - aggregation completeness;
//! - aggregation statistics.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - the public diagnostic-reporting API;
//! - the versioned JSON report envelope;
//! - report summary calculation;
//! - structured serialization;
//! - serialization error propagation.
//!
//! This module does not own:
//!
//! - diagnostic records;
//! - diagnostic identity;
//! - diagnostic ordering;
//! - diagnostic deduplication;
//! - diagnostic aggregation;
//! - diagnostic suppression;
//! - source maps;
//! - source-file storage;
//! - source-coordinate conversion;
//! - human-readable source rendering;
//! - compiler validity;
//! - compilation-stage execution.
//!
//! Canonical diagnostic record:
//!
//! `crate::frontend::semantic::diagnostics::Diagnostic`
//!
//! Canonical source span:
//!
//! `crate::frontend::ast::node::source::Span`
//!
//! Canonical aggregation result:
//!
//! `crate::compiler::diagnostics::aggregation::AggregatedDiagnostics`
//!
//! # Output contract
//!
//! The report is a JSON object with:
//!
//! - schema_version;
//! - status;
//! - summary;
//! - diagnostics.
//!
//! Source locations are serialized as source IDs and UTF-8 byte offsets.
//! They are not converted into line/column positions here. That conversion
//! belongs to the canonical source-map infrastructure and presentation layer.
//!
//! # Determinism
//!
//! - Diagnostic order is inherited from the canonical aggregate.
//! - Metadata is serialized from its ordered BTreeMap.
//! - Object fields are emitted in a stable, declared order.
//! - No timestamps, process IDs, addresses, random values, or worker IDs
//!   are introduced.
//! - No hash-map iteration is used.
//!
//! # Scalability
//!
//! No fixed diagnostic-count, source-size, hardware-count, or domain-count
//! ceiling is introduced.
//!
//! `write_json` serializes directly into a caller-provided writer instead of
//! first constructing a second complete JSON document in memory.
//!
//! `to_json_string` is provided for callers that explicitly want an owned
//! string and therefore accept its memory cost.
//!
//! Unlimited reporting does not promise infinite physical memory or output
//! capacity. Actual resource availability and caller-configured budgets
//! remain authoritative.
//!
//! # Error handling
//!
//! Serialization failures are returned to the caller.
//! No serialization failure is silently converted into an empty report.
//! Incomplete aggregation remains explicitly marked incomplete.
//!
//! # Security
//!
//! This module does not execute fix-its, open source files, interpret metadata
//! as instructions, or perform filesystem/network operations.
//!
//! # Rust and safety
//!
//! Rust 2021, Rust 1.97+.
//! Safe Rust only.
//! No global mutable state.

use core::fmt;
use serde::ser::{
    Serialize,
    SerializeMap,
    SerializeSeq,
    SerializeStruct,
    Serializer,
};

use crate::compiler::diagnostics::aggregation::{
    AggregatedDiagnostics,
    AggregationStatus,
    AggregationStatistics,
};

use crate::frontend::semantic::diagnostics::{
    Diagnostic,
    DiagnosticSeverity,
    DiagnosticValue,
    FixIt,
    FixItEdit,
};

use crate::frontend::ast::node::source::Span;

/// Stable identifier for the diagnostic-report JSON schema.
///
/// A schema change that is not backward-compatible must use a new version.
pub const DIAGNOSTIC_REPORT_SCHEMA: &str = "zamani.diagnostics.v1";

/// Status of a published diagnostic report.
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
}

impl From<AggregationStatus> for ReportStatus {
    fn from(status: AggregationStatus) -> Self {
        match status {
            AggregationStatus::Complete => Self::Complete,
            AggregationStatus::Incomplete => Self::Incomplete,
        }
    }
}

/// Summary information included in every report.
///
/// Counts use `u128` in the report model so that they are not tied to the
/// pointer width of the host on which the report is generated.
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
}

impl ReportSummary {
    /// Build a summary from an already finalized aggregate.
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
                    // The canonical severity enum is non-exhaustive.
                    // Unknown future variants are not misclassified.
                }
            }
        }

        summary
    }

    /// Number of diagnostic submissions.
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

    /// Number of duplicate submissions merged during aggregation.
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

/// A borrowed report over a finalized diagnostic aggregate.
///
/// The report does not clone the diagnostic collection. Its lifetime is
/// bounded by the aggregate supplied to `new`.
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

    /// Return the diagnostics in canonical reporting order.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        self.aggregate.diagnostics()
    }

    /// Return whether the aggregate is complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.aggregate.is_complete()
    }

    /// Serialize this report directly to a caller-provided writer.
    ///
    /// This is the preferred API for large reports because it avoids
    /// constructing an additional complete JSON string.
    ///
    /// The writer is owned by the caller. This method does not flush or close
    /// it; those lifecycle decisions belong to the caller.
    ///
    /// A serialization or writer failure is returned explicitly.
    pub fn write_json<W>(
        &self,
        writer: W,
    ) -> Result<(), ReportError>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, self)
            .map_err(ReportError::Serialization)
    }

    /// Serialize this report into an owned JSON string.
    ///
    /// This method allocates memory proportional to the resulting JSON
    /// document. Prefer `write_json` when reporting large diagnostic sets.
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

impl fmt::Display for ReportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
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
            11,
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

        state.serialize_field("has_errors", &self.has_errors())?;
        state.end()
    }
}

/// A source span serialized in canonical source coordinates.
///
/// These are source IDs and UTF-8 byte offsets. They are not line/column
/// coordinates and must not be interpreted as display positions.
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

/// Serialize one diagnostic span.
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

/// Borrowed view of the diagnostic sequence.
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
            sequence.serialize_element(&DiagnosticRecord(diagnostic))?;
        }

        sequence.end()
    }
}

/// Borrowed view of one canonical diagnostic.
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

        let mut state = serializer.serialize_struct(
            "Diagnostic",
            11,
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

/// Borrowed view of secondary source labels.
struct LabelsRef<'a>(
    &'a [crate::frontend::semantic::diagnostics::DiagnosticLabel],
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
        state.serialize_field("edits", &FixEditsRef(fix.edits()))?;

        state.end()
    }
}

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

                for (key, value) in values {
                    map.serialize_entry(key, value)?;
                }

                map.end()
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
        DiagnosticAggregator,
        AggregationPolicy,
    };

    use crate::frontend::semantic::diagnostics::{
        Diagnostic,
        DiagnosticCategory,
        DiagnosticPhase,
        DiagnosticSeverity,
        DiagnosticValue,
    };

    fn sample_diagnostic() -> Diagnostic {
        Diagnostic::new(
            "ZMN-TYPE-MISMATCH",
            DiagnosticPhase::TypeAnalysis,
            DiagnosticCategory::Type,
            DiagnosticSeverity::Error,
            "incompatible types",
            None,
        )
        .with_note("the expression has an incompatible type")
        .with_help("provide a compatible value")
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

        assert!(json.contains(DIAGNOSTIC_REPORT_SCHEMA));
    }

    #[test]
    fn report_contains_diagnostic_identity() {
        let aggregate = sample_report();
        let report = DiagnosticReport::new(&aggregate);

        let json = report
            .to_json_string()
            .expect("report should serialize");

        assert!(json.contains("ZMN-TYPE-MISMATCH"));
        assert!(json.contains("type_analysis"));
        assert!(json.contains("incompatible types"));
        assert!(json.contains("\"severity\":\"error\""));
    }

    #[test]
    fn report_contains_structured_metadata() {
        let aggregate = sample_report();
        let report = DiagnosticReport::new(&aggregate);

        let json = report
            .to_json_string()
            .expect("report should serialize");

        assert!(json.contains("\"expected\":\"Int\""));
        assert!(json.contains("\"actual\":\"Float\""));
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
        assert!(report.is_complete());
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

        assert_eq!(report.status(), ReportStatus::Incomplete);

        let json = report
            .to_json_string()
            .expect("incomplete report should still serialize");

        assert!(json.contains("\"status\":\"incomplete\""));
        assert!(json.contains("\"dropped\":1"));
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
    fn metadata_objects_are_serialized_deterministically() {
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
}
