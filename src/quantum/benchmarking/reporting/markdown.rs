//! Zamani Quantum Benchmarking — Markdown Reporting
//!
//! Production-grade Markdown rendering for the canonical
//! `BenchmarkSummary` reporting model.
//!
//! # Architectural role
//!
//! This module is a pure presentation layer.
//!
//! It consumes:
//!
//! ```text
//! quantum::benchmarking::reporting::summary::BenchmarkSummary
//!                                  │
//!                                  ▼
//!                    reporting::markdown
//!                                  │
//!                                  ▼
//!                         Markdown document
//! ```
//!
//! It deliberately does NOT:
//!
//! - execute quantum circuits;
//! - generate circuits;
//! - select a backend;
//! - access hardware;
//! - perform statistical analysis;
//! - calculate metrics;
//! - modify benchmark results;
//! - deserialize untrusted JSON;
//! - recompute benchmark success;
//! - perform protocol-specific interpretation;
//! - access global state;
//! - read environment variables;
//! - access the filesystem;
//! - print to stdout/stderr;
//! - generate timestamps;
//! - generate random identifiers.
//!
//! # Stable integration boundary
//!
//! `BenchmarkSummary` is the canonical input to this module.
//!
//! This is intentional. The summary layer already normalizes:
//!
//! - benchmark identity;
//! - scientific status;
//! - metrics;
//! - confidence intervals;
//! - findings;
//! - environment information;
//! - recommendations;
//! - bounded reporting data.
//!
//! Markdown therefore does not need to know how those values were produced.
//!
//! This prevents the dependency direction from becoming:
//!
//! ```text
//! Markdown -> protocol -> execution -> hardware
//! ```
//!
//! and instead keeps it:
//!
//! ```text
//! protocol / execution / analysis
//!             │
//!             ▼
//!     BenchmarkResult
//!             │
//!             ▼
//!     BenchmarkSummary
//!             │
//!             ▼
//!     Markdown renderer
//! ```
//!
//! # Production requirements
//!
//! This module provides:
//!
//! - deterministic output;
//! - bounded output size;
//! - bounded text rendering;
//! - Markdown table escaping;
//! - Markdown heading escaping;
//! - safe handling of control characters;
//! - finite floating-point validation;
//! - configurable decimal precision;
//! - configurable sections;
//! - stable section ordering;
//! - stable metric ordering;
//! - stable finding ordering;
//! - explicit scientific status;
//! - explicit success state;
//! - explicit confidence intervals;
//! - explicit uncertainty;
//! - explicit sample/shot/circuit counts;
//! - execution-environment presentation;
//! - recommendations;
//! - warnings and errors;
//! - schema metadata;
//! - optional machine-readable identifiers;
//! - deterministic regression-test output;
//! - no ANSI terminal escape sequences;
//! - no HTML injection;
//! - no unsafe code;
//! - Rust 1.97/1.97.1 compatibility.
//!
//! # Scientific semantics
//!
//! This renderer never changes the meaning of a metric.
//!
//! In particular:
//!
//! - it does not convert fractions into percentages unless explicitly
//!   configured to do so;
//! - it does not infer units;
//! - it does not recalculate confidence intervals;
//! - it does not change pass/fail semantics;
//! - it does not hide failed benchmarks;
//! - it does not hide warnings;
//! - it does not silently discard errors;
//! - it does not manufacture missing values;
//! - it does not sort metrics by "better" or "worse";
//! - it does not round values before scientific validation.
//!
//! # Security
//!
//! Benchmark names, metric descriptions, backend identifiers and findings can
//! originate from external systems. Markdown is therefore treated as an
//! untrusted presentation target.
//!
//! The renderer escapes:
//!
//! - `|` in tables;
//! - backticks;
//! - angle brackets;
//! - Markdown heading prefixes where appropriate;
//! - control characters;
//! - newlines in table cells.
//!
//! The renderer does not emit raw HTML.
//!
//! # Resource safety
//!
//! All rendering is bounded by `max_output_bytes`.
//!
//! Text values are bounded before being inserted into the output.
//!
//! A malformed or unexpectedly large benchmark summary therefore cannot force
//! an unbounded report allocation.
//!
//! # Integration contract
//!
//! `report.rs` should use:
//!
//! ```text
//! MarkdownReport::render(summary)
//! ```
//!
//! or:
//!
//! ```text
//! MarkdownReport::new(options).render(summary)
//! ```
//!
//! `report.rs` must not duplicate Markdown formatting logic.
//!
//! `json.rs` and `csv.rs` should consume the same `BenchmarkSummary` but remain
//! independent serializers.
//!
//! `table.rs` remains responsible for reusable tabular rendering primitives.
//! This file intentionally owns the complete document structure because a
//! Markdown benchmark report contains sections that are not tables.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust only.
//!
//! No nightly features.
//! No unsafe code.
//!
//! ---------------------------------------------------------------------------

#![deny(unsafe_code)]

use std::fmt::Write as FmtWrite;

use super::summary::{
    BenchmarkSummary,
    SummaryEnvironment,
    SummaryFinding,
    SummaryMetric,
    SummarySeverity,
    SummaryStatus,
};

// =============================================================================
// Public constants
// =============================================================================

/// Stable Markdown report schema identifier.
pub const MARKDOWN_REPORT_SCHEMA_ID: &str =
    "zamani.quantum.benchmark.markdown";

/// Stable Markdown report schema version.
pub const MARKDOWN_REPORT_SCHEMA_VERSION: u16 = 1;

/// Default maximum generated Markdown document size.
pub const DEFAULT_MAX_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

/// Default maximum number of decimal places displayed for floating-point
/// values.
pub const DEFAULT_DECIMAL_PLACES: usize = 6;

/// Maximum number of decimal places accepted by the renderer.
///
/// This prevents pathological formatting requests from creating extremely
/// large strings.
pub const MAX_DECIMAL_PLACES: usize = 18;

/// Default maximum bytes retained from a human-readable text field.
pub const DEFAULT_MAX_TEXT_BYTES: usize = 16 * 1024;

/// Maximum bytes accepted for a single rendered text field.
pub const MAX_TEXT_BYTES: usize = 1 * 1024 * 1024;

/// Default document title.
pub const DEFAULT_DOCUMENT_TITLE: &str =
    "Zamani Quantum Benchmark Report";

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing or rendering a Markdown report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkdownReportError {
    /// The requested decimal precision is too large.
    InvalidDecimalPlaces {
        /// Requested precision.
        value: usize,
    },

    /// The output limit is zero.
    InvalidOutputLimit,

    /// The text limit is zero.
    InvalidTextLimit,

    /// The benchmark identifier is empty.
    EmptyBenchmarkIdentifier,

    /// A metric contains a non-finite value.
    NonFiniteMetricValue {
        /// Metric identifier.
        metric_id: String,
    },

    /// A metric uncertainty is negative or non-finite.
    InvalidMetricUncertainty {
        /// Metric identifier.
        metric_id: String,
    },

    /// A confidence level is invalid.
    InvalidConfidenceLevel {
        /// Metric identifier.
        metric_id: String,
    },

    /// A confidence interval is malformed.
    InvalidConfidenceInterval {
        /// Metric identifier.
        metric_id: String,
    },

    /// The supplied summary exceeds the renderer's configured bounds.
    SummaryTooLarge {
        /// Human-readable field name.
        field: &'static str,
    },

    /// The renderer could not fit the required document within the configured
    /// output limit.
    OutputLimitExceeded {
        /// Configured maximum.
        max_bytes: usize,
    },
}

impl std::fmt::Display for MarkdownReportError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::InvalidDecimalPlaces { value } => write!(
                formatter,
                "Markdown decimal places {} exceed the maximum of {}",
                value,
                MAX_DECIMAL_PLACES
            ),

            Self::InvalidOutputLimit => {
                write!(formatter, "Markdown output limit must be greater than zero")
            }

            Self::InvalidTextLimit => {
                write!(formatter, "Markdown text limit must be greater than zero")
            }

            Self::EmptyBenchmarkIdentifier => {
                write!(formatter, "benchmark identifier must not be empty")
            }

            Self::NonFiniteMetricValue { metric_id } => write!(
                formatter,
                "metric '{}' contains a non-finite value",
                metric_id
            ),

            Self::InvalidMetricUncertainty { metric_id } => write!(
                formatter,
                "metric '{}' contains an invalid uncertainty",
                metric_id
            ),

            Self::InvalidConfidenceLevel { metric_id } => write!(
                formatter,
                "metric '{}' contains an invalid confidence level",
                metric_id
            ),

            Self::InvalidConfidenceInterval { metric_id } => write!(
                formatter,
                "metric '{}' contains an invalid confidence interval",
                metric_id
            ),

            Self::SummaryTooLarge { field } => write!(
                formatter,
                "benchmark summary field '{}' exceeds the Markdown renderer limits",
                field
            ),

            Self::OutputLimitExceeded { max_bytes } => write!(
                formatter,
                "Markdown report exceeds the configured {} byte output limit",
                max_bytes
            ),
        }
    }
}

impl std::error::Error for MarkdownReportError {}

/// Result type used by the Markdown renderer.
pub type MarkdownResult<T> = Result<T, MarkdownReportError>;

// =============================================================================
// Configuration
// =============================================================================

/// Controls which sections are emitted into a Markdown report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkdownSections {
    /// Include report metadata.
    pub metadata: bool,

    /// Include benchmark identity/status.
    pub overview: bool,

    /// Include metrics.
    pub metrics: bool,

    /// Include confidence information.
    pub confidence: bool,

    /// Include execution environment.
    pub environment: bool,

    /// Include findings.
    pub findings: bool,

    /// Include recommendations.
    pub recommendations: bool,
}

impl Default for MarkdownSections {
    fn default() -> Self {
        Self {
            metadata: true,
            overview: true,
            metrics: true,
            confidence: true,
            environment: true,
            findings: true,
            recommendations: true,
        }
    }
}

impl MarkdownSections {
    /// Returns a configuration with every report section enabled.
    pub const fn all() -> Self {
        Self {
            metadata: true,
            overview: true,
            metrics: true,
            confidence: true,
            environment: true,
            findings: true,
            recommendations: true,
        }
    }

    /// Returns a minimal report configuration.
    pub const fn minimal() -> Self {
        Self {
            metadata: false,
            overview: true,
            metrics: true,
            confidence: true,
            environment: false,
            findings: true,
            recommendations: false,
        }
    }
}

/// Markdown report rendering options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownReportOptions {
    /// Document title.
    pub title: String,

    /// Maximum output size in bytes.
    pub max_output_bytes: usize,

    /// Maximum bytes retained from one human-readable field.
    pub max_text_bytes: usize,

    /// Decimal places used for floating-point values.
    pub decimal_places: usize,

    /// Whether values should use scientific notation when necessary.
    pub scientific_notation: bool,

    /// Whether metric identifiers are rendered alongside human names.
    pub show_metric_ids: bool,

    /// Whether the benchmark schema version is rendered.
    pub show_schema_version: bool,

    /// Which document sections are enabled.
    pub sections: MarkdownSections,
}

impl Default for MarkdownReportOptions {
    fn default() -> Self {
        Self {
            title: DEFAULT_DOCUMENT_TITLE.to_string(),
            max_output_bytes: DEFAULT_MAX_OUTPUT_BYTES,
            max_text_bytes: DEFAULT_MAX_TEXT_BYTES,
            decimal_places: DEFAULT_DECIMAL_PLACES,
            scientific_notation: true,
            show_metric_ids: true,
            show_schema_version: true,
            sections: MarkdownSections::default(),
        }
    }
}

impl MarkdownReportOptions {
    /// Validates renderer configuration.
    pub fn validate(&self) -> MarkdownResult<()> {
        if self.max_output_bytes == 0 {
            return Err(MarkdownReportError::InvalidOutputLimit);
        }

        if self.max_text_bytes == 0 {
            return Err(MarkdownReportError::InvalidTextLimit);
        }

        if self.max_text_bytes > MAX_TEXT_BYTES {
            return Err(MarkdownReportError::SummaryTooLarge {
                field: "max_text_bytes",
            });
        }

        if self.decimal_places > MAX_DECIMAL_PLACES {
            return Err(MarkdownReportError::InvalidDecimalPlaces {
                value: self.decimal_places,
            });
        }

        Ok(())
    }

    /// Sets the document title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets the maximum output size.
    pub fn with_max_output_bytes(mut self, value: usize) -> Self {
        self.max_output_bytes = value;
        self
    }

    /// Sets the maximum text-field size.
    pub fn with_max_text_bytes(mut self, value: usize) -> Self {
        self.max_text_bytes = value;
        self
    }

    /// Sets the floating-point precision.
    pub fn with_decimal_places(mut self, value: usize) -> Self {
        self.decimal_places = value;
        self
    }

    /// Enables or disables scientific notation.
    pub fn with_scientific_notation(mut self, enabled: bool) -> Self {
        self.scientific_notation = enabled;
        self
    }

    /// Enables or disables metric identifiers.
    pub fn with_metric_ids(mut self, enabled: bool) -> Self {
        self.show_metric_ids = enabled;
        self
    }

    /// Enables or disables schema metadata.
    pub fn with_schema_version(mut self, enabled: bool) -> Self {
        self.show_schema_version = enabled;
        self
    }

    /// Sets the report sections.
    pub fn with_sections(mut self, sections: MarkdownSections) -> Self {
        self.sections = sections;
        self
    }
}

// =============================================================================
// Renderer
// =============================================================================

/// Production Markdown renderer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownReport {
    options: MarkdownReportOptions,
}

impl Default for MarkdownReport {
    fn default() -> Self {
        Self::new(MarkdownReportOptions::default())
    }
}

impl MarkdownReport {
    /// Creates a Markdown renderer with validated options.
    ///
    /// Invalid options are normalized conservatively rather than causing a
    /// panic. Call [`Self::try_new`] when invalid configuration must be
    /// reported explicitly.
    pub fn new(options: MarkdownReportOptions) -> Self {
        let mut normalized = options;

        if normalized.max_output_bytes == 0 {
            normalized.max_output_bytes = DEFAULT_MAX_OUTPUT_BYTES;
        }

        if normalized.max_text_bytes == 0 {
            normalized.max_text_bytes = DEFAULT_MAX_TEXT_BYTES;
        }

        normalized.max_text_bytes =
            normalized.max_text_bytes.min(MAX_TEXT_BYTES);

        normalized.decimal_places =
            normalized.decimal_places.min(MAX_DECIMAL_PLACES);

        Self {
            options: normalized,
        }
    }

    /// Creates a renderer while returning invalid configuration errors.
    pub fn try_new(
        options: MarkdownReportOptions,
    ) -> MarkdownResult<Self> {
        options.validate()?;

        Ok(Self { options })
    }

    /// Returns the renderer configuration.
    #[must_use]
    pub fn options(&self) -> &MarkdownReportOptions {
        &self.options
    }

    /// Renders one complete benchmark summary.
    ///
    /// This is the primary public integration point for `report.rs`.
    pub fn render(
        &self,
        summary: &BenchmarkSummary,
    ) -> MarkdownResult<String> {
        self.validate_summary(summary)?;

        let mut document = String::new();

        self.render_title(&mut document)?;
        self.render_metadata(&mut document, summary)?;
        self.render_overview(&mut document, summary)?;
        self.render_metrics(&mut document, summary)?;
        self.render_environment(&mut document, summary)?;
        self.render_findings(&mut document, summary)?;
        self.render_recommendations(&mut document, summary)?;
        self.render_footer(&mut document)?;

        if document.len() > self.options.max_output_bytes {
            return Err(MarkdownReportError::OutputLimitExceeded {
                max_bytes: self.options.max_output_bytes,
            });
        }

        Ok(document)
    }

    /// Renders a summary using the default production configuration.
    pub fn render_default(
        summary: &BenchmarkSummary,
    ) -> MarkdownResult<String> {
        Self::default().render(summary)
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    fn validate_summary(
        &self,
        summary: &BenchmarkSummary,
    ) -> MarkdownResult<()> {
        self.options.validate()?;

        if summary.identity.benchmark_id.trim().is_empty() {
            return Err(MarkdownReportError::EmptyBenchmarkIdentifier);
        }

        if summary.identity.benchmark_id.len()
            > self.options.max_text_bytes
        {
            return Err(MarkdownReportError::SummaryTooLarge {
                field: "benchmark_id",
            });
        }

        if summary.headline.len() > self.options.max_text_bytes {
            return Err(MarkdownReportError::SummaryTooLarge {
                field: "headline",
            });
        }

        if let Some(description) = &summary.description {
            if description.len() > self.options.max_text_bytes {
                return Err(MarkdownReportError::SummaryTooLarge {
                    field: "description",
                });
            }
        }

        for metric in &summary.metrics {
            self.validate_metric(metric)?;
        }

        for finding in &summary.findings {
            if finding.code.len() > self.options.max_text_bytes
                || finding.message.len() > self.options.max_text_bytes
            {
                return Err(MarkdownReportError::SummaryTooLarge {
                    field: "finding",
                });
            }
        }

        for recommendation in &summary.recommendations {
            if recommendation.len() > self.options.max_text_bytes {
                return Err(MarkdownReportError::SummaryTooLarge {
                    field: "recommendation",
                });
            }
        }

        self.validate_environment(&summary.environment)?;

        Ok(())
    }

    fn validate_metric(
        &self,
        metric: &SummaryMetric,
    ) -> MarkdownResult<()> {
        if !metric.value.is_finite() {
            return Err(MarkdownReportError::NonFiniteMetricValue {
                metric_id: metric.id.clone(),
            });
        }

        if let Some(uncertainty) = metric.uncertainty {
            if !uncertainty.is_finite() || uncertainty < 0.0 {
                return Err(
                    MarkdownReportError::InvalidMetricUncertainty {
                        metric_id: metric.id.clone(),
                    },
                );
            }
        }

        if let Some(confidence) = &metric.confidence {
            if !confidence.level.is_finite()
                || !(0.0 < confidence.level && confidence.level < 1.0)
            {
                return Err(
                    MarkdownReportError::InvalidConfidenceLevel {
                        metric_id: metric.id.clone(),
                    },
                );
            }

            if !confidence.lower.is_finite()
                || !confidence.upper.is_finite()
                || confidence.lower > confidence.upper
            {
                return Err(
                    MarkdownReportError::InvalidConfidenceInterval {
                        metric_id: metric.id.clone(),
                    },
                );
            }
        }

        Ok(())
    }

    fn validate_environment(
        &self,
        environment: &SummaryEnvironment,
    ) -> MarkdownResult<()> {
        let fields = [
            environment.backend_id.as_deref(),
            environment.provider.as_deref(),
            environment.technology.as_deref(),
            environment.compiler_version.as_deref(),
            environment.optimization_level.as_deref(),
            environment.routing_configuration.as_deref(),
            environment.scheduling_configuration.as_deref(),
            environment.calibration_id.as_deref(),
            environment.calibration_timestamp.as_deref(),
        ];

        for field in fields.iter().flatten() {
            if field.len() > self.options.max_text_bytes {
                return Err(MarkdownReportError::SummaryTooLarge {
                    field: "environment",
                });
            }
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Document sections
    // -------------------------------------------------------------------------

    fn render_title(
        &self,
        document: &mut String,
    ) -> MarkdownResult<()> {
        push_line(
            document,
            &format!(
                "# {}",
                escape_heading_text(&bounded_text(
                    &self.options.title,
                    self.options.max_text_bytes,
                ))
            ),
        );

        push_line(document, "");

        Ok(())
    }

    fn render_metadata(
        &self,
        document: &mut String,
        summary: &BenchmarkSummary,
    ) -> MarkdownResult<()> {
        if !self.options.sections.metadata {
            return Ok(());
        }

        push_line(document, "## Report Metadata");
        push_line(document, "");

        push_line(
            document,
            "| Field | Value |",
        );
        push_line(
            document,
            "| --- | --- |",
        );

        push_line(
            document,
            &format!(
                "| Report schema | `{}` v{} |",
                MARKDOWN_REPORT_SCHEMA_ID,
                MARKDOWN_REPORT_SCHEMA_VERSION
            ),
        );

        if self.options.show_schema_version {
            push_line(
                document,
                &format!(
                    "| Summary schema | `{}` |",
                    escape_table_text(&summary.schema_version)
                ),
            );
        }

        push_line(
            document,
            &format!(
                "| Benchmark | `{}` |",
                escape_table_text(
                    &summary.identity.benchmark_id
                )
            ),
        );

        if let Some(version) =
            summary.identity.benchmark_version.as_deref()
        {
            push_line(
                document,
                &format!(
                    "| Benchmark version | `{}` |",
                    escape_table_text(version)
                ),
            );
        }

        if let Some(experiment_id) =
            summary.identity.experiment_id.as_deref()
        {
            push_line(
                document,
                &format!(
                    "| Experiment | `{}` |",
                    escape_table_text(experiment_id)
                ),
            );
        }

        if let Some(workload_id) =
            summary.identity.workload_id.as_deref()
        {
            push_line(
                document,
                &format!(
                    "| Workload | `{}` |",
                    escape_table_text(workload_id)
                ),
            );
        }

        push_line(document, "");

        Ok(())
    }

    fn render_overview(
        &self,
        document: &mut String,
        summary: &BenchmarkSummary,
    ) -> MarkdownResult<()> {
        if !self.options.sections.overview {
            return Ok(());
        }

        push_line(document, "## Overview");
        push_line(document, "");

        push_line(
            document,
            &format!(
                "**Status:** {}",
                status_badge(summary.status)
            ),
        );

        push_line(
            document,
            &format!(
                "**Successful:** {}",
                boolean_label(summary.successful)
            ),
        );

        push_line(
            document,
            &format!(
                "**Metrics:** {}",
                summary.metric_count
            ),
        );

        push_line(
            document,
            &format!(
                "**Findings:** {} \
                 ({} errors, {} warnings, {} informational)",
                summary.finding_count,
                summary.error_count,
                summary.warning_count,
                summary.info_count,
            ),
        );

        push_line(document, "");

        push_line(
            document,
            &escape_paragraph_text(
                &bounded_text(
                    &summary.headline,
                    self.options.max_text_bytes,
                ),
            ),
        );

        push_line(document, "");

        if let Some(description) = &summary.description {
            push_line(document, "### Description");
            push_line(document, "");
            push_line(
                document,
                &escape_paragraph_text(
                    &bounded_text(
                        description,
                        self.options.max_text_bytes,
                    ),
                ),
            );
            push_line(document, "");
        }

        Ok(())
    }

    fn render_metrics(
        &self,
        document: &mut String,
        summary: &BenchmarkSummary,
    ) -> MarkdownResult<()> {
        if !self.options.sections.metrics {
            return Ok(());
        }

        push_line(document, "## Metrics");
        push_line(document, "");

        if summary.metrics.is_empty() {
            push_line(
                document,
                "_No metrics were produced._",
            );
            push_line(document, "");
            return Ok(());
        }

        let mut header =
            String::from("| Metric | Value | Unit");

        if self.options.show_metric_ids {
            header.push_str(" | ID");
        }

        header.push_str(" | Direction | Quality");

        if self.options.sections.confidence {
            header.push_str(" | Uncertainty | Confidence");
        }

        header.push_str(" | Samples | Shots | Circuits |");

        push_line(document, &header);

        let mut separator =
            String::from("| --- | ---: | ---");

        if self.options.show_metric_ids {
            separator.push_str(" | ---");
        }

        separator.push_str(" | --- | ---");

        if self.options.sections.confidence {
            separator.push_str(" | ---: | ---");
        }

        separator.push_str(" | ---: | ---: | ---: |");

        push_line(document, &separator);

        for metric in &summary.metrics {
            let mut row = String::new();

            row.push_str("| ");
            row.push_str(
                &escape_table_text(&metric.name)
            );
            row.push_str(" | ");

            row.push_str(
                &format_float(
                    metric.value,
                    self.options.decimal_places,
                    self.options.scientific_notation,
                ),
            );

            row.push_str(" | ");
            row.push_str(
                &escape_table_text(&metric.unit)
            );

            if self.options.show_metric_ids {
                row.push_str(" | `");
                row.push_str(
                    &escape_table_text(&metric.id)
                );
                row.push('`');
            }

            row.push_str(" | ");
            row.push_str(
                &escape_table_text(
                    metric_direction_label(metric)
                ),
            );

            row.push_str(" | ");
            row.push_str(
                &escape_table_text(
                    metric_quality_label(metric)
                ),
            );

            if self.options.sections.confidence {
                row.push_str(" | ");

                match metric.uncertainty {
                    Some(value) => row.push_str(
                        &format_float(
                            value,
                            self.options.decimal_places,
                            self.options.scientific_notation,
                        ),
                    ),
                    None => row.push('—'),
                }

                row.push_str(" | ");

                match &metric.confidence {
                    Some(confidence) => {
                        row.push_str(
                            &format_confidence(
                                confidence.level,
                                confidence.lower,
                                confidence.upper,
                                &confidence.method,
                                self.options.decimal_places,
                                self.options.scientific_notation,
                            ),
                        );
                    }
                    None => row.push('—'),
                }
            }

            row.push_str(" | ");
            row.push_str(
                &optional_count(metric.sample_count),
            );

            row.push_str(" | ");
            row.push_str(
                &optional_count(metric.shot_count),
            );

            row.push_str(" | ");
            row.push_str(
                &optional_count(metric.circuit_count),
            );

            row.push_str(" |");

            push_line(document, &row);
        }

        push_line(document, "");

        self.render_metric_descriptions(document, summary);

        Ok(())
    }

    fn render_metric_descriptions(
        &self,
        document: &mut String,
        summary: &BenchmarkSummary,
    ) {
        let described = summary
            .metrics
            .iter()
            .filter(|metric| metric.description.is_some());

        let mut emitted = false;

        for metric in described {
            if !emitted {
                push_line(
                    document,
                    "### Metric Descriptions",
                );
                push_line(document, "");
                emitted = true;
            }

            push_line(
                document,
                &format!(
                    "- **{}** — {}",
                    escape_inline_text(&metric.name),
                    escape_paragraph_text(
                        &bounded_text(
                            metric
                                .description
                                .as_deref()
                                .unwrap_or(""),
                            self.options.max_text_bytes,
                        ),
                    ),
                ),
            );
        }

        if emitted {
            push_line(document, "");
        }
    }

    fn render_environment(
        &self,
        document: &mut String,
        summary: &BenchmarkSummary,
    ) -> MarkdownResult<()> {
        if !self.options.sections.environment {
            return Ok(());
        }

        let environment = &summary.environment;

        if environment_is_empty(environment) {
            return Ok(());
        }

        push_line(
            document,
            "## Execution Environment",
        );
        push_line(document, "");

        push_line(
            document,
            "| Field | Value |",
        );
        push_line(
            document,
            "| --- | --- |",
        );

        render_environment_field(
            document,
            "Backend",
            environment.backend_id.as_deref(),
        );

        render_environment_field(
            document,
            "Provider",
            environment.provider.as_deref(),
        );

        render_environment_field(
            document,
            "Technology",
            environment.technology.as_deref(),
        );

        render_environment_field(
            document,
            "Compiler version",
            environment.compiler_version.as_deref(),
        );

        render_environment_field(
            document,
            "Optimization level",
            environment.optimization_level.as_deref(),
        );

        render_environment_field(
            document,
            "Routing configuration",
            environment.routing_configuration.as_deref(),
        );

        render_environment_field(
            document,
            "Scheduling configuration",
            environment.scheduling_configuration.as_deref(),
        );

        render_environment_field(
            document,
            "Calibration ID",
            environment.calibration_id.as_deref(),
        );

        render_environment_field(
            document,
            "Calibration timestamp",
            environment.calibration_timestamp.as_deref(),
        );

        push_line(document, "");

        Ok(())
    }

    fn render_findings(
        &self,
        document: &mut String,
        summary: &BenchmarkSummary,
    ) -> MarkdownResult<()> {
        if !self.options.sections.findings {
            return Ok(());
        }

        push_line(document, "## Findings");
        push_line(document, "");

        if summary.findings.is_empty() {
            push_line(
                document,
                "_No warnings, errors, or informational findings were recorded._",
            );
            push_line(document, "");
            return Ok(());
        }

        for finding in &summary.findings {
            let prefix = finding_prefix(finding.severity);

            let mut line = String::new();

            line.push_str(prefix);
            line.push(' ');

            if !finding.code.is_empty() {
                line.push('`');
                line.push_str(
                    &escape_inline_text(&finding.code),
                );
                line.push_str("` ");
            }

            line.push_str(
                &escape_paragraph_text(
                    &bounded_text(
                        &finding.message,
                        self.options.max_text_bytes,
                    ),
                ),
            );

            if let Some(metric_id) =
                finding.metric_id.as_deref()
            {
                line.push_str(" — metric `");
                line.push_str(
                    &escape_inline_text(metric_id),
                );
                line.push('`');
            }

            push_line(document, &line);
        }

        push_line(document, "");

        Ok(())
    }

    fn render_recommendations(
        &self,
        document: &mut String,
        summary: &BenchmarkSummary,
    ) -> MarkdownResult<()> {
        if !self.options.sections.recommendations {
            return Ok(());
        }

        push_line(
            document,
            "## Recommendations",
        );
        push_line(document, "");

        if summary.recommendations.is_empty() {
            push_line(
                document,
                "_No recommendations were generated._",
            );
            push_line(document, "");
            return Ok(());
        }

        for recommendation in &summary.recommendations {
            push_line(
                document,
                &format!(
                    "- {}",
                    escape_paragraph_text(
                        &bounded_text(
                            recommendation,
                            self.options.max_text_bytes,
                        ),
                    ),
                ),
            );
        }

        push_line(document, "");

        Ok(())
    }

    fn render_footer(
        &self,
        document: &mut String,
    ) -> MarkdownResult<()> {
        push_line(
            document,
            "---",
        );

        push_line(
            document,
            &format!(
                "_Generated by Zamani Quantum Benchmarking Markdown \
                 reporter `{}` v{}._",
                MARKDOWN_REPORT_SCHEMA_ID,
                MARKDOWN_REPORT_SCHEMA_VERSION
            ),
        );

        Ok(())
    }
}

// =============================================================================
// Free rendering API
// =============================================================================

/// Renders a benchmark summary using the default production Markdown
/// configuration.
///
/// This is intentionally a thin convenience wrapper around `MarkdownReport`.
pub fn render_markdown(
    summary: &BenchmarkSummary,
) -> MarkdownResult<String> {
    MarkdownReport::default().render(summary)
}

/// Renders a benchmark summary with explicit options.
pub fn render_markdown_with_options(
    summary: &BenchmarkSummary,
    options: MarkdownReportOptions,
) -> MarkdownResult<String> {
    MarkdownReport::try_new(options)?.render(summary)
}

// =============================================================================
// Formatting helpers
// =============================================================================

fn push_line(
    output: &mut String,
    line: &str,
) {
    output.push_str(line);
    output.push('\n');
}

fn bounded_text(
    value: &str,
    max_bytes: usize,
) -> String {
    truncate_utf8(value, max_bytes)
}

fn truncate_utf8(
    value: &str,
    max_bytes: usize,
) -> String {
    if value.len() <= max_bytes {
        return sanitize_text(value);
    }

    if max_bytes <= 3 {
        return ".".repeat(max_bytes);
    }

    let target = max_bytes - 3;

    let mut end = target.min(value.len());

    while end > 0 && !value.is_char_boundary(end) {
        end -= 1;
    }

    let mut result = value[..end].to_string();
    result.push_str("...");

    sanitize_text(&result)
}

fn sanitize_text(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            '\n' | '\r' | '\t' => ' ',
            character if character.is_control() => ' ',
            character => character,
        })
        .collect()
}

fn escape_table_text(value: &str) -> String {
    sanitize_text(value)
        .replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace('`', "\\`")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_inline_text(value: &str) -> String {
    sanitize_text(value)
        .replace('\\', "\\\\")
        .replace('`', "\\`")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_heading_text(value: &str) -> String {
    let value = sanitize_text(value);

    let value = value.trim_start_matches('#');

    value
        .replace('\\', "\\\\")
        .replace('`', "\\`")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_paragraph_text(value: &str) -> String {
    let value = sanitize_text(value);

    value
        .replace('\\', "\\\\")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn format_float(
    value: f64,
    decimal_places: usize,
    scientific_notation: bool,
) -> String {
    if !value.is_finite() {
        return "invalid".to_string();
    }

    if value == 0.0 {
        return format!("{:.*}", decimal_places, 0.0_f64);
    }

    let absolute = value.abs();

    if scientific_notation
        && (absolute >= 1.0e12 || absolute < 1.0e-9)
    {
        format!("{:.*e}", decimal_places, value)
    } else {
        format!("{:.*}", decimal_places, value)
    }
}

fn format_confidence(
    level: f64,
    lower: f64,
    upper: f64,
    method: &str,
    decimal_places: usize,
    scientific_notation: bool,
) -> String {
    let level_percent =
        format_float(
            level * 100.0,
            decimal_places,
            scientific_notation,
        );

    let lower =
        format_float(
            lower,
            decimal_places,
            scientific_notation,
        );

    let upper =
        format_float(
            upper,
            decimal_places,
            scientific_notation,
        );

    format!(
        "{}% [{} … {}] ({})",
        level_percent,
        lower,
        upper,
        escape_inline_text(method),
    )
}

fn optional_count(value: Option<u64>) -> String {
    match value {
        Some(value) => value.to_string(),
        None => "—".to_string(),
    }
}

fn boolean_label(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn status_badge(status: SummaryStatus) -> &'static str {
    match status {
        SummaryStatus::Complete => "PASS — complete",

        SummaryStatus::CompleteWithWarnings => {
            "PASS — complete with warnings"
        }

        SummaryStatus::Failed => "FAIL — benchmark criterion failed",

        SummaryStatus::Inconclusive => {
            "INCONCLUSIVE — no scientific decision"
        }

        SummaryStatus::NotApplicable => {
            "NOT APPLICABLE"
        }

        SummaryStatus::Cancelled => {
            "CANCELLED"
        }

        SummaryStatus::TimedOut => {
            "TIMED OUT"
        }
    }
}

fn finding_prefix(severity: SummarySeverity) -> &'static str {
    match severity {
        SummarySeverity::Info => "- [INFO]",

        SummarySeverity::Warning => "- [WARNING]",

        SummarySeverity::Error => "- [ERROR]",
    }
}

fn metric_direction_label(
    metric: &SummaryMetric,
) -> &'static str {
    match metric.direction {
        super::super::core::metric::MetricDirection::HigherIsBetter => {
            "higher is better"
        }

        super::super::core::metric::MetricDirection::LowerIsBetter => {
            "lower is better"
        }

        super::super::core::metric::MetricDirection::Target => {
            "target"
        }

        super::super::core::metric::MetricDirection::Neutral => {
            "neutral"
        }
    }
}

fn metric_quality_label(
    metric: &SummaryMetric,
) -> &'static str {
    match metric.quality {
        super::super::core::metric::MetricQuality::Measured => {
            "measured"
        }

        super::super::core::metric::MetricQuality::Estimated => {
            "estimated"
        }

        super::super::core::metric::MetricQuality::Derived => {
            "derived"
        }

        super::super::core::metric::MetricQuality::Simulated => {
            "simulated"
        }

        super::super::core::metric::MetricQuality::Unavailable => {
            "unavailable"
        }
    }
}

fn environment_is_empty(
    environment: &SummaryEnvironment,
) -> bool {
    environment.backend_id.is_none()
        && environment.provider.is_none()
        && environment.technology.is_none()
        && environment.compiler_version.is_none()
        && environment.optimization_level.is_none()
        && environment.routing_configuration.is_none()
        && environment.scheduling_configuration.is_none()
        && environment.calibration_id.is_none()
        && environment.calibration_timestamp.is_none()
}

fn render_environment_field(
    document: &mut String,
    name: &str,
    value: Option<&str>,
) {
    if let Some(value) = value {
        push_line(
            document,
            &format!(
                "| {} | {} |",
                escape_table_text(name),
                escape_table_text(value),
            ),
        );
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options_are_valid() {
        let options = MarkdownReportOptions::default();

        assert!(options.validate().is_ok());
        assert_eq!(
            options.decimal_places,
            DEFAULT_DECIMAL_PLACES
        );
    }

    #[test]
    fn renderer_normalizes_zero_limits() {
        let renderer = MarkdownReport::new(
            MarkdownReportOptions {
                max_output_bytes: 0,
                max_text_bytes: 0,
                decimal_places: 100,
                ..MarkdownReportOptions::default()
            },
        );

        assert!(
            renderer.options().max_output_bytes > 0
        );

        assert!(
            renderer.options().max_text_bytes > 0
        );

        assert!(
            renderer.options().decimal_places
                <= MAX_DECIMAL_PLACES
        );
    }

    #[test]
    fn renderer_rejects_excessive_precision_when_using_try_new() {
        let options = MarkdownReportOptions {
            decimal_places: MAX_DECIMAL_PLACES + 1,
            ..MarkdownReportOptions::default()
        };

        assert!(matches!(
            MarkdownReport::try_new(options),
            Err(MarkdownReportError::InvalidDecimalPlaces { .. })
        ));
    }

    #[test]
    fn table_values_are_escaped() {
        let value = "a | b < c > d `code`";

        let escaped = escape_table_text(value);

        assert!(escaped.contains("\\|"));
        assert!(escaped.contains("&lt;"));
        assert!(escaped.contains("&gt;"));
        assert!(escaped.contains("\\`"));
    }

    #[test]
    fn control_characters_are_removed_from_single_cells() {
        let value = "a\nb\rc\td";

        let sanitized = sanitize_text(value);

        assert_eq!(sanitized, "a b c d");
    }

    #[test]
    fn utf8_truncation_preserves_character_boundaries() {
        let value = "αβγδεζη";

        let truncated = truncate_utf8(value, 8);

        assert!(truncated.is_char_boundary(
            truncated.len()
        ));
    }

    #[test]
    fn finite_float_formatting_is_deterministic() {
        assert_eq!(
            format_float(1.23456789, 4, false),
            "1.2346"
        );

        assert_eq!(
            format_float(0.0, 4, false),
            "0.0000"
        );
    }

    #[test]
    fn non_finite_float_is_not_rendered_as_nan_or_infinity() {
        assert_eq!(
            format_float(
                f64::NAN,
                4,
                false
            ),
            "invalid"
        );

        assert_eq!(
            format_float(
                f64::INFINITY,
                4,
                false
            ),
            "invalid"
        );
    }

    #[test]
    fn status_labels_preserve_scientific_distinctions() {
        assert_eq!(
            status_badge(
                SummaryStatus::Complete
            ),
            "PASS — complete"
        );

        assert_eq!(
            status_badge(
                SummaryStatus::Failed
            ),
            "FAIL — benchmark criterion failed"
        );

        assert_eq!(
            status_badge(
                SummaryStatus::Inconclusive
            ),
            "INCONCLUSIVE — no scientific decision"
        );
    }

    #[test]
    fn empty_environment_is_detected() {
        assert!(
            environment_is_empty(
                &SummaryEnvironment::default()
            )
        );
    }
}