
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Distributed execution results for Zamani.
//!
//! Path: src/distributed/execution/results.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! This module owns the in-memory result contract for distributed
//! execution:
//!
//! - successful task results;
//! - successful execution reports;
//! - partial reports after failed execution;
//! - execution failure envelopes;
//! - deterministic result ordering;
//! - result aggregation and summary information.
//!
//! # Architectural ownership
//!
//! This module owns result envelopes and their aggregation.
//!
//! It does not own:
//!
//! - source-language values or types;
//! - semantic analysis;
//! - ZUIR, ZCIR, quantum::ir, HDLIR, or other canonical IRs;
//! - backend compilation;
//! - task planning, partitioning, placement, routing, or scheduling;
//! - execution-provider selection;
//! - runtime lifecycle transitions;
//! - cancellation propagation;
//! - transport protocols or wire encoding;
//! - durable persistence or checkpoint storage;
//! - cryptographic hashing or signing;
//! - canonical provenance semantics.
//!
//! Output values are supplied by the authoritative execution provider.
//! Their representation and interpretation remain with the existing
//! domain IR, runtime, and backend integrations.
//!
//! # Integration
//!
//! - `crate::distributed::ids`: canonical PlanId and TaskId.
//! - `crate::distributed::error`: canonical DistributedError.
//! - `execution::executor`: constructs task results and reports.
//! - `execution::lifecycle`: records lifecycle transitions independently.
//! - `execution::mod`: exports this module's public API.
//! - `transport`: introduces versioned DTOs when results cross a process
//!   or network boundary.
//! - `runtime` and persistence: own durable result retention.
//! - provenance subsystem: owns semantic provenance and evidence.
//!
//! # Result validity
//!
//! A successful task result means its execution provider returned success.
//! It does not independently prove semantic correctness, authorization,
//! durability, remote replication, or cryptographic authenticity.
//!
//! A successful execution report means all work items supplied to the
//! executor completed successfully according to the provider contract.
//!
//! A partial report is never represented as a successful report.
//!
//! # Failure semantics
//!
//! A failed invocation retains results already confirmed successful.
//! The original DistributedError is preserved without replacing its
//! classification or source chain.
//!
//! A provider error indicating an uncertain remote outcome remains
//! uncertain. This module never changes it into a definite failure or
//! authorizes an automatic retry.
//!
//! # Time semantics
//!
//! Durations are measured locally with a monotonic clock.
//! They are not distributed wall-clock timestamps and must not be used
//! to infer ordering between different machines.
//!
//! # Serialization
//!
//! These in-memory result types deliberately do not derive Serialize
//! or Deserialize.
//!
//! Output values may have domain-specific serialization requirements.
//! Cross-process persistence and transport must use explicitly versioned
//! DTOs, with validation before constructing trusted domain objects.
//!
//! # Scalability
//!
//! No application-defined maximum is imposed on:
//!
//! - task results;
//! - result collections;
//! - output sizes;
//! - execution reports;
//! - partial reports.
//!
//! Actual memory, address-space, output representation, and configured
//! operational budgets remain finite. Allocation failure and resource
//! exhaustion must be handled by the owning runtime or process policy.

use std::fmt;
use std::time::{Duration, Instant};

use crate::distributed::error::DistributedError;
use crate::distributed::ids::{PlanId, TaskId};

// -----------------------------------------------------------------------------
// Task result
// -----------------------------------------------------------------------------

/// Confirmed successful output from one logical distributed task.
///
/// The output type is supplied by the existing execution provider.
/// This type does not introduce a competing Zamani value representation.
///
/// A result is created only after the provider returns `Ok(output)`.
///
/// The provider identifier records which provider returned the successful
/// result. It is descriptive execution provenance, not an authenticated
/// identity or proof of provider trust.
pub struct TaskExecutionResult<O> {
    pub(crate) task_id: TaskId,
    pub(crate) provider_id: String,
    pub(crate) output: O,
    pub(crate) elapsed: Duration,
}

impl<O> TaskExecutionResult<O> {
    /// Construct a task result from a confirmed successful provider response.
    ///
    /// This constructor is crate-visible because result creation belongs
    /// to the execution subsystem. External callers should obtain results
    /// through the executor rather than manufacture successful records.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider identifier is empty, whitespace-only,
    /// or contains control characters.
    pub(crate) fn new(
        task_id: TaskId,
        provider_id: String,
        output: O,
        elapsed: Duration,
    ) -> Result<Self, ResultValidationError> {
        validate_provider_id(&provider_id)?;

        Ok(Self {
            task_id,
            provider_id,
            output,
            elapsed,
        })
    }

    /// Returns the logical task identifier.
    #[must_use]
    pub const fn task_id(&self) -> TaskId {
        self.task_id
    }

    /// Returns the identifier of the provider that executed the task.
    ///
    /// This is the provider's registered identifier, not a physical
    /// machine identifier or an authentication credential.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    /// Returns the output produced by the execution provider.
    #[must_use]
    pub fn output(&self) -> &O {
        &self.output
    }

    /// Consumes the result and returns its output.
    #[must_use]
    pub fn into_output(self) -> O {
        self.output
    }

    /// Returns the locally measured execution duration.
    ///
    /// This measurement uses a monotonic local clock. It is not a
    /// globally comparable timestamp.
    #[must_use]
    pub const fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// Returns the result's output together with its logical task identity.
    #[must_use]
    pub fn into_parts(self) -> (TaskId, O) {
        (self.task_id, self.output)
    }
}

impl<O: fmt::Debug> fmt::Debug for TaskExecutionResult<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaskExecutionResult")
            .field("task_id", &self.task_id)
            .field("provider_id", &self.provider_id)
            .field("output", &self.output)
            .field("elapsed", &self.elapsed)
            .finish()
    }
}

// -----------------------------------------------------------------------------
// Result validation
// -----------------------------------------------------------------------------

/// Validation failure when constructing an execution result.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ResultValidationError {
    /// The provider identifier is invalid.
    InvalidProviderId {
        /// The supplied provider identifier.
        provider_id: String,
    },
}

impl fmt::Display for ResultValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidProviderId { provider_id } => {
                write!(
                    f,
                    "invalid execution provider identifier: {provider_id:?}"
                )
            }
        }
    }
}

impl std::error::Error for ResultValidationError {}

fn validate_provider_id(
    provider_id: &str,
) -> Result<(), ResultValidationError> {
    if provider_id.trim().is_empty()
        || provider_id.chars().any(char::is_control)
    {
        return Err(ResultValidationError::InvalidProviderId {
            provider_id: provider_id.to_owned(),
        });
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Execution summary
// -----------------------------------------------------------------------------

/// Aggregate statistics for one execution report.
///
/// These statistics describe the records held by the report.
/// They do not claim that the report contains every event from a remote
/// runtime or every operation performed by a provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionSummary {
    completed_tasks: usize,
    total_task_time: Duration,
    elapsed: Duration,
}

impl ExecutionSummary {
    /// Returns the number of confirmed successful task results.
    #[must_use]
    pub const fn completed_tasks(&self) -> usize {
        self.completed_tasks
    }

    /// Returns the sum of locally measured task durations.
    ///
    /// This is cumulative task time, not necessarily wall-clock time.
    /// Parallel execution can make this value greater than report elapsed
    /// time.
    #[must_use]
    pub const fn total_task_time(&self) -> Duration {
        self.total_task_time
    }

    /// Returns the total local duration of the execution invocation.
    #[must_use]
    pub const fn elapsed(&self) -> Duration {
        self.elapsed
    }
}

// -----------------------------------------------------------------------------
// Successful execution report
// -----------------------------------------------------------------------------

/// Successful report for a complete distributed execution invocation.
///
/// A report is produced only after every work item supplied to the executor
/// has returned a successful result.
///
/// The report preserves the order in which the executor accepted completed
/// task results. When the executor follows a validated sequential plan,
/// this is the plan's execution order.
///
/// The report does not independently establish semantic correctness,
/// durable persistence, remote replication, or provider authenticity.
pub struct ExecutionReport<O> {
    pub(crate) plan_id: PlanId,
    pub(crate) results: Vec<TaskExecutionResult<O>>,
    pub(crate) started_at: Instant,
    pub(crate) finished_at: Instant,
}

impl<O> ExecutionReport<O> {
    /// Constructs a completed report.
    ///
    /// This constructor is crate-visible. Only the execution subsystem
    /// should finalize a successful report after all work has succeeded.
    #[must_use]
    pub(crate) fn new(
        plan_id: PlanId,
        results: Vec<TaskExecutionResult<O>>,
        started_at: Instant,
        finished_at: Instant,
    ) -> Self {
        Self {
            plan_id,
            results,
            started_at,
            finished_at,
        }
    }

    /// Returns the logical plan identifier.
    #[must_use]
    pub const fn plan_id(&self) -> PlanId {
        self.plan_id
    }

    /// Returns all successful task results in report order.
    #[must_use]
    pub fn results(&self) -> &[TaskExecutionResult<O>] {
        &self.results
    }

    /// Returns the number of successful task results.
    #[must_use]
    pub fn completed_tasks(&self) -> usize {
        self.results.len()
    }

    /// Returns whether the report contains no task results.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Returns the total local execution duration.
    ///
    /// The duration is calculated using monotonic instants and saturates
    /// rather than underflowing if the supplied instants are inconsistent.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.finished_at
            .saturating_duration_since(self.started_at)
    }

    /// Returns aggregate statistics for this report.
    #[must_use]
    pub fn summary(&self) -> ExecutionSummary {
        let total_task_time = self.results.iter().fold(
            Duration::ZERO,
            |total, result| total.saturating_add(result.elapsed()),
        );

        ExecutionSummary {
            completed_tasks: self.results.len(),
            total_task_time,
            elapsed: self.elapsed(),
        }
    }

    /// Returns a result for the supplied logical task identifier.
    ///
    /// The first matching result is returned. Task identity uniqueness
    /// remains a validated-plan responsibility.
    #[must_use]
    pub fn result_for(
        &self,
        task_id: TaskId,
    ) -> Option<&TaskExecutionResult<O>> {
        self.results
            .iter()
            .find(|result| result.task_id() == task_id)
    }

    /// Returns an iterator over successful results.
    ///
    /// The iterator preserves report order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &TaskExecutionResult<O>> {
        self.results.iter()
    }

    /// Consumes the report and returns its successful task results.
    #[must_use]
    pub fn into_results(self) -> Vec<TaskExecutionResult<O>> {
        self.results
    }

    /// Consumes the report and returns the plan identifier and results.
    #[must_use]
    pub fn into_parts(self) -> (PlanId, Vec<TaskExecutionResult<O>>) {
        (self.plan_id, self.results)
    }
}

impl<O: fmt::Debug> fmt::Debug for ExecutionReport<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecutionReport")
            .field("plan_id", &self.plan_id)
            .field("results", &self.results)
            .field("summary", &self.summary())
            .finish()
    }
}

// -----------------------------------------------------------------------------
// Failed execution
// -----------------------------------------------------------------------------

/// Failure of a distributed execution invocation.
///
/// Results from tasks that were already confirmed successful are retained.
/// The report is partial and MUST NOT be interpreted as a successful
/// execution of the complete plan.
///
/// The original distributed error is preserved. In particular, an
/// `OperationOutcomeUnknown` error must remain uncertain; this type does
/// not reinterpret it as a definite failure or authorize a retry.
pub struct ExecutionFailure<O> {
    pub(crate) error: DistributedError,
    pub(crate) partial_report: ExecutionReport<O>,
}

impl<O> ExecutionFailure<O> {
    /// Constructs a failed execution envelope.
    ///
    /// This constructor is crate-visible because the executor owns the
    /// decision that an invocation has failed.
    #[must_use]
    pub(crate) fn new(
        error: DistributedError,
        partial_report: ExecutionReport<O>,
    ) -> Self {
        Self {
            error,
            partial_report,
        }
    }

    /// Returns the original classified distributed error.
    #[must_use]
    pub fn error(&self) -> &DistributedError {
        &self.error
    }

    /// Consumes the failure and returns its original distributed error.
    #[must_use]
    pub fn into_error(self) -> DistributedError {
        self.error
    }

    /// Returns the results confirmed successful before the failure.
    #[must_use]
    pub fn partial_report(&self) -> &ExecutionReport<O> {
        &self.partial_report
    }

    /// Returns the logical plan identifier associated with the failure.
    #[must_use]
    pub const fn plan_id(&self) -> PlanId {
        self.partial_report.plan_id()
    }

    /// Returns the number of tasks confirmed successful before failure.
    #[must_use]
    pub fn completed_tasks(&self) -> usize {
        self.partial_report.completed_tasks()
    }

    /// Consumes the failure and returns its error and partial report.
    #[must_use]
    pub fn into_parts(self) -> (DistributedError, ExecutionReport<O>) {
        (self.error, self.partial_report)
    }
}

impl<O: fmt::Debug> fmt::Debug for ExecutionFailure<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecutionFailure")
            .field("error", &self.error)
            .field("partial_report", &self.partial_report)
            .finish()
    }
}

// -----------------------------------------------------------------------------
// Aggregation
// -----------------------------------------------------------------------------

/// Collects successful task results for one logical plan.
///
/// This collector is an in-memory aggregation utility. It does not execute
/// tasks, validate dependencies, establish task uniqueness, or provide
/// durable storage.
///
/// The caller is responsible for supplying results from the appropriate
/// execution plan and preserving the plan's required ordering.
#[derive(Debug)]
pub struct ExecutionResultCollector<O> {
    plan_id: PlanId,
    results: Vec<TaskExecutionResult<O>>,
    started_at: Instant,
}

impl<O> ExecutionResultCollector<O> {
    /// Creates an empty collector for a plan.
    #[must_use]
    pub fn new(plan_id: PlanId) -> Self {
        Self {
            plan_id,
            results: Vec::new(),
            started_at: Instant::now(),
        }
    }

    /// Adds a provider-confirmed successful task result.
    ///
    /// The collector preserves insertion order.
    pub fn push(&mut self, result: TaskExecutionResult<O>) {
        self.results.push(result);
    }

    /// Returns the number of results currently collected.
    #[must_use]
    pub fn len(&self) -> usize {
        self.results.len()
    }

    /// Returns whether no results have been collected.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Returns the logical plan identifier.
    #[must_use]
    pub const fn plan_id(&self) -> PlanId {
        self.plan_id
    }

    /// Borrows the collected results in insertion order.
    #[must_use]
    pub fn results(&self) -> &[TaskExecutionResult<O>] {
        &self.results
    }

    /// Finalizes the collected results as a successful report.
    ///
    /// The caller MUST call this only after confirming that every required
    /// work item in the plan has completed successfully. The collector
    /// cannot independently establish plan completeness.
    #[must_use]
    pub fn finish(self) -> ExecutionReport<O> {
        ExecutionReport::new(
            self.plan_id,
            self.results,
            self.started_at,
            Instant::now(),
        )
    }

    /// Finalizes the collected results as a partial report for a failure.
    ///
    /// The supplied error is preserved without reclassification.
    #[must_use]
    pub fn fail(
        self,
        error: DistributedError,
    ) -> ExecutionFailure<O> {
        let partial_report = ExecutionReport::new(
            self.plan_id,
            self.results,
            self.started_at,
            Instant::now(),
        );

        ExecutionFailure::new(error, partial_report)
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use crate::distributed::ids::{PlanId, TaskId};

    fn plan_id() -> PlanId {
        PlanId::new(1, 1).expect("valid test plan ID")
    }

    fn task_id(sequence: u128) -> TaskId {
        TaskId::new(1, sequence).expect("valid test task ID")
    }

    fn result(
        sequence: u128,
        output: u64,
    ) -> TaskExecutionResult<u64> {
        TaskExecutionResult::new(
            task_id(sequence),
            "test-provider".to_owned(),
            output,
            Duration::from_millis(10),
        )
        .expect("valid test result")
    }

    #[test]
    fn task_result_preserves_identity_output_and_provider() {
        let value = result(2, 42);

        assert_eq!(value.task_id(), task_id(2));
        assert_eq!(value.provider_id(), "test-provider");
        assert_eq!(*value.output(), 42);
        assert_eq!(value.elapsed(), Duration::from_millis(10));
    }

    #[test]
    fn rejects_empty_provider_identifier() {
        let value = TaskExecutionResult::new(
            task_id(1),
            "  ".to_owned(),
            42_u64,
            Duration::ZERO,
        );

        assert!(matches!(
            value,
            Err(ResultValidationError::InvalidProviderId { .. })
        ));
    }

    #[test]
    fn rejects_control_characters_in_provider_identifier() {
        let value = TaskExecutionResult::new(
            task_id(1),
            "provider\ninvalid".to_owned(),
            42_u64,
            Duration::ZERO,
        );

        assert!(matches!(
            value,
            Err(ResultValidationError::InvalidProviderId { .. })
        ));
    }

    #[test]
    fn report_preserves_result_order() {
        let started = Instant::now();

        let report = ExecutionReport::new(
            plan_id(),
            vec![result(1, 10), result(2, 20), result(3, 30)],
            started,
            Instant::now(),
        );

        assert_eq!(report.completed_tasks(), 3);
        assert_eq!(*report.results()[0].output(), 10);
        assert_eq!(*report.results()[1].output(), 20);
        assert_eq!(*report.results()[2].output(), 30);
    }

    #[test]
    fn report_supports_logical_task_lookup() {
        let report = ExecutionReport::new(
            plan_id(),
            vec![result(1, 10), result(2, 20)],
            Instant::now(),
            Instant::now(),
        );

        assert_eq!(
            report.result_for(task_id(2)).map(|item| *item.output()),
            Some(20)
        );

        assert!(report.result_for(task_id(99)).is_none());
    }

    #[test]
    fn summary_aggregates_task_durations() {
        let report = ExecutionReport::new(
            plan_id(),
            vec![result(1, 10), result(2, 20)],
            Instant::now(),
            Instant::now(),
        );

        let summary = report.summary();

        assert_eq!(summary.completed_tasks(), 2);
        assert_eq!(
            summary.total_task_time(),
            Duration::from_millis(20)
        );
    }

    #[test]
    fn empty_plan_can_produce_an_empty_successful_report() {
        let report: ExecutionReport<u64> = ExecutionReport::new(
            plan_id(),
            Vec::new(),
            Instant::now(),
            Instant::now(),
        );

        assert!(report.is_empty());
        assert_eq!(report.completed_tasks(), 0);
    }

    #[test]
    fn collector_preserves_insertion_order() {
        let mut collector = ExecutionResultCollector::new(plan_id());

        collector.push(result(1, 100));
        collector.push(result(2, 200));

        assert_eq!(collector.len(), 2);

        let report = collector.finish();

        assert_eq!(*report.results()[0].output(), 100);
        assert_eq!(*report.results()[1].output(), 200);
    }

    #[test]
    fn collector_failure_retains_completed_results() {
        let mut collector = ExecutionResultCollector::new(plan_id());

        collector.push(result(1, 100));

        let failure = collector.fail(
            DistributedError::planning_failed(
                "test",
                "deliberate test failure",
            ),
        );

        assert_eq!(failure.completed_tasks(), 1);
        assert_eq!(
            *failure.partial_report().results()[0].output(),
            100
        );
    }

    #[test]
    fn failure_keeps_original_error_classification() {
        let error = DistributedError::planning_failed(
            "test",
            "planning failed",
        );

        let failure = ExecutionFailure::new(
            error,
            ExecutionReport::new(
                plan_id(),
                Vec::<TaskExecutionResult<u64>>::new(),
                Instant::now(),
                Instant::now(),
            ),
        );

        assert_eq!(
            failure.error().code(),
            "distributed.planning_failed"
        );
    }

    #[test]
    fn consuming_report_returns_original_results() {
        let report = ExecutionReport::new(
            plan_id(),
            vec![result(1, 123)],
            Instant::now(),
            Instant::now(),
        );

        let (id, results) = report.into_parts();

        assert_eq!(id, plan_id());
        assert_eq!(results.len(), 1);
        assert_eq!(*results[0].output(), 123);
    }
}
