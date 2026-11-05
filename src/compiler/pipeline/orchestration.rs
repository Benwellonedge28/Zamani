
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Pipeline Orchestration.
//!
//! This module owns the lifecycle and coordination of one compiler-pipeline
//! invocation. It composes the existing stage, ordering, execution,
//! compilation-context, compilation-state and failure contracts.
//!
//! It does not define another stage system or executor.
//!
//! # Architectural ownership
//!
//! This module owns:
//! - PipelineOrchestrator.
//! - PipelineOrchestrationError and its classification.
//! - OrchestrationExecutionReport and its stage records.
//! - Compilation lifecycle transitions around pipeline execution.
//! - Conversion of execution results into orchestration results.
//!
//! Existing modules remain authoritative for:
//! - Stage contracts: pipeline::stage.
//! - Dependency graph validation: pipeline::ordering.
//! - Stage execution: pipeline::execution.
//! - Failure normalization: pipeline::failure.
//! - Recovery policy: pipeline::recovery.
//! - Compilation configuration and budgets: compiler::context and
//!   compiler::configuration.
//! - Domain semantics, IR, optimization, hardware and runtime execution.
//!
//! # Scalability
//!
//! No fixed stage-count, device-count, worker-count, memory, operation-count,
//! or hardware-capacity limit is imposed here.
//!
//! The pipeline uses the resource budgets and cancellation state supplied
//! through CompilationContext. Unlimited configured budgets do not imply
//! infinite physical resources.
//!
//! # Determinism
//!
//! The existing StageOrdering defines deterministic topological order.
//! The existing PipelineExecutor executes that order deterministically.
//! This orchestrator does not introduce nondeterministic concurrent execution.
//!
//! # Conditional stages
//!
//! A skipped stage is not a completed stage. Its dependents cannot consume
//! its output. Required stages must complete successfully.
//!
//! # Grammar boundary
//!
//! Source-level compilation declarations belong to the grammar, frontend
//! and semantic authorities. This module coordinates compiler implementation
//! stages only. It does not interpret grammar declarations or source intent.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::compiler::context::compilation_context::CompilationContext;
use crate::compiler::context::compilation_state::{
    CompilationLifecycle,
    CompilationStateError,
    CompilationStateTracker,
};

use crate::compiler::pipeline::execution::{
    ExecutionError,
    ExecutionErrorKind,
    ExecutionReport as PipelineExecutionReport,
    PipelineExecutor,
    StageExecutionStatus as PipelineStageExecutionStatus,
};

use crate::compiler::pipeline::ordering::{
    OrderingError,
    StageOrdering,
};

use crate::compiler::pipeline::stage::{
    Stage,
    StageError,
    StageId,
    StageInput,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version for pipeline orchestration.
pub const PIPELINE_ORCHESTRATION_API_VERSION: u16 = 1;

// ============================================================================
// ORCHESTRATION STAGE STATUS
// ============================================================================

/// Final orchestration-level status of a stage.
///
/// This is an immutable reporting representation. The execution subsystem
/// remains the authority for actual stage invocation and output ownership.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StageExecutionStatus {
    /// The stage completed successfully.
    Completed,

    /// The stage deliberately declined execution.
    Skipped {
        reason: Arc<str>,
    },

    /// The stage could not execute because a prerequisite did not complete.
    Blocked {
        dependency: StageId,
        reason: Arc<str>,
    },

    /// The stage invocation failed.
    Failed {
        message: Arc<str>,
    },
}

// ============================================================================
// ORCHESTRATION STAGE RECORD
// ============================================================================

/// Immutable record of one stage's orchestration outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageExecutionRecord {
    stage_id: StageId,
    status: StageExecutionStatus,
}

impl StageExecutionRecord {
    /// Creates a completed-stage record.
    #[must_use]
    pub fn completed(stage_id: StageId) -> Self {
        Self {
            stage_id,
            status: StageExecutionStatus::Completed,
        }
    }

    /// Creates a skipped-stage record.
    #[must_use]
    pub fn skipped(
        stage_id: StageId,
        reason: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            stage_id,
            status: StageExecutionStatus::Skipped {
                reason: reason.into(),
            },
        }
    }

    /// Creates a blocked-stage record.
    #[must_use]
    pub fn blocked(
        stage_id: StageId,
        dependency: StageId,
        reason: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            stage_id,
            status: StageExecutionStatus::Blocked {
                dependency,
                reason: reason.into(),
            },
        }
    }

    /// Creates a failed-stage record.
    #[must_use]
    pub fn failed(
        stage_id: StageId,
        message: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            stage_id,
            status: StageExecutionStatus::Failed {
                message: message.into(),
            },
        }
    }

    /// Returns the stage identifier.
    #[must_use]
    pub fn stage_id(&self) -> &StageId {
        &self.stage_id
    }

    /// Returns the final stage status.
    #[must_use]
    pub fn status(&self) -> &StageExecutionStatus {
        &self.status
    }

    /// Returns whether the stage completed successfully.
    #[must_use]
    pub fn is_completed(&self) -> bool {
        matches!(self.status, StageExecutionStatus::Completed)
    }
}

// ============================================================================
// ORCHESTRATION EXECUTION REPORT
// ============================================================================

/// Immutable summary of one pipeline invocation.
///
/// The report owns orchestration-level stage records and the set of required
/// stages. Actual stage outputs remain owned by the execution report.
///
/// Keeping the two responsibilities separate avoids copying or duplicating
/// domain-specific IR, artifacts or other stage payloads.
#[derive(Debug, Clone, Default)]
pub struct ExecutionReport {
    records: std::collections::BTreeMap<StageId, StageExecutionRecord>,
    required_stages: BTreeSet<StageId>,
}

impl ExecutionReport {
    /// Creates an empty report with the caller's mandatory-stage contract.
    #[must_use]
    pub fn new(required_stages: BTreeSet<StageId>) -> Self {
        Self {
            records: std::collections::BTreeMap::new(),
            required_stages,
        }
    }

    /// Converts the existing executor report without taking ownership.
    ///
    /// Completed, skipped and blocked states are preserved exactly.
    #[must_use]
    pub fn from_execution(report: &PipelineExecutionReport) -> Self {
        let mut result = Self::new(report.required_stages().clone());

        for (stage_id, record) in report.records() {
            let converted = match record.status() {
                PipelineStageExecutionStatus::Completed => {
                    StageExecutionRecord::completed(stage_id.clone())
                }

                PipelineStageExecutionStatus::Skipped => {
                    StageExecutionRecord::skipped(
                        stage_id.clone(),
                        record.reason().unwrap_or("stage was skipped"),
                    )
                }

                PipelineStageExecutionStatus::Blocked => {
                    // The executor records the blocking explanation.
                    // The precise prerequisite is resolved by orchestration
                    // when it is known; it is not fabricated here.
                    StageExecutionRecord {
                        stage_id: stage_id.clone(),
                        status: StageExecutionStatus::Blocked {
                            dependency: stage_id.clone(),
                            reason: Arc::from(
                                record.reason().unwrap_or(
                                    "a prerequisite did not complete",
                                ),
                            ),
                        },
                    }
                }
            };

            result.records.insert(stage_id.clone(), converted);
        }

        result
    }

    /// Adds or replaces one stage record.
    ///
    /// This is used by orchestration when an executor error identifies
    /// the failed stage after the executor has returned its partial report.
    fn insert(&mut self, record: StageExecutionRecord) {
        self.records
            .insert(record.stage_id().clone(), record);
    }

    /// Returns all stage records in deterministic identifier order.
    #[must_use]
    pub fn records(
        &self,
    ) -> &std::collections::BTreeMap<StageId, StageExecutionRecord> {
        &self.records
    }

    /// Returns a stage record.
    #[must_use]
    pub fn record(&self, stage: &StageId) -> Option<&StageExecutionRecord> {
        self.records.get(stage)
    }

    /// Returns the required stages.
    #[must_use]
    pub fn required_stages(&self) -> &BTreeSet<StageId> {
        &self.required_stages
    }

    /// Returns the number of stages represented in the report.
    #[must_use]
    pub fn stage_count(&self) -> usize {
        self.records.len()
    }

    /// Returns the number of completed stages.
    #[must_use]
    pub fn completed_count(&self) -> usize {
        self.records
            .values()
            .filter(|record| record.is_completed())
            .count()
    }

    /// Returns the number of skipped stages.
    #[must_use]
    pub fn skipped_count(&self) -> usize {
        self.records
            .values()
            .filter(|record| {
                matches!(
                    record.status(),
                    StageExecutionStatus::Skipped { .. }
                )
            })
            .count()
    }

    /// Returns the number of blocked stages.
    #[must_use]
    pub fn blocked_count(&self) -> usize {
        self.records
            .values()
            .filter(|record| {
                matches!(
                    record.status(),
                    StageExecutionStatus::Blocked { .. }
                )
            })
            .count()
    }

    /// Returns the number of failed stages.
    #[must_use]
    pub fn failed_count(&self) -> usize {
        self.records
            .values()
            .filter(|record| {
                matches!(
                    record.status(),
                    StageExecutionStatus::Failed { .. }
                )
            })
            .count()
    }

    /// Returns whether every required stage completed successfully.
    ///
    /// A missing, skipped, blocked or failed stage never counts as complete.
    #[must_use]
    pub fn all_required_stages_completed(&self) -> bool {
        self.required_stages.iter().all(|stage| {
            self.records
                .get(stage)
                .is_some_and(StageExecutionRecord::is_completed)
        })
    }

    /// Returns whether the report contains any incomplete stage.
    #[must_use]
    pub fn has_incomplete_stages(&self) -> bool {
        self.records.values().any(|record| !record.is_completed())
    }

    /// Returns whether all required stages completed.
    ///
    /// This does not replace the lifecycle tracker's success state.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.all_required_stages_completed()
    }
}

// ============================================================================
// ORCHESTRATION ERROR CLASSIFICATION
// ============================================================================

/// Stable classification of orchestration failures.
///
/// The variants are deliberately aligned with the failure conversion
/// contract in pipeline::failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PipelineOrchestrationErrorKind {
    InvalidStageGraph,
    MissingRequiredStage,
    RequiredStageSkipped,
    RequiredStageBlocked,
    StageFailed,
    Cancelled,
    LifecycleFailure,
    IncompleteRequiredWork,
}

impl fmt::Display for PipelineOrchestrationErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidStageGraph => "invalid stage graph",
            Self::MissingRequiredStage => "missing required stage",
            Self::RequiredStageSkipped => "required stage skipped",
            Self::RequiredStageBlocked => "required stage blocked",
            Self::StageFailed => "stage failed",
            Self::Cancelled => "pipeline cancelled",
            Self::LifecycleFailure => "lifecycle failure",
            Self::IncompleteRequiredWork => "incomplete required work",
        })
    }
}

// ============================================================================
// ORCHESTRATION ERROR
// ============================================================================

/// A typed failure produced while coordinating a pipeline invocation.
///
/// The original StageError is retained whenever one exists. The partial
/// report is always available, including for failures before stage execution.
#[derive(Debug, Clone)]
pub struct PipelineOrchestrationError {
    kind: PipelineOrchestrationErrorKind,
    stage: Option<StageId>,
    message: Arc<str>,
    stage_error: Option<StageError>,
    partial_report: ExecutionReport,
    ordering_error: Option<OrderingError>,
}

impl PipelineOrchestrationError {
    fn new(
        kind: PipelineOrchestrationErrorKind,
        stage: Option<StageId>,
        message: impl Into<Arc<str>>,
        partial_report: ExecutionReport,
    ) -> Self {
        Self {
            kind,
            stage,
            message: message.into(),
            stage_error: None,
            partial_report,
            ordering_error: None,
        }
    }

    fn from_ordering(
        error: OrderingError,
        report: ExecutionReport,
    ) -> Self {
        Self {
            kind: PipelineOrchestrationErrorKind::InvalidStageGraph,
            stage: None,
            message: Arc::from(error.to_string()),
            stage_error: None,
            partial_report: report,
            ordering_error: Some(error),
        }
    }

    fn from_execution(
        error: &ExecutionError,
    ) -> Self {
        let kind = match error.kind() {
            ExecutionErrorKind::Cancelled => {
                PipelineOrchestrationErrorKind::Cancelled
            }

            ExecutionErrorKind::RequiredStageSkipped => {
                PipelineOrchestrationErrorKind::RequiredStageSkipped
            }

            ExecutionErrorKind::DependencyNotCompleted => {
                PipelineOrchestrationErrorKind::RequiredStageBlocked
            }

            ExecutionErrorKind::MissingRequiredStage => {
                PipelineOrchestrationErrorKind::MissingRequiredStage
            }

            ExecutionErrorKind::StageFailed => {
                PipelineOrchestrationErrorKind::StageFailed
            }

            ExecutionErrorKind::StateTracking => {
                PipelineOrchestrationErrorKind::LifecycleFailure
            }

            _ => PipelineOrchestrationErrorKind::InvalidStageGraph,
        };

        let mut report = ExecutionReport::from_execution(error.partial_report());

        if let Some(stage) = error.stage() {
            if let Some(cause) = error.stage_cause() {
                report.insert(StageExecutionRecord::failed(
                    stage.clone(),
                    cause.to_string(),
                ));
            }
        }

        Self {
            kind,
            stage: error.stage().cloned(),
            message: Arc::from(error.message()),
            stage_error: error.stage_cause().cloned(),
            partial_report: report,
            ordering_error: None,
        }
    }

    fn from_lifecycle(
        error: CompilationStateError,
        report: ExecutionReport,
    ) -> Self {
        let kind = match error {
            CompilationStateError::CancellationRequested => {
                PipelineOrchestrationErrorKind::Cancelled
            }

            _ => PipelineOrchestrationErrorKind::LifecycleFailure,
        };

        Self::new(kind, None, error.to_string(), report)
    }

    /// Returns the error classification.
    #[must_use]
    pub const fn kind(&self) -> PipelineOrchestrationErrorKind {
        self.kind
    }

    /// Returns the affected stage, if any.
    #[must_use]
    pub fn stage(&self) -> Option<&StageId> {
        self.stage.as_ref()
    }

    /// Returns the error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the original stage error, when available.
    #[must_use]
    pub fn stage_error(&self) -> Option<&StageError> {
        self.stage_error.as_ref()
    }

    /// Returns the partial orchestration report.
    #[must_use]
    pub fn partial_report(&self) -> &ExecutionReport {
        &self.partial_report
    }

    /// Returns the original graph-ordering error, when available.
    #[must_use]
    pub fn ordering_error(&self) -> Option<&OrderingError> {
        self.ordering_error.as_ref()
    }
}

impl fmt::Display for PipelineOrchestrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.stage.as_ref() {
            Some(stage) => write!(
                formatter,
                "pipeline orchestration failed ({}) at stage `{}`: {}",
                self.kind,
                stage,
                self.message,
            ),

            None => write!(
                formatter,
                "pipeline orchestration failed ({}): {}",
                self.kind,
                self.message,
            ),
        }
    }
}

impl Error for PipelineOrchestrationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(error) = self.stage_error.as_ref() {
            return Some(error);
        }

        self.ordering_error
            .as_ref()
            .map(|error| error as &(dyn Error + 'static))
    }
}

// ============================================================================
// PIPELINE ORCHESTRATOR
// ============================================================================

/// Coordinates one complete compiler-pipeline invocation.
///
/// The orchestrator is stateless. All per-compilation state is supplied
/// by the caller or created locally for the invocation.
///
/// It does not own a global stage registry, worker pool, backend or runtime.
#[derive(Debug, Default, Clone, Copy)]
pub struct PipelineOrchestrator;

impl PipelineOrchestrator {
    /// Creates a pipeline orchestrator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Executes a complete pipeline using the existing stage and executor
    /// contracts.
    ///
    /// The supplied stage list must contain exactly one implementation for
    /// every stage in the validated graph.
    ///
    /// Every required stage must be present and complete successfully.
    ///
    /// The CompilationContext is shared by all stages. The state tracker
    /// created here uses the context's existing cancellation token.
    ///
    /// No retries or fallbacks are performed by this method.
    pub fn execute(
        &self,
        compilation: &CompilationContext,
        stages: &[Arc<dyn Stage>],
        initial_input: StageInput,
        required_stages: &BTreeSet<StageId>,
    ) -> Result<ExecutionReport, PipelineOrchestrationError> {
        let mut report = ExecutionReport::new(required_stages.clone());

        // Build the canonical dependency graph from the registered
        // implementations. StageOrdering validates duplicates, missing
        // dependencies and cycles.
        let ordering = match StageOrdering::build(
            stages.iter().map(|stage| stage.descriptor()),
        ) {
            Ok(ordering) => ordering,

            Err(error) => {
                let orchestration_error =
                    PipelineOrchestrationError::from_ordering(
                        error,
                        report,
                    );

                return Err(orchestration_error);
            }
        };

        // A required stage must be registered. Do not let an empty required
        // set or an incomplete registry accidentally imply success.
        for required in required_stages {
            if !ordering.contains(required) {
                return Err(PipelineOrchestrationError::new(
                    PipelineOrchestrationErrorKind::MissingRequiredStage,
                    Some(required.clone()),
                    format!(
                        "required stage `{required}` is not registered"
                    ),
                    report,
                ));
            }
        }

        // Share the compilation context's cancellation authority.
        let state = CompilationStateTracker::with_cancellation(
            compilation.cancellation_token(),
        );

        if let Err(error) = state.start() {
            return Err(PipelineOrchestrationError::from_lifecycle(
                error,
                report,
            ));
        }

        let executor = PipelineExecutor::new();

        match executor.execute(
            compilation,
            &state,
            &ordering,
            stages,
            initial_input,
            required_stages,
        ) {
            Ok(execution_report) => {
                report = ExecutionReport::from_execution(&execution_report);

                // Enforce the mandatory-stage contract again at the
                // orchestration boundary. This is intentional defence
                // in depth; the executor remains the primary enforcer.
                if !report.all_required_stages_completed() {
                    let error = PipelineOrchestrationError::new(
                        PipelineOrchestrationErrorKind::IncompleteRequiredWork,
                        None,
                        "one or more required stages did not complete",
                        report,
                    );

                    self.fail_state(&state, &error);

                    return Err(error);
                }

                if let Err(error) = state.complete() {
                    let orchestration_error =
                        PipelineOrchestrationError::from_lifecycle(
                            error,
                            report,
                        );

                    self.fail_state(&state, &orchestration_error);

                    return Err(orchestration_error);
                }

                Ok(report)
            }

            Err(execution_error) => {
                let orchestration_error =
                    PipelineOrchestrationError::from_execution(
                        &execution_error,
                    );

                self.finish_failed_invocation(
                    compilation,
                    &state,
                    &orchestration_error,
                );

                Err(orchestration_error)
            }
        }
    }

    /// Marks a non-cancellation failure in the shared lifecycle tracker.
    fn fail_state(
        &self,
        state: &CompilationStateTracker,
        error: &PipelineOrchestrationError,
    ) {
        if error.kind() == PipelineOrchestrationErrorKind::Cancelled {
            let _ = self.mark_cancelled(state);
        } else {
            let _ = state.fail(error.to_string());
        }
    }

    /// Completes the appropriate terminal transition after executor failure.
    fn finish_failed_invocation(
        &self,
        compilation: &CompilationContext,
        state: &CompilationStateTracker,
        error: &PipelineOrchestrationError,
    ) {
        if error.kind() == PipelineOrchestrationErrorKind::Cancelled
            || compilation.is_cancelled()
            || state.is_cancelled()
        {
            let _ = self.mark_cancelled(state);
        } else {
            let _ = state.fail(error.to_string());
        }
    }

    /// Requests and acknowledges cancellation using the existing tracker.
    ///
    /// The state tracker and CompilationContext share the same cancellation
    /// token. This method does not introduce another cancellation mechanism.
    fn mark_cancelled(
        &self,
        state: &CompilationStateTracker,
    ) -> Result<(), CompilationStateError> {
        match state.lifecycle()? {
            CompilationLifecycle::Created
            | CompilationLifecycle::Running => {
                state.request_cancel()?;
            }

            CompilationLifecycle::CancellationRequested => {}

            CompilationLifecycle::Cancelled => return Ok(()),

            CompilationLifecycle::Completed
            | CompilationLifecycle::Failed => {
                return Err(CompilationStateError::TerminalState {
                    state: state.lifecycle()?,
                });
            }
        }

        if state.lifecycle()? == CompilationLifecycle::CancellationRequested {
            state.mark_cancelled()?;
        }

        Ok(())
    }
}

// ============================================================================
// REGRESSION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::configuration::budget::CompilationBudget;
    use crate::compiler::context::compilation_context::CompilationContextBuilder;
    use crate::compiler::pipeline::stage::{
        StageDescriptor,
        StageError,
        StageErrorKind,
        StageExecution,
        StageOutput,
    };
    use crate::compiler_types::CompilerConfig;

    struct TestStage {
        descriptor: StageDescriptor,
        output: &'static str,
        should_fail: bool,
        should_skip: bool,
    }

    impl TestStage {
        fn new(
            id: &str,
            dependencies: Vec<StageId>,
            output: &'static str,
        ) -> Self {
            let descriptor = StageDescriptor::new(
                StageId::new(id).expect("valid stage identifier"),
                id,
                "1",
                "orchestration test stage",
            )
            .expect("valid descriptor")
            .with_dependencies(dependencies)
            .expect("valid dependencies");

            Self {
                descriptor,
                output,
                should_fail: false,
                should_skip: false,
            }
        }

        fn failing(mut self) -> Self {
            self.should_fail = true;
            self
        }

        fn skipping(mut self) -> Self {
            self.should_skip = true;
            self
        }
    }

    impl Stage for TestStage {
        fn descriptor(&self) -> &StageDescriptor {
            &self.descriptor
        }

        fn execute(
            &self,
            execution: &StageExecution<'_>,
        ) -> Result<StageOutput, StageError> {
            execution.context().check_cancelled()?;

            if self.should_fail {
                return Err(StageError::new(
                    StageErrorKind::Execution,
                    "test stage failed",
                ));
            }

            if self.should_skip {
                // A stage's skip decision belongs in should_execute.
                // This branch is deliberately unreachable in this test
                // helper; the default decision is Run.
            }

            Ok(StageOutput::new(self.output.to_owned()))
        }

        fn should_execute(
            &self,
            _context: &crate::compiler::pipeline::stage::StageExecutionContext<'_>,
            _input: &crate::compiler::pipeline::stage::StageInput,
        ) -> Result<
            crate::compiler::pipeline::stage::StageDecision,
            StageError,
        > {
            if self.should_skip {
                crate::compiler::pipeline::stage::StageDecision::skip(
                    "not applicable to this test",
                )
            } else {
                Ok(crate::compiler::pipeline::stage::StageDecision::Run)
            }
        }
    }

    fn context() -> CompilationContext {
        CompilationContextBuilder::new(CompilerConfig::default())
            .budget(CompilationBudget::unlimited())
            .build()
            .expect("valid compilation context")
    }

    fn id(value: &str) -> StageId {
        StageId::new(value).expect("valid stage ID")
    }

    fn required(values: &[&str]) -> BTreeSet<StageId> {
        values.iter().map(|value| id(value)).collect()
    }

    fn stage(value: TestStage) -> Arc<dyn Stage> {
        Arc::new(value)
    }

    #[test]
    fn api_version_is_stable() {
        assert_eq!(PIPELINE_ORCHESTRATION_API_VERSION, 1);
    }

    #[test]
    fn empty_pipeline_can_complete_when_no_stages_are_required() {
        let result = PipelineOrchestrator::new().execute(
            &context(),
            &[],
            StageInput::empty(),
            &BTreeSet::new(),
        );

        assert!(result.is_ok());
        assert!(result.expect("successful report").is_successful());
    }

    #[test]
    fn required_stage_must_be_registered() {
        let result = PipelineOrchestrator::new().execute(
            &context(),
            &[],
            StageInput::empty(),
            &required(&["frontend"]),
        );

        let error = result.expect_err("missing stage must fail");

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::MissingRequiredStage
        );
    }

    #[test]
    fn stages_execute_in_dependency_order() {
        let stages = vec![
            stage(TestStage::new("backend", vec![id("ir")], "binary")),
            stage(TestStage::new("ir", vec![id("frontend")], "ir")),
            stage(TestStage::new("frontend", vec![], "ast")),
        ];

        let result = PipelineOrchestrator::new().execute(
            &context(),
            &stages,
            StageInput::empty(),
            &required(&["frontend", "ir", "backend"]),
        );

        let report = result.expect("pipeline should succeed");

        assert_eq!(report.completed_count(), 3);
        assert!(report.all_required_stages_completed());
    }

    #[test]
    fn stage_failure_preserves_partial_report_and_typed_cause() {
        let stages = vec![
            stage(TestStage::new("frontend", vec![], "ast")),
            stage(
                TestStage::new("semantic", vec![id("frontend")], "typed")
                    .failing(),
            ),
        ];

        let result = PipelineOrchestrator::new().execute(
            &context(),
            &stages,
            StageInput::empty(),
            &required(&["frontend", "semantic"]),
        );

        let error = result.expect_err("stage failure must propagate");

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::StageFailed
        );

        assert_eq!(error.stage(), Some(&id("semantic")));
        assert!(error.stage_error().is_some());
        assert_eq!(error.partial_report().completed_count(), 1);
        assert_eq!(error.partial_report().failed_count(), 1);
    }

    #[test]
    fn required_stage_skip_is_not_success() {
        let stages = vec![
            stage(TestStage::new("frontend", vec![], "ast").skipping()),
        ];

        let result = PipelineOrchestrator::new().execute(
            &context(),
            &stages,
            StageInput::empty(),
            &required(&["frontend"]),
        );

        let error = result.expect_err("required skip must fail");

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::RequiredStageSkipped
        );

        assert_eq!(error.partial_report().skipped_count(), 1);
        assert!(!error.partial_report().all_required_stages_completed());
    }

    #[test]
    fn skipped_prerequisite_blocks_dependent_stage() {
        let stages = vec![
            stage(TestStage::new("frontend", vec![], "ast").skipping()),
            stage(TestStage::new("semantic", vec![id("frontend")], "typed")),
        ];

        let result = PipelineOrchestrator::new().execute(
            &context(),
            &stages,
            StageInput::empty(),
            &required(&["semantic"]),
        );

        let error = result.expect_err("dependent required work must fail");

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::RequiredStageBlocked
        );

        assert!(!error.partial_report().all_required_stages_completed());
    }

    #[test]
    fn duplicate_stage_identifiers_are_rejected() {
        let stages = vec![
            stage(TestStage::new("frontend", vec![], "ast")),
            stage(TestStage::new("frontend", vec![], "another ast")),
        ];

        let result = PipelineOrchestrator::new().execute(
            &context(),
            &stages,
            StageInput::empty(),
            &required(&["frontend"]),
        );

        let error = result.expect_err("duplicate stages must fail");

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::InvalidStageGraph
        );
    }

    #[test]
    fn dependency_cycles_are_rejected() {
        let stages = vec![
            stage(TestStage::new("a", vec![id("b")], "a")),
            stage(TestStage::new("b", vec![id("a")], "b")),
        ];

        let result = PipelineOrchestrator::new().execute(
            &context(),
            &stages,
            StageInput::empty(),
            &required(&["a"]),
        );

        let error = result.expect_err("cycle must fail");

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::InvalidStageGraph
        );
    }

    #[test]
    fn no_required_stage_is_inferred_from_an_empty_registry() {
        let result = PipelineOrchestrator::new().execute(
            &context(),
            &[],
            StageInput::empty(),
            &BTreeSet::new(),
        );

        let report = result.expect("empty optional pipeline is valid");

        assert!(report.all_required_stages_completed());
    }

    #[test]
    fn failure_report_conversion_keeps_required_stage_contract() {
        let report = ExecutionReport::new(required(&["backend"]));

        assert!(!report.all_required_stages_completed());
        assert!(!report.is_successful());
    }
}
