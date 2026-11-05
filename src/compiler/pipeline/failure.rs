
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Pipeline Failure Management.
//!
//! Path:
//!     src/compiler/pipeline/failure.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: Rust 1.97
//!
//! Safety:
//!     Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Provides a structured, deterministic and extensible representation of
//! compiler pipeline failures.
//!
//! This module integrates existing failures from:
//!
//!     pipeline::stage
//!     pipeline::execution
//!     pipeline::orchestration
//!     pipeline::ordering
//!
//! It preserves:
//!
//! - Original typed stage errors.
//! - Failure classifications.
//! - Failure origins.
//! - Stage identities.
//! - Partial execution information.
//! - Required-stage information.
//! - Failure severity.
//! - Retryability declarations.
//! - Stable diagnostic codes.
//!
//! This module does not execute recovery.
//!
//! Recovery policies and recovery execution belong to pipeline::recovery.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - PipelineFailure.
//! - FailureCode.
//! - FailureCategory.
//! - FailureSeverity.
//! - FailureOrigin.
//! - FailureDisposition.
//! - RetryDeclaration.
//! - FailureStageStatus.
//! - FailureStageRecord.
//! - FailureReport.
//! - FailureRelation.
//! - FailureContext.
//! - Conversion of existing pipeline failures.
//! - Conversion into existing compiler diagnostics.
//!
//! This module does NOT own:
//!
//! - StageError.
//! - ExecutionError.
//! - PipelineOrchestrationError.
//! - OrderingError.
//! - CompilationStateTracker.
//! - CompilationContext.
//! - CancellationToken.
//! - CompilationBudget.
//! - Retry policy.
//! - Backoff.
//! - Recovery execution.
//! - Compiler semantic analysis.
//! - Canonical IR.
//! - Backend implementations.
//! - Runtime execution.
//!
//! ============================================================================
//! FAILURE PROPAGATION
//! ============================================================================
//!
//! Stage execution
//!       |
//!       v
//! Typed StageError
//!       |
//!       v
//! ExecutionError / PipelineOrchestrationError
//!       |
//!       v
//! PipelineFailure
//!       |
//!       +----> Diagnostics
//!       |
//!       +----> Recovery policy
//!       |
//!       +----> Observability
//!       |
//!       +----> Compilation result
//!
//! A failure is never converted into successful compilation.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! No universal limits are imposed on:
//!
//! - Stages.
//! - Failure records.
//! - Dependencies.
//! - Computing devices.
//! - CPUs.
//! - GPUs.
//! - QPUs.
//! - FPGAs.
//! - Distributed nodes.
//! - Memory.
//! - IR operations.
//! - Compilation units.
//!
//! Collections grow according to available resources.
//!
//! Allocation failures remain possible and are not represented as
//! successful compilation.
//!
//! Operational limits are owned by CompilationContext and CompilationBudget.
//!
//! ============================================================================
//! GRAMMAR BOUNDARY
//! ============================================================================
//!
//! The grammar describes source-level compilation intent.
//!
//! This module describes compiler implementation failures.
//!
//! Source-language errors must retain their original semantic ownership.
//!
//! A target infeasibility must not be misreported as a grammar failure.
//!
//! A missing hardware capability must not be silently converted into
//! a successful fallback.
//!
//! ============================================================================

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::compiler::diagnostics::{
    Diagnostic,
    DiagnosticLevel,
};

use crate::compiler::pipeline::execution::{
    ExecutionError,
    ExecutionErrorKind,
    ExecutionReport as ExecutionExecutionReport,
    StageExecutionStatus as ExecutionStageStatus,
};

use crate::compiler::pipeline::orchestration::{
    PipelineOrchestrationError,
    PipelineOrchestrationErrorKind,
    ExecutionReport as OrchestrationExecutionReport,
    StageExecutionStatus as OrchestrationStageStatus,
};

use crate::compiler::pipeline::ordering::OrderingError;

use crate::compiler::pipeline::stage::{
    StageError,
    StageErrorKind,
    StageId,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version for pipeline failure management.
///
/// Independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Compiler release version.
/// - Semantic model version.
/// - AST version.
/// - IR version.
/// - Backend ABI version.
pub const PIPELINE_FAILURE_API_VERSION: u16 = 1;

// ============================================================================
// STABLE FAILURE CODES
// ============================================================================

/// Stable machine-readable pipeline failure codes.
///
/// These codes are independent of human-readable error messages.
///
/// Adding new variants is backward-compatible for callers that handle
/// unknown codes through the string representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum FailureCode {
    StageExecution,
    StageCancelled,
    StageBudgetExceeded,
    StageInvalidInput,
    StageUnsupportedOperation,
    StageIncompatibleConfiguration,
    StageInvalidDecision,
    StageExecutionContext,
    StageInternalInvariant,

    DuplicateStage,
    MissingStageImplementation,
    UnexpectedStageImplementation,
    OrderingMismatch,
    MissingRequiredStage,
    RequiredStageSkipped,
    DependencyNotCompleted,
    PipelineStageFailed,
    PipelineCancelled,
    PipelineStateTracking,
    PipelineInternalInvariant,

    InvalidStageGraph,
    RequiredStageBlocked,
    LifecycleFailure,
    IncompleteRequiredWork,

    OrderingDuplicateStage,
    OrderingMissingDependency,
    OrderingCycle,
    OrderingInternalInvariant,

    ExternalFailure,
}

impl FailureCode {
    /// Returns the stable machine-readable code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StageExecution => "ZC-PIPE-001",
            Self::StageCancelled => "ZC-PIPE-002",
            Self::StageBudgetExceeded => "ZC-PIPE-003",
            Self::StageInvalidInput => "ZC-PIPE-004",
            Self::StageUnsupportedOperation => "ZC-PIPE-005",
            Self::StageIncompatibleConfiguration => "ZC-PIPE-006",
            Self::StageInvalidDecision => "ZC-PIPE-007",
            Self::StageExecutionContext => "ZC-PIPE-008",
            Self::StageInternalInvariant => "ZC-PIPE-009",

            Self::DuplicateStage => "ZC-PIPE-010",
            Self::MissingStageImplementation => "ZC-PIPE-011",
            Self::UnexpectedStageImplementation => "ZC-PIPE-012",
            Self::OrderingMismatch => "ZC-PIPE-013",
            Self::MissingRequiredStage => "ZC-PIPE-014",
            Self::RequiredStageSkipped => "ZC-PIPE-015",
            Self::DependencyNotCompleted => "ZC-PIPE-016",
            Self::PipelineStageFailed => "ZC-PIPE-017",
            Self::PipelineCancelled => "ZC-PIPE-018",
            Self::PipelineStateTracking => "ZC-PIPE-019",
            Self::PipelineInternalInvariant => "ZC-PIPE-020",

            Self::InvalidStageGraph => "ZC-PIPE-021",
            Self::RequiredStageBlocked => "ZC-PIPE-022",
            Self::LifecycleFailure => "ZC-PIPE-023",
            Self::IncompleteRequiredWork => "ZC-PIPE-024",

            Self::OrderingDuplicateStage => "ZC-PIPE-025",
            Self::OrderingMissingDependency => "ZC-PIPE-026",
            Self::OrderingCycle => "ZC-PIPE-027",
            Self::OrderingInternalInvariant => "ZC-PIPE-028",

            Self::ExternalFailure => "ZC-PIPE-999",
        }
    }
}

impl fmt::Display for FailureCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// FAILURE CATEGORY
// ============================================================================

/// Broad classification of pipeline failures.
///
/// Categories allow diagnostics, recovery policies and monitoring systems
/// to process failures without depending on human-readable messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum FailureCategory {
    StageExecution,
    Configuration,
    Dependency,
    Ordering,
    Resource,
    Cancellation,
    Verification,
    UnsupportedOperation,
    Lifecycle,
    Internal,
    External,
}

impl fmt::Display for FailureCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::StageExecution => "stage execution",
            Self::Configuration => "configuration",
            Self::Dependency => "dependency",
            Self::Ordering => "ordering",
            Self::Resource => "resource",
            Self::Cancellation => "cancellation",
            Self::Verification => "verification",
            Self::UnsupportedOperation => "unsupported operation",
            Self::Lifecycle => "lifecycle",
            Self::Internal => "internal",
            Self::External => "external",
        })
    }
}

// ============================================================================
// FAILURE SEVERITY
// ============================================================================

/// Describes the operational severity of a pipeline failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum FailureSeverity {
    /// Informational condition that did not prevent required work.
    Information,

    /// Recoverable or non-terminal problem.
    Warning,

    /// A compilation operation cannot currently complete.
    Error,

    /// An internal invariant or correctness guarantee was violated.
    Critical,
}

// ============================================================================
// FAILURE ORIGIN
// ============================================================================

/// Identifies the subsystem that originally reported the failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FailureOrigin {
    Stage,
    Execution,
    Orchestration,
    Ordering,
    External,
}

// ============================================================================
// FAILURE DISPOSITION
// ============================================================================

/// Describes the outcome that the pipeline must preserve.
///
/// This does not perform recovery or change lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FailureDisposition {
    /// Compilation must not be reported as successful.
    Terminal,

    /// Compilation was cancelled.
    Cancelled,

    /// Required work was not completed.
    Incomplete,

    /// An internal implementation invariant was violated.
    InternalFailure,

    /// The failure is recorded but does not independently establish
    /// that required compilation work failed.
    NonTerminal,
}

// ============================================================================
// RETRY DECLARATION
// ============================================================================

/// Reports whether the original failure source declared retryability.
///
/// This is deliberately NOT a retry policy.
///
/// The recovery subsystem decides whether retrying is legal after
/// considering idempotency, resource availability, provenance and
/// execution context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RetryDeclaration {
    NotDeclared,

    DeclaredByStage,
}

impl RetryDeclaration {
    /// Returns whether the original stage declared retryability.
    #[must_use]
    pub const fn is_declared_retryable(self) -> bool {
        matches!(self, Self::DeclaredByStage)
    }
}

// ============================================================================
// STAGE EXECUTION SNAPSHOT
// ============================================================================

/// Normalized stage status retained in a failure report.
///
/// This is a snapshot, not another execution-state authority.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FailureStageStatus {
    Completed,

    Skipped {
        reason: Arc<str>,
    },

    Blocked {
        reason: Arc<str>,
    },

    Failed {
        message: Arc<str>,
    },

    NotStarted,
}

/// Immutable record of one stage's state at failure time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailureStageRecord {
    stage_id: StageId,
    status: FailureStageStatus,
}

impl FailureStageRecord {
    /// Creates a failure-stage snapshot.
    #[must_use]
    pub fn new(
        stage_id: StageId,
        status: FailureStageStatus,
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

    /// Returns the captured stage status.
    #[must_use]
    pub fn status(&self) -> &FailureStageStatus {
        &self.status
    }

    /// Returns whether the stage completed successfully.
    #[must_use]
    pub fn is_completed(&self) -> bool {
        matches!(self.status, FailureStageStatus::Completed)
    }
}

// ============================================================================
// FAILURE REPORT
// ============================================================================

/// Immutable summary of the pipeline execution state at failure time.
///
/// The original execution reports remain owned by their respective
/// execution/orchestration subsystems.
///
/// This report copies only the information needed for failure analysis.
#[derive(Debug, Clone, Default)]
pub struct FailureReport {
    records: BTreeMap<StageId, FailureStageRecord>,

    required_stages: BTreeSet<StageId>,
}

impl FailureReport {
    /// Creates an empty failure report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the captured stage records.
    #[must_use]
    pub fn records(&self) -> &BTreeMap<StageId, FailureStageRecord> {
        &self.records
    }

    /// Returns the mandatory stages.
    #[must_use]
    pub fn required_stages(&self) -> &BTreeSet<StageId> {
        &self.required_stages
    }

    /// Returns the number of captured stages.
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
                    FailureStageStatus::Skipped { .. }
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
                    FailureStageStatus::Blocked { .. }
                )
            })
            .count()
    }

    /// Returns the number of explicitly failed stages.
    #[must_use]
    pub fn failed_count(&self) -> usize {
        self.records
            .values()
            .filter(|record| {
                matches!(
                    record.status(),
                    FailureStageStatus::Failed { .. }
                )
            })
            .count()
    }

    /// Returns whether every required stage has a completed record.
    ///
    /// Missing records do not count as completed.
    #[must_use]
    pub fn all_required_stages_completed(&self) -> bool {
        self.required_stages.iter().all(|required| {
            self.records
                .get(required)
                .is_some_and(FailureStageRecord::is_completed)
        })
    }

    /// Returns the record for a stage.
    #[must_use]
    pub fn record(
        &self,
        stage_id: &StageId,
    ) -> Option<&FailureStageRecord> {
        self.records.get(stage_id)
    }

    /// Captures an execution report without taking ownership of it.
    #[must_use]
    pub fn from_execution(
        report: &ExecutionExecutionReport,
    ) -> Self {
        let mut result = Self {
            records: BTreeMap::new(),
            required_stages: report.required_stages().clone(),
        };

        for (stage_id, record) in report.records() {
            let status = match record.status() {
                ExecutionStageStatus::Completed => {
                    FailureStageStatus::Completed
                }

                ExecutionStageStatus::Skipped => {
                    FailureStageStatus::Skipped {
                        reason: Arc::from(
                            record.reason().unwrap_or(
                                "stage was skipped",
                            ),
                        ),
                    }
                }

                ExecutionStageStatus::Blocked => {
                    FailureStageStatus::Blocked {
                        reason: Arc::from(
                            record.reason().unwrap_or(
                                "stage was blocked",
                            ),
                        ),
                    }
                }
            };

            result.records.insert(
                stage_id.clone(),
                FailureStageRecord::new(
                    stage_id.clone(),
                    status,
                ),
            );
        }

        result
    }

    /// Captures an orchestration report without taking ownership of it.
    #[must_use]
    pub fn from_orchestration(
        report: &OrchestrationExecutionReport,
    ) -> Self {
        let mut result = Self {
            records: BTreeMap::new(),
            required_stages: report.required_stages().clone(),
        };

        for (stage_id, record) in report.records() {
            let status = match record.status() {
                OrchestrationStageStatus::Completed => {
                    FailureStageStatus::Completed
                }

                OrchestrationStageStatus::Skipped { reason } => {
                    FailureStageStatus::Skipped {
                        reason: reason.clone(),
                    }
                }

                OrchestrationStageStatus::Blocked {
                    dependency,
                    reason,
                } => {
                    FailureStageStatus::Blocked {
                        reason: Arc::from(
                            format!(
                                "dependency `{dependency}`: {reason}"
                            ),
                        ),
                    }
                }

                OrchestrationStageStatus::Failed { message } => {
                    FailureStageStatus::Failed {
                        message: message.clone(),
                    }
                }
            };

            result.records.insert(
                stage_id.clone(),
                FailureStageRecord::new(
                    stage_id.clone(),
                    status,
                ),
            );
        }

        result
    }
}

// ============================================================================
// FAILURE RELATIONS
// ============================================================================

/// Relationship between a failure and another affected stage.
///
/// This describes causality or propagation without creating another
/// dependency graph.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FailureRelation {
    /// The stage directly caused the failure.
    CausedBy {
        stage: StageId,
    },

    /// The stage could not execute because another stage failed.
    BlockedBy {
        stage: StageId,
        dependency: StageId,
    },

    /// The stage is affected by an upstream failure.
    AffectedBy {
        stage: StageId,
        upstream: StageId,
    },
}

// ============================================================================
// FAILURE CONTEXT
// ============================================================================

/// Optional contextual information attached to a pipeline failure.
#[derive(Debug, Clone, Default)]
pub struct FailureContext {
    compilation_id: Option<Arc<str>>,

    operation: Option<Arc<str>>,

    stage: Option<StageId>,

    relations: Vec<FailureRelation>,
}

impl FailureContext {
    /// Creates an empty context.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Associates a compilation identifier.
    ///
    /// The identifier is an opaque logical identifier.
    #[must_use]
    pub fn with_compilation_id(
        mut self,
        value: impl Into<Arc<str>>,
    ) -> Self {
        self.compilation_id = Some(value.into());
        self
    }

    /// Associates an operation name.
    #[must_use]
    pub fn with_operation(
        mut self,
        value: impl Into<Arc<str>>,
    ) -> Self {
        self.operation = Some(value.into());
        self
    }

    /// Associates a stage.
    #[must_use]
    pub fn with_stage(
        mut self,
        stage: StageId,
    ) -> Self {
        self.stage = Some(stage);
        self
    }

    /// Adds a failure relationship.
    #[must_use]
    pub fn with_relation(
        mut self,
        relation: FailureRelation,
    ) -> Self {
        self.relations.push(relation);
        self
    }

    /// Returns the compilation identifier.
    #[must_use]
    pub fn compilation_id(&self) -> Option<&str> {
        self.compilation_id.as_deref()
    }

    /// Returns the operation name.
    #[must_use]
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Returns the associated stage.
    #[must_use]
    pub fn stage(&self) -> Option<&StageId> {
        self.stage.as_ref()
    }

    /// Returns the failure relationships.
    #[must_use]
    pub fn relations(&self) -> &[FailureRelation] {
        &self.relations
    }
}

// ============================================================================
// PIPELINE FAILURE
// ============================================================================

/// Canonical structured representation of a pipeline failure.
///
/// This type preserves the original stage error whenever one exists.
///
/// It does not replace the originating error type.
#[derive(Debug, Clone)]
pub struct PipelineFailure {
    code: FailureCode,

    category: FailureCategory,

    severity: FailureSeverity,

    origin: FailureOrigin,

    disposition: FailureDisposition,

    retry_declaration: RetryDeclaration,

    stage: Option<StageId>,

    message: Arc<str>,

    stage_cause: Option<StageError>,

    report: FailureReport,

    context: FailureContext,
}

impl PipelineFailure {
    /// Creates a structured failure from a stage error.
    #[must_use]
    pub fn from_stage(
        stage: StageId,
        cause: StageError,
    ) -> Self {
        let code = code_from_stage(cause.kind());

        let category = category_from_stage(cause.kind());

        let severity = severity_from_stage(cause.kind());

        let disposition = disposition_from_stage(cause.kind());

        let retry_declaration = if cause.is_retryable() {
            RetryDeclaration::DeclaredByStage
        } else {
            RetryDeclaration::NotDeclared
        };

        Self {
            code,
            category,
            severity,
            origin: FailureOrigin::Stage,
            disposition,
            retry_declaration,
            context: FailureContext::new()
                .with_stage(stage.clone()),
            stage: Some(stage),
            message: Arc::from(cause.message()),
            stage_cause: Some(cause),
            report: FailureReport::new(),
        }
    }

    /// Creates a structured failure from an execution error.
    ///
    /// The partial report is retained as a normalized snapshot.
    #[must_use]
    pub fn from_execution(
        error: &ExecutionError,
    ) -> Self {
        let stage = error.stage().cloned();

        let cause = error.stage_cause().cloned();

        let (code, category, severity, disposition) =
            classify_execution(error.kind());

        let retry_declaration = cause
            .as_ref()
            .map(|cause| {
                if cause.is_retryable() {
                    RetryDeclaration::DeclaredByStage
                } else {
                    RetryDeclaration::NotDeclared
                }
            })
            .unwrap_or(RetryDeclaration::NotDeclared);

        let mut context = FailureContext::new();

        if let Some(stage_id) = stage.as_ref() {
            context = context.with_stage(stage_id.clone());
        }

        Self {
            code,
            category,
            severity,
            origin: FailureOrigin::Execution,
            disposition,
            retry_declaration,
            stage,
            message: Arc::from(error.message()),
            stage_cause: cause,
            report: FailureReport::from_execution(
                error.partial_report(),
            ),
            context,
        }
    }

    /// Creates a structured failure from an orchestration error.
    ///
    /// The original typed StageError is retained whenever supplied by
    /// the orchestration subsystem.
    #[must_use]
    pub fn from_orchestration(
        error: &PipelineOrchestrationError,
    ) -> Self {
        let stage = error.stage().cloned();

        let cause = error.stage_error().cloned();

        let (code, category, severity, disposition) =
            classify_orchestration(error.kind());

        let retry_declaration = cause
            .as_ref()
            .map(|cause| {
                if cause.is_retryable() {
                    RetryDeclaration::DeclaredByStage
                } else {
                    RetryDeclaration::NotDeclared
                }
            })
            .unwrap_or(RetryDeclaration::NotDeclared);

        let mut context = FailureContext::new();

        if let Some(stage_id) = stage.as_ref() {
            context = context.with_stage(stage_id.clone());
        }

        Self {
            code,
            category,
            severity,
            origin: FailureOrigin::Orchestration,
            disposition,
            retry_declaration,
            stage,
            message: Arc::from(error.message()),
            stage_cause: cause,
            report: FailureReport::from_orchestration(
                error.partial_report(),
            ),
            context,
        }
    }

    /// Creates a failure from an ordering error.
    #[must_use]
    pub fn from_ordering(
        error: &OrderingError,
    ) -> Self {
        let (code, category, severity, disposition) =
            classify_ordering(error);

        Self {
            code,
            category,
            severity,
            origin: FailureOrigin::Ordering,
            disposition,
            retry_declaration: RetryDeclaration::NotDeclared,
            stage: None,
            message: Arc::from(error.to_string()),
            stage_cause: None,
            report: FailureReport::new(),
            context: FailureContext::new(),
        }
    }

    /// Creates a structured external pipeline failure.
    ///
    /// The external subsystem remains responsible for its own error type.
    #[must_use]
    pub fn external(
        message: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            code: FailureCode::ExternalFailure,
            category: FailureCategory::External,
            severity: FailureSeverity::Error,
            origin: FailureOrigin::External,
            disposition: FailureDisposition::Terminal,
            retry_declaration: RetryDeclaration::NotDeclared,
            stage: None,
            message: message.into(),
            stage_cause: None,
            report: FailureReport::new(),
            context: FailureContext::new(),
        }
    }

    /// Attaches additional contextual information.
    #[must_use]
    pub fn with_context(
        mut self,
        context: FailureContext,
    ) -> Self {
        if let Some(stage) = context.stage() {
            self.stage = Some(stage.clone());
        }

        self.context = context;

        self
    }

    /// Returns the stable failure code.
    #[must_use]
    pub const fn code(&self) -> FailureCode {
        self.code
    }

    /// Returns the stable machine-readable code string.
    #[must_use]
    pub const fn code_str(&self) -> &'static str {
        self.code.as_str()
    }

    /// Returns the failure category.
    #[must_use]
    pub const fn category(&self) -> FailureCategory {
        self.category
    }

    /// Returns the failure severity.
    #[must_use]
    pub const fn severity(&self) -> FailureSeverity {
        self.severity
    }

    /// Returns the original failure subsystem.
    #[must_use]
    pub const fn origin(&self) -> FailureOrigin {
        self.origin
    }

    /// Returns the required failure disposition.
    #[must_use]
    pub const fn disposition(&self) -> FailureDisposition {
        self.disposition
    }

    /// Returns the retryability declaration.
    ///
    /// This does not authorize retrying.
    #[must_use]
    pub const fn retry_declaration(&self) -> RetryDeclaration {
        self.retry_declaration
    }

    /// Returns whether the original stage declared retryability.
    ///
    /// Recovery policy must still independently authorize a retry.
    #[must_use]
    pub const fn is_declared_retryable(&self) -> bool {
        self.retry_declaration.is_declared_retryable()
    }

    /// Returns the affected stage.
    #[must_use]
    pub fn stage(&self) -> Option<&StageId> {
        self.stage.as_ref()
    }

    /// Returns the failure message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the original stage error.
    #[must_use]
    pub fn stage_cause(&self) -> Option<&StageError> {
        self.stage_cause.as_ref()
    }

    /// Returns the normalized partial execution report.
    #[must_use]
    pub fn report(&self) -> &FailureReport {
        &self.report
    }

    /// Returns the additional failure context.
    #[must_use]
    pub fn context(&self) -> &FailureContext {
        &self.context
    }

    /// Returns whether the failure represents cancellation.
    #[must_use]
    pub const fn is_cancelled(&self) -> bool {
        matches!(
            self.disposition,
            FailureDisposition::Cancelled
        )
    }

    /// Returns whether compilation must not be reported as successful.
    #[must_use]
    pub const fn prevents_success(&self) -> bool {
        matches!(
            self.disposition,
            FailureDisposition::Terminal
                | FailureDisposition::Cancelled
                | FailureDisposition::Incomplete
                | FailureDisposition::InternalFailure
        )
    }

    /// Converts the failure into the existing compiler diagnostic type.
    ///
    /// No diagnostic engine is created or duplicated.
    #[must_use]
    pub fn to_diagnostic(&self) -> Diagnostic {
        let mut message = String::new();

        if let Some(stage) = self.stage.as_ref() {
            message.push_str("stage `");
            message.push_str(stage.as_str());
            message.push_str("`: ");
        }

        message.push_str(self.message());

        let diagnostic = Diagnostic::new(
            diagnostic_level(self.severity),
            self.code_str(),
            message,
        );

        if let Some(operation) = self.context.operation() {
            diagnostic.with_note(
                format!("operation: {operation}"),
            )
        } else {
            diagnostic
        }
    }
}

impl fmt::Display for PipelineFailure {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "[{}] {} failure",
            self.code,
            self.category,
        )?;

        if let Some(stage) = self.stage.as_ref() {
            write!(formatter, " at stage `{stage}`")?;
        }

        write!(formatter, ": {}", self.message)
    }
}

impl Error for PipelineFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.stage_cause
            .as_ref()
            .map(|cause| cause as &(dyn Error + 'static))
    }
}

// ============================================================================
// CLASSIFICATION HELPERS
// ============================================================================

fn code_from_stage(kind: StageErrorKind) -> FailureCode {
    match kind {
        StageErrorKind::Execution => FailureCode::StageExecution,

        StageErrorKind::Cancelled => FailureCode::StageCancelled,

        StageErrorKind::BudgetExceeded => {
            FailureCode::StageBudgetExceeded
        }

        StageErrorKind::InvalidInput => {
            FailureCode::StageInvalidInput
        }

        StageErrorKind::UnsupportedOperation => {
            FailureCode::StageUnsupportedOperation
        }

        StageErrorKind::IncompatibleConfiguration => {
            FailureCode::StageIncompatibleConfiguration
        }

        StageErrorKind::InvalidDecision => {
            FailureCode::StageInvalidDecision
        }

        StageErrorKind::ExecutionContext => {
            FailureCode::StageExecutionContext
        }

        StageErrorKind::InternalInvariant => {
            FailureCode::StageInternalInvariant
        }
    }
}

fn category_from_stage(kind: StageErrorKind) -> FailureCategory {
    match kind {
        StageErrorKind::Execution => FailureCategory::StageExecution,

        StageErrorKind::Cancelled => FailureCategory::Cancellation,

        StageErrorKind::BudgetExceeded => FailureCategory::Resource,

        StageErrorKind::InvalidInput => FailureCategory::Configuration,

        StageErrorKind::UnsupportedOperation => {
            FailureCategory::UnsupportedOperation
        }

        StageErrorKind::IncompatibleConfiguration => {
            FailureCategory::Configuration
        }

        StageErrorKind::InvalidDecision => FailureCategory::StageExecution,

        StageErrorKind::ExecutionContext => FailureCategory::Lifecycle,

        StageErrorKind::InternalInvariant => FailureCategory::Internal,
    }
}

fn severity_from_stage(kind: StageErrorKind) -> FailureSeverity {
    match kind {
        StageErrorKind::InternalInvariant => FailureSeverity::Critical,

        StageErrorKind::ExecutionContext => FailureSeverity::Critical,

        StageErrorKind::Cancelled => FailureSeverity::Warning,

        _ => FailureSeverity::Error,
    }
}

fn disposition_from_stage(kind: StageErrorKind) -> FailureDisposition {
    match kind {
        StageErrorKind::Cancelled => FailureDisposition::Cancelled,

        StageErrorKind::InternalInvariant => {
            FailureDisposition::InternalFailure
        }

        _ => FailureDisposition::Terminal,
    }
}

fn classify_execution(
    kind: ExecutionErrorKind,
) -> (
    FailureCode,
    FailureCategory,
    FailureSeverity,
    FailureDisposition,
) {
    use ExecutionErrorKind::*;

    match kind {
        DuplicateStageImplementation => (
            FailureCode::DuplicateStage,
            FailureCategory::Ordering,
            FailureSeverity::Critical,
            FailureDisposition::InternalFailure,
        ),

        MissingStageImplementation => (
            FailureCode::MissingStageImplementation,
            FailureCategory::Configuration,
            FailureSeverity::Error,
            FailureDisposition::Terminal,
        ),

        UnexpectedStageImplementation => (
            FailureCode::UnexpectedStageImplementation,
            FailureCategory::Configuration,
            FailureSeverity::Error,
            FailureDisposition::Terminal,
        ),

        OrderingMismatch => (
            FailureCode::OrderingMismatch,
            FailureCategory::Ordering,
            FailureSeverity::Critical,
            FailureDisposition::InternalFailure,
        ),

        MissingRequiredStage => (
            FailureCode::MissingRequiredStage,
            FailureCategory::Dependency,
            FailureSeverity::Error,
            FailureDisposition::Incomplete,
        ),

        RequiredStageSkipped => (
            FailureCode::RequiredStageSkipped,
            FailureCategory::Dependency,
            FailureSeverity::Error,
            FailureDisposition::Incomplete,
        ),

        DependencyNotCompleted => (
            FailureCode::DependencyNotCompleted,
            FailureCategory::Dependency,
            FailureSeverity::Error,
            FailureDisposition::Incomplete,
        ),

        StageFailed => (
            FailureCode::PipelineStageFailed,
            FailureCategory::StageExecution,
            FailureSeverity::Error,
            FailureDisposition::Terminal,
        ),

        Cancelled => (
            FailureCode::PipelineCancelled,
            FailureCategory::Cancellation,
            FailureSeverity::Warning,
            FailureDisposition::Cancelled,
        ),

        StateTracking => (
            FailureCode::PipelineStateTracking,
            FailureCategory::Lifecycle,
            FailureSeverity::Critical,
            FailureDisposition::InternalFailure,
        ),

        InternalInvariant => (
            FailureCode::PipelineInternalInvariant,
            FailureCategory::Internal,
            FailureSeverity::Critical,
            FailureDisposition::InternalFailure,
        ),
    }
}

fn classify_orchestration(
    kind: PipelineOrchestrationErrorKind,
) -> (
    FailureCode,
    FailureCategory,
    FailureSeverity,
    FailureDisposition,
) {
    use PipelineOrchestrationErrorKind::*;

    match kind {
        InvalidStageGraph => (
            FailureCode::InvalidStageGraph,
            FailureCategory::Ordering,
            FailureSeverity::Critical,
            FailureDisposition::InternalFailure,
        ),

        MissingRequiredStage => (
            FailureCode::MissingRequiredStage,
            FailureCategory::Dependency,
            FailureSeverity::Error,
            FailureDisposition::Incomplete,
        ),

        RequiredStageSkipped => (
            FailureCode::RequiredStageSkipped,
            FailureCategory::Dependency,
            FailureSeverity::Error,
            FailureDisposition::Incomplete,
        ),

        RequiredStageBlocked => (
            FailureCode::RequiredStageBlocked,
            FailureCategory::Dependency,
            FailureSeverity::Error,
            FailureDisposition::Incomplete,
        ),

        StageFailed => (
            FailureCode::PipelineStageFailed,
            FailureCategory::StageExecution,
            FailureSeverity::Error,
            FailureDisposition::Terminal,
        ),

        Cancelled => (
            FailureCode::PipelineCancelled,
            FailureCategory::Cancellation,
            FailureSeverity::Warning,
            FailureDisposition::Cancelled,
        ),

        LifecycleFailure => (
            FailureCode::LifecycleFailure,
            FailureCategory::Lifecycle,
            FailureSeverity::Critical,
            FailureDisposition::InternalFailure,
        ),

        IncompleteRequiredWork => (
            FailureCode::IncompleteRequiredWork,
            FailureCategory::Dependency,
            FailureSeverity::Error,
            FailureDisposition::Incomplete,
        ),
    }
}

fn classify_ordering(
    error: &OrderingError,
) -> (
    FailureCode,
    FailureCategory,
    FailureSeverity,
    FailureDisposition,
) {
    match error {
        OrderingError::DuplicateStage { .. } => (
            FailureCode::OrderingDuplicateStage,
            FailureCategory::Ordering,
            FailureSeverity::Error,
            FailureDisposition::Terminal,
        ),

        OrderingError::MissingDependency { .. } => (
            FailureCode::OrderingMissingDependency,
            FailureCategory::Dependency,
            FailureSeverity::Error,
            FailureDisposition::Terminal,
        ),

        OrderingError::CycleDetected { .. } => (
            FailureCode::OrderingCycle,
            FailureCategory::Ordering,
            FailureSeverity::Error,
            FailureDisposition::Terminal,
        ),

        OrderingError::InternalInvariant { .. } => (
            FailureCode::OrderingInternalInvariant,
            FailureCategory::Internal,
            FailureSeverity::Critical,
            FailureDisposition::InternalFailure,
        ),
    }
}

fn diagnostic_level(severity: FailureSeverity) -> DiagnosticLevel {
    match severity {
        FailureSeverity::Information => DiagnosticLevel::Note,

        FailureSeverity::Warning => DiagnosticLevel::Warning,

        FailureSeverity::Error | FailureSeverity::Critical => {
            DiagnosticLevel::Error
        }
    }
}

// ============================================================================
// REGRESSION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::pipeline::stage::{
        StageError,
        StageErrorKind,
        StageId,
    };

    fn stage_id(value: &str) -> StageId {
        StageId::new(value).expect("valid stage ID")
    }

    #[test]
    fn api_version_is_stable() {
        assert_eq!(PIPELINE_FAILURE_API_VERSION, 1);
    }

    #[test]
    fn failure_codes_are_non_empty_and_stable() {
        let codes = [
            FailureCode::StageExecution,
            FailureCode::StageCancelled,
            FailureCode::StageBudgetExceeded,
            FailureCode::PipelineCancelled,
            FailureCode::OrderingCycle,
            FailureCode::ExternalFailure,
        ];

        for code in codes {
            assert!(!code.as_str().is_empty());
        }

        assert_eq!(
            FailureCode::StageExecution.as_str(),
            "ZC-PIPE-001"
        );

        assert_eq!(
            FailureCode::OrderingCycle.as_str(),
            "ZC-PIPE-027"
        );
    }

    #[test]
    fn stage_failure_preserves_original_typed_error() {
        let original = StageError::new(
            StageErrorKind::InvalidInput,
            "invalid IR input",
        );

        let failure = PipelineFailure::from_stage(
            stage_id("ir-verification"),
            original.clone(),
        );

        assert_eq!(
            failure.stage(),
            Some(&stage_id("ir-verification"))
        );

        assert_eq!(
            failure.code(),
            FailureCode::StageInvalidInput
        );

        assert_eq!(
            failure.stage_cause(),
            Some(&original)
        );

        assert!(failure.prevents_success());
    }

    #[test]
    fn retryability_is_only_a_declaration() {
        let cause = StageError::retryable(
            StageErrorKind::Execution,
            "temporary provider failure",
        );

        let failure = PipelineFailure::from_stage(
            stage_id("backend"),
            cause,
        );

        assert!(failure.is_declared_retryable());

        assert_eq!(
            failure.disposition(),
            FailureDisposition::Terminal
        );

        // A retry declaration never makes a failed compilation successful.
        assert!(failure.prevents_success());
    }

    #[test]
    fn cancellation_has_distinct_disposition() {
        let cause = StageError::new(
            StageErrorKind::Cancelled,
            "compilation cancelled",
        );

        let failure = PipelineFailure::from_stage(
            stage_id("optimization"),
            cause,
        );

        assert!(failure.is_cancelled());

        assert!(failure.prevents_success());

        assert_eq!(
            failure.category(),
            FailureCategory::Cancellation
        );
    }

    #[test]
    fn resource_exhaustion_is_not_a_hardware_limit() {
        let cause = StageError::new(
            StageErrorKind::BudgetExceeded,
            "configured memory budget exhausted",
        );

        let failure = PipelineFailure::from_stage(
            stage_id("lowering"),
            cause,
        );

        assert_eq!(
            failure.code(),
            FailureCode::StageBudgetExceeded
        );

        assert_eq!(
            failure.category(),
            FailureCategory::Resource
        );
    }

    #[test]
    fn failure_report_starts_empty() {
        let report = FailureReport::new();

        assert_eq!(report.stage_count(), 0);

        assert_eq!(report.completed_count(), 0);

        assert!(report.all_required_stages_completed());
    }

    #[test]
    fn failure_stage_record_preserves_status() {
        let record = FailureStageRecord::new(
            stage_id("frontend"),
            FailureStageStatus::Completed,
        );

        assert!(record.is_completed());

        assert_eq!(
            record.stage_id(),
            &stage_id("frontend")
        );
    }

    #[test]
    fn skipped_required_stage_is_not_completed() {
        let required = stage_id("verification");

        let mut report = FailureReport::new();

        report.required_stages.insert(required.clone());

        report.records.insert(
            required.clone(),
            FailureStageRecord::new(
                required,
                FailureStageStatus::Skipped {
                    reason: Arc::from("not applicable"),
                },
            ),
        );

        assert!(!report.all_required_stages_completed());

        assert_eq!(report.skipped_count(), 1);
    }

    #[test]
    fn missing_required_stage_is_not_completed() {
        let mut report = FailureReport::new();

        report.required_stages.insert(
            stage_id("artifact-validation"),
        );

        assert!(!report.all_required_stages_completed());
    }

    #[test]
    fn external_failure_is_explicit() {
        let failure = PipelineFailure::external(
            "external compilation provider failed",
        );

        assert_eq!(
            failure.origin(),
            FailureOrigin::External
        );

        assert_eq!(
            failure.category(),
            FailureCategory::External
        );

        assert!(failure.prevents_success());
    }

    #[test]
    fn diagnostic_conversion_preserves_stable_code() {
        let failure = PipelineFailure::from_stage(
            stage_id("semantic-validation"),
            StageError::new(
                StageErrorKind::UnsupportedOperation,
                "operation is unsupported",
            ),
        );

        let diagnostic = failure.to_diagnostic();

        assert_eq!(
            diagnostic.code,
            FailureCode::StageUnsupportedOperation.as_str()
        );

        assert!(diagnostic.is_error());

        assert!(
            diagnostic.message.contains("semantic-validation")
        );
    }

    #[test]
    fn context_preserves_operation_information() {
        let context = FailureContext::new()
            .with_compilation_id("compilation-001")
            .with_operation("portable-lowering");

        let failure = PipelineFailure::external(
            "lowering provider failed",
        )
        .with_context(context);

        assert_eq!(
            failure.context().compilation_id(),
            Some("compilation-001")
        );

        assert_eq!(
            failure.context().operation(),
            Some("portable-lowering")
        );
    }

    #[test]
    fn failure_display_contains_machine_code() {
        let failure = PipelineFailure::from_stage(
            stage_id("backend"),
            StageError::new(
                StageErrorKind::Execution,
                "backend execution failed",
            ),
        );

        let rendered = failure.to_string();

        assert!(rendered.contains("ZC-PIPE-001"));

        assert!(rendered.contains("backend"));
    }

    #[test]
    fn internal_invariants_are_critical() {
        let failure = PipelineFailure::from_stage(
            stage_id("pipeline"),
            StageError::new(
                StageErrorKind::InternalInvariant,
                "invariant violated",
            ),
        );

        assert_eq!(
            failure.severity(),
            FailureSeverity::Critical
        );

        assert_eq!(
            failure.disposition(),
            FailureDisposition::InternalFailure
        );
    }
}
