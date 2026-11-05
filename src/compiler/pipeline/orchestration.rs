
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Pipeline Orchestration.
//!
//! Path:
//!     src/compiler/pipeline/orchestration.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Coordinates one complete execution of a registered compiler-stage graph.
//!
//! This module consumes the canonical contracts defined by:
//!
//!     compiler::pipeline::stage
//!     compiler::pipeline::ordering
//!     compiler::context::compilation_context
//!     compiler::context::compilation_state
//!
//! It provides:
//!
//! - Stage implementation registration.
//! - Stage graph validation.
//! - Deterministic dependency-aware execution.
//! - Conditional stage execution.
//! - Direct dependency output delivery.
//! - Required-stage enforcement.
//! - Explicit skipped and blocked outcomes.
//! - Cancellation propagation.
//! - Compilation lifecycle management.
//! - Partial execution reports.
//! - Structured orchestration errors.
//!
//! ============================================================================
//! OWNERSHIP BOUNDARIES
//! ============================================================================
//!
//! This module owns pipeline orchestration only.
//!
//! It does NOT own:
//!
//! - Source-language semantics.
//! - Grammar interpretation.
//! - AST construction.
//! - Type inference.
//! - Ownership or borrow analysis.
//! - Effect analysis.
//! - Capability semantics.
//! - Resource requirement semantics.
//! - Canonical IR.
//! - IR verification algorithms.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Physical scheduling.
//! - Physical routing.
//! - Backend implementation.
//! - Artifact serialization.
//! - Runtime execution.
//! - Distributed execution.
//! - Retry or recovery policy.
//!
//! Those responsibilities remain with their authoritative subsystems.
//!
//! ============================================================================
//! EXECUTION MODEL
//! ============================================================================
//!
//! The orchestrator:
//!
//! 1. Validates the registered stage implementations.
//! 2. Builds a deterministic StageOrdering.
//! 3. Validates the required-stage set.
//! 4. Creates a lifecycle tracker sharing the compilation cancellation
//!    token.
//! 5. Starts the compilation lifecycle.
//! 6. Executes dependency-ready stages.
//! 7. Preserves outputs and explicit skipped/blocked outcomes.
//! 8. Rejects successful completion when a required stage did not complete.
//! 9. Completes, fails, or marks the lifecycle cancelled.
//!
//! This implementation is intentionally sequential.
//!
//! Sequential execution is valid for every declared StageConcurrency
//! contract. It does not assume that independent stages are automatically
//! thread-safe or that available worker resources are unlimited.
//!
//! A future parallel dispatcher may execute independent stages concurrently,
//! but it must preserve the same dependency, cancellation, required-stage,
//! output, and failure contracts.
//!
//! ============================================================================
//! INPUT CONTRACT
//! ============================================================================
//!
//! Every stage receives a StageInput containing StageInputs.
//!
//! StageInputs contains:
//!
//! - The original pipeline input.
//! - The successful outputs of the stage's direct dependencies.
//!
//! This explicit fan-in contract prevents a multi-domain pipeline from
//! accidentally passing only the output of the immediately preceding stage.
//!
//! Stage implementations should downcast their StageInput to StageInputs
//! and then retrieve the original input and dependency outputs they require.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module imposes no universal limits on:
//!
//! - Pipeline stages.
//! - Stage dependencies.
//! - Compilation units.
//! - Modules.
//! - Functions.
//! - CPUs.
//! - Cores.
//! - Workers.
//! - GPUs.
//! - FPGAs.
//! - ASICs.
//! - QPUs.
//! - Accelerators.
//! - Nodes.
//! - Devices.
//! - Memory.
//! - Storage.
//! - Qubits.
//! - Tensor dimensions.
//! - Tensor rank.
//! - IR operations.
//!
//! Operational limits are supplied by CompilationContext and the stages
//! that perform measurable work.
//!
//! This module does not silently truncate work.
//!
//! No finite implementation can promise infinite physical resources.
//!
//! ============================================================================
//! GRAMMAR AND SEMANTIC BOUNDARY
//! ============================================================================
//!
//! The source grammar describes language constructs and compilation intent.
//!
//! The semantic model determines their meaning.
//!
//! This module receives already-constructed compiler-stage implementations.
//!
//! It does not interpret grammar declarations or derive hardware execution
//! decisions directly from source syntax.
//!
//! Relevant repository authorities include:
//!
//!     grammar/Zamani.g4
//!     grammar/compile/compile.g4
//!     grammar/specification/semantic-model.md
//!     grammar/validation/semantic-boundaries.md
//!
//! The grammar and semantic model remain authoritative for source meaning.
//!
//! ============================================================================

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::compiler::context::compilation_context::CompilationContext;
use crate::compiler::context::compilation_state::{
    CompilationPhase,
    CompilationStateError,
    CompilationStateTracker,
};

use crate::compiler::pipeline::ordering::{
    OrderingError,
    StageOrdering,
};

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

/// Public API version of pipeline orchestration.
///
/// Independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Semantic-model version.
/// - Compiler release version.
/// - AST version.
/// - IR version.
/// - Backend ABI version.
pub const PIPELINE_ORCHESTRATION_API_VERSION: u16 = 1;

// ============================================================================
// STAGE INPUTS
// ============================================================================

/// Input bundle supplied to an individual pipeline stage.
///
/// This is the canonical input contract used by PipelineOrchestrator.
///
/// The bundle contains the original pipeline input and the successful
/// outputs of the stage's direct dependencies.
///
/// Outputs from unrelated stages are deliberately not exposed as direct
/// dependencies. They remain available in the final ExecutionReport.
///
/// Domain-specific payload types remain owned by their respective
/// subsystems.
#[derive(Clone, Default)]
pub struct StageInputs {
    initial_input: StageInput,

    dependency_outputs: BTreeMap<StageId, StageOutput>,
}

impl StageInputs {
    /// Creates a stage input bundle.
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
    #[must_use]
    pub fn initial_input(&self) -> &StageInput {
        &self.initial_input
    }

    /// Returns the output of one direct dependency.
    ///
    /// Returns None if the requested stage is not a direct dependency
    /// or has no successfully completed output.
    #[must_use]
    pub fn dependency_output(
        &self,
        dependency: &StageId,
    ) -> Option<&StageOutput> {
        self.dependency_outputs.get(dependency)
    }

    /// Returns all successfully completed direct dependency outputs.
    #[must_use]
    pub fn dependency_outputs(
        &self,
    ) -> &BTreeMap<StageId, StageOutput> {
        &self.dependency_outputs
    }

    /// Returns whether there are dependency outputs.
    #[must_use]
    pub fn has_dependency_outputs(&self) -> bool {
        !self.dependency_outputs.is_empty()
    }
}

impl fmt::Debug for StageInputs {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("StageInputs")
            .field("initial_input", &self.initial_input)
            .field(
                "dependency_output_count",
                &self.dependency_outputs.len(),
            )
            .finish()
    }
}

// ============================================================================
// EXECUTION STATUS
// ============================================================================

/// Final recorded state of one pipeline stage.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StageExecutionStatus {
    /// The stage completed successfully.
    Completed,

    /// The stage deliberately skipped execution.
    Skipped {
        reason: Arc<str>,
    },

    /// The stage could not execute because a prerequisite did not complete.
    Blocked {
        dependency: StageId,
        reason: Arc<str>,
    },

    /// The stage started but failed.
    Failed {
        message: Arc<str>,
    },
}

// ============================================================================
// EXECUTION RECORD
// ============================================================================

/// Immutable record of one stage's execution outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageExecutionRecord {
    stage_id: StageId,

    status: StageExecutionStatus,
}

impl StageExecutionRecord {
    /// Creates a stage execution record.
    #[must_use]
    pub fn new(
        stage_id: StageId,
        status: StageExecutionStatus,
    ) -> Self {
        Self {
            stage_id,
            status,
        }
    }

    /// Returns the stage identifier.
    #[must_use]
    pub fn stage_id(&self) -> &StageId {
        &self.stage_id
    }

    /// Returns the recorded status.
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

/// Complete or partial report of one pipeline execution.
///
/// The report retains successful outputs even when a later stage fails.
///
/// A report is not automatically evidence of successful compilation.
/// Call is_successful() to verify that every required stage completed.
#[derive(Debug, Clone, Default)]
pub struct ExecutionReport {
    records: BTreeMap<StageId, StageExecutionRecord>,

    outputs: BTreeMap<StageId, StageOutput>,

    required_stages: BTreeSet<StageId>,
}

impl ExecutionReport {
    /// Creates an empty report with the required-stage contract.
    #[must_use]
    pub fn new(
        required_stages: BTreeSet<StageId>,
    ) -> Self {
        Self {
            records: BTreeMap::new(),
            outputs: BTreeMap::new(),
            required_stages,
        }
    }

    /// Returns every recorded stage outcome.
    #[must_use]
    pub fn records(
        &self,
    ) -> &BTreeMap<StageId, StageExecutionRecord> {
        &self.records
    }

    /// Returns every successfully completed stage output.
    #[must_use]
    pub fn outputs(
        &self,
    ) -> &BTreeMap<StageId, StageOutput> {
        &self.outputs
    }

    /// Returns the output of a successfully completed stage.
    #[must_use]
    pub fn output_of(
        &self,
        stage: &StageId,
    ) -> Option<&StageOutput> {
        self.outputs.get(stage)
    }

    /// Returns a stage's execution record.
    #[must_use]
    pub fn record(
        &self,
        stage: &StageId,
    ) -> Option<&StageExecutionRecord> {
        self.records.get(stage)
    }

    /// Returns the required stage identifiers.
    #[must_use]
    pub fn required_stages(
        &self,
    ) -> &BTreeSet<StageId> {
        &self.required_stages
    }

    /// Returns whether all required stages completed successfully.
    ///
    /// Optional stages may be skipped or blocked without invalidating
    /// the report, provided no required stage is affected.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.required_stages.iter().all(|stage| {
            self.records
                .get(stage)
                .is_some_and(StageExecutionRecord::is_completed)
        })
    }

    /// Returns whether any stage was skipped or blocked.
    #[must_use]
    pub fn has_skipped_or_blocked_stages(&self) -> bool {
        self.records.values().any(|record| {
            matches!(
                record.status(),
                StageExecutionStatus::Skipped { .. }
                    | StageExecutionStatus::Blocked { .. }
            )
        })
    }

    /// Returns the number of recorded stages.
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

    fn record(
        &mut self,
        stage_id: StageId,
        status: StageExecutionStatus,
    ) {
        let record = StageExecutionRecord::new(
            stage_id.clone(),
            status,
        );

        self.records.insert(stage_id, record);
    }

    fn record_completed(
        &mut self,
        stage_id: StageId,
        output: StageOutput,
    ) {
        self.outputs.insert(stage_id.clone(), output);

        self.record(
            stage_id,
            StageExecutionStatus::Completed,
        );
    }
}

// ============================================================================
// ORCHESTRATION ERROR CLASSIFICATION
// ============================================================================

/// Classification of pipeline orchestration failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PipelineOrchestrationErrorKind {
    /// The registered stage graph is invalid.
    InvalidStageGraph,

    /// A required stage does not exist in the registered graph.
    MissingRequiredStage,

    /// A required stage was deliberately skipped.
    RequiredStageSkipped,

    /// A required stage was blocked by an unsuccessful prerequisite.
    RequiredStageBlocked,

    /// A stage failed.
    StageFailed,

    /// Compilation was cancelled.
    Cancelled,

    /// Compilation lifecycle tracking failed.
    LifecycleFailure,

    /// The pipeline completed without satisfying its required-stage contract.
    IncompleteRequiredWork,
}

impl fmt::Display for PipelineOrchestrationErrorKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidStageGraph => {
                formatter.write_str("invalid compiler stage graph")
            }

            Self::MissingRequiredStage => {
                formatter.write_str("required compiler stage is not registered")
            }

            Self::RequiredStageSkipped => {
                formatter.write_str("required compiler stage was skipped")
            }

            Self::RequiredStageBlocked => {
                formatter.write_str("required compiler stage was blocked")
            }

            Self::StageFailed => {
                formatter.write_str("compiler stage failed")
            }

            Self::Cancelled => {
                formatter.write_str("compilation was cancelled")
            }

            Self::LifecycleFailure => {
                formatter.write_str("compilation lifecycle tracking failed")
            }

            Self::IncompleteRequiredWork => {
                formatter.write_str("required compiler work did not complete")
            }
        }
    }
}

// ============================================================================
// ORCHESTRATION ERROR
// ============================================================================

/// Structured orchestration failure.
///
/// The partial report is retained so callers can inspect successful work
/// completed before the failure.
///
/// Stage errors are preserved as typed errors rather than reduced to
/// diagnostic strings.
#[derive(Debug)]
pub struct PipelineOrchestrationError {
    kind: PipelineOrchestrationErrorKind,

    stage: Option<StageId>,

    message: Arc<str>,

    stage_error: Option<StageError>,

    partial_report: ExecutionReport,
}

impl PipelineOrchestrationError {
    fn new(
        kind: PipelineOrchestrationErrorKind,
        stage: Option<StageId>,
        message: impl Into<Arc<str>>,
        stage_error: Option<StageError>,
        partial_report: ExecutionReport,
    ) -> Self {
        Self {
            kind,
            stage,
            message: message.into(),
            stage_error,
            partial_report,
        }
    }

    /// Returns the failure classification.
    #[must_use]
    pub const fn kind(
        &self,
    ) -> PipelineOrchestrationErrorKind {
        self.kind
    }

    /// Returns the stage associated with the failure, when applicable.
    #[must_use]
    pub fn stage(&self) -> Option<&StageId> {
        self.stage.as_ref()
    }

    /// Returns the explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    /// Returns the original stage error, when applicable.
    #[must_use]
    pub fn stage_error(&self) -> Option<&StageError> {
        self.stage_error.as_ref()
    }

    /// Returns the partial execution report.
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

impl fmt::Display for PipelineOrchestrationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match &self.stage {
            Some(stage) => write!(
                formatter,
                "{} at stage `{stage}`: {}",
                self.kind,
                self.message
            ),

            None => write!(
                formatter,
                "{}: {}",
                self.kind,
                self.message
            ),
        }
    }
}

impl Error for PipelineOrchestrationError {
    fn source(
        &self,
    ) -> Option<&(dyn Error + 'static)> {
        self.stage_error
            .as_ref()
            .map(|error| error as &(dyn Error + 'static))
    }
}

// ============================================================================
// PIPELINE RESULT
// ============================================================================

/// Successful pipeline execution result.
///
/// The lifecycle tracker is returned so callers can inspect the final
/// compilation state without introducing another lifecycle implementation.
#[derive(Debug, Clone)]
pub struct PipelineRun {
    report: ExecutionReport,

    state: CompilationStateTracker,
}

impl PipelineRun {
    /// Returns the execution report.
    #[must_use]
    pub fn report(&self) -> &ExecutionReport {
        &self.report
    }

    /// Returns the compilation lifecycle tracker.
    #[must_use]
    pub fn state(&self) -> &CompilationStateTracker {
        &self.state
    }

    /// Consumes the result and returns its report.
    #[must_use]
    pub fn into_report(self) -> ExecutionReport {
        self.report
    }
}

// ============================================================================
// PIPELINE ORCHESTRATOR
// ============================================================================

/// Coordinates the execution of registered compiler stages.
///
/// The orchestrator is stateless. Each invocation receives its own:
///
/// - CompilationContext.
/// - Stage implementations.
/// - Initial input.
/// - Required-stage set.
/// - Lifecycle tracker.
///
/// It does not maintain process-global stage state.
#[derive(Debug, Default, Clone, Copy)]
pub struct PipelineOrchestrator;

impl PipelineOrchestrator {
    /// Creates a stateless pipeline orchestrator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Executes a registered compiler pipeline.
    ///
    /// The required-stage set is supplied by the higher-level compiler
    /// orchestration layer. This module does not guess which domain stages
    /// are mandatory for a particular source program.
    ///
    /// The lifecycle tracker is created using the cancellation token owned
    /// by CompilationContext. Consequently, context cancellation and
    /// lifecycle cancellation observe the same cancellation state.
    ///
    /// The method starts and terminates the lifecycle itself.
    ///
    /// Stage implementations must consume StageInputs from their StageInput.
    pub fn run(
        &self,
        compilation: &CompilationContext,
        stages: &[Arc<dyn Stage>],
        initial_input: StageInput,
        required_stages: BTreeSet<StageId>,
    ) -> Result<PipelineRun, PipelineOrchestrationError> {
        let mut report = ExecutionReport::new(required_stages);

        // --------------------------------------------------------------------
        // 1. Validate stage implementations and construct ordering.
        // --------------------------------------------------------------------

        let descriptors = stages
            .iter()
            .map(|stage| stage.descriptor())
            .collect::<Vec<_>>();

        let ordering = StageOrdering::build(
            descriptors.iter().copied(),
        )
        .map_err(|error: OrderingError| {
            PipelineOrchestrationError::new(
                PipelineOrchestrationErrorKind::InvalidStageGraph,
                None,
                error.to_string(),
                None,
                report.clone(),
            )
        })?;

        // --------------------------------------------------------------------
        // 2. Validate required-stage membership.
        // --------------------------------------------------------------------

        for required in report.required_stages() {
            if !ordering.contains(required) {
                return Err(
                    PipelineOrchestrationError::new(
                        PipelineOrchestrationErrorKind::MissingRequiredStage,
                        Some(required.clone()),
                        format!(
                            "required stage `{required}` is absent from the registered stage graph"
                        ),
                        None,
                        report,
                    ),
                );
            }
        }

        // --------------------------------------------------------------------
        // 3. Build the implementation registry.
        //
        // StageOrdering has already rejected duplicate identifiers.
        // --------------------------------------------------------------------

        let mut registry = BTreeMap::<StageId, &Arc<dyn Stage>>::new();

        for stage in stages {
            registry.insert(
                stage.descriptor().id().clone(),
                stage,
            );
        }

        // --------------------------------------------------------------------
        // 4. Create one lifecycle tracker sharing the compilation token.
        // --------------------------------------------------------------------

        let state = CompilationStateTracker::with_cancellation(
            compilation.cancellation_token(),
        );

        if let Err(error) = state.start() {
            return Err(
                PipelineOrchestrationError::new(
                    PipelineOrchestrationErrorKind::LifecycleFailure,
                    None,
                    error.to_string(),
                    None,
                    report,
                ),
            );
        }

        // --------------------------------------------------------------------
        // 5. Execute dependency levels in deterministic order.
        //
        // The current implementation is sequential by design.
        // --------------------------------------------------------------------

        for level in ordering.levels() {
            for stage_id in level {
                if let Err(error) = Self::check_active(
                    compilation,
                    &state,
                ) {
                    return Err(Self::terminate_failed_run(
                        &state,
                        report,
                        None,
                        error.kind,
                        error.message,
                        error.stage_error,
                    ));
                }

                let stage = match registry.get(stage_id) {
                    Some(stage) => *stage,

                    None => {
                        return Err(Self::terminate_failed_run(
                            &state,
                            report,
                            Some(stage_id.clone()),
                            PipelineOrchestrationErrorKind::InvalidStageGraph,
                            Arc::from(
                                "ordered stage has no registered implementation",
                            ),
                            None,
                        ));
                    }
                };

                let descriptor = stage.descriptor();

                // ------------------------------------------------------------
                // 5a. Update lifecycle progress.
                // ------------------------------------------------------------

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
                    let kind = if state.is_cancelled()
                        || compilation.is_cancelled()
                    {
                        PipelineOrchestrationErrorKind::Cancelled
                    } else {
                        PipelineOrchestrationErrorKind::LifecycleFailure
                    };

                    return Err(Self::terminate_failed_run(
                        &state,
                        report,
                        Some(stage_id.clone()),
                        kind,
                        Arc::from(error.to_string()),
                        None,
                    ));
                }

                // ------------------------------------------------------------
                // 5b. Verify direct dependencies completed successfully.
                // ------------------------------------------------------------

                let dependencies = match ordering.dependencies_of(stage_id) {
                    Some(dependencies) => dependencies,

                    None => {
                        return Err(Self::terminate_failed_run(
                            &state,
                            report,
                            Some(stage_id.clone()),
                            PipelineOrchestrationErrorKind::InvalidStageGraph,
                            Arc::from(
                                "ordered stage has no dependency metadata",
                            ),
                            None,
                        ));
                    }
                };

                let mut dependency_outputs =
                    BTreeMap::<StageId, StageOutput>::new();

                let mut unsuccessful_dependency = None;

                for dependency in dependencies {
                    match report.record(dependency) {
                        Some(record)
                            if record.is_completed() =>
                        {
                            match report.output_of(dependency) {
                                Some(output) => {
                                    dependency_outputs.insert(
                                        dependency.clone(),
                                        output.clone(),
                                    );
                                }

                                None => {
                                    return Err(
                                        Self::terminate_failed_run(
                                            &state,
                                            report,
                                            Some(stage_id.clone()),
                                            PipelineOrchestrationErrorKind::InvalidStageGraph,
                                            Arc::from(
                                                "completed dependency has no recorded output",
                                            ),
                                            None,
                                        ),
                                    );
                                }
                            }
                        }

                        Some(_) => {
                            unsuccessful_dependency =
                                Some(dependency.clone());

                            break;
                        }

                        None => {
                            return Err(Self::terminate_failed_run(
                                &state,
                                report,
                                Some(stage_id.clone()),
                                PipelineOrchestrationErrorKind::InvalidStageGraph,
                                Arc::from(
                                    "dependency has no execution record",
                                ),
                                None,
                            ));
                        }
                    }
                }

                // ------------------------------------------------------------
                // 5c. Block stages whose prerequisites did not complete.
                // ------------------------------------------------------------

                if let Some(dependency) = unsuccessful_dependency {
                    let reason: Arc<str> = Arc::from(format!(
                        "prerequisite `{dependency}` did not complete successfully"
                    ));

                    report.record(
                        stage_id.clone(),
                        StageExecutionStatus::Blocked {
                            dependency: dependency.clone(),
                            reason: Arc::clone(&reason),
                        },
                    );

                    if report.required_stages().contains(stage_id) {
                        return Err(Self::terminate_failed_run(
                            &state,
                            report,
                            Some(stage_id.clone()),
                            PipelineOrchestrationErrorKind::RequiredStageBlocked,
                            reason,
                            None,
                        ));
                    }

                    continue;
                }

                // ------------------------------------------------------------
                // 5d. Build the canonical stage input bundle.
                // ------------------------------------------------------------

                let stage_inputs = StageInputs::new(
                    initial_input.clone(),
                    dependency_outputs,
                );

                let stage_input = StageInput::new(stage_inputs);

                // ------------------------------------------------------------
                // 5e. Execute the stage through its canonical Stage::run API.
                //
                // Stage::run already performs:
                //
                // - cancellation checking;
                // - conditional execution;
                // - execution;
                // - post-execution cancellation checking.
                // ------------------------------------------------------------

                let outcome = match stage.run(
                    compilation,
                    stage_input,
                ) {
                    Ok(outcome) => outcome,

                    Err(stage_error) => {
                        let kind = if stage_error.kind()
                            == StageErrorKind::Cancelled
                            || compilation.is_cancelled()
                            || state.is_cancelled()
                        {
                            PipelineOrchestrationErrorKind::Cancelled
                        } else {
                            PipelineOrchestrationErrorKind::StageFailed
                        };

                        report.record(
                            stage_id.clone(),
                            StageExecutionStatus::Failed {
                                message: Arc::from(
                                    stage_error.to_string(),
                                ),
                            },
                        );

                        return Err(Self::terminate_failed_run(
                            &state,
                            report,
                            Some(stage_id.clone()),
                            kind,
                            Arc::from(stage_error.to_string()),
                            Some(stage_error),
                        ));
                    }
                };

                // ------------------------------------------------------------
                // 5f. Preserve explicit completion and skip outcomes.
                // ------------------------------------------------------------

                match outcome {
                    StageOutcome::Completed(output) => {
                        report.record_completed(
                            stage_id.clone(),
                            output,
                        );
                    }

                    StageOutcome::Skipped { reason } => {
                        report.record(
                            stage_id.clone(),
                            StageExecutionStatus::Skipped {
                                reason: Arc::clone(&reason),
                            },
                        );

                        if report.required_stages().contains(stage_id) {
                            return Err(Self::terminate_failed_run(
                                &state,
                                report,
                                Some(stage_id.clone()),
                                PipelineOrchestrationErrorKind::RequiredStageSkipped,
                                reason,
                                None,
                            ));
                        }
                    }
                }
            }
        }

        // --------------------------------------------------------------------
        // 6. Verify required stages before declaring successful execution.
        // --------------------------------------------------------------------

        if !report.is_successful() {
            return Err(Self::terminate_failed_run(
                &state,
                report,
                None,
                PipelineOrchestrationErrorKind::IncompleteRequiredWork,
                Arc::from(
                    "one or more required stages did not complete successfully",
                ),
                None,
            ));
        }

        // --------------------------------------------------------------------
        // 7. Final cancellation check.
        //
        // A cancellation request must never be reported as success.
        // --------------------------------------------------------------------

        if let Err(error) = Self::check_active(
            compilation,
            &state,
        ) {
            return Err(Self::terminate_failed_run(
                &state,
                report,
                None,
                error.kind,
                error.message,
                error.stage_error,
            ));
        }

        // --------------------------------------------------------------------
        // 8. Complete the lifecycle.
        //
        // Lifecycle completion is owned by orchestration, not individual
        // stages.
        // --------------------------------------------------------------------

        if let Err(error) = state.complete() {
            let kind = if state.is_cancelled()
                || compilation.is_cancelled()
            {
                PipelineOrchestrationErrorKind::Cancelled
            } else {
                PipelineOrchestrationErrorKind::LifecycleFailure
            };

            return Err(
                PipelineOrchestrationError::new(
                    kind,
                    None,
                    error.to_string(),
                    None,
                    report,
                ),
            );
        }

        Ok(PipelineRun {
            report,
            state,
        })
    }

    // ========================================================================
    // INTERNAL CANCELLATION AND LIFECYCLE HELPERS
    // ========================================================================

    fn check_active(
        compilation: &CompilationContext,
        state: &CompilationStateTracker,
    ) -> Result<(), InternalExecutionError> {
        if let Err(error) = compilation.check_cancelled() {
            return Err(InternalExecutionError {
                kind: PipelineOrchestrationErrorKind::Cancelled,
                message: Arc::from(error.to_string()),
                stage_error: None,
            });
        }

        if let Err(error) = state.check_active() {
            let kind = if state.is_cancelled()
                || compilation.is_cancelled()
            {
                PipelineOrchestrationErrorKind::Cancelled
            } else {
                PipelineOrchestrationErrorKind::LifecycleFailure
            };

            return Err(InternalExecutionError {
                kind,
                message: Arc::from(error.to_string()),
                stage_error: None,
            });
        }

        Ok(())
    }

    fn terminate_failed_run(
        state: &CompilationStateTracker,
        report: ExecutionReport,
        stage: Option<StageId>,
        kind: PipelineOrchestrationErrorKind,
        message: Arc<str>,
        stage_error: Option<StageError>,
    ) -> PipelineOrchestrationError {
        let lifecycle_result = if kind
            == PipelineOrchestrationErrorKind::Cancelled
        {
            state.mark_cancelled()
        } else {
            state.fail(Arc::clone(&message))
        };

        let final_message = match lifecycle_result {
            Ok(()) => message,

            Err(lifecycle_error) => Arc::from(format!(
                "{message}; additionally, lifecycle transition failed: {lifecycle_error}"
            )),
        };

        PipelineOrchestrationError::new(
            kind,
            stage,
            final_message,
            stage_error,
            report,
        )
    }
}

// ============================================================================
// INTERNAL EXECUTION ERROR
// ============================================================================

struct InternalExecutionError {
    kind: PipelineOrchestrationErrorKind,

    message: Arc<str>,

    stage_error: Option<StageError>,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::context::compilation_state::{
        CompilationLifecycle,
        CompilationPhase,
    };

    use crate::compiler::pipeline::stage::{
        StageDescriptor,
        StageDecision,
        StageExecution,
        StageId,
        StageOutcome,
        StageOutput,
    };

    use crate::compiler_types::CompilerConfig;

    struct TestStage {
        descriptor: StageDescriptor,

        skip: bool,

        output_value: usize,
    }

    impl TestStage {
        fn new(
            id: &str,
            dependencies: &[&str],
            skip: bool,
            output_value: usize,
        ) -> Self {
            let mut descriptor = StageDescriptor::new(
                StageId::new(id).expect("valid stage ID"),
                id,
                "1",
                "Pipeline orchestration test stage",
            )
            .expect("valid stage descriptor");

            for dependency in dependencies {
                descriptor = descriptor
                    .with_dependency(
                        StageId::new(*dependency)
                            .expect("valid dependency ID"),
                    )
                    .expect("valid dependency");
            }

            descriptor = descriptor.with_phase(
                CompilationPhase::SemanticAnalysis,
            );

            Self {
                descriptor,
                skip,
                output_value,
            }
        }
    }

    impl Stage for TestStage {
        fn descriptor(&self) -> &StageDescriptor {
            &self.descriptor
        }

        fn should_execute(
            &self,
            _context: &crate::compiler::pipeline::stage::StageExecutionContext<'_>,
            _input: &StageInput,
        ) -> Result<StageDecision, StageError> {
            if self.skip {
                StageDecision::skip("skipped by test configuration")
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
                        "expected StageInputs",
                    )
                })?;

            // Verify that the fan-in contract is available.
            let _initial_input = inputs.initial_input();

            Ok(StageOutput::new(self.output_value))
        }
    }

    fn context() -> CompilationContext {
        CompilationContext::new(
            CompilerConfig::default(),
        )
        .expect("valid compilation context")
    }

    fn stage(
        id: &str,
        dependencies: &[&str],
        skip: bool,
        output: usize,
    ) -> Arc<dyn Stage> {
        Arc::new(TestStage::new(
            id,
            dependencies,
            skip,
            output,
        ))
    }

    fn id(value: &str) -> StageId {
        StageId::new(value).expect("valid stage ID")
    }

    #[test]
    fn executes_dependency_ordered_stages() {
        let stages = vec![
            stage("frontend", &[], false, 10),
            stage("semantic", &["frontend"], false, 20),
            stage("ir", &["semantic"], false, 30),
        ];

        let required = BTreeSet::from([
            id("frontend"),
            id("semantic"),
            id("ir"),
        ]);

        let result = PipelineOrchestrator::new()
            .run(
                &context(),
                &stages,
                StageInput::empty(),
                required,
            )
            .expect("pipeline should complete");

        assert!(result.report().is_successful());

        assert_eq!(
            result.report().completed_count(),
            3,
        );

        assert_eq!(
            result.report().stage_count(),
            3,
        );

        assert_eq!(
            result.state()
                .lifecycle()
                .expect("lifecycle should be readable"),
            CompilationLifecycle::Completed,
        );
    }

    #[test]
    fn executes_fan_in_dependencies() {
        let stages = vec![
            stage("classical", &[], false, 10),
            stage("quantum", &[], false, 20),
            stage(
                "hybrid",
                &["classical", "quantum"],
                false,
                30,
            ),
        ];

        let required = BTreeSet::from([
            id("classical"),
            id("quantum"),
            id("hybrid"),
        ]);

        let result = PipelineOrchestrator::new()
            .run(
                &context(),
                &stages,
                StageInput::empty(),
                required,
            )
            .expect("hybrid fan-in should complete");

        assert!(result.report().is_successful());

        let hybrid_inputs = StageInputs::new(
            StageInput::empty(),
            BTreeMap::from([
                (id("classical"), StageOutput::new(10usize)),
                (id("quantum"), StageOutput::new(20usize)),
            ]),
        );

        assert_eq!(
            hybrid_inputs
                .dependency_output(&id("classical"))
                .and_then(|output| output.downcast_ref::<usize>()),
            Some(&10usize),
        );

        assert_eq!(
            hybrid_inputs
                .dependency_output(&id("quantum"))
                .and_then(|output| output.downcast_ref::<usize>()),
            Some(&20usize),
        );
    }

    #[test]
    fn optional_skipped_stage_is_not_reported_as_completed() {
        let stages = vec![
            stage("frontend", &[], false, 10),
            stage("optional_quantum", &["frontend"], true, 20),
        ];

        let required = BTreeSet::from([
            id("frontend"),
        ]);

        let result = PipelineOrchestrator::new()
            .run(
                &context(),
                &stages,
                StageInput::empty(),
                required,
            )
            .expect("optional skip should be permitted");

        assert!(result.report().is_successful());

        assert_eq!(
            result.report().skipped_count(),
            1,
        );

        assert!(
            !result.report()
                .record(&id("optional_quantum"))
                .expect("skip record")
                .is_completed()
        );
    }

    #[test]
    fn dependent_of_skipped_stage_is_blocked() {
        let stages = vec![
            stage("frontend", &[], false, 10),
            stage("optional_domain", &["frontend"], true, 20),
            stage("dependent", &["optional_domain"], false, 30),
        ];

        let required = BTreeSet::from([
            id("frontend"),
        ]);

        let result = PipelineOrchestrator::new()
            .run(
                &context(),
                &stages,
                StageInput::empty(),
                required,
            )
            .expect("optional blocked branch should be permitted");

        assert_eq!(
            result.report().skipped_count(),
            1,
        );

        assert_eq!(
            result.report().blocked_count(),
            1,
        );

        assert!(result.report().is_successful());
    }

    #[test]
    fn required_skipped_stage_fails_pipeline() {
        let stages = vec![
            stage("frontend", &[], true, 10),
        ];

        let required = BTreeSet::from([
            id("frontend"),
        ]);

        let error = PipelineOrchestrator::new()
            .run(
                &context(),
                &stages,
                StageInput::empty(),
                required,
            )
            .expect_err("required skipped stage must fail");

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::RequiredStageSkipped,
        );

        assert_eq!(
            error.partial_report().skipped_count(),
            1,
        );
    }

    #[test]
    fn required_blocked_stage_fails_pipeline() {
        let stages = vec![
            stage("frontend", &[], true, 10),
            stage("semantic", &["frontend"], false, 20),
        ];

        let required = BTreeSet::from([
            id("semantic"),
        ]);

        let error = PipelineOrchestrator::new()
            .run(
                &context(),
                &stages,
                StageInput::empty(),
                required,
            )
            .expect_err("required blocked stage must fail");

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::RequiredStageBlocked,
        );
    }

    #[test]
    fn missing_required_stage_is_rejected() {
        let stages = vec![
            stage("frontend", &[], false, 10),
        ];

        let required = BTreeSet::from([
            id("semantic"),
        ]);

        let error = PipelineOrchestrator::new()
            .run(
                &context(),
                &stages,
                StageInput::empty(),
                required,
            )
            .expect_err("missing required stage must fail");

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::MissingRequiredStage,
        );
    }

    #[test]
    fn empty_stage_graph_can_represent_empty_work() {
        let stages: Vec<Arc<dyn Stage>> = Vec::new();

        let result = PipelineOrchestrator::new()
            .run(
                &context(),
                &stages,
                StageInput::empty(),
                BTreeSet::new(),
            )
            .expect("empty work graph should complete");

        assert!(result.report().is_successful());

        assert_eq!(
            result.report().stage_count(),
            0,
        );
    }

    #[test]
    fn duplicate_stage_identifiers_are_rejected() {
        let stages = vec![
            stage("frontend", &[], false, 10),
            stage("frontend", &[], false, 20),
        ];

        let error = PipelineOrchestrator::new()
            .run(
                &context(),
                &stages,
                StageInput::empty(),
                BTreeSet::new(),
            )
            .expect_err("duplicate IDs must be rejected");

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::InvalidStageGraph,
        );
    }

    #[test]
    fn dependency_cycle_is_rejected() {
        let stages = vec![
            stage("a", &["b"], false, 10),
            stage("b", &["a"], false, 20),
        ];

        let error = PipelineOrchestrator::new()
            .run(
                &context(),
                &stages,
                StageInput::empty(),
                BTreeSet::new(),
            )
            .expect_err("dependency cycle must be rejected");

        assert_eq!(
            error.kind(),
            PipelineOrchestrationErrorKind::InvalidStageGraph,
        );
    }

    #[test]
    fn stage_output_is_preserved_in_report() {
        let stages = vec![
            stage("frontend", &[], false, 42),
        ];

        let required = BTreeSet::from([
            id("frontend"),
        ]);

        let result = PipelineOrchestrator::new()
            .run(
                &context(),
                &stages,
                StageInput::empty(),
                required,
            )
            .expect("pipeline should complete");

        assert_eq!(
            result.report()
                .output_of(&id("frontend"))
                .and_then(|output| output.downcast_ref::<usize>()),
            Some(&42usize),
        );
    }
}
