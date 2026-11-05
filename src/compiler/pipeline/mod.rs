
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Pipeline Module.
//!
//! Path:
//!     src/compiler/pipeline/mod.rs
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
//! This module is the public composition boundary for the Zamani compiler
//! pipeline.
//!
//! It integrates the existing pipeline implementation modules:
//!
//!     stage.rs
//!         Defines the canonical compiler-stage contract.
//!
//!     stages.rs
//!         Defines standard pipeline stages and domain extension points.
//!
//!     ordering.rs
//!         Validates stage dependencies and creates deterministic ordering.
//!
//!     execution.rs
//!         Executes stages and preserves their outputs and execution records.
//!
//!     failure.rs
//!         Provides structured pipeline failure information.
//!
//!     recovery.rs
//!         Evaluates retry and fallback policies.
//!
//!     orchestration.rs
//!         Coordinates the lifecycle of a pipeline invocation.
//!
//! This module does not implement a second version of any of those systems.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - Public composition of the pipeline subsystem.
//! - Public module visibility.
//! - Stable, documented re-exports.
//! - Public naming for potentially conflicting types.
//! - Pipeline-level API discoverability.
//!
//! This module does NOT own:
//!
//! - Compiler-stage implementations.
//! - Stage registration or dynamic plugin loading.
//! - Stage dependency algorithms.
//! - Stage execution.
//! - Retry execution.
//! - Recovery execution.
//! - Compilation lifecycle state.
//! - Compilation configuration.
//! - Resource-budget accounting.
//! - Cancellation-token implementation.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership analysis.
//! - Effect analysis.
//! - Capability semantics.
//! - Requirement semantics.
//! - Canonical intermediate representations.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Physical scheduling.
//! - Physical routing.
//! - Backend implementations.
//! - Artifact generation.
//! - Runtime execution.
//!
//! These responsibilities remain with their existing authoritative modules.
//!
//! ============================================================================
//! DEPENDENCY ARCHITECTURE
//! ============================================================================
//!
//! The intended dependency direction is:
//!
//!     compiler::configuration
//!                |
//!                v
//!     compiler::context
//!                |
//!                v
//!       pipeline::stage
//!                |
//!                v
//!       pipeline::stages
//!                |
//!                v
//!       pipeline::ordering
//!                |
//!                v
//!       pipeline::execution
//!                |
//!                v
//!      pipeline::orchestration
//!
//!       pipeline::failure
//!                |
//!                v
//!       pipeline::recovery
//!
//! Failure normalization and recovery decisions are consumed by the
//! orchestration layer when an explicit recovery integration is implemented.
//!
//! This module only exposes these existing components.
//!
//! ============================================================================
//! POCO-REAF AND SCALABILITY
//! ============================================================================
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module introduces no fixed limits on:
//!
//! - Pipeline stages.
//! - Stage dependencies.
//! - Compilation units.
//! - Modules.
//! - Functions.
//! - CPUs.
//! - CPU cores.
//! - Workers.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Accelerators.
//! - Distributed nodes.
//! - Memory.
//! - Storage.
//! - Qubits.
//! - Tensor dimensions.
//! - Tensor rank.
//! - IR operations.
//! - Hardware capacity.
//!
//! The pipeline consumes configurable operational budgets through the
//! existing CompilationContext and budget subsystem.
//!
//! An unlimited configured budget does not guarantee unlimited physical
//! resources.
//!
//! Actual execution remains constrained by available memory, processor
//! capacity, operating-system limits, backend capabilities and security
//! policy.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! Deterministic stage ordering is owned by pipeline::ordering.
//!
//! Deterministic sequential stage execution is owned by pipeline::execution.
//!
//! This composition module does not introduce concurrency or alter execution
//! order.
//!
//! Parallel execution must be introduced through an explicitly designed
//! executor that preserves dependency, output, cancellation and failure
//! contracts.
//!
//! ============================================================================
//! CONDITIONAL DOMAIN SUPPORT
//! ============================================================================
//!
//! The standard stage definitions distinguish:
//!
//! - Required stages.
//! - Conditional stages.
//! - Policy-controlled stages.
//!
//! Domain-specific stages can be registered using the existing Stage
//! and StageDescriptor contracts.
//!
//! A program that does not use a domain must not be forced through that
//! domain's lowering or realization stages.
//!
//! A conditional stage must not be treated as successfully completed merely
//! because it was skipped.
//!
//! Required-stage completion remains mandatory for successful compilation.
//!
//! ============================================================================
//! FAILURE AND RECOVERY
//! ============================================================================
//!
//! Pipeline failures are represented by PipelineFailure.
//!
//! Recovery decisions are represented by RecoveryDecision.
//!
//! RecoveryPlanner evaluates explicit policy and safety evidence.
//!
//! The recovery module does not itself execute retries or fallbacks.
//!
//! In particular:
//!
//! - A retryable error does not prove that replay is safe.
//! - Retrying must require explicit replay-safety evidence.
//! - Cancellation must not be silently overridden.
//! - Internal correctness-invariant failures must not be retried.
//! - Fallbacks must preserve semantics, requirements and constraints.
//! - An unverified fallback must never be reported as successful.
//!
//! The actual retry or fallback execution must be implemented by the
//! orchestration/execution integration, not by this module.
//!
//! ============================================================================
//! GRAMMAR BOUNDARY
//! ============================================================================
//!
//! The authoritative grammar composition roots include:
//!
//!     grammar/Zamani.g4
//!     grammar/antlr/ZamaniParser.g4
//!     grammar/core/program.g4
//!     grammar/compile/compile.g4
//!
//! The grammar defines source-level language constructs and compilation
//! intent.
//!
//! This module exposes compiler implementation infrastructure.
//!
//! It does not parse grammar declarations, define language syntax, or
//! interpret source-level compilation intent.
//!
//! The frontend and semantic authorities must validate source declarations
//! before those declarations influence compiler execution.
//!
//! ============================================================================
//! MODULE DECLARATIONS
//! ============================================================================

/// Canonical compiler-stage contracts.
pub mod stage;

/// Standard compiler pipeline stages and domain extension points.
pub mod stages;

/// Dependency validation and deterministic stage ordering.
pub mod ordering;

/// Stage execution and execution reports.
pub mod execution;

/// Structured pipeline failure representation and normalization.
pub mod failure;

/// Recovery policy evaluation and safety evidence.
pub mod recovery;

/// Pipeline lifecycle orchestration.
pub mod orchestration;

// ============================================================================
// API VERSIONING
// ============================================================================

/// Version of the composed pipeline module API.
///
/// Individual pipeline components retain their own API versions.
/// This version identifies the public composition surface.
pub const PIPELINE_API_VERSION: u16 = 1;

// ============================================================================
// STAGE CONTRACT RE-EXPORTS
// ============================================================================

pub use stage::{
    Stage,
    StageConcurrency,
    StageDecision,
    StageDescriptor,
    StageDescriptorError,
    StageDeterminism,
    StageError,
    StageErrorKind,
    StageExecution,
    StageExecutionContext,
    StageId,
    StageIdError,
    StageInput,
    StageOutcome,
    StageOutput,
    STAGE_API_VERSION,
};

// ============================================================================
// STANDARD STAGE RE-EXPORTS
// ============================================================================

pub use stages::{
    domain_stage_descriptor,
    standard_stage_definitions,
    standard_stage_id,
    validate_standard_definitions,
    StandardStageDefinition,
    StandardStageError,
    StandardStageRole,
    StageActivation,
    PIPELINE_STAGES_API_VERSION,
};

// ============================================================================
// ORDERING RE-EXPORTS
// ============================================================================

pub use ordering::{
    OrderingError,
    StageOrdering,
    PIPELINE_ORDERING_API_VERSION,
};

// ============================================================================
// EXECUTION RE-EXPORTS
// ============================================================================

// The execution subsystem and orchestration subsystem both define
// execution-report-related public types.
//
// Explicit aliases avoid ambiguous names and preserve access to both
// contracts without changing their original definitions.

pub use execution::{
    ExecutionError,
    ExecutionErrorKind,
    ExecutionReport as StageExecutionReport,
    PipelineExecutor,
    StageExecutionRecord as ExecutorStageExecutionRecord,
    StageExecutionStatus as ExecutorStageExecutionStatus,
    StageInputs,
    PIPELINE_EXECUTION_API_VERSION,
};

// ============================================================================
// FAILURE RE-EXPORTS
// ============================================================================

pub use failure::{
    FailureCategory,
    FailureCode,
    FailureContext,
    FailureDisposition,
    FailureOrigin,
    FailureRelation,
    FailureReport,
    FailureSeverity,
    FailureStageRecord,
    FailureStageStatus,
    PipelineFailure,
    RetryDeclaration,
    PIPELINE_FAILURE_API_VERSION,
};

// ============================================================================
// RECOVERY RE-EXPORTS
// ============================================================================

pub use recovery::{
    FallbackEvidence,
    RecoveryAction,
    RecoveryDecision,
    RecoveryPlanner,
    RecoveryPolicy,
    RecoveryPolicyError,
    RecoveryReason,
    RecoveryRequest,
    RecoveryResult,
    RetrySafety,
    PIPELINE_RECOVERY_API_VERSION,
};

// ============================================================================
// ORCHESTRATION RE-EXPORTS
// ============================================================================

// These aliases deliberately distinguish orchestration records from
// executor records.
//
// The underlying types remain owned by orchestration.rs.

pub use orchestration::{
    ExecutionReport as OrchestrationExecutionReport,
    PipelineOrchestrationError,
    PipelineOrchestrationErrorKind,
    PipelineOrchestrator,
    StageExecutionRecord as OrchestrationStageExecutionRecord,
    StageExecutionStatus as OrchestrationStageExecutionStatus,
    PIPELINE_ORCHESTRATION_API_VERSION,
};

// ============================================================================
// PUBLIC API CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // Module API
    // ------------------------------------------------------------------------

    #[test]
    fn pipeline_api_version_is_available() {
        assert_eq!(PIPELINE_API_VERSION, 1);
    }

    #[test]
    fn component_api_versions_are_available() {
        assert_eq!(STAGE_API_VERSION, 1);
        assert_eq!(PIPELINE_STAGES_API_VERSION, 1);
        assert_eq!(PIPELINE_ORDERING_API_VERSION, 1);
        assert_eq!(PIPELINE_EXECUTION_API_VERSION, 1);
        assert_eq!(PIPELINE_FAILURE_API_VERSION, 1);
        assert_eq!(PIPELINE_RECOVERY_API_VERSION, 1);
        assert_eq!(PIPELINE_ORCHESTRATION_API_VERSION, 1);
    }

    // ------------------------------------------------------------------------
    // Stage contract
    // ------------------------------------------------------------------------

    #[test]
    fn stage_identifiers_are_exposed_through_pipeline_root() {
        let identifier = StageId::new("compiler.frontend.parse")
            .expect("valid stage identifier");

        assert_eq!(
            identifier.as_str(),
            "compiler.frontend.parse"
        );
    }

    #[test]
    fn stage_identifiers_do_not_have_an_arbitrary_length_limit() {
        let identifier = format!(
            "compiler.{}",
            "stage".repeat(10_000)
        );

        let stage_id = StageId::new(identifier.clone())
            .expect("dynamically sized stage identifiers are supported");

        assert_eq!(stage_id.as_str(), identifier);
    }

    // ------------------------------------------------------------------------
    // Standard stage definitions
    // ------------------------------------------------------------------------

    #[test]
    fn standard_stage_definitions_are_available() {
        let definitions = standard_stage_definitions()
            .expect("standard stage definitions must be valid");

        assert!(
            !definitions.is_empty(),
            "the standard pipeline must define its standard stages"
        );

        validate_standard_definitions(&definitions)
            .expect("standard stage definitions must validate");
    }

    #[test]
    fn standard_stage_definitions_preserve_activation_metadata() {
        let definitions = standard_stage_definitions()
            .expect("standard stage definitions must be valid");

        assert!(
            definitions.iter().any(|definition| {
                definition.activation() == StageActivation::Required
            }),
            "the standard pipeline must define required stages"
        );

        assert!(
            definitions.iter().any(|definition| {
                definition.activation() == StageActivation::Conditional
            }),
            "the standard pipeline must support conditional stages"
        );
    }

    // ------------------------------------------------------------------------
    // Ordering
    // ------------------------------------------------------------------------

    #[test]
    fn ordering_api_is_available_through_pipeline_root() {
        let definitions = standard_stage_definitions()
            .expect("standard stage definitions must be valid");

        let descriptors = definitions
            .iter()
            .map(|definition| definition.descriptor().clone())
            .collect::<Vec<_>>();

        let ordering = StageOrdering::from_slice(&descriptors)
            .expect("standard stage dependencies must be valid");

        assert_eq!(
            ordering.stage_count(),
            descriptors.len()
        );

        assert!(!ordering.is_empty());
    }

    // ------------------------------------------------------------------------
    // Recovery API
    // ------------------------------------------------------------------------

    #[test]
    fn recovery_defaults_are_conservative() {
        let policy = RecoveryPolicy::new();

        assert_eq!(
            policy.max_retries(),
            0,
            "retries must be explicitly enabled"
        );

        assert!(
            !policy.fallback_allowed(),
            "fallback must be explicitly enabled"
        );
    }

    #[test]
    fn recovery_retry_budget_is_configurable() {
        let policy = RecoveryPolicy::new()
            .with_max_retries(usize::MAX);

        assert_eq!(
            policy.max_retries(),
            usize::MAX
        );
    }

    // ------------------------------------------------------------------------
    // Orchestration API
    // ------------------------------------------------------------------------

    #[test]
    fn orchestration_api_is_available() {
        let orchestrator = PipelineOrchestrator::new();

        let _ = orchestrator;
    }

    // ------------------------------------------------------------------------
    // Public type disambiguation
    // ------------------------------------------------------------------------

    #[test]
    fn executor_and_orchestration_reports_have_distinct_public_names() {
        let _: Option<StageExecutionReport> = None;
        let _: Option<OrchestrationExecutionReport> = None;

        let _: Option<ExecutorStageExecutionRecord> = None;
        let _: Option<OrchestrationStageExecutionRecord> = None;

        let _: Option<ExecutorStageExecutionStatus> = None;
        let _: Option<OrchestrationStageExecutionStatus> = None;
    }
}
