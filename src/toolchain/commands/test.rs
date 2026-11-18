#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Test Command
//!
//! Coordinates test discovery and execution through the repository's
//! existing testing infrastructure.
//!
//! This module owns command orchestration and structured reporting only.
//! It does not implement test discovery, compiler testing, source parsing,
//! semantic analysis, runtime execution, hardware discovery, or scheduling.
//!
//! A production deployment must register a real TestExecutionProvider.
//! No default provider is supplied because silently running zero tests,
//! fabricated tests, or unrelated tests would produce misleading results.

use serde::{Deserialize, Serialize};

use crate::compiler::testing::regression::{
    RegressionCase,
    RegressionOutcome,
    RegressionResult,
    RegressionRunner,
    RegressionStatistics,
};

use crate::toolchain::api::request::TestRequest;
use crate::toolchain::errors::{
    ToolchainError,
    ToolchainResult,
};
use crate::toolchain::tasks::cancellation::CancellationToken;

/// Schema identifier for machine-readable test reports.
pub const TEST_REPORT_SCHEMA: &str = "zamani.toolchain.test-report.v1";

/// A provider that discovers the actual tests belonging to a request.
///
/// Implementations adapt existing project, workspace, compiler, runtime,
/// conformance, fixture, and domain-specific test authorities.
///
/// The provider must:
///
/// - Respect the request's project scope and selectors.
/// - Respect requested target and profile intent.
/// - Apply the request's operational budgets through the canonical
///   budget/accounting services.
/// - Check cancellation while discovering tests.
/// - Return actual executable test cases, not synthetic placeholders.
/// - Avoid claiming that an undiscovered or unsupported test was executed.
///
/// The returned iterator may borrow from the provider, request, or other
/// provider-owned data for the duration of the command.
pub trait TestExecutionProvider {
    /// Discover the actual test cases selected by the request.
    fn discover<'a>(
        &'a self,
        request: &'a TestRequest,
        cancellation: &'a CancellationToken,
    ) -> ToolchainResult<
        Box<dyn Iterator<Item = RegressionCase<'a>> + 'a>,
    >;
}

/// Machine-readable status of the overall test operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestRunStatus {
    /// At least one test ran and every test passed.
    Passed,

    /// At least one test failed.
    Failed,

    /// No tests were discovered.
    NoTests,

    /// Cancellation was requested before the complete suite finished.
    Cancelled,
}

impl TestRunStatus {
    /// Return the stable machine-readable status string.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::NoTests => "no_tests",
            Self::Cancelled => "cancelled",
        }
    }
}

/// Machine-readable outcome of one executed test case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestCaseStatus {
    /// The test completed successfully.
    Passed,

    /// The test detected a failure.
    Failed,
}

/// Structured report for one completed test case.
///
/// The command emits these reports incrementally through the supplied
/// observer. It does not retain every case report in memory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestCaseReport {
    /// Stable test-case identifier.
    pub id: String,

    /// Optional human-readable description.
    pub description: Option<String>,

    /// Actual test outcome.
    pub status: TestCaseStatus,

    /// Failure explanation, when the test failed.
    pub failure_reason: Option<String>,

    /// Observed execution time in nanoseconds.
    ///
    /// This is diagnostic information, not a correctness guarantee.
    pub elapsed_nanoseconds: u128,
}

impl From<&RegressionResult> for TestCaseReport {
    fn from(result: &RegressionResult) -> Self {
        let (status, failure_reason) = match result.outcome() {
            RegressionOutcome::Passed => {
                (TestCaseStatus::Passed, None)
            }
            RegressionOutcome::Failed { reason } => {
                (TestCaseStatus::Failed, Some(reason.clone()))
            }
        };

        Self {
            id: result.id().as_str().to_owned(),
            description: result.description().map(str::to_owned),
            status,
            failure_reason,
            elapsed_nanoseconds: result.elapsed().as_nanos(),
        }
    }
}

/// Aggregate counts for one test operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestStatistics {
    /// Number of test cases actually executed.
    pub total: u128,

    /// Number of test cases that passed.
    pub passed: u128,

    /// Number of test cases that failed.
    pub failed: u128,
}

impl From<RegressionStatistics> for TestStatistics {
    fn from(statistics: RegressionStatistics) -> Self {
        Self {
            total: statistics.total(),
            passed: statistics.passed(),
            failed: statistics.failed(),
        }
    }
}

/// Structured result of a test command.
///
/// A cancelled result may contain partial statistics. Those statistics
/// describe only the cases that actually completed before cancellation.
/// They must never be interpreted as a completed successful test run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestCommandResult {
    /// Machine-readable report schema.
    pub schema: String,

    /// Overall test-operation status.
    pub status: TestRunStatus,

    /// Aggregate statistics for completed cases.
    pub statistics: TestStatistics,

    /// Whether the complete discovered iterator was consumed.
    pub completed: bool,
}

impl TestCommandResult {
    fn new(
        status: TestRunStatus,
        statistics: RegressionStatistics,
        completed: bool,
    ) -> Self {
        Self {
            schema: TEST_REPORT_SCHEMA.to_owned(),
            status,
            statistics: statistics.into(),
            completed,
        }
    }
}

/// Iterator adapter that stops supplying cases after cancellation.
///
/// The regression runner remains responsible for executing cases and
/// recording outcomes. This adapter only makes cooperative cancellation
/// visible at case boundaries.
struct CancellationAwareCases<'a> {
    cases: Box<dyn Iterator<Item = RegressionCase<'a>> + 'a>,
    cancellation: &'a CancellationToken,
}

impl<'a> Iterator for CancellationAwareCases<'a> {
    type Item = RegressionCase<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cancellation.is_cancelled() {
            return None;
        }

        self.cases.next()
    }
}

/// Execute the test request using a registered test provider.
///
/// `on_case_result` is called immediately after each test completes.
/// Callers can use it to serialize reports, write them to a report sink,
/// stream them to an IDE, or forward them to an automation client.
///
/// The callback is intentionally supplied by the caller. This command
/// does not choose a filesystem format, network transport, or logging
/// destination.
///
/// Test failures are represented in `TestCommandResult`; they are not
/// converted into infrastructure errors. Discovery and framework failures
/// remain `ToolchainError` values.
///
/// An empty suite returns `NoTests`, never `Passed`.
pub fn execute_tests<P, F>(
    request: &TestRequest,
    provider: &P,
    cancellation: &CancellationToken,
    mut on_case_result: F,
) -> ToolchainResult<TestCommandResult>
where
    P: TestExecutionProvider + ?Sized,
    F: FnMut(&TestCaseReport),
{
    request.validate().map_err(|error| {
        ToolchainError::validation_failed(error.to_string())
            .with_operation("commands.test.validate")
            .with_source(error)
    })?;

    cancellation.check_cancelled()?;

    let discovered = provider
        .discover(request, cancellation)
        .map_err(|error| {
            error.with_operation("commands.test.discover")
        })?;

    cancellation.check_cancelled()?;

    let cases = CancellationAwareCases {
        cases: discovered,
        cancellation,
    };

    let runner = RegressionRunner::new();

    let statistics = runner
        .run_streaming(cases, |result| {
            let report = TestCaseReport::from(result);
            on_case_result(&report);
        })
        .map_err(|error| {
            ToolchainError::provider_failed(
                "The regression test framework could not complete the test run",
            )
            .with_operation("commands.test.execute")
            .with_source(error)
        })?;

    // The runner may finish normally because the cancellation-aware
    // iterator stopped supplying cases. Check cancellation before deciding
    // whether the suite completed.
    if cancellation.is_cancelled() {
        return Ok(TestCommandResult::new(
            TestRunStatus::Cancelled,
            statistics,
            false,
        ));
    }

    let status = if statistics.total() == 0 {
        TestRunStatus::NoTests
    } else if statistics.failed() > 0 {
        TestRunStatus::Failed
    } else {
        TestRunStatus::Passed
    };

    Ok(TestCommandResult::new(
        status,
        statistics,
        true,
    ))
}