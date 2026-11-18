
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Task Reporting
//!
//! Path: `src/toolchain/tasks/reporting.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Converts completed task-executor results into stable, deterministic,
//! structured reports suitable for:
//!
//! - Build orchestration.
//! - Command-line presentation.
//! - IDE and editor integrations.
//! - Human and AI-agent clients.
//! - Machine-readable JSON output.
//! - Auditing and reproducibility workflows.
//!
//! ## Ownership
//!
//! This module owns report representation, report validation, aggregation,
//! and report rendering.
//!
//! It does not own:
//!
//! - Task definitions or lifecycle transitions (`tasks::model`).
//! - Task execution or execution policy (`tasks::executor`).
//! - Provider registration (`tasks::registry`).
//! - Dependency planning or graph ordering (`tasks::dependencies`,
//!   `build::graph`).
//! - Cancellation coordination (`tasks::cancellation`).
//! - Artifact creation, storage, or integrity verification.
//! - Compiler, backend, hardware, runtime, or distributed scheduling.
//! - Global logging, telemetry collection, or secret management.
//!
//! ## Determinism
//!
//! - Task records retain the executor's declared execution order.
//! - Artifact maps use ordered maps.
//! - JSON object fields are emitted in a stable order.
//! - Durations use integer seconds and nanoseconds.
//! - No wall-clock timestamp is fabricated from a monotonic `Instant`.
//!
//! ## Scalability
//!
//! This module introduces no fixed task, artifact, output, domain, target,
//! processor, accelerator, or report-size ceiling.
//!
//! Report storage is proportional to the supplied execution records and
//! artifact references. Rendering is streaming-capable through `write_json`
//! and `write_text`; callers can therefore avoid creating an additional
//! complete output string.
//!
//! Actual memory, address-space, and output-stream availability remain
//! practical constraints. A caller may enforce configurable operational
//! output budgets outside this module.
//!
//! ## Security
//!
//! Reports may contain provider-supplied error messages. Callers that expose
//! reports outside a trusted process boundary should use
//! `from_execution_with_redactor` and provide the repository's authoritative
//! secret-redaction policy. This module does not guess which strings are
//! credentials or silently modify diagnostic evidence.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::num::NonZeroU128;
use std::time::Duration;

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::ids::{ArtifactId, TaskId};
use crate::toolchain::result::ToolchainResult;

use super::executor::{
    TaskExecutionRecord,
    TaskExecutionReport,
    ExecutionId,
};
use super::model::{TaskFailure, TaskStatus};

/// Current schema version for the task-report JSON representation.
///
/// This is independent of the Zamani language, compiler, IR, package,
/// and toolchain API versions.
pub const TASK_REPORT_SCHEMA_VERSION: &str = "1";

/// A stable, structured task outcome for reporting.
///
/// This type is deliberately independent of the task model's mutable
/// lifecycle state. It is a snapshot of the outcome at report time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReportedTaskStatus {
    /// Task was created but never made ready.
    Pending,

    /// Task became ready but did not start.
    Ready,

    /// Task was waiting for prerequisites.
    Blocked {
        /// Recorded blocking explanation.
        reason: String,
    },

    /// Task was executing when the snapshot was taken.
    Running {
        /// Execution attempt.
        attempt: NonZeroU128,
    },

    /// Task completed successfully.
    Succeeded {
        /// Successful execution attempt.
        attempt: NonZeroU128,
    },

    /// Task execution failed.
    Failed {
        /// Failed execution attempt.
        attempt: NonZeroU128,

        /// Stable failure classification.
        code: String,

        /// Human-readable failure explanation.
        message: String,

        /// Whether retry may be considered by the owning executor.
        retryable: bool,
    },

    /// Task was cancelled.
    Cancelled {
        /// Attempt, if execution had started.
        attempt: Option<NonZeroU128>,

        /// Cancellation explanation.
        reason: String,
    },

    /// Task was intentionally skipped.
    Skipped {
        /// Explanation for the skip.
        reason: String,
    },
}

impl ReportedTaskStatus {
    /// Returns the stable machine-readable status name.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Ready => "ready",
            Self::Blocked { .. } => "blocked",
            Self::Running { .. } => "running",
            Self::Succeeded { .. } => "succeeded",
            Self::Failed { .. } => "failed",
            Self::Cancelled { .. } => "cancelled",
            Self::Skipped { .. } => "skipped",
        }
    }

    /// Returns whether the task completed successfully.
    #[must_use]
    pub const fn is_successful(&self) -> bool {
        matches!(self, Self::Succeeded { .. })
    }

    /// Returns whether the task reached a terminal state.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Succeeded { .. }
                | Self::Failed { .. }
                | Self::Cancelled { .. }
                | Self::Skipped { .. }
        )
    }
}

impl From<&TaskStatus> for ReportedTaskStatus {
    fn from(status: &TaskStatus) -> Self {
        match status {
            TaskStatus::Pending => Self::Pending,
            TaskStatus::Ready => Self::Ready,

            TaskStatus::Blocked { reason } => Self::Blocked {
                reason: reason.clone(),
            },

            TaskStatus::Running { attempt } => Self::Running {
                attempt: *attempt,
            },

            TaskStatus::Succeeded { attempt, .. } => Self::Succeeded {
                attempt: *attempt,
            },

            TaskStatus::Failed { attempt, failure } => {
                Self::Failed {
                    attempt: *attempt,
                    code: failure.code.clone(),
                    message: failure.message.clone(),
                    retryable: failure.retryable,
                }
            }

            TaskStatus::Cancelled { attempt, reason } => {
                Self::Cancelled {
                    attempt: *attempt,
                    reason: reason.clone(),
                }
            }

            TaskStatus::Skipped { reason } => Self::Skipped {
                reason: reason.clone(),
            },
        }
    }
}

/// One task's immutable report record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReportedTask {
    task_id: String,
    status: ReportedTaskStatus,
    started_after: Duration,
    elapsed: Duration,
    outputs: BTreeMap<String, String>,
    error: Option<String>,
}

impl ReportedTask {
    /// Returns the canonical textual task identity.
    #[must_use]
    pub fn task_id(&self) -> &str {
        &self.task_id
    }

    /// Returns the task outcome.
    #[must_use]
    pub fn status(&self) -> &ReportedTaskStatus {
        &self.status
    }

    /// Returns the elapsed time from execution start until task start.
    #[must_use]
    pub const fn started_after(&self) -> Duration {
        self.started_after
    }

    /// Returns the task's measured execution duration.
    #[must_use]
    pub const fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// Returns declared output names and their artifact identities.
    #[must_use]
    pub fn outputs(&self) -> &BTreeMap<String, String> {
        &self.outputs
    }

    /// Returns the executor-supplied error description, if present.
    #[must_use]
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
}

/// Aggregate task counts.
///
/// These counts are derived from the task records, not trusted blindly
/// from independently maintained executor counters.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TaskReportSummary {
    /// Number of task records.
    pub total: usize,

    /// Tasks not yet made ready.
    pub pending: usize,

    /// Tasks ready to execute.
    pub ready: usize,

    /// Tasks blocked by prerequisites.
    pub blocked: usize,

    /// Tasks currently running at snapshot time.
    pub running: usize,

    /// Successfully completed tasks.
    pub succeeded: usize,

    /// Failed tasks.
    pub failed: usize,

    /// Cancelled tasks.
    pub cancelled: usize,

    /// Intentionally skipped tasks.
    pub skipped: usize,
}

impl TaskReportSummary {
    /// Returns whether every reported task succeeded.
    ///
    /// An empty report is considered complete because it contains no
    /// unsuccessful or unfinished task. Build orchestration may impose
    /// a separate non-empty-build requirement.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.total == self.succeeded
    }

    /// Returns whether any task failed.
    #[must_use]
    pub const fn has_failures(&self) -> bool {
        self.failed != 0
    }

    /// Returns whether any task was cancelled.
    #[must_use]
    pub const fn has_cancellations(&self) -> bool {
        self.cancelled != 0
    }

    /// Returns whether any task remains unfinished.
    #[must_use]
    pub const fn has_unfinished_tasks(&self) -> bool {
        self.pending != 0
            || self.ready != 0
            || self.blocked != 0
            || self.running != 0
    }
}

/// Complete, validated task-execution report.
///
/// This report is a snapshot. It does not keep executor state alive,
/// retain provider objects, or own artifact storage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskReport {
    schema_version: String,
    execution_id: String,
    elapsed: Duration,
    summary: TaskReportSummary,
    tasks: Vec<ReportedTask>,
    outputs: BTreeMap<String, BTreeMap<String, String>>,
}

impl TaskReport {
    /// Build a report from the executor's completed report.
    ///
    /// Error messages are copied without modification. Use
    /// `from_execution_with_redactor` when the report will cross a
    /// trust boundary.
    pub fn from_execution(
        execution: &TaskExecutionReport,
    ) -> ToolchainResult<Self> {
        Self::from_execution_with_redactor(execution, str::to_owned)
    }

    /// Build a report while applying the caller's authoritative
    /// diagnostic redaction policy to error and failure messages.
    ///
    /// The redactor must be deterministic if reproducible report output
    /// is required. It must not remove stable error codes or alter task
    /// and artifact identities.
    pub fn from_execution_with_redactor<F>(
        execution: &TaskExecutionReport,
        redact: F,
    ) -> ToolchainResult<Self>
    where
        F: Fn(&str) -> String,
    {
        let mut tasks = Vec::new();
        tasks.try_reserve(execution.records.len()).map_err(|error| {
            ToolchainError::resource_exhausted(format!(
                "Unable to reserve task report records: {error}"
            ))
            .with_operation("tasks.reporting")
        })?;

        let mut seen = BTreeSet::new();
        let mut summary = TaskReportSummary::default();

        for record in &execution.records {
            let task_id = record.task_id.to_string();

            if !seen.insert(task_id.clone()) {
                return Err(report_validation_error(format!(
                    "Execution report contains duplicate task identity '{task_id}'"
                )));
            }

            let status = ReportedTaskStatus::from(&record.status);

            increment_summary(&mut summary, &status)?;

            let mut outputs = BTreeMap::new();
            for (name, artifact) in &record.outputs {
                outputs.insert(name.clone(), artifact.to_string());
            }

            let error = record.error.as_deref().map(&redact);

            let reported = ReportedTask {
                task_id,
                status,
                started_after: record.started_after,
                elapsed: record.elapsed,
                outputs,
                error,
            };

            validate_record(record, &reported)?;
            tasks.push(reported);
        }

        summary.total = tasks.len();

        // The executor's aggregate counters are checked against the
        // record-level outcomes. This prevents contradictory reports
        // from being presented as trustworthy execution evidence.
        validate_executor_counters(execution, &summary)?;
        validate_execution_outputs(execution, &tasks)?;

        let mut outputs = BTreeMap::new();
        for (task_id, task_outputs) in &execution.outputs {
            let mut converted = BTreeMap::new();

            for (name, artifact) in task_outputs {
                converted.insert(name.clone(), artifact.to_string());
            }

            outputs.insert(task_id.to_string(), converted);
        }

        Ok(Self {
            schema_version: TASK_REPORT_SCHEMA_VERSION.to_owned(),
            execution_id: execution.execution_id.as_str().to_owned(),
            elapsed: execution.elapsed,
            summary,
            tasks,
            outputs,
        })
    }

    /// Returns the report schema version.
    #[must_use]
    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    /// Returns the executor invocation identity.
    #[must_use]
    pub fn execution_id(&self) -> &str {
        &self.execution_id
    }

    /// Returns total execution elapsed time.
    #[must_use]
    pub const fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// Returns the derived aggregate summary.
    #[must_use]
    pub const fn summary(&self) -> &TaskReportSummary {
        &self.summary
    }

    /// Returns task records in executor execution order.
    #[must_use]
    pub fn tasks(&self) -> &[ReportedTask] {
        &self.tasks
    }

    /// Returns the successfully collected task outputs.
    #[must_use]
    pub fn outputs(&self) -> &BTreeMap<String, BTreeMap<String, String>> {
        &self.outputs
    }

    /// Returns whether every reported task succeeded.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.summary.is_success()
    }

    /// Render the report as deterministic JSON.
    ///
    /// This convenience method allocates a string proportional to the
    /// rendered report size. Use `write_json` to stream into a caller-owned
    /// formatter instead.
    pub fn to_json(&self) -> String {
        let mut output = String::new();
        // Writing to String is infallible.
        let _ = self.write_json(&mut output);
        output
    }

    /// Stream deterministic JSON into a caller-owned formatter.
    ///
    /// No report-size ceiling is imposed by this method. The destination
    /// controls its own storage and write-failure behavior.
    pub fn write_json<W: fmt::Write>(
        &self,
        writer: &mut W,
    ) -> fmt::Result {
        write!(
            writer,
            "{{\"schema_version\":{},\"execution_id\":{},\"elapsed\":",
            json_string(&self.schema_version),
            json_string(&self.execution_id),
        )?;

        write_duration_json(writer, self.elapsed)?;

        write!(
            writer,
            ",\"summary\":{{\"total\":{},\"pending\":{},\"ready\":{},\
             \"blocked\":{},\"running\":{},\"succeeded\":{},\
             \"failed\":{},\"cancelled\":{},\"skipped\":{}}},\"tasks\":[",
            self.summary.total,
            self.summary.pending,
            self.summary.ready,
            self.summary.blocked,
            self.summary.running,
            self.summary.succeeded,
            self.summary.failed,
            self.summary.cancelled,
            self.summary.skipped,
        )?;

        for (index, task) in self.tasks.iter().enumerate() {
            if index != 0 {
                writer.write_char(',')?;
            }

            write_task_json(writer, task)?;
        }

        writer.write_str("],\"outputs\":{")?;

        for (task_index, (task_id, task_outputs)) in
            self.outputs.iter().enumerate()
        {
            if task_index != 0 {
                writer.write_char(',')?;
            }

            write!(
                writer,
                "{}:{{",
                json_string(task_id),
            )?;

            for (output_index, (name, artifact)) in
                task_outputs.iter().enumerate()
            {
                if output_index != 0 {
                    writer.write_char(',')?;
                }

                write!(
                    writer,
                    "{}:{}",
                    json_string(name),
                    json_string(artifact),
                )?;
            }

            writer.write_char('}')?;
        }

        writer.write_str("}}")
    }

    /// Render a concise human-readable report.
    ///
    /// Task order matches the executor's recorded order.
    pub fn to_text(&self) -> String {
        let mut output = String::new();
        let _ = self.write_text(&mut output);
        output
    }

    /// Stream a human-readable report into a caller-owned formatter.
    pub fn write_text<W: fmt::Write>(
        &self,
        writer: &mut W,
    ) -> fmt::Result {
        writeln!(writer, "Zamani task execution report")?;
        writeln!(writer, "Execution: {}", self.execution_id)?;
        writeln!(writer, "Schema: {}", self.schema_version)?;
        writeln!(
            writer,
            "Elapsed: {}.{:09}s",
            self.elapsed.as_secs(),
            self.elapsed.subsec_nanos(),
        )?;

        writeln!(
            writer,
            "Tasks: {} total, {} succeeded, {} failed, \
             {} cancelled, {} skipped",
            self.summary.total,
            self.summary.succeeded,
            self.summary.failed,
            self.summary.cancelled,
            self.summary.skipped,
        )?;

        for task in &self.tasks {
            writeln!(
                writer,
                "- {} [{}] started-after={}s elapsed={}s",
                task.task_id,
                task.status.as_str(),
                format_duration(task.started_after),
                format_duration(task.elapsed),
            )?;

            if let Some(error) = &task.error {
                writeln!(writer, "  error: {error}")?;
            }

            for (name, artifact) in &task.outputs {
                writeln!(writer, "  output {name}: {artifact}")?;
            }
        }

        Ok(())
    }
}

fn increment_summary(
    summary: &mut TaskReportSummary,
    status: &ReportedTaskStatus,
) -> ToolchainResult<()> {
    let counter = match status {
        ReportedTaskStatus::Pending => &mut summary.pending,
        ReportedTaskStatus::Ready => &mut summary.ready,
        ReportedTaskStatus::Blocked { .. } => &mut summary.blocked,
        ReportedTaskStatus::Running { .. } => &mut summary.running,
        ReportedTaskStatus::Succeeded { .. } => &mut summary.succeeded,
        ReportedTaskStatus::Failed { .. } => &mut summary.failed,
        ReportedTaskStatus::Cancelled { .. } => &mut summary.cancelled,
        ReportedTaskStatus::Skipped { .. } => &mut summary.skipped,
    };

    *counter = counter.checked_add(1).ok_or_else(|| {
        ToolchainError::resource_exhausted(
            "Task report status counter overflow",
        )
        .with_operation("tasks.reporting")
    })?;

    Ok(())
}

fn validate_record(
    source: &TaskExecutionRecord,
    reported: &ReportedTask,
) -> ToolchainResult<()> {
    match &source.status {
        TaskStatus::Succeeded {
            outputs: status_outputs,
            ..
        } => {
            if status_outputs != &source.outputs {
                return Err(report_validation_error(format!(
                    "Task '{}' has inconsistent successful output records",
                    reported.task_id
                )));
            }
        }

        _ if !source.outputs.is_empty() => {
            return Err(report_validation_error(format!(
                "Non-successful task '{}' contains reported outputs",
                reported.task_id
            )));
        }

        _ => {}
    }

    if let ReportedTaskStatus::Failed { message, .. } = &reported.status {
        if message.is_empty() {
            return Err(report_validation_error(format!(
                "Failed task '{}' has an empty failure message",
                reported.task_id
            )));
        }
    }

    Ok(())
}

fn validate_executor_counters(
    execution: &TaskExecutionReport,
    summary: &TaskReportSummary,
) -> ToolchainResult<()> {
    if execution.succeeded != summary.succeeded
        || execution.failed != summary.failed
        || execution.cancelled != summary.cancelled
    {
        return Err(report_validation_error(
            "Executor aggregate counters do not match its task records",
        ));
    }

    Ok(())
}

fn validate_execution_outputs(
    execution: &TaskExecutionReport,
    tasks: &[ReportedTask],
) -> ToolchainResult<()> {
    let mut expected = BTreeMap::<String, BTreeMap<String, String>>::new();

    for task in tasks {
        if task.status.is_successful() {
            expected.insert(task.task_id.clone(), task.outputs.clone());
        }
    }

    let mut actual = BTreeMap::<String, BTreeMap<String, String>>::new();

    for (task_id, task_outputs) in &execution.outputs {
        let mut converted = BTreeMap::new();

        for (name, artifact) in task_outputs {
            converted.insert(name.clone(), artifact.to_string());
        }

        actual.insert(task_id.to_string(), converted);
    }

    if expected != actual {
        return Err(report_validation_error(
            "Executor output index does not match successful task records",
        ));
    }

    Ok(())
}

fn report_validation_error(
    message: impl Into<String>,
) -> crate::toolchain::errors::ToolchainError {
    ToolchainError::validation_failed(message)
        .with_operation("tasks.reporting")
}

fn write_task_json<W: fmt::Write>(
    writer: &mut W,
    task: &ReportedTask,
) -> fmt::Result {
    write!(
        writer,
        "{{\"task_id\":{},\"status\":{},\"started_after\":",
        json_string(&task.task_id),
        json_string(task.status.as_str()),
    )?;

    write_duration_json(writer, task.started_after)?;

    writer.write_str(",\"elapsed\":")?;
    write_duration_json(writer, task.elapsed)?;

    writer.write_str(",\"status_details\":")?;
    write_status_details_json(writer, &task.status)?;

    writer.write_str(",\"outputs\":{")?;

    for (index, (name, artifact)) in task.outputs.iter().enumerate() {
        if index != 0 {
            writer.write_char(',')?;
        }

        write!(
            writer,
            "{}:{}",
            json_string(name),
            json_string(artifact),
        )?;
    }

    writer.write_str("},\"error\":")?;

    match &task.error {
        Some(error) => writer.write_str(&json_string(error))?,
        None => writer.write_str("null")?,
    }

    writer.write_char('}')
}

fn write_status_details_json<W: fmt::Write>(
    writer: &mut W,
    status: &ReportedTaskStatus,
) -> fmt::Result {
    match status {
        ReportedTaskStatus::Pending
        | ReportedTaskStatus::Ready
        | ReportedTaskStatus::Succeeded { .. } => {
            writer.write_str("{}")
        }

        ReportedTaskStatus::Blocked { reason }
        | ReportedTaskStatus::Skipped { reason } => {
            write!(writer, "{{\"reason\":{}}}", json_string(reason))
        }

        ReportedTaskStatus::Running { attempt }
        | ReportedTaskStatus::Succeeded { attempt } => {
            write!(writer, "{{\"attempt\":{}}}", attempt)
        }

        ReportedTaskStatus::Failed {
            attempt,
            code,
            message,
            retryable,
        } => {
            write!(
                writer,
                "{{\"attempt\":{},\"code\":{},\"message\":{},\
                 \"retryable\":{}}}",
                attempt,
                json_string(code),
                json_string(message),
                retryable,
            )
        }

        ReportedTaskStatus::Cancelled { attempt, reason } => {
            writer.write_str("{\"attempt\":")?;

            match attempt {
                Some(attempt) => write!(writer, "{attempt}")?,
                None => writer.write_str("null")?,
            }

            write!(
                writer,
                ",\"reason\":{}}}",
                json_string(reason),
            )
        }
    }
}

fn write_duration_json<W: fmt::Write>(
    writer: &mut W,
    duration: Duration,
) -> fmt::Result {
    write!(
        writer,
        "{{\"seconds\":{},\"nanoseconds\":{}}}",
        duration.as_secs(),
        duration.subsec_nanos(),
    )
}

fn format_duration(duration: Duration) -> String {
    format!(
        "{}.{:09}",
        duration.as_secs(),
        duration.subsec_nanos(),
    )
}

/// Escape a string according to JSON string-value rules.
///
/// The returned value includes the surrounding quotation marks.
fn json_string(value: &str) -> String {
    let mut output = String::with_capacity(value.len().saturating_add(2));
    output.push('"');

    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{0008}' => output.push_str("\\b"),
            '\u{000C}' => output.push_str("\\f"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),

            character if character <= '\u{001F}' => {
                use fmt::Write as _;

                let _ = write!(
                    output,
                    "\\u{:04x}",
                    character as u32,
                );
            }

            character => output.push(character),
        }
    }

    output.push('"');
    output
}

#[cfg(test)]
mod tests {
    use super::{
        json_string,
        ReportedTaskStatus,
        TaskReportSummary,
    };

    #[test]
    fn status_names_are_stable() {
        assert_eq!(
            ReportedTaskStatus::Pending.as_str(),
            "pending"
        );

        assert_eq!(
            ReportedTaskStatus::Succeeded {
                attempt: std::num::NonZeroU128::new(1)
                    .expect("one is nonzero"),
            }
            .as_str(),
            "succeeded"
        );
    }

    #[test]
    fn summary_requires_every_task_to_succeed() {
        let summary = TaskReportSummary {
            total: 3,
            succeeded: 2,
            failed: 1,
            ..TaskReportSummary::default()
        };

        assert!(!summary.is_success());
        assert!(summary.has_failures());
        assert!(!summary.has_cancellations());
    }

    #[test]
    fn summary_distinguishes_unfinished_tasks() {
        let summary = TaskReportSummary {
            total: 2,
            succeeded: 1,
            running: 1,
            ..TaskReportSummary::default()
        };

        assert!(summary.has_unfinished_tasks());
        assert!(!summary.is_success());
    }

    #[test]
    fn json_escaping_handles_control_characters() {
        assert_eq!(
            json_string("a\"b\\c\n"),
            "\"a\\\"b\\\\c\\n\""
        );

        assert_eq!(
            json_string("\u{0001}"),
            "\"\\u0001\""
        );
    }

    #[test]
    fn json_escaping_preserves_unicode() {
        assert_eq!(
            json_string("Zamani — quantum"),
            "\"Zamani — quantum\""
        );
    }
}
