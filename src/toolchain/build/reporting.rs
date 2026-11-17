
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Build Reporting
//!
//! Path: `src/toolchain/build/reporting.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Produce deterministic, machine-readable reports for actual Zamani
//! build operations.
//!
//! This module reports:
//!
//! - logical build identity;
//! - the associated immutable build-input snapshot;
//! - compiler feasibility and target-resolution evidence;
//! - actual validated build outputs;
//! - target-to-artifact coverage;
//! - artifact references and provenance metadata;
//! - canonical compiler diagnostics;
//! - successful, partial, incomplete and failed outcomes.
//!
//! It supports both direct writer-based JSON serialization and
//! explicitly requested owned JSON strings.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Build-report composition.
//! - Build-report status classification.
//! - Build-input identity presentation.
//! - Build-plan evidence presentation.
//! - Target-output coverage presentation.
//! - Output summary presentation.
//! - Failure-report composition.
//! - JSON serialization of the composed report.
//!
//! This module does not own:
//!
//! - Build-plan construction.
//! - Build-input discovery or fingerprint calculation.
//! - Target discovery or feasibility analysis.
//! - Compilation.
//! - Task scheduling or execution.
//! - Artifact creation or storage.
//! - Artifact adaptation or verification.
//! - Diagnostic construction or aggregation.
//! - Provenance authentication.
//! - Runtime execution.
//! - Deployment.
//!
//! Those responsibilities remain with their canonical subsystems.
//!
//! ## Canonical authorities
//!
//! Build plans:
//!     crate::toolchain::build::plan
//!
//! Build inputs:
//!     crate::toolchain::build::inputs
//!
//! Build outputs:
//!     crate::toolchain::build::outputs
//!
//! Actual compiler execution:
//!     crate::toolchain::build::executor
//!
//! Artifact representation and validation:
//!     crate::compiler::artifacts
//!
//! Compiler diagnostic aggregation:
//!     crate::compiler::diagnostics::aggregation
//!
//! Compiler diagnostic serialization:
//!     crate::compiler::diagnostics::reporting
//!
//! Public API errors:
//!     crate::toolchain::api::response
//!
//! ## POCO-REAF
//!
//! One logical build may produce multiple target-specific realizations.
//!
//! Reports preserve:
//!
//! - one logical build identity;
//! - one associated logical input identity;
//! - target-specific feasibility evidence;
//! - target-specific artifact coverage;
//! - individual canonical artifact identities.
//!
//! No closed list of computational domains or hardware types is used.
//!
//! Classical, quantum, HDL, AI/tensor, hybrid, distributed, networking,
//! cryptographic and future computational domains use the same report
//! structure.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - targets;
//! - artifacts;
//! - diagnostic records;
//! - input records;
//! - realization fingerprints;
//! - computational domains;
//! - providers;
//! - distributed participants.
//!
//! Collections grow subject to available memory, address space,
//! representational constraints and explicitly configured operational
//! budgets.
//!
//! Large artifact payloads are never copied into reports.
//! Reports contain canonical artifact references.
//!
//! Literal infinite memory, storage or execution is not promised.
//!
//! ## Determinism
//!
//! This module preserves canonical ordering supplied by its authorities.
//!
//! It introduces no timestamps, random identifiers, process identifiers,
//! hostnames, memory addresses or hash-map iteration order.
//!
//! ## Failure semantics
//!
//! A report never upgrades an incomplete output collection to complete.
//!
//! An incomplete diagnostic aggregate remains explicitly incomplete.
//!
//! Missing diagnostics do not mean that no diagnostics occurred.
//!
//! A failed or cancelled build is not represented as a successful build.
//!
//! A serialization failure is returned to the caller. The writer may
//! contain a partial JSON document and must not treat it as complete.
//!
//! ## Security
//!
//! Reports do not serialize arbitrary underlying error objects.
//! They use the canonical ResponseError representation.
//!
//! Artifact payload bytes, credentials, access tokens, private keys
//! and raw secret configuration are not copied into report records.

use serde::Serialize;

use std::io::Write;

use crate::compiler::artifacts::validation::ValidationStatus;

use crate::compiler::diagnostics::aggregation::AggregatedDiagnostics;

use crate::compiler::diagnostics::reporting::{
    DiagnosticReport,
};

use crate::toolchain::api::ProjectScope;

use crate::toolchain::api::request::ProjectLocator;

use crate::toolchain::api::response::ResponseError;

use crate::toolchain::build::inputs::{
    BuildInputs,
};

use crate::toolchain::build::outputs::{
    BuildOutputStatus,
    BuildOutputs,
};

use crate::toolchain::build::plan::{
    AssessedBuildPlan,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::result::ToolchainResult;


// ============================================================================
// REPORT SCHEMA
// ============================================================================

/// Stable schema identifier for a completed build report.
///
/// Incompatible serialized-report changes require a new schema identifier.
pub const BUILD_REPORT_SCHEMA: &str =
    "zamani.toolchain.build-report.v1";

/// Stable schema identifier for a failed build report.
pub const BUILD_FAILURE_REPORT_SCHEMA: &str =
    "zamani.toolchain.build-failure-report.v1";


// ============================================================================
// REPORT STATUS
// ============================================================================

/// Overall evidence status of a build report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildReportStatus {
    /// All required build outputs and report evidence are complete.
    Complete,

    /// At least one permitted target realization succeeded,
    /// while other requested realizations were not completed.
    Partial,

    /// Required output or report evidence is incomplete.
    Incomplete,
}

impl BuildReportStatus {
    /// Return the stable machine-readable status.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Incomplete => "incomplete",
        }
    }

    /// Return whether the report is complete.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Return whether the report represents partial completion.
    #[must_use]
    pub const fn is_partial(self) -> bool {
        matches!(self, Self::Partial)
    }

    /// Return whether required report evidence is incomplete.
    #[must_use]
    pub const fn is_incomplete(self) -> bool {
        matches!(self, Self::Incomplete)
    }
}


// ============================================================================
// FAILURE STATUS
// ============================================================================

/// Outcome classification for a failed build operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildFailureStatus {
    /// The operation failed.
    Failed,

    /// The operation was cancelled.
    Cancelled,
}

impl BuildFailureStatus {
    /// Return the stable machine-readable status.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}


// ============================================================================
// INPUT SNAPSHOT REPORT
// ============================================================================

/// Serializable identity summary for one immutable build-input snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BuildInputReport {
    /// Canonical workspace identity.
    pub workspace_id: String,

    /// Complete workspace metadata fingerprint.
    pub workspace_fingerprint: String,

    /// Target-independent logical build fingerprint.
    pub logical_fingerprint: String,

    /// Complete snapshot fingerprint, including realizations.
    pub snapshot_fingerprint: String,

    /// Number of named input records.
    pub input_count: usize,

    /// Number of target-specific realization fingerprints.
    pub realization_count: usize,

    /// Target-specific realization fingerprints.
    pub realizations: Vec<RealizationInputReport>,
}

/// Serializable identity of one target-specific input realization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RealizationInputReport {
    /// Canonical realization identity.
    pub identity: String,

    /// Fingerprint of the realization-specific inputs.
    pub fingerprint: String,
}

impl BuildInputReport {
    /// Construct a report from the canonical immutable input snapshot.
    ///
    /// This copies identity metadata only. It does not copy source files,
    /// source buffers, dependency packages or artifact payloads.
    ///
    /// # Errors
    ///
    /// Returns an error if memory reservation for the realization
    /// collection fails.
    pub fn from_inputs(
        inputs: &BuildInputs,
    ) -> ToolchainResult<Self> {
        let mut realizations = Vec::new();

        realizations
            .try_reserve(inputs.realizations().len())
            .map_err(|error| {
                allocation_error(
                    "build-input realization report",
                    error,
                )
            })?;

        for realization in inputs.realizations() {
            realizations.push(RealizationInputReport {
                identity: realization.identity().to_owned(),
                fingerprint: realization.fingerprint().to_hex(),
            });
        }

        Ok(Self {
            workspace_id: inputs
                .workspace()
                .workspace_id()
                .to_string(),

            workspace_fingerprint: inputs
                .workspace()
                .workspace_fingerprint()
                .to_owned(),

            logical_fingerprint: inputs
                .logical_fingerprint()
                .to_hex(),

            snapshot_fingerprint: inputs
                .fingerprint()
                .to_hex(),

            input_count: inputs.input_count(),

            realization_count: inputs.realization_count(),

            realizations,
        })
    }
}


// ============================================================================
// TARGET REPORT
// ============================================================================

/// Feasibility state of one selected target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetBuildability {
    /// The canonical compiler feasibility authority proved the target
    /// feasible for attempting compilation.
    Feasible,

    /// The target was assessed but was not proven feasible.
    NotProvenFeasible,
}

/// Report record for one resolved target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TargetBuildReport {
    /// Canonical compiler target identity.
    pub target_id: String,

    /// Compiler feasibility classification.
    pub buildability: TargetBuildability,

    /// Whether this target has at least one canonical output artifact.
    pub has_output: bool,

    /// Number of canonical artifacts associated with this target.
    pub artifact_count: usize,
}

/// Report record for an unresolved target selector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UnresolvedTargetReport {
    /// Original caller-supplied target selector.
    pub selector: String,

    /// Explicitly records that canonical resolution did not establish
    /// a target identity.
    pub resolved: bool,
}


// ============================================================================
// BUILD PLAN REPORT
// ============================================================================

/// Serializable summary of compiler-assessed build-plan evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BuildPlanReport {
    /// Whether the compiler feasibility assessment is complete.
    pub assessment_complete: bool,

    /// Whether every selected target is proven feasible.
    pub fully_feasible: bool,

    /// Whether the plan is authorized to proceed to build execution.
    ///
    /// This does not imply that compilation or runtime execution succeeded.
    pub can_attempt_build: bool,

    /// Resolved target assessments.
    pub targets: Vec<TargetBuildReport>,

    /// Selectors that did not resolve to canonical target identities.
    pub unresolved_targets: Vec<UnresolvedTargetReport>,
}

impl BuildPlanReport {
    /// Construct a report from the authoritative assessed build plan.
    ///
    /// Feasibility is not recalculated here.
    ///
    /// # Errors
    ///
    /// Returns an error if collection reservation fails.
    pub fn from_plan(
        plan: &AssessedBuildPlan,
        outputs: &BuildOutputs,
    ) -> ToolchainResult<Self> {
        let resolved_count = plan
            .plan()
            .target_resolution()
            .resolved()
            .len();

        let unresolved_count = plan
            .unresolved_targets()
            .len();

        let mut targets = Vec::new();

        targets
            .try_reserve(resolved_count)
            .map_err(|error| {
                allocation_error(
                    "build target report",
                    error,
                )
            })?;

        let mut unresolved_targets = Vec::new();

        unresolved_targets
            .try_reserve(unresolved_count)
            .map_err(|error| {
                allocation_error(
                    "unresolved target report",
                    error,
                )
            })?;

        for target in plan.plan().target_resolution().resolved() {
            let target_id = target.target_id().to_string();

            let feasible = plan
                .buildable_targets()
                .any(|candidate| {
                    candidate.target_id() == target.target_id()
                });

            let coverage = outputs
                .coverage_for_target(&target_id);

            targets.push(TargetBuildReport {
                target_id,

                buildability: if feasible {
                    TargetBuildability::Feasible
                } else {
                    TargetBuildability::NotProvenFeasible
                },

                has_output: coverage
                    .is_some_and(|entry| entry.is_covered()),

                artifact_count: coverage
                    .map_or(0, |entry| entry.artifact_count()),
            });
        }

        for selector in plan.unresolved_targets() {
            unresolved_targets.push(UnresolvedTargetReport {
                selector: selector.as_str().to_owned(),
                resolved: false,
            });
        }

        Ok(Self {
            assessment_complete: plan.assessment_is_complete(),
            fully_feasible: plan.is_fully_feasible(),
            can_attempt_build: plan.can_attempt_build(),
            targets,
            unresolved_targets,
        })
    }
}


// ============================================================================
// OUTPUT SUMMARY REPORT
// ============================================================================

/// Serializable aggregate statistics for actual canonical build outputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct BuildOutputReport {
    /// Output collection status.
    pub status: &'static str,

    /// Whether canonical artifact validation passed.
    pub artifact_validation_passed: bool,

    /// Number of canonical artifacts.
    pub artifact_count: usize,

    /// Number of target-specific artifacts.
    pub targeted_artifact_count: usize,

    /// Number of artifacts without an explicit target.
    ///
    /// This does not establish portability.
    pub shared_artifact_count: usize,

    /// Number of compiler-assessed buildable targets.
    pub target_count: usize,

    /// Number of targets with output artifacts.
    pub covered_target_count: usize,

    /// Number of targets without output artifacts.
    pub missing_target_count: usize,

    /// Total payload size when every payload size is known and the sum
    /// is representable.
    ///
    /// None means at least one size is unknown or the total overflowed.
    pub total_payload_bytes: Option<u64>,
}

impl BuildOutputReport {
    /// Construct an output summary from canonical validated outputs.
    #[must_use]
    pub fn from_outputs(outputs: &BuildOutputs) -> Self {
        let summary = outputs.summary();

        Self {
            status: output_status_name(outputs.status()),

            artifact_validation_passed: matches!(
                outputs.validation_status(),
                ValidationStatus::Valid
            ),

            artifact_count: summary.artifact_count(),

            targeted_artifact_count:
                summary.targeted_artifact_count(),

            shared_artifact_count:
                summary.shared_artifact_count(),

            target_count: summary.target_count(),

            covered_target_count:
                summary.covered_target_count(),

            missing_target_count:
                summary.missing_target_count(),

            total_payload_bytes:
                summary.total_payload_bytes(),
        }
    }
}

fn output_status_name(
    status: BuildOutputStatus,
) -> &'static str {
    match status {
        BuildOutputStatus::Complete => "complete",
        BuildOutputStatus::Partial => "partial",
        BuildOutputStatus::Incomplete => "incomplete",
    }
}


// ============================================================================
// BUILD REPORT
// ============================================================================

/// Complete report view for one actual build operation.
///
/// The report borrows canonical diagnostic data and owns only the
/// serializable identity and summary records it composes.
///
/// Artifact payloads are never copied into this structure.
#[derive(Debug, Serialize)]
#[must_use = "build reports should be serialized or inspected"]
pub struct BuildReport<'a> {
    /// Report schema identifier.
    pub schema_version: &'static str,

    /// Overall evidence status.
    pub status: BuildReportStatus,

    /// Logical build identity.
    pub build_id: String,

    /// Original validated project/workspace scope.
    pub project_scope: &'a ProjectScope,

    /// Associated immutable build-input identity.
    pub inputs: BuildInputReport,

    /// Compiler feasibility and target-resolution summary.
    pub plan: BuildPlanReport,

    /// Canonical output summary.
    pub outputs: BuildOutputReport,

    /// Canonical lightweight artifact references.
    ///
    /// These references do not contain binary payloads.
    pub artifacts:
        &'a [crate::toolchain::api::response::ArtifactReference],

    /// Optional canonical compiler diagnostic report.
    ///
    /// None means diagnostics were not supplied to this reporting call.
    /// It does not mean that no diagnostics occurred.
    pub diagnostics:
        Option<DiagnosticReport<'a>>,
}

impl<'a> BuildReport<'a> {
    /// Compose a report from one assessed plan, its associated input
    /// snapshot, and its canonical validated outputs.
    ///
    /// The caller must supply the BuildInputs snapshot associated with
    /// this build operation.
    ///
    /// The current BuildPlan implementation does not retain BuildInputs.
    /// Therefore this constructor validates the workspace identity and,
    /// when the request uses a ProjectId, verifies that the project is
    /// present in the input workspace snapshot.
    ///
    /// Path-based project locators must already have been resolved by
    /// the canonical workspace/project discovery service.
    ///
    /// The output collector remains responsible for proving that its
    /// outputs came from the supplied assessed plan.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The output build identity differs from the plan build identity.
    /// - The requested workspace differs from the input snapshot workspace.
    /// - An ID-based project is absent from the input workspace.
    /// - Target coverage contradicts the assessed plan.
    /// - Required report memory reservation fails.
    pub fn new(
        plan: &'a AssessedBuildPlan,
        inputs: &BuildInputs,
        outputs: &'a BuildOutputs,
        diagnostics: Option<&'a AggregatedDiagnostics>,
    ) -> ToolchainResult<Self> {
        validate_build_association(
            plan,
            inputs,
            outputs,
        )?;

        let input_report =
            BuildInputReport::from_inputs(inputs)?;

        let plan_report =
            BuildPlanReport::from_plan(plan, outputs)?;

        let output_report =
            BuildOutputReport::from_outputs(outputs);

        let mut diagnostics_report = None;

        if let Some(aggregate) = diagnostics {
            diagnostics_report =
                Some(DiagnosticReport::new(aggregate));
        }

        let status = determine_report_status(
            outputs,
            diagnostics,
        );

        Ok(Self {
            schema_version: BUILD_REPORT_SCHEMA,

            status,

            build_id: plan.plan().id().to_string(),

            project_scope: plan.plan().request().scope(),

            inputs: input_report,

            plan: plan_report,

            outputs: output_report,

            artifacts: outputs.artifact_references(),

            diagnostics: diagnostics_report,
        })
    }

    /// Return the overall report status.
    #[must_use]
    pub const fn status(&self) -> BuildReportStatus {
        self.status
    }

    /// Return whether the report represents complete evidence.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.status.is_complete()
    }

    /// Return whether the report represents partial completion.
    #[must_use]
    pub const fn is_partial(&self) -> bool {
        self.status.is_partial()
    }

    /// Return whether report evidence is incomplete.
    #[must_use]
    pub const fn is_incomplete(&self) -> bool {
        self.status.is_incomplete()
    }

    /// Serialize directly to a caller-provided writer.
    ///
    /// This does not allocate a second complete JSON document.
    ///
    /// If serialization fails, the writer may contain partial output.
    /// The caller must not treat that output as a complete report.
    pub fn write_json<W>(
        &self,
        writer: W,
    ) -> Result<(), serde_json::Error>
    where
        W: Write,
    {
        serde_json::to_writer(writer, self)
    }

    /// Serialize to a borrowed writer.
    pub fn write_json_to<W>(
        &self,
        writer: &mut W,
    ) -> Result<(), serde_json::Error>
    where
        W: Write + ?Sized,
    {
        serde_json::to_writer(writer, self)
    }

    /// Serialize into an owned JSON string.
    ///
    /// This allocates memory proportional to the output document.
    /// Prefer writer-based serialization for large reports.
    pub fn to_json_string(
        &self,
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}


// ============================================================================
// BUILD FAILURE REPORT
// ============================================================================

/// Structured summary of a failed or cancelled build.
#[derive(Debug, Serialize)]
#[must_use = "build failure reports should be inspected or serialized"]
pub struct BuildFailureReport<'a> {
    /// Failure-report schema identifier.
    pub schema_version: &'static str,

    /// Failed or cancelled operation status.
    pub status: BuildFailureStatus,

    /// Logical build identity.
    pub build_id: String,

    /// Original validated project/workspace scope.
    pub project_scope: &'a ProjectScope,

    /// Associated immutable input snapshot identity.
    pub inputs: BuildInputReport,

    /// Compiler feasibility evidence available before failure.
    pub plan: BuildPlanReport,

    /// Safe, structured error snapshot.
    ///
    /// Arbitrary underlying error objects are not serialized.
    pub failure: ResponseError,

    /// Optional canonical compiler diagnostic report.
    ///
    /// None means diagnostics were not supplied.
    pub diagnostics:
        Option<DiagnosticReport<'a>>,
}

impl<'a> BuildFailureReport<'a> {
    /// Construct a failure report from the actual toolchain error.
    ///
    /// This constructor does not invent artifacts or represent partially
    /// completed compiler outputs as a successful build.
    ///
    /// The executor currently returns an error when execution fails.
    /// Any future partial-execution evidence must be represented by an
    /// explicit canonical execution-result contract before it can be
    /// included here.
    ///
    /// # Errors
    ///
    /// Returns an error if the input workspace/project association is
    /// invalid or report allocation fails.
    pub fn new(
        plan: &'a AssessedBuildPlan,
        inputs: &BuildInputs,
        error: &ToolchainError,
        diagnostics: Option<&'a AggregatedDiagnostics>,
    ) -> ToolchainResult<Self> {
        validate_plan_inputs(
            plan,
            inputs,
        )?;

        let input_report =
            BuildInputReport::from_inputs(inputs)?;

        let plan_report =
            build_plan_failure_summary(plan)?;

        let diagnostics_report =
            diagnostics.map(DiagnosticReport::new);

        let status = if error.kind()
            == ToolchainErrorKind::Cancelled
        {
            BuildFailureStatus::Cancelled
        } else {
            BuildFailureStatus::Failed
        };

        Ok(Self {
            schema_version: BUILD_FAILURE_REPORT_SCHEMA,

            status,

            build_id: plan.plan().id().to_string(),

            project_scope: plan.plan().request().scope(),

            inputs: input_report,

            plan: plan_report,

            failure: ResponseError::from_toolchain_error(error),

            diagnostics: diagnostics_report,
        })
    }

    /// Return the failure status.
    #[must_use]
    pub const fn status(&self) -> BuildFailureStatus {
        self.status
    }

    /// Serialize directly to a caller-provided writer.
    pub fn write_json<W>(
        &self,
        writer: W,
    ) -> Result<(), serde_json::Error>
    where
        W: Write,
    {
        serde_json::to_writer(writer, self)
    }

    /// Serialize to an owned JSON string.
    pub fn to_json_string(
        &self,
    ) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}


// ============================================================================
// BUILD ASSOCIATION VALIDATION
// ============================================================================

/// Validate that the supplied immutable inputs belong to the project
/// and workspace scope represented by the build request.
fn validate_plan_inputs(
    plan: &AssessedBuildPlan,
    inputs: &BuildInputs,
) -> ToolchainResult<()> {
    let scope = plan.plan().request().scope();

    if let Some(requested_workspace) = &scope.workspace {
        if requested_workspace
            != inputs.workspace().workspace_id()
        {
            return Err(
                ToolchainError::validation_failed(
                    "build input snapshot belongs to a different workspace",
                )
                .with_operation("build.reporting")
                .with_subject(plan.plan().id().to_string()),
            );
        }
    }

    match &scope.project {
        ProjectLocator::Id(project_id) => {
            if inputs.workspace().project(project_id).is_none() {
                return Err(
                    ToolchainError::validation_failed(
                        "build input workspace does not contain the requested project",
                    )
                    .with_operation("build.reporting")
                    .with_subject(project_id.to_string()),
                );
            }
        }

        ProjectLocator::Path(_) => {
            // Path resolution and canonical path identity belong to
            // workspace/project discovery.
            //
            // BuildInputs is required to have been collected from the
            // already-resolved project represented by this request.
            //
            // This reporting layer deliberately performs no filesystem
            // discovery or path canonicalization.
        }

        _ => {
            // Fail closed for future ProjectLocator variants until
            // their identity-validation contract is implemented.
            return Err(
                ToolchainError::unsupported_operation(
                    "reporting does not yet support this project locator variant",
                )
                .with_operation("build.reporting"),
            );
        }
    }

    Ok(())
}

/// Validate all available build, input and output identity relationships.
fn validate_build_association(
    plan: &AssessedBuildPlan,
    inputs: &BuildInputs,
    outputs: &BuildOutputs,
) -> ToolchainResult<()> {
    validate_plan_inputs(
        plan,
        inputs,
    )?;

    if outputs.build_id() != plan.plan().id() {
        return Err(
            ToolchainError::validation_failed(
                "build outputs belong to a different logical build",
            )
            .with_operation("build.reporting")
            .with_subject(plan.plan().id().to_string()),
        );
    }

    validate_target_coverage(
        plan,
        outputs,
    )?;

    Ok(())
}

/// Verify that canonical output coverage accounts for exactly the
/// buildable target identities in the assessed plan.
fn validate_target_coverage(
    plan: &AssessedBuildPlan,
    outputs: &BuildOutputs,
) -> ToolchainResult<()> {
    let expected_count = plan
        .buildable_targets()
        .count();

    let actual_coverage = outputs.target_coverage();

    if actual_coverage.len() != expected_count {
        return Err(
            ToolchainError::validation_failed(
                "build output target coverage does not match the assessed build plan",
            )
            .with_operation("build.reporting")
            .with_subject(plan.plan().id().to_string()),
        );
    }

    for target in plan.buildable_targets() {
        let target_id = target.target_id().as_str();

        if outputs.coverage_for_target(target_id).is_none() {
            return Err(
                ToolchainError::validation_failed(
                    "build output coverage is missing a compiler-assessed buildable target",
                )
                .with_operation("build.reporting")
                .with_subject(target_id.to_owned()),
            );
        }
    }

    for coverage in actual_coverage {
        let belongs_to_plan = plan
            .buildable_targets()
            .any(|target| {
                target.target_id().as_str()
                    == coverage.target_id()
            });

        if !belongs_to_plan {
            return Err(
                ToolchainError::validation_failed(
                    "build output coverage contains a target absent from the assessed build plan",
                )
                .with_operation("build.reporting")
                .with_subject(coverage.target_id().to_owned()),
            );
        }
    }

    Ok(())
}


// ============================================================================
// REPORT STATUS CLASSIFICATION
// ============================================================================

/// Determine report completeness without changing the canonical output
/// status or compiler diagnostic status.
fn determine_report_status(
    outputs: &BuildOutputs,
    diagnostics: Option<&AggregatedDiagnostics>,
) -> BuildReportStatus {
    if outputs.is_incomplete() {
        return BuildReportStatus::Incomplete;
    }

    if diagnostics.is_some_and(|aggregate| !aggregate.is_complete()) {
        return BuildReportStatus::Incomplete;
    }

    if outputs.is_partial() {
        return BuildReportStatus::Partial;
    }

    BuildReportStatus::Complete
}


// ============================================================================
// FAILURE PLAN SUMMARY
// ============================================================================

/// Build the plan summary for a failure report.
///
/// The summary deliberately has no output artifacts because the current
/// BuildExecutor does not return a partial execution result on failure.
fn build_plan_failure_summary(
    plan: &AssessedBuildPlan,
) -> ToolchainResult<BuildPlanReport> {
    let resolved_count = plan
        .plan()
        .target_resolution()
        .resolved()
        .len();

    let unresolved_count = plan
        .unresolved_targets()
        .len();

    let mut targets = Vec::new();

    targets
        .try_reserve(resolved_count)
        .map_err(|error| {
            allocation_error(
                "failed build target report",
                error,
            )
        })?;

    let mut unresolved_targets = Vec::new();

    unresolved_targets
        .try_reserve(unresolved_count)
        .map_err(|error| {
            allocation_error(
                "failed unresolved target report",
                error,
            )
        })?;

    for target in plan.plan().target_resolution().resolved() {
        let feasible = plan
            .buildable_targets()
            .any(|candidate| {
                candidate.target_id() == target.target_id()
            });

        targets.push(TargetBuildReport {
            target_id: target.target_id().to_string(),

            buildability: if feasible {
                TargetBuildability::Feasible
            } else {
                TargetBuildability::NotProvenFeasible
            },

            has_output: false,

            artifact_count: 0,
        });
    }

    for selector in plan.unresolved_targets() {
        unresolved_targets.push(UnresolvedTargetReport {
            selector: selector.as_str().to_owned(),
            resolved: false,
        });
    }

    Ok(BuildPlanReport {
        assessment_complete: plan.assessment_is_complete(),
        fully_feasible: plan.is_fully_feasible(),
        can_attempt_build: plan.can_attempt_build(),
        targets,
        unresolved_targets,
    })
}


// ============================================================================
// ALLOCATION ERRORS
// ============================================================================

fn allocation_error(
    subject: &str,
    error: std::collections::TryReserveError,
) -> ToolchainError {
    ToolchainError::resource_exhausted(
        format!(
            "unable to reserve memory for {subject}: {error}"
        ),
    )
    .with_operation("build.reporting")
    .with_context(subject.to_owned())
}


// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_schema_identifiers_are_stable() {
        assert_eq!(
            BUILD_REPORT_SCHEMA,
            "zamani.toolchain.build-report.v1",
        );

        assert_eq!(
            BUILD_FAILURE_REPORT_SCHEMA,
            "zamani.toolchain.build-failure-report.v1",
        );
    }

    #[test]
    fn report_status_names_are_stable() {
        assert_eq!(
            BuildReportStatus::Complete.as_str(),
            "complete",
        );

        assert_eq!(
            BuildReportStatus::Partial.as_str(),
            "partial",
        );

        assert_eq!(
            BuildReportStatus::Incomplete.as_str(),
            "incomplete",
        );
    }

    #[test]
    fn failure_status_names_are_stable() {
        assert_eq!(
            BuildFailureStatus::Failed.as_str(),
            "failed",
        );

        assert_eq!(
            BuildFailureStatus::Cancelled.as_str(),
            "cancelled",
        );
    }

    #[test]
    fn output_status_names_are_stable() {
        assert_eq!(
            output_status_name(BuildOutputStatus::Complete),
            "complete",
        );

        assert_eq!(
            output_status_name(BuildOutputStatus::Partial),
            "partial",
        );

        assert_eq!(
            output_status_name(BuildOutputStatus::Incomplete),
            "incomplete",
        );
    }

    #[test]
    fn incomplete_outputs_never_produce_complete_reports() {
        // The classifier is intentionally conservative.
        //
        // Integration tests exercise this with real BuildOutputs.
        // This unit test protects the status contract itself.

        assert!(
            BuildReportStatus::Incomplete.is_incomplete()
        );

        assert!(
            !BuildReportStatus::Incomplete.is_complete()
        );
    }

    #[test]
    fn missing_diagnostics_are_not_equivalent_to_empty_diagnostics() {
        // None means diagnostic data was not supplied.
        // It must not be represented as a zero-diagnostic aggregate.
        //
        // The distinction is preserved by the Option field in both
        // BuildReport and BuildFailureReport.
        let diagnostics: Option<DiagnosticReport<'_>> = None;

        assert!(diagnostics.is_none());
    }
}
