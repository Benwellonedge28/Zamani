//! Zamani Quantum Benchmarking — Tabular Reporting
//!
//! Production-grade, backend-independent tabular rendering for
//! `BenchmarkResult` and canonical `Metric` values.
//!
//! # Architectural role
//!
//! This module converts already-computed benchmark results into deterministic
//! human-readable tables. It does NOT:
//!
//! - execute benchmarks;
//! - generate circuits;
//! - calculate metrics;
//! - perform statistical analysis;
//! - modify benchmark results;
//! - communicate with hardware;
//! - perform JSON serialization;
//! - perform CSV serialization;
//! - perform Markdown document generation;
//! - select benchmark protocols.
//!
//! The dependency direction is:
//!
//! ```text
//! Quantum IR / frontend
//!          │
//!          ▼
//! Benchmark execution
//!          │
//!          ▼
//! Statistics / metrics
//!          │
//!          ▼
//! core::result::BenchmarkResult
//!          │
//!          ▼
//! reporting::table
//! ```
//!
//! # Production requirements
//!
//! This implementation provides:
//!
//! - deterministic column ordering;
//! - deterministic row ordering;
//! - plain-text tables;
//! - Markdown tables;
//! - compact metric tables;
//! - complete benchmark-result tables;
//! - configurable alignment;
//! - configurable numeric precision;
//! - bounded rendering;
//! - bounded cell lengths;
//! - safe handling of control characters;
//! - Unicode-width-independent formatting;
//! - no terminal escape sequences;
//! - no mutation of benchmark results;
//! - no logging side effects;
//! - no panics for malformed user-controlled cell contents;
//! - explicit handling of empty tables;
//! - stable machine-readable metric identifiers;
//! - metric uncertainty/confidence rendering;
//! - execution-status rendering;
//! - scientific benchmark-status rendering;
//! - warning/error counts;
//! - deterministic output suitable for CI and regression fixtures.
//!
//! # Important semantic boundary
//!
//! A table is a presentation of a result. It must never change the scientific
//! meaning of a result.
//!
//! In particular:
//!
//! - percentages are not silently converted from fractions;
//! - confidence intervals are not recalculated;
//! - units are not inferred from metric names;
//! - metrics are not sorted according to "better/worse" semantics unless the
//!   caller explicitly requests that behavior;
//! - NaN/infinity are never manufactured by formatting;
//! - partial execution remains visibly partial;
//! - benchmark failure remains distinct from execution failure.
//!
//! # Integration
//!
//! The canonical integration is:
//!
//! ```text
//! quantum::benchmarking::core::result::BenchmarkResult
//!                         │
//!                         ▼
//! quantum::benchmarking::reporting::table::BenchmarkTable
//!                         │
//!             ┌───────────┴───────────┐
//!             ▼                       ▼
//!        Plain text                Markdown
//! ```
//!
//! Future reporting modules should consume this module rather than
//! reimplementing table formatting.
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

use super::super::core::metric::{
    Metric,
    MetricDirection,
    MetricKind,
    MetricQuality,
    MetricUnit,
};
use super::super::core::result::{
    BenchmarkResult,
    BenchmarkStatus,
    ResultExecutionStatus,
    ResultMessageSeverity,
};

// =============================================================================
// Public constants
// =============================================================================

/// Default maximum number of rows rendered by one table.
pub const DEFAULT_MAX_ROWS: usize = 16_384;

/// Default maximum number of columns rendered by one table.
pub const DEFAULT_MAX_COLUMNS: usize = 64;

/// Default maximum UTF-8 bytes retained in one cell before truncation.
pub const DEFAULT_MAX_CELL_BYTES: usize = 4_096;

/// Default number of fractional decimal places.
pub const DEFAULT_DECIMAL_PLACES: usize = 6;

/// Maximum decimal places accepted by the renderer.
///
/// This prevents pathological formatting requests from producing enormous
/// strings.
pub const MAX_DECIMAL_PLACES: usize = 18;

/// Default maximum total rendered output size.
///
/// This protects CI/reporting paths from accidentally rendering enormous
/// result sets.
pub const DEFAULT_MAX_OUTPUT_BYTES: usize = 4 * 1024 * 1024;

/// Stable table schema identifier.
pub const TABLE_SCHEMA_ID: &str = "zamani.quantum.benchmark.table";

/// Stable table schema version.
pub const TABLE_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Alignment
// =============================================================================

/// Horizontal alignment of a table cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// Left-align text.
    Left,

    /// Center-align text.
    Center,

    /// Right-align numeric values.
    Right,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Left
    }
}

// =============================================================================
// Rendering format
// =============================================================================

/// Supported textual table formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableFormat {
    /// Human-readable ASCII-style table.
    Plain,

    /// GitHub/CommonMark-compatible Markdown table.
    Markdown,
}

impl Default for TableFormat {
    fn default() -> Self {
        Self::Plain
    }
}

// =============================================================================
// Cell
// =============================================================================

/// One immutable table cell.
///
/// Cell contents are owned by the table and are therefore safe to render after
/// the source benchmark result has gone out of scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCell {
    value: String,
    alignment: Alignment,
}

impl TableCell {
    /// Creates a left-aligned cell.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: sanitize_cell_text(&value.into()),
            alignment: Alignment::Left,
        }
    }

    /// Creates a cell with explicit alignment.
    pub fn aligned(value: impl Into<String>, alignment: Alignment) -> Self {
        Self {
            value: sanitize_cell_text(&value.into()),
            alignment,
        }
    }

    /// Returns the cell contents.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the alignment.
    #[must_use]
    pub fn alignment(&self) -> Alignment {
        self.alignment
    }
}

// =============================================================================
// Column
// =============================================================================

/// Table column definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableColumn {
    name: String,
    alignment: Alignment,
}

impl TableColumn {
    /// Creates a column with left alignment.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: sanitize_cell_text(&name.into()),
            alignment: Alignment::Left,
        }
    }

    /// Creates a column with explicit alignment.
    pub fn aligned(name: impl Into<String>, alignment: Alignment) -> Self {
        Self {
            name: sanitize_cell_text(&name.into()),
            alignment,
        }
    }

    /// Returns the column name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the column alignment.
    #[must_use]
    pub fn alignment(&self) -> Alignment {
        self.alignment
    }
}

// =============================================================================
// Table
// =============================================================================

/// Immutable, deterministic tabular representation.
///
/// A `Table` is intentionally independent of benchmark protocol logic. It can
/// therefore also be used by future benchmark analysis/reporting modules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    title: Option<String>,
    columns: Vec<TableColumn>,
    rows: Vec<Vec<TableCell>>,
    max_cell_bytes: usize,
}

impl Table {
    /// Creates an empty table.
    ///
    /// The caller should normally use [`TableBuilder`] for construction.
    pub fn new(columns: Vec<TableColumn>) -> Self {
        Self {
            title: None,
            columns,
            rows: Vec::new(),
            max_cell_bytes: DEFAULT_MAX_CELL_BYTES,
        }
    }

    /// Sets the optional title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(sanitize_cell_text(&title.into()));
        self
    }

    /// Sets the maximum number of UTF-8 bytes retained per cell.
    ///
    /// Values of zero are normalized to one.
    pub fn with_max_cell_bytes(mut self, max_cell_bytes: usize) -> Self {
        self.max_cell_bytes = max_cell_bytes.max(1);
        self
    }

    /// Adds a row.
    ///
    /// Rows whose length does not match the number of columns are ignored.
    ///
    /// This method is intentionally infallible so reporting cannot panic while
    /// attempting to display a partially corrupted upstream artifact.
    pub fn push_row(&mut self, row: Vec<TableCell>) {
        if row.len() != self.columns.len() {
            return;
        }

        let row = row
            .into_iter()
            .map(|cell| {
                TableCell::aligned(
                    truncate_utf8(cell.value(), self.max_cell_bytes),
                    cell.alignment(),
                )
            })
            .collect();

        self.rows.push(row);
    }

    /// Returns the optional title.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the columns.
    #[must_use]
    pub fn columns(&self) -> &[TableColumn] {
        &self.columns
    }

    /// Returns the rows.
    #[must_use]
    pub fn rows(&self) -> &[Vec<TableCell>] {
        &self.rows
    }

    /// Returns the number of columns.
    #[must_use]
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// Returns the number of rows.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns true when there are no data rows.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Renders the table.
    ///
    /// Rendering is bounded by `max_output_bytes`.
    ///
    /// If the configured output bound is reached, the renderer appends a
    /// deterministic truncation marker.
    #[must_use]
    pub fn render(&self, format: TableFormat, max_output_bytes: usize) -> String {
        let max_output_bytes = max_output_bytes.max(1);

        match format {
            TableFormat::Plain => self.render_plain(max_output_bytes),
            TableFormat::Markdown => self.render_markdown(max_output_bytes),
        }
    }

    /// Renders using the production default output limit.
    #[must_use]
    pub fn render_default(&self, format: TableFormat) -> String {
        self.render(format, DEFAULT_MAX_OUTPUT_BYTES)
    }

    fn render_plain(&self, max_output_bytes: usize) -> String {
        if self.columns.is_empty() {
            return bounded_string(
                self.title.as_deref().unwrap_or("Empty table"),
                max_output_bytes,
            );
        }

        let widths = self.calculate_widths();

        let mut output = String::new();

        if let Some(title) = &self.title {
            push_bounded_line(
                &mut output,
                title,
                max_output_bytes,
            );

            if output.len() < max_output_bytes {
                push_bounded_line(
                    &mut output,
                    "",
                    max_output_bytes,
                );
            }
        }

        let separator = build_plain_separator(&widths);

        if !push_bounded_line(
            &mut output,
            &separator,
            max_output_bytes,
        ) {
            return output;
        }

        let header = build_plain_row(
            self.columns
                .iter()
                .map(|column| TableCell::aligned(
                    column.name.clone(),
                    column.alignment,
                ))
                .collect::<Vec<_>>()
                .as_slice(),
            &widths,
        );

        if !push_bounded_line(
            &mut output,
            &header,
            max_output_bytes,
        ) {
            return output;
        }

        if !push_bounded_line(
            &mut output,
            &separator,
            max_output_bytes,
        ) {
            return output;
        }

        for row in &self.rows {
            if !push_bounded_line(
                &mut output,
                &build_plain_row(row, &widths),
                max_output_bytes,
            ) {
                break;
            }
        }

        if output.len() < separator.len().saturating_add(1) {
            return output;
        }

        if !output.ends_with('\n') {
            output.push('\n');
        }

        output
    }

    fn render_markdown(&self, max_output_bytes: usize) -> String {
        if self.columns.is_empty() {
            return bounded_string(
                self.title.as_deref().unwrap_or("Empty table"),
                max_output_bytes,
            );
        }

        let mut output = String::new();

        if let Some(title) = &self.title {
            if !push_bounded_line(
                &mut output,
                &format!("### {}", escape_markdown_text(title)),
                max_output_bytes,
            ) {
                return output;
            }

            if !push_bounded_line(
                &mut output,
                "",
                max_output_bytes,
            ) {
                return output;
            }
        }

        let header = format!(
            "| {} |",
            self.columns
                .iter()
                .map(|column| escape_markdown_text(column.name()))
                .collect::<Vec<_>>()
                .join(" | ")
        );

        if !push_bounded_line(
            &mut output,
            &header,
            max_output_bytes,
        ) {
            return output;
        }

        let separator = format!(
            "| {} |",
            self.columns
                .iter()
                .map(|column| markdown_alignment_marker(column.alignment()))
                .collect::<Vec<_>>()
                .join(" | ")
        );

        if !push_bounded_line(
            &mut output,
            &separator,
            max_output_bytes,
        ) {
            return output;
        }

        for row in &self.rows {
            let line = format!(
                "| {} |",
                row.iter()
                    .map(|cell| escape_markdown_text(cell.value()))
                    .collect::<Vec<_>>()
                    .join(" | ")
            );

            if !push_bounded_line(
                &mut output,
                &line,
                max_output_bytes,
            ) {
                break;
            }
        }

        output
    }

    fn calculate_widths(&self) -> Vec<usize> {
        let mut widths = self
            .columns
            .iter()
            .map(|column| visible_width(column.name()))
            .collect::<Vec<_>>();

        for row in &self.rows {
            for (index, cell) in row.iter().enumerate() {
                if let Some(width) = widths.get_mut(index) {
                    *width = (*width).max(visible_width(cell.value()));
                }
            }
        }

        widths
            .into_iter()
            .map(|width| width.max(1))
            .collect()
    }
}

// =============================================================================
// Table builder
// =============================================================================

/// Bounded table builder.
///
/// This is the preferred API when a reporting module is constructing a custom
/// table.
#[derive(Debug, Clone)]
pub struct TableBuilder {
    title: Option<String>,
    columns: Vec<TableColumn>,
    rows: Vec<Vec<TableCell>>,
    max_rows: usize,
    max_columns: usize,
    max_cell_bytes: usize,
}

impl TableBuilder {
    /// Creates a production-default builder.
    pub fn new() -> Self {
        Self {
            title: None,
            columns: Vec::new(),
            rows: Vec::new(),
            max_rows: DEFAULT_MAX_ROWS,
            max_columns: DEFAULT_MAX_COLUMNS,
            max_cell_bytes: DEFAULT_MAX_CELL_BYTES,
        }
    }

    /// Sets the title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(sanitize_cell_text(&title.into()));
        self
    }

    /// Sets maximum rows.
    pub fn max_rows(mut self, max_rows: usize) -> Self {
        self.max_rows = max_rows;
        self
    }

    /// Sets maximum columns.
    pub fn max_columns(mut self, max_columns: usize) -> Self {
        self.max_columns = max_columns;
        self
    }

    /// Sets maximum bytes per cell.
    pub fn max_cell_bytes(mut self, max_cell_bytes: usize) -> Self {
        self.max_cell_bytes = max_cell_bytes.max(1);
        self
    }

    /// Adds a column.
    ///
    /// Returns `false` if the configured column limit has been reached.
    pub fn column(&mut self, column: TableColumn) -> bool {
        if self.columns.len() >= self.max_columns {
            return false;
        }

        self.columns.push(column);
        true
    }

    /// Adds a row.
    ///
    /// Returns `false` if the row is rejected.
    pub fn row(&mut self, row: Vec<TableCell>) -> bool {
        if self.rows.len() >= self.max_rows {
            return false;
        }

        if row.len() != self.columns.len() {
            return false;
        }

        let row = row
            .into_iter()
            .map(|cell| {
                TableCell::aligned(
                    truncate_utf8(
                        cell.value(),
                        self.max_cell_bytes,
                    ),
                    cell.alignment(),
                )
            })
            .collect();

        self.rows.push(row);
        true
    }

    /// Finalizes the table.
    #[must_use]
    pub fn build(self) -> Table {
        Table {
            title: self.title,
            columns: self.columns,
            rows: self.rows,
            max_cell_bytes: self.max_cell_bytes,
        }
    }
}

impl Default for TableBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// BenchmarkResult integration
// =============================================================================

/// High-level production table view of a [`BenchmarkResult`].
///
/// This function deliberately reads the result without modifying or validating
/// it. Validation remains the responsibility of `BenchmarkResult::validate()`.
///
/// This distinction is important because reporting must also be able to render
/// incomplete/failed/partial result artifacts for diagnostics.
#[must_use]
pub fn benchmark_result_table(result: &BenchmarkResult) -> Table {
    let mut builder = TableBuilder::new()
        .title("Zamani Quantum Benchmark Result")
        .max_rows(DEFAULT_MAX_ROWS)
        .max_columns(DEFAULT_MAX_COLUMNS)
        .max_cell_bytes(DEFAULT_MAX_CELL_BYTES);

    let _ = builder.column(TableColumn::new("Field"));
    let _ = builder.column(TableColumn::new("Value"));

    let _ = builder.row(vec![
        TableCell::new("Schema"),
        TableCell::new(format!(
            "{} v{}",
            result.schema_id,
            result.schema_version
        )),
    ]);

    let _ = builder.row(vec![
        TableCell::new("Result ID"),
        TableCell::new(result.result_id.as_str()),
    ]);

    let _ = builder.row(vec![
        TableCell::new("Benchmark"),
        TableCell::new(format!(
            "{} v{}",
            result.benchmark.benchmark_id,
            result.benchmark.benchmark_version
        )),
    ]);

    let _ = builder.row(vec![
        TableCell::new("Benchmark status"),
        TableCell::new(benchmark_status_text(result.benchmark_status)),
    ]);

    let _ = builder.row(vec![
        TableCell::new("Execution status"),
        TableCell::new(execution_status_text(result.execution_status)),
    ]);

    let _ = builder.row(vec![
        TableCell::new("Metrics"),
        TableCell::aligned(
            result.metric_count().to_string(),
            Alignment::Right,
        ),
    ]);

    let _ = builder.row(vec![
        TableCell::new("Observations"),
        TableCell::aligned(
            result.observation_count().to_string(),
            Alignment::Right,
        ),
    ]);

    let _ = builder.row(vec![
        TableCell::new("Requested circuits"),
        TableCell::aligned(
            result.execution.requested_circuits.to_string(),
            Alignment::Right,
        ),
    ]);

    let _ = builder.row(vec![
        TableCell::new("Executed circuits"),
        TableCell::aligned(
            result.execution.executed_circuits.to_string(),
            Alignment::Right,
        ),
    ]);

    let _ = builder.row(vec![
        TableCell::new("Requested shots"),
        TableCell::aligned(
            result.execution.requested_shots.to_string(),
            Alignment::Right,
        ),
    ]);

    let _ = builder.row(vec![
        TableCell::new("Executed shots"),
        TableCell::aligned(
            result.execution.executed_shots.to_string(),
            Alignment::Right,
        ),
    ]);

    let _ = builder.row(vec![
        TableCell::new("Errors"),
        TableCell::aligned(
            result.errors().count().to_string(),
            Alignment::Right,
        ),
    ]);

    let _ = builder.row(vec![
        TableCell::new("Warnings"),
        TableCell::aligned(
            result.warnings().count().to_string(),
            Alignment::Right,
        ),
    ]);

    builder.build()
}

/// Creates a metric table from a benchmark result.
///
/// Metrics are kept in the exact order in which they occur in the canonical
/// result. This is intentional: presentation must not silently reorder
/// scientific data.
#[must_use]
pub fn metric_table(result: &BenchmarkResult) -> Table {
    let mut builder = TableBuilder::new()
        .title("Benchmark Metrics")
        .max_rows(DEFAULT_MAX_ROWS)
        .max_columns(12)
        .max_cell_bytes(DEFAULT_MAX_CELL_BYTES);

    add_metric_columns(&mut builder);

    for metric in result.metrics() {
        if builder.rows.len() >= DEFAULT_MAX_ROWS {
            break;
        }

        let _ = builder.row(metric_to_row(metric));
    }

    builder.build()
}

/// Creates a compact metric table containing only the selected metric kind.
///
/// The canonical metric identity is determined by `MetricKind::id()`.
#[must_use]
pub fn metric_kind_table(
    result: &BenchmarkResult,
    kind: &MetricKind,
) -> Table {
    let mut builder = TableBuilder::new()
        .title(format!("Metric: {}", kind.id()))
        .max_rows(DEFAULT_MAX_ROWS)
        .max_columns(12)
        .max_cell_bytes(DEFAULT_MAX_CELL_BYTES);

    add_metric_columns(&mut builder);

    for metric in result.metrics() {
        if metric.kind == *kind {
            if !builder.row_count_reached() {
                let _ = builder.row(metric_to_row(metric));
            }
        }
    }

    builder.build()
}

/// Creates a compact execution-accounting table.
#[must_use]
pub fn execution_table(result: &BenchmarkResult) -> Table {
    let mut builder = TableBuilder::new()
        .title("Benchmark Execution")
        .max_rows(32)
        .max_columns(3)
        .max_cell_bytes(DEFAULT_MAX_CELL_BYTES);

    let _ = builder.column(TableColumn::new("Quantity"));
    let _ = builder.column(TableColumn::new("Requested"));
    let _ = builder.column(TableColumn::new("Executed"));

    let _ = builder.row(vec![
        TableCell::new("Workloads"),
        TableCell::aligned(
            result.execution.requested_workloads.to_string(),
            Alignment::Right,
        ),
        TableCell::aligned(
            result.execution.completed_workloads.to_string(),
            Alignment::Right,
        ),
    ]);

    let _ = builder.row(vec![
        TableCell::new("Circuits"),
        TableCell::aligned(
            result.execution.requested_circuits.to_string(),
            Alignment::Right,
        ),
        TableCell::aligned(
            result.execution.executed_circuits.to_string(),
            Alignment::Right,
        ),
    ]);

    let _ = builder.row(vec![
        TableCell::new("Shots"),
        TableCell::aligned(
            result.execution.requested_shots.to_string(),
            Alignment::Right,
        ),
        TableCell::aligned(
            result.execution.executed_shots.to_string(),
            Alignment::Right,
        ),
    ]);

    builder.build()
}

/// Creates a table containing result warnings and errors.
///
/// This deliberately preserves their original order and severity.
#[must_use]
pub fn message_table(result: &BenchmarkResult) -> Table {
    let mut builder = TableBuilder::new()
        .title("Benchmark Messages")
        .max_rows(DEFAULT_MAX_ROWS)
        .max_columns(4)
        .max_cell_bytes(DEFAULT_MAX_CELL_BYTES);

    let _ = builder.column(TableColumn::new("Severity"));
    let _ = builder.column(TableColumn::new("Code"));
    let _ = builder.column(TableColumn::new("Scope"));
    let _ = builder.column(TableColumn::new("Message"));

    for message in &result.messages {
        if builder.rows.len() >= DEFAULT_MAX_ROWS {
            break;
        }

        let severity = match message.severity {
            ResultMessageSeverity::Info => "info",
            ResultMessageSeverity::Warning => "warning",
            ResultMessageSeverity::Error => "error",
        };

        let _ = builder.row(vec![
            TableCell::new(severity),
            TableCell::new(&message.code),
            TableCell::new(
                message.scope.as_deref().unwrap_or("—"),
            ),
            TableCell::new(&message.message),
        ]);
    }

    builder.build()
}

// =============================================================================
// Metric formatting
// =============================================================================

fn add_metric_columns(builder: &mut TableBuilder) {
    let _ = builder.column(TableColumn::new("Metric"));
    let _ = builder.column(TableColumn::new("Value"));
    let _ = builder.column(TableColumn::new("Unit"));
    let _ = builder.column(TableColumn::new("Uncertainty"));
    let _ = builder.column(TableColumn::new("Confidence"));
    let _ = builder.column(TableColumn::new("Samples"));
    let _ = builder.column(TableColumn::new("Shots"));
    let _ = builder.column(TableColumn::new("Circuits"));
    let _ = builder.column(TableColumn::new("Direction"));
    let _ = builder.column(TableColumn::new("Quality"));
    let _ = builder.column(TableColumn::new("Description"));
}

fn metric_to_row(metric: &Metric) -> Vec<TableCell> {
    vec![
        TableCell::new(metric.kind.id()),
        TableCell::aligned(
            format_metric_value(metric),
            Alignment::Right,
        ),
        TableCell::new(metric.unit.id()),
        TableCell::aligned(
            metric
                .uncertainty
                .map(|value| format_f64(value.get()))
                .unwrap_or_else(|| "—".to_owned()),
            Alignment::Right,
        ),
        TableCell::new(format_confidence(metric)),
        TableCell::aligned(
            metric
                .sample_count
                .map(|value| value.to_string())
                .unwrap_or_else(|| "—".to_owned()),
            Alignment::Right,
        ),
        TableCell::aligned(
            metric
                .shot_count
                .map(|value| value.to_string())
                .unwrap_or_else(|| "—".to_owned()),
            Alignment::Right,
        ),
        TableCell::aligned(
            metric
                .circuit_count
                .map(|value| value.to_string())
                .unwrap_or_else(|| "—".to_owned()),
            Alignment::Right,
        ),
        TableCell::new(format_direction(metric.direction)),
        TableCell::new(format_quality(metric.quality)),
        TableCell::new(
            metric
                .description
                .as_deref()
                .unwrap_or("—"),
        ),
    ]
}

fn format_metric_value(metric: &Metric) -> String {
    let value = metric.value.get();

    if metric.kind.requires_unit_interval() {
        format_f64(value)
    } else {
        format_f64(value)
    }
}

fn format_confidence(metric: &Metric) -> String {
    match &metric.confidence {
        Some(confidence) => format!(
            "{}% [{}, {}] ({})",
            format_percentage(confidence.level.get()),
            format_f64(confidence.lower.get()),
            format_f64(confidence.upper.get()),
            confidence_method_name(&confidence.method),
        ),
        None => "—".to_owned(),
    }
}

fn confidence_method_name(
    method: &super::super::core::metric::ConfidenceMethod,
) -> &'static str {
    use super::super::core::metric::ConfidenceMethod;

    match method {
        ConfidenceMethod::Wilson => "Wilson",
        ConfidenceMethod::ClopperPearson => "Clopper-Pearson",
        ConfidenceMethod::NormalApproximation => {
            "Normal approximation"
        }
        ConfidenceMethod::Bootstrap => "Bootstrap",
        ConfidenceMethod::Bayesian => "Bayesian",
        ConfidenceMethod::BackendProvided => "Backend provided",
        ConfidenceMethod::Custom(_) => "Custom",
    }
}

fn format_direction(direction: MetricDirection) -> &'static str {
    match direction {
        MetricDirection::HigherIsBetter => "higher",
        MetricDirection::LowerIsBetter => "lower",
        MetricDirection::Neutral => "neutral",
    }
}

fn format_quality(quality: MetricQuality) -> &'static str {
    match quality {
        MetricQuality::Observed => "observed",
        MetricQuality::Derived => "derived",
        MetricQuality::Estimated => "estimated",
        MetricQuality::Fitted => "fitted",
        MetricQuality::Approximate => "approximate",
        MetricQuality::Uncertain => "uncertain",
        MetricQuality::Invalid => "invalid",
    }
}

fn format_f64(value: f64) -> String {
    if !value.is_finite() {
        return "invalid".to_owned();
    }

    if value == 0.0 {
        return "0".to_owned();
    }

    format!("{:.6}", value)
}

fn format_percentage(value: f64) -> String {
    if !value.is_finite() {
        return "invalid".to_owned();
    }

    format!("{:.3}", value * 100.0)
}

// =============================================================================
// Status formatting
// =============================================================================

fn benchmark_status_text(status: BenchmarkStatus) -> &'static str {
    match status {
        BenchmarkStatus::Inconclusive => "inconclusive",
        BenchmarkStatus::Passed => "passed",
        BenchmarkStatus::Failed => "failed",
        BenchmarkStatus::NotApplicable => "not_applicable",
    }
}

fn execution_status_text(
    status: ResultExecutionStatus,
) -> &'static str {
    match status {
        ResultExecutionStatus::NotExecuted => "not_executed",
        ResultExecutionStatus::Running => "running",
        ResultExecutionStatus::Completed => "completed",
        ResultExecutionStatus::PartiallyCompleted => {
            "partially_completed"
        }
        ResultExecutionStatus::Cancelled => "cancelled",
        ResultExecutionStatus::Failed => "failed",
    }
}

// =============================================================================
// Plain table formatting
// =============================================================================

fn build_plain_separator(widths: &[usize]) -> String {
    let mut output = String::new();

    output.push('+');

    for width in widths {
        output.push_str(&"-".repeat(width.saturating_add(2)));
        output.push('+');
    }

    output
}

fn build_plain_row(
    cells: &[TableCell],
    widths: &[usize],
) -> String {
    let mut output = String::new();

    output.push('|');

    for (index, cell) in cells.iter().enumerate() {
        let width = widths.get(index).copied().unwrap_or(1);

        output.push(' ');
        output.push_str(&align_text(
            cell.value(),
            width,
            cell.alignment(),
        ));
        output.push(' ');
        output.push('|');
    }

    output
}

fn align_text(
    value: &str,
    width: usize,
    alignment: Alignment,
) -> String {
    let value_width = visible_width(value);

    if value_width >= width {
        return value.to_owned();
    }

    let padding = width - value_width;

    match alignment {
        Alignment::Left => {
            format!("{}{}", value, " ".repeat(padding))
        }
        Alignment::Right => {
            format!("{}{}", " ".repeat(padding), value)
        }
        Alignment::Center => {
            let left = padding / 2;
            let right = padding - left;

            format!(
                "{}{}{}",
                " ".repeat(left),
                value,
                " ".repeat(right),
            )
        }
    }
}

// =============================================================================
// Markdown formatting
// =============================================================================

fn markdown_alignment_marker(
    alignment: Alignment,
) -> &'static str {
    match alignment {
        Alignment::Left => ":---",
        Alignment::Center => ":---:",
        Alignment::Right => "---:",
    }
}

fn escape_markdown_text(value: &str) -> String {
    let mut output = String::with_capacity(value.len());

    for character in value.chars() {
        match character {
            '|' => output.push_str("\\|"),
            '\r' | '\n' => output.push(' '),
            '\\' => output.push_str("\\\\"),
            _ => output.push(character),
        }
    }

    output
}

// =============================================================================
// Safety helpers
// =============================================================================

/// Sanitizes cell text.
///
/// Terminal control characters are removed/replaced. This is important because
/// benchmark metadata can originate from external hardware, providers, files,
//! or untrusted serialized results.
fn sanitize_cell_text(value: &str) -> String {
    let mut output = String::with_capacity(value.len());

    for character in value.chars() {
        if character == '\n' || character == '\r' || character == '\t' {
            output.push(' ');
        } else if character.is_control() {
            output.push('�');
        } else {
            output.push(character);
        }
    }

    output
}

/// Truncates a UTF-8 string without splitting a code point.
fn truncate_utf8(value: &str, max_bytes: usize) -> String {
    if value.len() <= max_bytes {
        return value.to_owned();
    }

    if max_bytes <= 3 {
        return value
            .char_indices()
            .take_while(|(index, _)| *index < max_bytes)
            .map(|(_, character)| character)
            .collect();
    }

    let target = max_bytes - 3;

    let mut end = 0;

    for (index, character) in value.char_indices() {
        let next = index + character.len_utf8();

        if next > target {
            break;
        }

        end = next;
    }

    let mut output = value[..end].to_owned();
    output.push_str("...");
    output
}

/// Returns a conservative visible width.
///
/// This intentionally uses Unicode scalar count rather than terminal-specific
/// East Asian width rules so the result remains deterministic across terminals,
/// CI systems and operating systems.
fn visible_width(value: &str) -> usize {
    value.chars().count()
}

fn bounded_string(
    value: &str,
    max_bytes: usize,
) -> String {
    truncate_utf8(value, max_bytes.max(1))
}

fn push_bounded_line(
    output: &mut String,
    line: &str,
    max_bytes: usize,
) -> bool {
    let required = line.len().saturating_add(1);

    if output.len().saturating_add(required) <= max_bytes {
        output.push_str(line);
        output.push('\n');
        return true;
    }

    let remaining = max_bytes.saturating_sub(output.len());

    if remaining == 0 {
        return false;
    }

    let marker = "... [table output truncated]";

    if remaining >= marker.len().saturating_add(1) {
        output.push_str(marker);
        output.push('\n');
    } else {
        let truncated = truncate_utf8(marker, remaining);
        output.push_str(&truncated);
    }

    false
}

// =============================================================================
// Additional builder convenience
// =============================================================================

impl TableBuilder {
    fn row_count_reached(&self) -> bool {
        self.rows.len() >= self.max_rows
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_table_is_safe() {
        let table = Table::new(Vec::new());

        let plain = table.render(
            TableFormat::Plain,
            DEFAULT_MAX_OUTPUT_BYTES,
        );

        assert!(!plain.is_empty());
    }

    #[test]
    fn builder_rejects_mismatched_rows() {
        let mut builder = TableBuilder::new();

        assert!(builder.column(TableColumn::new("A")));
        assert!(builder.column(TableColumn::new("B")));

        assert!(!builder.row(vec![TableCell::new("only one")]));
        assert_eq!(builder.build().row_count(), 0);
    }

    #[test]
    fn plain_table_is_deterministic() {
        let mut builder = TableBuilder::new();

        let _ = builder.column(TableColumn::new("Name"));
        let _ = builder.column(
            TableColumn::aligned(
                "Value",
                Alignment::Right,
            ),
        );

        let _ = builder.row(vec![
            TableCell::new("alpha"),
            TableCell::aligned(
                "42",
                Alignment::Right,
            ),
        ]);

        let table = builder.build();

        let first = table.render_default(TableFormat::Plain);
        let second = table.render_default(TableFormat::Plain);

        assert_eq!(first, second);
        assert!(first.contains("alpha"));
        assert!(first.contains("42"));
    }

    #[test]
    fn markdown_escapes_pipes() {
        let mut builder = TableBuilder::new();

        let _ = builder.column(TableColumn::new("Value"));

        let _ = builder.row(vec![
            TableCell::new("a | b"),
        ]);

        let table = builder.build();

        let output = table.render_default(TableFormat::Markdown);

        assert!(output.contains("a \\| b"));
    }

    #[test]
    fn control_characters_are_sanitized() {
        let cell = TableCell::new("hello\nworld\tvalue");

        assert_eq!(cell.value(), "hello world value");
    }

    #[test]
    fn truncation_does_not_split_utf8() {
        let value = "你好世界";

        let truncated = truncate_utf8(value, 7);

        assert!(truncated.is_char_boundary(truncated.len()));
        assert!(truncated.len() <= 7);
    }

    #[test]
    fn output_is_bounded() {
        let mut builder = TableBuilder::new();

        let _ = builder.column(TableColumn::new("Value"));

        for _ in 0..100 {
            let _ = builder.row(vec![
                TableCell::new("this is deliberately long"),
            ]);
        }

        let table = builder.build();

        let output = table.render(
            TableFormat::Plain,
            128,
        );

        assert!(output.len() <= 128);
    }

    #[test]
    fn alignment_is_deterministic() {
        assert_eq!(
            align_text("x", 3, Alignment::Left),
            "x  "
        );

        assert_eq!(
            align_text("x", 3, Alignment::Right),
            "  x"
        );

        assert_eq!(
            align_text("x", 3, Alignment::Center),
            " x "
        );
    }

    #[test]
    fn metric_kind_identifier_is_preserved() {
        let kind = MetricKind::QuantumVolume;

        assert_eq!(
            kind.id(),
            "quantum_volume"
        );
    }
}