#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Pipeline Orchestration.
//!
//! Path: src/compiler/pipeline/orchestration.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Coordinates one complete compiler-pipeline invocation.
//!
//! This module owns:
//! - PipelineOrchestrator.
//! - Orchestration-level execution reports.
//! - Orchestration-level stage records.
//! - Orchestration error classification.
//! - Compilation lifecycle transitions around execution.
//! - Conversion of executor reports and errors.
//!
//! It delegates:
//! - Stage contracts to pipeline::stage.
//! - Dependency validation to pipeline::ordering.
//! - Stage execution to pipeline::execution.
//! - Failure normalization to pipeline::failure.
//! - Recovery policy to pipeline::recovery.
//! - Budgets and cancellation to CompilationContext.
//! - Lifecycle transitions to CompilationStateTracker.
//!
//! # Architectural boundaries
//!
//! This module MUST NOT implement:
//! - Source parsing or grammar interpretation.
//! - Semantic analysis or type inference.
//! - A competing AST or semantic model.
//! - A competing universal or domain IR.
//! - Optimization algorithms.
//! - Physical resource allocation.
//! - Hardware discovery.
//! - Physical scheduling or routing.
//! - Backend code generation.
//! - Linking implementation.
//! - Runtime execution.
//! - Automatic retries or fallbacks.
//!
//! # POCO-REAF
//!
//! No arbitrary universal limits are imposed on:
//! - Pipeline stages.
//! - Stage dependencies.
//! - Compilation units.
//! - Computing domains.
//! - CPUs, GPUs, FPGAs or QPUs.
//! - Distributed participants.
//! - Memory or storage.
//! - Qubits.
//! - Tensor dimensions or rank.
//! - IR operations.
//!
//! Operational budgets are supplied through CompilationContext.
//!
//! Physical limitations belong to the relevant target, provider,
//! resource-management and runtime authorities.
//!
//! No finite implementation can promise infinite physical resources.
//!
//! # Determinism
//!
//! StageOrdering owns deterministic dependency ordering.
//! PipelineExecutor owns deterministic sequential execution.
//! This module does not introduce another ordering or scheduling policy.
//!
//! # Conditional stages
//!
//! A skipped stage is not completed.
//! A dependent stage cannot consume a skipped prerequisite's output.
//! Required stages must complete successfully.
//!
//! # Grammar boundary
//!
//! Source-level compilation intent is defined by:
//! - grammar/Zamani.g4
//! - grammar/compile/compile.g4
//! - grammar/specification/compilation-model.md
//! - grammar/specification/execution-model.md
//! - grammar/specification/poco-reaf.md
//!
//! The frontend semantic authority determines source meaning.
//! This module coordinates compiler implementation stages only.

use std::collections::{BTreeMap, BTreeSet};
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
///
/// Independent of the Zamani language, grammar, semantic-model,
/// IR, backend and runtime versions.
pub const PIPELINE_ORCHESTRATION_API_VERSION: u16 = 1;

// ============================================================================
// STAGE STATUS
// ============================================================================

/// Final orchestration-level status of one pipeline stage.
///
/// This is a reporting representation. The execution subsystem
/// remains authoritative for stage invocation and output ownership.
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
// STAGE RECORD
// ============================================================================

/// Immutable orchestration record for one stage.
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

    /// Returns the stage status.
    #[must_use]
    pub fn status(&self) -> &StageExecutionStatus {
        &self.status
    }

    /// Returns whether the stage completed successfully.
    #[must_use]
    pub fn is_completed(&self) -> bool {
        matches!(
            self.status,
            StageExecutionStatus::Completed
        )
    }
}

// ============================================================================
// EXECUTION REPORT
// ============================================================================

/// Immutable summary of one pipeline invocation.
///
/// This report retains orchestration records and the mandatory-stage
/// contract. Actual stage outputs remain owned by PipelineExecutionReport.
///
/// Stage outputs are deliberately not copied into this report.
#[derive(Debug, Clone, Default)]
pub struct ExecutionReport {
    records: BTreeMap<StageId, StageExecutionRecord>,
    required_stages: BTreeSet<StageId>,
}

impl ExecutionReport {
    /// Creates an empty report with the supplied required-stage contract.
    #[must_use]
    pub fn new(required_stages: BTreeSet<StageId>) -> Self {
        Self {
            records: BTreeMap::new(),
            required_stages,
        }
    }

    /// Converts an executor report while preserving stage outcomes.
    ///
    /// In particular, the actual blocking prerequisite is retained.
    #[must_use]
    pub fn from_execution(
        report: &PipelineExecutionReport,
    ) -> Self {
        let mut result = Self::new(
            report.required_stages().clone(),
        );

        for (stage_id, record) in report.records() {
            let converted = match record.status() {
                PipelineStageExecutionStatus::Completed => {
                    StageExecutionRecord::completed(
                        stage_id.clone(),
                    )
                }

                PipelineStageExecutionStatus::Skipped => {
                    StageExecutionRecord::skipped(
                        stage_id.clone(),
                        record.reason().unwrap_or(
                            "stage was skipped",
                        ),
                    )
                }

                PipelineStageExecutionStatus::Blocked => {
                    // execution.rs records the exact prerequisite.
                    //
                    // The fallback is defensive against an invalid
                    // executor report. It does not claim that the stage
                    // actually depends on itself.
                    let dependency = record
                        .blocked_by()
                        .cloned()
                        .unwrap_or_else(|| stage_id.clone());

                    let reason = match record.blocked_by() {
                        Some(_) => record.reason().unwrap_or(
                            "a prerequisite did not complete",
                        ),

                        None => {
                            "executor reported a blocked stage \
                             without identifying its prerequisite"
                        }
                    };

                    StageExecutionRecord::blocked(
                        stage_id.clone(),
                        dependency,
                        reason,
                    )
                }
            };

            result.records.insert(
                stage_id.clone(),
                converted,
            );
        }

        result
    }

    /// Adds or replaces a stage record.
    fn insert(&mut self, record: StageExecutionRecord) {
        self.records.insert(
            record.stage_id().clone(),
            record,
        );
    }

    /// Returns all records in deterministic identifier order.
    #[must_use]
    pub fn records(
        &self,
    ) -> &BTreeMap<StageId, StageExecutionRecord> {
        &self.records
    }

    /// Returns one stage record.
    #[must_use]
    pub fn record(
        &self,
        stage: &StageId,
    ) -> Option<&StageExecutionRecord> {
        self.records.get(stage)
    }

    /// Returns the required-stage contract.
    #[must_use]
    pub fn required_stages(&self) -> &BTreeSet<StageId> {
        &self.required_stages
    }

    /// Returns the number of stages represented.
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
    /// A missing, skipped, blocked or failed stage is incomplete.
    #[must_use]
    pub fn all_required_stages_completed(&self) -> bool {
        self.required_stages.iter().all(|stage| {
            self.records
                .get(stage)
                .is_some_and(StageExecutionRecord::is_completed)
        })
    }

    /// Returns whether any recorded stage did not complete.
    #[must_use]
    pub fn has_incomplete_stages(&self) -> bool {
        self.records
            .values()
            .any(|record| !record.is_completed())
    }

    /// Returns whether the mandatory-stage contract is satisfied.
    ///
    /// This is not a substitute for successful lifecycle completion.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.all_required_stages_completed()
    }
}

// ============================================================================
// ERROR CLASSIFICATION
// ============================================================================

/// Stable classification of orchestration failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PipelineOrchestrationErrorKind {
    /// The stage dependency graph is invalid.
    InvalidStageGraph,

    /// A required stage is not registered.
    MissingRequiredStage,

    /// A required stage was skipped.
    RequiredStageSkipped,

    /// A required stage was blocked.
    RequiredStageBlocked,

    /// A stage failed.
    StageFailed,

    /// Compilation was cancelled.
    Cancelled,

    /// Lifecycle tracking failed.
    LifecycleFailure,

    /// Required work was not completed.
    IncompleteRequiredWork,

    /// An internal orchestration or executor invariant failed.
    InternalInvariant,
}

impl fmt::Display for PipelineOrchestrationErrorKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidStageGraph => "invalid stage graph",
            Self::MissingRequiredStage => "missing required stage",
            Self::RequiredStageSkipped => "required stage skipped",
            Self::RequiredStageBlocked => "required stage blocked",
            Self::StageFailed => "stage failed",
            Self::Cancelled => "pipeline cancelled",
            Self::LifecycleFailure => "lifecycle failure",
            Self::IncompleteRequiredWork => "incomplete required work",
            Self::InternalInvariant => "internal invariant failure",
        })
    }
}

// ============================================================================
// ORCHESTRATION ERROR
// ============================================================================

/// Structured failure produced by pipeline orchestration.
///
/// Partial stage records are retained for failures occurring before,
/// during or after stage execution.
///
/// Original stage, graph and lifecycle errors are retained where available.
#[derive(Debug, Clone)]
pub struct PipelineOrchestrationError {
    kind: PipelineOrchestrationErrorKind,
    stage: Option<StageId>,
    message: Arc<str>,
    stage_error: Option<StageError>,
    ordering_error: Option<OrderingError>,
    lifecycle_error: Option<CompilationStateError>,
    partial_report: ExecutionReport,
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
            ordering_error: None,
            lifecycle_error: None,
            partial_report,
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
            ordering_error: Some(error),
            lifecycle_error: None,
            partial_report: report,
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

            ExecutionErrorKind::MissingRequiredStage
            | ExecutionErrorKind::MissingStageImplementation => {
                PipelineOrchestrationErrorKind::MissingRequiredStage
            }

            ExecutionErrorKind::StageFailed => {
                PipelineOrchestrationErrorKind::StageFailed
            }

            ExecutionErrorKind::StateTracking => {
                PipelineOrchestrationErrorKind::LifecycleFailure
            }

            ExecutionErrorKind::InternalInvariant => {
                PipelineOrchestrationErrorKind::InternalInvariant
            }

            ExecutionErrorKind::DuplicateStageImplementation
            | ExecutionErrorKind::UnexpectedStageImplementation
            | ExecutionErrorKind::OrderingMismatch => {
                PipelineOrchestrationErrorKind::InvalidStageGraph
            }
        };

        let mut report = ExecutionReport::from_execution(
            error.partial_report(),
        );

        if let Some(stage) = error.stage() {
            if let Some(cause) = error.stage_cause() {
                report.insert(
                    StageExecutionRecord::failed(
                        stage.clone(),
                        cause.to_string(),
                    ),
                );
            } else if kind == PipelineOrchestrationErrorKind::InternalInvariant {
                report.insert(
                    StageExecutionRecord::failed(
                        stage.clone(),
                        error.message(),
                    ),
                );
            }
        }

        Self {
            kind,
            stage: error.stage().cloned(),
            message: Arc::from(error.message()),
            stage_error: error.stage_cause().cloned(),
            ordering_error: None,
            lifecycle_error: None,
            partial_report: report,
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

        Self {
            kind,
            stage: None,
            message: Arc::from(error.to_string()),
            stage_error: None,
            ordering_error: None,
            lifecycle_error: Some(error),
            partial_report: report,
        }
    }

    fn with_lifecycle_error(
        mut self,
        error: CompilationStateError,
    ) -> Self {
        self.message = Arc::from(format!(
            "{}; additionally, lifecycle finalization failed: {}",
            self.message,
            error,
        ));

        self.lifecycle_error = Some(error);
        self
    }

    /// Returns the failure classification.
    #[must_use]
    pub const fn kind(&self) -> PipelineOrchestrationErrorKind {
        self.kind
    }

    /// Returns the affected stage, if any.
    #[must_use]
    pub fn stage(&self) -> Option<&StageId> {
        self.stage.as_ref()
    }

    /// Returns the failure message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the original stage error, if any.
    #[must_use]
    pub fn stage_error(&self) -> Option<&StageError> {
        self.stage_error.as_ref()
    }

    /// Returns the original graph-ordering error, if any.
    #[must_use]
    pub fn ordering_error(&self) -> Option<&OrderingError> {
        self.ordering_error.as_ref()
    }

    /// Returns the lifecycle error, if any.
    #[must_use]
    pub fn lifecycle_error(&self) -> Option<&CompilationStateError> {
        self.lifecycle_error.as_ref()
    }

    /// Returns the partial orchestration report.
    #[must_use]
    pub fn partial_report(&self) -> &ExecutionReport {
        &self.partial_report
    }
}

impl fmt::Display for PipelineOrchestrationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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

        if let Some(error) = self.ordering_error.as_ref() {
            return Some(error);
        }

        self.lifecycle_error
            .as_ref()
            .map(|error| error as &(dyn Error + 'static))
    }
}

// ============================================================================
// PIPELINE ORCHESTRATOR
// ============================================================================

/// Coordinates one complete compiler-pipeline invocation.
///
/// The orchestrator is stateless. Each invocation receives its own
/// compilation context, stage implementations, input and required-stage
/// contract.
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

    /// Executes one validated compiler pipeline.
    ///
    /// The stage graph is validated before any stage is invoked.
    ///
    /// Every required stage must be registered and complete successfully.
    ///
    /// The state tracker shares the CompilationContext cancellation token.
    ///
    /// This method never retries a stage or substitutes a fallback.
    ///
    /// An empty pipeline with no required stages is permitted by this
    /// generic orchestration API. A production compiler driver must supply
    /// the required stages for the selected compilation route.
    pub fn execute(
        &self,
        compilation: &CompilationContext,
        stages: &[Arc<dyn Stage>],
        initial_input: StageInput,
        required_stages: &BTreeSet<StageId>,
    ) -> Result<ExecutionReport, PipelineOrchestrationError> {
        let mut report = ExecutionReport::new(
            required_stages.clone(),
        );

        // Create the lifecycle before preflight so that graph-validation
        // failures and missing required stages follow the same terminal
        // failure path as execution failures.
        let state = CompilationStateTracker::with_cancellation(
            compilation.cancellation_token(),
        );

        if let Err(error) = state.start() {
            return Err(
                PipelineOrchestrationError::from_lifecycle(
                    error,
                    report,
                ),
            );
        }

        // Do not execute any stage until the entire dependency graph
        // and required-stage contract have been validated.
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

                return Err(
                    self.finalize_failure(
                        compilation,
                        &state,
                        orchestration_error,
                    ),
                );
            }
        };

        // Validate that every required stage is present before execution.
        for required in required_stages {
            if !ordering.contains(required) {
                let error = PipelineOrchestrationError::new(
                    PipelineOrchestrationErrorKind::MissingRequiredStage,
                    Some(required.clone()),
                    format!(
                        "required stage `{required}` is not registered"
                    ),
                    report,
                );

                return Err(
                    self.finalize_failure(
                        compilation,
                        &state,
                        error,
                    ),
                );
            }
        }

        // Cancellation may have been requested during preflight.
        if let Err(error) = compilation.check_cancelled() {
            let orchestration_error =
                PipelineOrchestrationError::new(
                    PipelineOrchestrationErrorKind::Cancelled,
                    None,
                    error.to_string(),
                    report,
                );

            return Err(
                self.finalize_failure(
                    compilation,
                    &state,
                    orchestration_error,
                ),
            );
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
                report = ExecutionReport::from_execution(
                    &execution_report,
                );

                // Defence in depth: the executor enforces this contract,
                // and orchestration verifies it before lifecycle success.
                if !report.all_required_stages_completed() {
                    let error = PipelineOrchestrationError::new(
                        PipelineOrchestrationErrorKind::IncompleteRequiredWork,
                        None,
                        "one or more required stages did not complete",
                        report,
                    );

                    return Err(
                        self.finalize_failure(
                            compilation,
                            &state,
                            error,
                        ),
                    );
                }

                // Do not publish a successful lifecycle if cancellation
                // arrived after the executor's final stage check.
                if let Err(error) = compilation.check_cancelled() {
                    let orchestration_error =
                        PipelineOrchestrationError::new(
                            PipelineOrchestrationErrorKind::Cancelled,
                            None,
                            error.to_string(),
                            report,
                        );

                    return Err(
                        self.finalize_failure(
                            compilation,
                            &state,
                            orchestration_error,
                        ),
                    );
                }

                match state.complete() {
                    Ok(()) => Ok(report),

                    Err(error) => {
                        let orchestration_error =
                            PipelineOrchestrationError::from_lifecycle(
                                error,
                                report,
                            );

                        Err(
                            self.finalize_failure(
                                compilation,
                                &state,
                                orchestration_error,
                            ),
                        )
                    }
                }
            }

            Err(execution_error) => {
                let orchestration_error =
                    PipelineOrchestrationError::from_execution(
                        &execution_error,
                    );

                Err(
                    self.finalize_failure(
                        compilation,
                        &state,
                        orchestration_error,
                    ),
                )
            }
        }
    }

    /// Finalizes the lifecycle after an orchestration failure.
    ///
    /// Lifecycle-finalization errors are retained rather than discarded.
    fn finalize_failure(
        &self,
        compilation: &CompilationContext,
        state: &CompilationStateTracker,
        error: PipelineOrchestrationError,
    ) -> PipelineOrchestrationError {
        let cancellation_requested =
            error.kind() == PipelineOrchestrationErrorKind::Cancelled
                || compilation.is_cancelled()
                || state.is_cancelled();

        let transition = if cancellation_requested {
            self.mark_cancelled(state)
        } else {
            state.fail(error.to_string())
        };

        match transition {
            Ok(()) => error,

            Err(lifecycle_error) => {
                // A terminal lifecycle state must not be overwritten.
                // Preserve the original failure and the transition error.
                error.with_lifecycle_error(lifecycle_error)
            }
        }
    }

    /// Synchronizes cancellation with the lifecycle tracker.
    ///
    /// This uses the context's existing cancellation authority.
    /// No independent cancellation token is created.
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

            CompilationLifecycle::Cancelled => {
                return Ok(());
            }

            CompilationLifecycle::Completed
            | CompilationLifecycle::Failed => {
                return Err(
                    CompilationStateError::TerminalState {
                        state: state.lifecycle()?,
                    },
                );
            }
        }

        if state.lifecycle()?
            == CompilationLifecycle::CancellationRequested
        {
            state.mark_cancelled()?;
        }

        Ok(())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::configuration::budget::CompilationBudget;

    use crate::compiler::context::compilation_context::{
        CompilationContextBuilder,
    };

    use crate::compiler::pipeline::stage::{
        StageDecision,
        StageDescriptor,
        StageError,
        StageErrorKind,
        StageExecution,
        StageExecutionContext,
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
                StageId::new(id).expect("valid stage ID"),
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

        fn should_execute(
            &self,
            _context: &StageExecutionContext<'_>,
            _input: &StageInput,
        ) -> Result<StageDecision, StageError> {
            if self.should_skip {
                StageDecision::skip(
                    "not applicable to this test",
                )
            } else {
                Ok(StageDecision::Run)
            }
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

            Ok(StageOutput::new(
                self.output.to_owned(),
            ))
        }
    }

    fn context() -> CompilationContext {
        CompilationContextBuilder::new(
            CompilerConfig::default(),
        )
        .budget(CompilationBudget::unlimited())
        .build()
        .expect("valid compilation context")
    }

    fn id(value: &str) -> StageId {
        StageId::new(value).expect("valid stage ID")
    }

    fn required(values: &[&str]) -> BTreeSet<StageId> {
        values
            .iter()
            .map(|value| id(value))
            .collect()
    }

    fn stage(value: TestStage) -> Arc<dyn Stage> {
        Arc::new(value)
    }

    #[test]
    fn api_version_is_stable() {
        assert_eq!(
            PIPELINE_ORCHESTRATION_API_VERSION,
            1,
        );
    }

    #[test]
    fn empty_optional_pipeline_is_permitted() {
        let result = PipelineOrchestrator::new().execute(
            &context(),
            &[],
            StageInput::empty(),
            &BTreeSet::new(),
        );

        let report = result.expect(
            "empty optional pipeline should succeed",
        );

        assert!(report.is_successful());
        assert_eq!(report.stage_count(), 0);
    }

    #[test]
    fn required_stage_must_be_registered() {
        let result = PipelineOrchestrator::new().execute(
            &context(),
            &[],
            StageInput::empty(),
            &required(&["frontend"]),
        );

        let error = result.expect_err(
            "missing required stage must fail",
        );

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::MissingRequiredStage,
        );
    }

    #[test]
    fn stages_execute_in_dependency_order() {
        let stages = vec![
            stage(TestStage::new(
                "backend",
                vec![id("ir")],
                "binary",
            )),
            stage(TestStage::new(
                "ir",
                vec![id("frontend")],
                "ir",
            )),
            stage(TestStage::new(
                "frontend",
                vec![],
                "ast",
            )),
        ];

        let report = PipelineOrchestrator::new()
            .execute(
                &context(),
                &stages,
                StageInput::empty(),
                &required(&[
                    "frontend",
                    "ir",
                    "backend",
                ]),
            )
            .expect("pipeline should succeed");

        assert_eq!(report.completed_count(), 3);
        assert!(report.all_required_stages_completed());
    }

    #[test]
    fn stage_failure_preserves_partial_report_and_cause() {
        let stages = vec![
            stage(TestStage::new(
                "frontend",
                vec![],
                "ast",
            )),
            stage(
                TestStage::new(
                    "semantic",
                    vec![id("frontend")],
                    "typed",
                )
                .failing(),
            ),
        ];

        let error = PipelineOrchestrator::new()
            .execute(
                &context(),
                &stages,
                StageInput::empty(),
                &required(&[
                    "frontend",
                    "semantic",
                ]),
            )
            .expect_err("stage failure must propagate");

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::StageFailed,
        );

        assert_eq!(error.stage(), Some(&id("semantic")));
        assert!(error.stage_error().is_some());
        assert_eq!(error.partial_report().completed_count(), 1);
        assert_eq!(error.partial_report().failed_count(), 1);
    }

    #[test]
    fn required_stage_skip_is_not_success() {
        let stages = vec![
            stage(
                TestStage::new(
                    "frontend",
                    vec![],
                    "ast",
                )
                .skipping(),
            ),
        ];

        let error = PipelineOrchestrator::new()
            .execute(
                &context(),
                &stages,
                StageInput::empty(),
                &required(&["frontend"]),
            )
            .expect_err("required stage cannot be skipped");

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::RequiredStageSkipped,
        );

        assert_eq!(error.partial_report().skipped_count(), 1);
        assert!(!error.partial_report().is_successful());
    }

    #[test]
    fn blocked_stage_preserves_actual_prerequisite() {
        let stages = vec![
            stage(
                TestStage::new(
                    "frontend",
                    vec![],
                    "ast",
                )
                .skipping(),
            ),
            stage(TestStage::new(
                "semantic",
                vec![id("frontend")],
                "typed",
            )),
        ];

        let error = PipelineOrchestrator::new()
            .execute(
                &context(),
                &stages,
                StageInput::empty(),
                &required(&["semantic"]),
            )
            .expect_err("dependent required stage must fail");

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::RequiredStageBlocked,
        );

        let record = error
            .partial_report()
            .record(&id("semantic"))
            .expect("blocked stage record");

        match record.status() {
            StageExecutionStatus::Blocked {
                dependency,
                ..
            } => {
                assert_eq!(dependency, &id("frontend"));
            }

            other => {
                panic!("expected blocked status, got {other:?}");
            }
        }
    }

    #[test]
    fn duplicate_stage_identifiers_are_rejected() {
        let stages = vec![
            stage(TestStage::new(
                "frontend",
                vec![],
                "ast",
            )),
            stage(TestStage::new(
                "frontend",
                vec![],
                "another ast",
            )),
        ];

        let error = PipelineOrchestrator::new()
            .execute(
                &context(),
                &stages,
                StageInput::empty(),
                &required(&["frontend"]),
            )
            .expect_err("duplicate stages must fail");

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::InvalidStageGraph,
        );
    }

    #[test]
    fn dependency_cycles_are_rejected() {
        let stages = vec![
            stage(TestStage::new(
                "a",
                vec![id("b")],
                "a",
            )),
            stage(TestStage::new(
                "b",
                vec![id("a")],
                "b",
            )),
        ];

        let error = PipelineOrchestrator::new()
            .execute(
                &context(),
                &stages,
                StageInput::empty(),
                &required(&["a"]),
            )
            .expect_err("dependency cycle must fail");

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::InvalidStageGraph,
        );
    }

    #[test]
    fn cancelled_compilation_does_not_publish_success() {
        let compilation = context();

        compilation.cancellation_token().cancel();

        let result = PipelineOrchestrator::new().execute(
            &compilation,
            &[],
            StageInput::empty(),
            &BTreeSet::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn report_keeps_required_stage_contract() {
        let report = ExecutionReport::new(
            required(&["backend"]),
        );

        assert!(!report.all_required_stages_completed());
        assert!(!report.is_successful());
    }
}