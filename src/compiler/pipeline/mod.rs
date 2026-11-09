
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Pipeline Facade.
//!
//! Path: src/compiler/pipeline/mod.rs
//! Edition: 2021
//! Minimum supported Rust version: 1.97
//!
//! # Purpose
//!
//! This module is the public composition boundary for the compiler pipeline.
//!
//! It exposes the existing pipeline subsystems without introducing competing
//! implementations of their responsibilities.
//!
//! The pipeline coordinates compiler work. It does not define source-language
//! semantics, own canonical IRs, allocate physical hardware, or execute
//! compiled programs.
//!
//! # Component ownership
//!
//! | Module | Authoritative responsibility |
//! |---|---|
//! | stage | Stage identity, descriptors, contracts, inputs, outputs and errors |
//! | stages | Standard stage definitions and domain-stage descriptors |
//! | ordering | Dependency validation and deterministic topological ordering |
//! | execution | Stage invocation, prerequisite outputs and execution records |
//! | orchestration | Pipeline invocation lifecycle and coordination |
//! | failure | Structured failure classification and reporting |
//! | recovery | Recovery policy evaluation and safety evidence |
//!
//! A declaration or descriptor is not proof that a stage implementation has
//! been registered or that the corresponding functionality is operational.
//!
//! # Architectural boundaries
//!
//! This facade owns:
//!
//! - Public pipeline module composition.
//! - Public module visibility.
//! - Stable pipeline-level re-exports.
//! - Disambiguated names for overlapping report types.
//! - Facade-level API contract tests.
//!
//! It does not own:
//!
//! - Stage implementation or dynamic extension loading.
//! - Stage registration or provider discovery.
//! - Dependency graph ownership outside the stage-ordering contract.
//! - Stage execution algorithms.
//! - Compilation lifecycle state.
//! - Cancellation implementation.
//! - Budget accounting.
//! - Semantic analysis or source-language meaning.
//! - Type, ownership or effect semantics.
//! - Capability and requirement semantics.
//! - Canonical intermediate representations.
//! - Optimization or transformation algorithms.
//! - Physical resource discovery or allocation.
//! - Physical scheduling or routing.
//! - Backend implementation or code generation.
//! - Artifact serialization or deployment.
//! - Runtime execution.
//!
//! Those responsibilities remain with their existing repository authorities.
//!
//! # Dependency and integration model
//!
//! The conceptual integration is:
//!
//! ```text
//! compiler configuration
//!          |
//!          v
//! compilation context and lifecycle
//!          |
//!          v
//! stage contracts and stage definitions
//!          |
//!          v
//! dependency validation and ordering
//!          |
//!          v
//! stage execution
//!          |
//!          v
//! pipeline orchestration
//!          |
//!          v
//! compilation result / deployment handoff
//! ```
//!
//! Failure classification and recovery policy are cross-cutting:
//!
//! ```text
//! stage / ordering / execution / orchestration
//!                    |
//!                    v
//!              failure reporting
//!                    |
//!                    v
//!             recovery evaluation
//! ```
//!
//! This diagram describes responsibility boundaries, not a requirement that
//! every component directly imports the component immediately above it.
//!
//! The actual Rust dependencies are defined by the child modules.
//!
//! # Canonical repository integration
//!
//! The pipeline coordinates the following authorities:
//!
//! - `compiler::configuration`: compiler configuration and operational budgets.
//! - `compiler::context`: compilation context, cancellation and lifecycle.
//! - `compiler::analysis`: compiler-facing analysis coordination.
//! - `compiler::passes`: pass registration, ordering and execution contracts.
//! - `compiler::optimization`: optimization planning and validation.
//! - `compiler::planning`: target, capability and resource feasibility plans.
//! - `compiler::lowering`: representation conversion and target legalization.
//! - `compiler::backends`: compiler-facing backend integration.
//! - `compiler::artifacts`: artifact validation and metadata.
//! - `compiler::linking`: linking and composition.
//! - `compiler::diagnostics`: structured compiler diagnostics.
//! - `compiler::incremental`: fingerprints, invalidation and cache integration.
//! - `compiler::parallel`: compiler-work parallelism.
//! - `compiler::security`: verification and trust boundaries.
//!
//! The pipeline must consume these components through their established
//! contracts. It must not create replacement authorities.
//!
//! # Frontend and grammar boundary
//!
//! Source syntax and source-level compilation intent are owned by the grammar
//! and frontend, not by this module.
//!
//! Relevant grammar authorities include:
//!
//! - `grammar/Zamani.g4`
//! - `grammar/compile/compile.g4`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/execution-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! The frontend owns parsing, AST construction, structural validation and
//! integration with the canonical semantic model.
//!
//! The semantic authority determines the meaning of source declarations,
//! including requirements, capabilities, preferences, constraints, hints,
//! effects and provenance.
//!
//! This pipeline consumes validated compiler inputs. It must not parse grammar
//! productions, construct a second AST, or reinterpret source semantics.
//!
//! # Canonical representation boundaries
//!
//! The pipeline coordinates existing representation authorities:
//!
//! - `src/zuir/`: universal target-independent IR.
//! - `src/classical/`: classical computation representations.
//! - `src/quantum/ir/`: canonical quantum IR.
//! - `src/hdl/hdlir/`: hardware-description IR.
//! - `src/ai/`: AI computation integration.
//! - `src/hybrid/`: cross-domain integration.
//! - `src/distributed/`: distributed computation integration.
//!
//! Compiler IR adapters and lowering components must preserve those ownership
//! boundaries. This facade does not define another IR.
//!
//! # Conditional and domain-specific compilation
//!
//! A valid program need not use every computing domain.
//!
//! Consequently:
//!
//! - Required stages must complete successfully.
//! - Conditional stages may be inactive when their validated activation
//!   conditions are not satisfied.
//! - Policy-controlled stages must respect the applicable compilation policy.
//! - A skipped stage is not a completed stage.
//! - A dependent stage must not consume output from an incomplete prerequisite.
//! - A required stage must not be silently omitted.
//!
//! Domain activation must be based on validated compiler inputs and explicit
//! stage contracts, not guesses based on target names or source text.
//!
//! Standard stage definitions describe the standard pipeline contract.
//! They do not, by themselves, instantiate or register executable stages.
//!
//! # Ordering, execution and determinism
//!
//! `ordering` owns dependency validation and deterministic ordering.
//!
//! `execution` owns stage invocation, direct prerequisite-output delivery,
//! cancellation checks during execution and execution records.
//!
//! `orchestration` owns coordination of a pipeline invocation and its
//! lifecycle transitions.
//!
//! This facade introduces no additional scheduler, worker pool or execution
//! ordering policy.
//!
//! Any future parallel stage executor must explicitly preserve dependency
//! ordering, output visibility, cancellation, failure propagation and the
//! determinism contract of each stage.
//!
//! Compiler-work parallelism remains coordinated through the existing
//! `compiler::parallel` subsystem.
//!
//! # Failure and recovery
//!
//! `failure` owns structured failure normalization and reporting.
//!
//! `recovery` owns recovery policy evaluation and the evidence required to
//! consider retries or fallbacks.
//!
//! Exposing these APIs does not mean that recovery is automatically executed.
//!
//! In particular:
//!
//! - Retry permission is not proof that replay is safe.
//! - Retry requires explicit policy and replay-safety evidence.
//! - Cancellation must not be silently overridden.
//! - Internal correctness-invariant failures must not be treated as ordinary
//!   transient failures.
//! - A fallback must preserve the required semantics and constraints.
//! - An unverified fallback must not be reported as successful.
//!
//! Recovery execution must remain an explicit responsibility of the
//! appropriate orchestration or higher-level recovery integration.
//!
//! # POCO-REAF and scalability
//!
//! Zamani's objective is:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! This facade introduces no arbitrary universal ceilings on:
//!
//! - Pipeline stages or dependencies.
//! - Compilation units, modules or functions.
//! - CPUs, cores or workers.
//! - GPUs, FPGAs, ASICs or QPUs.
//! - Accelerators or processing elements.
//! - Distributed nodes or participants.
//! - Memory or storage.
//! - Physical or logical qubits.
//! - Tensor dimensions or rank.
//! - IR operations or transformations.
//! - Targets, providers or computing domains.
//!
//! The facade does not silently truncate collections or discard stage,
//! dependency, failure or provenance information.
//!
//! Operational budgets are consumed through the existing compilation
//! context and configuration authorities.
//!
//! Configurable budgets are operational controls, not semantic hardware
//! limits. An unlimited configured budget does not promise unlimited physical
//! resources.
//!
//! Actual feasibility remains subject to available resources, target
//! capabilities, operating-system limits, representation limits and policy.
//!
//! # Runtime boundary
//!
//! Compiler execution means executing compiler stages.
//!
//! It does not mean executing the compiled program.
//!
//! Physical resource acquisition, device allocation, physical scheduling,
//! runtime dispatch and program execution remain responsibilities of the
//! runtime, hardware and deployment authorities.
//!
//! # API compatibility
//!
//! Existing public module paths and re-export names are retained in this
//! facade. Component APIs remain owned and versioned by their respective
//! modules.
//!
//! Changes to a child component must be reflected here deliberately when its
//! public API is intended to be part of the pipeline facade.
//!
//! This facade must not re-export private implementation details merely to
//! make unrelated modules compile.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden in this module and its implementation.
//!
//! No global mutable state, implicit worker pool, hidden retry loop or
//! independent cancellation mechanism is introduced here.

// ============================================================================
// MODULE DECLARATIONS
// ============================================================================

/// Canonical compiler-stage contracts.
pub mod stage;

/// Standard pipeline stage definitions and domain-stage descriptors.
pub mod stages;

/// Dependency validation and deterministic stage ordering.
pub mod ordering;

/// Stage invocation and execution reports.
pub mod execution;

/// Structured pipeline failure classification and reporting.
pub mod failure;

/// Recovery policy evaluation and safety evidence.
pub mod recovery;

/// Pipeline invocation lifecycle orchestration.
pub mod orchestration;

// ============================================================================
// API VERSION
// ============================================================================

/// Version of the public pipeline composition surface.
///
/// This version is independent of the Zamani language version, grammar
/// version, semantic-model version, IR versions, backend versions and runtime
/// versions.
///
/// Child modules expose their own API versions.
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

// Execution and orchestration have separate report types. Explicit aliases
// preserve both public contracts without introducing ambiguous root names.

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

// These aliases distinguish orchestration-level reporting from executor-level
// reporting. The original types remain owned by orchestration.rs.

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
// FACADE CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn stage_identifier_is_available_from_facade() {
        let identifier =
            StageId::new("compiler.frontend.parse")
                .expect("valid stage identifier");

        assert_eq!(
            identifier.as_str(),
            "compiler.frontend.parse"
        );
    }

    #[test]
    fn stage_identifiers_are_not_subject_to_an_arbitrary_length_ceiling() {
        let value = format!(
            "compiler.{}",
            "stage".repeat(10_000)
        );

        let identifier = StageId::new(value.clone())
            .expect("dynamically sized stage identifier");

        assert_eq!(identifier.as_str(), value);
    }

    #[test]
    fn standard_stage_definitions_are_available_and_valid() {
        let definitions = standard_stage_definitions()
            .expect("standard stage definitions");

        assert!(
            !definitions.is_empty(),
            "standard pipeline definitions must not be empty"
        );

        validate_standard_definitions(&definitions)
            .expect("standard stage definitions must validate");
    }

    #[test]
    fn standard_definitions_include_required_and_conditional_activation() {
        let definitions = standard_stage_definitions()
            .expect("standard stage definitions");

        assert!(
            definitions.iter().any(|definition| {
                definition.activation() == StageActivation::Required
            }),
            "the standard pipeline must describe required stages"
        );

        assert!(
            definitions.iter().any(|definition| {
                definition.activation() == StageActivation::Conditional
            }),
            "the standard pipeline must support conditional stages"
        );
    }

    #[test]
    fn standard_stage_dependencies_produce_a_valid_ordering() {
        let definitions = standard_stage_definitions()
            .expect("standard stage definitions");

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

    #[test]
    fn recovery_is_conservative_by_default() {
        let policy = RecoveryPolicy::new();

        assert_eq!(policy.max_retries(), 0);
        assert!(!policy.fallback_allowed());
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

    #[test]
    fn orchestration_api_is_available() {
        let orchestrator = PipelineOrchestrator::new();

        let _ = orchestrator;
    }

    #[test]
    fn executor_and_orchestration_report_names_are_unambiguous() {
        let _: Option<StageExecutionReport> = None;
        let _: Option<OrchestrationExecutionReport> = None;

        let _: Option<ExecutorStageExecutionRecord> = None;
        let _: Option<OrchestrationStageExecutionRecord> = None;

        let _: Option<ExecutorStageExecutionStatus> = None;
        let _: Option<OrchestrationStageExecutionStatus> = None;
    }
}
