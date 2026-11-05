
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Pipeline Execution.
//!
//! Path:
//!     src/compiler/pipeline/execution.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Executes an already-registered and dependency-ordered collection of
//! compiler stages.
//!
//! This module consumes the existing:
//!
//!     pipeline::stage
//!         Stage
//!         StageId
//!         StageDescriptor
//!         StageInput
//!         StageOutput
//!         StageOutcome
//!         StageError
//!         StageErrorKind
//!
//!     pipeline::ordering
//!         StageOrdering
//!
//!     context::compilation_context
//!         CompilationContext
//!
//!     context::compilation_state
//!         CompilationStateTracker
//!         CompilationPhase
//!
//! The executor does not create a second stage system.
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - PipelineExecutor.
//! - StageInputs.
//! - StageExecutionStatus.
//! - StageExecutionRecord.
//! - ExecutionReport.
//! - ExecutionError.
//! - ExecutionErrorKind.
//! - Deterministic sequential stage execution.
//! - Dependency-output delivery.
//! - Required-stage enforcement.
//!
//! This module does NOT own:
//!
//! - Stage registration.
//! - Dependency graph construction.
//! - Stage ordering algorithms.
//! - Pipeline orchestration.
//! - Compilation lifecycle initialization.
//! - Compilation lifecycle completion.
//! - Compilation lifecycle failure transitions.
//! - Automatic retries.
//! - Recovery or fallback policy.
//! - Compiler diagnostics.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership analysis.
//! - Capability semantics.
//! - Resource allocation.
//! - Hardware discovery.
//! - Hardware scheduling.
//! - Physical routing.
//! - Canonical IR.
//! - IR transformations.
//! - Backend implementations.
//! - Runtime execution.
//! - Distributed execution.
//!
//! ============================================================================
//! EXECUTION INVARIANTS
//! ============================================================================
//!
//! 1. Every registered stage must occur in StageOrdering.
//! 2. Every ordered stage must have exactly one implementation.
//! 3. Stage descriptors must agree with the validated ordering.
//! 4. A stage executes only after all direct dependencies complete.
//! 5. A skipped stage is never recorded as completed.
//! 6. A blocked stage is never recorded as completed.
//! 7. A required stage must complete successfully.
//! 8. A dependent stage cannot consume output from an incomplete prerequisite.
//! 9. Cancellation prevents successful execution.
//! 10. Stage failures are never silently converted into success.
//! 11. The executor never retries a failed stage.
//! 12. The executor never changes source semantics.
//! 13. The executor never allocates physical hardware.
//! 14. The executor never silently truncates work.
//!
//! ============================================================================
//! CONDITIONAL EXECUTION
//! ============================================================================
//!
//! A stage may return StageOutcome::Skipped.
//!
//! A skipped stage:
//!
//! - is recorded as Skipped;
//! - does not produce an output;
//! - cannot satisfy another stage's dependency;
//! - does not automatically fail an unrelated branch.
//!
//! If a required stage is skipped, execution fails.
//!
//! If a required stage depends on a skipped or blocked stage, execution
//! fails because its prerequisite did not complete.
//!
//! The orchestrator must supply the complete set of required stages.
//! This is necessary because StageDescriptor deliberately does not own
//! compilation-policy decisions about whether a stage is mandatory.
//!
//! ============================================================================
//! DEPENDENCY OUTPUT DELIVERY
//! ============================================================================
//!
//! Every stage receives a StageInput containing a StageInputs value.
//!
//! StageInputs provides:
//!
//! - The original compilation input.
//! - Outputs from the stage's direct dependencies.
//!
//! This supports dependency fan-in without inventing another IR or
//! requiring a stage to depend on whichever unrelated stage happened
//! to execute immediately before it.
//!
//! A stage can retrieve the wrapper using:
//!
//!     execution.input().downcast_ref::<StageInputs>()
//!
//! A stage can retrieve its original input using:
//!
//!     inputs.initial_input()
//!
//! A stage can retrieve a direct dependency's output using:
//!
//!     inputs.dependency_output(&dependency_id)
//!
//! Domain-specific payload types remain owned by their authoritative
//! repository subsystems.
//!
//! ============================================================================
//! CONCURRENCY
//! ============================================================================
//!
//! This implementation executes stages sequentially in deterministic
//! topological-level order.
//!
//! Sequential execution is compatible with every StageConcurrency
//! declaration because no stage instances are invoked concurrently.
//!
//! This module does not create worker threads or impose a worker ceiling.
//!
//! Parallel execution must be integrated with the existing parallel-build
//! infrastructure and must preserve the same dependency and output rules.
//!
//! ============================================================================
//! LIFECYCLE
//! ============================================================================
//!
//! Pipeline orchestration owns:
//!
//! - CompilationStateTracker::start.
//! - CompilationStateTracker::complete.
//! - CompilationStateTracker::fail.
//! - CompilationStateTracker::mark_cancelled.
//!
//! This module only:
//!
//! - checks that compilation is active;
//! - records the current stage and phase;
//! - checks cancellation;
//! - returns execution results.
//!
//! The caller must start the CompilationStateTracker before invoking
//! PipelineExecutor.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! This module imposes no fixed maximum for:
//!
//! - stages;
//! - dependencies;
//! - compilation units;
//! - domain operations;
//! - CPUs;
//! - GPUs;
//! - FPGAs;
//! - QPUs;
//! - accelerators;
//! - nodes;
//! - workers;
//! - memory;
//! - storage;
//! - qubits;
//! - tensor dimensions;
//! - IR entities.
//!
//! Practical limits are determined by available memory, processing
//! resources, and explicitly configured budgets.
//!
//! ============================================================================

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::compiler::context::compilation_context::CompilationContext;
use crate::compiler::context::compilation_state::{
    CompilationStateError,
    CompilationStateTracker,
};

use crate::compiler::pipeline::ordering::StageOrdering;

use crate::compiler::pipeline::stage::{
    Stage,
    StageError,
    StageErrorKind,
    StageId,
    StageInput,
    StageOutcome,
    StageOutput,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version for the pipeline execution contract.
///
/// This version is independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - AST version.
/// - Semantic-model version.
/// - IR version.
/// - Backend ABI version.
/// - Compiler release version.
pub const PIPELINE_EXECUTION_API_VERSION: u16 = 1;

// ============================================================================
// STAGE INPUT BUNDLE
// ============================================================================

/// Inputs made available to one compiler stage.
///
/// The original compilation input is preserved separately from the outputs
/// of direct prerequisites.
///
/// This prevents an unrelated stage's output from accidentally becoming
/// another stage's input merely because it ran immediately beforehand.
///
/// Dependency outputs are ordered by StageId for deterministic access.
#[derive(Debug, Clone)]
pub struct StageInputs {
    initial_input: StageInput,

    dependency_outputs: BTreeMap<StageId, StageOutput>,
}

impl StageInputs {
    /// Creates the input bundle for one stage.
    ///
    /// The executor normally constructs this value.
    #[must_use]
    pub fn new(
        initial_input: StageInput,
        dependency_outputs: BTreeMap<StageId, StageOutput>,
    ) -> Self {
        Self {
            initial_input,
            dependency_outputs,
        }
    }

    /// Returns the original pipeline input.
    ///
    /// The returned value is not modified by intermediate stages.
    #[must_use]
    pub fn initial_input(&self) -> &StageInput {
        &self.initial_input
    }

    /// Returns the output of one direct dependency.
    ///
    /// Returns None when:
    ///
    /// - the supplied stage is not a direct dependency;
    /// - the dependency did not produce a completed output.
    #[must_use]
    pub fn dependency_output(
        &self,
        dependency: &StageId,
    ) -> Option<&StageOutput> {
        self.dependency_outputs.get(dependency)
    }

    /// Returns all completed direct-dependency outputs.
    ///
    /// The map is deterministically ordered by StageId.
    #[must_use]
    pub fn dependency_outputs(
        &self,
    ) -> &BTreeMap<StageId, StageOutput> {
        &self.dependency_outputs
    }

    /// Returns the number of available direct-dependency outputs.
    #[must_use]
    pub fn dependency_output_count(&self) -> usize {
        self.dependency_outputs.len()
    }

    /// Returns whether the original input is empty.
    #[must_use]
    pub fn initial_input_is_empty(&self) -> bool {
        self.initial_input.is_empty()
    }
}

// ============================================================================
// EXECUTION STATUS
// ============================================================================

/// Final recorded status of an individual stage invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StageExecutionStatus {
    /// The stage executed successfully and produced an output.
    Completed,

    /// The stage deliberately declined execution.
    Skipped,

    /// The stage could not execute because a prerequisite did not complete.
    Blocked,
}

// ============================================================================
// EXECUTION RECORD
// ============================================================================

/// Immutable record of one stage's execution result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageExecutionRecord {
    stage_id: StageId,

    status: StageExecutionStatus,

    reason: Option<Arc<str>>,
}

impl StageExecutionRecord {
    fn completed(stage_id: StageId) -> Self {
        Self {
            stage_id,
            status: StageExecutionStatus::Completed,
            reason: None,
        }
    }

    fn skipped(
        stage_id: StageId,
        reason: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            stage_id,
            status: StageExecutionStatus::Skipped,
            reason: Some(reason.into()),
        }
    }

    fn blocked(
        stage_id: StageId,
        reason: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            stage_id,
            status: StageExecutionStatus::Blocked,
            reason: Some(reason.into()),
        }
    }

    /// Returns the stage identifier.
    #[must_use]
    pub fn stage_id(&self) -> &StageId {
        &self.stage_id
    }

    /// Returns the final stage status.
    #[must_use]
    pub const fn status(&self) -> StageExecutionStatus {
        self.status
    }

    /// Returns the skip or blocking explanation, when applicable.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Returns whether the stage completed successfully.
    #[must_use]
    pub const fn is_completed(&self) -> bool {
        matches!(self.status, StageExecutionStatus::Completed)
    }
}

// ============================================================================
// EXECUTION REPORT
// ============================================================================

/// Report produced by a successfully traversed pipeline.
///
/// Optional stages may be skipped or blocked without invalidating an
/// otherwise successful compilation.
///
/// Required stages must all have Completed status.
///
/// Stage outputs remain type-erased and are owned by their respective
/// domain subsystems.
#[derive(Debug, Clone, Default)]
pub struct ExecutionReport {
    records: BTreeMap<StageId, StageExecutionRecord>,

    outputs: BTreeMap<StageId, StageOutput>,

    required_stages: BTreeSet<StageId>,

    last_output: Option<StageOutput>,
}

impl ExecutionReport {
    fn new(required_stages: BTreeSet<StageId>) -> Self {
        Self {
            records: BTreeMap::new(),
            outputs: BTreeMap::new(),
            required_stages,
            last_output: None,
        }
    }

    fn record_completed(
        &mut self,
        stage_id: StageId,
        output: StageOutput,
    ) {
        self.outputs.insert(stage_id.clone(), output.clone());

        self.last_output = Some(output);

        self.records.insert(
            stage_id.clone(),
            StageExecutionRecord::completed(stage_id),
        );
    }

    fn record_skipped(
        &mut self,
        stage_id: StageId,
        reason: impl Into<Arc<str>>,
    ) {
        self.records.insert(
            stage_id.clone(),
            StageExecutionRecord::skipped(stage_id, reason),
        );
    }

    fn record_blocked(
        &mut self,
        stage_id: StageId,
        reason: impl Into<Arc<str>>,
    ) {
        self.records.insert(
            stage_id.clone(),
            StageExecutionRecord::blocked(stage_id, reason),
        );
    }

    /// Returns all stage execution records.
    ///
    /// Records are ordered by StageId.
    #[must_use]
    pub fn records(
        &self,
    ) -> &BTreeMap<StageId, StageExecutionRecord> {
        &self.records
    }

    /// Returns the record for a particular stage.
    #[must_use]
    pub fn record(
        &self,
        stage_id: &StageId,
    ) -> Option<&StageExecutionRecord> {
        self.records.get(stage_id)
    }

    /// Returns the output produced by a completed stage.
    #[must_use]
    pub fn output_of(
        &self,
        stage_id: &StageId,
    ) -> Option<&StageOutput> {
        self.outputs.get(stage_id)
    }

    /// Returns all outputs produced by completed stages.
    #[must_use]
    pub fn outputs(
        &self,
    ) -> &BTreeMap<StageId, StageOutput> {
        &self.outputs
    }

    /// Returns the output of the last successfully completed stage
    /// in deterministic execution order.
    ///
    /// This is not necessarily the output of a designated finalization
    /// stage. Callers should use output_of when a particular stage's
    /// output is required.
    #[must_use]
    pub fn last_output(&self) -> Option<&StageOutput> {
        self.last_output.as_ref()
    }

    /// Returns the set of stages that the caller declared mandatory.
    #[must_use]
    pub fn required_stages(&self) -> &BTreeSet<StageId> {
        &self.required_stages
    }

    /// Returns the number of stages that received an execution decision.
    #[must_use]
    pub fn stage_count(&self) -> usize {
        self.records.len()
    }

    /// Returns the number of successfully completed stages.
    #[must_use]
    pub fn completed_count(&self) -> usize {
        self.records
            .values()
            .filter(|record| record.is_completed())
            .count()
    }

    /// Returns the number of deliberately skipped stages.
    #[must_use]
    pub fn skipped_count(&self) -> usize {
        self.records
            .values()
            .filter(|record| {
                record.status() == StageExecutionStatus::Skipped
            })
            .count()
    }

    /// Returns the number of blocked stages.
    #[must_use]
    pub fn blocked_count(&self) -> usize {
        self.records
            .values()
            .filter(|record| {
                record.status() == StageExecutionStatus::Blocked
            })
            .count()
    }

    /// Returns whether every required stage completed successfully.
    ///
    /// An empty required-stage set is vacuously satisfied. Production
    /// orchestration should supply its actual mandatory-stage contract.
    #[must_use]
    pub fn all_required_stages_completed(&self) -> bool {
        self.required_stages.iter().all(|stage_id| {
            self.records
                .get(stage_id)
                .is_some_and(StageExecutionRecord::is_completed)
        })
    }

    /// Returns whether any stage was skipped or blocked.
    #[must_use]
    pub fn has_skipped_or_blocked_stages(&self) -> bool {
        self.skipped_count() > 0 || self.blocked_count() > 0
    }

    /// Returns whether the report satisfies its required-stage contract.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.all_required_stages_completed()
    }
}

// ============================================================================
// EXECUTION ERROR
// ============================================================================

/// Classification of pipeline-execution failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ExecutionErrorKind {
    /// The same stage implementation was registered more than once.
    DuplicateStageImplementation,

    /// An ordered stage has no registered implementation.
    MissingStageImplementation,

    /// An implementation is not present in the supplied ordering.
    UnexpectedStageImplementation,

    /// A stage descriptor does not agree with the validated ordering.
    OrderingMismatch,

    /// A caller-declared required stage is not registered.
    MissingRequiredStage,

    /// A required stage deliberately skipped execution.
    RequiredStageSkipped,

    /// A required stage was blocked by an incomplete prerequisite.
    DependencyNotCompleted,

    /// A stage implementation returned an error.
    StageFailed,

    /// Compilation cancellation was requested.
    Cancelled,

    /// The compilation lifecycle tracker rejected an operation.
    StateTracking,

    /// An internal execution invariant was violated.
    InternalInvariant,
}

/// Structured pipeline-execution failure.
///
/// The partial report is retained so that orchestration, diagnostics,
/// and observability can inspect completed work without pretending
/// the pipeline succeeded.
#[derive(Debug, Clone)]
pub struct ExecutionError {
    kind: ExecutionErrorKind,

    stage: Option<StageId>,

    message: Arc<str>,

    cause: Option<StageError>,

    partial_report: ExecutionReport,
}

impl ExecutionError {
    fn new(
        kind: ExecutionErrorKind,
        stage: Option<StageId>,
        message: impl Into<Arc<str>>,
        partial_report: ExecutionReport,
    ) -> Self {
        Self {
            kind,
            stage,
            message: message.into(),
            cause: None,
            partial_report,
        }
    }

    fn from_stage(
        stage: StageId,
        cause: StageError,
        partial_report: ExecutionReport,
    ) -> Self {
        let kind = if cause.kind() == StageErrorKind::Cancelled {
            ExecutionErrorKind::Cancelled
        } else {
            ExecutionErrorKind::StageFailed
        };

        Self {
            kind,
            stage: Some(stage),
            message: Arc::from(cause.to_string()),
            cause: Some(cause),
            partial_report,
        }
    }

    fn from_state(
        stage: Option<StageId>,
        cause: CompilationStateError,
        partial_report: ExecutionReport,
    ) -> Self {
        Self::new(
            ExecutionErrorKind::StateTracking,
            stage,
            cause.to_string(),
            partial_report,
        )
    }

    /// Returns the failure classification.
    #[must_use]
    pub const fn kind(&self) -> ExecutionErrorKind {
        self.kind
    }

    /// Returns the stage associated with the failure, if any.
    #[must_use]
    pub fn stage(&self) -> Option<&StageId> {
        self.stage.as_ref()
    }

    /// Returns the failure description.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the original stage error, when the failure came from
    /// a stage implementation.
    #[must_use]
    pub fn stage_cause(&self) -> Option<&StageError> {
        self.cause.as_ref()
    }

    /// Returns the report accumulated before failure.
    #[must_use]
    pub fn partial_report(&self) -> &ExecutionReport {
        &self.partial_report
    }

    /// Consumes the error and returns its partial report.
    #[must_use]
    pub fn into_partial_report(self) -> ExecutionReport {
        self.partial_report
    }
}

impl fmt::Display for ExecutionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match &self.stage {
            Some(stage) => write!(
                formatter,
                "pipeline execution failed ({:?}) at stage `{}`: {}",
                self.kind,
                stage,
                self.message,
            ),

            None => write!(
                formatter,
                "pipeline execution failed ({:?}): {}",
                self.kind,
                self.message,
            ),
        }
    }
}

impl Error for ExecutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause
            .as_ref()
            .map(|cause| cause as &(dyn Error + 'static))
    }
}

// ============================================================================
// PIPELINE EXECUTOR
// ============================================================================

/// Executes an already-validated compiler-stage ordering.
///
/// The executor is stateless.
///
/// It does not own a global registry, compiler configuration, lifecycle,
/// worker pool, backend, or runtime.
///
/// The supplied CompilationContext and CompilationStateTracker remain
/// authoritative.
#[derive(Debug, Default, Clone, Copy)]
pub struct PipelineExecutor;

impl PipelineExecutor {
    /// Creates a pipeline executor.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Executes the supplied stages in validated dependency order.
    ///
    /// # Arguments
    ///
    /// `compilation`
    ///     Shared immutable compilation configuration, budget accounting,
    ///     and cancellation context.
    ///
    /// `state`
    ///     The lifecycle tracker already started by orchestration.
    ///
    /// `ordering`
    ///     A validated StageOrdering built from these stage descriptors.
    ///
    /// `stages`
    ///     Exactly one implementation for every stage in the ordering.
    ///
    /// `initial_input`
    ///     The original source or compilation input.
    ///
    /// `required_stages`
    ///     Every stage that must complete for this pipeline invocation
    ///     to be considered successful.
    ///
    /// # Execution semantics
    ///
    /// Stages are traversed by StageOrdering::levels().
    ///
    /// Each level is traversed in its existing deterministic order.
    ///
    /// For every stage:
    ///
    /// 1. Check cancellation and lifecycle.
    /// 2. Verify that direct prerequisites completed.
    /// 3. Update compilation progress.
    /// 4. Collect completed prerequisite outputs.
    /// 5. Invoke Stage::run.
    /// 6. Record the exact outcome.
    /// 7. Check cancellation and lifecycle again.
    ///
    /// The executor does not call start(), complete(), fail(), or
    /// mark_cancelled(). Those terminal lifecycle transitions belong
    /// to pipeline orchestration.
    ///
    /// # Errors
    ///
    /// Returns a structured error for:
    ///
    /// - Invalid stage registration.
    /// - Ordering mismatch.
    /// - Missing required stages.
    /// - Required-stage skipping.
    /// - Incomplete dependencies.
    /// - Stage failure.
    /// - Cancellation.
    /// - Lifecycle tracking failure.
    ///
    /// Every execution error retains the partial execution report.
    pub fn execute(
        &self,
        compilation: &CompilationContext,
        state: &CompilationStateTracker,
        ordering: &StageOrdering,
        stages: &[Arc<dyn Stage>],
        initial_input: StageInput,
        required_stages: &BTreeSet<StageId>,
    ) -> Result<ExecutionReport, ExecutionError> {
        let mut report = ExecutionReport::new(
            required_stages.clone(),
        );

        // --------------------------------------------------------------------
        // 1. Validate the supplied stage implementations.
        // --------------------------------------------------------------------

        let stage_map = self.validate_registry(
            ordering,
            stages,
            required_stages,
            &report,
        )?;

        // --------------------------------------------------------------------
        // 2. Verify the shared compilation lifecycle.
        // --------------------------------------------------------------------

        self.check_active(
            compilation,
            state,
            None,
            &report,
        )?;

        // --------------------------------------------------------------------
        // 3. Execute deterministic topological levels.
        // --------------------------------------------------------------------

        for level in ordering.levels() {
            for stage_id in level {
                self.check_active(
                    compilation,
                    state,
                    Some(stage_id.clone()),
                    &report,
                )?;

                let stage = stage_map
                    .get(stage_id)
                    .copied()
                    .ok_or_else(|| {
                        ExecutionError::new(
                            ExecutionErrorKind::InternalInvariant,
                            Some(stage_id.clone()),
                            "validated stage implementation disappeared",
                            report.clone(),
                        )
                    })?;

                // ------------------------------------------------------------
                // 3.1. Check all direct dependencies.
                // ------------------------------------------------------------

                let dependencies = ordering
                    .dependencies_of(stage_id)
                    .ok_or_else(|| {
                        ExecutionError::new(
                            ExecutionErrorKind::InternalInvariant,
                            Some(stage_id.clone()),
                            "ordered stage has no dependency entry",
                            report.clone(),
                        )
                    })?;

                let mut dependency_outputs =
                    BTreeMap::<StageId, StageOutput>::new();

                let mut incomplete_dependency: Option<StageId> = None;

                for dependency in dependencies {
                    match report.record(dependency) {
                        Some(record)
                            if record.status()
                                == StageExecutionStatus::Completed =>
                        {
                            let output = report
                                .output_of(dependency)
                                .ok_or_else(|| {
                                    ExecutionError::new(
                                        ExecutionErrorKind::InternalInvariant,
                                        Some(stage_id.clone()),
                                        "completed dependency has no output entry",
                                        report.clone(),
                                    )
                                })?;

                            dependency_outputs.insert(
                                dependency.clone(),
                                output.clone(),
                            );
                        }

                        Some(_) => {
                            incomplete_dependency =
                                Some(dependency.clone());
                            break;
                        }

                        None => {
                            return Err(
                                ExecutionError::new(
                                    ExecutionErrorKind::InternalInvariant,
                                    Some(stage_id.clone()),
                                    format!(
                                        "dependency `{dependency}` was not \
                                         processed before dependent stage"
                                    ),
                                    report,
                                ),
                            );
                        }
                    }
                }

                // ------------------------------------------------------------
                // 3.2. Block stages whose prerequisites did not complete.
                // ------------------------------------------------------------

                if let Some(dependency) = incomplete_dependency {
                    let reason: Arc<str> = Arc::from(format!(
                        "required prerequisite `{dependency}` did not \
                         complete successfully"
                    ));

                    report.record_blocked(
                        stage_id.clone(),
                        reason.clone(),
                    );

                    if required_stages.contains(stage_id) {
                        return Err(
                            ExecutionError::new(
                                ExecutionErrorKind::DependencyNotCompleted,
                                Some(stage_id.clone()),
                                reason,
                                report,
                            ),
                        );
                    }

                    continue;
                }

                // ------------------------------------------------------------
                // 3.3. Update compilation progress.
                // ------------------------------------------------------------

                let descriptor = stage.descriptor();

                let progress_result = match descriptor.phase() {
                    Some(phase) => state.advance(
                        phase,
                        descriptor.name(),
                    ),

                    None => state.set_stage(
                        descriptor.name(),
                    ),
                };

                if let Err(error) = progress_result {
                    return Err(
                        ExecutionError::from_state(
                            Some(stage_id.clone()),
                            error,
                            report,
                        ),
                    );
                }

                // ------------------------------------------------------------
                // 3.4. Construct dependency-aware stage input.
                // ------------------------------------------------------------

                let inputs = StageInputs::new(
                    initial_input.clone(),
                    dependency_outputs,
                );

                let stage_input = StageInput::new(inputs);

                // ------------------------------------------------------------
                // 3.5. Execute the stage.
                // ------------------------------------------------------------

                let outcome = match stage.run(
                    compilation,
                    stage_input,
                ) {
                    Ok(outcome) => outcome,

                    Err(error) => {
                        let cancelled =
                            error.kind() == StageErrorKind::Cancelled
                                || compilation.is_cancelled()
                                || state.is_cancelled();

                        if cancelled {
                            return Err(
                                ExecutionError::from_stage(
                                    stage_id.clone(),
                                    error,
                                    report,
                                ),
                            );
                        }

                        return Err(
                            ExecutionError::from_stage(
                                stage_id.clone(),
                                error,
                                report,
                            ),
                        );
                    }
                };

                // ------------------------------------------------------------
                // 3.6. Verify lifecycle and cancellation after execution.
                // ------------------------------------------------------------

                self.check_active(
                    compilation,
                    state,
                    Some(stage_id.clone()),
                    &report,
                )?;

                // ------------------------------------------------------------
                // 3.7. Record the exact stage outcome.
                // ------------------------------------------------------------

                match outcome {
                    StageOutcome::Completed(output) => {
                        report.record_completed(
                            stage_id.clone(),
                            output,
                        );
                    }

                    StageOutcome::Skipped { reason } => {
                        report.record_skipped(
                            stage_id.clone(),
                            reason.clone(),
                        );

                        if required_stages.contains(stage_id) {
                            return Err(
                                ExecutionError::new(
                                    ExecutionErrorKind::RequiredStageSkipped,
                                    Some(stage_id.clone()),
                                    reason,
                                    report,
                                ),
                            );
                        }
                    }

                    _ => {
                        return Err(
                            ExecutionError::new(
                                ExecutionErrorKind::InternalInvariant,
                                Some(stage_id.clone()),
                                "stage returned an unknown outcome",
                                report,
                            ),
                        );
                    }
                }
            }
        }

        // --------------------------------------------------------------------
        // 4. Enforce the complete required-stage contract.
        // --------------------------------------------------------------------

        for required_stage in required_stages {
            let completed = report
                .record(required_stage)
                .is_some_and(StageExecutionRecord::is_completed);

            if !completed {
                return Err(
                    ExecutionError::new(
                        ExecutionErrorKind::RequiredStageSkipped,
                        Some(required_stage.clone()),
                        "required stage did not complete successfully",
                        report,
                    ),
                );
            }
        }

        // --------------------------------------------------------------------
        // 5. Ensure cancellation did not race with final return.
        // --------------------------------------------------------------------

        self.check_active(
            compilation,
            state,
            None,
            &report,
        )?;

        Ok(report)
    }

    // =========================================================================
    // REGISTRY VALIDATION
    // =========================================================================

    fn validate_registry<'a>(
        &self,
        ordering: &StageOrdering,
        stages: &'a [Arc<dyn Stage>],
        required_stages: &BTreeSet<StageId>,
        report: &ExecutionReport,
    ) -> Result<BTreeMap<StageId, &'a dyn Stage>, ExecutionError> {
        let mut stage_map =
            BTreeMap::<StageId, &'a dyn Stage>::new();

        // --------------------------------------------------------------------
        // Validate unique implementations.
        // --------------------------------------------------------------------

        for stage in stages {
            let descriptor = stage.descriptor();
            let stage_id = descriptor.id().clone();

            if stage_map
                .insert(stage_id.clone(), stage.as_ref())
                .is_some()
            {
                return Err(
                    ExecutionError::new(
                        ExecutionErrorKind::DuplicateStageImplementation,
                        Some(stage_id.clone()),
                        format!(
                            "multiple implementations were registered \
                             for stage `{stage_id}`"
                        ),
                        report.clone(),
                    ),
                );
            }
        }

        // --------------------------------------------------------------------
        // Validate exact agreement between registry and ordering.
        // --------------------------------------------------------------------

        for stage_id in ordering.ordered_stages() {
            let stage = stage_map.get(stage_id).ok_or_else(|| {
                ExecutionError::new(
                    ExecutionErrorKind::MissingStageImplementation,
                    Some(stage_id.clone()),
                    format!(
                        "ordering contains stage `{stage_id}` without \
                         a registered implementation"
                    ),
                    report.clone(),
                )
            })?;

            let mut descriptor_dependencies =
                stage.descriptor().dependencies().to_vec();

            descriptor_dependencies.sort();

            let ordered_dependencies = ordering
                .dependencies_of(stage_id)
                .ok_or_else(|| {
                    ExecutionError::new(
                        ExecutionErrorKind::OrderingMismatch,
                        Some(stage_id.clone()),
                        "ordering is missing the stage dependency entry",
                        report.clone(),
                    )
                })?;

            if descriptor_dependencies.as_slice()
                != ordered_dependencies
            {
                return Err(
                    ExecutionError::new(
                        ExecutionErrorKind::OrderingMismatch,
                        Some(stage_id.clone()),
                        format!(
                            "stage `{stage_id}` descriptor dependencies \
                             disagree with StageOrdering"
                        ),
                        report.clone(),
                    ),
                );
            }
        }

        // --------------------------------------------------------------------
        // Reject implementations absent from the ordering.
        // --------------------------------------------------------------------

        for stage_id in stage_map.keys() {
            if !ordering.contains(stage_id) {
                return Err(
                    ExecutionError::new(
                        ExecutionErrorKind::UnexpectedStageImplementation,
                        Some(stage_id.clone()),
                        format!(
                            "stage implementation `{stage_id}` is not \
                             present in StageOrdering"
                        ),
                        report.clone(),
                    ),
                );
            }
        }

        // --------------------------------------------------------------------
        // Validate mandatory stages before executing anything.
        // --------------------------------------------------------------------

        for required_stage in required_stages {
            if !ordering.contains(required_stage)
                || !stage_map.contains_key(required_stage)
            {
                return Err(
                    ExecutionError::new(
                        ExecutionErrorKind::MissingRequiredStage,
                        Some(required_stage.clone()),
                        format!(
                            "required stage `{required_stage}` is not \
                             registered in the pipeline"
                        ),
                        report.clone(),
                    ),
                );
            }
        }

        Ok(stage_map)
    }

    // =========================================================================
    // CANCELLATION AND LIFECYCLE CHECKS
    // =========================================================================

    fn check_active(
        &self,
        compilation: &CompilationContext,
        state: &CompilationStateTracker,
        stage: Option<StageId>,
        report: &ExecutionReport,
    ) -> Result<(), ExecutionError> {
        if let Err(error) = compilation.check_cancelled() {
            return Err(
                ExecutionError::new(
                    ExecutionErrorKind::Cancelled,
                    stage,
                    error.to_string(),
                    report.clone(),
                ),
            );
        }

        if let Err(error) = state.check_active() {
            return Err(
                ExecutionError::from_state(
                    stage,
                    error,
                    report.clone(),
                ),
            );
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

    use crate::compiler::context::compilation_state::CompilationPhase;

    use crate::compiler::pipeline::stage::{
        StageConcurrency,
        StageDecision,
        StageDescriptor,
        StageDeterminism,
        StageExecution,
        StageExecutionContext,
        StageId,
        StageOutcome,
        StageOutput,
    };

    use crate::compiler_types::CompilerConfig;

    // ------------------------------------------------------------------------
    // TEST STAGE
    // ------------------------------------------------------------------------

    struct TestStage {
        descriptor: StageDescriptor,

        skip: bool,

        expected_dependency: Option<StageId>,
    }

    impl TestStage {
        fn new(
            id: &str,
            dependencies: &[&str],
            skip: bool,
            expected_dependency: Option<&str>,
        ) -> Self {
            let mut descriptor = StageDescriptor::new(
                StageId::new(id).expect("valid stage ID"),
                format!("Test stage {id}"),
                "1",
                format!("Test implementation for {id}"),
            )
            .expect("valid descriptor")
            .with_phase(CompilationPhase::Initialization)
            .with_determinism(StageDeterminism::Deterministic)
            .with_concurrency(StageConcurrency::InstanceExclusive);

            for dependency in dependencies {
                descriptor = descriptor
                    .with_dependency(
                        StageId::new(*dependency)
                            .expect("valid dependency"),
                    )
                    .expect("valid dependency declaration");
            }

            Self {
                descriptor,
                skip,
                expected_dependency: expected_dependency.map(
                    |value| {
                        StageId::new(value)
                            .expect("valid expected dependency")
                    },
                ),
            }
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
            if self.skip {
                StageDecision::skip(
                    "not applicable to this test program",
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

            let inputs = execution
                .input()
                .downcast_ref::<StageInputs>()
                .ok_or_else(|| {
                    StageError::new(
                        StageErrorKind::InvalidInput,
                        "expected StageInputs wrapper",
                    )
                })?;

            if let Some(dependency) = &self.expected_dependency {
                let output = inputs
                    .dependency_output(dependency)
                    .ok_or_else(|| {
                        StageError::new(
                            StageErrorKind::InvalidInput,
                            format!(
                                "missing dependency output `{dependency}`"
                            ),
                        )
                    })?;

                if output.downcast_ref::<String>().is_none() {
                    return Err(
                        StageError::new(
                            StageErrorKind::InvalidInput,
                            "dependency output had an unexpected type",
                        ),
                    );
                }
            }

            Ok(StageOutput::new(format!(
                "completed:{}",
                self.descriptor.id()
            )))
        }
    }

    // ------------------------------------------------------------------------
    // TEST HELPERS
    // ------------------------------------------------------------------------

    fn compilation() -> CompilationContext {
        CompilationContext::new(CompilerConfig::default())
            .expect("valid test configuration")
    }

    fn state(
        compilation: &CompilationContext,
    ) -> CompilationStateTracker {
        let tracker = CompilationStateTracker::with_cancellation(
            compilation.cancellation_token(),
        );

        tracker.start().expect("start test compilation");

        tracker
    }

    fn id(value: &str) -> StageId {
        StageId::new(value).expect("valid test stage ID")
    }

    fn ordering(
        stages: &[Arc<dyn Stage>],
    ) -> StageOrdering {
        let descriptors = stages
            .iter()
            .map(|stage| stage.descriptor().clone())
            .collect::<Vec<_>>();

        StageOrdering::from_slice(&descriptors)
            .expect("valid stage ordering")
    }

    fn required(values: &[&str]) -> BTreeSet<StageId> {
        values.iter().map(|value| id(value)).collect()
    }

    // ------------------------------------------------------------------------
    // API AND INPUT CONTRACT
    // ------------------------------------------------------------------------

    #[test]
    fn execution_api_version_is_nonzero() {
        assert_ne!(PIPELINE_EXECUTION_API_VERSION, 0);
    }

    #[test]
    fn stage_inputs_preserve_initial_input() {
        let inputs = StageInputs::new(
            StageInput::new(String::from("source")),
            BTreeMap::new(),
        );

        assert_eq!(
            inputs
                .initial_input()
                .downcast_ref::<String>()
                .map(String::as_str),
            Some("source"),
        );

        assert!(inputs.dependency_outputs().is_empty());
    }

    #[test]
    fn stage_inputs_expose_dependency_outputs() {
        let dependency = id("frontend.parse");

        let mut outputs = BTreeMap::new();

        outputs.insert(
            dependency.clone(),
            StageOutput::new(String::from("parsed")),
        );

        let inputs = StageInputs::new(
            StageInput::empty(),
            outputs,
        );

        assert_eq!(
            inputs
                .dependency_output(&dependency)
                .and_then(|output| {
                    output.downcast_ref::<String>()
                })
                .map(String::as_str),
            Some("parsed"),
        );
    }

    // ------------------------------------------------------------------------
    // NORMAL EXECUTION
    // ------------------------------------------------------------------------

    #[test]
    fn executor_runs_stages_in_dependency_order() {
        let compilation = compilation();
        let state = state(&compilation);

        let stages: Vec<Arc<dyn Stage>> = vec![
            Arc::new(TestStage::new(
                "frontend.parse",
                &[],
                false,
                None,
            )),
            Arc::new(TestStage::new(
                "semantic.analysis",
                &["frontend.parse"],
                false,
                Some("frontend.parse"),
            )),
        ];

        let ordering = ordering(&stages);

        let report = PipelineExecutor::new()
            .execute(
                &compilation,
                &state,
                &ordering,
                &stages,
                StageInput::new(String::from("source")),
                &required(&[
                    "frontend.parse",
                    "semantic.analysis",
                ]),
            )
            .expect("pipeline should complete");

        assert_eq!(report.stage_count(), 2);
        assert_eq!(report.completed_count(), 2);
        assert_eq!(report.skipped_count(), 0);
        assert_eq!(report.blocked_count(), 0);
        assert!(report.is_successful());

        assert!(
            report
                .output_of(&id("semantic.analysis"))
                .is_some()
        );
    }

    #[test]
    fn executor_delivers_direct_dependency_outputs() {
        let compilation = compilation();
        let state = state(&compilation);

        let stages: Vec<Arc<dyn Stage>> = vec![
            Arc::new(TestStage::new(
                "source.parse",
                &[],
                false,
                None,
            )),
            Arc::new(TestStage::new(
                "ir.generate",
                &["source.parse"],
                false,
                Some("source.parse"),
            )),
        ];

        let ordering = ordering(&stages);

        let report = PipelineExecutor::new()
            .execute(
                &compilation,
                &state,
                &ordering,
                &stages,
                StageInput::empty(),
                &required(&["ir.generate"]),
            )
            .expect("dependency output should be delivered");

        assert!(report.is_successful());
    }

    // ------------------------------------------------------------------------
    // CONDITIONAL EXECUTION
    // ------------------------------------------------------------------------

    #[test]
    fn optional_stage_can_be_skipped() {
        let compilation = compilation();
        let state = state(&compilation);

        let stages: Vec<Arc<dyn Stage>> = vec![
            Arc::new(TestStage::new(
                "classical.analysis",
                &[],
                false,
                None,
            )),
            Arc::new(TestStage::new(
                "quantum.analysis",
                &[],
                true,
                None,
            )),
        ];

        let ordering = ordering(&stages);

        let report = PipelineExecutor::new()
            .execute(
                &compilation,
                &state,
                &ordering,
                &stages,
                StageInput::empty(),
                &required(&["classical.analysis"]),
            )
            .expect("optional skip must not fail unrelated work");

        assert_eq!(report.completed_count(), 1);
        assert_eq!(report.skipped_count(), 1);

        assert!(
            report.is_successful()
        );

        assert_eq!(
            report
                .record(&id("quantum.analysis"))
                .expect("skip record")
                .status(),
            StageExecutionStatus::Skipped,
        );
    }

    #[test]
    fn required_stage_cannot_be_skipped() {
        let compilation = compilation();
        let state = state(&compilation);

        let stages: Vec<Arc<dyn Stage>> = vec![
            Arc::new(TestStage::new(
                "required.analysis",
                &[],
                true,
                None,
            )),
        ];

        let ordering = ordering(&stages);

        let error = PipelineExecutor::new()
            .execute(
                &compilation,
                &state,
                &ordering,
                &stages,
                StageInput::empty(),
                &required(&["required.analysis"]),
            )
            .expect_err("required stage must not be skipped");

        assert_eq!(
            error.kind(),
            ExecutionErrorKind::RequiredStageSkipped,
        );

        assert_eq!(
            error
                .partial_report()
                .record(&id("required.analysis"))
                .expect("skip record")
                .status(),
            StageExecutionStatus::Skipped,
        );
    }

    #[test]
    fn dependent_stage_is_blocked_when_prerequisite_skips() {
        let compilation = compilation();
        let state = state(&compilation);

        let stages: Vec<Arc<dyn Stage>> = vec![
            Arc::new(TestStage::new(
                "optional.frontend",
                &[],
                true,
                None,
            )),
            Arc::new(TestStage::new(
                "required.semantic",
                &["optional.frontend"],
                false,
                None,
            )),
        ];

        let ordering = ordering(&stages);

        let error = PipelineExecutor::new()
            .execute(
                &compilation,
                &state,
                &ordering,
                &stages,
                StageInput::empty(),
                &required(&["required.semantic"]),
            )
            .expect_err("dependent required stage must be blocked");

        assert_eq!(
            error.kind(),
            ExecutionErrorKind::DependencyNotCompleted,
        );

        assert_eq!(
            error
                .partial_report()
                .record(&id("required.semantic"))
                .expect("blocked record")
                .status(),
            StageExecutionStatus::Blocked,
        );
    }

    // ------------------------------------------------------------------------
    // REGISTRY VALIDATION
    // ------------------------------------------------------------------------

    #[test]
    fn duplicate_stage_implementations_are_rejected() {
        let compilation = compilation();
        let state = state(&compilation);

        let stage = Arc::new(TestStage::new(
            "duplicate.stage",
            &[],
            false,
            None,
        ));

        let stages: Vec<Arc<dyn Stage>> = vec![
            stage.clone(),
            stage,
        ];

        let descriptors = vec![
            stages[0].descriptor().clone(),
        ];

        let ordering = StageOrdering::from_slice(&descriptors)
            .expect("single-stage ordering");

        let error = PipelineExecutor::new()
            .execute(
                &compilation,
                &state,
                &ordering,
                &stages,
                StageInput::empty(),
                &required(&["duplicate.stage"]),
            )
            .expect_err("duplicate implementations must fail");

        assert_eq!(
            error.kind(),
            ExecutionErrorKind::DuplicateStageImplementation,
        );
    }

    #[test]
    fn missing_required_stage_is_rejected_before_execution() {
        let compilation = compilation();
        let state = state(&compilation);

        let stages: Vec<Arc<dyn Stage>> = vec![
            Arc::new(TestStage::new(
                "registered.stage",
                &[],
                false,
                None,
            )),
        ];

        let ordering = ordering(&stages);

        let error = PipelineExecutor::new()
            .execute(
                &compilation,
                &state,
                &ordering,
                &stages,
                StageInput::empty(),
                &required(&["missing.stage"]),
            )
            .expect_err("missing required stage must fail");

        assert_eq!(
            error.kind(),
            ExecutionErrorKind::MissingRequiredStage,
        );

        assert_eq!(error.partial_report().stage_count(), 0);
    }

    // ------------------------------------------------------------------------
    // CANCELLATION
    // ------------------------------------------------------------------------

    #[test]
    fn cancelled_compilation_does_not_execute_stages() {
        let compilation = compilation();
        let state = state(&compilation);

        let stages: Vec<Arc<dyn Stage>> = vec![
            Arc::new(TestStage::new(
                "cancelled.stage",
                &[],
                false,
                None,
            )),
        ];

        let ordering = ordering(&stages);

        compilation.cancellation_token().cancel();

        let error = PipelineExecutor::new()
            .execute(
                &compilation,
                &state,
                &ordering,
                &stages,
                StageInput::empty(),
                &required(&["cancelled.stage"]),
            )
            .expect_err("cancelled compilation must fail");

        assert_eq!(
            error.kind(),
            ExecutionErrorKind::Cancelled,
        );

        assert_eq!(error.partial_report().stage_count(), 0);
    }

    // ------------------------------------------------------------------------
    // LIFECYCLE
    // ------------------------------------------------------------------------

    #[test]
    fn executor_does_not_complete_the_compilation_lifecycle() {
        let compilation = compilation();
        let state = state(&compilation);

        let stages: Vec<Arc<dyn Stage>> = vec![
            Arc::new(TestStage::new(
                "final.stage",
                &[],
                false,
                None,
            )),
        ];

        let ordering = ordering(&stages);

        PipelineExecutor::new()
            .execute(
                &compilation,
                &state,
                &ordering,
                &stages,
                StageInput::empty(),
                &required(&["final.stage"]),
            )
            .expect("stage should complete");

        assert!(
            state
                .lifecycle()
                .expect("lifecycle snapshot")
                != crate::compiler::context::compilation_state::
                    CompilationLifecycle::Completed
        );
    }

    // ------------------------------------------------------------------------
    // REPORT CONTRACT
    // ------------------------------------------------------------------------

    #[test]
    fn report_distinguishes_completed_skipped_and_blocked() {
        let compilation = compilation();
        let state = state(&compilation);

        let stages: Vec<Arc<dyn Stage>> = vec![
            Arc::new(TestStage::new(
                "completed",
                &[],
                false,
                None,
            )),
            Arc::new(TestStage::new(
                "skipped",
                &[],
                true,
                None,
            )),
            Arc::new(TestStage::new(
                "blocked",
                &["skipped"],
                false,
                None,
            )),
        ];

        let ordering = ordering(&stages);

        let report = PipelineExecutor::new()
            .execute(
                &compilation,
                &state,
                &ordering,
                &stages,
                StageInput::empty(),
                &required(&["completed"]),
            )
            .expect("optional blocked branch is permitted");

        assert_eq!(report.completed_count(), 1);
        assert_eq!(report.skipped_count(), 1);
        assert_eq!(report.blocked_count(), 1);

        assert!(
            report.has_skipped_or_blocked_stages()
        );

        assert!(
            report.is_successful()
        );
    }
}
