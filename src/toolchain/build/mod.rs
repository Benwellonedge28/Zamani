
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Build Subsystem
//!
//! Path: `src/toolchain/build/mod.rs`
//!
//! Rust edition: 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Mission
//!
//! Provides the canonical composition and public integration boundary
//! for Zamani project builds.
//!
//! The build subsystem coordinates the transformation of an accepted
//! logical build request into one or more verified target-specific
//! build outputs.
//!
//! Its architecture supports:
//!
//! - Classical computation.
//! - Quantum computation.
//! - Hardware description and hardware realization.
//! - AI and tensor computation.
//! - Hybrid computation.
//! - Distributed computation.
//! - Embedded and accelerator computation.
//! - Networking and cryptographic computation.
//! - Future registered computational domains.
//!
//! The subsystem does not require every target or provider to support
//! every computational domain.
//!
//! ## POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! One logical build may produce multiple compatible target-specific
//! realizations.
//!
//! The build subsystem preserves:
//!
//! - One logical BuildId.
//! - One validated logical build request.
//! - One immutable build plan.
//! - Authoritative feasibility evidence.
//! - One dependency graph for build orchestration.
//! - Shared logical build-input identity.
//! - Separate target-realization input identities.
//! - Target-specific compilation outcomes.
//! - Verified artifact identities and provenance.
//!
//! Different targets may produce different artifact formats and
//! representations while preserving the applicable source-program
//! semantic contracts.
//!
//! A target request is not proof of feasibility.
//! A discovered target is not proof of resource availability.
//! A feasible target is not a physical resource reservation.
//! A successful compilation is not proof of runtime compatibility.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Build subsystem composition.
//! - Public build module organization.
//! - Deliberate public API re-exports.
//! - Build subsystem documentation.
//! - Cross-module architectural boundary.
//!
//! It does not own:
//!
//! - Source grammar, lexing or parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Language-level requirements or capabilities.
//! - Dependency resolution.
//! - Project or workspace discovery.
//! - Manifest parsing.
//! - Target registration or discovery.
//! - Capability matching.
//! - Resource planning or physical allocation.
//! - Feasibility analysis.
//! - Compiler implementation.
//! - Backend selection or code generation.
//! - External process execution.
//! - Physical hardware discovery.
//! - Runtime scheduling or execution.
//! - Package creation or publication.
//! - Deployment execution.
//!
//! These responsibilities remain with their canonical authorities.
//!
//! ## Canonical repository authorities
//!
//! Public build requests:
//! `crate::toolchain::api::request`
//!
//! Validated internal requests:
//! `build::request`
//!
//! Build identities:
//! `crate::toolchain::ids`
//!
//! Toolchain errors:
//! `crate::toolchain::errors`
//!
//! Operational budgets:
//! `crate::toolchain::configuration::budget`
//!
//! Workspace metadata:
//! `crate::toolchain::workspace::metadata`
//!
//! Project configuration:
//! `crate::zamani_project_config`
//!
//! Compiler service:
//! `crate::compiler::api`
//!
//! Target identities:
//! `crate::compiler::configuration::target`
//!
//! Feasibility assessment:
//! `crate::compiler::planning::feasibility`
//!
//! Compiler incremental infrastructure:
//! `crate::compiler::incremental`
//!
//! Artifact representation and validation:
//! `crate::compiler::artifacts`
//!
//! Backend generation:
//! `crate::backend`
//!
//! Physical hardware integration:
//! `crate::zhal`
//!
//! Runtime execution:
//! `crate::zur`
//!
//! ## Build lifecycle
//!
//! ```text
//! Public API BuildRequest
//!          |
//!          v
//! Validated internal BuildRequest
//!          |
//!          v
//! Project / workspace validation
//!          |
//!          v
//! Dependency resolution
//!          |
//!          v
//! Canonical target resolution
//!          |
//!          v
//! Immutable BuildPlan
//!          |
//!          v
//! Compiler feasibility assessment
//!          |
//!          v
//! AssessedBuildPlan
//!          |
//!          v
//! BuildInputs snapshot
//!          |
//!          v
//! BuildGraph construction
//!          |
//!          v
//! Build task execution
//!          |
//!          v
//! CompilerService
//!          |
//!          v
//! Backend and artifact production
//!          |
//!          v
//! Artifact verification
//!          |
//!          v
//! BuildOutputs
//!          |
//!          v
//! Incremental state and reporting
//! ```
//!
//! The lifecycle describes integration contracts. It does not imply
//! that every upstream service is already production-ready or that
//! every stage is currently orchestrated by a single BuildService.
//!
//! ## Scalability
//!
//! No arbitrary universal maximum is introduced for:
//!
//! - Projects.
//! - Source units.
//! - Build inputs.
//! - Build tasks.
//! - Build dependencies.
//! - Targets.
//! - Computing domains.
//! - CPUs or processing cores.
//! - GPUs or other accelerators.
//! - FPGAs, ASICs or QPUs.
//! - Quantum resources.
//! - Distributed participants.
//! - Memory.
//! - Storage.
//! - Tensor dimensions.
//! - Build artifacts.
//!
//! Actual operations remain subject to:
//!
//! - Available physical resources.
//! - Rust representation and address-space limits.
//! - Operating-system constraints.
//! - Provider capabilities.
//! - Explicit operational budgets.
//! - Security and authorization policies.
//! - Semantic and target compatibility.
//!
//! Resource exhaustion must be reported honestly.
//!
//! Literal infinite computation, memory or storage is not promised.
//!
//! ## Determinism
//!
//! Build subsystem composition introduces no hidden global state,
//! implicit host-target selection, random identifiers or timestamps.
//!
//! Deterministic ordering, identity and fingerprint rules are owned
//! by the corresponding build modules.
//!
//! ## Concurrency
//!
//! This module does not create worker pools, schedulers, locks or
//! global mutable execution state.
//!
//! Build graph construction and task execution remain separate
//! responsibilities.
//!
//! Concurrent execution must respect the canonical shared operational
//! budget and cancellation contracts.
//!
//! ## Security
//!
//! Unsafe Rust is forbidden.
//!
//! Untrusted build requests must pass structural validation.
//!
//! Build outputs must not be reported as verified until the
//! authoritative artifact verification operation succeeds.
//!
//! Provider failures, compiler failures, cancellation and resource
//! exhaustion must not be converted into successful build results.
//!
//! ## Versioning
//!
//! The in-process build module is not itself a serialized protocol.
//!
//! Public API versions, build-input schema versions, incremental
//! schema versions, compiler versions and artifact formats remain
//! independently versioned by their respective authorities.
//!
//! Persisted build plans or build graphs must use explicitly
//! versioned serialization contracts.
//!
//! ## Module integration
//!
//! `request`
//! - Validates and preserves caller build intent.
//!
//! `plan`
//! - Owns BuildPlan and AssessedBuildPlan.
//! - Associates resolved targets with feasibility evidence.
//!
//! `graph`
//! - Owns the deterministic build-task dependency graph.
//! - Validates dependency relationships and cycles.
//! - Does not execute or schedule tasks.
//!
//! `executor`
//! - Invokes the public compiler service.
//! - Enforces feasibility, cancellation and budget contracts.
//! - Validates compilation outputs.
//! - Requires authoritative artifact verification.
//!
//! `inputs`
//! - Composes immutable build-input snapshots.
//! - Separates logical and target-realization fingerprints.
//!
//! `outputs`
//! - Collects verified execution outputs.
//! - Associates canonical artifacts with target realizations.
//! - Validates output coverage.
//!
//! `incremental`
//! - Coordinates incremental-build decisions.
//! - Reuses compiler-owned incremental infrastructure.
//!
//! `reporting`
//! - Produces structured build and failure reports.
//! - Preserves logical and target-specific outcomes.
//!
//! ## Public API compatibility
//!
//! The public API request remains:
//!
//! `crate::toolchain::api::BuildRequest`
//!
//! The validated internal request is exported here as:
//!
//! `crate::toolchain::build::ValidatedBuildRequest`
//!
//! This distinction prevents accidental substitution of unvalidated
//! caller input for the internal validated build boundary.


// ============================================================================
// BUILD MODULES
// ============================================================================

/// Validated build-request boundary.
///
/// Converts the public API request into an immutable internal request.
/// Does not perform project discovery or feasibility analysis.
pub mod request;

/// Immutable build plans and compiler feasibility association.
///
/// Owns BuildPlan, target-resolution results and AssessedBuildPlan.
pub mod plan;

/// Deterministic build-task dependency graph.
///
/// Owns graph structure, dependency validation, cycle detection
/// and dependency-first ordering. Does not execute tasks.
pub mod graph;

/// Compiler-facing build execution orchestration.
///
/// Coordinates source providers, target configurations, cancellation,
/// budget accounting, compiler invocation and artifact verification.
pub mod executor;

/// Immutable build-input snapshots and fingerprints.
///
/// Separates target-independent logical build identity from
/// target-specific realization identity.
pub mod inputs;

/// Verified build-output collection and target coverage.
///
/// Adapts execution results to the canonical compiler artifact model.
pub mod outputs;

/// Incremental-build coordination.
///
/// Reuses compiler-owned incremental infrastructure and invalidation.
pub mod incremental;

/// Structured build and failure reporting.
///
/// Produces deterministic machine-readable build reports.
pub mod reporting;


// ============================================================================
// VALIDATED REQUEST API
// ============================================================================

/// Internal validated build request.
///
/// This alias is intentionally distinct from
/// `crate::toolchain::api::BuildRequest`, which represents
/// externally supplied caller intent.
pub use request::BuildRequest as ValidatedBuildRequest;


// ============================================================================
// BUILD PLANNING API
// ============================================================================

/// Canonical resolved build target.
pub use plan::ResolvedBuildTarget;

/// Canonical target-resolution result.
pub use plan::BuildTargetResolution;

/// Immutable logical build plan.
pub use plan::BuildPlan;

/// Build plan associated with compiler feasibility evidence.
pub use plan::AssessedBuildPlan;


// ============================================================================
// BUILD GRAPH API
// ============================================================================

/// Deterministic build-task dependency graph.
pub use graph::BuildGraph;


// ============================================================================
// BUILD EXECUTION API
// ============================================================================

/// Validated source unit supplied to the compiler.
pub use executor::BuildSourceUnit;

/// Source acquisition provider contract.
pub use executor::BuildSourceProvider;

/// Target-specific compiler configuration.
pub use executor::TargetCompilerConfiguration;

/// Target compiler configuration provider contract.
pub use executor::TargetConfigurationProvider;

/// Authoritative artifact verification contract.
pub use executor::ArtifactVerifier;

/// Build cancellation probe contract.
pub use executor::CancellationProbe;

/// Cancellation implementation for operations that are not cancelled.
pub use executor::NeverCancelled;

/// Verified target-specific build artifact.
pub use executor::VerifiedBuildArtifact;

/// Result of actual build execution.
pub use executor::BuildExecutionResult;

/// Generic build executor.
pub use executor::BuildExecutor;

/// Default compiler-API-backed build executor.
pub use executor::DefaultBuildExecutor;


// ============================================================================
// BUILD INPUT API
// ============================================================================

/// Build-input scope classification.
pub use inputs::BuildInputScope;

/// Build-input authority identity.
pub use inputs::BuildInputAuthority;

/// One fingerprinted build-input record.
pub use inputs::BuildInputRecord;

/// Target-realization fingerprint.
pub use inputs::RealizationFingerprint;

/// Immutable build-input snapshot.
pub use inputs::BuildInputs;

/// Builder for immutable build-input snapshots.
pub use inputs::BuildInputsBuilder;

/// Build-input API version.
pub use inputs::BUILD_INPUTS_API_VERSION;

/// Build-input schema version.
pub use inputs::BUILD_INPUTS_SCHEMA_VERSION;


// ============================================================================
// BUILD OUTPUT API
// ============================================================================

/// Canonical artifact adaptation contract.
pub use outputs::CanonicalArtifactAdapter;

/// Build-output completeness classification.
pub use outputs::BuildOutputStatus;

/// Target-to-artifact coverage.
pub use outputs::TargetOutputCoverage;

/// Aggregate build-output statistics.
pub use outputs::BuildOutputSummary;

/// Validated build outputs.
pub use outputs::BuildOutputs;


// ============================================================================
// INCREMENTAL BUILD API
// ============================================================================

/// Identity of an incremental build artifact.
pub use incremental::IncrementalArtifactId;

/// Logical or target-specific artifact scope.
pub use incremental::BuildArtifactScope;

/// Classification of incremental input changes.
pub use incremental::IncrementalChangeKind;

/// Target-realization input change.
pub use incremental::RealizationChange;

/// Structured incremental-build report.
pub use incremental::IncrementalBuildReport;

/// Incremental-build coordination service.
pub use incremental::IncrementalBuildCoordinator;

/// Incremental-build API version.
pub use incremental::INCREMENTAL_BUILD_API_VERSION;

/// Incremental-build schema version.
pub use incremental::INCREMENTAL_BUILD_SCHEMA_VERSION;


// ============================================================================
// BUILD REPORTING API
// ============================================================================

/// Build-report completion classification.
pub use reporting::BuildReportStatus;

/// Build failure classification.
pub use reporting::BuildFailureStatus;

/// Logical build-input report.
pub use reporting::BuildInputReport;

/// Target-realization input report.
pub use reporting::RealizationInputReport;

/// Target buildability classification.
pub use reporting::TargetBuildability;

/// Target-specific build report.
pub use reporting::TargetBuildReport;

/// Report for unresolved target selection.
pub use reporting::UnresolvedTargetReport;

/// Structured build-plan report.
pub use reporting::BuildPlanReport;

/// Structured build-output report.
pub use reporting::BuildOutputReport;

/// Structured build report.
pub use reporting::BuildReport;

/// Structured build-failure report.
pub use reporting::BuildFailureReport;

/// Build-report schema identifier.
pub use reporting::BUILD_REPORT_SCHEMA;

/// Build-failure report schema identifier.
pub use reporting::BUILD_FAILURE_REPORT_SCHEMA;


// ============================================================================
// COMPOSITION CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_the_validated_request_without_shadowing_api_request() {
        fn accepts_validated_request(
            _: Option<ValidatedBuildRequest>,
        ) {
        }

        accepts_validated_request(None);
    }

    #[test]
    fn exposes_build_plan_contracts() {
        fn accepts_plan(
            _: Option<BuildPlan>,
            _: Option<AssessedBuildPlan>,
        ) {
        }

        accepts_plan(None, None);
    }

    #[test]
    fn exposes_build_graph_contract() {
        let graph = BuildGraph::new();

        assert!(graph.is_empty());
        assert_eq!(graph.task_count(), 0);
        assert_eq!(graph.dependency_count(), 0);
    }

    #[test]
    fn exposes_build_executor_contract() {
        fn accepts_executor<C: crate::compiler::api::CompilerService>(
            _: Option<BuildExecutor<C>>,
        ) {
        }

        accepts_executor::<crate::compiler::api::CompilerApi>(None);
    }

    #[test]
    fn exposes_build_input_contracts() {
        fn accepts_inputs(
            _: Option<BuildInputs>,
            _: Option<BuildInputsBuilder>,
        ) {
        }

        accepts_inputs(None, None);
    }

    #[test]
    fn exposes_build_output_contracts() {
        fn accepts_outputs(
            _: Option<BuildOutputs>,
            _: Option<BuildOutputSummary>,
        ) {
        }

        accepts_outputs(None, None);
    }

    #[test]
    fn exposes_incremental_build_contracts() {
        fn accepts_incremental(
            _: Option<IncrementalBuildCoordinator>,
            _: Option<IncrementalBuildReport>,
        ) {
        }

        accepts_incremental(None, None);
    }

    #[test]
    fn exposes_reporting_contracts() {
        fn accepts_reports(
            _: Option<BuildReport<'static>>,
            _: Option<BuildFailureReport<'static>>,
        ) {
        }

        accepts_reports(None, None);
    }

    #[test]
    fn build_schema_versions_are_available() {
        assert_ne!(BUILD_INPUTS_API_VERSION, 0);
        assert_ne!(BUILD_INPUTS_SCHEMA_VERSION, 0);
        assert_ne!(INCREMENTAL_BUILD_API_VERSION, 0);
        assert_ne!(INCREMENTAL_BUILD_SCHEMA_VERSION, 0);
        assert!(!BUILD_REPORT_SCHEMA.is_empty());
        assert!(!BUILD_FAILURE_REPORT_SCHEMA.is_empty());
    }
}
