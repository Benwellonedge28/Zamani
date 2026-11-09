
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Compiler — Pipeline Failure Management.
//!
//! Path: src/compiler/pipeline/failure.rs
//! Edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Normalize and preserve failures originating from the compiler pipeline.
//!
//! This module provides:
//!
//! - Stable machine-readable failure codes.
//! - Typed failure classification.
//! - Failure origin and disposition.
//! - Retryability declarations.
//! - Original typed error preservation.
//! - Partial execution reports.
//! - Required-stage completion verification.
//! - Blocked-stage dependency relationships.
//! - Deterministic failure reporting.
//! - Conversion into canonical frontend diagnostics.
//!
//! # Canonical integration
//!
//! pipeline::stage
//! pipeline::execution
//! pipeline::orchestration
//! pipeline::ordering
//! pipeline::recovery
//! compiler::diagnostics
//! frontend::semantic::diagnostics
//!
//! # Ownership
//!
//! This module owns failure normalization only.
//!
//! It does not:
//!
//! - Execute recovery.
//! - Retry failed stages.
//! - Execute fallback implementations.
//! - Change compilation lifecycle state.
//! - Allocate hardware.
//! - Schedule physical resources.
//! - Modify source semantics.
//! - Define grammar or semantic errors.
//! - Implement backend execution.
//!
//! # POCO-REAF
//!
//! No universal ceilings are imposed on:
//!
//! - Pipeline stages.
//! - Failure records.
//! - Dependencies.
//! - Computing domains.
//! - Devices.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Distributed nodes.
//! - Memory.
//! - IR operations.
//!
//! Collections grow according to available resources.
//!
//! Configured operational budgets remain owned by the compilation
//! context and configuration subsystems.
//!
//! # Failure invariant
//!
//! A failed or incomplete required stage must never be represented
//! as successfully completed compilation.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::frontend::semantic::diagnostics::{
    Diagnostic,
    DiagnosticCategory,
    DiagnosticPhase,
    DiagnosticSeverity,
    RecoveryClassification,
};

use crate::compiler::pipeline::stage::{
    StageError,
    StageErrorKind,
    StageId,
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

// ============================================================================
// API VERSION
// ============================================================================

/// Version of the public pipeline failure API.
///
/// Independent of language, grammar, semantic, IR, backend and runtime
/// versions.
pub const PIPELINE_FAILURE_API_VERSION: u16 = 1;

// ============================================================================
// STABLE FAILURE CODES
// ============================================================================

/// Stable machine-readable compiler pipeline failure identifiers.
///
/// Existing codes must never be reassigned to different meanings.
///
/// New codes must be appended without changing existing identifiers.
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

    /// A stage attempted budgeted work without declaring its
    /// required default budget dimension.
    StageMissingBudgetDimension,

    ExternalFailure,
}

impl FailureCode {
    /// Return the stable diagnostic identifier.
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

            Self::StageMissingBudgetDimension => "ZC-PIPE-029",

            Self::ExternalFailure => "ZC-PIPE-999",
        }
    }
}

impl fmt::Display for FailureCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// FAILURE CATEGORY
// ============================================================================

/// Broad classification of pipeline failures.
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

impl FailureCategory {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StageExecution => "stage_execution",
            Self::Configuration => "configuration",
            Self::Dependency => "dependency",
            Self::Ordering => "ordering",
            Self::Resource => "resource",
            Self::Cancellation => "cancellation",
            Self::Verification => "verification",
            Self::UnsupportedOperation => "unsupported_operation",
            Self::Lifecycle => "lifecycle",
            Self::Internal => "internal",
            Self::External => "external",
        }
    }
}

impl fmt::Display for FailureCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// FAILURE SEVERITY
// ============================================================================

/// Operational severity of a pipeline failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum FailureSeverity {
    Information,
    Warning,
    Error,
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

impl FailureOrigin {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stage => "stage",
            Self::Execution => "execution",
            Self::Orchestration => "orchestration",
            Self::Ordering => "ordering",
            Self::External => "external",
        }
    }
}

// ============================================================================
// FAILURE DISPOSITION
// ============================================================================

/// Describes the effect of the failure on compilation success.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FailureDisposition {
    /// Required compilation work cannot be reported successful.
    Terminal,

    /// Compilation was cancelled.
    Cancelled,

    /// Required work remains incomplete.
    Incomplete,

    /// An internal correctness invariant was violated.
    InternalFailure,

    /// The failure does not independently establish that required
    /// compilation work has failed.
    NonTerminal,
}

impl FailureDisposition {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Terminal => "terminal",
            Self::Cancelled => "cancelled",
            Self::Incomplete => "incomplete",
            Self::InternalFailure => "internal_failure",
            Self::NonTerminal => "non_terminal",
        }
    }
}

// ============================================================================
// RETRY DECLARATION
// ============================================================================

/// Retryability information declared by the original failure source.
///
/// This is not authorization to retry.
///
/// Recovery must independently verify replay safety, policy,
/// category authorization and operational budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RetryDeclaration {
    NotDeclared,
    DeclaredByStage,
}

impl RetryDeclaration {
    #[must_use]
    pub const fn is_declared_retryable(self) -> bool {
        matches!(self, Self::DeclaredByStage)
    }
}

// ============================================================================
// FAILURE STAGE STATUS
// ============================================================================

/// Normalized immutable snapshot of an individual stage.
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

    /// The stage had not started when failure was observed.
    NotStarted,
}

/// Immutable stage execution record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailureStageRecord {
    stage_id: StageId,
    status: FailureStageStatus,
}

impl FailureStageRecord {
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

    #[must_use]
    pub fn stage_id(&self) -> &StageId {
        &self.stage_id
    }

    #[must_use]
    pub fn status(&self) -> &FailureStageStatus {
        &self.status
    }

    #[must_use]
    pub fn is_completed(&self) -> bool {
        matches!(self.status, FailureStageStatus::Completed)
    }
}

// ============================================================================
// FAILURE RELATIONSHIPS
// ============================================================================

/// Causal relationship between affected pipeline stages.
///
/// These records preserve failure relationships without creating
/// another authoritative dependency graph.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FailureRelation {
    CausedBy {
        stage: StageId,
    },

    BlockedBy {
        stage: StageId,
        dependency: StageId,
    },

    AffectedBy {
        stage: StageId,
        upstream: StageId,
    },
}

// ============================================================================
// FAILURE REPORT
// ============================================================================

/// Immutable normalized snapshot of pipeline execution at failure time.
///
/// The original execution and orchestration reports remain owned by
/// their respective subsystems.
///
/// This report does not own stage outputs or domain IR.
#[derive(Debug, Clone, Default)]
pub struct FailureReport {
    records: BTreeMap<StageId, FailureStageRecord>,

    required_stages: BTreeSet<StageId>,

    relations: Vec<FailureRelation>,
}

impl FailureReport {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn records(&self) -> &BTreeMap<StageId, FailureStageRecord> {
        &self.records
    }

    #[must_use]
    pub fn required_stages(&self) -> &BTreeSet<StageId> {
        &self.required_stages
    }

    #[must_use]
    pub fn relations(&self) -> &[FailureRelation] {
        &self.relations
    }

    #[must_use]
    pub fn stage_count(&self) -> usize {
        self.records.len()
    }

    #[must_use]
    pub fn completed_count(&self) -> usize {
        self.count_status(|status| {
            matches!(status, FailureStageStatus::Completed)
        })
    }

    #[must_use]
    pub fn skipped_count(&self) -> usize {
        self.count_status(|status| {
            matches!(status, FailureStageStatus::Skipped { .. })
        })
    }

    #[must_use]
    pub fn blocked_count(&self) -> usize {
        self.count_status(|status| {
            matches!(status, FailureStageStatus::Blocked { .. })
        })
    }

    #[must_use]
    pub fn failed_count(&self) -> usize {
        self.count_status(|status| {
            matches!(status, FailureStageStatus::Failed { .. })
        })
    }

    #[must_use]
    pub fn not_started_count(&self) -> usize {
        self.count_status(|status| {
            matches!(status, FailureStageStatus::NotStarted)
        })
    }

    fn count_status(
        &self,
        predicate: impl Fn(&FailureStageStatus) -> bool,
    ) -> usize {
        self.records
            .values()
            .filter(|record| predicate(record.status()))
            .count()
    }

    /// Missing records are never considered successful.
    #[must_use]
    pub fn all_required_stages_completed(&self) -> bool {
        self.required_stages.iter().all(|required| {
            self.records
                .get(required)
                .is_some_and(FailureStageRecord::is_completed)
        })
    }

    #[must_use]
    pub fn record(
        &self,
        stage_id: &StageId,
    ) -> Option<&FailureStageRecord> {
        self.records.get(stage_id)
    }

    /// Record an explicitly failed stage.
    ///
    /// Used when the executor returns an error before its partial
    /// report has been able to record the failed invocation.
    fn record_failed(
        &mut self,
        stage: StageId,
        message: impl Into<Arc<str>>,
    ) {
        let record = FailureStageRecord::new(
            stage.clone(),
            FailureStageStatus::Failed {
                message: message.into(),
            },
        );

        self.records.insert(stage.clone(), record);

        self.relations.push(
            FailureRelation::CausedBy {
                stage,
            },
        );
    }

    /// Capture an executor report.
    #[must_use]
    pub fn from_execution(
        report: &ExecutionExecutionReport,
    ) -> Self {
        let mut result = Self {
            records: BTreeMap::new(),
            required_stages: report.required_stages().clone(),
            relations: Vec::new(),
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
                                "a prerequisite did not complete",
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

        result.add_missing_required_stages();

        result
    }

    /// Capture an orchestration report.
    #[must_use]
    pub fn from_orchestration(
        report: &OrchestrationExecutionReport,
    ) -> Self {
        let mut result = Self {
            records: BTreeMap::new(),
            required_stages: report.required_stages().clone(),
            relations: Vec::new(),
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
                    result.relations.push(
                        FailureRelation::BlockedBy {
                            stage: stage_id.clone(),
                            dependency: dependency.clone(),
                        },
                    );

                    FailureStageStatus::Blocked {
                        reason: reason.clone(),
                    }
                }

                OrchestrationStageStatus::Failed { message } => {
                    result.relations.push(
                        FailureRelation::CausedBy {
                            stage: stage_id.clone(),
                        },
                    );

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

        result.add_missing_required_stages();

        result
    }

    /// Every missing required stage receives an explicit NotStarted
    /// record. It is never silently treated as completed.
    fn add_missing_required_stages(&mut self) {
        for stage in self.required_stages.iter() {
            if !self.records.contains_key(stage) {
                self.records.insert(
                    stage.clone(),
                    FailureStageRecord::new(
                        stage.clone(),
                        FailureStageStatus::NotStarted,
                    ),
                );
            }
        }
    }
}

// ============================================================================
// FAILURE CONTEXT
// ============================================================================

/// Additional contextual information for a pipeline failure.
#[derive(Debug, Clone, Default)]
pub struct FailureContext {
    compilation_id: Option<Arc<str>>,
    operation: Option<Arc<str>>,
    stage: Option<StageId>,
    relations: Vec<FailureRelation>,
}

impl FailureContext {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_compilation_id(
        mut self,
        value: impl Into<Arc<str>>,
    ) -> Self {
        self.compilation_id = Some(value.into());
        self
    }

    #[must_use]
    pub fn with_operation(
        mut self,
        value: impl Into<Arc<str>>,
    ) -> Self {
        self.operation = Some(value.into());
        self
    }

    #[must_use]
    pub fn with_stage(
        mut self,
        stage: StageId,
    ) -> Self {
        self.stage = Some(stage);
        self
    }

    #[must_use]
    pub fn with_relation(
        mut self,
        relation: FailureRelation,
    ) -> Self {
        self.relations.push(relation);
        self
    }

    #[must_use]
    pub fn compilation_id(&self) -> Option<&str> {
        self.compilation_id.as_deref()
    }

    #[must_use]
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    #[must_use]
    pub fn stage(&self) -> Option<&StageId> {
        self.stage.as_ref()
    }

    #[must_use]
    pub fn relations(&self) -> &[FailureRelation] {
        &self.relations
    }
}

// ============================================================================
// TYPED FAILURE CAUSES
// ============================================================================

/// Preserves the original typed error.
///
/// The normalized failure must not destroy the originating error
/// or replace it with a human-readable string.
#[derive(Debug, Clone)]
enum FailureCause {
    Stage(StageError),
    Execution(ExecutionError),
    Orchestration(PipelineOrchestrationError),
    Ordering(OrderingError),
}

impl FailureCause {
    fn stage_error(&self) -> Option<&StageError> {
        match self {
            Self::Stage(error) => Some(error),
            Self::Execution(error) => error.stage_cause(),
            Self::Orchestration(error) => error.stage_error(),
            Self::Ordering(_) => None,
        }
    }

    fn as_error(&self) -> &(dyn Error + 'static) {
        match self {
            Self::Stage(error) => error,
            Self::Execution(error) => error,
            Self::Orchestration(error) => error,
            Self::Ordering(error) => error,
        }
    }
}

// ============================================================================
// PIPELINE FAILURE
// ============================================================================

/// Canonical normalized representation of a compiler pipeline failure.
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
    cause: Option<FailureCause>,
    report: FailureReport,
    context: FailureContext,
}

impl PipelineFailure {
    /// Normalize a direct stage failure.
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

        let mut report = FailureReport::new();
        report.record_failed(
            stage.clone(),
            cause.message(),
        );

        Self {
            code,
            category,
            severity,
            origin: FailureOrigin::Stage,
            disposition,
            retry_declaration,
            stage: Some(stage.clone()),
            message: Arc::from(cause.message()),
            cause: Some(FailureCause::Stage(cause)),
            report,
            context: FailureContext::new().with_stage(stage),
        }
    }

    /// Normalize an executor failure.
    #[must_use]
    pub fn from_execution(
        error: &ExecutionError,
    ) -> Self {
        let stage = error.stage().cloned();
        let stage_cause = error.stage_cause();

        let classification = match stage_cause {
            Some(cause)
                if error.kind() == ExecutionErrorKind::StageFailed
                    || error.kind() == ExecutionErrorKind::Cancelled =>
            {
                (
                    code_from_stage(cause.kind()),
                    category_from_stage(cause.kind()),
                    severity_from_stage(cause.kind()),
                    disposition_from_stage(cause.kind()),
                )
            }

            _ => classify_execution(error.kind()),
        };

        let retry_declaration = match stage_cause {
            Some(cause) if cause.is_retryable() => {
                RetryDeclaration::DeclaredByStage
            }
            _ => RetryDeclaration::NotDeclared,
        };

        let mut report = FailureReport::from_execution(
            error.partial_report(),
        );

        if let (Some(stage), Some(cause)) = (
            stage.as_ref(),
            stage_cause,
        ) {
            report.record_failed(
                stage.clone(),
                cause.message(),
            );
        }

        let mut context = FailureContext::new();

        if let Some(stage) = stage.as_ref() {
            context = context.with_stage(stage.clone());
        }

        Self {
            code: classification.0,
            category: classification.1,
            severity: classification.2,
            disposition: classification.3,
            origin: FailureOrigin::Execution,
            retry_declaration,
            stage,
            message: Arc::from(error.message()),
            cause: Some(FailureCause::Execution(error.clone())),
            report,
            context,
        }
    }

    /// Normalize an orchestration failure.
    #[must_use]
    pub fn from_orchestration(
        error: &PipelineOrchestrationError,
    ) -> Self {
        let stage = error.stage().cloned();
        let stage_cause = error.stage_error();

        let classification = if let Some(ordering) = error.ordering_error() {
            classify_ordering(ordering)
        } else if let Some(cause) = stage_cause {
            code_from_stage(cause.kind())
                .pipe_classification(cause.kind())
        } else {
            classify_orchestration(error.kind())
        };

        let retry_declaration = match stage_cause {
            Some(cause) if cause.is_retryable() => {
                RetryDeclaration::DeclaredByStage
            }
            _ => RetryDeclaration::NotDeclared,
        };

        let mut report = FailureReport::from_orchestration(
            error.partial_report(),
        );

        if let (Some(stage), Some(cause)) = (
            stage.as_ref(),
            stage_cause,
        ) {
            report.record_failed(
                stage.clone(),
                cause.message(),
            );
        }

        let mut context = FailureContext::new();

        if let Some(stage) = stage.as_ref() {
            context = context.with_stage(stage.clone());
        }

        Self {
            code: classification.0,
            category: classification.1,
            severity: classification.2,
            disposition: classification.3,
            origin: FailureOrigin::Orchestration,
            retry_declaration,
            stage,
            message: Arc::from(error.message()),
            cause: Some(FailureCause::Orchestration(error.clone())),
            report,
            context,
        }
    }

    /// Normalize a direct ordering failure.
    #[must_use]
    pub fn from_ordering(
        error: &OrderingError,
    ) -> Self {
        let classification = classify_ordering(error);

        Self {
            code: classification.0,
            category: classification.1,
            severity: classification.2,
            disposition: classification.3,
            origin: FailureOrigin::Ordering,
            retry_declaration: RetryDeclaration::NotDeclared,
            stage: None,
            message: Arc::from(error.to_string()),
            cause: Some(FailureCause::Ordering(error.clone())),
            report: FailureReport::new(),
            context: FailureContext::new(),
        }
    }

    /// Create a failure from an external integration.
    ///
    /// External systems retain ownership of their original error types.
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
            cause: None,
            report: FailureReport::new(),
            context: FailureContext::new(),
        }
    }

    /// Attach additional contextual information.
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

    #[must_use]
    pub const fn code(&self) -> FailureCode {
        self.code
    }

    #[must_use]
    pub const fn code_str(&self) -> &'static str {
        self.code.as_str()
    }

    #[must_use]
    pub const fn category(&self) -> FailureCategory {
        self.category
    }

    #[must_use]
    pub const fn severity(&self) -> FailureSeverity {
        self.severity
    }

    #[must_use]
    pub const fn origin(&self) -> FailureOrigin {
        self.origin
    }

    #[must_use]
    pub const fn disposition(&self) -> FailureDisposition {
        self.disposition
    }

    #[must_use]
    pub const fn retry_declaration(&self) -> RetryDeclaration {
        self.retry_declaration
    }

    /// A declaration is not retry authorization.
    #[must_use]
    pub const fn is_declared_retryable(&self) -> bool {
        self.retry_declaration.is_declared_retryable()
    }

    #[must_use]
    pub fn stage(&self) -> Option<&StageId> {
        self.stage.as_ref()
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the original typed stage error, when present.
    #[must_use]
    pub fn stage_cause(&self) -> Option<&StageError> {
        self.cause
            .as_ref()
            .and_then(FailureCause::stage_error)
    }

    #[must_use]
    pub fn report(&self) -> &FailureReport {
        &self.report
    }

    #[must_use]
    pub fn context(&self) -> &FailureContext {
        &self.context
    }

    #[must_use]
    pub const fn is_cancelled(&self) -> bool {
        matches!(
            self.disposition,
            FailureDisposition::Cancelled
        )
    }

    /// Determine whether this failure prevents successful compilation.
    ///
    /// This is deliberately conservative.
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

    /// Convert this failure into the canonical frontend diagnostic.
    ///
    /// Pipeline diagnostics are session-level diagnostics and therefore
    /// have no fabricated source span.
    #[must_use]
    pub fn to_diagnostic(&self) -> Diagnostic {
        let mut message = String::new();

        if let Some(stage) = self.stage.as_ref() {
            message.push_str("stage `");
            message.push_str(stage.as_str());
            message.push_str("`: ");
        }

        message.push_str(self.message());

        let mut diagnostic = Diagnostic::new(
            self.code_str(),
            DiagnosticPhase::Extension(
                String::from("compiler.pipeline"),
            ),
            diagnostic_category(self.category),
            diagnostic_severity(self.severity),
            message,
            None,
        )
        .with_recovery(diagnostic_recovery(self.disposition))
        .with_metadata(
            "pipeline.failure.category",
            self.category.as_str(),
        )
        .with_metadata(
            "pipeline.failure.origin",
            self.origin.as_str(),
        )
        .with_metadata(
            "pipeline.failure.disposition",
            self.disposition.as_str(),
        )
        .with_metadata(
            "pipeline.failure.retry_declared",
            self.is_declared_retryable(),
        )
        .with_metadata(
            "pipeline.failure.completed_stages",
            self.report.completed_count() as u128,
        )
        .with_metadata(
            "pipeline.failure.failed_stages",
            self.report.failed_count() as u128,
        )
        .with_metadata(
            "pipeline.failure.blocked_stages",
            self.report.blocked_count() as u128,
        );

        if let Some(operation) = self.context.operation() {
            diagnostic = diagnostic.with_note(
                format!("operation: {operation}"),
            );
        }

        if let Some(compilation_id) = self.context.compilation_id() {
            diagnostic = diagnostic.with_metadata(
                "pipeline.failure.compilation_id",
                compilation_id,
            );
        }

        if let Some(stage) = self.stage.as_ref() {
            diagnostic = diagnostic.with_metadata(
                "pipeline.failure.stage",
                stage.as_str(),
            );
        }

        diagnostic
    }
}

impl fmt::Display for PipelineFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} failure",
            self.code,
            self.category,
        )?;

        if let Some(stage) = self.stage.as_ref() {
            write!(f, " at stage `{stage}`")?;
        }

        write!(f, ": {}", self.message)
    }
}

impl Error for PipelineFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause
            .as_ref()
            .map(FailureCause::as_error)
    }
}

// ============================================================================
// CLASSIFICATION
// ============================================================================

type Classification = (
    FailureCode,
    FailureCategory,
    FailureSeverity,
    FailureDisposition,
);

trait ClassificationExt {
    fn pipe_classification(self, kind: StageErrorKind) -> Classification;
}

impl ClassificationExt for FailureCode {
    fn pipe_classification(self, kind: StageErrorKind) -> Classification {
        (
            self,
            category_from_stage(kind),
            severity_from_stage(kind),
            disposition_from_stage(kind),
        )
    }
}

fn code_from_stage(kind: StageErrorKind) -> FailureCode {
    match kind {
        StageErrorKind::Execution => FailureCode::StageExecution,
        StageErrorKind::Cancelled => FailureCode::StageCancelled,
        StageErrorKind::BudgetExceeded => FailureCode::StageBudgetExceeded,
        StageErrorKind::InvalidInput => FailureCode::StageInvalidInput,
        StageErrorKind::UnsupportedOperation => {
            FailureCode::StageUnsupportedOperation
        }
        StageErrorKind::IncompatibleConfiguration => {
            FailureCode::StageIncompatibleConfiguration
        }
        StageErrorKind::InvalidDecision => FailureCode::StageInvalidDecision,
        StageErrorKind::MissingBudgetDimension => {
            FailureCode::StageMissingBudgetDimension
        }
        StageErrorKind::ExecutionContext => FailureCode::StageExecutionContext,
        StageErrorKind::InternalInvariant => FailureCode::StageInternalInvariant,
    }
}

fn category_from_stage(kind: StageErrorKind) -> FailureCategory {
    match kind {
        StageErrorKind::Execution => FailureCategory::StageExecution,
        StageErrorKind::Cancelled => FailureCategory::Cancellation,
        StageErrorKind::BudgetExceeded
        | StageErrorKind::MissingBudgetDimension => FailureCategory::Resource,
        StageErrorKind::InvalidInput
        | StageErrorKind::IncompatibleConfiguration => {
            FailureCategory::Configuration
        }
        StageErrorKind::UnsupportedOperation => {
            FailureCategory::UnsupportedOperation
        }
        StageErrorKind::InvalidDecision => FailureCategory::StageExecution,
        StageErrorKind::ExecutionContext => FailureCategory::Lifecycle,
        StageErrorKind::InternalInvariant => FailureCategory::Internal,
    }
}

fn severity_from_stage(kind: StageErrorKind) -> FailureSeverity {
    match kind {
        StageErrorKind::InternalInvariant
        | StageErrorKind::ExecutionContext => FailureSeverity::Critical,
        StageErrorKind::Cancelled => FailureSeverity::Warning,
        _ => FailureSeverity::Error,
    }
}

fn disposition_from_stage(kind: StageErrorKind) -> FailureDisposition {
    match kind {
        StageErrorKind::Cancelled => FailureDisposition::Cancelled,
        StageErrorKind::InternalInvariant
        | StageErrorKind::ExecutionContext => {
            FailureDisposition::InternalFailure
        }
        _ => FailureDisposition::Terminal,
    }
}

fn classify_execution(kind: ExecutionErrorKind) -> Classification {
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
) -> Classification {
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

fn classify_ordering(error: &OrderingError) -> Classification {
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

// ============================================================================
// CANONICAL DIAGNOSTIC CONVERSION
// ============================================================================

fn diagnostic_category(
    category: FailureCategory,
) -> DiagnosticCategory {
    match category {
        FailureCategory::Configuration => DiagnosticCategory::Configuration,
        FailureCategory::Dependency => DiagnosticCategory::Contract,
        FailureCategory::Ordering => DiagnosticCategory::Contract,
        FailureCategory::Resource => DiagnosticCategory::Resource,
        FailureCategory::Cancellation => DiagnosticCategory::OperationalLimit,
        FailureCategory::Verification => DiagnosticCategory::Contract,
        FailureCategory::UnsupportedOperation => DiagnosticCategory::Capability,
        FailureCategory::Lifecycle
        | FailureCategory::Internal => DiagnosticCategory::Internal,
        FailureCategory::StageExecution
        | FailureCategory::External => {
            DiagnosticCategory::Extension(
                String::from("compiler.pipeline"),
            )
        }
    }
}

fn diagnostic_severity(
    severity: FailureSeverity,
) -> DiagnosticSeverity {
    match severity {
        FailureSeverity::Information => DiagnosticSeverity::Information,
        FailureSeverity::Warning => DiagnosticSeverity::Warning,
        FailureSeverity::Error => DiagnosticSeverity::Error,
        FailureSeverity::Critical => DiagnosticSeverity::Fatal,
    }
}

fn diagnostic_recovery(
    disposition: FailureDisposition,
) -> RecoveryClassification {
    match disposition {
        FailureDisposition::NonTerminal => RecoveryClassification::Continue,
        FailureDisposition::Incomplete
        | FailureDisposition::Cancelled => {
            RecoveryClassification::Undetermined
        }
        FailureDisposition::Terminal
        | FailureDisposition::InternalFailure => {
            RecoveryClassification::Unrecoverable
        }
    }
}

// ============================================================================
// REGRESSION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn stage_id(value: &str) -> StageId {
        StageId::new(value).expect("valid stage ID")
    }

    #[test]
    fn api_version_is_stable() {
        assert_eq!(PIPELINE_FAILURE_API_VERSION, 1);
    }

    #[test]
    fn failure_codes_are_stable() {
        assert_eq!(
            FailureCode::StageExecution.as_str(),
            "ZC-PIPE-001",
        );

        assert_eq!(
            FailureCode::OrderingCycle.as_str(),
            "ZC-PIPE-027",
        );

        assert_eq!(
            FailureCode::StageMissingBudgetDimension.as_str(),
            "ZC-PIPE-029",
        );
    }

    #[test]
    fn stage_failure_preserves_original_error() {
        let original = StageError::new(
            StageErrorKind::InvalidInput,
            "invalid IR input",
        );

        let failure = PipelineFailure::from_stage(
            stage_id("ir-verification"),
            original.clone(),
        );

        assert_eq!(
            failure.stage_cause(),
            Some(&original),
        );

        assert_eq!(
            failure.code(),
            FailureCode::StageInvalidInput,
        );

        assert!(failure.prevents_success());
        assert_eq!(failure.report().failed_count(), 1);
    }

    #[test]
    fn missing_budget_dimension_has_explicit_classification() {
        let failure = PipelineFailure::from_stage(
            stage_id("resource-planning"),
            StageError::new(
                StageErrorKind::MissingBudgetDimension,
                "missing default budget dimension",
            ),
        );

        assert_eq!(
            failure.code(),
            FailureCode::StageMissingBudgetDimension,
        );

        assert_eq!(
            failure.category(),
            FailureCategory::Resource,
        );
    }

    #[test]
    fn retryability_is_not_retry_authorization() {
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
            FailureDisposition::Terminal,
        );

        assert!(failure.prevents_success());
    }

    #[test]
    fn cancellation_has_distinct_disposition() {
        let failure = PipelineFailure::from_stage(
            stage_id("optimization"),
            StageError::new(
                StageErrorKind::Cancelled,
                "compilation cancelled",
            ),
        );

        assert!(failure.is_cancelled());
        assert!(failure.prevents_success());

        assert_eq!(
            failure.category(),
            FailureCategory::Cancellation,
        );
    }

    #[test]
    fn failure_report_preserves_required_work() {
        let mut report = FailureReport::new();

        let required = stage_id("verification");

        report.required_stages.insert(required.clone());

        report.records.insert(
            required.clone(),
            FailureStageRecord::new(
                required,
                FailureStageStatus::Skipped {
                    reason: Arc::from("skipped"),
                },
            ),
        );

        assert!(!report.all_required_stages_completed());
        assert_eq!(report.skipped_count(), 1);
    }

    #[test]
    fn absent_required_stage_is_not_started() {
        let required = stage_id("artifact-validation");

        let mut report = FailureReport::new();

        report.required_stages.insert(required.clone());
        report.add_missing_required_stages();

        assert!(!report.all_required_stages_completed());

        assert_eq!(report.not_started_count(), 1);

        assert!(matches!(
            report.record(&required).map(FailureStageRecord::status),
            Some(FailureStageStatus::NotStarted),
        ));
    }

    #[test]
    fn external_failure_is_terminal() {
        let failure = PipelineFailure::external(
            "external compilation provider failed",
        );

        assert_eq!(
            failure.origin(),
            FailureOrigin::External,
        );

        assert_eq!(
            failure.category(),
            FailureCategory::External,
        );

        assert!(failure.prevents_success());
    }

    #[test]
    fn diagnostic_uses_canonical_frontend_api() {
        let failure = PipelineFailure::from_stage(
            stage_id("semantic-validation"),
            StageError::new(
                StageErrorKind::UnsupportedOperation,
                "operation unsupported",
            ),
        );

        let diagnostic = failure.to_diagnostic();

        assert_eq!(
            diagnostic.code().as_str(),
            FailureCode::StageUnsupportedOperation.as_str(),
        );

        assert_eq!(
            diagnostic.severity(),
            DiagnosticSeverity::Error,
        );

        assert!(diagnostic.message().contains("semantic-validation"));

        assert_eq!(
            diagnostic.metadata_value(
                "pipeline.failure.category",
            ),
            Some(&crate::frontend::semantic::diagnostics::DiagnosticValue::Text(
                String::from("unsupported_operation"),
            )),
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

        let display = failure.to_string();

        assert!(display.contains("ZC-PIPE-001"));
        assert!(display.contains("backend"));
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
            FailureSeverity::Critical,
        );

        assert_eq!(
            failure.disposition(),
            FailureDisposition::InternalFailure,
        );
    }
}
